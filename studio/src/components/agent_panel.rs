use icondata_lu::{
    LuArrowUp, LuCommand, LuCornerDownLeft, LuGripHorizontal, LuHistory, LuPlus, LuSparkles, LuX, LuZap,
};
use leptos::prelude::*;
use web_sys::{KeyboardEvent, MouseEvent, SubmitEvent};

use crate::actions::{create_new_project, generate, stop_agent};
use crate::components::activity_log::ActivityLog;
use crate::data::{
    mode_meta, system_meta, ActivityEntry, ChatHistoryItem, ConnectionState, DesignSystemId,
    ModeId, PRESETS,
};
use crate::i18n::{localize_digits, t, Language};
use crate::icons::Icon;
use crate::settings::SettingsView;

#[component]
pub fn AgentPanel(
    language: RwSignal<Language>,
    prompt: RwSignal<String>,
    running: RwSignal<bool>,
    design_system: RwSignal<DesignSystemId>,
    mode: RwSignal<ModeId>,
    deep: RwSignal<bool>,
    entries: RwSignal<Vec<ActivityEntry>>,
    connection: RwSignal<ConnectionState>,
    settings: RwSignal<Option<SettingsView>>,
    history: RwSignal<Vec<ChatHistoryItem>>,
    history_open: RwSignal<bool>,
    preview_version: RwSignal<u64>,
) -> impl IntoView {
    let can_generate = Memo::new(move |_| !prompt.get().trim().is_empty() && !running.get());
    let activity_height = RwSignal::new(260_i32);
    let is_dragging = RwSignal::new(false);
    let start_y = RwSignal::new(0_i32);
    let start_h = RwSignal::new(260_i32);

    let on_mouse_down = move |ev: MouseEvent| {
        ev.prevent_default();
        is_dragging.set(true);
        start_y.set(ev.client_y());
        start_h.set(activity_height.get());
    };

    let on_mouse_move = move |ev: MouseEvent| {
        if is_dragging.get() {
            let delta = start_y.get() - ev.client_y();
            let new_h = (start_h.get() + delta).clamp(120, 600);
            activity_height.set(new_h);
        }
    };

    let on_mouse_up = move |_: MouseEvent| {
        if is_dragging.get() {
            is_dragging.set(false);
        }
    };

    view! {
        <aside
            class="zd-sidebar"
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
            on:mouseleave=on_mouse_up
        >
            <div class="zd-sidebar-header">
                <div class="zd-agent-top">
                    <div class="zd-agent-profile">
                        <div class="zd-agent-icon">
                            <Icon data=LuSparkles size=15 stroke_width=2.1 />
                            <Show when=move || running.get()>
                                <span class="zd-agent-pulse">
                                    <span class="zd-agent-pulse-ping"></span>
                                    <span class="zd-agent-pulse-dot"></span>
                                </span>
                            </Show>
                        </div>
                        <div class="zd-agent-meta">
                            <h1 class="zd-agent-title">
                                {move || t(language.get(), "Agent", "عامل")}
                            </h1>
                            <p
                                class="zd-agent-subtitle"
                                title=move || {
                                    let lang = language.get();
                                    settings
                                        .get()
                                        .map(|view| view.summary())
                                        .unwrap_or_else(|| t(lang, "Offline template", "قالب آفلاین").to_string())
                                }
                            >
                                {move || {
                                    let lang = language.get();
                                    settings
                                        .get()
                                        .map(|view| view.summary())
                                        .unwrap_or_else(|| t(lang, "Offline template", "قالب آفلاین").to_string())
                                }}
                            </p>
                        </div>
                    </div>
                    <div class="zd-agent-top-right">
                        <button
                            type="button"
                            class="zd-btn-soft zd-agent-new-btn"
                            on:click=move |_| create_new_project(prompt, preview_version, entries, language.get())
                            title=move || t(language.get(), "New Project Canvas", "پروژه جدید (شروع با بوم خالی)")
                        >
                            <Icon data=LuPlus size=12 />
                            <span>{move || t(language.get(), "New", "جدید")}</span>
                        </button>

                        <button
                            type="button"
                            class="zd-history-toggle-btn"
                            on:click=move |_| history_open.set(true)
                            title=move || t(language.get(), "Project History", "تاریخچه پروژه‌ها")
                        >
                            <Icon data=LuHistory size=13 />
                            <Show when=move || !history.get().is_empty()>
                                <span class="zd-history-pill">
                                    {move || {
                                        let count = history.get().len().to_string();
                                        localize_digits(&count, language.get())
                                    }}
                                </span>
                            </Show>
                        </button>
                        <span class=move || {
                            if running.get() {
                                "zd-agent-status-badge zd-agent-status-badge--running"
                            } else {
                                "zd-agent-status-badge zd-agent-status-badge--ready"
                            }
                        }>
                            {move || {
                                let lang = language.get();
                                if running.get() {
                                    t(lang, "Designing", "در حال طراحی")
                                } else {
                                    t(lang, "Ready", "آماده")
                                }
                            }}
                        </span>
                    </div>
                </div>

                <form on:submit=move |event: SubmitEvent| {
                    event.prevent_default();
                    generate(prompt, running, design_system, mode, deep, entries, Some(history));
                }>
                    <div class="zd-composer">
                        <textarea
                            prop:value=move || prompt.get()
                            on:input=move |event| prompt.set(event_target_value(&event))
                            on:keydown=move |event: KeyboardEvent| {
                                if (event.meta_key() || event.ctrl_key()) && event.key() == "Enter" {
                                    event.prevent_default();
                                    generate(prompt, running, design_system, mode, deep, entries, Some(history));
                                }
                            }
                            rows="4"
                            maxlength="2000"
                            placeholder=move || {
                                let lang = language.get();
                                match mode.get() {
                                    ModeId::Motion => t(lang, "Describe the motion piece: story, beats, copy, mood…", "روایت، ریتم، متن و حس قطعه موشن را توصیف کنید…"),
                                    ModeId::Slides => t(lang, "Describe the deck: audience, narrative, key slides…", "مخاطب، روایت و اسلایدهای کلیدی دک را توصیف کنید…"),
                                    _ => t(lang, "Describe the interface you want to create…", "رابط کاربری مد نظر خود را توصیف کنید…"),
                                }
                            }
                            class="zd-composer-textarea"
                        ></textarea>
                        <div class="zd-composer-footer">
                            <span class="zd-composer-hint">
                                <Icon data=LuCommand size=10 />
                                <span class="zd-composer-hint-sep">"+"</span>
                                <Icon data=LuCornerDownLeft size=10 />
                                <span class="zd-composer-hint-text">
                                    {move || t(language.get(), "to generate", "برای ساخت")}
                                </span>
                            </span>
                            <Show
                                when=move || running.get()
                                fallback=move || view! {
                                    <button
                                        type="submit"
                                        disabled=move || !can_generate.get()
                                        class=move || {
                                            if can_generate.get() {
                                                "zd-btn-submit zd-btn-submit--active"
                                            } else {
                                                "zd-btn-submit zd-btn-submit--disabled"
                                            }
                                        }
                                    >
                                        <span>
                                            {move || {
                                                let lang = language.get();
                                                if deep.get() {
                                                    t(lang, "Deep generate", "ساخت عمیق")
                                                } else {
                                                    t(lang, "Generate", "ساخت")
                                                }
                                            }}
                                        </span>
                                        <Icon data=LuArrowUp size=11 stroke_width=2.5 />
                                    </button>
                                }
                            >
                                <button
                                    type="button"
                                    on:click=move |_| stop_agent(entries, running)
                                    title=move || t(language.get(), "Stop the current run", "توقف اجرای جاری")
                                    class="zd-btn-stop"
                                >
                                    <Icon data=LuX size=11 stroke_width=2.5 />
                                    <span>{move || t(language.get(), "Stop", "توقف")}</span>
                                </button>
                            </Show>
                        </div>
                    </div>
                </form>

                <div class="zd-deep-review-card">
                    <div class="zd-deep-review-info">
                        <Icon data=LuSparkles size=12 class="zd-deep-review-sparkle" />
                        <div class="zd-deep-review-copy">
                            <p class="zd-deep-review-title">
                                {move || t(language.get(), "Deep review", "بررسی عمیق")}
                            </p>
                            <p class="zd-deep-review-desc">
                                {move || t(language.get(), "Render, critique, rewrite · 3 passes", "رندر، نقد، بازنویسی · ۳ مرحله")}
                            </p>
                        </div>
                    </div>
                    <button
                        type="button"
                        role="switch"
                        aria-checked=move || if deep.get() { "true" } else { "false" }
                        disabled=move || running.get()
                        on:click=move |_| deep.update(|value| *value = !*value)
                        title=move || t(
                            language.get(),
                            "Render four frames, let the model critique the cut, then rewrite it — up to three times. Requires an OpenAI-compatible or Anthropic API provider. Slower and uses more tokens.",
                            "رندر چهار فریم، نقد خروجی توسط مدل، و سپس بازنویسی — تا سه بار. نیازمند ارائه‌دهنده سازگار با OpenAI یا Anthropic است. کندتر و مصرف توکن بیشتر."
                        )
                        class=move || {
                            if running.get() {
                                "zd-toggle zd-toggle--disabled"
                            } else if deep.get() {
                                "zd-toggle zd-toggle--on"
                            } else {
                                "zd-toggle zd-toggle--off"
                            }
                        }
                    >
                        <span class="zd-toggle-thumb"></span>
                    </button>
                </div>

                <div class="zd-agent-substrip">
                    <span class="zd-agent-substrip-meta">
                        <span
                            class="zd-agent-substrip-dot"
                            style=move || format!("background-color: {}", system_meta(design_system.get()).dot)
                        ></span>
                        <span class="zd-agent-substrip-text">
                            {move || system_meta(design_system.get()).label(language.get())}
                        </span>
                        <span class="zd-agent-substrip-sep">"/"</span>
                        <span class="zd-agent-substrip-text">
                            {move || mode_meta(mode.get()).label(language.get())}
                        </span>
                    </span>
                    <span class="zd-agent-substrip-count">
                        {move || {
                            let count = prompt.with(|v| v.chars().count()).to_string();
                            let text = format!("{count}/2000");
                            localize_digits(&text, language.get())
                        }}
                    </span>
                </div>

                <div class="zd-presets-section">
                    <div class="zd-presets-header">
                        <span class="zd-presets-title">
                            {move || t(language.get(), "Start from a preset", "شروع از یک پیش‌فرض")}
                        </span>
                        <Icon data=LuZap size=11 class="zd-presets-zap" />
                    </div>
                    <div class="zd-presets-grid">
                        {PRESETS
                            .iter()
                            .map(|preset| {
                                let preset = *preset;
                                view! {
                                    <button
                                        type="button"
                                        on:click=move |_| {
                                            prompt.set(preset.prompt.to_string());
                                            design_system.set(preset.design_system);
                                            mode.set(preset.mode);
                                        }
                                        title=preset.prompt
                                        class="zd-preset-card"
                                    >
                                        <div class="zd-preset-title-row">
                                            <Icon
                                                data=preset.icon
                                                size=12
                                                class="zd-preset-icon"
                                            />
                                            <span class="zd-preset-title">
                                                {move || preset.title(language.get())}
                                            </span>
                                        </div>
                                        <span class="zd-preset-desc">
                                            {move || preset.description(language.get())}
                                        </span>
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            </div>

            <div
                class="zd-activity-resizer"
                on:mousedown=on_mouse_down
                title=move || t(language.get(), "Drag to resize activity log", "برای تغییر ارتفاع لاگ فعالیت بکشید")
            >
                <div class="zd-activity-resizer-line"></div>
                <div class="zd-activity-resizer-thumb">
                    <Icon data=LuGripHorizontal size=12 />
                </div>
            </div>

            <div
                class="zd-activity-wrapper"
                style=move || format!("height: {}px; flex: none;", activity_height.get())
            >
                <ActivityLog
                    language=language
                    entries=entries
                    running=running
                    connection=connection
                />
            </div>
        </aside>
    }
}
