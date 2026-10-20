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

use agent::{AgentEvent, AgentOrchestrator, PromptRequest};
use settings::{Settings, KNOWN_PROVIDERS, PROVIDER_OPENAI};

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
        // Serve the agent's live output. ServeDir handles traversal protection
        // and content types; index.html is served for `/preview/`.
        .nest_service(
            "/preview",
            ServeDir::new(&workspace).append_index_html_on_directories(true),
        )
        // The studio itself is a Rust/WASM bundle compiled by Trunk. Axum
        // serves it from the same origin as the API, so SSE and the preview
        // iframe need no proxy or CORS exceptions. No CORS layer: the studio is
        // same-origin, and a permissive one would let any website reach this
        // daemon's settings and model credentials.
        .fallback_service(ServeDir::new(&studio).append_index_html_on_directories(true))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

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

    let manifest_relative = Path::new(env!("CARGO_MANIFEST_DIR")).join("../workspace");
    let candidates = [PathBuf::from("../workspace"), manifest_relative];

    for candidate in candidates {
        if candidate.is_dir() {
            return std::fs::canonicalize(&candidate).unwrap_or(candidate);
        }
    }

    let fallback = PathBuf::from("../workspace");
    let _ = std::fs::create_dir_all(&fallback);
    std::fs::canonicalize(&fallback).unwrap_or(fallback)
}

/// Locate the Trunk output regardless of the process working directory.
fn resolve_studio() -> PathBuf {
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

    let mut models = if !specific_models.is_empty() {
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
    Json(request): Json<PromptRequest>,
) -> impl IntoResponse {
    if request.prompt.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "prompt must not be empty" })),
        )
            .into_response();
    }

    let orchestrator = state.orchestrator.clone();
    let sender = state.event_sender.clone();
    let lock = state.agent_lock.clone();
    let design_system = request.design_system.clone();
    let mode = request.mode.clone();
    let deep = request.deep;

    info!(
        "Accepted prompt (design_system={design_system}, mode={mode}, deep={deep}, len={})",
        request.prompt.len()
    );

    history::add_history_entry(
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
            "design_system": design_system,
            "mode": mode,
            "deep": deep,
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

async fn get_history() -> impl IntoResponse {
    let items = history::load_history();
    Json(json!({ "items": items })).into_response()
}

#[derive(Deserialize)]
struct AddHistoryPayload {
    prompt: String,
    design_system: String,
    mode: String,
    #[serde(default)]
    deep: bool,
}

async fn post_history(Json(payload): Json<AddHistoryPayload>) -> impl IntoResponse {
    let item = history::add_history_entry(
        payload.prompt,
        payload.design_system,
        payload.mode,
        payload.deep,
    );
    Json(json!({ "status": "ok", "item": item })).into_response()
}

async fn delete_history_item(AxumPath(id): AxumPath<String>) -> impl IntoResponse {
    let deleted = history::delete_history_entry(&id);
    Json(json!({ "status": if deleted { "ok" } else { "not_found" } })).into_response()
}

async fn clear_all_history() -> impl IntoResponse {
    let _ = history::clear_history();
    Json(json!({ "status": "ok" })).into_response()
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
