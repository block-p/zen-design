#![cfg_attr(windows, windows_subsystem = "windows")]

mod agent;
mod detector;
mod export;
mod frames;
mod history;
mod platform;
mod providers;
mod settings;
mod skills;
mod tts;
mod updater;
mod watcher;

use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Path as AxumPath, Query, State},
    http::StatusCode,
    response::{
        sse::{Event, Sse},
        IntoResponse,
    },
    routing::{delete, get, post},
    Json, Router,
};
use futures_util::stream::{self, Stream, StreamExt};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio_stream::wrappers::{BroadcastStream, IntervalStream};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[derive(RustEmbed)]
#[folder = "../studio/dist/"]
struct StudioAssets;

use agent::{truncate_words, AgentEvent, AgentOrchestrator, CustomThemeTokens, PromptRequest};
use settings::{Settings, KNOWN_PROVIDERS, PROVIDER_ANTHROPIC, PROVIDER_OPENAI};

const DEFAULT_BIND_ADDR: &str = "127.0.0.1:7456";
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Clone)]
struct AppState {
    event_sender: broadcast::Sender<String>,
    orchestrator: Arc<AgentOrchestrator>,
    agent_lock: Arc<Mutex<()>>,
    /// Handle for the most recent agent task, so the studio can stop a long
    /// model/agent run instead of waiting for it.
    agent_run: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    settings: Arc<RwLock<Settings>>,
    http: reqwest::Client,
    /// Only one browser recording at a time: two Chrome instances on the same
    /// virtual displays would fight over the frame buffer.
    exporting: Arc<AtomicBool>,
    /// Set once ffmpeg is capturing. The holding page polls this and only then
    /// swaps in the artifact, so the video starts at the artifact's first frame.
    export_armed: Arc<AtomicBool>,
    /// Loopback origin the daemon uses to load the preview inside Chrome.
    public_base: String,
    export_dir: PathBuf,
    workspace: PathBuf,
    current_session_id: Arc<tokio::sync::RwLock<Option<String>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("zen_design_server=info,tower_http=info")),
        )
        .with_target(false)
        .compact()
        .init();

    let workspace = resolve_workspace()?;
    let studio = resolve_studio();
    let bind_addr =
        std::env::var("ZEN_BIND_ADDR").unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_string());
    let public_base = public_base_url(&bind_addr);
    let export_dir = resolve_export_dir();
    std::fs::create_dir_all(&export_dir)?;
    info!("Workspace: {}", workspace.display());
    info!("Rust studio: {}", studio.display());
    info!("MP4 exports: {}", export_dir.display());
    if !studio.join("index.html").is_file() {
        warn!("Studio bundle is missing; run `trunk build` in studio/ before starting the daemon");
    }

    history::bootstrap_existing_workspace(&workspace);

    let settings = Arc::new(RwLock::new(Settings::load()));
    {
        let current = settings.read().await;
        info!(
            "Model provider: {} · settings at {}",
            current.provider_label(),
            Settings::config_path().display()
        );
    }

    // A generous connect timeout, but no total timeout: model reasoning may
    // take as long as it takes. Progress streams over `/api/events`.
    let http = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .user_agent("ZenDesign/0.1")
        .build()?;

    // The broadcast channel fans agent events out to every connected studio tab.
    let (event_sender, _) = broadcast::channel::<String>(256);

    // Keep the watcher handle alive for the lifetime of the process. Dropping it
    // stops the OS watch, so it deliberately lives until `serve` returns.
    let _file_watcher = watcher::start_watcher(&workspace.to_string_lossy(), event_sender.clone())
        .map_err(|e| format!("failed to start workspace watcher: {e}"))?;

    let current_session_id = Arc::new(tokio::sync::RwLock::new(None));

    let orchestrator = Arc::new(AgentOrchestrator::new(
        workspace.to_string_lossy().to_string(),
        event_sender.clone(),
        settings.clone(),
        http.clone(),
        current_session_id.clone(),
    ));

    let state = AppState {
        event_sender,
        orchestrator,
        agent_lock: Arc::new(Mutex::new(())),
        agent_run: Arc::new(Mutex::new(None)),
        settings,
        http,
        exporting: Arc::new(AtomicBool::new(false)),
        export_armed: Arc::new(AtomicBool::new(false)),
        public_base,
        export_dir,
        workspace: workspace.clone(),
        current_session_id,
    };

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/events", get(events))
        .route("/api/prompt", post(prompt))
        .route("/api/agent/stop", post(stop_agent))
        .route("/api/export", post(start_export))
        .route("/api/export/latest", get(export_latest))
        .route("/api/export/pptx", get(export_pptx))
        .route("/api/export/stage", get(export_stage))
        .route("/api/export/armed", get(export_armed))
        .route("/api/settings", get(get_settings).put(put_settings))
        .route("/api/settings/detected", get(get_detected_sources))
        .route("/api/settings/apply-detected", post(apply_detected_source))
        .route(
            "/api/settings/import-opencode",
            post(import_opencode_settings),
        )
        .route("/api/models", get(models))
        .route(
            "/api/history",
            get(get_history)
                .post(post_history)
                .delete(clear_all_history),
        )
        .route("/api/history/:id", delete(delete_history_item))
        .route("/api/history/:id/activate", post(activate_history_item))
        .route("/api/history/new-project", post(new_project))
        .route("/api/themes/generate", post(generate_theme))
        .route("/api/slides/ai-action", post(slide_ai_action))
        .route(
            "/api/tts",
            get(tts_synthesize_get).post(tts_synthesize_post),
        )
        .route("/api/artifact", get(get_artifact).put(save_artifact))
        .route("/api/system/version", get(system_version))
        .route("/api/system/check-update", get(system_check_update))
        .route("/api/system/update", post(system_perform_update))
        // Serve the agent's live output. ServeDir handles traversal protection
        // and content types; index.html is served for `/preview/`.
        .nest_service(
            "/preview",
            ServeDir::new(&workspace).append_index_html_on_directories(true),
        );

    let app = if studio.join("index.html").is_file() {
        info!("Serving studio from disk: {}", studio.display());
        app.fallback_service(ServeDir::new(&studio).append_index_html_on_directories(true))
    } else {
        info!("Serving studio from standalone embedded assets");
        app.fallback(static_embedded_handler)
    };

    let app = app.layer(TraceLayer::new_for_http()).with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("ZenDesign daemon listening on http://{bind_addr}");
    info!("Studio: http://{bind_addr}/");
    info!("Artifact preview: http://{bind_addr}/preview/index.html");

    maybe_open_app_window(&bind_addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn maybe_open_app_window(bind_addr: &str) {
    if std::env::var("ZEN_NO_OPEN").is_ok() {
        return;
    }
    let url = format!("http://{bind_addr}/");
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        #[cfg(windows)]
        {
            // First try launching Edge in standalone application window mode (--app)
            let mut cmd = std::process::Command::new("cmd");
            crate::platform::configure_command(&mut cmd);
            let launched = cmd
                .args(["/C", "start", "msedge", &format!("--app={url}")])
                .spawn();
            if launched.is_err() {
                // Fallback to default browser
                let mut cmd = std::process::Command::new("cmd");
                crate::platform::configure_command(&mut cmd);
                let _ = cmd.args(["/C", "start", &url]).spawn();
            }
        }
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open").arg(&url).spawn();
        }
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
        }
    });
}

/// Resolve the workspace folder whether the binary is started from `server/`,
/// from the repository root, or via an installed binary. A GUI launch (macOS
/// Finder, Windows shortcut) often has `/` or `C:\Windows\System32` as its
/// working directory, so a relative fallback must never be writable-by-luck.
fn resolve_workspace() -> std::io::Result<PathBuf> {
    if let Ok(custom) = std::env::var("ZEN_WORKSPACE_DIR") {
        let custom = custom.trim();
        if !custom.is_empty() {
            let path = PathBuf::from(custom);
            std::fs::create_dir_all(&path).map_err(|error| {
                std::io::Error::new(
                    error.kind(),
                    format!(
                        "Could not create ZEN_WORKSPACE_DIR {}: {error}. Choose a writable folder.",
                        path.display()
                    ),
                )
            })?;
            return Ok(std::fs::canonicalize(&path).unwrap_or(path));
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        let mut curr = exe.parent();
        while let Some(parent) = curr {
            let ws = parent.join("workspace");
            if ws.is_dir() {
                return Ok(std::fs::canonicalize(&ws).unwrap_or(ws));
            }
            curr = parent.parent();
        }
    }

    let local = PathBuf::from("./workspace");
    if local.is_dir() {
        return Ok(std::fs::canonicalize(&local).unwrap_or(local));
    }

    let manifest_relative = Path::new(env!("CARGO_MANIFEST_DIR")).join("../workspace");
    let candidates = [PathBuf::from("../workspace"), manifest_relative];

    for candidate in candidates {
        if candidate.is_dir() {
            return Ok(std::fs::canonicalize(&candidate).unwrap_or(candidate));
        }
    }

    // Installed/GUI launch: use a per-user data directory, never `./workspace`
    // relative to a read-only working directory.
    let fallback = platform::default_workspace();
    if let Err(error) = std::fs::create_dir_all(&fallback) {
        warn!(
            "Could not create the user workspace {}: {error}; trying a temporary folder",
            fallback.display()
        );
        let temporary = std::env::temp_dir().join("zen-design-workspace");
        std::fs::create_dir_all(&temporary).map_err(|second| {
            std::io::Error::new(
                second.kind(),
                format!(
                    "No writable workspace. Could not create {} ({error}) or {} ({second}). Set ZEN_WORKSPACE_DIR to a writable folder.",
                    fallback.display(),
                    temporary.display()
                ),
            )
        })?;
        return Ok(std::fs::canonicalize(&temporary).unwrap_or(temporary));
    }

    Ok(std::fs::canonicalize(&fallback).unwrap_or(fallback))
}

/// Locate the Trunk output regardless of the process working directory.
fn resolve_studio() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        let mut curr = exe.parent();
        while let Some(parent) = curr {
            for candidate in ["dist", "studio/dist", "studio"] {
                let dir = parent.join(candidate);
                if dir.is_dir() {
                    return std::fs::canonicalize(&dir).unwrap_or(dir);
                }
            }
            curr = parent.parent();
        }
    }

    let local_dist = PathBuf::from("./dist");
    if local_dist.is_dir() {
        return std::fs::canonicalize(&local_dist).unwrap_or(local_dist);
    }

    let local_studio_dist = PathBuf::from("./studio/dist");
    if local_studio_dist.is_dir() {
        return std::fs::canonicalize(&local_studio_dist).unwrap_or(local_studio_dist);
    }

    let manifest_relative = Path::new(env!("CARGO_MANIFEST_DIR")).join("../studio/dist");
    let candidates = [PathBuf::from("../studio/dist"), manifest_relative];

    for candidate in candidates {
        if candidate.is_dir() {
            return std::fs::canonicalize(&candidate).unwrap_or(candidate);
        }
    }

    PathBuf::from("../studio/dist")
}

/// Origin Chrome should load the preview from. A wildcard bind is reachable
/// through loopback from the same machine.
fn public_base_url(bind_addr: &str) -> String {
    let (host, port) = bind_addr.rsplit_once(':').unwrap_or(("127.0.0.1", "7456"));
    let host = host.trim_start_matches('[').trim_end_matches(']');
    let host = if host.is_empty() || host == "0.0.0.0" || host == "::" {
        "127.0.0.1"
    } else {
        host
    };
    if host.contains(':') {
        format!("http://[{host}]:{port}")
    } else {
        format!("http://{host}:{port}")
    }
}

/// MP4s live outside the workspace so the file watcher never treats an export
/// as an artifact change (which would reload the preview mid-review).
fn resolve_export_dir() -> PathBuf {
    if let Ok(custom) = std::env::var("ZEN_EXPORT_DIR") {
        if !custom.trim().is_empty() {
            return PathBuf::from(custom);
        }
    }

    let base = platform::cache_dir();
    base.join("zen-design").join("exports")
}

async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

#[derive(Debug, Deserialize)]
struct SettingsUpdate {
    provider: String,
    base_url: Option<String>,
    model: Option<String>,
    api_key: Option<String>,
    elevenlabs_key: Option<String>,
    tts_voice: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ModelsQuery {
    provider: Option<String>,
}

/// Public view of the model settings. Keys are never echoed back: the studio
/// only learns whether one is configured and where it comes from.
async fn get_settings(State(state): State<AppState>) -> impl IntoResponse {
    let settings = state.settings.read().await;
    Json(settings.to_public_json())
}

async fn put_settings(
    State(state): State<AppState>,
    Json(update): Json<SettingsUpdate>,
) -> impl IntoResponse {
    let provider = update.provider.trim().to_string();
    if !KNOWN_PROVIDERS.contains(&provider.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Unknown provider: {provider}") })),
        )
            .into_response();
    }

    let mut settings = state.settings.write().await;
    settings.provider = provider.clone();
    settings.active_source = None;
    if let Some(profile) = settings.profile_mut(&provider) {
        if let Some(value) = update.base_url {
            profile.base_url = value.trim().to_string();
        }
        if let Some(value) = update.model {
            profile.model = value.trim().to_string();
        }
        if let Some(value) = update.api_key {
            profile.api_key = value.trim().to_string();
        }
    }
    if let Some(key) = update.elevenlabs_key {
        settings.elevenlabs_key = key.trim().to_string();
    }
    if let Some(voice) = update.tts_voice {
        settings.tts_voice = voice.trim().to_string();
    }

    match settings.save() {
        Ok(()) => Json(settings.to_public_json()).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": error })),
        )
            .into_response(),
    }
}

async fn models(
    State(state): State<AppState>,
    Query(query): Query<ModelsQuery>,
) -> impl IntoResponse {
    let provider = query
        .provider
        .unwrap_or_else(|| PROVIDER_OPENAI.to_string());
    let settings = state.settings.read().await.clone();
    match providers::list_models(&state.http, &settings, &provider).await {
        Ok(models) => Json(json!({ "provider": provider, "models": models })).into_response(),
        Err(error) => (StatusCode::BAD_GATEWAY, Json(json!({ "error": error }))).into_response(),
    }
}

async fn get_detected_sources() -> impl IntoResponse {
    let sources = detector::scan_local_sources();
    Json(json!({ "sources": sources })).into_response()
}

#[derive(Deserialize)]
struct ApplyDetectedPayload {
    id: String,
}

async fn apply_detected_source(
    State(state): State<AppState>,
    Json(payload): Json<ApplyDetectedPayload>,
) -> impl IntoResponse {
    let resolved = match detector::resolve_source(&payload.id) {
        Ok(res) => res,
        Err(err) => {
            return (StatusCode::BAD_REQUEST, Json(json!({ "error": err }))).into_response()
        }
    };

    let mut candidate = state.settings.read().await.clone();
    candidate.provider = resolved.provider.clone();
    candidate.active_source = Some(payload.id.clone());

    if resolved.provider == settings::PROVIDER_OPENAI {
        candidate.openai.base_url = resolved.base_url.clone();
        // Replace, don't merge: applying a keyless source must not keep the
        // previous provider's secret in the profile.
        candidate.openai.api_key = resolved.api_key.clone();
        if let Some(m) = resolved.model.clone() {
            candidate.openai.model = m;
        }
    } else if resolved.provider == settings::PROVIDER_ANTHROPIC {
        candidate.anthropic.base_url = resolved.base_url.clone();
        candidate.anthropic.api_key = resolved.api_key.clone();
        if let Some(m) = resolved.model.clone() {
            candidate.anthropic.model = m;
        }
    } else if resolved.provider == settings::PROVIDER_OPENCODE {
        if let Some(m) = resolved.model.clone() {
            candidate.opencode.model = m;
        }
    }

    let detected_list = detector::scan_local_sources();
    let specific_models = detected_list
        .iter()
        .find(|s| s.id == payload.id)
        .map(|s| s.models.clone())
        .unwrap_or_default();

    let models = if !specific_models.is_empty() {
        specific_models
    } else {
        let mut live_models = Vec::new();
        if let Ok(live) = providers::list_models(&state.http, &candidate, &candidate.provider).await
        {
            if !live.is_empty() {
                live_models = live;
            }
        }
        if live_models.is_empty() {
            if let Some(m) = resolved.model.as_ref() {
                if !m.trim().is_empty() {
                    live_models.push(m.clone());
                }
            }
        }
        live_models
    };

    let active_provider = candidate.provider.clone();
    let profile = candidate.profile_mut(&active_provider);
    if let Some(p) = profile {
        if p.model.trim().is_empty() || (!models.is_empty() && !models.contains(&p.model)) {
            p.model = default_model(&models);
        }
    }

    let view = {
        let mut settings = state.settings.write().await;
        *settings = candidate;
        match settings.save() {
            Ok(()) => settings.to_public_json(),
            Err(error) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": error })),
                )
                    .into_response()
            }
        }
    };

    Json(json!({
        "settings": view,
        "models": models,
        "applied_id": payload.id,
    }))
    .into_response()
}

struct ImportedProvider {
    name: String,
    base_url: String,
    api_key: String,
    models: Vec<String>,
}

/// One-click import from the OpenCode configuration. Useful for 9Router and
/// any other OpenAI-compatible provider the user already runs in OpenCode.
async fn import_opencode_settings(State(state): State<AppState>) -> impl IntoResponse {
    let imported = match read_opencode_provider() {
        Ok(imported) => imported,
        Err(error) => {
            return (StatusCode::BAD_REQUEST, Json(json!({ "error": error }))).into_response()
        }
    };

    let mut candidate = state.settings.read().await.clone();
    candidate.provider = PROVIDER_OPENAI.to_string();
    candidate.openai.base_url = imported.base_url.clone();
    if !imported.api_key.is_empty() {
        candidate.openai.api_key = imported.api_key.clone();
    }

    let mut models = imported.models.clone();
    if let Ok(live) = providers::list_models(&state.http, &candidate, PROVIDER_OPENAI).await {
        if !live.is_empty() {
            models = live;
        }
    }
    if candidate.openai.model.trim().is_empty()
        || !models.is_empty() && !models.contains(&candidate.openai.model)
    {
        candidate.openai.model = default_model(&models);
    }

    let warning = if imported.base_url.starts_with("http://")
        && !imported.base_url.contains("127.0.0.1")
        && !imported.base_url.contains("localhost")
    {
        Some("This endpoint uses plain HTTP: prompts and the API key are not encrypted in transit.")
    } else {
        None
    };

    let view = {
        let mut settings = state.settings.write().await;
        settings.provider = candidate.provider.clone();
        settings.openai = candidate.openai.clone();
        match settings.save() {
            Ok(()) => settings.to_public_json(),
            Err(error) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": error })),
                )
                    .into_response()
            }
        }
    };

    info!(
        "Imported provider '{}' from OpenCode ({} models)",
        imported.name,
        models.len()
    );

    Json(json!({
        "settings": view,
        "models": models,
        "imported_from": imported.name,
        "warning": warning,
    }))
    .into_response()
}

fn read_opencode_provider() -> Result<ImportedProvider, String> {
    let endpoint = detector::read_opencode_endpoint()?;
    Ok(ImportedProvider {
        name: endpoint.provider_name,
        base_url: endpoint.base_url,
        api_key: endpoint.api_key,
        models: endpoint.models,
    })
}

fn default_model(models: &[String]) -> String {
    for wanted in [
        "ag/gemini-3.8-flash-high",
        "ag/gemini-3.8-flash",
        "ag/gemini-3.8-flash-medium",
        "ag/gemini-3.8-flash-low",
        "kr/glm-5",
        "kr/minimax-m2.1",
        "gemini-2.5-flash",
        "gemini-2.0-flash",
        "gpt-4o",
        "ag/claude-opus-4-6-thinking",
        "ag/claude-sonnet-4-6",
    ] {
        if models.iter().any(|model| model == wanted) {
            return wanted.to_string();
        }
    }
    models
        .first()
        .cloned()
        .unwrap_or_else(|| "ag/gemini-3.8-flash-high".to_string())
}

/// SSE endpoint. Unlike the old 800ms-fixed-timeout design this stream has no
/// deadline: it stays open for the lifetime of the tab. A named `heartbeat`
/// event is emitted every two seconds, including while an LLM is reasoning.
async fn events(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut interval = tokio::time::interval(HEARTBEAT_INTERVAL);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    // `interval`'s first tick is immediate, so the browser gets a heartbeat as
    // soon as the EventSource connects, then one every two seconds.

    let heartbeats = IntervalStream::new(interval).map(|_| {
        let payload = json!({
            "type": "heartbeat",
            "timestamp": now_millis(),
            "interval_ms": HEARTBEAT_INTERVAL.as_millis(),
        });
        Ok(Event::default()
            .event("heartbeat")
            .data(payload.to_string()))
    });

    let receiver = state.event_sender.subscribe();
    let agent_events = BroadcastStream::new(receiver).filter_map(|result| {
        let event = match result {
            Ok(message) => message_to_sse(message),
            Err(error) => {
                // A slow client missed some messages. Force a preview resync so
                // the UI cannot get stuck showing stale files.
                warn!("SSE client lagged ({error}); requesting preview resync");
                Some(Ok(Event::default().event("reload").data(
                    r#"{"event_type":"reload","message":"Event stream resynced"}"#,
                )))
            }
        };
        futures_util::future::ready(event)
    });

    let stream = stream::select(heartbeats, agent_events);
    Sse::new(stream)
}

/// Messages on the broadcast channel are JSON `AgentEvent`s, except the
/// watcher's plain `"reload"` marker. Translate them into named SSE frames so
/// the browser can use `addEventListener("reload", ...)`.
fn message_to_sse(message: String) -> Option<Result<Event, Infallible>> {
    if message == "reload" {
        return Some(Ok(Event::default().event("reload").data(
            json!({
                "event_type": "reload",
                "message": "Workspace files changed",
            })
            .to_string(),
        )));
    }

    match serde_json::from_str::<serde_json::Value>(&message) {
        Ok(value) => {
            let mut event_type = value
                .get("event_type")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("message")
                .to_string();
            // "error" is reserved by EventSource for connection failures.
            // Emitting a custom-named event keeps transport errors distinct
            // from agent failures in the studio UI.
            if event_type == "error" {
                event_type = "agent_error".to_string();
            }
            Some(Ok(Event::default().event(event_type).data(message)))
        }
        Err(_) => Some(Ok(Event::default().event("message").data(message))),
    }
}

async fn stop_running_agent(state: &AppState) {
    if let Some(handle) = state.agent_run.lock().await.take() {
        handle.abort();
        let event = AgentEvent {
            event_type: "agent_error".to_string(),
            message: "Previous generation cancelled.".to_string(),
        };
        if let Ok(payload) = serde_json::to_string(&event) {
            let _ = state.event_sender.send(payload);
        }
    }
}

// Accepts a brief, then runs the orchestrator in the background. The HTTP
/// request returns immediately (202) so a long reasoning pass can never collide
/// with a client-side request timeout; progress arrives over `/api/events`.
async fn prompt(
    State(state): State<AppState>,
    Json(mut request): Json<PromptRequest>,
) -> impl IntoResponse {
    if request.prompt.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "prompt must not be empty" })),
        )
            .into_response();
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let session_id = request
        .session_id
        .clone()
        .unwrap_or_else(|| format!("{timestamp}_{}", uuid::Uuid::new_v4()));
    request.session_id = Some(session_id.clone());

    // Stop previous running agent if any
    stop_running_agent(&state).await;

    // Track as currently active session
    *state.current_session_id.write().await = Some(session_id.clone());

    let orchestrator = state.orchestrator.clone();
    let sender = state.event_sender.clone();
    let lock = state.agent_lock.clone();
    let design_system = request.design_system.clone();
    let mode = request.mode.clone();
    let deep = request.deep;
    let workspace = state.workspace.clone();

    info!(
        "Accepted prompt for session {session_id} (design_system={design_system}, mode={mode}, deep={deep}, len={})",
        request.prompt.len()
    );

    let item = history::add_history_entry(
        &workspace,
        session_id.clone(),
        request.prompt.clone(),
        design_system.clone(),
        mode.clone(),
        deep,
    );

    // Serialize agent runs: concurrent writes to index.html would make the
    // preview flicker between two half-written artifacts.
    let handle = tokio::spawn(async move {
        let _guard = lock.lock().await;

        if let Err(error) = orchestrator.execute_prompt(request).await {
            warn!("Agent run failed: {error}");
            let event = AgentEvent {
                event_type: "error".to_string(),
                message: error,
            };
            if let Ok(payload) = serde_json::to_string(&event) {
                let _ = sender.send(payload);
            }
        }
    });

    // Remember the task so `POST /api/agent/stop` can cancel it. Completed
    // handles are filtered out by `is_finished` at stop time.
    *state.agent_run.lock().await = Some(handle);

    (
        StatusCode::ACCEPTED,
        Json(json!({
            "status": "accepted",
            "session_id": session_id,
            "design_system": design_system,
            "mode": mode,
            "deep": deep,
            "item": item,
        })),
    )
        .into_response()
}

/// Cancels the current model/agent run. Aborting the task drops the provider
/// future, and `kill_on_drop(true)` terminates an OpenCode child process.
async fn stop_agent(State(state): State<AppState>) -> impl IntoResponse {
    let handle = state.agent_run.lock().await.take();
    match handle {
        Some(handle) if !handle.is_finished() => {
            handle.abort();
            providers::emit_event(&state.event_sender, "agent_error", "Run stopped by you.");
            (StatusCode::OK, Json(json!({ "status": "stopped" }))).into_response()
        }
        _ => (StatusCode::OK, Json(json!({ "status": "idle" }))).into_response(),
    }
}

#[derive(Deserialize)]
struct ThemeGeneratePayload {
    prompt: String,
}

async fn generate_theme(
    State(state): State<AppState>,
    Json(payload): Json<ThemeGeneratePayload>,
) -> impl IntoResponse {
    let prompt = payload.prompt.trim();
    if prompt.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "prompt must not be empty" })),
        )
            .into_response();
    }

    let settings = state.settings.read().await.clone();

    // If an LLM provider (OpenAI, 9Router, Anthropic) is configured, try calling it
    if settings.provider == PROVIDER_OPENAI || settings.provider == PROVIDER_ANTHROPIC {
        if let Ok(theme) = generate_llm_theme(&state.http, &settings, prompt).await {
            info!(
                "Generated theme via LLM provider ({}): {}",
                settings.provider, theme.name
            );
            return (
                StatusCode::OK,
                Json(json!({ "theme": theme, "source": "llm" })),
            )
                .into_response();
        }
    }

    // Fallback to high-speed built-in algorithmic engine
    let theme = generate_algorithmic_theme(prompt);
    (
        StatusCode::OK,
        Json(json!({ "theme": theme, "source": "algorithmic" })),
    )
        .into_response()
}

async fn generate_llm_theme(
    client: &reqwest::Client,
    settings: &Settings,
    prompt: &str,
) -> Result<CustomThemeTokens, String> {
    let is_persian = prompt
        .chars()
        .any(|c| ('\u{0600}'..='\u{06FF}').contains(&c));
    let font_guide = if is_persian {
        "For Persian text, use \"'Vazirmatn', system-ui, sans-serif\" as font_family."
    } else {
        "Use a clean modern font stack like \"'Inter', system-ui, sans-serif\"."
    };

    let system_instructions = format!(
        "You are an elite Design System Architect & Colorist.\n\
        Your task is to generate cohesive, beautiful UI design tokens based on the user's theme prompt.\n\
        {}\n\
        You MUST respond with valid JSON ONLY (no markdown formatting, no code fences, no commentary).\n\
        The JSON MUST have exactly these keys:\n\
        {{\n\
          \"name\": \"Theme display name\",\n\
          \"bg\": \"#RRGGBB\",\n\
          \"text\": \"#RRGGBB\",\n\
          \"accent\": \"#RRGGBB\",\n\
          \"card_bg\": \"#RRGGBB\",\n\
          \"border_color\": \"#RRGGBB\",\n\
          \"font_family\": \"Font CSS stack\"\n\
        }}\n\
        All colors MUST be valid 7-character hexadecimal strings starting with #.",
        font_guide
    );

    let candidates = providers::resolve_candidates(settings);
    let mut raw_text = String::new();

    for candidate in &candidates {
        if candidate.provider == PROVIDER_OPENAI && !candidate.base_url.is_empty() {
            let body = json!({
                "model": candidate.model,
                "temperature": 0.5,
                "messages": [
                    { "role": "system", "content": system_instructions },
                    { "role": "user", "content": format!("Theme Prompt: {prompt}") }
                ]
            });
            let mut req = client
                .post(format!("{}/chat/completions", candidate.base_url))
                .json(&body);
            if !candidate.api_key.is_empty() {
                req = req.bearer_auth(&candidate.api_key);
            }
            if let Ok(resp) = req.send().await {
                if resp.status().is_success() {
                    if let Ok(res_json) = resp.json::<serde_json::Value>().await {
                        let content = res_json["choices"][0]["message"]["content"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        if !content.trim().is_empty() {
                            raw_text = content;
                            break;
                        }
                    }
                }
            }
        } else if candidate.provider == PROVIDER_ANTHROPIC
            && !candidate.base_url.is_empty()
            && !candidate.api_key.is_empty()
        {
            let body = json!({
                "model": candidate.model,
                "max_tokens": 1024,
                "system": system_instructions,
                "messages": [
                    { "role": "user", "content": format!("Theme Prompt: {prompt}") }
                ]
            });
            if let Ok(resp) = client
                .post(format!("{}/v1/messages", candidate.base_url))
                .header("x-api-key", candidate.api_key.trim())
                .header("anthropic-version", "2023-06-01")
                .json(&body)
                .send()
                .await
            {
                if resp.status().is_success() {
                    if let Ok(res_json) = resp.json::<serde_json::Value>().await {
                        let text = res_json["content"][0]["text"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        if !text.trim().is_empty() {
                            raw_text = text;
                            break;
                        }
                    }
                }
            }
        }
    }

    if raw_text.is_empty() {
        return Err("No LLM provider available for theme generation".to_string());
    }

    // Clean any markdown code blocks if the model wrapped it
    let clean_json = raw_text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    #[derive(Deserialize)]
    struct PartialTheme {
        name: String,
        bg: String,
        text: String,
        accent: String,
        card_bg: String,
        border_color: String,
        font_family: String,
    }

    let parsed: PartialTheme = serde_json::from_str(clean_json).map_err(|e| e.to_string())?;
    let dot = parsed.accent.clone();
    let swatch = format!(
        "linear-gradient(135deg, {} 0%, {} 100%)",
        parsed.accent, parsed.bg
    );

    Ok(CustomThemeTokens {
        name: parsed.name,
        bg: parsed.bg,
        text: parsed.text,
        accent: parsed.accent,
        font_family: parsed.font_family,
        card_bg: parsed.card_bg,
        border_color: parsed.border_color,
        dot,
        swatch,
    })
}

fn generate_algorithmic_theme(prompt: &str) -> CustomThemeTokens {
    let lower = prompt.to_lowercase();

    let is_persian = prompt
        .chars()
        .any(|c| ('\u{0600}'..='\u{06FF}').contains(&c));
    let default_font = if is_persian {
        "'Vazirmatn', system-ui, sans-serif".to_string()
    } else {
        "'Inter', system-ui, sans-serif".to_string()
    };

    // Insurance / Banking / Finance / سرمایه / بانک / بیمه / مالی / اعتباری (Corporate Trust Navy & Royal Blue)
    if lower.contains("بیمه")
        || lower.contains("بانک")
        || lower.contains("مالی")
        || lower.contains("سرمایه")
        || lower.contains("اعتبار")
        || lower.contains("insurance")
        || lower.contains("bank")
        || lower.contains("finance")
        || lower.contains("fintech")
    {
        let is_light = lower.contains("روشن") || lower.contains("سفید") || lower.contains("light");
        let (bg, card, border, text, accent, dot) = if is_light {
            (
                "#f8fafc", "#ffffff", "#cbd5e1", "#0f172a", "#1d4ed8", "#2563eb",
            )
        } else {
            (
                "#0b1329", "#111c38", "#1e293b", "#f1f5f9", "#3b82f6", "#60a5fa",
            )
        };
        CustomThemeTokens {
            name: if is_persian {
                "بیمه و اعتماد سازمانی / Corporate Trust".to_string()
            } else {
                "Corporate Trust (Insurance & Finance)".to_string()
            },
            bg: bg.to_string(),
            text: text.to_string(),
            accent: accent.to_string(),
            font_family: default_font,
            card_bg: card.to_string(),
            border_color: border.to_string(),
            dot: dot.to_string(),
            swatch: format!("linear-gradient(135deg, {accent} 0%, {bg} 100%)"),
        }
    } else if lower.contains("پزشکی")
        || lower.contains("درمان")
        || lower.contains("سلامت")
        || lower.contains("دارو")
        || lower.contains("بیمارستان")
        || lower.contains("medical")
        || lower.contains("health")
        || lower.contains("clinic")
        || lower.contains("pharma")
    {
        let is_light = lower.contains("روشن") || lower.contains("سفید") || lower.contains("light");
        let (bg, card, border, text, accent, dot) = if is_light {
            (
                "#f0fdfa", "#ffffff", "#99f6e4", "#134e4a", "#0d9488", "#14b8a6",
            )
        } else {
            (
                "#041f1e", "#072e2d", "#115e59", "#f0fdfa", "#14b8a6", "#2dd4bf",
            )
        };
        CustomThemeTokens {
            name: if is_persian {
                "پزشکی و سلامت / Clinical Teal".to_string()
            } else {
                "Clinical Teal (Health & Medical)".to_string()
            },
            bg: bg.to_string(),
            text: text.to_string(),
            accent: accent.to_string(),
            font_family: default_font,
            card_bg: card.to_string(),
            border_color: border.to_string(),
            dot: dot.to_string(),
            swatch: format!("linear-gradient(135deg, {accent} 0%, {bg} 100%)"),
        }
    } else if lower.contains("املاک")
        || lower.contains("ساختمان")
        || lower.contains("معماری")
        || lower.contains("سازه")
        || lower.contains("real estate")
        || lower.contains("architecture")
        || lower.contains("interior")
    {
        CustomThemeTokens {
            name: if is_persian {
                "معماری و املاک / Bronze Estate".to_string()
            } else {
                "Bronze Estate".to_string()
            },
            bg: "#141210".to_string(),
            text: "#f5f5f4".to_string(),
            accent: "#d97706".to_string(),
            font_family: default_font,
            card_bg: "#211d19".to_string(),
            border_color: "#443c33".to_string(),
            dot: "#f59e0b".to_string(),
            swatch: "linear-gradient(135deg, #d97706 0%, #141210 100%)".to_string(),
        }
    } else if lower.contains("کافه")
        || lower.contains("قهوه")
        || lower.contains("رستوران")
        || lower.contains("غذا")
        || lower.contains("فود")
        || lower.contains("cafe")
        || lower.contains("coffee")
        || lower.contains("restaurant")
    {
        CustomThemeTokens {
            name: if is_persian {
                "کافه و قهوه / Warm Espresso".to_string()
            } else {
                "Warm Espresso".to_string()
            },
            bg: "#180d09".to_string(),
            text: "#fed7aa".to_string(),
            accent: "#ea580c".to_string(),
            font_family: default_font,
            card_bg: "#2b1810".to_string(),
            border_color: "#4d2919".to_string(),
            dot: "#f97316".to_string(),
            swatch: "linear-gradient(135deg, #ea580c 0%, #180d09 100%)".to_string(),
        }
    // Red / Crimson / Ruby / سرخ / قرمز / زرشکی
    } else if lower.contains("قرمز")
        || lower.contains("سرخ")
        || lower.contains("زرشکی")
        || lower.contains("red")
        || lower.contains("crimson")
        || lower.contains("ruby")
        || lower.contains("blood")
        || lower.contains("scarlet")
    {
        let is_dark = lower.contains("مشکی")
            || lower.contains("سیاه")
            || lower.contains("تیره")
            || lower.contains("dark")
            || lower.contains("black")
            || !lower.contains("سفید");
        let (bg, card, border, text, accent) = if is_dark {
            ("#0a0a0c", "#161114", "#38171e", "#fce7f3", "#ef4444")
        } else {
            ("#fff5f5", "#ffffff", "#fed7d7", "#1a202c", "#e53e3e")
        };
        CustomThemeTokens {
            name: if is_persian {
                "قرمز شفق / Crimson Dark".to_string()
            } else {
                "Crimson Void".to_string()
            },
            bg: bg.to_string(),
            text: text.to_string(),
            accent: accent.to_string(),
            font_family: default_font,
            card_bg: card.to_string(),
            border_color: border.to_string(),
            dot: accent.to_string(),
            swatch: format!("linear-gradient(135deg, {accent} 0%, {bg} 100%)"),
        }
    } else if lower.contains("cyber")
        || lower.contains("neon")
        || lower.contains("arcade")
        || lower.contains("matrix")
        || lower.contains("نئون")
        || lower.contains("سایبر")
    {
        CustomThemeTokens {
            name: "Neon Cyberpunk".to_string(),
            bg: "#090a0f".to_string(),
            text: "#00f0ff".to_string(),
            accent: "#ff007f".to_string(),
            font_family: "'JetBrains Mono', monospace".to_string(),
            card_bg: "#121420".to_string(),
            border_color: "#ff007f55".to_string(),
            dot: "#00f0ff".to_string(),
            swatch: "linear-gradient(135deg, #ff007f 0%, #090a0f 100%)".to_string(),
        }
    } else if lower.contains("forest")
        || lower.contains("moss")
        || lower.contains("pine")
        || lower.contains("nature")
        || lower.contains("green")
        || lower.contains("سبز")
        || lower.contains("جنگل")
        || lower.contains("طبیعت")
    {
        CustomThemeTokens {
            name: if is_persian {
                "سبز نوردیک / Nordic Pine".to_string()
            } else {
                "Nordic Pine".to_string()
            },
            bg: "#0a1410".to_string(),
            text: "#ecfdf5".to_string(),
            accent: "#10b981".to_string(),
            font_family: default_font,
            card_bg: "#12241d".to_string(),
            border_color: "#1f4034".to_string(),
            dot: "#10b981".to_string(),
            swatch: "linear-gradient(135deg, #10b981 0%, #0a1410 100%)".to_string(),
        }
    } else if lower.contains("sunset")
        || lower.contains("warm")
        || lower.contains("coral")
        || lower.contains("autumn")
        || lower.contains("پاییز")
        || lower.contains("نارنجی")
        || lower.contains("آفتاب")
    {
        CustomThemeTokens {
            name: if is_persian {
                "غروب پاییزی / Sunset Ember".to_string()
            } else {
                "Sunset Ember".to_string()
            },
            bg: "#1f0d14".to_string(),
            text: "#fff1f2".to_string(),
            accent: "#f97316".to_string(),
            font_family: default_font,
            card_bg: "#321620".to_string(),
            border_color: "#582436".to_string(),
            dot: "#f97316".to_string(),
            swatch: "linear-gradient(135deg, #f97316 0%, #1f0d14 100%)".to_string(),
        }
    } else if lower.contains("gold")
        || lower.contains("luxury")
        || lower.contains("champagne")
        || lower.contains("royal")
        || lower.contains("طلا")
        || lower.contains("لوکس")
        || lower.contains("سلطنتی")
        || lower.contains("زرد")
    {
        CustomThemeTokens {
            name: if is_persian {
                "طلایی لوکس / Imperial Obsidian".to_string()
            } else {
                "Imperial Obsidian".to_string()
            },
            bg: "#0a0a0c".to_string(),
            text: "#faf5ef".to_string(),
            accent: "#eab308".to_string(),
            font_family: default_font,
            card_bg: "#16161c".to_string(),
            border_color: "#caa23440".to_string(),
            dot: "#eab308".to_string(),
            swatch: "linear-gradient(135deg, #eab308 0%, #0a0a0c 100%)".to_string(),
        }
    } else if lower.contains("ocean")
        || lower.contains("sea")
        || lower.contains("aqua")
        || lower.contains("blue")
        || lower.contains("دریا")
        || lower.contains("آبی")
        || lower.contains("اقیانوس")
    {
        CustomThemeTokens {
            name: if is_persian {
                "آبی اقیانوس / Abyssal Azure".to_string()
            } else {
                "Abyssal Azure".to_string()
            },
            bg: "#05111e".to_string(),
            text: "#f0f9ff".to_string(),
            accent: "#0284c7".to_string(),
            font_family: default_font,
            card_bg: "#0b2038".to_string(),
            border_color: "#133860".to_string(),
            dot: "#0284c7".to_string(),
            swatch: "linear-gradient(135deg, #0284c7 0%, #05111e 100%)".to_string(),
        }
    } else if lower.contains("purple")
        || lower.contains("violet")
        || lower.contains("lavender")
        || lower.contains("بنفش")
        || lower.contains("ارغوانی")
    {
        CustomThemeTokens {
            name: if is_persian {
                "بنفش کیهانی / Velvet Nebula".to_string()
            } else {
                "Velvet Nebula".to_string()
            },
            bg: "#120a22".to_string(),
            text: "#f5f3ff".to_string(),
            accent: "#a855f7".to_string(),
            font_family: default_font,
            card_bg: "#1d1236".to_string(),
            border_color: "#341f5e".to_string(),
            dot: "#a855f7".to_string(),
            swatch: "linear-gradient(135deg, #a855f7 0%, #120a22 100%)".to_string(),
        }
    } else if lower.contains("minimal")
        || lower.contains("light")
        || lower.contains("white")
        || lower.contains("paper")
        || lower.contains("سفید")
        || lower.contains("روشن")
    {
        CustomThemeTokens {
            name: if is_persian {
                "سفید مینیمال / Minimal White".to_string()
            } else {
                "Atelier Paper".to_string()
            },
            bg: "#f8f9fa".to_string(),
            text: "#18181b".to_string(),
            accent: "#2563eb".to_string(),
            font_family: default_font,
            card_bg: "#ffffff".to_string(),
            border_color: "#e4e4e7".to_string(),
            dot: "#2563eb".to_string(),
            swatch: "linear-gradient(135deg, #2563eb 0%, #f8f9fa 100%)".to_string(),
        }
    } else {
        // Deterministic hash-based derivation with clean HEX outputs
        let hash: u64 = prompt
            .bytes()
            .fold(5381, |acc, b| acc.wrapping_mul(33).wrapping_add(b as u64));
        let hue = (hash % 360) as f64;
        let s = 0.85;
        let l = 0.58;
        let c = (1.0 - (2.0 * l - 1.0_f64).abs()) * s;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0_f64).abs());
        let m = l - c / 2.0;
        let (r1, g1, b1) = if hue < 60.0 {
            (c, x, 0.0)
        } else if hue < 120.0 {
            (x, c, 0.0)
        } else if hue < 180.0 {
            (0.0, c, x)
        } else if hue < 240.0 {
            (0.0, x, c)
        } else if hue < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let accent = format!("#{r:02x}{g:02x}{b:02x}");
        let bg = "#0a0d14".to_string();
        let swatch = format!("linear-gradient(135deg, {accent} 0%, {bg} 100%)");

        let display_name = if is_persian {
            format!(
                "تم اختصاصی ({})",
                truncate_words(prompt, 18).trim_end_matches('…')
            )
        } else {
            format!(
                "Custom ({})",
                truncate_words(prompt, 18).trim_end_matches('…')
            )
        };

        CustomThemeTokens {
            name: display_name,
            bg,
            text: "#f1f5f9".to_string(),
            accent: accent.clone(),
            font_family: default_font,
            card_bg: "#131824".to_string(),
            border_color: "#1e293b".to_string(),
            dot: accent,
            swatch,
        }
    }
}

#[derive(Deserialize)]
struct SlideAiPayload {
    action: String, // "rewrite_slide", "summarize", "kpi_stats", "add_slide", "speaker_notes", "transform_layout"
    #[serde(default)]
    instruction: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    badge: Option<String>,
    #[serde(default)]
    subtitle: Option<String>,
    #[serde(default)]
    items: Option<Vec<String>>,
    #[serde(default)]
    target_layout: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct SlideAiResult {
    badge: String,
    title: String,
    subtitle: String,
    items: Vec<String>,
    speaker_notes: String,
    layout: String,
    html_snippet: String,
    source: String,
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn slide_header_html(badge: &str, title: &str, subtitle: &str) -> String {
    let sub_html = if !subtitle.trim().is_empty() {
        format!(
            "\n            <p class=\"s-sub body-text\">{}</p>",
            html_escape(subtitle)
        )
    } else {
        String::new()
    };
    format!(
        r#"        <header class="s-head">
            <div class="s-kicker category-badge"><span class="k-dot"></span><span class="k-txt">{}</span></div>
            <h2 class="s-title display-title">{}</h2>{}
        </header>"#,
        html_escape(badge),
        html_escape(title),
        sub_html
    )
}

fn render_slide_html(
    badge: &str,
    title: &str,
    subtitle: &str,
    items: &[String],
    speaker_notes: &str,
    layout: &str,
) -> String {
    let notes_attr = if !speaker_notes.is_empty() {
        format!(" data-speaker-notes=\"{}\"", html_escape(speaker_notes))
    } else {
        String::new()
    };
    let head = slide_header_html(badge, title, subtitle);

    match layout {
        "bento" => {
            let mut cards_html = String::new();
            for (i, item) in items.iter().enumerate() {
                let extra_cls = if i == 0 { " bento-hero" } else { "" };
                cards_html.push_str(&format!(
                    "                <div class=\"card{extra_cls}\"><p>{}</p></div>\n",
                    html_escape(item)
                ));
            }
            format!(
                r#"    <section class="slide" data-layout="bento"{notes_attr}>
        <div class="s-wrap slide-content">
{head}
            <div class="cards-grid bento-grid">
{cards_html}            </div>
        </div>
    </section>"#
            )
        }
        "stats" => {
            let mut stats_html = String::new();
            for (i, item) in items.iter().enumerate() {
                let (val, label) = if let Some(pos) = item.find(':') {
                    (&item[..pos], &item[pos + 1..])
                } else if let Some(pos) = item.find(" - ") {
                    (&item[..pos], &item[pos + 3..])
                } else {
                    let parts: Vec<&str> = item.split_whitespace().collect();
                    if parts.len() >= 2
                        && (parts[0].starts_with('+')
                            || parts[0].ends_with('%')
                            || parts[0].contains('٪')
                            || parts[0].chars().any(|c| {
                                c.is_ascii_digit() || ('\u{06F0}'..='\u{06F9}').contains(&c)
                            }))
                    {
                        (parts[0], item[parts[0].len()..].trim())
                    } else {
                        (item.as_str(), "")
                    }
                };
                let display_val = if val.trim().is_empty() {
                    format!("0{}", i + 1)
                } else {
                    val.trim().to_string()
                };
                let display_lbl = if label.trim().is_empty() {
                    item.trim().to_string()
                } else {
                    label.trim().to_string()
                };
                stats_html.push_str(&format!(
                    r#"                <div class="card stat-card">
                    <div class="stat-val">{}</div>
                    <div class="stat-label">{}</div>
                </div>\n"#,
                    html_escape(&display_val),
                    html_escape(&display_lbl)
                ));
            }
            format!(
                r#"    <section class="slide" data-layout="stats"{notes_attr}>
        <div class="s-wrap slide-content">
{head}
            <div class="cards-grid stats-grid">
{stats_html}            </div>
        </div>
    </section>"#
            )
        }
        "split" => {
            let mut cards_html = String::new();
            for item in items {
                cards_html.push_str(&format!(
                    "                <div class=\"card\"><p>{}</p></div>\n",
                    html_escape(item)
                ));
            }
            format!(
                r#"    <section class="slide" data-layout="split"{notes_attr}>
        <div class="s-wrap slide-content split-grid split-2">
            <div class="split-col-text">
{head}
            </div>
            <div class="split-col-cards cards-grid">
{cards_html}            </div>
        </div>
    </section>"#
            )
        }
        "timeline" => {
            let mut steps_html = String::new();
            for (i, item) in items.iter().enumerate() {
                steps_html.push_str(&format!(
                    r#"                <div class="timeline-step">
                    <div class="step-num">{:02}</div>
                    <div class="step-body"><p>{}</p></div>
                </div>\n"#,
                    i + 1,
                    html_escape(item)
                ));
            }
            format!(
                r#"    <section class="slide" data-layout="timeline"{notes_attr}>
        <div class="s-wrap slide-content">
{head}
            <div class="timeline-steps">
{steps_html}            </div>
        </div>
    </section>"#
            )
        }
        "comparison" => {
            let mut cards_html = String::new();
            for (i, item) in items.iter().enumerate() {
                let tag = if i % 2 == 0 {
                    "گزینه اول / Before"
                } else {
                    "گزینه دوم / After"
                };
                cards_html.push_str(&format!(
                    r#"                <div class="card comparison-col">
                    <div class="comparison-tag">{}</div>
                    <p>{}</p>
                </div>\n"#,
                    tag,
                    html_escape(item)
                ));
            }
            format!(
                r#"    <section class="slide" data-layout="comparison"{notes_attr}>
        <div class="s-wrap slide-content">
{head}
            <div class="cards-grid split-grid split-2">
{cards_html}            </div>
        </div>
    </section>"#
            )
        }
        "chart_bar" => {
            let default_data = [
                ("فاز اول / تحول معماری", 85, "+85%"),
                ("فاز دوم / مقیاس‌پذیری", 65, "65k req/s"),
                ("فاز سوم / یکپارچگی AI", 95, "99.4%"),
                ("فاز چهارم / توسعه بازار", 45, "4.8x"),
            ];
            let mut bars_html = String::new();
            for (i, item) in items.iter().enumerate() {
                let (label, pct, val) = if let Some(def) = default_data.get(i) {
                    let lbl = if item.trim().is_empty() {
                        def.0
                    } else {
                        item.as_str()
                    };
                    (lbl, def.1, def.2)
                } else {
                    (item.as_str(), 60 + (i as i32 * 10) % 35, "+40%")
                };
                let delay = i as f64 * 0.12;
                bars_html.push_str(&format!(
                    r#"                <div class="zd-bar-item" style="animation-delay: {delay:.2}s">
                    <span class="zd-bar-val">{val}</span>
                    <div class="zd-bar-track">
                        <div class="zd-bar-fill" style="height: {pct}%;"></div>
                    </div>
                    <div class="zd-bar-label">{label}</div>
                </div>\n"#,
                    delay = delay,
                    pct = pct,
                    val = val,
                    label = html_escape(label)
                ));
            }
            if bars_html.is_empty() {
                for (i, def) in default_data.iter().enumerate() {
                    let delay = i as f64 * 0.12;
                    bars_html.push_str(&format!(
                        r#"                <div class="zd-bar-item" style="animation-delay: {delay:.2}s">
                    <span class="zd-bar-val">{val}</span>
                    <div class="zd-bar-track">
                        <div class="zd-bar-fill" style="height: {pct}%;"></div>
                    </div>
                    <div class="zd-bar-label">{label}</div>
                </div>\n"#,
                        delay = delay,
                        pct = def.1,
                        val = def.2,
                        label = def.0
                    ));
                }
            }
            format!(
                r#"    <section class="slide" data-layout="chart_bar"{notes_attr}>
        <div class="s-wrap slide-content">
{head}
            <div class="zd-chart-container zd-chart-bar-wrap">
                <div class="zd-bars-grid">
{bars_html}                </div>
            </div>
        </div>
    </section>"#
            )
        }
        "chart_donut" => {
            let default_slices = [
                ("سهم فناوری‌های نوین", "45%", "#6366f1", "141.3 172.7", "0"),
                (
                    "بهینه‌سازی هزینه و انرژی",
                    "30%",
                    "#10b981",
                    "94.2 219.8",
                    "-141.3",
                ),
                ("سرعت و چابکی تیم", "25%", "#f59e0b", "78.5 235.5", "-235.5"),
            ];
            let mut legend_html = String::new();
            for (i, def) in default_slices.iter().enumerate() {
                let lbl = items.get(i).map(|s| s.as_str()).unwrap_or(def.0);
                legend_html.push_str(&format!(
                    r#"                    <div class="zd-donut-legend-item">
                        <span class="zd-donut-dot" style="background: {color};"></span>
                        <span class="zd-donut-legend-lbl">{lbl}</span>
                        <span class="zd-donut-legend-val">{val}</span>
                    </div>\n"#,
                    color = def.2,
                    lbl = html_escape(lbl),
                    val = def.1
                ));
            }
            format!(
                r##"    <section class="slide" data-layout="chart_donut"{notes_attr}>
        <div class="s-wrap slide-content split-grid split-2">
            <div class="split-col-text">
{head}
                <div class="zd-donut-legend">
{legend_html}                </div>
            </div>
            <div class="zd-donut-chart-col">
                <div class="zd-donut-svg-wrap">
                    <svg viewBox="0 0 120 120" class="zd-donut-svg">
                        <circle cx="60" cy="60" r="50" class="track-c"></circle>
                        <circle cx="60" cy="60" r="50" class="seg-a" stroke="#6366f1" stroke-dasharray="141.3 172.7" stroke-dashoffset="0"></circle>
                        <circle cx="60" cy="60" r="50" class="seg-b" stroke="#10b981" stroke-dasharray="94.2 219.8" stroke-dashoffset="-141.3"></circle>
                    </svg>
                    <div class="zd-donut-center-stat">
                        <div class="zd-donut-center-num">100%</div>
                        <div class="zd-donut-center-lbl">توزیع منابع</div>
                    </div>
                </div>
            </div>
        </div>
    </section>"##
            )
        }
        "chart_trend" => {
            let mut cards_html = String::new();
            for (i, item) in items.iter().take(3).enumerate() {
                let delta = if i == 0 {
                    "+140%"
                } else if i == 1 {
                    "+85%"
                } else {
                    "3.2x"
                };
                cards_html.push_str(&format!(
                    r#"                <div class="card zd-trend-mini-stat">
                    <div class="stat-val">{}</div>
                    <div class="stat-label">{}</div>
                </div>\n"#,
                    delta,
                    html_escape(item)
                ));
            }
            if cards_html.is_empty() {
                cards_html = r#"                <div class="card zd-trend-mini-stat"><div class="stat-val">+140%</div><div class="stat-label">شتاب در بازدهی</div></div>
                <div class="card zd-trend-mini-stat"><div class="stat-val">< 12ms</div><div class="stat-label">کاهش تاخیر پاسخ</div></div>
                <div class="card zd-trend-mini-stat"><div class="stat-val">99.9%</div><div class="stat-label">قابلیت اتکا</div></div>\n"#.to_string();
            }
            format!(
                r##"    <section class="slide" data-layout="chart_trend"{notes_attr}>
        <div class="s-wrap slide-content">
{head}
            <div class="zd-trend-hero">
                <div class="zd-trend-svg-wrap">
                    <svg viewBox="0 0 600 180" class="zd-trend-svg" preserveAspectRatio="none">
                        <defs>
                            <linearGradient id="trendGrad" x1="0" y1="0" x2="0" y2="1">
                                <stop offset="0%" stop-color="#06b6d4" stop-opacity="0.45"/>
                                <stop offset="100%" stop-color="#06b6d4" stop-opacity="0.0"/>
                            </linearGradient>
                        </defs>
                        <path d="M 0 160 Q 150 140 250 80 T 450 40 T 600 20 L 600 180 L 0 180 Z" fill="url(#trendGrad)" />
                        <path d="M 0 160 Q 150 140 250 80 T 450 40 T 600 20" fill="none" stroke="#06b6d4" stroke-width="4" stroke-linecap="round" class="zd-trend-line" />
                        <circle cx="250" cy="80" r="5" fill="#fff" stroke="#06b6d4" stroke-width="3" class="zd-trend-dot" />
                        <circle cx="450" cy="40" r="5" fill="#fff" stroke="#10b981" stroke-width="3" class="zd-trend-dot" />
                        <circle cx="600" cy="20" r="6" fill="#10b981" stroke="#fff" stroke-width="3" class="zd-trend-dot" />
                    </svg>
                </div>
                <div class="zd-trend-stats-row cards-grid">
{cards_html}                </div>
            </div>
        </div>
    </section>"##
            )
        }
        "roadmap" => {
            let mut steps_html = String::new();
            for (i, item) in items.iter().enumerate() {
                let num = i + 1;
                steps_html.push_str(&format!(
                    r#"            <div class="zd-roadmap-node">
                <div class="zd-roadmap-badge">Q0{}</div>
                <div class="zd-roadmap-dot"></div>
                <div class="card zd-roadmap-card"><div class="zd-roadmap-step-num">مرحله 0{}</div><p>{}</p></div>
            </div>\n"#,
                    num,
                    num,
                    html_escape(item)
                ));
            }
            if steps_html.is_empty() {
                steps_html = r#"            <div class="zd-roadmap-node">
                <div class="zd-roadmap-badge">Q01</div>
                <div class="zd-roadmap-dot"></div>
                <div class="card zd-roadmap-card"><div class="zd-roadmap-step-num">مرحله 01</div><p>طراحی ساختار پایه و تدوین استانداردهای تجربه کاربری</p></div>
            </div>
            <div class="zd-roadmap-node">
                <div class="zd-roadmap-badge">Q02</div>
                <div class="zd-roadmap-dot"></div>
                <div class="card zd-roadmap-card"><div class="zd-roadmap-step-num">مرحله 02</div><p>یکپارچه‌سازی ابزارهای هوش مصنوعی و خودکارسازی</p></div>
            </div>
            <div class="zd-roadmap-node">
                <div class="zd-roadmap-badge">Q03</div>
                <div class="zd-roadmap-dot"></div>
                <div class="card zd-roadmap-card"><div class="zd-roadmap-step-num">مرحله 03</div><p>بهینه‌سازی تجربه کاربری و پرفورمنس موتور رندر</p></div>
            </div>\n"#.to_string();
            }
            format!(
                r#"    <section class="slide" data-layout="roadmap"{notes_attr}>
        <div class="s-wrap slide-content">
{head}
            <div class="zd-roadmap-track">
{steps_html}            </div>
        </div>
    </section>"#
            )
        }
        _ => {
            let mut cards_html = String::new();
            for item in items {
                cards_html.push_str(&format!(
                    "                <div class=\"card\"><p>{}</p></div>\n",
                    html_escape(item)
                ));
            }
            format!(
                r#"    <section class="slide" data-layout="grid"{notes_attr}>
        <div class="s-wrap slide-content">
{head}
            <div class="cards-grid">
{cards_html}            </div>
        </div>
    </section>"#
            )
        }
    }
}

fn generate_algorithmic_slide_action(
    action: &str,
    instruction: &str,
    curr_title: &str,
    curr_badge: &str,
    curr_subtitle: &str,
    curr_items: &[String],
    target_layout: &str,
) -> SlideAiResult {
    let is_persian = format!("{action} {instruction} {curr_title} {curr_subtitle}")
        .chars()
        .any(|c| ('\u{0600}'..='\u{06FF}').contains(&c));

    let layout = if !target_layout.is_empty() {
        target_layout.to_string()
    } else if action == "kpi_stats" {
        "stats".to_string()
    } else {
        "bento".to_string()
    };

    let (badge, title, subtitle, items, speaker_notes) = match action {
        "summarize" => {
            let b = if !curr_badge.is_empty() {
                curr_badge.to_string()
            } else if is_persian {
                "خلاصه کلیدی".to_string()
            } else {
                "EXECUTIVE SUMMARY".to_string()
            };
            let t = if !curr_title.is_empty() {
                curr_title.to_string()
            } else if is_persian {
                "نکات برجسته و دستاوردها".to_string()
            } else {
                "Key Takeaways".to_string()
            };
            let sub = if !curr_subtitle.is_empty() {
                curr_subtitle
                    .split(['.', '،', '؛'])
                    .next()
                    .unwrap_or(curr_subtitle)
                    .trim()
                    .to_string()
            } else if is_persian {
                "فشرده‌ترین تصویر از نتایج و اهداف محوری این بخش.".to_string()
            } else {
                "Core highlights and distilled insights at a glance.".to_string()
            };
            let mut list = Vec::new();
            if curr_items.is_empty() {
                if is_persian {
                    list.push("دستیابی به هدف نهایی با بالاترین دقت".to_string());
                    list.push("بهینه‌سازی زمان اجرا و کاهش پیچیدگی‌ها".to_string());
                } else {
                    list.push("Streamlined execution with zero friction".to_string());
                    list.push("Accelerated delivery across all milestones".to_string());
                }
            } else {
                for item in curr_items.iter().take(3) {
                    let short = item.split(['.', ':', '،']).next().unwrap_or(item).trim();
                    list.push(short.to_string());
                }
            }
            let notes = if is_persian {
                format!(
                    "در این اسلاید تمرکز را روی ۳ پیام اصلی قرار می‌دهیم: {} و به مخاطب نشان می‌دهیم که این فاز با موفقیت جمع‌بندی شده است.",
                    list.join("، ")
                )
            } else {
                format!(
                    "Focus presenter delivery on the top takeaways: {}. Keep remarks punchy and highlight outcomes.",
                    list.join("; ")
                )
            };
            (b, t, sub, list, notes)
        }
        "kpi_stats" => {
            let b = if !curr_badge.is_empty() {
                curr_badge.to_string()
            } else if is_persian {
                "شاخص‌های کلیدی".to_string()
            } else {
                "KEY PERFORMANCE METRICS".to_string()
            };
            let t = if !curr_title.is_empty() {
                curr_title.to_string()
            } else if is_persian {
                "رشد و عملکرد در یک نگاه".to_string()
            } else {
                "Impact & Performance Metrics".to_string()
            };
            let sub = if is_persian {
                "داده‌های مستند از پیشرفت و شتاب فزاینده شاخص‌های کلیدی عملکرد.".to_string()
            } else {
                "Validated performance indicators demonstrating compounding scale.".to_string()
            };
            let mut list = Vec::new();
            if curr_items.is_empty() {
                if is_persian {
                    list.push("+85%: رشد سالانه کاربران".to_string());
                    list.push("10x: سرعت بارگذاری سریع‌تر".to_string());
                    list.push("99.9%: پایداری و تاب‌آوری سیستم".to_string());
                } else {
                    list.push("+85%: Year-over-year user growth".to_string());
                    list.push("10x: Latency and throughput optimization".to_string());
                    list.push("99.9%: Continuous platform uptime".to_string());
                }
            } else {
                let prefixes = ["+140%", "10x", "99.9%", "< 12ms", "4.9/5"];
                for (i, item) in curr_items.iter().enumerate() {
                    let p = prefixes[i % prefixes.len()];
                    list.push(format!("{}: {}", p, item));
                }
            }
            let notes = if is_persian {
                "در این اسلاید، آمار و سنجه‌های عددی کلیدی را به زبان ساده برای حاضرین تشریح کنید."
                    .to_string()
            } else {
                "Present these key metric data points to highlight measurable performance gains."
                    .to_string()
            };
            (b, t, sub, list, notes)
        }
        "speaker_notes" => {
            let b = curr_badge.to_string();
            let t = curr_title.to_string();
            let sub = curr_subtitle.to_string();
            let list = curr_items.to_vec();
            let items_joined_fa = list.join("، ");
            let items_joined_en = list.join(", ");
            let notes = if is_persian {
                format!(
                    "«سلام به همگی. در این اسلاید به {} می‌پردازیم. همان‌طور که می‌دانید، {} است. توجه شما را به این نکات جلب می‌کنم: {}. در صورت نیاز به توضیح بیشتر، در پایان در خدمت خواهم بود.»",
                    t,
                    if sub.is_empty() {
                        "این مبحث از اهمیت فوق‌العاده‌ای برخوردار است"
                    } else {
                        &sub
                    },
                    if list.is_empty() {
                        "تمرکز بر پایداری و کیفیت"
                    } else {
                        &items_joined_fa
                    }
                )
            } else {
                format!(
                    "\"Hello everyone. Turning now to {}. As you can see, {}. I want to draw your attention to: {}. Let's discuss any questions on this in Q&A.\"",
                    t,
                    if sub.is_empty() {
                        "this represents a foundational shift"
                    } else {
                        &sub
                    },
                    if list.is_empty() {
                        "core execution and stability"
                    } else {
                        &items_joined_en
                    }
                )
            };
            (b, t, sub, list, notes)
        }
        "add_slide" => {
            let user_topic = if !instruction.is_empty() {
                instruction
            } else {
                "گام‌های بعدی و نقشه راه"
            };
            let b = if is_persian {
                "فاز اجرایی"
            } else {
                "NEXT PHASE"
            };
            let t = user_topic.to_string();
            let sub = if is_persian {
                "برنامه‌ریزی اولویت‌بندی شده برای دستیابی به بالاترین ارزش در زمان تعیین‌شده."
            } else {
                "Prioritized strategic roadmap designed for compounding execution velocity."
            };
            let mut list = Vec::new();
            if is_persian {
                list.push("طراحی و بازبینی نهایی معماران محصول".to_string());
                list.push("پیاده‌سازی ماژول‌های هوش مصنوعی و اتوماسیون".to_string());
                list.push("عرضه عمومی، سنجش بازخورد و بهبود مستمر".to_string());
            } else {
                list.push("Architecture alignment & rapid prototyping".to_string());
                list.push("AI workflow automation & copilot rollout".to_string());
                list.push("Global rollout, feedback telemetry & iteration".to_string());
            }
            let notes = if is_persian {
                format!(
                    "«در این اسلاید نقشه راه {} را تشریح می‌کنیم و اقدامات مرحله به مرحله را برمی‌شماریم.»",
                    t
                )
            } else {
                format!(
                    "\"Here we walk through the roadmap for {}. Highlight the phased milestones.\"",
                    t
                )
            };
            (b.to_string(), t, sub.to_string(), list, notes)
        }
        _ => {
            let b = if !curr_badge.is_empty() {
                curr_badge.to_string()
            } else if is_persian {
                "معماری و نوآوری".to_string()
            } else {
                "CORE ARCHITECTURE".to_string()
            };
            // CRITICAL: Never overwrite title with instruction text!
            let t = if !curr_title.is_empty() {
                curr_title.to_string()
            } else if is_persian {
                "تحول و پایداری در مقیاس وسیع".to_string()
            } else {
                "Scalable Excellence & Transformation".to_string()
            };
            let sub = if !curr_subtitle.is_empty() {
                curr_subtitle.to_string()
            } else if is_persian {
                "رویکردی ساختاریافته به منظور ایجاد بیشترین بازدهی با کمترین پیچیدگی.".to_string()
            } else {
                "A structured paradigm to maximize leverage while eliminating unnecessary overhead."
                    .to_string()
            };
            let list = if curr_items.is_empty() {
                if is_persian {
                    vec![
                        "انعطاف‌پذیری بی‌نظیر و انطباق سریع".to_string(),
                        "سازگاری کامل با استانداردهای نوین طراحی".to_string(),
                        "تجربه کاربری بی‌نقص با بالاترین سرعت".to_string(),
                    ]
                } else {
                    vec![
                        "Uncompromising flexibility and rapid adaptation".to_string(),
                        "Seamless alignment with modern presentation standards".to_string(),
                        "Flawless user experience engineered for velocity".to_string(),
                    ]
                }
            } else {
                curr_items.to_vec()
            };
            let notes = if is_persian {
                format!(
                    "«توضیح دهید که چگونه {} به حل مسئله اصلی کمک می‌کند و چه مزایایی دارد.»",
                    t
                )
            } else {
                format!(
                    "\"Explain how {} directly addresses the core challenge and delivers value.\"",
                    t
                )
            };
            (b, t, sub, list, notes)
        }
    };

    let html_snippet =
        render_slide_html(&badge, &title, &subtitle, &items, &speaker_notes, &layout);

    SlideAiResult {
        badge,
        title,
        subtitle,
        items,
        speaker_notes,
        layout,
        html_snippet,
        source: "algorithmic".to_string(),
    }
}

fn extract_json_object(text: &str) -> Option<&str> {
    let start = text.find('{')?;
    let end = text.rfind('}')?;
    if end >= start {
        Some(&text[start..=end])
    } else {
        None
    }
}

async fn generate_llm_slide_action(
    client: &reqwest::Client,
    settings: &Settings,
    payload: &SlideAiPayload,
) -> Result<SlideAiResult, String> {
    let action = &payload.action;
    let instruction = payload.instruction.as_deref().unwrap_or_default();
    let title = payload.title.as_deref().unwrap_or_default();
    let badge = payload.badge.as_deref().unwrap_or_default();
    let subtitle = payload.subtitle.as_deref().unwrap_or_default();
    let items = payload.items.clone().unwrap_or_default();
    let target_layout = payload.target_layout.as_deref().unwrap_or("bento");

    let is_persian = format!(
        "{action} {instruction} {title} {subtitle} {}",
        items.join(" ")
    )
    .chars()
    .any(|c| ('\u{0600}'..='\u{06FF}').contains(&c));

    let lang_guide = if is_persian {
        "All generated output (badge, title, subtitle, items, speaker_notes) MUST be in natural, fluent, elegant Persian (فارسی). Maintain professional tone and high readability."
    } else {
        "Write in concise, professional, keynote-ready English."
    };

    let system_instructions = format!(
        "You are ZenSlides AI, an elite keynote presentation designer and speech coach.\n        {lang_guide}\n        The user wants to revise or generate a slide based on an instruction or action.\n        RULES:\n        1. CRITICAL: NEVER set the user's instruction text itself as the slide title! The instruction is a prompt command (e.g. \"summarize this\", \"make it punchy\", \"convert to numbers\"). The title must always be an actual meaningful headline for the slide topic.\n        2. Intelligently apply the user's instruction to rewrite/update the title, badge, subtitle, items, and speaker_notes.\n        3. You MUST reply with valid JSON ONLY. No markdown fences, no explanatory text, no greeting.\n        JSON schema:\n        {{\n          \"badge\": \"short uppercase category or kicker (1-3 words)\",\n          \"title\": \"concise, impactful slide title (5-10 words)\",\n          \"subtitle\": \"crisp 1-2 sentence explanatory subheadline\",\n          \"items\": [\"Key card or bullet point 1\", \"Key card or bullet point 2\", \"Key card or bullet point 3\"],\n          \"speaker_notes\": \"Natural speech talking points for the presenter to say out loud while showing this slide\"\n        }}"
    );

    let user_msg = format!(
        "Action: {action}\n        User Instruction: {instruction}\n        Current Badge: {badge}\n        Current Title: {title}\n        Current Subtitle: {subtitle}\n        Current Items: {items:?}\n        Target Layout: {target_layout}"
    );

    // Try all candidates dynamically (handles active model, router failover, detected sources)
    let candidates = providers::resolve_candidates(settings);
    let mut raw_text = String::new();

    for candidate in &candidates {
        if candidate.provider == PROVIDER_OPENAI && !candidate.base_url.is_empty() {
            let body = json!({
                "model": candidate.model,
                "temperature": 0.5,
                "messages": [
                    { "role": "system", "content": &system_instructions },
                    { "role": "user", "content": &user_msg }
                ]
            });
            let mut req = client
                .post(format!("{}/chat/completions", candidate.base_url))
                .json(&body);
            if !candidate.api_key.is_empty() {
                req = req.bearer_auth(&candidate.api_key);
            }
            if let Ok(resp) = req.send().await {
                if resp.status().is_success() {
                    if let Ok(res_json) = resp.json::<serde_json::Value>().await {
                        let content = res_json["choices"][0]["message"]["content"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        if !content.trim().is_empty() {
                            info!(
                                "Slide action succeeded with candidate {} ({})",
                                candidate.model, candidate.description
                            );
                            raw_text = content;
                            break;
                        }
                    }
                } else {
                    warn!(
                        "Slide action candidate {} returned status {}",
                        candidate.model,
                        resp.status()
                    );
                }
            }
        } else if candidate.provider == PROVIDER_ANTHROPIC
            && !candidate.base_url.is_empty()
            && !candidate.api_key.is_empty()
        {
            let body = json!({
                "model": candidate.model,
                "max_tokens": 1500,
                "system": &system_instructions,
                "messages": [
                    { "role": "user", "content": &user_msg }
                ]
            });
            if let Ok(resp) = client
                .post(format!("{}/v1/messages", candidate.base_url))
                .header("x-api-key", candidate.api_key.trim())
                .header("anthropic-version", "2023-06-01")
                .json(&body)
                .send()
                .await
            {
                if resp.status().is_success() {
                    if let Ok(res_json) = resp.json::<serde_json::Value>().await {
                        let text = res_json["content"][0]["text"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        if !text.trim().is_empty() {
                            info!(
                                "Slide action succeeded with Anthropic candidate {}",
                                candidate.model
                            );
                            raw_text = text;
                            break;
                        }
                    }
                }
            }
        }
    }

    if raw_text.is_empty() {
        return Err("No response received from any available LLM provider".to_string());
    }

    let json_str = extract_json_object(&raw_text)
        .ok_or_else(|| "Failed to locate JSON object in model response".to_string())?;

    #[derive(Deserialize)]
    struct LlmSlideOut {
        #[serde(default)]
        badge: String,
        #[serde(default)]
        title: String,
        #[serde(default)]
        subtitle: String,
        #[serde(default)]
        items: Vec<String>,
        #[serde(default)]
        speaker_notes: String,
    }

    let parsed: LlmSlideOut =
        serde_json::from_str(json_str).map_err(|e| format!("JSON parse error: {e}"))?;
    let layout = if !target_layout.is_empty() {
        target_layout.to_string()
    } else {
        "bento".to_string()
    };
    let html_snippet = render_slide_html(
        &parsed.badge,
        &parsed.title,
        &parsed.subtitle,
        &parsed.items,
        &parsed.speaker_notes,
        &layout,
    );

    Ok(SlideAiResult {
        badge: parsed.badge,
        title: parsed.title,
        subtitle: parsed.subtitle,
        items: parsed.items,
        speaker_notes: parsed.speaker_notes,
        layout,
        html_snippet,
        source: "llm".to_string(),
    })
}

async fn slide_ai_action(
    State(state): State<AppState>,
    Json(payload): Json<SlideAiPayload>,
) -> impl IntoResponse {
    let settings = state.settings.read().await.clone();

    // Try calling LLM first
    match generate_llm_slide_action(&state.http, &settings, &payload).await {
        Ok(result) => {
            info!(
                "Successfully processed slide AI action '{}' via LLM",
                payload.action
            );
            return Json(result).into_response();
        }
        Err(err) => {
            warn!("LLM slide action fell back to algorithmic: {err}");
        }
    }

    // High quality algorithmic fallback
    let result = generate_algorithmic_slide_action(
        &payload.action,
        payload.instruction.as_deref().unwrap_or_default(),
        payload.title.as_deref().unwrap_or_default(),
        payload.badge.as_deref().unwrap_or_default(),
        payload.subtitle.as_deref().unwrap_or_default(),
        payload.items.as_deref().unwrap_or(&[]),
        payload.target_layout.as_deref().unwrap_or_default(),
    );
    Json(result).into_response()
}

#[derive(Deserialize)]
struct TtsPayload {
    text: String,
    #[serde(default)]
    provider: Option<String>,
    #[serde(default)]
    voice: Option<String>,
    #[serde(default)]
    rate: Option<String>,
    #[serde(default)]
    api_key: Option<String>,
}

async fn tts_synthesize_internal(state: AppState, payload: TtsPayload) -> impl IntoResponse {
    let settings = state.settings.read().await;
    let provider = payload.provider.as_deref().unwrap_or("edge");
    let key = if let Some(k) = payload.api_key.as_deref().filter(|s| !s.trim().is_empty()) {
        Some(k.to_string())
    } else {
        match provider {
            "elevenlabs" => {
                let k = settings.elevenlabs_key();
                if !k.is_empty() {
                    Some(k)
                } else {
                    None
                }
            }
            "openai" => {
                let k = settings.openai_key();
                if !k.is_empty() {
                    Some(k)
                } else {
                    None
                }
            }
            _ => None,
        }
    };
    drop(settings);

    match tts::synthesize(
        Some(provider),
        &payload.text,
        payload.voice.as_deref(),
        payload.rate.as_deref(),
        key.as_deref(),
    )
    .await
    {
        Ok(bytes) => (
            StatusCode::OK,
            [
                (axum::http::header::CONTENT_TYPE, "audio/mpeg"),
                (axum::http::header::CACHE_CONTROL, "public, max-age=86400"),
            ],
            bytes,
        )
            .into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            serde_json::to_string(&json!({ "error": err })).unwrap_or_default(),
        )
            .into_response(),
    }
}

async fn tts_synthesize_post(
    State(state): State<AppState>,
    Json(payload): Json<TtsPayload>,
) -> impl IntoResponse {
    tts_synthesize_internal(state, payload).await
}

async fn tts_synthesize_get(
    State(state): State<AppState>,
    Query(payload): Query<TtsPayload>,
) -> impl IntoResponse {
    tts_synthesize_internal(state, payload).await
}

fn detect_artifact_mode(html: &str) -> &'static str {
    if html.contains("class=\"slide")
        || html.contains("class='slide")
        || html.contains("<section class=\"slide")
        || html.contains("deck-controls")
        || html.contains("Mode: slides")
        || html.contains("slides-deck")
    {
        "slides"
    } else if html.contains("motion-canvas")
        || html.contains("motion-artboard")
        || html.contains("Mode: motion")
    {
        "motion"
    } else {
        "prototype"
    }
}

async fn get_history(State(state): State<AppState>) -> impl IntoResponse {
    let items = history::load_history(&state.workspace);
    let index_path = state.workspace.join("index.html");
    let current_mode = if let Ok(html) = std::fs::read_to_string(&index_path) {
        detect_artifact_mode(&html)
    } else if let Some(first) = items.first() {
        &first.mode
    } else {
        "slides"
    };
    Json(json!({ "items": items, "current_mode": current_mode })).into_response()
}

#[derive(Deserialize)]
struct AddHistoryPayload {
    prompt: String,
    design_system: String,
    mode: String,
    #[serde(default)]
    deep: bool,
    #[serde(default)]
    session_id: Option<String>,
}

async fn post_history(
    State(state): State<AppState>,
    Json(payload): Json<AddHistoryPayload>,
) -> impl IntoResponse {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let sid = payload
        .session_id
        .unwrap_or_else(|| format!("{timestamp}_{}", uuid::Uuid::new_v4()));
    let item = history::add_history_entry(
        &state.workspace,
        sid,
        payload.prompt,
        payload.design_system,
        payload.mode,
        payload.deep,
    );
    Json(json!({ "status": "ok", "item": item })).into_response()
}

async fn delete_history_item(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> impl IntoResponse {
    if let Some(ref current) = *state.current_session_id.read().await {
        if current == &id {
            *state.current_session_id.write().await = None;
        }
    }
    let deleted = history::delete_history_entry(&state.workspace, &id);
    Json(json!({ "status": if deleted { "ok" } else { "not_found" } })).into_response()
}

async fn clear_all_history(State(state): State<AppState>) -> impl IntoResponse {
    *state.current_session_id.write().await = None;
    let _ = history::clear_history(&state.workspace);
    Json(json!({ "status": "ok" })).into_response()
}

async fn activate_history_item(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> impl IntoResponse {
    // 1. Stop any running agent so it does not conflict
    stop_running_agent(&state).await;

    // 2. Track as active session
    *state.current_session_id.write().await = Some(id.clone());

    // 3. Activate session in workspace (will auto-heal index.html if missing)
    match history::activate_session(&state.workspace, &id) {
        Ok(item) => {
            providers::emit_event(
                &state.event_sender,
                "reload",
                "Loaded project from history.",
            );
            (
                StatusCode::OK,
                Json(json!({ "status": "ok", "item": item })),
            )
                .into_response()
        }
        Err(err) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "error", "message": err })),
        )
            .into_response(),
    }
}

async fn new_project(State(state): State<AppState>) -> impl IntoResponse {
    stop_running_agent(&state).await;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let new_id = format!("{timestamp}_{}", uuid::Uuid::new_v4());
    *state.current_session_id.write().await = Some(new_id.clone());

    match history::create_new_project_session(&state.workspace, &new_id) {
        Ok(item) => {
            providers::emit_event(
                &state.event_sender,
                "reload",
                "Created new blank project canvas.",
            );
            (
                StatusCode::OK,
                Json(json!({ "status": "ok", "session_id": new_id, "item": item })),
            )
                .into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": err })),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct SaveArtifactPayload {
    html: String,
}

async fn get_artifact(State(state): State<AppState>) -> impl IntoResponse {
    let index_path = state.workspace.join("index.html");
    match std::fs::read_to_string(&index_path) {
        Ok(html) => {
            let mode = detect_artifact_mode(&html);
            (
                StatusCode::OK,
                Json(json!({ "status": "ok", "html": html, "mode": mode })),
            )
                .into_response()
        }
        Err(err) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "error", "message": err.to_string() })),
        )
            .into_response(),
    }
}

async fn save_artifact(
    State(state): State<AppState>,
    Json(payload): Json<SaveArtifactPayload>,
) -> impl IntoResponse {
    let index_path = state.workspace.join("index.html");
    match std::fs::write(&index_path, &payload.html) {
        Ok(()) => {
            if let Some(ref sid) = *state.current_session_id.read().await {
                let session_file = history::session_artifact_path(&state.workspace, sid);
                let _ = std::fs::write(&session_file, &payload.html);
                let _ = history::mark_has_artifact(&state.workspace, sid);
            }
            providers::emit_event(
                &state.event_sender,
                "reload",
                "Artifact updated via manual slides editor.",
            );
            (StatusCode::OK, Json(json!({ "status": "ok" }))).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": err.to_string() })),
        )
            .into_response(),
    }
}

async fn static_embedded_handler(uri: axum::http::Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    if path.is_empty() {
        path = "index.html".to_string();
    }

    match StudioAssets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            (
                [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                content.data,
            )
                .into_response()
        }
        None => {
            if let Some(content) = StudioAssets::get("index.html") {
                (
                    [(axum::http::header::CONTENT_TYPE, "text/html")],
                    content.data,
                )
                    .into_response()
            } else {
                (StatusCode::NOT_FOUND, "Not Found").into_response()
            }
        }
    }
}

fn now_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

#[derive(Debug, Deserialize)]
struct ExportRequest {
    #[serde(default)]
    seconds: Option<u64>,
}

/// Starts an MP4 recording of the live preview. The response is immediate;
/// progress and completion arrive as `export_status` / `export_done` SSE
/// frames, and the finished file is served by `/api/export/latest`.
async fn start_export(
    State(state): State<AppState>,
    Json(request): Json<ExportRequest>,
) -> impl IntoResponse {
    let seconds = request.seconds.unwrap_or(8).clamp(3, 20);

    if !cfg!(target_os = "linux") {
        return (
            StatusCode::NOT_IMPLEMENTED,
            Json(json!({
                "error": "MP4 export requires Linux/X11 for now. Use PowerPoint or PDF export on this platform."
            })),
        )
            .into_response();
    }

    // Recording without an artifact would only capture Chrome's error page.
    let preview = format!("{}/preview/index.html", state.public_base);
    let has_artifact = state
        .http
        .head(&preview)
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .map(|response| response.status().is_success())
        .unwrap_or(false);
    if !has_artifact {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "There is no artifact to export yet. Generate one first." })),
        )
            .into_response();
    }

    if state.exporting.swap(true, Ordering::SeqCst) {
        return (
            StatusCode::CONFLICT,
            Json(json!({ "error": "An export is already running." })),
        )
            .into_response();
    }

    let sender = state.event_sender.clone();
    let base = state.public_base.clone();
    let directory = state.export_dir.clone();
    let exporting = state.exporting.clone();
    let armed = state.export_armed.clone();
    providers::emit_event(&sender, "export_status", "Preparing the recording browser…");

    tokio::spawn(async move {
        let result = export::record_preview(&base, seconds, &directory, &sender, &armed).await;
        match result {
            Ok(outcome) => providers::emit_event(
                &sender,
                "export_done",
                format!("MP4 ready · {}", export::format_size(outcome.bytes)),
            ),
            Err(error) => providers::emit_event(&sender, "export_error", error),
        }
        armed.store(false, Ordering::SeqCst);
        exporting.store(false, Ordering::SeqCst);
    });

    (
        StatusCode::ACCEPTED,
        Json(json!({ "status": "recording", "seconds": seconds })),
    )
        .into_response()
}

async fn export_latest(State(state): State<AppState>) -> impl IntoResponse {
    let path = state.export_dir.join("preview.mp4");
    match tokio::fs::read(&path).await {
        Ok(bytes) => {
            let mut response = axum::response::Response::new(axum::body::Body::from(bytes));
            let headers = response.headers_mut();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static("video/mp4"),
            );
            headers.insert(
                axum::http::header::CONTENT_DISPOSITION,
                axum::http::HeaderValue::from_static(
                    "attachment; filename=\"zendesign-preview.mp4\"",
                ),
            );
            headers.insert(
                axum::http::header::CACHE_CONTROL,
                axum::http::HeaderValue::from_static("no-store"),
            );
            response
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "No export yet. Record one from the studio first." })),
        )
            .into_response(),
    }
}

const PPTX_EXPORTER_SCRIPT: &str = include_str!("../export_pptx.py");

async fn export_pptx(State(state): State<AppState>) -> impl IntoResponse {
    let workspace = &state.workspace;
    let html_path = workspace.join("index.html");
    if !html_path.exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "There is no presentation to export yet." })),
        )
            .into_response();
    }

    let _ = tokio::fs::create_dir_all(&state.export_dir).await;
    let pptx_path = state.export_dir.join("presentation.pptx");

    let script_path = std::env::temp_dir().join("zen_export_pptx.py");
    if let Err(e) = tokio::fs::write(&script_path, PPTX_EXPORTER_SCRIPT).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Could not prepare pptx exporter script: {e}") })),
        )
            .into_response();
    }

    let Some(python_cmd) = platform::python_binary() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Python 3 was not found. Install Python 3 with pip (python-pptx and beautifulsoup4) to export PowerPoint."
            })),
        )
            .into_response();
    };

    let mut pptx_cmd = tokio::process::Command::new(&python_cmd);
    crate::platform::configure_tokio_command(&mut pptx_cmd);
    let output = pptx_cmd
        .arg(&script_path)
        .arg(&html_path)
        .arg(&pptx_path)
        .output()
        .await;

    match output {
        Ok(out) if out.status.success() => match tokio::fs::read(&pptx_path).await {
            Ok(bytes) => {
                providers::emit_event(
                    &state.event_sender,
                    "activity",
                    "PowerPoint presentation exported (.pptx)",
                );
                let mut response = axum::response::Response::new(axum::body::Body::from(bytes));
                let headers = response.headers_mut();
                headers.insert(
                    axum::http::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static(
                        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
                    ),
                );
                headers.insert(
                    axum::http::header::CONTENT_DISPOSITION,
                    axum::http::HeaderValue::from_static(
                        "attachment; filename=\"presentation.pptx\"",
                    ),
                );
                headers.insert(
                    axum::http::header::CACHE_CONTROL,
                    axum::http::HeaderValue::from_static("no-store"),
                );
                response.into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Could not read generated pptx: {e}") })),
            )
                .into_response(),
        },
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("PPTX export script failed: {stderr}") })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Could not execute python pptx exporter: {e}") })),
        )
            .into_response(),
    }
}

/// A black holding page used while the recorder warms up. It polls
/// `/api/export/armed` and only mounts the artifact once frames are being
/// captured, which is what keeps the video aligned with the first frame.
const STAGE_PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>ZenDesign recording stage</title>
<style>
html, body { margin: 0; height: 100%; background: #000; overflow: hidden; }
iframe { display: block; width: 100vw; height: 100vh; border: 0; }
</style>
</head>
<body>
<script>
(function poll() {
  fetch('/api/export/armed', { cache: 'no-store' })
    .then(function (response) { return response.text(); })
    .then(function (value) {
      if (value.trim() === '1') {
        var frame = document.createElement('iframe');
        frame.src = '/preview/index.html?export=1&t=' + Date.now();
        document.body.appendChild(frame);
      } else {
        setTimeout(poll, 80);
      }
    })
    .catch(function () { setTimeout(poll, 150); });
})();
</script>
</body>
</html>"#;

async fn export_stage() -> impl IntoResponse {
    (
        [(axum::http::header::CACHE_CONTROL, "no-store")],
        axum::response::Html(STAGE_PAGE),
    )
}

async fn export_armed(State(state): State<AppState>) -> impl IntoResponse {
    (
        [(axum::http::header::CACHE_CONTROL, "no-store")],
        if state.export_armed.load(Ordering::SeqCst) {
            "1"
        } else {
            "0"
        },
    )
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    info!("Shutdown signal received, closing daemon");
}

async fn system_version() -> impl IntoResponse {
    Json(updater::get_version_info())
}

async fn system_check_update(State(state): State<AppState>) -> impl IntoResponse {
    match updater::check_for_updates(&state.http).await {
        Ok(info) => (StatusCode::OK, Json(json!({ "status": "ok", "data": info }))).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": err })),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
struct PerformUpdatePayload {
    #[serde(default)]
    download_url: Option<String>,
}

async fn system_perform_update(
    State(state): State<AppState>,
    payload: Option<Json<PerformUpdatePayload>>,
) -> impl IntoResponse {
    let dl = payload.as_ref().and_then(|p| p.download_url.as_deref());
    match updater::perform_update(&state.http, dl).await {
        Ok(res) => (StatusCode::OK, Json(json!({ "status": "ok", "data": res }))).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": err })),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loopback_base_url_handles_wildcards_and_ipv6() {
        assert_eq!(public_base_url("127.0.0.1:7456"), "http://127.0.0.1:7456");
        assert_eq!(public_base_url("0.0.0.0:7456"), "http://127.0.0.1:7456");
        assert_eq!(public_base_url("[::1]:7456"), "http://[::1]:7456");
        assert_eq!(public_base_url("[::]:7456"), "http://127.0.0.1:7456");
    }
}
