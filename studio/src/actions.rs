use gloo_net::http::Request;
use leptos::prelude::*;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;

use crate::data::{
    mode_meta, system_meta, ActivityEntry, ActivityKind, ChatHistoryItem, DesignSystemId,
    HistoryResponse, ModeId,
};
use crate::i18n::{t, Language};
use crate::settings::{
    ApplyDetectedResponse, DetectedSource, DetectedSourcesResponse, ErrorResponse, ImportResponse,
    ModelsResponse, ProviderId, SettingsView,
};
use crate::stream::push_activity;

/// Dispatches a brief to the Rust daemon. The HTTP call returns immediately;
/// progress and the final preview refresh arrive on the SSE stream.
pub fn generate(
    prompt: RwSignal<String>,
    running: RwSignal<bool>,
    design_system: RwSignal<DesignSystemId>,
    mode: RwSignal<ModeId>,
    deep: RwSignal<bool>,
    entries: RwSignal<Vec<ActivityEntry>>,
    history: Option<RwSignal<Vec<ChatHistoryItem>>>,
) {
    let brief = prompt.get_untracked().trim().to_string();
    if brief.is_empty() || running.get_untracked() {
        return;
    }

    running.set(true);
    push_activity(entries, ActivityKind::User, brief.clone());

    let system = design_system.get_untracked();
    let selected_mode = mode.get_untracked();
    let deep_review = deep.get_untracked();
    let system_label = system_meta(system).label;
    let mode_label = mode_meta(selected_mode).label;

    spawn_local(async move {
        let payload = json!({
            "prompt": brief,
            "design_system": system.as_str(),
            "mode": selected_mode.as_str(),
            "deep": deep_review,
        });

        let request = Request::post("/api/prompt")
            .header("Content-Type", "application/json")
            .body(payload.to_string());

        let request = match request {
            Ok(request) => request,
            Err(error) => {
                running.set(false);
                push_activity(
                    entries,
                    ActivityKind::Error,
                    format!("Could not build the request: {error}"),
                );
                return;
            }
        };

        match request.send().await {
            Ok(response) if response.ok() => {
                if let Some(hist) = history {
                    load_history(hist);
                }
                push_activity(
                    entries,
                    ActivityKind::Queued,
                    format!(
                        "Brief dispatched with {system_label} · {mode_label}{}. Waiting for the agent — the preview will refresh when the run finishes.",
                        if deep_review { " · deep review" } else { "" }
                    ),
                );
            }
            Ok(response) => {
                running.set(false);
                let status = response.status();
                let detail = response.text().await.unwrap_or_default();
                let message = if detail.trim().is_empty() {
                    format!("Daemon returned {status}")
                } else {
                    detail
                };
                push_activity(entries, ActivityKind::Error, message);
            }
            Err(error) => {
                running.set(false);
                push_activity(
                    entries,
                    ActivityKind::Error,
                    format!("Could not reach the ZenDesign daemon: {error}"),
                );
            }
        }
    });
}

/// Cancels the active model/agent run. The daemon aborts the provider future
/// (killing an OpenCode child process) and emits `agent_error`, which clears
/// the running state on the shared event stream.
pub fn stop_agent(entries: RwSignal<Vec<ActivityEntry>>, running: RwSignal<bool>) {
    push_activity(entries, ActivityKind::Status, "Stopping the current run…");
    spawn_local(async move {
        let request = Request::post("/api/agent/stop")
            .header("Content-Type", "application/json")
            .body("{}");

        let request = match request {
            Ok(request) => request,
            Err(error) => {
                running.set(false);
                push_activity(
                    entries,
                    ActivityKind::Error,
                    format!("Could not build the stop request: {error}"),
                );
                return;
            }
        };

        match request.send().await {
            Ok(response) if response.ok() => {
                // The SSE `agent_error` frame normally clears this first; set it
                // here too in case the event stream is reconnecting.
                running.set(false);
            }
            Ok(response) => {
                running.set(false);
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                push_activity(
                    entries,
                    ActivityKind::Error,
                    read_error(&body, &format!("Stop failed: daemon returned {status}")),
                );
            }
            Err(error) => {
                running.set(false);
                push_activity(
                    entries,
                    ActivityKind::Error,
                    format!("Could not reach the daemon to stop the run: {error}"),
                );
            }
        }
    });
}

/// Starts an 8-second MP4 recording of the live preview. The daemon records
/// the artifact in a headless browser; completion arrives as `export_done` on
/// the SSE stream, which also triggers the download.
pub fn export_video(exporting: RwSignal<bool>, entries: RwSignal<Vec<ActivityEntry>>) {
    if exporting.get_untracked() {
        return;
    }
    exporting.set(true);

    spawn_local(async move {
        let request = Request::post("/api/export")
            .header("Content-Type", "application/json")
            .body("{\"seconds\":8}");

        let request = match request {
            Ok(request) => request,
            Err(error) => {
                exporting.set(false);
                push_activity(
                    entries,
                    ActivityKind::Error,
                    format!("Could not build the export request: {error}"),
                );
                return;
            }
        };

        match request.send().await {
            Ok(response) if response.status() == 202 => {
                push_activity(
                    entries,
                    ActivityKind::Queued,
                    "Export started — recording 8 seconds at 1440×900.",
                );
            }
            Ok(response) => {
                exporting.set(false);
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                push_activity(
                    entries,
                    ActivityKind::Error,
                    read_error(
                        &body,
                        &format!("Export failed to start: daemon returned {status}"),
                    ),
                );
            }
            Err(error) => {
                exporting.set(false);
                push_activity(
                    entries,
                    ActivityKind::Error,
                    format!("Could not reach the daemon to export: {error}"),
                );
            }
        }
    });
}

/// Shown when the daemon is unreachable so the dialog is never silently empty.
fn scan_error() -> String {
    "Could not reach the ZenDesign daemon.".to_string()
}

fn read_error(body: &str, fallback: &str) -> String {
    serde_json::from_str::<ErrorResponse>(body)
        .ok()
        .and_then(|value| value.error)
        .filter(|message| !message.trim().is_empty())
        .unwrap_or_else(|| {
            if body.trim().is_empty() {
                fallback.to_string()
            } else {
                body.trim().chars().take(300).collect()
            }
        })
}

/// Loads `/api/settings` into the app-wide signal. A daemon built before the
/// provider routes existed answers 404 here; surface that instead of opening an
/// empty dialog with no explanation.
pub fn load_settings(settings: RwSignal<Option<SettingsView>>, error: RwSignal<Option<String>>) {
    spawn_local(async move {
        let response = match Request::get("/api/settings").send().await {
            Ok(response) => response,
            Err(problem) => {
                settings.set(None);
                error.set(Some(format!(
                    "Could not reach the ZenDesign daemon: {problem}"
                )));
                return;
            }
        };
        let ok = response.ok();
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        match serde_json::from_str::<SettingsView>(&body) {
            Ok(view) => {
                settings.set(Some(view));
                error.set(None);
            }
            Err(_) => {
                settings.set(None);
                error.set(Some(if ok {
                    "The daemon returned an unexpected settings payload.".to_string()
                } else {
                    format!(
                        "The daemon answered {status} for /api/settings. Restart ZenDesign to enable model connections."
                    )
                }));
            }
        }
    });
}

/// Loads model suggestions for the picker. Failures are silent: the model
/// field also accepts free text.
pub fn load_models(provider: &'static str, models: RwSignal<Vec<String>>) {
    spawn_local(async move {
        if provider == "template" {
            models.set(Vec::new());
            return;
        }
        let url = format!("/api/models?provider={provider}");
        let Ok(response) = Request::get(&url).send().await else {
            return;
        };
        let body = response.text().await.unwrap_or_default();
        if let Ok(parsed) = serde_json::from_str::<ModelsResponse>(&body) {
            models.set(parsed.models);
        }
    });
}

pub struct SaveSettings {
    pub provider: ProviderId,
    pub base_url: String,
    pub model: String,
    pub api_key: Option<String>,
    pub lang: Language,
}

pub fn save_settings(
    request: SaveSettings,
    settings: RwSignal<Option<SettingsView>>,
    entries: RwSignal<Vec<ActivityEntry>>,
    saving: RwSignal<bool>,
    open: RwSignal<bool>,
) {
    saving.set(true);
    spawn_local(async move {
        let SaveSettings {
            provider,
            base_url,
            model,
            api_key,
            lang,
        } = request;

        let mut payload = json!({
            "provider": provider.as_str(),
            "base_url": base_url,
            "model": model,
        });
        if let Some(key) = api_key {
            payload["api_key"] = json!(key);
        }

        let request = Request::put("/api/settings").header("Content-Type", "application/json");
        let request = match request.body(payload.to_string()) {
            Ok(request) => request,
            Err(error) => {
                saving.set(false);
                push_activity(
                    entries,
                    ActivityKind::Error,
                    format!("Could not build the settings request: {error}"),
                );
                return;
            }
        };

        match request.send().await {
            Ok(response) if response.ok() => {
                let body = response.text().await.unwrap_or_default();
                if let Ok(view) = serde_json::from_str::<SettingsView>(&body) {
                    settings.set(Some(view));
                }
                saving.set(false);
                open.set(false);
                push_activity(
                    entries,
                    ActivityKind::Status,
                    format!(
                        "{} · {}",
                        t(lang, "Model settings saved", "تنظیمات مدل ذخیره شد"),
                        provider.label(lang)
                    ),
                );
            }
            Ok(response) => {
                saving.set(false);
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                push_activity(
                    entries,
                    ActivityKind::Error,
                    read_error(&body, &format!("Daemon returned {status}")),
                );
            }
            Err(error) => {
                saving.set(false);
                push_activity(
                    entries,
                    ActivityKind::Error,
                    format!("Could not reach the daemon: {error}"),
                );
            }
        }
    });
}

pub fn import_opencode(
    settings: RwSignal<Option<SettingsView>>,
    models: RwSignal<Vec<String>>,
    entries: RwSignal<Vec<ActivityEntry>>,
    warning: RwSignal<Option<String>>,
    importing: RwSignal<bool>,
) {
    importing.set(true);
    spawn_local(async move {
        let request = Request::post("/api/settings/import-opencode")
            .header("Content-Type", "application/json");
        let response = match request.body("{}") {
            Ok(request) => request.send().await,
            Err(error) => {
                importing.set(false);
                push_activity(
                    entries,
                    ActivityKind::Error,
                    format!("Could not build the import request: {error}"),
                );
                return;
            }
        };

        match response {
            Ok(response) if response.ok() => {
                let body = response.text().await.unwrap_or_default();
                match serde_json::from_str::<ImportResponse>(&body) {
                    Ok(imported) => {
                        let name = imported.imported_from.clone();
                        warning.set(imported.warning.clone());
                        if !imported.models.is_empty() {
                            models.set(imported.models);
                        }
                        settings.set(Some(imported.settings));
                        importing.set(false);
                        push_activity(
                            entries,
                            ActivityKind::Status,
                            format!(
                                "Imported {name} from OpenCode · pick a model and Save{}",
                                if imported.warning.is_some() {
                                    " (plain HTTP endpoint)"
                                } else {
                                    ""
                                }
                            ),
                        );
                    }
                    Err(error) => {
                        importing.set(false);
                        push_activity(
                            entries,
                            ActivityKind::Error,
                            format!("Unexpected import response: {error}"),
                        );
                    }
                }
            }
            Ok(response) => {
                importing.set(false);
                let body = response.text().await.unwrap_or_default();
                push_activity(
                    entries,
                    ActivityKind::Error,
                    read_error(&body, "Import failed."),
                );
            }
            Err(error) => {
                importing.set(false);
                push_activity(
                    entries,
                    ActivityKind::Error,
                    format!("{}{error}", scan_error()),
                );
            }
        }
    });
}

pub fn load_detected_sources(sources: RwSignal<Vec<DetectedSource>>) {
    spawn_local(async move {
        if let Ok(response) = Request::get("/api/settings/detected").send().await {
            if response.ok() {
                if let Ok(body) = response.text().await {
                    if let Ok(data) = serde_json::from_str::<DetectedSourcesResponse>(&body) {
                        sources.set(data.sources);
                    }
                }
            }
        }
    });
}

pub fn apply_detected_source(
    id: String,
    applying: RwSignal<Option<String>>,
    settings: RwSignal<Option<SettingsView>>,
    models: RwSignal<Vec<String>>,
    entries: RwSignal<Vec<ActivityEntry>>,
    lang: Language,
) {
    applying.set(Some(id.clone()));
    let target_id = id.clone();
    spawn_local(async move {
        let request = Request::post("/api/settings/apply-detected")
            .header("Content-Type", "application/json")
            .body(json!({ "id": target_id }).to_string());

        let response = match request {
            Ok(req) => req.send().await,
            Err(e) => {
                applying.set(None);
                push_activity(entries, ActivityKind::Error, format!("Failed request: {e}"));
                return;
            }
        };

        match response {
            Ok(resp) if resp.ok() => {
                let body = resp.text().await.unwrap_or_default();
                if let Ok(res) = serde_json::from_str::<ApplyDetectedResponse>(&body) {
                    if !res.models.is_empty() {
                        models.set(res.models);
                    }
                    settings.set(Some(res.settings));
                    applying.set(None);
                    push_activity(
                        entries,
                        ActivityKind::Status,
                        t(
                            lang,
                            "Successfully connected to detected account!",
                            "اتصال به حساب شناسایی‌شده با موفقیت برقرار شد!",
                        ),
                    );
                } else {
                    applying.set(None);
                }
            }
            Ok(resp) => {
                applying.set(None);
                let text = resp.text().await.unwrap_or_default();
                push_activity(entries, ActivityKind::Error, text);
            }
            Err(e) => {
                applying.set(None);
                push_activity(entries, ActivityKind::Error, format!("Error: {e}"));
            }
        }
    });
}

pub fn save_model_choice(
    model: String,
    settings: RwSignal<Option<SettingsView>>,
    entries: RwSignal<Vec<ActivityEntry>>,
    lang: Language,
) {
    let current_provider = settings.get_untracked().map(|s| s.provider).unwrap_or_else(|| "openai".to_string());
    spawn_local(async move {
        let payload = json!({
            "provider": current_provider,
            "model": model,
        });
        if let Ok(req) = Request::put("/api/settings").header("Content-Type", "application/json").body(payload.to_string()) {
            if let Ok(resp) = req.send().await {
                if resp.ok() {
                    if let Ok(body) = resp.text().await {
                        if let Ok(view) = serde_json::from_str::<SettingsView>(&body) {
                            settings.set(Some(view));
                        }
                    }
                    push_activity(
                        entries,
                        ActivityKind::Status,
                        t(lang, "Model updated", "مدل بروزرسانی شد"),
                    );
                }
            }
        }
    });
}

pub fn load_history(history: RwSignal<Vec<ChatHistoryItem>>) {
    spawn_local(async move {
        if let Ok(resp) = Request::get("/api/history").send().await {
            if resp.ok() {
                if let Ok(body) = resp.text().await {
                    if let Ok(res) = serde_json::from_str::<HistoryResponse>(&body) {
                        history.set(res.items);
                    }
                }
            }
        }
    });
}

pub fn delete_history_item(id: String, history: RwSignal<Vec<ChatHistoryItem>>) {
    let target = id.clone();
    history.update(|items| items.retain(|i| i.id != target));
    spawn_local(async move {
        let _ = Request::delete(&format!("/api/history/{id}")).send().await;
    });
}

pub fn clear_all_history(history: RwSignal<Vec<ChatHistoryItem>>) {
    history.set(Vec::new());
    spawn_local(async move {
        let _ = Request::delete("/api/history").send().await;
    });
}


