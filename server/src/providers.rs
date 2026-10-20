//! Model-provider adapters.
//!
//! Four ways to turn a brief into an artifact:
//!
//! * `template`  — the deterministic, fully offline generator in `agent.rs`.
//! * `openai`    — any OpenAI-compatible `/chat/completions` endpoint, with SSE
//!   streaming. This covers 9Router (`ag/*` Antigravity models, `kr/*` Claude,
//!   DeepSeek, GLM…), OpenRouter, Ollama, LM Studio, vLLM and OpenAI itself.
//! * `anthropic` — the native Messages API with SSE streaming.
//! * `opencode`  — drives the local `opencode` CLI in `workspace/`, so the
//!   agent, its tools and its configured models (including 9Router) do the work.

use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant, SystemTime};

use futures_util::StreamExt;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::broadcast;
use tracing::info;

use crate::agent::AgentEvent;
use crate::settings::{Settings, PROVIDER_ANTHROPIC, PROVIDER_OPENAI, PROVIDER_OPENCODE};

/// Result of a provider run: either raw model text (the orchestrator extracts
/// and stores the HTML) or proof that the agent wrote the artifact itself.
pub enum Generated {
    Text(String),
    ArtifactWritten,
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
    match settings.provider.as_str() {
        PROVIDER_OPENAI => openai_generate(client, settings, system, user, images, sender)
            .await
            .map(Generated::Text),
        PROVIDER_ANTHROPIC => anthropic_generate(client, settings, system, user, images, sender)
            .await
            .map(Generated::Text),
        PROVIDER_OPENCODE => {
            if !images.is_empty() {
                return Err(
                    "Visual review needs an OpenAI-compatible or Anthropic API provider; \
                     OpenCode has no attached frames."
                        .to_string(),
                );
            }
            opencode_generate(settings, system, user, sender, workspace).await
        }
        other => Err(format!(
            "No model is configured (provider: {other}). Open Settings and connect one."
        )),
    }
}

/// List models for the settings dialog. OpenAI-compatible endpoints expose
/// `/models`; Anthropic is a fixed list; OpenCode is asked directly.
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
            let key = settings.openai_key();
            let mut request = client.get(format!("{base}/models"));
            if !key.is_empty() {
                request = request.bearer_auth(&key);
            }
            let response = request.send().await;
            let mut models: Vec<String> = Vec::new();

            if let Ok(resp) = response {
                if resp.status().is_success() {
                    if let Ok(value) = resp.json::<Value>().await {
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
                                }
                            }
                        }
                    }
                }
            }

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
            let binary =
                std::env::var("ZEN_OPENCODE_BIN").unwrap_or_else(|_| "opencode".to_string());
            let output = Command::new(&binary)
                .arg("models")
                .output()
                .await
                .map_err(|error| format!("Could not run `{binary} models`: {error}"))?;
            if !output.status.success() {
                return Err(format!("`{binary} models` exited with {}", output.status));
            }
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut models: Vec<String> = stdout
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_string)
                .collect();
            models.sort();
            models.dedup();
            Ok(models)
        }
        _ => Ok(Vec::new()),
    }
}

async fn openai_generate(
    client: &reqwest::Client,
    settings: &Settings,
    system: &str,
    user: &str,
    images: &[Vec<u8>],
    sender: &broadcast::Sender<String>,
) -> Result<String, String> {
    let base = settings.openai.base_url.trim().trim_end_matches('/');
    if base.is_empty() {
        return Err(
            "OpenAI-compatible base URL is not set. Open Settings → 9Router / OpenAI.".to_string(),
        );
    }
    let model = settings.openai.model.trim();
    if model.is_empty() {
        return Err("No model selected. Open Settings and choose one.".to_string());
    }

    let url = format!("{base}/chat/completions");
    emit_event(
        sender,
        "status",
        format!(
            "Contacting {base} · {model}{} — no reasoning timeout, stream stays open.",
            if images.is_empty() {
                String::new()
            } else {
                format!(" · {} frame(s) attached", images.len())
            }
        ),
    );

    let body = json!({
        "model": model,
        "stream": true,
        "temperature": if images.is_empty() { 0.6 } else { 0.45 },
        "max_tokens": MAX_OUTPUT_TOKENS,
        "messages": [
            { "role": "system", "content": system },
            { "role": "user", "content": openai_user_content(user, images) },
        ],
    });

    let key = settings.openai_key();
    let mut request = client
        .post(&url)
        .header("Accept", "text/event-stream")
        .json(&body);
    if !key.is_empty() {
        request = request.bearer_auth(key);
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

async fn anthropic_generate(
    client: &reqwest::Client,
    settings: &Settings,
    system: &str,
    user: &str,
    images: &[Vec<u8>],
    sender: &broadcast::Sender<String>,
) -> Result<String, String> {
    let base = settings.anthropic.base_url.trim().trim_end_matches('/');
    let base = if base.is_empty() {
        "https://api.anthropic.com"
    } else {
        base
    };
    let model = if settings.anthropic.model.trim().is_empty() {
        "claude-opus-4-6"
    } else {
        settings.anthropic.model.trim()
    };
    let key = settings.anthropic_key();
    if key.trim().is_empty() {
        return Err(
            "Anthropic API key is not configured. Add it in Settings or export ANTHROPIC_API_KEY."
                .to_string(),
        );
    }

    emit_event(
        sender,
        "status",
        format!(
            "Contacting {base} · {model}{} — streaming Messages API.",
            if images.is_empty() {
                String::new()
            } else {
                format!(" · {} frame(s) attached", images.len())
            }
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
async fn opencode_generate(
    settings: &Settings,
    system: &str,
    user: &str,
    sender: &broadcast::Sender<String>,
    workspace: &Path,
) -> Result<Generated, String> {
    let binary = std::env::var("ZEN_OPENCODE_BIN").unwrap_or_else(|_| "opencode".to_string());
    let model = settings.opencode.model.trim().to_string();

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

    let mut command = Command::new(&binary);
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

    let mut child = command
        .spawn()
        .map_err(|error| format!("Could not start `{binary}`: {error}"))?;

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
