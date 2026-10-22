use serde::Deserialize;

use crate::i18n::Language;

pub const OPENCODE_FREE_BASE_URL: &str = "https://opencode.ai";
pub const OPENCODE_FREE_DEFAULT_MODEL: &str = "nemotron-3.5-lightning-free";

/// Connection choices shown in the settings dialog.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProviderId {
    OpenAi,
    Anthropic,
    OpenCode,
    Template,
}

pub const PROVIDERS: [ProviderId; 4] = [
    ProviderId::OpenAi,
    ProviderId::Anthropic,
    ProviderId::OpenCode,
    ProviderId::Template,
];

impl ProviderId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OpenAi => "openai",
            Self::Anthropic => "anthropic",
            Self::OpenCode => "opencode",
            Self::Template => "template",
        }
    }

    pub fn label(self, lang: Language) -> &'static str {
        match (self, lang) {
            (Self::OpenAi, Language::En) => "OpenAI / Compatible API",
            (Self::OpenAi, Language::Fa) => "سازگار با OpenAI / API",
            (Self::Anthropic, _) => "Anthropic",
            (Self::OpenCode, Language::En) => "OpenCode / Free",
            (Self::OpenCode, Language::Fa) => "OpenCode / رایگان",
            (Self::Template, Language::En) => "Offline template",
            (Self::Template, Language::Fa) => "قالب آفلاین",
        }
    }

    pub fn hint(self, lang: Language) -> &'static str {
        match (self, lang) {
            (Self::OpenAi, Language::En) => {
                "OpenAI, 9Router, LM Studio, Ollama, vLLM or custom baseURL"
            }
            (Self::OpenAi, Language::Fa) => "OpenAI، ناین‌روتر، LM Studio، اولاما یا baseURL دلخواه",
            (Self::Anthropic, Language::En) => "Native Messages API · Claude Opus / Sonnet",
            (Self::Anthropic, Language::Fa) => "Messages API بومی · Claude Opus / Sonnet",
            (Self::OpenCode, Language::En) => "Free tier with no app/key, or read opencode.json",
            (Self::OpenCode, Language::Fa) => "نسخهٔ رایگان بدون اپ/کلید، یا خواندن opencode.json",
            (Self::Template, Language::En) => "No model, zero network, deterministic",
            (Self::Template, Language::Fa) => "بدون مدل، بدون شبکه، قطعی",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "openai" => Self::OpenAi,
            "anthropic" => Self::Anthropic,
            "opencode" => Self::OpenCode,
            _ => Self::Template,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct ProviderView {
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub api_key_set: bool,
    #[serde(default)]
    pub api_key_source: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SettingsView {
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub active_source: Option<String>,
    #[serde(default)]
    pub config_path: String,
    #[serde(default)]
    pub openai: ProviderView,
    #[serde(default)]
    pub anthropic: ProviderView,
    #[serde(default)]
    pub opencode: ProviderView,
}

impl SettingsView {
    pub fn profile(&self, provider: &str) -> &ProviderView {
        match provider {
            "openai" => &self.openai,
            "anthropic" => &self.anthropic,
            "opencode" => &self.opencode,
            _ => &self.opencode,
        }
    }

    pub fn summary(&self) -> String {
        match self.provider.as_str() {
            "openai" => {
                let profile = &self.openai;
                let host = profile
                    .base_url
                    .split("://")
                    .nth(1)
                    .and_then(|rest| rest.split('/').next())
                    .filter(|host| !host.is_empty())
                    .unwrap_or("OpenAI-compatible");
                if profile.model.is_empty() {
                    format!("{host} · no model")
                } else {
                    format!("{host} · {}", profile.model)
                }
            }
            "anthropic" => {
                let model = &self.anthropic.model;
                if model.is_empty() {
                    "Anthropic · no model".to_string()
                } else {
                    format!("Anthropic · {model}")
                }
            }
            "opencode" => {
                if self.opencode.model.is_empty() {
                    "OpenCode · default model".to_string()
                } else {
                    format!("OpenCode · {}", self.opencode.model)
                }
            }
            _ => "Offline template".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModelsResponse {
    #[serde(default)]
    pub models: Vec<String>,
    /// Present when the endpoint could not be asked for a model list; the
    /// picker treats that as "free text only" and stays silent.
    #[serde(default)]
    #[allow(dead_code)]
    pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ImportResponse {
    pub settings: SettingsView,
    #[serde(default)]
    pub models: Vec<String>,
    #[serde(default)]
    pub imported_from: String,
    #[serde(default)]
    pub warning: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ErrorResponse {
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DetectedSource {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub base_url: String,
    pub model: Option<String>,
    pub hint: String,
    pub has_key: bool,
    #[serde(default)]
    pub models: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DetectedSourcesResponse {
    #[serde(default)]
    pub sources: Vec<DetectedSource>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApplyDetectedResponse {
    pub settings: SettingsView,
    #[serde(default)]
    pub models: Vec<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub applied_id: String,
}
