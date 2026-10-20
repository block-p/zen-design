//! Local model-connection settings.
//!
//! Everything lives in a single TOML file owned by the user:
//!
//! * `ZEN_CONFIG_PATH` wins when set.
//! * otherwise `$XDG_CONFIG_HOME/zen-design/config.toml`
//! * otherwise `~/.config/zen-design/config.toml`
//!
//! API keys are stored in that file with `0600` permissions and are never
//! returned to the browser: `/api/settings` exposes presence + source only.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{info, warn};

pub const PROVIDER_TEMPLATE: &str = "template";
pub const PROVIDER_OPENAI: &str = "openai";
pub const PROVIDER_ANTHROPIC: &str = "anthropic";
pub const PROVIDER_OPENCODE: &str = "opencode";

pub const KNOWN_PROVIDERS: [&str; 4] = [
    PROVIDER_TEMPLATE,
    PROVIDER_OPENAI,
    PROVIDER_ANTHROPIC,
    PROVIDER_OPENCODE,
];

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProviderProfile {
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "default_provider")]
    pub provider: String,
    #[serde(default)]
    pub active_source: Option<String>,
    #[serde(default)]
    pub openai: ProviderProfile,
    #[serde(default)]
    pub anthropic: ProviderProfile,
    #[serde(default)]
    pub opencode: ProviderProfile,
}

fn default_provider() -> String {
    PROVIDER_TEMPLATE.to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            active_source: None,
            openai: ProviderProfile::default(),
            anthropic: ProviderProfile {
                base_url: "https://api.anthropic.com".to_string(),
                model: "claude-opus-4-6".to_string(),
                api_key: String::new(),
            },
            opencode: ProviderProfile::default(),
        }
    }
}

impl Settings {
    pub fn config_path() -> PathBuf {
        if let Ok(path) = std::env::var("ZEN_CONFIG_PATH") {
            if !path.trim().is_empty() {
                return PathBuf::from(path);
            }
        }

        let base = std::env::var("XDG_CONFIG_HOME")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var("HOME")
                    .ok()
                    .map(|home| Path::new(&home).join(".config"))
            })
            .unwrap_or_else(|| PathBuf::from(".config"));

        base.join("zen-design").join("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        match fs::read_to_string(&path) {
            Ok(raw) => match toml::from_str::<Settings>(&raw) {
                Ok(settings) => settings,
                Err(error) => {
                    warn!("Ignoring invalid settings at {}: {error}", path.display());
                    Settings::default()
                }
            },
            Err(_) => Settings::default(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("Could not create {}: {error}", parent.display()))?;
            restrict_dir(parent);
        }

        let body = toml::to_string_pretty(self)
            .map_err(|error| format!("Could not serialize settings: {error}"))?;

        let temp = path.with_extension("toml.tmp");
        fs::write(&temp, body).map_err(|error| format!("Could not write settings: {error}"))?;
        restrict_file(&temp);
        fs::rename(&temp, &path).map_err(|error| format!("Could not store settings: {error}"))?;

        info!("Saved model settings to {}", path.display());
        Ok(())
    }

    pub fn active_profile(&self) -> &ProviderProfile {
        match self.provider.as_str() {
            PROVIDER_OPENAI => &self.openai,
            PROVIDER_ANTHROPIC => &self.anthropic,
            PROVIDER_OPENCODE => &self.opencode,
            _ => &self.opencode,
        }
    }

    pub fn profile_mut(&mut self, provider: &str) -> Option<&mut ProviderProfile> {
        match provider {
            PROVIDER_OPENAI => Some(&mut self.openai),
            PROVIDER_ANTHROPIC => Some(&mut self.anthropic),
            PROVIDER_OPENCODE => Some(&mut self.opencode),
            _ => None,
        }
    }

    /// OpenAI-compatible key: profile first, then `OPENAI_API_KEY`.
    pub fn openai_key(&self) -> String {
        first_non_empty([&self.openai.api_key, &env_value("OPENAI_API_KEY")])
    }

    /// Anthropic key: profile first, then `ANTHROPIC_API_KEY`.
    pub fn anthropic_key(&self) -> String {
        first_non_empty([&self.anthropic.api_key, &env_value("ANTHROPIC_API_KEY")])
    }

    /// Human label for the active connection, e.g. `9router via OpenAI`.
    pub fn provider_label(&self) -> String {
        match self.provider.as_str() {
            PROVIDER_OPENAI => "OpenAI-compatible".to_string(),
            PROVIDER_ANTHROPIC => "Anthropic".to_string(),
            PROVIDER_OPENCODE => "OpenCode agent".to_string(),
            _ => "Offline template".to_string(),
        }
    }

    pub fn to_public_json(&self) -> serde_json::Value {
        json!({
            "provider": self.provider,
            "active_source": self.active_source,
            "config_path": Self::config_path().display().to_string(),
            "openai": public_profile(&self.openai, Some("OPENAI_API_KEY")),
            "anthropic": public_profile(&self.anthropic, Some("ANTHROPIC_API_KEY")),
            "opencode": public_profile(&self.opencode, None),
        })
    }
}

fn public_profile(profile: &ProviderProfile, env: Option<&str>) -> serde_json::Value {
    let env_value = env.map(env_value).unwrap_or_default();
    let (api_key_set, api_key_source) = if !profile.api_key.trim().is_empty() {
        (true, "config")
    } else if !env_value.trim().is_empty() {
        (true, "env")
    } else {
        (false, "none")
    };

    json!({
        "base_url": profile.base_url,
        "model": profile.model,
        "api_key_set": api_key_set,
        "api_key_source": api_key_source,
    })
}

fn env_value(name: &str) -> String {
    std::env::var(name).unwrap_or_default()
}

fn first_non_empty<'a>(values: impl IntoIterator<Item = &'a String>) -> String {
    values
        .into_iter()
        .map(|value| value.trim())
        .find(|value| !value.is_empty())
        .unwrap_or_default()
        .to_string()
}

#[cfg(unix)]
fn restrict_file(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o600));
}

#[cfg(not(unix))]
fn restrict_file(_path: &Path) {}

#[cfg(unix)]
fn restrict_dir(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
}

#[cfg(not(unix))]
fn restrict_dir(_path: &Path) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_provider_is_offline() {
        let settings = Settings::default();
        assert_eq!(settings.provider, PROVIDER_TEMPLATE);
        assert!(settings.anthropic.base_url.contains("anthropic.com"));
    }

    #[test]
    fn public_json_never_contains_the_raw_key() {
        let mut settings = Settings::default();
        settings.openai.api_key = "sk-secret-value".to_string();
        settings.openai.base_url = "http://example.test/v1".to_string();
        let view = settings.to_public_json().to_string();
        assert!(!view.contains("sk-secret-value"));
        assert!(view.contains("\"api_key_set\":true"));
        assert!(view.contains("\"api_key_source\":\"config\""));
    }

    #[test]
    fn profile_mut_targets_only_known_providers() {
        let mut settings = Settings::default();
        assert!(settings.profile_mut(PROVIDER_OPENAI).is_some());
        assert!(settings.profile_mut("antigravity").is_none());
    }
}
