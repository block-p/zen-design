//! Auto-detection of local AI agent and CLI credentials.
//!
//! Scans the host system for existing setups:
//! * Codex (`~/.codex/config.toml` + `~/.codex/auth.json`)
//! * Claude Code (`ANTHROPIC_API_KEY` + `~/.claude.json`)
//! * OpenCode (`~/.config/opencode/opencode.json`)
//! * Environment variables (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`)

use std::fs;
use std::path::PathBuf;

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

    // 5. Check Environment OPENAI_API_KEY
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

fn find_opencode_ag_provider() -> Option<AgProviderInfo> {
    let config_dir = xdg_config_dir()?.join("opencode");
    let candidate1 = config_dir.join("opencode.json");
    let candidate2 = config_dir.join("opencode.jsonc");
    let path = if candidate1.is_file() {
        Some(candidate1)
    } else if candidate2.is_file() {
        Some(candidate2)
    } else {
        None
    }?;

    let content = fs::read_to_string(&path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    let providers = value.get("provider")?.as_object()?;

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

            let api_key = if let Some(var) = raw_key.strip_prefix("{env:").and_then(|k| k.strip_suffix('}')) {
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
                        m.keys().find(|k| k.starts_with("ag/") || k.starts_with("kr/")).cloned()
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
        || home.as_ref().map(|h| h.join(".claude.json").is_file()).unwrap_or(false);

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

// ── OpenCode Detector ──────────────────────────────────────────────────────

fn scan_opencode() -> Option<DetectedSource> {
    let config_dir = xdg_config_dir()?.join("opencode");
    let candidate1 = config_dir.join("opencode.json");
    let candidate2 = config_dir.join("opencode.jsonc");
    let path = if candidate1.is_file() {
        Some(candidate1)
    } else if candidate2.is_file() {
        Some(candidate2)
    } else {
        None
    }?;

    let content = fs::read_to_string(&path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    let providers = value.get("provider")?.as_object()?;

    let (name, entry) = providers.get_key_value("9router").or_else(|| {
        providers.iter().find(|(_, entry)| {
            entry
                .pointer("/options/baseURL")
                .or_else(|| entry.pointer("/options/baseUrl"))
                .is_some()
        })
    })?;

    let options = entry.get("options")?;
    let base_url = options
        .get("baseURL")
        .or_else(|| options.get("baseUrl"))
        .and_then(|v| v.as_str())?
        .trim_end_matches('/')
        .to_string();

    let has_key = options
        .get("apiKey")
        .and_then(|v| v.as_str())
        .map(|k| !k.trim().is_empty())
        .unwrap_or(false);

    let detected_models: Vec<String> = entry
        .get("models")
        .and_then(|m| m.as_object())
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();

    let mut models = if !detected_models.is_empty() {
        detected_models
    } else {
        vec!["opencode/glm-5.3-flash-free".to_string()]
    };
    if base_url.contains("20130") || base_url.contains("9router") {
        if !models.contains(&"kr/auto".to_string()) {
            models.insert(0, "kr/auto".to_string());
        }
        if !models.contains(&"kr/claude-sonnet-4.5".to_string()) {
            models.insert(1, "kr/claude-sonnet-4.5".to_string());
        }
    }
    let first_model = models.first().cloned();

    Some(DetectedSource {
        id: "opencode".to_string(),
        name: format!("OpenCode ({name})"),
        provider: "openai".to_string(),
        base_url: base_url.clone(),
        model: first_model,
        hint: format!("{base_url} · ~/.config/opencode"),
        has_key,
        models,
    })
}

fn resolve_opencode() -> Result<ResolvedCredentials, String> {
    let config_dir = xdg_config_dir()
        .ok_or("Cannot locate config directory")?
        .join("opencode");
    let candidate1 = config_dir.join("opencode.json");
    let candidate2 = config_dir.join("opencode.jsonc");
    let path = if candidate1.is_file() {
        Some(candidate1)
    } else if candidate2.is_file() {
        Some(candidate2)
    } else {
        None
    }
    .ok_or("No OpenCode configuration file found")?;

    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let value: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    let providers = value
        .get("provider")
        .and_then(|v| v.as_object())
        .ok_or("No providers found in opencode config")?;

    let (_, entry) = providers
        .get_key_value("9router")
        .or_else(|| {
            providers.iter().find(|(_, entry)| {
                entry
                    .pointer("/options/baseURL")
                    .or_else(|| entry.pointer("/options/baseUrl"))
                    .is_some()
            })
        })
        .ok_or("No suitable provider in opencode config")?;

    let options = entry.get("options").ok_or("No options in provider")?;
    let base_url = options
        .get("baseURL")
        .or_else(|| options.get("baseUrl"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .trim_end_matches('/')
        .to_string();

    let api_key = options
        .get("apiKey")
        .and_then(|v| v.as_str())
        .map(|key| {
            if let Some(var) = key.strip_prefix('{').and_then(|k| k.strip_suffix('}')) {
                std::env::var(var).unwrap_or_default()
            } else {
                key.to_string()
            }
        })
        .unwrap_or_default();

    let detected_models: Vec<String> = entry
        .get("models")
        .and_then(|m| m.as_object())
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();
    let first_model = detected_models.first().cloned();

    Ok(ResolvedCredentials {
        provider: "openai".to_string(),
        base_url,
        model: first_model,
        api_key,
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
            println!("Detected: id={}, name={}, has_key={}, model={:?}", s.id, s.name, s.has_key, s.model);
            assert!(!s.id.is_empty());
            assert!(!s.name.is_empty());
        }
    }
}
