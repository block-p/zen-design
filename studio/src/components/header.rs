use icondata_lu::{LuCheck, LuChevronDown, LuGlobe, LuMoon, LuPalette, LuSettings, LuSparkles, LuSun};
use leptos::portal::Portal;
use leptos::prelude::*;
use web_sys::KeyboardEvent;

use crate::data::{
    system_meta, ConnectionState, CustomThemeTokens, DesignSystemId, ModeId, DESIGN_SYSTEMS, MODES,
};
use crate::i18n::{t, Language, Theme};
use crate::icons::Icon;
use crate::settings::SettingsView;

#[component]
#[allow(unused_variables)]
pub fn Header(
    language: RwSignal<Language>,
    theme: RwSignal<Theme>,
    design_system: RwSignal<DesignSystemId>,
    mode: RwSignal<ModeId>,
    connection: RwSignal<ConnectionState>,
    last_heartbeat: RwSignal<Option<f64>>,
    settings: RwSignal<Option<SettingsView>>,
    settings_open: RwSignal<bool>,
    custom_theme: RwSignal<Option<CustomThemeTokens>>,
    theme_builder_open: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <header class="zd-header">
            <div class="zd-header-brand">
                <div class="zd-header-logo-icon">
                    <Icon data=LuSparkles size=15 stroke_width=2.2 />
                </div>
                <div class="zd-header-brand-info">
                    <div class="zd-header-title-row">
                        <span class="zd-header-title">"ZenDesign"</span>
                        <span class="zd-header-badge">
                            {move || t(language.get(), "Studio", "استودیو")}
                        </span>
                    </div>
                    <span class="zd-header-tagline">
                        {move || t(language.get(), "Local-first agent workspace", "محیط کار عامل محلی-اول")}
                    </span>
                </div>
            </div>

            <div class="zd-header-nav">
                <div class="zd-header-nav-main">
                    <div class="zd-header-group">
                        <span class="zd-header-label">
                            {move || t(language.get(), "Design system", "سیستم طراحی")}
                        </span>
                        <DesignSystemPicker
                            language=language
                            value=design_system
                            custom_theme=custom_theme
                            theme_builder_open=theme_builder_open
                        />
                    </div>

                    <div class="zd-header-divider"></div>

                    <div class="zd-header-group">
                        <span class="zd-header-label">
                            {move || t(language.get(), "Mode", "حالت")}
                        </span>
                        <div class="zd-segmented">
                            {MODES
                                .iter()
                                .map(|item| {
                                    let item = *item;
                                    view! {
                                        <button
                                            type="button"
                                            on:click=move |_| mode.set(item.id)
                                            aria-pressed=move || (mode.get() == item.id).to_string()
                                            title=move || format!("{} — {}", item.label(language.get()), item.hint(language.get()))
                                            class=move || {
                                                if mode.get() == item.id {
                                                    "zd-segmented-btn zd-segmented-btn--active"
                                                } else {
                                                    "zd-segmented-btn"
                                                }
                                            }
                                        >
                                            <Icon data=item.icon size=13 stroke_width=1.9 />
                                            <span class="zd-segmented-btn-label">{move || item.label(language.get())}</span>
                                        </button>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>
                </div>

                <div class="zd-header-actions">
                    <button
                        type="button"
                        on:click=move |_| language.update(|l| *l = l.toggle())
                        title=move || language.get().title()
                        class="zd-btn-lang"
                    >
                        <Icon data=LuGlobe size=13 stroke_width=1.9 />
                        <span class="zd-btn-lang-tag">
                            {move || match language.get() {
                                Language::En => "فا",
                                Language::Fa => "EN",
                            }}
                        </span>
                    </button>

                    <button
                        type="button"
                        on:click=move |_| theme.update(|th| *th = th.toggle())
                        title=move || {
                            let lang = language.get();
                            match theme.get() {
                                Theme::Light => t(lang, "Switch to dark mode", "تغییر به حالت تاریک"),
                                Theme::Dark => t(lang, "Switch to light mode", "تغییر به حالت روشن"),
                            }
                        }
                        class="zd-btn-icon"
                    >
                        <Show
                            when=move || theme.get() == Theme::Dark
                            fallback=move || view! {
                                <Icon data=LuMoon size=14 stroke_width=1.9 />
                            }
                        >
                            <Icon data=LuSun size=14 stroke_width=1.9 />
                        </Show>
                    </button>

                    <button
                        type="button"
                        on:click=move |_| settings_open.set(true)
                        title=move || {
                            let lang = language.get();
                            settings
                                .get()
                                .map(|view| format!("{}: {}", t(lang, "Model", "مدل"), view.summary()))
                                .unwrap_or_else(|| t(lang, "Model settings", "تنظیمات مدل").to_string())
                        }
                        class="zd-btn-icon"
                    >
                        <Icon data=LuSettings size=14 stroke_width=1.9 />
                    </button>
                </div>
            </div>
        </header>
    }
}


#[component]
fn DesignSystemPicker(
    language: RwSignal<Language>,
    value: RwSignal<DesignSystemId>,
    custom_theme: RwSignal<Option<CustomThemeTokens>>,
    theme_builder_open: RwSignal<bool>,
) -> impl IntoView {
    let open = RwSignal::new(false);

    let active_dot = move || {
        if value.get() == DesignSystemId::Custom {
            custom_theme
                .get()
                .map(|c| c.dot)
                .unwrap_or_else(|| "#6366f1".to_string())
        } else {
            system_meta(value.get()).dot.to_string()
        }
    };

    let active_label = move || {
        if value.get() == DesignSystemId::Custom {
            custom_theme
                .get()
                .map(|c| c.name)
                .unwrap_or_else(|| t(language.get(), "Custom", "سفارشی").to_string())
        } else {
            system_meta(value.get()).label(language.get()).to_string()
        }
    };

    view! {
        <div
            class="zd-picker"
            on:keydown=move |event: KeyboardEvent| {
                if open.get_untracked() && event.key() == "Escape" {
                    open.set(false);
                }
            }
        >
            <button
                type="button"
                on:click=move |_| open.update(|is_open| *is_open = !*is_open)
                aria-haspopup="listbox"
                aria-expanded=move || open.get().to_string()
                class=move || {
                    if open.get() {
                        "zd-picker-trigger zd-picker-trigger--open"
                    } else {
                        "zd-picker-trigger"
                    }
                }
            >
                <span
                    class="zd-picker-dot"
                    style=move || {
                        let dot = active_dot();
                        format!(
                            "background-color: {}; box-shadow: 0 0 0 2px {}26;",
                            dot, dot,
                        )
                    }
                ></span>
                <span class="zd-picker-label">
                    {active_label}
                </span>
                <span class=move || {
                    if open.get() {
                        "zd-picker-chevron zd-picker-chevron--open"
                    } else {
                        "zd-picker-chevron"
                    }
                }>
                    <Icon data=LuChevronDown size=13 />
                </span>
            </button>

            <Show when=move || open.get()>
                <Portal>
                    <button
                        type="button"
                        tabindex="-1"
                        aria-hidden="true"
                        class="zd-picker-backdrop"
                        on:click=move |_| open.set(false)
                    ></button>
                </Portal>

                <div
                    role="listbox"
                    aria-label="Design system"
                    class="zd-picker-dropdown"
                >
                    <p class="zd-picker-dropdown-header">
                        {move || t(language.get(), "Design system", "سیستم طراحی")}
                    </p>

                    <div class="zd-picker-options-scroll">
                        {DESIGN_SYSTEMS
                            .iter()
                            .map(|system| {
                                let system = *system;
                                view! {
                                    <button
                                        type="button"
                                        role="option"
                                        aria-selected=move || (value.get() == system.id).to_string()
                                        on:click=move |_| {
                                            value.set(system.id);
                                            open.set(false);
                                        }
                                        class=move || {
                                            if value.get() == system.id {
                                                "zd-picker-option zd-picker-option--selected"
                                            } else {
                                                "zd-picker-option"
                                            }
                                        }
                                    >
                                        <span
                                            class="zd-picker-swatch"
                                            style=format!("background-image: {}", system.swatch)
                                        ></span>
                                        <span class="zd-picker-option-meta">
                                            <span class="zd-picker-option-title">
                                                {move || system.label(language.get())}
                                            </span>
                                            <span class="zd-picker-option-desc">
                                                {move || system.tagline(language.get())}
                                            </span>
                                        </span>
                                        <Show when=move || value.get() == system.id>
                                            <Icon data=LuCheck size=14 class="zd-picker-option-check" />
                                        </Show>
                                    </button>
                                }
                            })
                            .collect_view()}

                        <Show when=move || custom_theme.get().is_some()>
                            {move || {
                                let ct = custom_theme.get().unwrap();
                                let swatch = ct.swatch.clone();
                                let name = ct.name.clone();
                                view! {
                                    <button
                                        type="button"
                                        role="option"
                                        aria-selected=move || (value.get() == DesignSystemId::Custom).to_string()
                                        on:click=move |_| {
                                            value.set(DesignSystemId::Custom);
                                            open.set(false);
                                        }
                                        class=move || {
                                            if value.get() == DesignSystemId::Custom {
                                                "zd-picker-option zd-picker-option--selected"
                                            } else {
                                                "zd-picker-option"
                                            }
                                        }
                                    >
                                        <span
                                            class="zd-picker-swatch"
                                            style=format!("background-image: {}", swatch)
                                        ></span>
                                        <span class="zd-picker-option-meta">
                                            <span class="zd-picker-option-title">
                                                {name}
                                            </span>
                                            <span class="zd-picker-option-desc">
                                                {move || t(language.get(), "Custom Session Theme", "تم اختصاصی این سشن")}
                                            </span>
                                        </span>
                                        <Show when=move || value.get() == DesignSystemId::Custom>
                                            <Icon data=LuCheck size=14 class="zd-picker-option-check" />
                                        </Show>
                                    </button>
                                }
                            }}
                        </Show>
                    </div>

                    <div class="zd-picker-footer">
                        <button
                            type="button"
                            class="zd-picker-builder-btn"
                            on:click=move |_| {
                                open.set(false);
                                theme_builder_open.set(true);
                            }
                        >
                            <Icon data=LuPalette size=13 />
                            <span>
                                {move || t(language.get(), "Build Custom Theme (AI)…", "سازنده تم سفارشی (هوش مصنوعی)…")}
                            </span>
                        </button>
                    </div>
                </div>
            </Show>
        </div>
    }
}
