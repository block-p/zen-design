//! Persistent chat and multi-session project history storage.
//!
//! Stores user briefs and generation metadata in `~/.config/zen-design/history.json`
//! and isolated session folders in `workspace/sessions/{id}/` so projects, artifacts,
//! and past designs can be browsed, previewed, and restored across sessions.

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
    #[serde(default)]
    pub has_artifact: bool,
}

fn history_path() -> PathBuf {
    if let Ok(path) = std::env::var("ZEN_HISTORY_PATH") {
        if !path.trim().is_empty() {
            return PathBuf::from(path);
        }
    }
    crate::platform::app_history_file()
}

pub fn sessions_dir(workspace: &Path) -> PathBuf {
    workspace.join("sessions")
}

pub fn session_artifact_path(workspace: &Path, id: &str) -> PathBuf {
    sessions_dir(workspace).join(id).join("index.html")
}

pub fn session_meta_path(workspace: &Path, id: &str) -> PathBuf {
    sessions_dir(workspace).join(id).join("meta.json")
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn render_session_placeholder_html(
    _id: &str,
    prompt: &str,
    design_system: &str,
    mode: &str,
) -> String {
    let clean_title = if prompt.trim().is_empty() {
        "New Project Canvas".to_string()
    } else {
        let first_line = prompt.lines().next().unwrap_or("").trim();
        if first_line.chars().count() > 60 {
            let truncated: String = first_line.chars().take(60).collect();
            format!("{truncated}…")
        } else {
            first_line.to_string()
        }
    };
    let clean_prompt = if prompt.trim().is_empty() {
        "Project canvas ready. Enter your creative brief in the agent panel to compose a slide deck, prototype, or kinetic motion piece.".to_string()
    } else {
        prompt.trim().to_string()
    };

    let title_esc = escape_html(&clean_title);
    let prompt_esc = escape_html(&clean_prompt);
    let ds_esc = escape_html(design_system);
    let mode_esc = escape_html(mode);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ZenDesign — {title_esc}</title>
  <style>
    *, *::before, *::after {{ box-sizing: border-box; margin: 0; padding: 0; }}
    body {{
      background: #0d0e12;
      color: #e2e8f0;
      font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      padding: 24px;
      text-align: center;
    }}
    .canvas-card {{
      max-width: 600px;
      width: 100%;
      background: #161822;
      border: 1px solid #2b2d3d;
      border-radius: 16px;
      padding: 40px 32px;
      box-shadow: 0 20px 40px rgba(0,0,0,0.4);
    }}
    .icon {{
      width: 48px;
      height: 48px;
      margin: 0 auto 16px;
      border-radius: 12px;
      background: rgba(94, 106, 210, 0.15);
      color: #5e6ad2;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 24px;
    }}
    h1 {{ font-size: 20px; font-weight: 600; margin-bottom: 12px; color: #f8fafc; word-break: break-word; }}
    p {{ font-size: 14px; color: #94a3b8; line-height: 1.6; word-break: break-word; white-space: pre-wrap; text-align: left; }}
    .badge-row {{ display: flex; gap: 8px; justify-content: center; margin-top: 20px; flex-wrap: wrap; }}
    .badge {{
      font-size: 11px;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      color: #a5b4fc;
      background: rgba(99, 102, 241, 0.12);
      border: 1px solid rgba(99, 102, 241, 0.25);
      padding: 4px 10px;
      border-radius: 9999px;
    }}
    .hint {{
      margin-top: 24px;
      display: inline-block;
      font-size: 12px;
      color: #64748b;
      background: #10121a;
      padding: 6px 14px;
      border-radius: 9999px;
      border: 1px solid #1e2130;
    }}
  </style>
</head>
<body>
  <div class="canvas-card">
    <div class="icon">✦</div>
    <h1>{title_esc}</h1>
    <p>{prompt_esc}</p>
    <div class="badge-row">
      <span class="badge">{ds_esc}</span>
      <span class="badge">{mode_esc}</span>
    </div>
    <div class="hint">Project Session Initialized</div>
  </div>
</body>
</html>"#
    )
}

pub fn ensure_session_artifact(
    workspace: &Path,
    id: &str,
    prompt: &str,
    design_system: &str,
    mode: &str,
) -> PathBuf {
    let sdir = sessions_dir(workspace).join(id);
    let _ = fs::create_dir_all(&sdir);
    let art = sdir.join("index.html");

    // If it already exists and is non-empty, return it
    if art.is_file() {
        if let Ok(meta) = fs::metadata(&art) {
            if meta.len() > 0 {
                return art;
            }
        }
    }

    // Try copying from workspace/index.html if available and non-empty
    let ws_index = workspace.join("index.html");
    let mut copied = false;
    if ws_index.is_file() {
        if let Ok(content) = fs::read_to_string(&ws_index) {
            if !content.trim().is_empty() {
                copied = fs::write(&art, content).is_ok();
            }
        }
    }

    if !copied {
        let html = render_session_placeholder_html(id, prompt, design_system, mode);
        let _ = fs::write(&art, html);
    }

    info!("Ensured session artifact exists at {:?}", art);
    art
}

pub fn mark_has_artifact(workspace: &Path, id: &str) -> Result<(), String> {
    let mut items = load_history(workspace);
    if let Some(it) = items.iter_mut().find(|it| it.id == id) {
        it.has_artifact = true;
        let _ = save_history(&items);
    }
    let meta_file = session_meta_path(workspace, id);
    if meta_file.is_file() {
        if let Ok(content) = fs::read_to_string(&meta_file) {
            if let Ok(mut item) = serde_json::from_str::<ChatHistoryItem>(&content) {
                item.has_artifact = true;
                if let Ok(updated) = serde_json::to_string_pretty(&item) {
                    let _ = fs::write(&meta_file, updated);
                }
            }
        }
    }
    Ok(())
}

pub fn heal_all_sessions(workspace: &Path) {
    let sdir = sessions_dir(workspace);
    if !sdir.is_dir() {
        return;
    }

    let entries = match fs::read_dir(&sdir) {
        Ok(e) => e,
        Err(_) => return,
    };

    let mut healed_count = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let id = match path.file_name().and_then(|n| n.to_str()) {
            Some(id) => id.to_string(),
            None => continue,
        };

        let art = path.join("index.html");
        let needs_heal = !art.is_file() || fs::metadata(&art).map(|m| m.len() == 0).unwrap_or(true);

        if needs_heal {
            let meta_file = path.join("meta.json");
            let (prompt, design_system, mode) = if let Ok(content) = fs::read_to_string(&meta_file) {
                if let Ok(item) = serde_json::from_str::<ChatHistoryItem>(&content) {
                    (item.prompt, item.design_system, item.mode)
                } else {
                    (String::new(), "linear".to_string(), "prototype".to_string())
                }
            } else {
                (String::new(), "linear".to_string(), "prototype".to_string())
            };

            let html = render_session_placeholder_html(&id, &prompt, &design_system, &mode);
            if fs::write(&art, html).is_ok() {
                healed_count += 1;
                if !meta_file.is_file() {
                    let item = ChatHistoryItem {
                        id: id.clone(),
                        prompt,
                        design_system,
                        mode,
                        deep: false,
                        timestamp: 0,
                        has_artifact: true,
                    };
                    if let Ok(json) = serde_json::to_string_pretty(&item) {
                        let _ = fs::write(&meta_file, json);
                    }
                }
            }
        }
    }

    let mut history_items = load_history(workspace);
    let mut history_changed = false;
    for item in &mut history_items {
        let art = session_artifact_path(workspace, &item.id);
        if !art.is_file() || fs::metadata(&art).map(|m| m.len() == 0).unwrap_or(true) {
            let _ = ensure_session_artifact(workspace, &item.id, &item.prompt, &item.design_system, &item.mode);
        }
        if !item.has_artifact {
            item.has_artifact = true;
            history_changed = true;
        }
    }

    if history_changed {
        let _ = save_history(&history_items);
    }

    if healed_count > 0 {
        info!("Healed {healed_count} session folders with missing index.html");
    }
}

pub fn load_history(workspace: &Path) -> Vec<ChatHistoryItem> {
    let path = history_path();
    let mut items: Vec<ChatHistoryItem> = if path.is_file() {
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
    } else {
        Vec::new()
    };

    // Update has_artifact status and populate from disk if needed
    for item in &mut items {
        item.has_artifact = session_artifact_path(workspace, &item.id).is_file();
    }

    items
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
    workspace: &Path,
    id: String,
    prompt: String,
    design_system: String,
    mode: String,
    deep: bool,
) -> ChatHistoryItem {
    let mut items = load_history(workspace);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // 1. Immediately isolate project folder and create index.html right now
    let _ = ensure_session_artifact(workspace, &id, &prompt, &design_system, &mode);

    let item = ChatHistoryItem {
        id: id.clone(),
        prompt,
        design_system,
        mode,
        deep,
        timestamp,
        has_artifact: true,
    };

    // 2. Ensure session directory exists and write meta.json
    let sdir = sessions_dir(workspace).join(&id);
    let _ = fs::create_dir_all(&sdir);
    if let Ok(meta_json) = serde_json::to_string_pretty(&item) {
        let _ = fs::write(session_meta_path(workspace, &id), meta_json);
    }

    // 3. Prepend or update existing entry
    if let Some(idx) = items.iter().position(|it| it.id == id) {
        items[idx] = item.clone();
    } else {
        items.insert(0, item.clone());
    }

    if items.len() > MAX_HISTORY_ITEMS {
        items.truncate(MAX_HISTORY_ITEMS);
    }

    if let Err(err) = save_history(&items) {
        warn!("Failed to save chat history: {err}");
    } else {
        info!("Saved prompt and session to chat history (id={id})");
    }

    item
}

pub fn save_session_artifact(workspace: &Path, id: &str, html: &str) -> Result<(), String> {
    let sdir = sessions_dir(workspace).join(id);
    fs::create_dir_all(&sdir).map_err(|e| e.to_string())?;
    let path = sdir.join("index.html");
    fs::write(&path, html).map_err(|e| e.to_string())?;
    info!(
        "Saved session artifact to {:?} ({} bytes)",
        path,
        html.len()
    );

    let _ = mark_has_artifact(workspace, id);
    Ok(())
}

pub fn activate_session(workspace: &Path, id: &str) -> Result<ChatHistoryItem, String> {
    let sdir = sessions_dir(workspace).join(id);
    let session_file = sdir.join("index.html");

    // Heal if artifact is missing so activating never gives 404
    if !session_file.is_file() {
        warn!("Session artifact was missing for id '{id}'; auto-healing now");
        let meta_file = session_meta_path(workspace, id);
        let (prompt, design_system, mode) = if let Ok(content) = fs::read_to_string(&meta_file) {
            if let Ok(item) = serde_json::from_str::<ChatHistoryItem>(&content) {
                (item.prompt, item.design_system, item.mode)
            } else {
                (String::new(), "linear".to_string(), "prototype".to_string())
            }
        } else {
            (String::new(), "linear".to_string(), "prototype".to_string())
        };
        let _ = ensure_session_artifact(workspace, id, &prompt, &design_system, &mode);
    }

    let target = workspace.join("index.html");
    fs::copy(&session_file, &target)
        .map_err(|e| format!("Failed to copy session artifact: {e}"))?;
    info!(
        "Activated session '{id}' -> copied {:?} to {:?}",
        session_file, target
    );

    let items = load_history(workspace);
    if let Some(item) = items.into_iter().find(|it| it.id == id) {
        Ok(item)
    } else {
        let meta_file = session_meta_path(workspace, id);
        if let Ok(content) = fs::read_to_string(&meta_file) {
            if let Ok(item) = serde_json::from_str::<ChatHistoryItem>(&content) {
                return Ok(item);
            }
        }
        Ok(ChatHistoryItem {
            id: id.to_string(),
            prompt: String::new(),
            design_system: "linear".to_string(),
            mode: "prototype".to_string(),
            deep: false,
            timestamp: 0,
            has_artifact: true,
        })
    }
}

pub fn delete_history_entry(workspace: &Path, id: &str) -> bool {
    let sdir = sessions_dir(workspace).join(id);
    if sdir.is_dir() {
        let _ = fs::remove_dir_all(&sdir);
        info!("Removed session directory {:?}", sdir);
    }

    let mut items = load_history(workspace);
    let initial_len = items.len();
    items.retain(|item| item.id != id);
    if items.len() < initial_len {
        let _ = save_history(&items);
        true
    } else {
        false
    }
}

pub fn clear_history(workspace: &Path) -> Result<(), String> {
    let sdir = sessions_dir(workspace);
    if sdir.is_dir() {
        let _ = fs::remove_dir_all(&sdir);
        info!("Cleared all session directories in {:?}", sdir);
    }
    save_history(&[])
}

pub fn create_new_project_session(workspace: &Path, id: &str) -> Result<ChatHistoryItem, String> {
    let starter = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ZenDesign — New Project</title>
  <style>
    *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      background: #0d0e12;
      color: #e2e8f0;
      font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      padding: 24px;
      text-align: center;
    }
    .canvas-card {
      max-width: 520px;
      background: #161822;
      border: 1px dashed #2b2d3d;
      border-radius: 16px;
      padding: 40px 32px;
    }
    .icon {
      width: 48px;
      height: 48px;
      margin: 0 auto 16px;
      border-radius: 12px;
      background: rgba(94, 106, 210, 0.15);
      color: #5e6ad2;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 24px;
    }
    h1 { font-size: 20px; font-weight: 600; margin-bottom: 8px; color: #f8fafc; }
    p { font-size: 14px; color: #94a3b8; line-height: 1.6; }
    .hint {
      margin-top: 20px;
      display: inline-block;
      font-size: 12px;
      color: #64748b;
      background: #10121a;
      padding: 6px 12px;
      border-radius: 9999px;
      border: 1px solid #1e2130;
    }
  </style>
</head>
<body>
  <div class="canvas-card">
    <div class="icon">✦</div>
    <h1>New Project Canvas</h1>
    <p>Enter your design brief in the agent panel to compose a prototype, slide deck, or kinetic motion piece.</p>
    <div class="hint">Waiting for your prompt…</div>
  </div>
</body>
</html>"#;

    let sdir = sessions_dir(workspace).join(id);
    fs::create_dir_all(&sdir).map_err(|e| format!("Failed to create project directory: {e}"))?;

    let session_target = sdir.join("index.html");
    fs::write(&session_target, starter).map_err(|e| format!("Failed to write session index.html: {e}"))?;

    let workspace_target = workspace.join("index.html");
    fs::write(&workspace_target, starter).map_err(|e| format!("Failed to create new project: {e}"))?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let item = ChatHistoryItem {
        id: id.to_string(),
        prompt: "New Project Canvas".to_string(),
        design_system: "linear".to_string(),
        mode: "prototype".to_string(),
        deep: false,
        timestamp,
        has_artifact: true,
    };

    if let Ok(meta_json) = serde_json::to_string_pretty(&item) {
        let _ = fs::write(session_meta_path(workspace, id), meta_json);
    }

    let mut items = load_history(workspace);
    items.insert(0, item.clone());
    if items.len() > MAX_HISTORY_ITEMS {
        items.truncate(MAX_HISTORY_ITEMS);
    }
    let _ = save_history(&items);

    info!("Initialized new project canvas session '{id}' at {:?}", sdir);
    Ok(item)
}

pub fn create_new_project(workspace: &Path) -> Result<(), String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let id = format!("{timestamp}_{}", uuid::Uuid::new_v4());
    create_new_project_session(workspace, &id).map(|_| ())
}

pub fn bootstrap_existing_workspace(workspace: &Path) {
    // 1. Heal all existing session folders first
    heal_all_sessions(workspace);

    let index_file = workspace.join("index.html");
    if !index_file.is_file() {
        return;
    }

    let existing_history = load_history(workspace);
    if !existing_history.is_empty() {
        return;
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let initial_id = format!("{timestamp}_{}", uuid::Uuid::new_v4());
    let sdir = sessions_dir(workspace).join(&initial_id);
    if let Err(e) = fs::create_dir_all(&sdir) {
        warn!("Failed to create bootstrap session directory: {e}");
        return;
    }

    let session_index = sdir.join("index.html");
    if let Err(e) = fs::copy(&index_file, &session_index) {
        warn!("Failed to copy initial workspace to session: {e}");
        return;
    }

    let initial_item = ChatHistoryItem {
        id: initial_id.clone(),
        prompt: "Current workspace project".to_string(),
        design_system: "linear".to_string(),
        mode: "prototype".to_string(),
        deep: false,
        timestamp,
        has_artifact: true,
    };

    if let Ok(meta) = serde_json::to_string_pretty(&initial_item) {
        let _ = fs::write(session_meta_path(workspace, &initial_id), meta);
    }

    let _ = save_history(&[initial_item]);
    info!("Bootstrapped initial workspace session '{initial_id}'");
}
