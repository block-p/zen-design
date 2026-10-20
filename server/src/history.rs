//! Persistent chat and prompt history storage.
//!
//! Stores user briefs and generation metadata in `~/.config/zen-design/history.json`
//! so prompts, modes, and design system choices can be browsed and restored across sessions.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tracing::{info, warn};

const MAX_HISTORY_ITEMS: usize = 150;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatHistoryItem {
    pub id: String,
    pub prompt: String,
    pub design_system: String,
    pub mode: String,
    #[serde(default)]
    pub deep: bool,
    pub timestamp: u64,
}

fn history_path() -> PathBuf {
    if let Ok(path) = std::env::var("ZEN_HISTORY_PATH") {
        if !path.trim().is_empty() {
            return PathBuf::from(path);
        }
    }

    let base = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .map(|home| Path::new(&home).join(".config"))
        })
        .unwrap_or_else(|| PathBuf::from(".config"));

    base.join("zen-design").join("history.json")
}

pub fn load_history() -> Vec<ChatHistoryItem> {
    let path = history_path();
    if !path.is_file() {
        return Vec::new();
    }

    match fs::read_to_string(&path) {
        Ok(raw) => match serde_json::from_str::<Vec<ChatHistoryItem>>(&raw) {
            Ok(items) => items,
            Err(err) => {
                warn!("Ignoring corrupt history file at {}: {err}", path.display());
                Vec::new()
            }
        },
        Err(_) => Vec::new(),
    }
}

pub fn save_history(items: &[ChatHistoryItem]) -> Result<(), String> {
    let path = history_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let json = serde_json::to_string_pretty(items).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

pub fn add_history_entry(
    prompt: String,
    design_system: String,
    mode: String,
    deep: bool,
) -> ChatHistoryItem {
    let mut items = load_history();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let id = format!("{}_{}", timestamp, items.len() + 1);

    let item = ChatHistoryItem {
        id: id.clone(),
        prompt,
        design_system,
        mode,
        deep,
        timestamp,
    };

    // Prepend so newest is first
    items.insert(0, item.clone());
    if items.len() > MAX_HISTORY_ITEMS {
        items.truncate(MAX_HISTORY_ITEMS);
    }

    if let Err(err) = save_history(&items) {
        warn!("Failed to save chat history: {err}");
    } else {
        info!("Saved prompt to chat history (id={id})");
    }

    item
}

pub fn delete_history_entry(id: &str) -> bool {
    let mut items = load_history();
    let initial_len = items.len();
    items.retain(|item| item.id != id);
    if items.len() < initial_len {
        let _ = save_history(&items);
        true
    } else {
        false
    }
}

pub fn clear_history() -> Result<(), String> {
    save_history(&[])
}
