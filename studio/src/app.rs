use leptos::prelude::*;
use send_wrapper::SendWrapper;

use crate::actions::{load_history, load_settings};
use crate::components::{
    AgentPanel, Header, HistoryModal, PreviewPanel, SettingsPanel, ThemeBuilderModal,
};
use crate::data::{
    ActivityEntry, ActivityKind, AspectRatio, ChatHistoryItem, ConnectionState, CustomThemeTokens,
    DesignSystemId, DeviceId, ModeId,
};
use crate::i18n::{load_language, load_theme, store_language, store_theme, t};
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
    let custom_theme = RwSignal::new(None::<CustomThemeTokens>);
    let theme_builder_open = RwSignal::new(false);
    let aspect_ratio = RwSignal::new(AspectRatio::R16_9);
    let duration_secs = RwSignal::new(10_u32);
    let sound_effects = RwSignal::new(true);
    let tts_text = RwSignal::new(String::new());
    let tts_lang = RwSignal::new("fa-IR".to_string());

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

    let sidebar_width = RwSignal::new(380_i32);
    let is_resizing_sidebar = RwSignal::new(false);
    let resize_start_x = RwSignal::new(0_i32);
    let resize_start_w = RwSignal::new(380_i32);

    let on_sidebar_resizer_down = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        is_resizing_sidebar.set(true);
        resize_start_x.set(ev.client_x());
        resize_start_w.set(sidebar_width.get());
    };

    let on_app_mouse_move = move |ev: web_sys::MouseEvent| {
        if is_resizing_sidebar.get() {
            let delta = ev.client_x() - resize_start_x.get();
            let new_w = (resize_start_w.get() + delta).clamp(280, 800);
            sidebar_width.set(new_w);
        }
    };

    let on_app_mouse_up = move |_: web_sys::MouseEvent| {
        if is_resizing_sidebar.get() {
            is_resizing_sidebar.set(false);
        }
    };

    view! {
        <div
            class="zd-app"
            on:mousemove=on_app_mouse_move
            on:mouseup=on_app_mouse_up
        >
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
                    custom_theme=custom_theme
                    theme_builder_open=theme_builder_open
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
                        custom_theme=custom_theme
                        theme_builder_open=theme_builder_open
                        aspect_ratio=aspect_ratio
                        duration_secs=duration_secs
                        sound_effects=sound_effects
                        tts_text=tts_text
                        tts_lang=tts_lang
                        sidebar_width=sidebar_width
                    />

                    <div
                        class="zd-sidebar-resizer"
                        class:zd-sidebar-resizer--active=move || is_resizing_sidebar.get()
                        on:mousedown=on_sidebar_resizer_down
                        title=move || t(language.get(), "Drag to resize sidebar width", "برای تغییر عرض پنل بکشید")
                    >
                        <div class="zd-sidebar-resizer-line"></div>
                    </div>

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
                        mode=mode
                        aspect_ratio=aspect_ratio
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

                <ThemeBuilderModal
                    language=language
                    open=theme_builder_open
                    custom_theme=custom_theme
                    design_system=design_system
                    entries=entries
                />
            </div>
        </div>
    }
}
