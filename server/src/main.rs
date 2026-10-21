mod agent;
mod detector;
mod export;
mod frames;
mod history;
mod providers;
mod settings;
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
use serde::Deserialize;
use serde_json::json;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio_stream::wrappers::{BroadcastStream, IntervalStream};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../studio/dist/"]
struct StudioAssets;

use agent::{AgentEvent, AgentOrchestrator, CustomThemeTokens, PromptRequest, truncate_words};
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

    let workspace = resolve_workspace();
    std::fs::create_dir_all(&workspace)?;
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

    let orchestrator = Arc::new(AgentOrchestrator::new(
        workspace.to_string_lossy().to_string(),
        event_sender.clone(),
        settings.clone(),
        http.clone(),
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
        .route("/api/history", get(get_history).post(post_history).delete(clear_all_history))
        .route("/api/history/:id", delete(delete_history_item))
        .route("/api/history/:id/activate", post(activate_history_item))
        .route("/api/history/new-project", post(new_project))
        .route("/api/themes/generate", post(generate_theme))
        .route("/api/artifact", get(get_artifact).put(save_artifact))
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

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Resolve the workspace folder whether the binary is started from `server/`,
/// from the repository root, or via an installed binary.
fn resolve_workspace() -> PathBuf {
    if let Ok(custom) = std::env::var("ZEN_WORKSPACE_DIR") {
        let path = PathBuf::from(custom);
        if let Err(error) = std::fs::create_dir_all(&path) {
            warn!("Could not create ZEN_WORKSPACE_DIR {:?}: {error}", path);
        }
        return std::fs::canonicalize(&path).unwrap_or(path);
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let exe_ws = parent.join("workspace");
            if exe_ws.is_dir() {
                return std::fs::canonicalize(&exe_ws).unwrap_or(exe_ws);
            }
        }
    }

    let local = PathBuf::from("./workspace");
    if local.is_dir() {
        return std::fs::canonicalize(&local).unwrap_or(local);
    }

    let manifest_relative = Path::new(env!("CARGO_MANIFEST_DIR")).join("../workspace");
    let candidates = [PathBuf::from("../workspace"), manifest_relative];

    for candidate in candidates {
        if candidate.is_dir() {
            return std::fs::canonicalize(&candidate).unwrap_or(candidate);
        }
    }

    let fallback = PathBuf::from("./workspace");
    let _ = std::fs::create_dir_all(&fallback);
    std::fs::canonicalize(&fallback).unwrap_or(fallback)
}

/// Locate the Trunk output regardless of the process working directory.
fn resolve_studio() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let exe_dist = parent.join("dist");
            if exe_dist.is_dir() {
                return std::fs::canonicalize(&exe_dist).unwrap_or(exe_dist);
            }
            let exe_studio = parent.join("studio");
            if exe_studio.is_dir() {
                return std::fs::canonicalize(&exe_studio).unwrap_or(exe_studio);
            }
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

    let base = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))
        .unwrap_or_else(std::env::temp_dir);
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
        if !resolved.api_key.is_empty() {
            candidate.openai.api_key = resolved.api_key.clone();
        }
        if let Some(m) = resolved.model.clone() {
            candidate.openai.model = m;
        }
    } else if resolved.provider == settings::PROVIDER_ANTHROPIC {
        candidate.anthropic.base_url = resolved.base_url.clone();
        if !resolved.api_key.is_empty() {
            candidate.anthropic.api_key = resolved.api_key.clone();
        }
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
        if let Ok(live) = providers::list_models(&state.http, &candidate, &candidate.provider).await {
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
    let base = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .map(|home| Path::new(&home).join(".config"))
        });
    let Some(base) = base else {
        return Err("Could not locate the OpenCode configuration directory.".to_string());
    };
    let directory = base.join("opencode");

    let mut path = None;
    for name in ["opencode.json", "opencode.jsonc"] {
        let candidate = directory.join(name);
        if candidate.is_file() {
            path = Some(candidate);
            break;
        }
    }
    let path = path.ok_or_else(|| {
        format!(
            "No OpenCode configuration found in {}.",
            directory.display()
        )
    })?;

    let raw = std::fs::read_to_string(&path)
        .map_err(|error| format!("Could not read {}: {error}", path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|error| format!("Could not parse {}: {error}", path.display()))?;

    let providers = value
        .get("provider")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| format!("{} has no provider entries.", path.display()))?;

    let chosen = providers
        .get_key_value("9router")
        .or_else(|| {
            providers.iter().find(|(_, entry)| {
                entry
                    .pointer("/options/baseURL")
                    .or_else(|| entry.pointer("/options/baseUrl"))
                    .is_some()
            })
        })
        .ok_or_else(|| "No OpenCode provider with options.baseURL was found.".to_string())?;

    let (name, entry) = chosen;
    let options = entry
        .get("options")
        .ok_or_else(|| format!("Provider '{name}' has no options."))?;

    let base_url = options
        .get("baseURL")
        .or_else(|| options.get("baseUrl"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .trim()
        .trim_end_matches('/')
        .to_string();
    if base_url.is_empty() {
        return Err(format!("Provider '{name}' has an empty base URL."));
    }

    let api_key = options
        .get("apiKey")
        .and_then(serde_json::Value::as_str)
        .map(resolve_secret)
        .unwrap_or_default();

    let mut models: Vec<String> = entry
        .get("models")
        .and_then(serde_json::Value::as_object)
        .map(|models| models.keys().cloned().collect())
        .unwrap_or_default();
    models.sort();

    Ok(ImportedProvider {
        name: name.clone(),
        base_url,
        api_key,
        models,
    })
}

fn resolve_secret(raw: &str) -> String {
    let trimmed = raw.trim();
    if let Some(name) = trimmed
        .strip_prefix("{env:")
        .and_then(|rest| rest.strip_suffix('}'))
    {
        std::env::var(name).unwrap_or_default()
    } else {
        trimmed.to_string()
    }
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
    models.first().cloned().unwrap_or_else(|| "ag/gemini-3.8-flash-high".to_string())
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

/// Accepts a brief, then runs the orchestrator in the background. The HTTP
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
        .unwrap_or_else(|| format!("{timestamp}_{}", now_millis() % 10000));
    request.session_id = Some(session_id.clone());

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
            info!("Generated theme via LLM provider ({}): {}", settings.provider, theme.name);
            return (StatusCode::OK, Json(json!({ "theme": theme, "source": "llm" }))).into_response();
        }
    }

    // Fallback to high-speed built-in algorithmic engine
    let theme = generate_algorithmic_theme(prompt);
    (StatusCode::OK, Json(json!({ "theme": theme, "source": "algorithmic" }))).into_response()
}

async fn generate_llm_theme(
    client: &reqwest::Client,
    settings: &Settings,
    prompt: &str,
) -> Result<CustomThemeTokens, String> {
    let is_persian = prompt.chars().any(|c| ('\u{0600}'..='\u{06FF}').contains(&c));
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

    let raw_text = match settings.provider.as_str() {
        PROVIDER_OPENAI => {
            let base = settings.openai.base_url.trim().trim_end_matches('/');
            let model = settings.openai.model.trim();
            if base.is_empty() || model.is_empty() {
                return Err("OpenAI provider unconfigured".to_string());
            }
            let key = settings.openai_key();
            let body = json!({
                "model": model,
                "temperature": 0.5,
                "messages": [
                    { "role": "system", "content": system_instructions },
                    { "role": "user", "content": format!("Theme Prompt: {prompt}") }
                ]
            });
            let mut req = client.post(format!("{base}/chat/completions")).json(&body);
            if !key.is_empty() {
                req = req.bearer_auth(key);
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            if !resp.status().is_success() {
                return Err(format!("OpenAI returned status {}", resp.status()));
            }
            let res_json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
            res_json["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or_default()
                .to_string()
        }
        PROVIDER_ANTHROPIC => {
            let base = settings.anthropic.base_url.trim().trim_end_matches('/');
            let model = if settings.anthropic.model.trim().is_empty() {
                "claude-opus-4-6"
            } else {
                settings.anthropic.model.trim()
            };
            let key = settings.anthropic_key();
            if key.is_empty() {
                return Err("Anthropic API key missing".to_string());
            }
            let body = json!({
                "model": model,
                "max_tokens": 1024,
                "system": system_instructions,
                "messages": [
                    { "role": "user", "content": format!("Theme Prompt: {prompt}") }
                ]
            });
            let resp = client
                .post(format!("{base}/v1/messages"))
                .header("x-api-key", key.trim())
                .header("anthropic-version", "2023-06-01")
                .json(&body)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            if !resp.status().is_success() {
                return Err(format!("Anthropic returned status {}", resp.status()));
            }
            let res_json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
            res_json["content"][0]["text"]
                .as_str()
                .unwrap_or_default()
                .to_string()
        }
        _ => return Err("Unsupported provider for theme generation".to_string()),
    };

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
    let swatch = format!("linear-gradient(135deg, {} 0%, {} 100%)", parsed.accent, parsed.bg);

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

    let is_persian = prompt.chars().any(|c| ('\u{0600}'..='\u{06FF}').contains(&c));
    let default_font = if is_persian {
        "'Vazirmatn', system-ui, sans-serif".to_string()
    } else {
        "'Inter', system-ui, sans-serif".to_string()
    };

    // Insurance / Banking / Finance / سرمایه / بانک / بیمه / مالی / اعتباری (Corporate Trust Navy & Royal Blue)
    if lower.contains("بیمه") || lower.contains("بانک") || lower.contains("مالی") || lower.contains("سرمایه") || lower.contains("اعتبار") || lower.contains("insurance") || lower.contains("bank") || lower.contains("finance") || lower.contains("fintech") {
        let is_light = lower.contains("روشن") || lower.contains("سفید") || lower.contains("light");
        let (bg, card, border, text, accent, dot) = if is_light {
            ("#f8fafc", "#ffffff", "#cbd5e1", "#0f172a", "#1d4ed8", "#2563eb")
        } else {
            ("#0b1329", "#111c38", "#1e293b", "#f1f5f9", "#3b82f6", "#60a5fa")
        };
        CustomThemeTokens {
            name: if is_persian { "بیمه و اعتماد سازمانی / Corporate Trust".to_string() } else { "Corporate Trust (Insurance & Finance)".to_string() },
            bg: bg.to_string(),
            text: text.to_string(),
            accent: accent.to_string(),
            font_family: default_font,
            card_bg: card.to_string(),
            border_color: border.to_string(),
            dot: dot.to_string(),
            swatch: format!("linear-gradient(135deg, {accent} 0%, {bg} 100%)"),
        }
    } else if lower.contains("پزشکی") || lower.contains("درمان") || lower.contains("سلامت") || lower.contains("دارو") || lower.contains("بیمارستان") || lower.contains("medical") || lower.contains("health") || lower.contains("clinic") || lower.contains("pharma") {
        let is_light = lower.contains("روشن") || lower.contains("سفید") || lower.contains("light");
        let (bg, card, border, text, accent, dot) = if is_light {
            ("#f0fdfa", "#ffffff", "#99f6e4", "#134e4a", "#0d9488", "#14b8a6")
        } else {
            ("#041f1e", "#072e2d", "#115e59", "#f0fdfa", "#14b8a6", "#2dd4bf")
        };
        CustomThemeTokens {
            name: if is_persian { "پزشکی و سلامت / Clinical Teal".to_string() } else { "Clinical Teal (Health & Medical)".to_string() },
            bg: bg.to_string(),
            text: text.to_string(),
            accent: accent.to_string(),
            font_family: default_font,
            card_bg: card.to_string(),
            border_color: border.to_string(),
            dot: dot.to_string(),
            swatch: format!("linear-gradient(135deg, {accent} 0%, {bg} 100%)"),
        }
    } else if lower.contains("املاک") || lower.contains("ساختمان") || lower.contains("معماری") || lower.contains("سازه") || lower.contains("real estate") || lower.contains("architecture") || lower.contains("interior") {
        CustomThemeTokens {
            name: if is_persian { "معماری و املاک / Bronze Estate".to_string() } else { "Bronze Estate".to_string() },
            bg: "#141210".to_string(),
            text: "#f5f5f4".to_string(),
            accent: "#d97706".to_string(),
            font_family: default_font,
            card_bg: "#211d19".to_string(),
            border_color: "#443c33".to_string(),
            dot: "#f59e0b".to_string(),
            swatch: "linear-gradient(135deg, #d97706 0%, #141210 100%)".to_string(),
        }
    } else if lower.contains("کافه") || lower.contains("قهوه") || lower.contains("رستوران") || lower.contains("غذا") || lower.contains("فود") || lower.contains("cafe") || lower.contains("coffee") || lower.contains("restaurant") {
        CustomThemeTokens {
            name: if is_persian { "کافه و قهوه / Warm Espresso".to_string() } else { "Warm Espresso".to_string() },
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
    } else if lower.contains("قرمز") || lower.contains("سرخ") || lower.contains("زرشکی") || lower.contains("red") || lower.contains("crimson") || lower.contains("ruby") || lower.contains("blood") || lower.contains("scarlet") {
        let is_dark = lower.contains("مشکی") || lower.contains("سیاه") || lower.contains("تیره") || lower.contains("dark") || lower.contains("black") || !lower.contains("سفید");
        let (bg, card, border, text, accent) = if is_dark {
            ("#0a0a0c", "#161114", "#38171e", "#fce7f3", "#ef4444")
        } else {
            ("#fff5f5", "#ffffff", "#fed7d7", "#1a202c", "#e53e3e")
        };
        CustomThemeTokens {
            name: if is_persian { "قرمز شفق / Crimson Dark".to_string() } else { "Crimson Void".to_string() },
            bg: bg.to_string(),
            text: text.to_string(),
            accent: accent.to_string(),
            font_family: default_font,
            card_bg: card.to_string(),
            border_color: border.to_string(),
            dot: accent.to_string(),
            swatch: format!("linear-gradient(135deg, {accent} 0%, {bg} 100%)"),
        }
    } else if lower.contains("cyber") || lower.contains("neon") || lower.contains("arcade") || lower.contains("matrix") || lower.contains("نئون") || lower.contains("سایبر") {
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
    } else if lower.contains("forest") || lower.contains("moss") || lower.contains("pine") || lower.contains("nature") || lower.contains("green") || lower.contains("سبز") || lower.contains("جنگل") || lower.contains("طبیعت") {
        CustomThemeTokens {
            name: if is_persian { "سبز نوردیک / Nordic Pine".to_string() } else { "Nordic Pine".to_string() },
            bg: "#0a1410".to_string(),
            text: "#ecfdf5".to_string(),
            accent: "#10b981".to_string(),
            font_family: default_font,
            card_bg: "#12241d".to_string(),
            border_color: "#1f4034".to_string(),
            dot: "#10b981".to_string(),
            swatch: "linear-gradient(135deg, #10b981 0%, #0a1410 100%)".to_string(),
        }
    } else if lower.contains("sunset") || lower.contains("warm") || lower.contains("coral") || lower.contains("autumn") || lower.contains("پاییز") || lower.contains("نارنجی") || lower.contains("آفتاب") {
        CustomThemeTokens {
            name: if is_persian { "غروب پاییزی / Sunset Ember".to_string() } else { "Sunset Ember".to_string() },
            bg: "#1f0d14".to_string(),
            text: "#fff1f2".to_string(),
            accent: "#f97316".to_string(),
            font_family: default_font,
            card_bg: "#321620".to_string(),
            border_color: "#582436".to_string(),
            dot: "#f97316".to_string(),
            swatch: "linear-gradient(135deg, #f97316 0%, #1f0d14 100%)".to_string(),
        }
    } else if lower.contains("gold") || lower.contains("luxury") || lower.contains("champagne") || lower.contains("royal") || lower.contains("طلا") || lower.contains("لوکس") || lower.contains("سلطنتی") || lower.contains("زرد") {
        CustomThemeTokens {
            name: if is_persian { "طلایی لوکس / Imperial Obsidian".to_string() } else { "Imperial Obsidian".to_string() },
            bg: "#0a0a0c".to_string(),
            text: "#faf5ef".to_string(),
            accent: "#eab308".to_string(),
            font_family: default_font,
            card_bg: "#16161c".to_string(),
            border_color: "#caa23440".to_string(),
            dot: "#eab308".to_string(),
            swatch: "linear-gradient(135deg, #eab308 0%, #0a0a0c 100%)".to_string(),
        }
    } else if lower.contains("ocean") || lower.contains("sea") || lower.contains("aqua") || lower.contains("blue") || lower.contains("دریا") || lower.contains("آبی") || lower.contains("اقیانوس") {
        CustomThemeTokens {
            name: if is_persian { "آبی اقیانوس / Abyssal Azure".to_string() } else { "Abyssal Azure".to_string() },
            bg: "#05111e".to_string(),
            text: "#f0f9ff".to_string(),
            accent: "#0284c7".to_string(),
            font_family: default_font,
            card_bg: "#0b2038".to_string(),
            border_color: "#133860".to_string(),
            dot: "#0284c7".to_string(),
            swatch: "linear-gradient(135deg, #0284c7 0%, #05111e 100%)".to_string(),
        }
    } else if lower.contains("purple") || lower.contains("violet") || lower.contains("lavender") || lower.contains("بنفش") || lower.contains("ارغوانی") {
        CustomThemeTokens {
            name: if is_persian { "بنفش کیهانی / Velvet Nebula".to_string() } else { "Velvet Nebula".to_string() },
            bg: "#120a22".to_string(),
            text: "#f5f3ff".to_string(),
            accent: "#a855f7".to_string(),
            font_family: default_font,
            card_bg: "#1d1236".to_string(),
            border_color: "#341f5e".to_string(),
            dot: "#a855f7".to_string(),
            swatch: "linear-gradient(135deg, #a855f7 0%, #120a22 100%)".to_string(),
        }
    } else if lower.contains("minimal") || lower.contains("light") || lower.contains("white") || lower.contains("paper") || lower.contains("سفید") || lower.contains("روشن") {
        CustomThemeTokens {
            name: if is_persian { "سفید مینیمال / Minimal White".to_string() } else { "Atelier Paper".to_string() },
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
        let hash: u64 = prompt.bytes().fold(5381, |acc, b| acc.wrapping_mul(33).wrapping_add(b as u64));
        let hue = (hash % 360) as f64;
        let s = 0.85;
        let l = 0.58;
        let c = (1.0 - (2.0 * l - 1.0_f64).abs()) * s;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0_f64).abs());
        let m = l - c / 2.0;
        let (r1, g1, b1) = if hue < 60.0 { (c, x, 0.0) }
            else if hue < 120.0 { (x, c, 0.0) }
            else if hue < 180.0 { (0.0, c, x) }
            else if hue < 240.0 { (0.0, x, c) }
            else if hue < 300.0 { (x, 0.0, c) }
            else { (c, 0.0, x) };
        let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let accent = format!("#{r:02x}{g:02x}{b:02x}");
        let bg = "#0a0d14".to_string();
        let swatch = format!("linear-gradient(135deg, {accent} 0%, {bg} 100%)");

        let display_name = if is_persian {
            format!("تم اختصاصی ({})", truncate_words(prompt, 18).trim_end_matches('…'))
        } else {
            format!("Custom ({})", truncate_words(prompt, 18).trim_end_matches('…'))
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

async fn get_history(State(state): State<AppState>) -> impl IntoResponse {
    let items = history::load_history(&state.workspace);
    Json(json!({ "items": items })).into_response()
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
        .unwrap_or_else(|| format!("{timestamp}_{}", now_millis() % 10000));
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
    let deleted = history::delete_history_entry(&state.workspace, &id);
    Json(json!({ "status": if deleted { "ok" } else { "not_found" } })).into_response()
}

async fn clear_all_history(State(state): State<AppState>) -> impl IntoResponse {
    let _ = history::clear_history(&state.workspace);
    Json(json!({ "status": "ok" })).into_response()
}

async fn activate_history_item(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> impl IntoResponse {
    match history::activate_session(&state.workspace, &id) {
        Ok(item) => {
            providers::emit_event(
                &state.event_sender,
                "reload",
                "Loaded project from history.",
            );
            (StatusCode::OK, Json(json!({ "status": "ok", "item": item }))).into_response()
        }
        Err(err) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "error", "message": err })),
        )
            .into_response(),
    }
}

async fn new_project(State(state): State<AppState>) -> impl IntoResponse {
    match history::create_new_project(&state.workspace) {
        Ok(()) => {
            providers::emit_event(
                &state.event_sender,
                "reload",
                "Created new blank project canvas.",
            );
            (StatusCode::OK, Json(json!({ "status": "ok" }))).into_response()
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
        Ok(html) => (
            StatusCode::OK,
            Json(json!({ "status": "ok", "html": html })),
        )
            .into_response(),
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
    match std::fs::write(&index_path, payload.html) {
        Ok(()) => {
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
    let workspace = resolve_workspace();
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

    let python_cmd = if cfg!(windows) { "python" } else { "python3" };

    let output = tokio::process::Command::new(python_cmd)
        .arg(&script_path)
        .arg(&html_path)
        .arg(&pptx_path)
        .output()
        .await;

    match output {
        Ok(out) if out.status.success() => {
            match tokio::fs::read(&pptx_path).await {
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
            }
        }
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
