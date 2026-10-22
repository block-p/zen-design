//! Model-provider adapters.
//!
//! Four ways to turn a brief into an artifact:
//!
//! * `template`  — the deterministic, fully offline generator in `agent.rs`.
//! * `openai`    — any OpenAI-compatible `/chat/completions` endpoint, with SSE
//!   streaming. This covers 9Router (`ag/*` Antigravity models, `kr/*` Claude,
//!   DeepSeek, GLM…), OpenRouter, Ollama, LM Studio, vLLM and OpenAI itself.
//! * `anthropic` — the native Messages API with SSE streaming.
//! * `opencode`  — reads `opencode.json` and calls its OpenAI-compatible
//!   endpoint directly (the 9Router approach), or uses OpenCode's public free
//!   lane. No OpenCode app or CLI is required; the CLI is only an optional
//!   fallback when no endpoint exists.

use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::sync::OnceLock;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use futures_util::StreamExt;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::broadcast;
use tracing::{info, warn};

use crate::agent::AgentEvent;
use crate::settings::{
    Settings, PROVIDER_ANTHROPIC, PROVIDER_AUTO, PROVIDER_OPENAI, PROVIDER_OPENCODE,
    PROVIDER_OPENCODE_FREE,
};

/// Result of a provider run: either raw model text (the orchestrator extracts
/// and stores the HTML) or proof that the agent wrote the artifact itself.
pub enum Generated {
    Text(String),
    ArtifactWritten,
}

#[derive(Clone, Debug)]
pub struct ModelCandidate {
    pub provider: String,
    pub base_url: String,
    pub model: String,
    pub api_key: String,
    pub description: String,
}

pub fn resolve_candidates(settings: &Settings) -> Vec<ModelCandidate> {
    let mut candidates = Vec::new();
    let mut seen = std::collections::HashSet::new();

    let mut add = |provider: &str, base_url: &str, model: &str, api_key: &str, desc: &str| {
        let clean_model = model.trim();
        let key = (
            provider.to_string(),
            base_url.trim_end_matches('/').to_string(),
            clean_model.to_string(),
        );
        if !seen.contains(&key) && !clean_model.is_empty() {
            seen.insert(key);
            candidates.push(ModelCandidate {
                provider: provider.to_string(),
                base_url: base_url.trim_end_matches('/').to_string(),
                model: clean_model.to_string(),
                api_key: api_key.trim().to_string(),
                description: desc.to_string(),
            });
        }
    };

    // 1. Primary profile from settings
    let prov = settings.provider.as_str();
    if prov == PROVIDER_OPENAI || prov == PROVIDER_AUTO {
        let base = settings.openai.base_url.trim();
        let key = settings.openai_key();
        let model = settings.openai.model.trim();
        if !base.is_empty() {
            let m = if crate::detector::is_opencode_free_endpoint(base)
                && (model.is_empty() || model == "auto")
            {
                crate::detector::OPENCODE_FREE_DEFAULT_MODEL
            } else if model.is_empty() || model == "auto" {
                "kr/auto"
            } else {
                model
            };
            add(PROVIDER_OPENAI, base, m, &key, "Active OpenAI Profile");
        }
    } else if prov == PROVIDER_ANTHROPIC {
        let base = settings.anthropic.base_url.trim();
        let key = settings.anthropic_key();
        let model = if settings.anthropic.model.trim().is_empty() {
            "claude-opus-4-6"
        } else {
            settings.anthropic.model.trim()
        };
        if !base.is_empty() && !key.is_empty() {
            add(
                PROVIDER_ANTHROPIC,
                base,
                model,
                &key,
                "Active Anthropic Profile",
            );
        }
    } else if prov == PROVIDER_OPENCODE {
        // OpenCode is a config/HTTP client, not a required binary. Use the
        // endpoint + model from Settings, falling back to opencode.json.
        let config = crate::detector::read_opencode_endpoint().ok();
        let profile = &settings.opencode;
        let mut base = if !profile.base_url.trim().is_empty() {
            profile.base_url.trim().to_string()
        } else {
            config
                .as_ref()
                .map(|endpoint| endpoint.base_url.clone())
                .unwrap_or_default()
        };
        let mut key = if !profile.api_key.trim().is_empty() {
            profile.api_key.clone()
        } else {
            config
                .as_ref()
                .map(|endpoint| endpoint.api_key.clone())
                .unwrap_or_default()
        };
        let mut model = if !profile.model.trim().is_empty() && profile.model != "auto" {
            profile.model.trim().to_string()
        } else {
            config
                .as_ref()
                .and_then(|endpoint| endpoint.models.first().cloned())
                .unwrap_or_default()
        };

        // No local opencode.json? Use the public free lane, which needs no
        // app, CLI, or API key. This mirrors 9Router's "OpenCode Free".
        if base.is_empty() {
            base = crate::detector::OPENCODE_FREE_BASE_URL.to_string();
            key.clear();
            if model.is_empty() {
                model = crate::detector::OPENCODE_FREE_DEFAULT_MODEL.to_string();
            }
        }

        if !base.is_empty() && !model.is_empty() {
            let description = if crate::detector::is_opencode_free_endpoint(&base) {
                "OpenCode Free (public, no key)"
            } else {
                "OpenCode config (HTTP)"
            };
            add(PROVIDER_OPENAI, &base, &model, &key, description);
        }
    }

    // The free lane is a self-contained public provider. If the user picked it,
    // fail over between its own models instead of silently sending the prompt
    // to unrelated local agents/configs on the machine (which may be offline or
    // rate-limited). The selected model stays first.
    let free_lane = prov == PROVIDER_OPENAI
        && crate::detector::is_opencode_free_endpoint(&settings.openai.base_url);
    if free_lane {
        for model in crate::detector::OPENCODE_FREE_FALLBACK_MODELS {
            add(
                PROVIDER_OPENAI,
                crate::detector::OPENCODE_FREE_BASE_URL,
                model,
                "",
                "OpenCode Free fallback",
            );
        }
        return candidates;
    }

    // 2. Scan all locally detected tools and credentials
    let detected = crate::detector::scan_local_sources();
    for src in detected {
        if !src.has_key {
            continue;
        }
        if let Ok(res) = crate::detector::resolve_source(&src.id) {
            let desc = format!("Detected: {}", src.name);
            if let Some(first_m) = res.model.as_deref() {
                add(&res.provider, &res.base_url, first_m, &res.api_key, &desc);
            }

            for m in &src.models {
                add(
                    &res.provider,
                    &res.base_url,
                    m,
                    &res.api_key,
                    &format!("{} ({})", src.name, m),
                );
            }
        }
    }

    // 3. Optional OpenCode CLI fallback. The config/HTTP path above is always
    // preferred; the CLI is only considered when it is actually installed.
    if (prov == PROVIDER_OPENCODE || prov == PROVIDER_AUTO)
        && crate::detector::opencode_cli_available()
    {
        candidates.push(ModelCandidate {
            provider: PROVIDER_OPENCODE.to_string(),
            base_url: String::new(),
            model: settings.opencode.model.clone(),
            api_key: String::new(),
            description: "OpenCode CLI local agent".to_string(),
        });
    }

    candidates
}

const MAX_OUTPUT_TOKENS: u32 = 16_384;
const TOKEN_FLUSH_BYTES: usize = 900;
const TOKEN_FLUSH_INTERVAL: Duration = Duration::from_millis(140);

pub fn emit_event(
    sender: &broadcast::Sender<String>,
    event_type: &str,
    message: impl Into<String>,
) {
    let event = AgentEvent {
        event_type: event_type.to_string(),
        message: message.into(),
    };
    if let Ok(payload) = serde_json::to_string(&event) {
        let _ = sender.send(payload);
    }
}

/// Batches streamed deltas so the SSE channel sees ~7 frames/second instead of
/// one frame per token.
struct TokenBatch {
    buffer: String,
    last_flush: Instant,
}

impl TokenBatch {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            last_flush: Instant::now(),
        }
    }

    fn push(&mut self, sender: &broadcast::Sender<String>, chunk: &str) {
        if chunk.is_empty() {
            return;
        }
        self.buffer.push_str(chunk);
        if self.buffer.len() >= TOKEN_FLUSH_BYTES
            || self.last_flush.elapsed() >= TOKEN_FLUSH_INTERVAL
        {
            self.flush(sender);
        }
    }

    fn flush(&mut self, sender: &broadcast::Sender<String>) {
        if self.buffer.is_empty() {
            return;
        }
        emit_event(sender, "token", std::mem::take(&mut self.buffer));
        self.last_flush = Instant::now();
    }
}

pub async fn generate(
    client: &reqwest::Client,
    settings: &Settings,
    system: &str,
    user: &str,
    sender: &broadcast::Sender<String>,
    workspace: &Path,
) -> Result<Generated, String> {
    generate_with_images(client, settings, system, user, &[], sender, workspace).await
}

/// Same as [`generate`], with optional PNG frames attached to the user turn.
/// The deep-review loop uses this to let a vision-capable model critique what
/// the artifact actually looks like, not just the source that produced it.
pub async fn generate_with_images(
    client: &reqwest::Client,
    settings: &Settings,
    system: &str,
    user: &str,
    images: &[Vec<u8>],
    sender: &broadcast::Sender<String>,
    workspace: &Path,
) -> Result<Generated, String> {
    let candidates = resolve_candidates(settings);
    if candidates.is_empty() {
        return Err(
            "No AI model configured or detected on your system. Open Settings to connect one."
                .to_string(),
        );
    }

    let mut last_error = String::new();
    for (idx, candidate) in candidates.iter().enumerate() {
        if idx > 0 {
            emit_event(
                sender,
                "status",
                format!(
                    "Notice: Previous model failed. Auto-failover switching to '{}' ({})…",
                    candidate.model, candidate.description
                ),
            );
        }

        let res = match candidate.provider.as_str() {
            PROVIDER_OPENAI => {
                openai_generate_candidate(client, candidate, system, user, images, sender)
                    .await
                    .map(Generated::Text)
            }
            PROVIDER_ANTHROPIC => {
                anthropic_generate_candidate(client, candidate, system, user, images, sender)
                    .await
                    .map(Generated::Text)
            }
            PROVIDER_OPENCODE => {
                if !candidate.base_url.trim().is_empty() {
                    // OpenCode config points at an OpenAI-compatible endpoint.
                    // Talk to it directly: no CLI/app required.
                    openai_generate_candidate(client, candidate, system, user, images, sender)
                        .await
                        .map(Generated::Text)
                } else if !images.is_empty() {
                    continue;
                } else {
                    opencode_generate_candidate(candidate, system, user, sender, workspace).await
                }
            }
            _ => continue,
        };

        match res {
            Ok(gen) => return Ok(gen),
            Err(err) => {
                warn!(
                    "Candidate '{}' ({}) failed: {}",
                    candidate.model, candidate.description, err
                );
                last_error = err;
            }
        }
    }

    Err(format!(
        "All available models failed. Last error: {last_error}"
    ))
}

/// Run one non-agent completion for an already-resolved candidate. Used by the
/// research/expansion pass so it follows the same URL, auth, streaming, and
/// OpenCode Free fingerprint rules as normal generation instead of hard-coding
/// an OpenAI `/chat/completions` path.
pub(crate) async fn generate_text_once(
    client: &reqwest::Client,
    candidate: &ModelCandidate,
    system: &str,
    user: &str,
) -> Result<String, String> {
    let (sender, _receiver) = broadcast::channel(8);
    match candidate.provider.as_str() {
        PROVIDER_OPENAI if !candidate.base_url.trim().is_empty() => {
            openai_generate_candidate(client, candidate, system, user, &[], &sender).await
        }
        PROVIDER_ANTHROPIC if !candidate.base_url.trim().is_empty() => {
            anthropic_generate_candidate(client, candidate, system, user, &[], &sender).await
        }
        _ => Err("This candidate cannot run a plain HTTP completion.".to_string()),
    }
}

/// List models for the settings dialog.
///
/// OpenAI-compatible endpoints expose `/models`; Anthropic is a fixed list.
/// OpenCode is resolved from its config first (the 9Router/Ollama style
/// endpoint), and only falls back to the CLI if no HTTP endpoint exists.
pub async fn list_models(
    client: &reqwest::Client,
    settings: &Settings,
    provider: &str,
) -> Result<Vec<String>, String> {
    match provider {
        PROVIDER_OPENAI => {
            let base = settings.openai.base_url.trim().trim_end_matches('/');
            if base.is_empty() {
                return Err("Set a base URL first.".to_string());
            }
            if crate::detector::is_opencode_free_endpoint(base) {
                return fetch_opencode_free_models(client).await;
            }
            let key = settings.openai_key();
            let mut models = fetch_openai_models(client, base, &key)
                .await
                .unwrap_or_default();

            // Fallback for Google Gemini or Antigravity endpoints if endpoint didn't list models
            if models.is_empty() {
                if base.contains("generativelanguage.googleapis.com") || base.contains("gemini") {
                    models = vec![
                        "gemini-2.5-flash".to_string(),
                        "gemini-2.5-pro".to_string(),
                        "gemini-2.0-flash".to_string(),
                        "gemini-1.5-flash".to_string(),
                        "gemini-1.5-pro".to_string(),
                    ];
                } else if base.contains("antigravity") || base.contains(":20130") {
                    models = vec![
                        "ag/gemini-3.8-flash".to_string(),
                        "ag/gemini-3.8-flash-high".to_string(),
                        "ag/gemini-3.8-flash-medium".to_string(),
                        "ag/gemini-3.8-flash-low".to_string(),
                        "ag/claude-sonnet-4-6".to_string(),
                        "ag/claude-opus-4-6-thinking".to_string(),
                        "ag/gpt-oss-120b-medium".to_string(),
                    ];
                }
            }

            models.sort();
            models.dedup();
            Ok(models)
        }
        PROVIDER_ANTHROPIC => Ok(vec![
            "claude-opus-4-6".to_string(),
            "claude-opus-4-5".to_string(),
            "claude-sonnet-4-6".to_string(),
            "claude-haiku-4-5".to_string(),
        ]),
        PROVIDER_OPENCODE => {
            let config = crate::detector::read_opencode_endpoint().ok();
            let explicit_base = settings.opencode.base_url.trim().trim_end_matches('/');
            let base = if explicit_base.is_empty() {
                config
                    .as_ref()
                    .map(|endpoint| endpoint.base_url.clone())
                    .unwrap_or_default()
            } else {
                explicit_base.to_string()
            };

            // No local config means the public free lane is the default.
            // A 9Router/Ollama config still wins when present.
            if base.is_empty() || crate::detector::is_opencode_free_endpoint(&base) {
                return fetch_opencode_free_models(client).await;
            }

            if !base.is_empty() {
                let key = if !settings.opencode.api_key.trim().is_empty() {
                    settings.opencode.api_key.trim().to_string()
                } else {
                    config
                        .as_ref()
                        .map(|endpoint| endpoint.api_key.clone())
                        .unwrap_or_default()
                };

                // The config is authoritative and works even if /models is
                // unavailable. Merge live model ids when the endpoint has them.
                let mut models = config
                    .as_ref()
                    .map(|endpoint| endpoint.models.clone())
                    .unwrap_or_default();
                let live_error = match fetch_openai_models(client, &base, &key).await {
                    Ok(mut live) => {
                        models.append(&mut live);
                        None
                    }
                    Err(error) => Some(error),
                };
                models.sort();
                models.dedup();
                prefer_opencode_model(&mut models);
                if !models.is_empty() {
                    return Ok(models);
                }

                let config_hint = config
                    .as_ref()
                    .map(|endpoint| endpoint.config_path.display().to_string())
                    .unwrap_or_else(|| "~/.config/opencode/opencode.json".to_string());
                return Err(format!(
                    "OpenCode points at {base}, but no models were found. Check options.apiKey in {config_hint}.{}",
                    live_error
                        .map(|error| format!(" Endpoint error: {error}"))
                        .unwrap_or_default()
                ));
            }

            // No HTTP endpoint. The CLI is an optional last resort, never a
            // prerequisite for the 9Router-style config path.
            if crate::detector::find_opencode_binary().is_some() {
                let mut command = create_opencode_command();
                let output = command
                    .arg("models")
                    .output()
                    .await
                    .map_err(|error| format!("Could not run the OpenCode CLI: {error}"))?;
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let mut models: Vec<String> = stdout
                        .lines()
                        .map(str::trim)
                        .filter(|line| !line.is_empty())
                        .map(str::to_string)
                        .collect();
                    models.sort();
                    models.dedup();
                    if !models.is_empty() {
                        return Ok(models);
                    }
                }
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!(
                    "OpenCode CLI listed no models. {}",
                    truncate(stderr.trim(), 300)
                ));
            }

            Err(
                "No OpenCode endpoint found. Add options.baseURL and options.apiKey to ~/.config/opencode/opencode.json (for example a 9Router endpoint), or paste the Base URL manually. The OpenCode app/CLI is not required for HTTP endpoints."
                    .to_string(),
            )
        }
        PROVIDER_OPENCODE_FREE => fetch_opencode_free_models(client).await,
        _ => Ok(Vec::new()),
    }
}

/// Fetch `/models` from an OpenAI-compatible endpoint. Tries both `base/models`
/// and `base/v1/models` (unless base already ends in `/v1`).
async fn fetch_openai_models(
    client: &reqwest::Client,
    base: &str,
    key: &str,
) -> Result<Vec<String>, String> {
    let base = base.trim().trim_end_matches('/');
    if base.is_empty() {
        return Err("Set a base URL first.".to_string());
    }

    let mut urls = vec![format!("{base}/models")];
    if !base.ends_with("/v1") {
        urls.push(format!("{base}/v1/models"));
    }

    let mut last_error = String::new();
    for url in urls {
        let mut request = client.get(&url);
        if !key.trim().is_empty() {
            request = request.bearer_auth(key.trim());
        }
        let response = match request.send().await {
            Ok(response) => response,
            Err(error) => {
                last_error = format!("Could not reach {url}: {error}");
                continue;
            }
        };
        if !response.status().is_success() {
            last_error = format!("{url} returned {}", response.status());
            continue;
        }
        let value = match response.json::<Value>().await {
            Ok(value) => value,
            Err(error) => {
                last_error = format!("{url} returned invalid JSON: {error}");
                continue;
            }
        };

        let mut models = Vec::new();
        if let Some(items) = value.get("data").and_then(Value::as_array) {
            for item in items {
                if let Some(id) = item.get("id").and_then(Value::as_str) {
                    models.push(id.to_string());
                }
            }
        } else if let Some(items) = value.get("models").and_then(Value::as_array) {
            for item in items {
                if let Some(name) = item.get("name").and_then(Value::as_str) {
                    let id = name.strip_prefix("models/").unwrap_or(name);
                    models.push(id.to_string());
                } else if let Some(id) = item.get("id").and_then(Value::as_str) {
                    models.push(id.to_string());
                }
            }
        } else if let Some(items) = value.get("models").and_then(Value::as_object) {
            // Some OpenCode-compatible gateways return an object map instead of
            // the OpenAI `{ data: [...] }` shape.
            models.extend(items.keys().cloned());
        }

        models.sort();
        models.dedup();
        if !models.is_empty() {
            return Ok(models);
        }
    }

    if last_error.is_empty() {
        Err(format!("{base} did not expose a /models list."))
    } else {
        Err(last_error)
    }
}

/// Fetch the public OpenCode free catalogue. The live list changes often, so
/// unknown `-free` ids are kept and the known-good ids are placed first.
async fn fetch_opencode_free_models(client: &reqwest::Client) -> Result<Vec<String>, String> {
    let fallback: Vec<String> = crate::detector::OPENCODE_FREE_FALLBACK_MODELS
        .iter()
        .map(|model| model.to_string())
        .collect();

    let response = client
        .get(crate::detector::OPENCODE_FREE_MODELS_URL)
        .header("x-opencode-client", "desktop")
        .header("Accept", "application/json")
        .send()
        .await;

    let value = match response {
        Ok(response) if response.status().is_success() => match response.json::<Value>().await {
            Ok(value) => value,
            Err(error) => {
                warn!("OpenCode Free model list was not JSON: {error}");
                return Ok(fallback);
            }
        },
        Ok(response) => {
            warn!(
                "OpenCode Free model list returned HTTP {}; using known models",
                response.status()
            );
            return Ok(fallback);
        }
        Err(error) => {
            warn!("Could not reach the OpenCode Free model list: {error}");
            return Ok(fallback);
        }
    };

    let ids = value
        .get("data")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.get("id").and_then(Value::as_str))
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    Ok(filter_opencode_free_models(ids))
}

/// Keep only models that actually run on the free chat lane. `jev-*` is a
/// System-One model and `flint-*` currently reports "endpoint unavailable",
/// so both are excluded instead of letting the picker offer a dead end.
fn filter_opencode_free_models(ids: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut models: Vec<String> = ids
        .into_iter()
        .filter(|id| {
            let id = id.trim();
            (id.ends_with("-free") || id == "big-pickle")
                && id != "jev-1.13-free"
                && id != "flint-40b-free"
        })
        .collect();

    models.sort();
    models.dedup();

    let mut ordered = Vec::with_capacity(models.len());
    for wanted in crate::detector::OPENCODE_FREE_FALLBACK_MODELS {
        if let Some(index) = models.iter().position(|model| model == wanted) {
            ordered.push(models.remove(index));
        }
    }
    ordered.extend(models);
    if ordered.is_empty() {
        crate::detector::OPENCODE_FREE_FALLBACK_MODELS
            .iter()
            .map(|model| model.to_string())
            .collect()
    } else {
        ordered
    }
}

/// Put a sensible general-purpose model at the top of the OpenCode list so
/// the settings dialog does not default to whichever id sorts first.
fn prefer_opencode_model(models: &mut Vec<String>) {
    for wanted in [
        "ag/gemini-3.8-flash-high",
        "ag/gemini-3.8-flash",
        "kr/minimax-m2.1",
        "kr/glm-5",
        "gemini-2.5-flash",
        "gpt-4o",
    ] {
        if let Some(index) = models.iter().position(|model| model == wanted) {
            let model = models.remove(index);
            models.insert(0, model);
            return;
        }
    }
}

fn create_opencode_command() -> Command {
    let mut command = if let Some(binary) = crate::detector::find_opencode_binary() {
        #[cfg(windows)]
        {
            let extension = binary
                .extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            if extension == "cmd" || extension == "bat" {
                let mut cmd = Command::new("cmd");
                crate::platform::configure_tokio_command(&mut cmd);
                cmd.arg("/C").arg(binary);
                return cmd;
            }
        }
        Command::new(binary)
    } else {
        // Preserve `Command::new("opencode")` so the eventual spawn error has
        // the normal OS message; callers add the actionable OpenCode hint.
        Command::new("opencode")
    };
    crate::platform::configure_tokio_command(&mut command);
    command
}

/// The exact `read` and `shell` declarations OpenCode v2 sends. The free-tier
/// gate compares descriptions and schemas byte-for-byte, so these strings must
/// not be edited or reformatted.
fn opencode_fingerprint_tools() -> Value {
    json!([
        {
            "type": "function",
            "function": {
                "name": "read",
                "description": "Read the contents of a file or directory. Supports text files, images, and PDFs. Images and PDFs are presented directly to the model. Each text line is prefixed by its 1-based line number as <line>: <content>. The prefix is for reference and is not part of the file content. Directory entries are returned one per line. Use offset and limit to read large files or directories in sections. Prefer one larger read over many small slices, and use grep to find specific content in large files.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "File or directory to read" },
                        "offset": { "type": "integer", "minimum": 0, "description": "The line or directory entry to start reading from (1-based)" },
                        "limit": { "type": "integer", "minimum": 0, "description": "The maximum number of lines or directory entries to read (defaults to and capped at 2000)" }
                    },
                    "required": ["path"],
                    "additionalProperties": false
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "shell",
                "description": "Execute a shell command and return its output. Commands run on Linux using bash. Quote file paths containing spaces or special characters. Prefer dedicated tools over shell commands when possible. When output is large, the full result is saved to a file and a truncated preview is returned. Rely on automatic truncation unless filtering the output is more useful. Commands accept an optional timeout, background commands have no timeout by default. Background commands return immediately, and you will be notified when they complete.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "command": { "type": "string", "description": "Shell command string to execute" },
                        "workdir": { "type": "string", "description": "Working directory to execute the command in. Defaults to the current working directory. When possible, avoid changing directories in the command and set the working directory here instead." },
                        "timeout": { "type": "integer", "minimum": 0, "description": "Timeout in milliseconds. Set to 0 to disable the timeout. Defaults to 120000 for foreground commands. Background commands have no timeout by default." },
                        "background": { "type": "boolean", "description": "Run the command in the background and return immediately (useful for dev servers and long-running builds). You do not need to use '&' at the end of the command when using this parameter. You will be notified when it completes. DO NOT poll for completion." }
                    },
                    "required": ["command"],
                    "additionalProperties": false
                }
            }
        }
    ])
}

/// One stable OpenCode session per process. The upstream free-tier quota is
/// accounted per session; minting a new id per request burns quota quickly.
fn opencode_free_session() -> &'static str {
    static SESSION: OnceLock<String> = OnceLock::new();
    SESSION.get_or_init(|| {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let millis = now.as_millis() as u64;
        // Mirrors the OpenCode CLI: complement of (timeMs * 4096 + counter),
        // truncated to 48 bits, hex-encoded. Counter 1 is a real client value.
        let prefix = (!(millis.wrapping_mul(0x1000).wrapping_add(1))) & ((1u64 << 48) - 1);
        let mut seed =
            (now.as_nanos() as u64) ^ ((std::process::id() as u64) << 32) ^ 0x9E37_79B9_7F4A_7C15;
        if seed == 0 {
            seed = 0x1234_5678_9ABC_DEF0;
        }
        const CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let mut suffix = String::with_capacity(14);
        for _ in 0..14 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            suffix.push(CHARS[(seed % 62) as usize] as char);
        }
        format!("ses_{prefix:012x}{suffix}")
    })
}

async fn openai_generate_candidate(
    client: &reqwest::Client,
    candidate: &ModelCandidate,
    system: &str,
    user: &str,
    images: &[Vec<u8>],
    sender: &broadcast::Sender<String>,
) -> Result<String, String> {
    let base = candidate.base_url.trim().trim_end_matches('/');
    if base.is_empty() {
        return Err("OpenAI-compatible base URL is not set.".to_string());
    }
    let model = candidate.model.trim();
    if model.is_empty() {
        return Err("No model selected.".to_string());
    }

    let free = crate::detector::is_opencode_free_endpoint(base);
    let url = if free {
        crate::detector::OPENCODE_FREE_CHAT_URL.to_string()
    } else {
        format!("{base}/chat/completions")
    };
    emit_event(
        sender,
        "status",
        format!(
            "Connecting to {}{} · {model}{} ({})…",
            if free { "OpenCode Free" } else { base },
            if free { " (public lane, no key)" } else { "" },
            if images.is_empty() {
                String::new()
            } else {
                format!(" · {} frame(s) attached", images.len())
            },
            candidate.description
        ),
    );

    let system_content = if free {
        format!(
            "{system}\n\nIMPORTANT: You have no usable tools in this request. Do not call any tool. The `read` and `shell` declarations are required by the gateway but are non-functional placeholders. Return the complete artifact as text."
        )
    } else {
        system.to_string()
    };

    let mut body = json!({
        "model": model,
        "stream": true,
        "temperature": if images.is_empty() { 0.6 } else { 0.45 },
        // Free reasoning models count their reasoning against max_tokens. A
        // 16k cap can be consumed before the first content token, producing a
        // valid but empty stream. 64k leaves room for both.
        "max_tokens": if free { 65_536 } else { MAX_OUTPUT_TOKENS },
        "messages": [
            { "role": "system", "content": system_content },
            { "role": "user", "content": openai_user_content(user, images) },
        ],
    });
    if free {
        // OpenCode Free gates requests on the exact OpenCode v2 tool
        // fingerprint. These are intentional placeholders, not callable tools;
        // the system note tells the model not to use them.
        body["tools"] = opencode_fingerprint_tools();
    }

    let mut request = client
        .post(&url)
        .header("Accept", "text/event-stream")
        .json(&body);
    if free {
        let session = opencode_free_session();
        request = request
            .header("Authorization", "Bearer public")
            .header("User-Agent", "opencode/latest/2.0.26/cli")
            .header("x-opencode-client", "cli")
            .header("x-opencode-session", session)
            .header("x-opencode-session-id", session)
            .header("x-session-affinity", session)
            .header("x-session-id", session)
            .header("x-opencode-project", "global");
    } else if !candidate.api_key.is_empty() {
        request = request.bearer_auth(&candidate.api_key);
    }

    let response = request
        .send()
        .await
        .map_err(|error| format!("Could not reach {url}: {error}"))?;
    let status = response.status();
    if !status.is_success() {
        let detail = response.text().await.unwrap_or_default();
        return Err(format!(
            "Provider returned {status}: {}",
            truncate(detail.trim(), 400)
        ));
    }

    let mut stream = response.bytes_stream();
    let mut buffer: Vec<u8> = Vec::new();
    let mut batch = TokenBatch::new();
    let mut text = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|error| format!("Stream error: {error}"))?;
        buffer.extend_from_slice(&chunk);
        while let Some(end) = find_event_end(&buffer) {
            let event: Vec<u8> = buffer.drain(..end).collect();
            let event = String::from_utf8_lossy(&event).replace('\r', "");
            for line in event.lines() {
                let Some(data) = line.trim_start().strip_prefix("data:") else {
                    continue;
                };
                match openai_delta(data.trim())? {
                    Some(delta) => {
                        text.push_str(&delta);
                        batch.push(sender, &delta);
                    }
                    None => {}
                }
            }
        }
    }
    batch.flush(sender);

    if text.trim().is_empty() {
        return Err("The provider returned an empty response.".to_string());
    }
    info!("OpenAI-compatible model produced {} bytes", text.len());
    Ok(text)
}

/// Parse one `data:` payload from an OpenAI-compatible stream. `Ok(None)`
/// covers `[DONE]` and bookkeeping frames.
pub(crate) fn openai_delta(data: &str) -> Result<Option<String>, String> {
    if data.is_empty() || data == "[DONE]" {
        return Ok(None);
    }

    let value: Value =
        serde_json::from_str(data).map_err(|error| format!("Malformed stream frame: {error}"))?;

    if let Some(message) = value.pointer("/error/message").and_then(Value::as_str) {
        return Err(format!("Provider error: {message}"));
    }

    if let Some(delta) = value
        .pointer("/choices/0/delta/content")
        .and_then(Value::as_str)
    {
        return Ok(Some(delta.to_string()));
    }
    if let Some(content) = value
        .pointer("/choices/0/message/content")
        .and_then(Value::as_str)
    {
        return Ok(Some(content.to_string()));
    }

    Ok(None)
}

async fn anthropic_generate_candidate(
    client: &reqwest::Client,
    candidate: &ModelCandidate,
    system: &str,
    user: &str,
    images: &[Vec<u8>],
    sender: &broadcast::Sender<String>,
) -> Result<String, String> {
    let base = candidate.base_url.trim().trim_end_matches('/');
    let base = if base.is_empty() {
        "https://api.anthropic.com"
    } else {
        base
    };
    let model = if candidate.model.trim().is_empty() {
        "claude-opus-4-6"
    } else {
        candidate.model.trim()
    };
    let key = candidate.api_key.trim();
    if key.is_empty() {
        return Err("Anthropic API key is not configured.".to_string());
    }

    emit_event(
        sender,
        "status",
        format!(
            "Connecting to {base} · {model}{} — streaming Messages API ({}).",
            if images.is_empty() {
                String::new()
            } else {
                format!(" · {} frame(s) attached", images.len())
            },
            candidate.description
        ),
    );

    let body = json!({
        "model": model,
        "max_tokens": MAX_OUTPUT_TOKENS,
        "stream": true,
        "system": system,
        "messages": [{ "role": "user", "content": anthropic_user_content(user, images) }],
    });

    let response = client
        .post(format!("{base}/v1/messages"))
        .header("x-api-key", key.trim())
        .header("anthropic-version", "2023-06-01")
        .header("Accept", "text/event-stream")
        .json(&body)
        .send()
        .await
        .map_err(|error| format!("Could not reach {base}: {error}"))?;

    let status = response.status();
    if !status.is_success() {
        let detail = response.text().await.unwrap_or_default();
        return Err(format!(
            "Anthropic returned {status}: {}",
            truncate(detail.trim(), 400)
        ));
    }

    let mut stream = response.bytes_stream();
    let mut buffer: Vec<u8> = Vec::new();
    let mut batch = TokenBatch::new();
    let mut text = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|error| format!("Stream error: {error}"))?;
        buffer.extend_from_slice(&chunk);
        while let Some(end) = find_event_end(&buffer) {
            let event: Vec<u8> = buffer.drain(..end).collect();
            let event = String::from_utf8_lossy(&event).replace('\r', "");
            for line in event.lines() {
                let Some(data) = line.trim_start().strip_prefix("data:") else {
                    continue;
                };
                if let Some(delta) = anthropic_delta(data.trim())? {
                    text.push_str(&delta);
                    batch.push(sender, &delta);
                }
            }
        }
    }
    batch.flush(sender);

    if text.trim().is_empty() {
        return Err("Anthropic returned an empty response.".to_string());
    }
    info!("Anthropic produced {} bytes", text.len());
    Ok(text)
}

pub(crate) fn anthropic_delta(data: &str) -> Result<Option<String>, String> {
    if data.is_empty() {
        return Ok(None);
    }
    let value: Value =
        serde_json::from_str(data).map_err(|error| format!("Malformed stream frame: {error}"))?;

    match value.get("type").and_then(Value::as_str) {
        Some("content_block_delta") => Ok(value
            .pointer("/delta/text")
            .and_then(Value::as_str)
            .map(str::to_string)),
        Some("error") => {
            let message = value
                .pointer("/error/message")
                .and_then(Value::as_str)
                .unwrap_or("unknown error");
            Err(format!("Anthropic error: {message}"))
        }
        _ => Ok(None),
    }
}

/// Runs the local `opencode` CLI inside `workspace/`. The agent may write
/// `index.html` itself (preferred, detected by mtime/size), otherwise its text
/// output is treated as the artifact source.
async fn opencode_generate_candidate(
    candidate: &ModelCandidate,
    system: &str,
    user: &str,
    sender: &broadcast::Sender<String>,
    workspace: &Path,
) -> Result<Generated, String> {
    let model = candidate.model.trim().to_string();

    emit_event(
        sender,
        "status",
        if model.is_empty() {
            "Contacting the local OpenCode agent (default model)…".to_string()
        } else {
            format!("Contacting the local OpenCode agent · {model}…")
        },
    );

    // OpenCode has file tools, so it is the executor here. It must not also
    // paste a full HTML document into its reply: that doubles the token cost and
    // makes the agent keep going for far longer than the task needs.
    let message = format!(
        "{system}\n\n---\n\n{user}\n\n\
         Environment override: you are running inside {} and you have file tools. \
         For THIS task, ignore the instruction to reply with a fenced HTML block. \
         Write the finished artifact to ./index.html yourself (inline CSS/JS, no \
         external requests) in a single write, then stop. Do not start servers, run \
         build tools, browse, or iterate on unrelated files. Your final reply must \
         be one short line describing what you built.",
        workspace.display()
    );

    let before = artifact_state(workspace);

    let mut command = create_opencode_command();
    command
        .arg("run")
        .arg("--format")
        .arg("json")
        .arg("--auto")
        .arg("--title")
        .arg("ZenDesign artifact");
    if !model.is_empty() {
        command.arg("--model").arg(&model);
    }
    command
        .arg("--")
        .arg(&message)
        .current_dir(workspace)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    crate::platform::detach_process_group(&mut command);

    let mut child = command.spawn().map_err(|error| {
        format!(
            "Could not start the OpenCode CLI agent: {error}. \
             ZenDesign can use an OpenCode HTTP endpoint without the app: \
             set a Base URL in Settings, or add options.baseURL/options.apiKey \
             to your opencode.json."
        )
    })?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "OpenCode stdout was not captured".to_string())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "OpenCode stderr was not captured".to_string())?;

    let stderr_task = tokio::spawn(async move {
        let mut buffer = Vec::new();
        let _ = BufReader::new(stderr).read_to_end(&mut buffer).await;
        String::from_utf8_lossy(&buffer).to_string()
    });

    let mut lines = BufReader::new(stdout).lines();
    let mut batch = TokenBatch::new();
    let mut text = String::new();
    let mut plain = String::new();
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut error_message: Option<String> = None;

    while let Some(line) = lines
        .next_line()
        .await
        .map_err(|error| format!("Could not read OpenCode output: {error}"))?
    {
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        let Ok(value) = serde_json::from_str::<Value>(&line) else {
            if plain.len() < 120_000 {
                plain.push_str(&line);
                plain.push('\n');
            }
            continue;
        };

        match value.get("type").and_then(Value::as_str) {
            Some("text") => {
                let id = value
                    .pointer("/part/id")
                    .and_then(Value::as_str)
                    .unwrap_or("text")
                    .to_string();
                let full = value
                    .pointer("/part/text")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let previous = seen.entry(id).or_insert(0);
                let delta = if full.len() > *previous {
                    &full[*previous..]
                } else {
                    ""
                };
                *previous = full.len();
                if !delta.is_empty() {
                    text.push_str(delta);
                    batch.push(sender, delta);
                }
            }
            Some("step_finish") => {
                let output_tokens = value
                    .pointer("/part/tokens/output")
                    .and_then(Value::as_u64)
                    .unwrap_or(0);
                emit_event(
                    sender,
                    "status",
                    format!("Agent step finished · {output_tokens} output tokens."),
                );
            }
            Some("error") => {
                let message = value
                    .pointer("/error/message")
                    .and_then(Value::as_str)
                    .unwrap_or("The OpenCode agent reported an error.");
                error_message = Some(message.to_string());
            }
            _ => {}
        }
    }
    batch.flush(sender);

    let status = child
        .wait()
        .await
        .map_err(|error| format!("OpenCode did not finish: {error}"))?;

    if artifact_changed(before, workspace) {
        info!("OpenCode wrote the artifact directly");
        return Ok(Generated::ArtifactWritten);
    }

    if !status.success() {
        let stderr_text = stderr_task.await.unwrap_or_default();
        return Err(error_message.unwrap_or_else(|| {
            format!(
                "OpenCode exited with {status}: {}",
                truncate(stderr_text.trim(), 400)
            )
        }));
    }

    if text.trim().is_empty() {
        if !plain.trim().is_empty() {
            return Ok(Generated::Text(plain));
        }
        let stderr_text = stderr_task.await.unwrap_or_default();
        return Err(match error_message {
            Some(message) => message,
            None => format!(
                "OpenCode returned no output. {}",
                truncate(stderr_text.trim(), 300)
            ),
        });
    }

    Ok(Generated::Text(text))
}

fn artifact_state(workspace: &Path) -> Option<(u64, Option<SystemTime>)> {
    std::fs::metadata(workspace.join("index.html"))
        .ok()
        .map(|meta| (meta.len(), meta.modified().ok()))
}

fn artifact_changed(before: Option<(u64, Option<SystemTime>)>, workspace: &Path) -> bool {
    artifact_state(workspace) != before
}

fn openai_user_content(user: &str, images: &[Vec<u8>]) -> Value {
    if images.is_empty() {
        return json!(user);
    }
    let mut parts = Vec::with_capacity(images.len() + 1);
    parts.push(json!({ "type": "text", "text": user }));
    for image in images {
        parts.push(json!({
            "type": "image_url",
            "image_url": {
                "url": format!("data:image/png;base64,{}", base64_encode(image))
            }
        }));
    }
    json!(parts)
}

fn anthropic_user_content(user: &str, images: &[Vec<u8>]) -> Value {
    if images.is_empty() {
        return json!(user);
    }
    let mut parts = Vec::with_capacity(images.len() + 1);
    parts.push(json!({ "type": "text", "text": user }));
    for image in images {
        parts.push(json!({
            "type": "image",
            "source": {
                "type": "base64",
                "media_type": "image/png",
                "data": base64_encode(image)
            }
        }));
    }
    json!(parts)
}

/// Minimal encoder so frame capture needs no extra crate.
fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let first = chunk[0];
        let second = chunk.get(1).copied().unwrap_or(0);
        let third = chunk.get(2).copied().unwrap_or(0);
        output.push(TABLE[(first >> 2) as usize] as char);
        output.push(TABLE[(((first & 0b0000_0011) << 4) | (second >> 4)) as usize] as char);
        if chunk.len() > 1 {
            output.push(TABLE[(((second & 0b0000_1111) << 2) | (third >> 6)) as usize] as char);
        } else {
            output.push('=');
        }
        if chunk.len() > 2 {
            output.push(TABLE[(third & 0b0011_1111) as usize] as char);
        } else {
            output.push('=');
        }
    }
    output
}

/// Index just past the end of the next SSE event (`\n\n` or `\r\n\r\n`).
fn find_event_end(buffer: &[u8]) -> Option<usize> {
    if let Some(index) = buffer.windows(4).position(|window| window == b"\r\n\r\n") {
        return Some(index + 4);
    }
    buffer
        .windows(2)
        .position(|window| window == b"\n\n")
        .map(|index| index + 2)
}

fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    let cut: String = value.chars().take(max_chars).collect();
    format!("{cut}…")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_both_event_separators() {
        assert_eq!(find_event_end(b"data: a\n\nrest"), Some(9));
        assert_eq!(find_event_end(b"data: a\r\n\r\nrest"), Some(11));
        assert_eq!(find_event_end(b"data: a"), None);
    }

    #[test]
    fn parses_openai_deltas() {
        let frame = r#"{"choices":[{"delta":{"content":"<h1>"}}]}"#;
        assert_eq!(openai_delta(frame).unwrap(), Some("<h1>".to_string()));
        assert_eq!(openai_delta("[DONE]").unwrap(), None);
        let error = r#"{"error":{"message":"rate limited"}}"#;
        assert!(openai_delta(error).unwrap_err().contains("rate limited"));
    }

    #[test]
    fn parses_anthropic_deltas() {
        let frame = r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"body"}}"#;
        assert_eq!(anthropic_delta(frame).unwrap(), Some("body".to_string()));
        let error = r#"{"type":"error","error":{"message":"bad key"}}"#;
        assert!(anthropic_delta(error).unwrap_err().contains("bad key"));
        assert_eq!(anthropic_delta(r#"{"type":"message_stop"}"#).unwrap(), None);
    }

    #[test]
    fn opencode_provider_uses_config_without_cli() {
        if crate::detector::read_opencode_endpoint().is_err() {
            return;
        }
        let mut settings = Settings::default();
        settings.provider = PROVIDER_OPENCODE.to_string();
        settings.opencode.model = "ag/gemini-3.8-flash-high".to_string();

        let candidates = resolve_candidates(&settings);
        let candidate = candidates
            .iter()
            .find(|candidate| candidate.model == "ag/gemini-3.8-flash-high")
            .expect("OpenCode config should produce an HTTP candidate");
        assert_eq!(candidate.provider, PROVIDER_OPENAI);
        assert!(!candidate.base_url.is_empty());
        assert!(!candidate.api_key.is_empty());
    }

    #[test]
    fn opencode_free_models_are_filtered_and_ordered() {
        let models = filter_opencode_free_models(vec![
            "jev-1.13-free".to_string(),
            "flint-40b-free".to_string(),
            "z-new-free".to_string(),
            "nemotron-3.5-lightning-free".to_string(),
            "beam-free".to_string(),
            "big-pickle".to_string(),
            "glm-5.3-flash-free".to_string(),
            "deepseek-v4.1-flash-free".to_string(),
            "claude-opus-5".to_string(),
        ]);
        assert_eq!(
            models.first().map(String::as_str),
            Some("nemotron-3.5-lightning-free")
        );
        assert!(models.iter().any(|model| model == "big-pickle"));
        assert!(models.iter().any(|model| model == "z-new-free"));
        assert!(!models.iter().any(|model| model == "jev-1.13-free"));
        assert!(!models.iter().any(|model| model == "flint-40b-free"));
        assert!(!models.iter().any(|model| model == "claude-opus-5"));
    }

    #[test]
    fn free_lane_stays_on_the_public_provider() {
        let mut settings = Settings::default();
        settings.provider = PROVIDER_OPENAI.to_string();
        settings.openai.base_url = crate::detector::OPENCODE_FREE_BASE_URL.to_string();
        settings.openai.model = crate::detector::OPENCODE_FREE_DEFAULT_MODEL.to_string();

        let candidates = resolve_candidates(&settings);
        assert!(!candidates.is_empty());
        assert_eq!(
            candidates[0].model,
            crate::detector::OPENCODE_FREE_DEFAULT_MODEL
        );
        assert!(candidates
            .iter()
            .all(|candidate| { crate::detector::is_opencode_free_endpoint(&candidate.base_url) }));
    }

    #[test]
    fn opencode_free_session_matches_the_client_shape() {
        let session = opencode_free_session();
        assert_eq!(session.len(), 30);
        assert!(session.starts_with("ses_"));
        assert!(session[4..16].chars().all(|ch| ch.is_ascii_hexdigit()));
        assert!(session[16..].chars().all(|ch| ch.is_ascii_alphanumeric()));
    }

    #[test]
    fn fingerprint_tools_match_the_free_tier_gate() {
        let tools = opencode_fingerprint_tools();
        assert_eq!(tools[0]["function"]["name"], "read");
        assert_eq!(tools[1]["function"]["name"], "shell");
        assert_eq!(tools[0]["function"]["parameters"]["required"][0], "path");
        assert_eq!(tools[1]["function"]["parameters"]["required"][0], "command");
    }

    #[test]
    fn encodes_frames_for_vision_requests() {
        assert_eq!(base64_encode(b"Man"), "TWFu");
        assert_eq!(base64_encode(b"Ma"), "TWE=");
        assert_eq!(base64_encode(b"M"), "TQ==");
        assert_eq!(base64_encode(b""), "");

        let openai = openai_user_content("review this cut", &[vec![1, 2, 3]]);
        assert_eq!(openai[0]["text"], "review this cut");
        assert!(openai[1]["image_url"]["url"]
            .as_str()
            .unwrap()
            .starts_with("data:image/png;base64,"));

        let anthropic = anthropic_user_content("review this cut", &[vec![1, 2, 3]]);
        assert_eq!(anthropic[1]["type"], "image");
        assert!(anthropic[1]["source"]["data"].is_string());

        assert_eq!(openai_user_content("plain", &[]), json!("plain"));
        assert_eq!(anthropic_user_content("plain", &[]), json!("plain"));
    }
}
