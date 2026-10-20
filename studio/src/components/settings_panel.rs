use icondata_lu::{
    LuBot, LuCheck, LuCircleAlert, LuKey, LuLoaderCircle, LuPlug, LuRefreshCw, LuSave, LuServer,
    LuSparkles, LuTerminal, LuX,
};
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

use crate::actions::{
    apply_detected_source, import_opencode, load_detected_sources, load_models, save_model_choice,
    save_settings, SaveSettings,
};
use crate::data::ActivityEntry;
use crate::i18n::{t, Language};
use crate::icons::Icon;
use crate::settings::{DetectedSource, ProviderId, SettingsView, PROVIDERS};

#[component]
pub fn SettingsPanel(
    language: RwSignal<Language>,
    open: RwSignal<bool>,
    settings: RwSignal<Option<SettingsView>>,
    settings_error: RwSignal<Option<String>>,
    entries: RwSignal<Vec<ActivityEntry>>,
) -> impl IntoView {
    let provider = RwSignal::new(ProviderId::Template);
    let base_url = RwSignal::new(String::new());
    let model = RwSignal::new(String::new());
    let api_key = RwSignal::new(String::new());
    let clear_key = RwSignal::new(false);
    let models = RwSignal::new(Vec::<String>::new());
    let saving = RwSignal::new(false);
    let importing = RwSignal::new(false);
    let warning = RwSignal::new(None::<String>);
    let detected_sources = RwSignal::new(Vec::<DetectedSource>::new());
    let applying_detected = RwSignal::new(None::<String>);

    // Escape closes the dialog from anywhere.
    if let Some(window) = web_sys::window() {
        let handler = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
            if event.key() == "Escape" && open.get_untracked() {
                open.set(false);
            }
        });
        let _ =
            window.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
        handler.forget();
    }

    let apply_profile = move |id: ProviderId, view: &SettingsView| {
        let profile = view.profile(id.as_str());
        base_url.set(profile.base_url.clone());
        model.set(profile.model.clone());
    };

    // Seed the form every time the dialog opens.
    Effect::new(move |_| {
        if !open.get() {
            return;
        }
        load_detected_sources(detected_sources);
        if let Some(view) = settings.get() {
            let id = ProviderId::from_str(&view.provider);
            provider.set(id);
            apply_profile(id, &view);
            api_key.set(String::new());
            clear_key.set(false);
            load_models(id.as_str(), models);
        }
    });

    // Follow settings changes while the dialog is open.
    Effect::new(move |_| {
        let Some(view) = settings.get() else {
            return;
        };
        if !open.get_untracked() {
            return;
        }
        let id = ProviderId::from_str(&view.provider);
        provider.set(id);
        apply_profile(id, &view);
    });

    let choose_provider = move |id: ProviderId| {
        provider.set(id);
        api_key.set(String::new());
        clear_key.set(false);
        warning.set(None);
        if let Some(view) = settings.get_untracked() {
            apply_profile(id, &view);
        }
        load_models(id.as_str(), models);
    };

    let start_import = move |_| {
        warning.set(None);
        import_opencode(settings, models, entries, warning, importing);
    };

    let start_save = move |_| {
        let key = if clear_key.get_untracked() {
            Some(String::new())
        } else {
            let current = api_key.get_untracked();
            if current.trim().is_empty() {
                None
            } else {
                Some(current)
            }
        };

        save_settings(
            SaveSettings {
                provider: provider.get_untracked(),
                base_url: base_url.get_untracked(),
                model: model.get_untracked(),
                api_key: key,
                lang: language.get_untracked(),
            },
            settings,
            entries,
            saving,
            open,
        );
    };

    let active_tab = RwSignal::new("accounts"); // "accounts" or "custom"

    view! {
        <Show when=move || open.get()>
            <Portal>
                <div
                    class="zd-modal-backdrop"
                    on:click=move |event| {
                        if event.target() == event.current_target() {
                            open.set(false);
                        }
                    }
                >
                    <div
                        role="dialog"
                        aria-modal="true"
                        class="zd-modal-window zd-modal-window--redesign"
                    >
                        <header class="zd-modal-header">
                            <div class="zd-modal-header-info">
                                <h2 class="zd-modal-title">
                                    {move || t(language.get(), "Settings & AI Provider", "تنظیمات و سرویس‌دهنده هوش مصنوعی")}
                                </h2>
                                <p class="zd-modal-subtitle">
                                    {move || t(language.get(), "Connect your local agent, API key, or custom endpoint", "اتصال به عامل‌های محلی سیستم، کلید API یا اندپوینت دلخواه")}
                                </p>
                            </div>
                            <button
                                type="button"
                                on:click=move |_| open.set(false)
                                title=move || t(language.get(), "Close", "بستن")
                                class="zd-btn-icon"
                            >
                                <Icon data=LuX size=14 />
                            </button>
                        </header>

                        // Tab switcher
                        <div class="zd-settings-nav-tabs">
                            <button
                                type="button"
                                class=move || if active_tab.get() == "accounts" { "zd-settings-tab zd-settings-tab--active" } else { "zd-settings-tab" }
                                on:click=move |_| active_tab.set("accounts")
                            >
                                <Icon data=LuPlug size=13 />
                                <span>{move || t(language.get(), "Connected Accounts", "حساب‌های متصل")}</span>
                            </button>
                            <button
                                type="button"
                                class=move || if active_tab.get() == "custom" { "zd-settings-tab zd-settings-tab--active" } else { "zd-settings-tab" }
                                on:click=move |_| active_tab.set("custom")
                            >
                                <Icon data=LuKey size=13 />
                                <span>{move || t(language.get(), "Manual API & Advanced", "تنظیم دستی API و پیشرفته")}</span>
                            </button>
                        </div>

                        <div class="zd-modal-body">
                            <Show when=move || settings_error.get().is_some()>
                                <div class="zd-modal-alert zd-modal-alert--error">
                                    <Icon data=LuCircleAlert size=14 class="zd-modal-alert-icon" />
                                    <p class="zd-modal-alert-text">
                                        {move || settings_error.get().unwrap_or_default()}
                                    </p>
                                </div>
                            </Show>

                            // TAB 1: ACCOUNTS
                            <Show when=move || active_tab.get() == "accounts">
                                <div class="zd-detected-section-clean">
                                    <div class="zd-detected-grid">
                                        {move || {
                                            let lang = language.get();
                                            detected_sources
                                                .get()
                                                .into_iter()
                                                .map(|source| {
                                                    let click_id = source.id.clone();
                                                    let check_id = source.id.clone();
                                                    let icon = match source.id.as_str() {
                                                        "antigravity" => LuSparkles,
                                                        "codex" => LuTerminal,
                                                        "claude_code" => LuBot,
                                                        "opencode" => LuServer,
                                                        _ => LuKey,
                                                    };
                                                    let s_id = source.id.clone();
                                                    let s_base = source.base_url.clone();
                                                    let is_active = Memo::new(move |_| {
                                                        if let Some(view) = settings.get() {
                                                            if let Some(active) = view.active_source.as_deref() {
                                                                active == s_id
                                                            } else if s_id == "antigravity" {
                                                                 view.provider == "openai" && (view.openai.model.starts_with("ag/") || view.openai.base_url == s_base)
                                                            } else if s_id == "codex" {
                                                                 view.provider == "openai" && (view.openai.model.starts_with("gpt-") || view.openai.base_url.contains("codex"))
                                                            } else if s_id == "claude_code" {
                                                                 view.provider == "anthropic" || (view.provider == "openai" && view.openai.base_url.contains("claude"))
                                                            } else if s_id == "opencode" {
                                                                 view.provider == "opencode" || (view.provider == "openai" && (view.openai.model.starts_with("opencode/") || view.openai.base_url.contains("4096") || view.openai.base_url.contains("opencode")))
                                                            } else {
                                                                 false
                                                            }
                                                        } else {
                                                            false
                                                        }
                                                    });
                                                    let card_models = source.models.clone();

                                                    view! {
                                                        <div
                                                            class=move || {
                                                                if is_active.get() {
                                                                    "zd-detected-card zd-detected-card--active"
                                                                } else {
                                                                    "zd-detected-card"
                                                                }
                                                            }
                                                        >
                                                            <div class="zd-detected-card-top">
                                                                <div class="zd-detected-icon-box">
                                                                    <Icon data=icon size=15 />
                                                                </div>
                                                                <div class="zd-detected-info">
                                                                    <div class="zd-detected-name-row">
                                                                        <span class="zd-detected-name">{source.name}</span>
                                                                        <Show when=move || is_active.get()>
                                                                            <span class="zd-badge-active-dot">
                                                                                <Icon data=LuCheck size=10 />
                                                                                <span>{t(lang, "Active", "فعال")}</span>
                                                                            </span>
                                                                        </Show>
                                                                    </div>
                                                                    <span class="zd-detected-desc">{source.hint}</span>
                                                                </div>
                                                            </div>
                                                            <div class="zd-detected-card-action">
                                                                <Show
                                                                    when=move || is_active.get()
                                                                    fallback=move || {
                                                                        let cid = click_id.clone();
                                                                        let chk = check_id.clone();
                                                                        view! {
                                                                            <button
                                                                                type="button"
                                                                                disabled=move || applying_detected.get().is_some()
                                                                                on:click=move |_| {
                                                                                    apply_detected_source(
                                                                                        cid.clone(),
                                                                                        applying_detected,
                                                                                        settings,
                                                                                        models,
                                                                                        entries,
                                                                                        lang,
                                                                                    );
                                                                                }
                                                                                class="zd-detected-connect-btn"
                                                                            >
                                                                                <Show
                                                                                    when=move || applying_detected.get().as_deref() == Some(&chk)
                                                                                    fallback=move || view! { <span>{t(lang, "Connect", "اتصال")}</span> }
                                                                                >
                                                                                    <Icon data=LuLoaderCircle size=11 class="zd-spin" />
                                                                                    <span>{t(lang, "Connecting…", "در حال اتصال…")}</span>
                                                                                </Show>
                                                                            </button>
                                                                        }
                                                                    }
                                                                >
                                                                    <div class="zd-detected-active-box">
                                                                        <div class="zd-card-model-label">
                                                                            {t(lang, "Active Model:", "مدل فعال:")}
                                                                        </div>
                                                                        {
                                                                            let cur_model = model.get();
                                                                            let c_models = card_models.clone();
                                                                            (!c_models.is_empty()).then(|| view! {
                                                                                <select
                                                                                    class="zd-select--compact"
                                                                                    dir="ltr"
                                                                                    prop:value=move || model.get()
                                                                                    on:change=move |event| {
                                                                                        let val = event_target_value(&event);
                                                                                        model.set(val.clone());
                                                                                        save_model_choice(val, settings, entries, lang);
                                                                                    }
                                                                                >
                                                                                    {c_models.iter().map(|m| {
                                                                                        let is_sel = *m == cur_model;
                                                                                        view! {
                                                                                            <option value=m.clone() selected=is_sel>
                                                                                                {m.clone()}
                                                                                            </option>
                                                                                        }
                                                                                    }).collect_view()}
                                                                                </select>
                                                                            })
                                                                        }
                                                                    </div>
                                                                </Show>
                                                            </div>
                                                        </div>
                                                    }
                                                })
                                                .collect_view()
                                        }}
                                    </div>
                                </div>
                            </Show>

                            // TAB 2: MANUAL & ADVANCED
                            <Show when=move || active_tab.get() == "custom">
                                <div class="zd-providers-grid">
                                    {PROVIDERS
                                        .iter()
                                        .map(|id| {
                                            let id = *id;
                                            let icon = match id {
                                                ProviderId::OpenAi => LuServer,
                                                ProviderId::Anthropic => LuSparkles,
                                                ProviderId::OpenCode => LuBot,
                                                ProviderId::Template => LuCheck,
                                            };
                                            view! {
                                                <button
                                                    type="button"
                                                    on:click=move |_| choose_provider(id)
                                                    class=move || {
                                                        if provider.get() == id {
                                                            "zd-provider-card zd-provider-card--active"
                                                        } else {
                                                            "zd-provider-card"
                                                        }
                                                    }
                                                >
                                                    <div class="zd-provider-title-row">
                                                        <Icon data=icon size=13 class="zd-provider-icon" />
                                                        <span class="zd-provider-label">
                                                            {id.label(language.get())}
                                                        </span>
                                                    </div>
                                                    <span class="zd-provider-hint">
                                                        {id.hint(language.get())}
                                                    </span>
                                                </button>
                                            }
                                        })
                                        .collect_view()}
                                </div>

                                <Show when=move || provider.get() == ProviderId::OpenAi>
                                    <div class="zd-form-section">
                                        <div class="zd-form-row-between">
                                            <span class="zd-form-label">
                                                {move || t(language.get(), "Import from OpenCode", "دریافت از OpenCode")}
                                            </span>
                                            <button
                                                type="button"
                                                disabled=move || importing.get()
                                                on:click=start_import
                                                class="zd-btn-soft"
                                            >
                                                <Show
                                                    when=move || importing.get()
                                                    fallback=move || view! {
                                                        <Icon data=LuRefreshCw size=11 />
                                                        <span>{move || t(language.get(), "Import 9Router", "دریافت 9Router")}</span>
                                                    }
                                                >
                                                    <Icon data=LuLoaderCircle size=11 class="zd-spin" />
                                                    <span>{move || t(language.get(), "Importing…", "در حال دریافت…")}</span>
                                                </Show>
                                            </button>
                                        </div>
                                        <label class="zd-field">
                                            <span class="zd-form-label">
                                                {move || t(language.get(), "Base URL", "آدرس پایه (Base URL)")}
                                            </span>
                                            <input
                                                type="text"
                                                class="zd-input"
                                                placeholder="http://host:port/v1"
                                                autocomplete="off"
                                                spellcheck="false"
                                                dir="ltr"
                                                prop:value=move || base_url.get()
                                                on:input=move |event| base_url.set(event_target_value(&event))
                                            />
                                        </label>
                                    </div>
                                </Show>

                                <Show when=move || provider.get() != ProviderId::Template>
                                    <div class="zd-field">
                                        <div class="zd-form-row-between">
                                            <span class="zd-form-label">
                                                {move || t(language.get(), "Model", "مدل")}
                                            </span>
                                            <Show when=move || !models.get().is_empty()>
                                                <span class="zd-form-hint" style="margin: 0;">
                                                    {move || format!("{} {}", models.get().len(), t(language.get(), "models available", "مدل موجود"))}
                                                </span>
                                            </Show>
                                        </div>

                                        <Show
                                            when=move || !models.get().is_empty()
                                            fallback=move || view! {
                                                <input
                                                    type="text"
                                                    class="zd-input zd-input--mono"
                                                    placeholder=move || match provider.get() {
                                                        ProviderId::Anthropic => "claude-opus-4-6".to_string(),
                                                        ProviderId::OpenCode => t(language.get(), "leave blank for the OpenCode default", "برای پیش‌فرض OpenCode خالی بگذارید").to_string(),
                                                        _ => "ag/gemini-3.8-flash-high".to_string(),
                                                    }
                                                    autocomplete="off"
                                                    spellcheck="false"
                                                    dir="ltr"
                                                    prop:value=move || model.get()
                                                    on:input=move |event| model.set(event_target_value(&event))
                                                />
                                            }
                                        >
                                            <select
                                                class="zd-select"
                                                dir="ltr"
                                                prop:value=move || model.get()
                                                on:change=move |event| {
                                                    let val = event_target_value(&event);
                                                    model.set(val);
                                                }
                                            >
                                                {move || {
                                                    let list = models.get();
                                                    let cur = model.get();
                                                    list.into_iter().map(|m| {
                                                        let is_selected = m == cur;
                                                        let m_val = m.clone();
                                                        view! {
                                                            <option value=m_val selected=is_selected>
                                                                {m}
                                                            </option>
                                                        }
                                                    }).collect_view()
                                                }}
                                            </select>
                                        </Show>
                                    </div>
                                </Show>

                                <Show when=move || {
                                    provider.get() == ProviderId::OpenAi
                                        || provider.get() == ProviderId::Anthropic
                                }>
                                    <label class="zd-field">
                                        <span class="zd-form-label">
                                            {move || t(language.get(), "API key", "کلید API")}
                                        </span>
                                        <div class="zd-input-group">
                                            <input
                                                type="password"
                                                class="zd-input zd-input--flex"
                                                placeholder=move || {
                                                    let lang = language.get();
                                                    let view = settings.get();
                                                    let set = view
                                                        .as_ref()
                                                        .map(|view| {
                                                            view.profile(provider.get().as_str()).api_key_set
                                                        })
                                                        .unwrap_or(false);
                                                    if set && !clear_key.get() {
                                                        t(lang, "•••••••• (leave blank to keep)", "•••••••• (برای حفظ خالی بگذارید)").to_string()
                                                    } else {
                                                        "sk-…".to_string()
                                                    }
                                                }
                                                autocomplete="off"
                                                dir="ltr"
                                                prop:value=move || api_key.get()
                                                on:input=move |event| api_key.set(event_target_value(&event))
                                            />
                                            <Show when=move || {
                                                settings
                                                    .get()
                                                    .as_ref()
                                                    .map(|view| {
                                                        view.profile(provider.get().as_str()).api_key_set
                                                    })
                                                    .unwrap_or(false)
                                            }>
                                                <button
                                                    type="button"
                                                    on:click=move |_| clear_key.update(|value| *value = !*value)
                                                    class=move || {
                                                        if clear_key.get() {
                                                            "zd-btn-clear-key zd-btn-clear-key--active"
                                                        } else {
                                                            "zd-btn-clear-key"
                                                        }
                                                    }
                                                >
                                                    {move || {
                                                        let lang = language.get();
                                                        if clear_key.get() {
                                                            t(lang, "Key will be cleared", "کلید پاک خواهد شد")
                                                        } else {
                                                            t(lang, "Clear key", "پاکسازی کلید")
                                                        }
                                                    }}
                                                </button>
                                            </Show>
                                        </div>
                                    </label>
                                </Show>

                                <Show when=move || provider.get() == ProviderId::Template>
                                    <p class="zd-info-box">
                                        {move || t(
                                            language.get(),
                                            "The deterministic offline generator writes the artifact locally with zero network calls. Pick a provider above when you want a real model in the loop.",
                                            "تولیدکننده آفلاین قطعی، آرتیفکت را به صورت محلی و بدون نیاز به شبکه می‌سازد. هر زمان که مدلی واقعی نیاز دارید، یکی از ارائه‌دهندگان بالا را انتخاب کنید."
                                        )}
                                    </p>
                                </Show>

                                <div class="zd-modal-actions-inline">
                                    <button
                                        type="button"
                                        disabled=move || saving.get()
                                        on:click=start_save
                                        class="zd-btn-primary"
                                    >
                                        <Show
                                            when=move || saving.get()
                                            fallback=move || view! {
                                                <Icon data=LuSave size=12 />
                                                <span>{move || t(language.get(), "Save Manual Configuration", "ذخیره تنظیمات دستی")}</span>
                                            }
                                        >
                                            <Icon data=LuLoaderCircle size=12 class="zd-spin" />
                                            <span>{move || t(language.get(), "Saving…", "در حال ذخیره…")}</span>
                                        </Show>
                                    </button>
                                </div>
                            </Show>

                            <Show when=move || warning.get().is_some()>
                                <div class="zd-modal-alert zd-modal-alert--warning">
                                    <Icon data=LuCircleAlert size=14 class="zd-modal-alert-icon" />
                                    <p class="zd-modal-alert-text">
                                        {move || warning.get().unwrap_or_default()}
                                    </p>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </Portal>
        </Show>
    }
}

