//! Auto-detection of local AI agent and CLI credentials.
//!
//! Scans the host system for existing setups:
//! * Codex (`~/.codex/config.toml` + `~/.codex/auth.json`)
//! * Claude Code (`ANTHROPIC_API_KEY` + `~/.claude.json`)
//! * OpenCode (`~/.config/opencode/opencode.json`)
//! * OpenCode Free (public no-auth lane, no app or API key)
//! * Environment variables (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`)

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetectedSource {
    pub id: String,
    pub name: String,
    pub provider: String, // "openai" or "anthropic"
    pub base_url: String,
    pub model: Option<String>,
    pub hint: String,
    pub has_key: bool,
    #[serde(default)]
    pub models: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ResolvedCredentials {
    pub provider: String,
    pub base_url: String,
    pub model: Option<String>,
    pub api_key: String,
}

/// OpenCode's public free tier. 9Router talks to this lane directly with the
/// OpenCode v2 client fingerprint, so ZenDesign can do the same without the
/// OpenCode app, CLI, or an API key.
pub const OPENCODE_FREE_BASE_URL: &str = "https://opencode.ai";
pub const OPENCODE_FREE_MODELS_URL: &str = "https://opencode.ai/zen/v1/models";
pub const OPENCODE_FREE_CHAT_URL: &str = "https://opencode.ai/inference/v1/chat/completions";
pub const OPENCODE_FREE_DEFAULT_MODEL: &str = "nemotron-3.5-lightning-free";

/// Known working free chat models. The live `/zen/v1/models` response is
/// merged on top of this list, so new ids appear automatically. Nemotron is
/// fast and returns content without a long reasoning phase; Beam is the next
/// choice, then the larger reasoning models.
pub const OPENCODE_FREE_FALLBACK_MODELS: [&str; 8] = [
    "nemotron-3.5-lightning-free",
    "beam-free",
    "glm-5.3-flash-free",
    "deepseek-v4.1-flash-free",
    "minimax-m3.1-flash-free",
    "mimo-v2.6-flash-free",
    "big-pickle",
    "nemotron-3.5-super-vl-free",
];

/// True only for the real OpenCode host, not for a 9Router proxy or a
/// look-alike domain that merely contains the string.
pub fn is_opencode_free_endpoint(base_url: &str) -> bool {
    reqwest::Url::parse(base_url.trim())
        .ok()
        .and_then(|url| url.host_str().map(str::to_ascii_lowercase))
        .map(|host| host == "opencode.ai" || host == "www.opencode.ai")
        .unwrap_or(false)
}

/// The useful part of an OpenCode provider entry.
///
/// OpenCode is only a client: its `opencode.json` points at an
/// OpenAI-compatible endpoint such as 9Router, Ollama, or a self-hosted
/// gateway. Reading this file lets ZenDesign call that endpoint directly,
/// without installing or launching the OpenCode app/CLI.
#[derive(Clone, Debug)]
pub struct OpenCodeEndpoint {
    pub provider_name: String,
    pub base_url: String,
    pub api_key: String,
    pub models: Vec<String>,
    pub config_path: PathBuf,
}

fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .or_else(|| std::env::var("USERPROFILE").ok())
        .map(PathBuf::from)
}

fn xdg_config_dir() -> Option<PathBuf> {
    std::env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var("APPDATA").ok().map(PathBuf::from))
        .or_else(|| home_dir().map(|h| h.join(".config")))
}

/// Detects all local tools and credentials that ZenDesign can seamlessly connect to.
pub fn scan_local_sources() -> Vec<DetectedSource> {
    let mut sources = Vec::new();

    // 1. Antigravity Agent
    if let Some(source) = scan_antigravity() {
        sources.push(source);
    }

    // 2. OpenAI Codex (Codex CLI / ~/.codex)
    if let Some(source) = scan_codex() {
        sources.push(source);
    }

    // 3. Claude Code (~/.claude.json or ANTHROPIC_API_KEY)
    if let Some(source) = scan_claude_code() {
        sources.push(source);
    }

    // 4. OpenCode (~/.config/opencode/opencode.json)
    if let Some(source) = scan_opencode() {
        sources.push(source);
    }

    // 5. OpenCode Free: public no-auth lane used by 9Router. It needs neither
    // the OpenCode app/CLI nor an API key, so it is always available.
    sources.push(scan_opencode_free());

    // 6. Check Environment OPENAI_API_KEY
    if let Some(source) = scan_env_openai() {
        sources.push(source);
    }

    sources
}

/// Resolves actual credentials when user clicks to apply a source.
pub fn resolve_source(id: &str) -> Result<ResolvedCredentials, String> {
    match id {
        "antigravity" => resolve_antigravity(),
        "codex" => resolve_codex(),
        "claude_code" => resolve_claude_code(),
        "opencode" => resolve_opencode(),
        "opencode_free" => resolve_opencode_free(),
        "env_openai" => resolve_env_openai(),
        _ => Err(format!("Unknown source id: {id}")),
    }
}

// ── Antigravity Detector ───────────────────────────────────────────────────

struct AgProviderInfo {
    base_url: String,
    api_key: String,
    model: Option<String>,
}

/// Strips line and block comments from JSONC content so standard serde_json can parse it.
pub fn strip_json_comments(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    let mut escape = false;

    while let Some(ch) = chars.next() {
        if in_string {
            out.push(ch);
            if escape {
                escape = false;
            } else if ch == '\\' {
                escape = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '"' {
            in_string = true;
            out.push(ch);
            continue;
        }

        if ch == '/' {
            if let Some(&next) = chars.peek() {
                if next == '/' {
                    chars.next();
                    // Line comment: skip until newline
                    for c in chars.by_ref() {
                        if c == '\n' {
                            out.push('\n');
                            break;
                        }
                    }
                    continue;
                } else if next == '*' {
                    chars.next();
                    // Block comment: skip until */
                    while let Some(c) = chars.next() {
                        if c == '*' && chars.peek() == Some(&'/') {
                            chars.next();
                            break;
                        }
                    }
                    continue;
                }
            }
        }

        out.push(ch);
    }
    out
}

/// Comprehensive cross-platform discovery of OpenCode configuration files,
/// supporting both CLI and Desktop (.exe) installations.
/// Checks folders: opencode, OpenCode, opencode-desktop, OpenCode Desktop
/// Checks files: opencode.json, config.json, settings.json, opencode.jsonc, config.jsonc
pub fn find_opencode_config_file() -> Option<PathBuf> {
    let mut search_dirs = Vec::new();

    // 1. Current workspace / relative
    search_dirs.push(PathBuf::from(".opencode"));
    search_dirs.push(PathBuf::from("."));

    // 2. User home: ~/.config/<folder>, ~/.<folder>
    if let Some(home) = home_dir() {
        for folder in [
            "opencode",
            "OpenCode",
            "opencode-desktop",
            "OpenCode Desktop",
        ] {
            search_dirs.push(home.join(".config").join(folder));
            search_dirs.push(home.join(format!(".{folder}")));
        }
    }

    // 3. Native config directory (Application Support on macOS, APPDATA on
    // Windows, XDG on Linux). New OpenCode desktop builds use this location.
    for folder in [
        "opencode",
        "OpenCode",
        "opencode-desktop",
        "OpenCode Desktop",
    ] {
        search_dirs.push(crate::platform::config_dir().join(folder));
    }

    // 4. XDG_CONFIG_HOME (explicit override)
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.trim().is_empty() {
            let base = PathBuf::from(xdg);
            for folder in [
                "opencode",
                "OpenCode",
                "opencode-desktop",
                "OpenCode Desktop",
            ] {
                search_dirs.push(base.join(folder));
            }
        }
    }

    // 5. Windows %APPDATA% and %LOCALAPPDATA% (where Desktop .exe stores configs)
    if let Ok(appdata) = std::env::var("APPDATA") {
        if !appdata.trim().is_empty() {
            let base = PathBuf::from(appdata);
            for folder in [
                "opencode",
                "OpenCode",
                "opencode-desktop",
                "OpenCode Desktop",
            ] {
                search_dirs.push(base.join(folder));
            }
        }
    }
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        if !local.trim().is_empty() {
            let base = PathBuf::from(local);
            for folder in [
                "opencode",
                "OpenCode",
                "opencode-desktop",
                "OpenCode Desktop",
            ] {
                search_dirs.push(base.join(folder));
            }
        }
    }

    let file_names = [
        "opencode.json",
        "opencode.jsonc",
        "config.json",
        "config.jsonc",
        "settings.json",
    ];

    for dir in search_dirs {
        for name in file_names {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    None
}

fn find_opencode_ag_provider() -> Option<AgProviderInfo> {
    let path = find_opencode_config_file()?;
    let content = fs::read_to_string(&path).ok()?;
    let cleaned = strip_json_comments(&content);
    let value: serde_json::Value = serde_json::from_str(&cleaned).ok()?;
    let providers = value
        .get("provider")
        .or_else(|| value.get("providers"))?
        .as_object()?;

    for (name, entry) in providers {
        let has_ag_models = entry
            .get("models")
            .and_then(|m| m.as_object())
            .map(|models| models.keys().any(|k| k.starts_with("ag/")))
            .unwrap_or(false);

        let is_named_ag = name.to_lowercase().contains("antigravity");

        if has_ag_models || is_named_ag {
            let options = entry.get("options")?;
            let base_url = options
                .get("baseURL")
                .or_else(|| options.get("baseUrl"))
                .and_then(|v| v.as_str())?
                .trim_end_matches('/')
                .to_string();

            let raw_key = options
                .get("apiKey")
                .and_then(|v| v.as_str())
                .unwrap_or_default();

            let api_key = if let Some(var) = raw_key
                .strip_prefix("{env:")
                .and_then(|k| k.strip_suffix('}'))
            {
                std::env::var(var).unwrap_or_default()
            } else if let Some(var) = raw_key.strip_prefix('{').and_then(|k| k.strip_suffix('}')) {
                std::env::var(var).unwrap_or_default()
            } else {
                raw_key.to_string()
            };

            let first_model = entry
                .get("models")
                .and_then(|m| m.as_object())
                .and_then(|m| {
                    if m.contains_key("kr/auto") {
                        Some("kr/auto".to_string())
                    } else if m.contains_key("kr/claude-sonnet-4.5") {
                        Some("kr/claude-sonnet-4.5".to_string())
                    } else if m.contains_key("ag/gemini-3.8-flash-high") {
                        Some("ag/gemini-3.8-flash-high".to_string())
                    } else if m.contains_key("ag/gemini-3.8-flash") {
                        Some("ag/gemini-3.8-flash".to_string())
                    } else {
                        m.keys()
                            .find(|k| k.starts_with("ag/") || k.starts_with("kr/"))
                            .cloned()
                    }
                })
                .or_else(|| Some("kr/auto".to_string()));

            return Some(AgProviderInfo {
                base_url,
                api_key,
                model: first_model,
            });
        }
    }

    None
}

fn scan_antigravity() -> Option<DetectedSource> {
    let is_antigravity = std::env::var("ANTIGRAVITY_AGENT").is_ok()
        || home_dir()
            .map(|h| h.join(".gemini/antigravity").is_dir())
            .unwrap_or(false)
        || xdg_config_dir()
            .map(|c| c.join("Antigravity").is_dir())
            .unwrap_or(false);

    // 1. Check if direct API key or base URL is set in environment
    let env_key = std::env::var("ANTIGRAVITY_API_KEY")
        .or_else(|_| std::env::var("GEMINI_API_KEY"))
        .or_else(|_| std::env::var("GOOGLE_API_KEY"))
        .unwrap_or_default();
    let env_url = std::env::var("ANTIGRAVITY_BASE_URL").ok();

    let ag_models = vec![
        "kr/auto".to_string(),
        "kr/claude-sonnet-4.5".to_string(),
        "ag/gemini-3.8-flash-high".to_string(),
        "ag/gemini-3.8-flash".to_string(),
        "ag/gemini-3.8-flash-medium".to_string(),
        "ag/gemini-3.8-flash-low".to_string(),
    ];

    if !env_key.trim().is_empty() {
        let base_url = env_url.unwrap_or_else(|| {
            "https://generativelanguage.googleapis.com/v1beta/openai".to_string()
        });
        return Some(DetectedSource {
            id: "antigravity".to_string(),
            name: "Antigravity".to_string(),
            provider: "openai".to_string(),
            base_url,
            model: Some("ag/gemini-3.8-flash-high".to_string()),
            hint: "Gemini / Antigravity key detected in environment".to_string(),
            has_key: true,
            models: ag_models,
        });
    }

    // 2. Check if local OpenCode has Antigravity (ag/*) models configured
    if let Some(ag) = find_opencode_ag_provider() {
        let has_key = !ag.api_key.trim().is_empty();
        return Some(DetectedSource {
            id: "antigravity".to_string(),
            name: "Antigravity".to_string(),
            provider: "openai".to_string(),
            base_url: ag.base_url,
            model: ag.model,
            hint: "Local Antigravity models (Gemini 3.8 Flash)".to_string(),
            has_key,
            models: ag_models,
        });
    }

    // 3. Fallback: Antigravity IDE / app detected on host
    if is_antigravity {
        return Some(DetectedSource {
            id: "antigravity".to_string(),
            name: "Antigravity".to_string(),
            provider: "openai".to_string(),
            base_url: "https://generativelanguage.googleapis.com/v1beta/openai".to_string(),
            model: Some("gemini-2.5-flash".to_string()),
            hint: "Antigravity IDE detected (نیاز به کلید GEMINI_API_KEY)".to_string(),
            has_key: false,
            models: ag_models,
        });
    }

    None
}

fn resolve_antigravity() -> Result<ResolvedCredentials, String> {
    // 1. Direct env vars
    let env_key = std::env::var("ANTIGRAVITY_API_KEY")
        .or_else(|_| std::env::var("GEMINI_API_KEY"))
        .or_else(|_| std::env::var("GOOGLE_API_KEY"))
        .unwrap_or_default();
    let env_url = std::env::var("ANTIGRAVITY_BASE_URL").ok();

    if !env_key.trim().is_empty() {
        let base_url = env_url.unwrap_or_else(|| {
            "https://generativelanguage.googleapis.com/v1beta/openai".to_string()
        });
        return Ok(ResolvedCredentials {
            provider: "openai".to_string(),
            base_url,
            model: Some("gemini-2.5-flash".to_string()),
            api_key: env_key,
        });
    }

    // 2. Check local OpenCode ag/* provider
    if let Some(ag) = find_opencode_ag_provider() {
        return Ok(ResolvedCredentials {
            provider: "openai".to_string(),
            base_url: ag.base_url,
            model: ag.model,
            api_key: ag.api_key,
        });
    }

    Err("No Antigravity credentials found. Please set GEMINI_API_KEY or enter your API key in Settings.".to_string())
}

// ── Codex Detector ─────────────────────────────────────────────────────────

fn scan_codex() -> Option<DetectedSource> {
    let home = home_dir()?;
    let codex_dir = home.join(".codex");
    let config_path = codex_dir.join("config.toml");
    let auth_path = codex_dir.join("auth.json");

    if !codex_dir.is_dir() {
        return None;
    }

    let mut base_url = "https://api.openai.com/v1".to_string();
    let mut model = Some("gpt-5.5".to_string());
    let mut provider_name = "Codex".to_string();

    if let Ok(content) = fs::read_to_string(&config_path) {
        if let Ok(toml_val) = content.parse::<toml::Value>() {
            if let Some(m) = toml_val.get("model").and_then(|v| v.as_str()) {
                model = Some(m.to_string());
            }
            if let Some(custom) = toml_val
                .get("model_providers")
                .and_then(|v| v.get("custom"))
            {
                if let Some(url) = custom.get("base_url").and_then(|v| v.as_str()) {
                    base_url = url.trim_end_matches('/').to_string();
                }
                if let Some(name) = custom.get("name").and_then(|v| v.as_str()) {
                    provider_name = format!("Codex ({name})");
                }
            }
        }
    }

    let mut has_key = false;
    if let Ok(content) = fs::read_to_string(&auth_path) {
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(key) = json_val.get("OPENAI_API_KEY").and_then(|v| v.as_str()) {
                has_key = !key.trim().is_empty();
            }
        }
    }

    Some(DetectedSource {
        id: "codex".to_string(),
        name: provider_name,
        provider: "openai".to_string(),
        base_url: base_url.clone(),
        model,
        hint: format!("{base_url} · ~/.codex"),
        has_key,
        models: vec![
            "gpt-6-astra".to_string(),
            "gpt-5".to_string(),
            "o3-mini".to_string(),
            "gpt-4o".to_string(),
        ],
    })
}

fn resolve_codex() -> Result<ResolvedCredentials, String> {
    let home = home_dir().ok_or("Cannot locate HOME directory")?;
    let codex_dir = home.join(".codex");
    let config_path = codex_dir.join("config.toml");
    let auth_path = codex_dir.join("auth.json");

    let mut base_url = "https://api.openai.com/v1".to_string();
    let mut model = None;

    if let Ok(content) = fs::read_to_string(&config_path) {
        if let Ok(toml_val) = content.parse::<toml::Value>() {
            if let Some(m) = toml_val.get("model").and_then(|v| v.as_str()) {
                model = Some(m.to_string());
            }
            if let Some(custom) = toml_val
                .get("model_providers")
                .and_then(|v| v.get("custom"))
            {
                if let Some(url) = custom.get("base_url").and_then(|v| v.as_str()) {
                    base_url = url.trim_end_matches('/').to_string();
                }
            }
        }
    }

    let mut api_key = String::new();
    if let Ok(content) = fs::read_to_string(&auth_path) {
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(key) = json_val.get("OPENAI_API_KEY").and_then(|v| v.as_str()) {
                api_key = key.trim().to_string();
            }
        }
    }

    Ok(ResolvedCredentials {
        provider: "openai".to_string(),
        base_url,
        model,
        api_key,
    })
}

// ── Claude Code Detector ───────────────────────────────────────────────────

fn scan_claude_code() -> Option<DetectedSource> {
    let mut api_key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_default();
    let mut base_url = std::env::var("ANTHROPIC_BASE_URL")
        .unwrap_or_else(|_| "https://api.anthropic.com".to_string());
    let mut model = Some("claude-opus-4-6".to_string());

    let home = home_dir();
    let claude_dir = home.as_ref().map(|h| h.join(".claude"));
    let settings_path = claude_dir.as_ref().map(|d| d.join("settings.json"));

    if let Some(path) = settings_path {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(env) = val.get("env") {
                    if api_key.is_empty() {
                        if let Some(k) = env.get("ANTHROPIC_API_KEY").and_then(|v| v.as_str()) {
                            api_key = k.trim().to_string();
                        }
                    }
                    if let Some(u) = env.get("ANTHROPIC_BASE_URL").and_then(|v| v.as_str()) {
                        base_url = u.trim_end_matches('/').to_string();
                    }
                }
                if let Some(m) = val.get("model").and_then(|v| v.as_str()) {
                    model = Some(m.to_string());
                }
            }
        }
    }

    let has_key = !api_key.trim().is_empty();
    let claude_installed = claude_dir.map(|d| d.is_dir()).unwrap_or(false)
        || home
            .as_ref()
            .map(|h| h.join(".claude.json").is_file())
            .unwrap_or(false);

    if !has_key && !claude_installed {
        return None;
    }

    let provider = if base_url.contains("anthropic.com") {
        "anthropic".to_string()
    } else {
        "openai".to_string()
    };

    Some(DetectedSource {
        id: "claude_code".to_string(),
        name: "Claude Code".to_string(),
        provider,
        base_url: base_url.clone(),
        model,
        hint: if has_key {
            format!("{base_url} · ~/.claude/settings.json")
        } else {
            "~/.claude · Claude Code CLI".to_string()
        },
        has_key,
        models: vec![
            "fable".to_string(),
            "claude-opus-4-6".to_string(),
            "claude-sonnet-4-6".to_string(),
            "claude-haiku-4-5".to_string(),
        ],
    })
}

fn resolve_claude_code() -> Result<ResolvedCredentials, String> {
    let mut api_key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_default();
    let mut base_url = std::env::var("ANTHROPIC_BASE_URL")
        .unwrap_or_else(|_| "https://api.anthropic.com".to_string());
    let mut model = Some("claude-opus-4-6".to_string());

    let home = home_dir();
    let claude_dir = home.as_ref().map(|h| h.join(".claude"));
    let settings_path = claude_dir.as_ref().map(|d| d.join("settings.json"));

    if let Some(path) = settings_path {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(env) = val.get("env") {
                    if api_key.is_empty() {
                        if let Some(k) = env.get("ANTHROPIC_API_KEY").and_then(|v| v.as_str()) {
                            api_key = k.trim().to_string();
                        }
                    }
                    if let Some(u) = env.get("ANTHROPIC_BASE_URL").and_then(|v| v.as_str()) {
                        base_url = u.trim_end_matches('/').to_string();
                    }
                }
                if let Some(m) = val.get("model").and_then(|v| v.as_str()) {
                    model = Some(m.to_string());
                }
            }
        }
    }

    let provider = if base_url.contains("anthropic.com") {
        "anthropic".to_string()
    } else {
        "openai".to_string()
    };

    Ok(ResolvedCredentials {
        provider,
        base_url,
        model,
        api_key,
    })
}

// ── OpenCode config / CLI ─────────────────────────────────────────────────

/// Read the first usable OpenCode provider entry.
///
/// Priority:
/// 1. a provider literally named `9router` (the common setup),
/// 2. the first provider that has `options.baseURL`.
///
/// OpenCode itself is only a client. Its config points at an
/// OpenAI-compatible endpoint, so reading the file is enough to call that
/// endpoint directly — the OpenCode app/CLI does not need to be installed.
pub fn read_opencode_endpoint() -> Result<OpenCodeEndpoint, String> {
    let path = find_opencode_config_file().ok_or_else(|| {
        "No OpenCode configuration file found (checked ~/.config/opencode, %APPDATA%, %LOCALAPPDATA%, and workspace).".to_string()
    })?;
    let raw = fs::read_to_string(&path)
        .map_err(|error| format!("Could not read {}: {error}", path.display()))?;
    parse_opencode_endpoint(&raw, &path)
}

fn parse_opencode_endpoint(raw: &str, path: &Path) -> Result<OpenCodeEndpoint, String> {
    let cleaned = strip_json_comments(raw);
    let value: serde_json::Value = serde_json::from_str(&cleaned)
        .map_err(|error| format!("Could not parse {}: {error}", path.display()))?;
    let providers = value
        .get("provider")
        .or_else(|| value.get("providers"))
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| format!("{} has no provider entries.", path.display()))?;

    let (provider_name, entry) = providers
        .get_key_value("9router")
        .or_else(|| {
            providers.iter().find(|(_, entry)| {
                entry
                    .pointer("/options/baseURL")
                    .or_else(|| entry.pointer("/options/baseUrl"))
                    .is_some()
            })
        })
        .ok_or_else(|| {
            format!(
                "{} has no provider with options.baseURL. Add one (for example a 9Router or Ollama endpoint) to use OpenCode without the desktop app.",
                path.display()
            )
        })?;

    let options = entry
        .get("options")
        .ok_or_else(|| format!("Provider '{provider_name}' has no options."))?;
    let base_url = options
        .get("baseURL")
        .or_else(|| options.get("baseUrl"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .trim()
        .trim_end_matches('/')
        .to_string();
    if base_url.is_empty() {
        return Err(format!("Provider '{provider_name}' has an empty base URL."));
    }

    let api_key = options
        .get("apiKey")
        .and_then(serde_json::Value::as_str)
        .map(resolve_opencode_secret)
        .unwrap_or_default();
    let is_local = base_url.contains("127.0.0.1") || base_url.contains("localhost");
    let api_key = if api_key.is_empty() && is_local {
        // Some local gateways reject a missing Authorization header even when
        // they do not validate the token.
        "sk-opencode-local".to_string()
    } else {
        api_key
    };

    let mut models: Vec<String> = entry
        .get("models")
        .and_then(serde_json::Value::as_object)
        .map(|models| models.keys().cloned().collect())
        .unwrap_or_default();

    // A top-level/default model may be written as `provider/model`; expose the
    // endpoint-native id and keep it as the first suggestion.
    let mut preferred = None;
    if let Some(default_model) = value.get("model").and_then(serde_json::Value::as_str) {
        let prefix = format!("{provider_name}/");
        let native = default_model
            .strip_prefix(&prefix)
            .unwrap_or(default_model)
            .trim()
            .to_string();
        if !native.is_empty() {
            preferred = Some(native.clone());
            if !models.iter().any(|model| model == &native) {
                models.push(native);
            }
        }
    }
    models.sort();
    models.dedup();
    if let Some(preferred) = preferred {
        if let Some(index) = models.iter().position(|model| model == &preferred) {
            let model = models.remove(index);
            models.insert(0, model);
        }
    }

    Ok(OpenCodeEndpoint {
        provider_name: provider_name.clone(),
        base_url,
        api_key,
        models,
        config_path: path.to_path_buf(),
    })
}

fn resolve_opencode_secret(raw: &str) -> String {
    let trimmed = raw.trim();
    if let Some(inner) = trimmed
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
    {
        let name = inner.strip_prefix("env:").unwrap_or(inner);
        return std::env::var(name).unwrap_or_default();
    }
    trimmed.to_string()
}

/// Locate the OpenCode executable without spawning it.
pub fn find_opencode_binary() -> Option<PathBuf> {
    if let Ok(custom) = std::env::var("ZEN_OPENCODE_BIN") {
        let custom = custom.trim();
        if !custom.is_empty() {
            let path = PathBuf::from(custom);
            if path.is_file() {
                return Some(path);
            }
            if !custom.contains(std::path::MAIN_SEPARATOR) {
                if let Some(found) = find_in_path(custom) {
                    return Some(found);
                }
            }
        }
    }

    #[cfg(windows)]
    {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            let local = PathBuf::from(local);
            for candidate in [
                local.join("Programs").join("OpenCode").join("OpenCode.exe"),
                local.join("Programs").join("opencode").join("opencode.exe"),
                local
                    .join("Programs")
                    .join("OpenCode")
                    .join("resources")
                    .join("bin")
                    .join("opencode.exe"),
            ] {
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
        if let Some(home) = home_dir() {
            for candidate in [
                home.join(".opencode").join("bin").join("opencode.exe"),
                home.join(".bun").join("bin").join("opencode.exe"),
            ] {
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
        for name in ["opencode.exe", "opencode.cmd", "opencode.bat"] {
            if let Some(found) = find_in_path(name) {
                return Some(found);
            }
        }
    }

    #[cfg(not(windows))]
    {
        for name in ["opencode", "opencode-cli"] {
            if let Some(found) = find_in_path(name) {
                return Some(found);
            }
        }
        if let Some(home) = home_dir() {
            for candidate in [
                home.join(".opencode").join("bin").join("opencode"),
                home.join(".local").join("bin").join("opencode"),
                home.join(".bun").join("bin").join("opencode"),
            ] {
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
        for path in ["/opt/homebrew/bin/opencode", "/usr/local/bin/opencode"] {
            let path = PathBuf::from(path);
            if path.is_file() {
                return Some(path);
            }
        }
    }

    None
}

pub fn opencode_cli_available() -> bool {
    find_opencode_binary().is_some()
}

fn find_in_path(name: &str) -> Option<PathBuf> {
    crate::platform::find_in_path(name)
}

fn scan_opencode() -> Option<DetectedSource> {
    let endpoint = read_opencode_endpoint().ok()?;
    let is_local =
        endpoint.base_url.contains("127.0.0.1") || endpoint.base_url.contains("localhost");
    let has_key = !endpoint.api_key.is_empty() || is_local;
    let first_model = endpoint.models.first().cloned();

    Some(DetectedSource {
        id: "opencode".to_string(),
        name: format!("OpenCode ({})", endpoint.provider_name),
        provider: "openai".to_string(),
        base_url: endpoint.base_url.clone(),
        model: first_model,
        hint: format!("{} · {}", endpoint.base_url, endpoint.config_path.display()),
        has_key,
        models: endpoint.models,
    })
}

fn resolve_opencode() -> Result<ResolvedCredentials, String> {
    let endpoint = read_opencode_endpoint()?;
    Ok(ResolvedCredentials {
        provider: "openai".to_string(),
        base_url: endpoint.base_url,
        model: endpoint.models.first().cloned(),
        api_key: endpoint.api_key,
    })
}

fn scan_opencode_free() -> DetectedSource {
    DetectedSource {
        id: "opencode_free".to_string(),
        name: "OpenCode Free".to_string(),
        provider: "openai".to_string(),
        base_url: OPENCODE_FREE_BASE_URL.to_string(),
        model: Some(OPENCODE_FREE_DEFAULT_MODEL.to_string()),
        hint: "Public free tier · no app, no CLI, no API key".to_string(),
        has_key: true,
        models: OPENCODE_FREE_FALLBACK_MODELS
            .iter()
            .map(|model| model.to_string())
            .collect(),
    }
}

fn resolve_opencode_free() -> Result<ResolvedCredentials, String> {
    Ok(ResolvedCredentials {
        provider: "openai".to_string(),
        base_url: OPENCODE_FREE_BASE_URL.to_string(),
        model: Some(OPENCODE_FREE_DEFAULT_MODEL.to_string()),
        api_key: String::new(),
    })
}

// ── Environment OpenAI Detector ────────────────────────────────────────────

fn scan_env_openai() -> Option<DetectedSource> {
    let key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
    if key.trim().is_empty() {
        return None;
    }

    Some(DetectedSource {
        id: "env_openai".to_string(),
        name: "OpenAI (Environment)".to_string(),
        provider: "openai".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        model: Some("gpt-4o".to_string()),
        hint: "OPENAI_API_KEY set in environment".to_string(),
        has_key: true,
        models: vec!["gpt-4o".to_string(), "gpt-4o-mini".to_string()],
    })
}

fn resolve_env_openai() -> Result<ResolvedCredentials, String> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
    Ok(ResolvedCredentials {
        provider: "openai".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        model: Some("gpt-4o".to_string()),
        api_key,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_does_not_panic() {
        let sources = scan_local_sources();
        for s in &sources {
            println!(
                "Detected: id={}, name={}, has_key={}, model={:?}",
                s.id, s.name, s.has_key, s.model
            );
            assert!(!s.id.is_empty());
            assert!(!s.name.is_empty());
        }
    }

    #[test]
    fn opencode_free_is_always_detected_without_a_key() {
        let sources = scan_local_sources();
        let source = sources
            .iter()
            .find(|source| source.id == "opencode_free")
            .expect("free source is always present");
        assert!(source.has_key);
        assert_eq!(source.base_url, OPENCODE_FREE_BASE_URL);
        assert_eq!(source.model.as_deref(), Some(OPENCODE_FREE_DEFAULT_MODEL));

        let resolved = resolve_source("opencode_free").expect("free source resolves");
        assert_eq!(resolved.provider, "openai");
        assert_eq!(resolved.base_url, OPENCODE_FREE_BASE_URL);
        assert!(resolved.api_key.is_empty(), "free lane must not need a key");
    }

    #[test]
    fn free_endpoint_detection_is_host_exact() {
        assert!(is_opencode_free_endpoint("https://opencode.ai"));
        assert!(is_opencode_free_endpoint(
            "https://opencode.ai/inference/v1"
        ));
        assert!(is_opencode_free_endpoint("https://www.opencode.ai/zen/v1"));
        assert!(!is_opencode_free_endpoint(
            "https://opencode.ai.evil.test/v1"
        ));
        assert!(!is_opencode_free_endpoint("http://91.217.166.11:20130/v1"));
    }

    #[test]
    fn parses_opencode_provider_models_without_cli() {
        let raw = r#"
        {
          "model": "9router/ag/gemini-3.8-flash-high",
          "provider": {
            "9router": {
              "options": {
                "baseURL": "http://localhost:20130/v1/",
                "apiKey": "test-key"
              },
              "models": {
                "ag/gemini-3.8-flash": { "name": "Gemini Flash" },
                "ag/claude-sonnet-4-6": { "name": "Claude Sonnet" }
              }
            }
          }
        }
        "#;

        let endpoint =
            parse_opencode_endpoint(raw, Path::new("/tmp/opencode.json")).expect("config parses");
        assert_eq!(endpoint.provider_name, "9router");
        assert_eq!(endpoint.base_url, "http://localhost:20130/v1");
        assert_eq!(endpoint.api_key, "test-key");
        assert_eq!(
            endpoint.models.first().map(String::as_str),
            Some("ag/gemini-3.8-flash-high"),
            "the configured default model should be first"
        );
        assert!(endpoint
            .models
            .iter()
            .any(|model| model == "ag/claude-sonnet-4-6"));
        assert!(endpoint
            .models
            .iter()
            .any(|model| model == "ag/gemini-3.8-flash"));
    }

    #[test]
    fn rejects_opencode_config_without_endpoint() {
        let raw = r#"{ "provider": { "local": { "models": {} } } }"#;
        let error = parse_opencode_endpoint(raw, Path::new("/tmp/opencode.json"))
            .expect_err("no baseURL should fail");
        assert!(
            error.contains("options.baseURL"),
            "unexpected error: {error}"
        );
    }
}
