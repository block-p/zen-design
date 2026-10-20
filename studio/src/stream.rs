use std::cell::Cell;
use std::rc::Rc;

use gloo_timers::callback::Interval;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventSource, MessageEvent};

use crate::data::{ActivityEntry, ActivityKind, ConnectionState};

thread_local! {
    static NEXT_ID: Cell<u64> = const { Cell::new(0) };
    /// Activity entry currently receiving streamed model tokens.
    static STREAM_ENTRY: Cell<u64> = const { Cell::new(0) };
    static STREAM_BYTES: Cell<u64> = const { Cell::new(0) };
}

pub fn push_activity(
    entries: RwSignal<Vec<ActivityEntry>>,
    kind: ActivityKind,
    message: impl Into<String>,
) -> u64 {
    let id = NEXT_ID.with(|next| {
        let id = next.get() + 1;
        next.set(id);
        id
    });

    let entry = ActivityEntry {
        id,
        kind,
        message: message.into(),
        at: js_sys::Date::now(),
    };

    entries.update(|list| {
        if list.len() >= 200 {
            list.remove(0);
        }
        list.push(entry);
    });

    id
}

/// Model tokens arrive as many small SSE frames. Rather than dumping raw HTML
/// into the log, keep one entry that shows a live size counter.
fn append_stream(entries: RwSignal<Vec<ActivityEntry>>, chunk: &str) {
    if chunk.is_empty() {
        return;
    }

    let bytes = STREAM_BYTES.with(|counter| {
        let total = counter.get().saturating_add(chunk.len() as u64);
        counter.set(total);
        total
    });
    let label = format!("Writing artifact · {:.1} KB", bytes as f64 / 1024.0);

    let existing = STREAM_ENTRY.with(Cell::get);
    if existing != 0 {
        let mut updated = false;
        entries.update(|list| {
            if let Some(last) = list.last_mut() {
                if last.id == existing {
                    last.kind = ActivityKind::Writing;
                    last.message = label.clone();
                    updated = true;
                }
            }
        });
        if updated {
            return;
        }
    }

    let id = push_activity(entries, ActivityKind::Writing, label);
    STREAM_ENTRY.with(|slot| slot.set(id));
}

fn reset_stream() {
    STREAM_ENTRY.with(|slot| slot.set(0));
    STREAM_BYTES.with(|counter| counter.set(0));
}

fn read_payload(event: &Event) -> (Option<String>, String) {
    let Some(message) = event.dyn_ref::<MessageEvent>() else {
        return (None, "Agent update received.".to_string());
    };
    let Some(raw) = message.data().as_string() else {
        return (None, "Agent update received.".to_string());
    };

    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(value) => {
            let event_type = value
                .get("event_type")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string);
            let message = value
                .get("message")
                .and_then(serde_json::Value::as_str)
                .unwrap_or(raw.as_str())
                .trim()
                .to_string();
            (event_type, message)
        }
        Err(_) => (None, raw),
    }
}

fn classify(event_type: Option<&str>, message: &str) -> ActivityKind {
    match event_type {
        Some("done") => ActivityKind::Done,
        Some("agent_error") | Some("error") => ActivityKind::Error,
        Some("reload") => ActivityKind::Reload,
        Some("thinking") => ActivityKind::Thinking,
        Some("writing") => ActivityKind::Writing,
        None | Some("status") | Some("message") => {
            let lower = message.to_ascii_lowercase();
            if ["apply", "compos", "writ", "render"]
                .iter()
                .any(|needle| lower.contains(needle))
            {
                ActivityKind::Writing
            } else if ["analy", "think", "read", "plan", "reason"]
                .iter()
                .any(|needle| lower.contains(needle))
            {
                ActivityKind::Thinking
            } else {
                ActivityKind::Status
            }
        }
        _ => ActivityKind::Status,
    }
}

/// Keeps the EventSource and every WASM closure alive for the studio lifetime.
pub struct EventStreamGuard {
    source: EventSource,
    _listeners: Vec<Closure<dyn FnMut(Event)>>,
    _open: Closure<dyn FnMut(Event)>,
    _error: Closure<dyn FnMut(Event)>,
    _watchdog: Interval,
}

impl Drop for EventStreamGuard {
    fn drop(&mut self) {
        self.source.close();
    }
}

/// Opens `/api/events` and wires named SSE frames into Leptos signals.
///
/// EventSource reconnects by itself after daemon restarts. A separate watchdog
/// catches the half-open case where the socket stays alive but heartbeats stop.
pub fn start_event_stream(
    connection: RwSignal<ConnectionState>,
    entries: RwSignal<Vec<ActivityEntry>>,
    last_heartbeat: RwSignal<Option<f64>>,
    event_count: RwSignal<u32>,
    running: RwSignal<bool>,
    preview_version: RwSignal<u64>,
    last_reload_at: RwSignal<Option<f64>>,
    exporting: RwSignal<bool>,
) -> Result<EventStreamGuard, String> {
    let source = EventSource::new("/api/events")
        .map_err(|_| "Could not open the daemon event stream.".to_string())?;

    // While an agent writes files (OpenCode in particular rewrites
    // `index.html` on every step), reloading the iframe on each write shows
    // partial documents and looks like the preview is broken. Buffer those
    // reloads and apply one refresh when the run ends.
    let pending_reload = Rc::new(Cell::new(false));

    let opened_once = Rc::new(Cell::new(false));
    let on_open = {
        let opened_once = opened_once.clone();
        let pending = pending_reload.clone();
        Closure::<dyn FnMut(Event)>::new(move |_| {
            last_heartbeat.set(Some(js_sys::Date::now()));
            connection.set(ConnectionState::Live);

            // The first iframe load can fail if the page is opened while the
            // daemon is restarting. A reconnect is the moment to try again;
            // otherwise the preview fallback would latch until a file changes.
            if opened_once.replace(true) {
                if running.get_untracked() {
                    pending.set(true);
                } else {
                    preview_version.update(|version| *version = version.saturating_add(1));
                    last_reload_at.set(Some(js_sys::Date::now()));
                }
                push_activity(entries, ActivityKind::Status, "Daemon reconnected.");
            }
        })
    };
    let open_fn: &js_sys::Function = on_open.as_ref().unchecked_ref();
    source.set_onopen(Some(open_fn));

    let on_error = Closure::<dyn FnMut(Event)>::new(move |_| {
        // Transport errors are normal during daemon restarts. EventSource
        // retries automatically; the pill simply reflects the gap.
        connection.set(ConnectionState::Reconnecting);
    });
    let error_fn: &js_sys::Function = on_error.as_ref().unchecked_ref();
    source.set_onerror(Some(error_fn));

    let heartbeat_cb = Closure::<dyn FnMut(Event)>::new(move |_| {
        last_heartbeat.set(Some(js_sys::Date::now()));
        connection.set(ConnectionState::Live);
    });

    let status_cb = Closure::<dyn FnMut(Event)>::new(move |event| {
        let (event_type, message) = read_payload(&event);
        let message = if message.trim().is_empty() {
            "Agent update received.".to_string()
        } else {
            message
        };
        event_count.update(|count| *count = count.saturating_add(1));
        push_activity(entries, classify(event_type.as_deref(), &message), message);
    });

    let done_cb = {
        let pending = pending_reload.clone();
        Closure::<dyn FnMut(Event)>::new(move |event| {
            let (_, message) = read_payload(&event);
            let message = if message.trim().is_empty() {
                "Artifact rendered.".to_string()
            } else {
                message
            };
            event_count.update(|count| *count = count.saturating_add(1));
            reset_stream();
            push_activity(entries, ActivityKind::Done, message);
            running.set(false);
            if pending.replace(false) {
                preview_version.update(|version| *version = version.saturating_add(1));
                last_reload_at.set(Some(js_sys::Date::now()));
            }
        })
    };

    let token_cb = Closure::<dyn FnMut(Event)>::new(move |event| {
        let (_, message) = read_payload(&event);
        append_stream(entries, &message);
    });

    let agent_error_cb = {
        let pending = pending_reload.clone();
        Closure::<dyn FnMut(Event)>::new(move |event: Event| {
            // Only server-sent `agent_error` frames carry JSON. Native transport
            // errors are handled by `onerror` above.
            if event.dyn_ref::<MessageEvent>().is_none() {
                return;
            }
            let (_, message) = read_payload(&event);
            let message = if message.trim().is_empty() {
                "The agent run failed.".to_string()
            } else {
                message
            };
            event_count.update(|count| *count = count.saturating_add(1));
            reset_stream();
            push_activity(entries, ActivityKind::Error, message);
            running.set(false);
            if pending.replace(false) {
                preview_version.update(|version| *version = version.saturating_add(1));
                last_reload_at.set(Some(js_sys::Date::now()));
            }
        })
    };

    let reload_cb = {
        let pending = pending_reload.clone();
        Closure::<dyn FnMut(Event)>::new(move |event| {
            let (_, message) = read_payload(&event);
            event_count.update(|count| *count = count.saturating_add(1));

            if running.get_untracked() {
                if !pending.replace(true) {
                    push_activity(
                        entries,
                        ActivityKind::Status,
                        "Artifact updated — preview will refresh when the run finishes.",
                    );
                }
                return;
            }

            let message = if message.trim().is_empty() {
                "Workspace files changed — preview reloaded.".to_string()
            } else {
                message
            };
            reset_stream();
            push_activity(entries, ActivityKind::Reload, message);
            let now = js_sys::Date::now();
            preview_version.update(|version| *version = version.saturating_add(1));
            last_reload_at.set(Some(now));
        })
    };

    let export_status_cb = Closure::<dyn FnMut(Event)>::new(move |event| {
        let (_, message) = read_payload(&event);
        if message.trim().is_empty() {
            return;
        }
        event_count.update(|count| *count = count.saturating_add(1));
        push_activity(entries, ActivityKind::Status, message);
    });

    let export_done_cb = Closure::<dyn FnMut(Event)>::new(move |event| {
        let (_, message) = read_payload(&event);
        let message = if message.trim().is_empty() {
            "MP4 ready — download started.".to_string()
        } else {
            format!("{message} — download started.")
        };
        event_count.update(|count| *count = count.saturating_add(1));
        exporting.set(false);
        push_activity(entries, ActivityKind::Done, message);
        download_latest_export();
    });

    let export_error_cb = Closure::<dyn FnMut(Event)>::new(move |event| {
        let (_, message) = read_payload(&event);
        let message = if message.trim().is_empty() {
            "The MP4 export failed.".to_string()
        } else {
            message
        };
        event_count.update(|count| *count = count.saturating_add(1));
        exporting.set(false);
        push_activity(entries, ActivityKind::Error, message);
    });

    for (name, callback) in [
        ("heartbeat", &heartbeat_cb),
        ("status", &status_cb),
        ("writing", &status_cb),
        ("token", &token_cb),
        ("done", &done_cb),
        ("agent_error", &agent_error_cb),
        ("reload", &reload_cb),
        ("message", &status_cb),
        ("export_status", &export_status_cb),
        ("export_done", &export_done_cb),
        ("export_error", &export_error_cb),
    ] {
        let listener: &js_sys::Function = callback.as_ref().unchecked_ref();
        let _ = source.add_event_listener_with_callback(name, listener);
    }

    let watchdog = Interval::new(1_000, move || {
        let Some(last) = last_heartbeat.get_untracked() else {
            return;
        };
        if connection.get_untracked() == ConnectionState::Live
            && js_sys::Date::now() - last > 6_500.0
        {
            connection.set(ConnectionState::Stalled);
        }
    });

    Ok(EventStreamGuard {
        source,
        _listeners: vec![
            heartbeat_cb,
            status_cb,
            token_cb,
            done_cb,
            agent_error_cb,
            reload_cb,
            export_status_cb,
            export_done_cb,
            export_error_cb,
        ],
        _open: on_open,
        _error: on_error,
        _watchdog: watchdog,
    })
}

/// Triggers the browser's download flow for the freshly recorded MP4. Using a
/// detached anchor (rather than `window.open`) is not popup-blocked when the
/// trigger originates from an SSE event instead of a click.
fn download_latest_export() {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let Ok(element) = document.create_element("a") else {
        return;
    };
    let _ = element.set_attribute("href", "/api/export/latest");
    let _ = element.set_attribute("download", "zendesign-preview.mp4");
    if let Ok(anchor) = element.dyn_into::<web_sys::HtmlAnchorElement>() {
        anchor.click();
    }
}
