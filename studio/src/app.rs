use leptos::prelude::*;
use send_wrapper::SendWrapper;

use crate::actions::{load_history, load_settings};
use crate::components::{AgentPanel, Header, HistoryModal, PreviewPanel, SettingsPanel};
use crate::data::{
    ActivityEntry, ActivityKind, ChatHistoryItem, ConnectionState, DesignSystemId, DeviceId, ModeId,
};
use crate::i18n::{load_language, load_theme, store_language, store_theme};
use crate::settings::SettingsView;
use crate::stream::{push_activity, start_event_stream};

#[component]
pub fn App() -> impl IntoView {
    let language = RwSignal::new(load_language());
    let theme = RwSignal::new(load_theme());

    Effect::new(move |_| {
        let lang = language.get();
        store_language(lang);
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(root) = document.document_element() {
                let _ = root.set_attribute("lang", lang.as_str());
                let _ = root.set_attribute("dir", lang.dir());
            }
        }
    });

    Effect::new(move |_| {
        let th = theme.get();
        store_theme(th);
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(root) = document.document_element() {
                let _ = root.set_attribute("data-theme", th.as_str());
            }
        }
    });

    let design_system = RwSignal::new(DesignSystemId::Stripe);
    let mode = RwSignal::new(ModeId::Prototype);
    let device = RwSignal::new(DeviceId::Desktop);

    let prompt = RwSignal::new(String::new());
    let running = RwSignal::new(false);
    let deep = RwSignal::new(false);
    let exporting = RwSignal::new(false);
    let settings = RwSignal::new(None::<SettingsView>);
    let init_settings_open = web_sys::window()
        .and_then(|w| w.location().search().ok())
        .map(|s| s.contains("settings=1"))
        .unwrap_or(false);
    let settings_open = RwSignal::new(init_settings_open);
    let settings_error = RwSignal::new(None::<String>);

    let history = RwSignal::new(Vec::<ChatHistoryItem>::new());
    let init_history_open = web_sys::window()
        .and_then(|w| w.location().search().ok())
        .map(|s| s.contains("history=1"))
        .unwrap_or(false);
    let history_open = RwSignal::new(init_history_open);

    let preview_version = RwSignal::new(1_u64);
    let last_reload_at = RwSignal::new(None::<f64>);

    let connection = RwSignal::new(ConnectionState::Connecting);
    let entries = RwSignal::new(Vec::<ActivityEntry>::new());
    let last_heartbeat = RwSignal::new(None::<f64>);
    let event_count = RwSignal::new(0_u32);

    load_settings(settings, settings_error);
    load_history(history);

    match start_event_stream(
        connection,
        entries,
        last_heartbeat,
        event_count,
        running,
        preview_version,
        last_reload_at,
        exporting,
    ) {
        Ok(guard) => {
            let guard = SendWrapper::new(guard);
            on_cleanup(move || drop(guard));
        }
        Err(error) => {
            connection.set(ConnectionState::Reconnecting);
            push_activity(entries, ActivityKind::Error, error);
        }
    }

    view! {
        <div class="zd-app">
            <div class="zd-shell">
                <Header
                    language=language
                    theme=theme
                    design_system=design_system
                    mode=mode
                    connection=connection
                    last_heartbeat=last_heartbeat
                    settings=settings
                    settings_open=settings_open
                />

                <main class="zd-main">
                    <AgentPanel
                        language=language
                        prompt=prompt
                        running=running
                        design_system=design_system
                        mode=mode
                        deep=deep
                        entries=entries
                        connection=connection
                        settings=settings
                        history=history
                        history_open=history_open
                        preview_version=preview_version
                    />

                    <PreviewPanel
                        language=language
                        device=device
                        preview_version=preview_version
                        last_reload_at=last_reload_at
                        event_count=event_count
                        connection=connection
                        running=running
                        exporting=exporting
                        entries=entries
                    />
                </main>

                <SettingsPanel
                    language=language
                    open=settings_open
                    settings=settings
                    settings_error=settings_error
                    entries=entries
                />

                <HistoryModal
                    language=language
                    open=history_open
                    history=history
                    prompt=prompt
                    design_system=design_system
                    mode=mode
                    deep=deep
                    running=running
                    entries=entries
                    preview_version=preview_version
                />
            </div>
        </div>
    }
}
