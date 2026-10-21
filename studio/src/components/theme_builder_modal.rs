use gloo_net::http::Request;
use icondata_lu::{LuCheck, LuPalette, LuSlidersHorizontal, LuSparkles, LuWand, LuX, LuZap};
use leptos::portal::Portal;
use leptos::prelude::*;
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::KeyboardEvent;

use crate::data::{ActivityEntry, ActivityKind, CustomThemeTokens, DesignSystemId};
use crate::i18n::{t, Language};
use crate::icons::Icon;
use crate::stream::push_activity;

#[derive(Deserialize)]
struct ThemeGenerateResponse {
    theme: CustomThemeTokens,
}

fn normalize_to_hex(color: &str) -> String {
    let c = color.trim();
    if c.starts_with('#') {
        if c.len() == 7 {
            return c.to_string();
        } else if c.len() == 4 {
            let r = &c[1..2];
            let g = &c[2..3];
            let b = &c[3..4];
            return format!("#{r}{r}{g}{g}{b}{b}");
        }
    }
    if c.starts_with("hsl") {
        if let Some(start) = c.find('(') {
            if let Some(end) = c.rfind(')') {
                let inner = &c[start + 1..end];
                let parts: Vec<&str> = inner.split(',').collect();
                if parts.len() >= 3 {
                    let h: f64 = parts[0].trim().parse().unwrap_or(0.0);
                    let s: f64 = parts[1].trim().trim_end_matches('%').parse::<f64>().unwrap_or(0.0) / 100.0;
                    let l: f64 = parts[2].trim().trim_end_matches('%').parse::<f64>().unwrap_or(0.0) / 100.0;
                    
                    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
                    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
                    let m = l - c / 2.0;

                    let (r1, g1, b1) = if h < 60.0 {
                        (c, x, 0.0)
                    } else if h < 120.0 {
                        (x, c, 0.0)
                    } else if h < 180.0 {
                        (0.0, c, x)
                    } else if h < 240.0 {
                        (0.0, x, c)
                    } else if h < 300.0 {
                        (x, 0.0, c)
                    } else {
                        (c, 0.0, x)
                    };

                    let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
                    let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
                    let b = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;

                    return format!("#{r:02x}{g:02x}{b:02x}");
                }
            }
        }
    }
    if c.is_empty() {
        "#38bdf8".to_string()
    } else {
        c.to_string()
    }
}

#[component]
pub fn ThemeBuilderModal(
    language: RwSignal<Language>,
    open: RwSignal<bool>,
    custom_theme: RwSignal<Option<CustomThemeTokens>>,
    design_system: RwSignal<DesignSystemId>,
    entries: RwSignal<Vec<ActivityEntry>>,
    preview_version: RwSignal<u64>,
    settings: RwSignal<Option<crate::settings::SettingsView>>,
    settings_open: RwSignal<bool>,
) -> impl IntoView {
    let prompt_input = RwSignal::new(String::new());
    let generating = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    // Working copy of theme tokens
    let theme_name = RwSignal::new("My Custom Theme".to_string());
    let bg_color = RwSignal::new("#0d1117".to_string());
    let text_color = RwSignal::new("#f0f6fc".to_string());
    let accent_color = RwSignal::new("#38bdf8".to_string());
    let card_bg = RwSignal::new("#161b22".to_string());
    let border_color = RwSignal::new("#30363d".to_string());
    let initial_font = if language.get_untracked() == Language::Fa {
        "'Vazirmatn', system-ui, sans-serif".to_string()
    } else {
        "'Plus Jakarta Sans', system-ui, sans-serif".to_string()
    };
    let font_family = RwSignal::new(initial_font);

    // When modal opens or custom_theme changes, populate fields
    Effect::new(move |_| {
        if open.get() {
            if let Some(existing) = custom_theme.get() {
                theme_name.set(existing.name);
                bg_color.set(existing.bg);
                text_color.set(existing.text);
                accent_color.set(existing.accent);
                card_bg.set(existing.card_bg);
                border_color.set(existing.border_color);
                font_family.set(existing.font_family);
            } else if language.get() == Language::Fa && font_family.get_untracked().contains("Jakarta") {
                font_family.set("'Vazirmatn', system-ui, sans-serif".to_string());
            }
        }
    });

    let on_generate = move |_| {
        let prompt = prompt_input.get().trim().to_string();
        if prompt.is_empty() || generating.get() {
            return;
        }

        generating.set(true);
        error_msg.set(None);

        spawn_local(async move {
            let body = json!({ "prompt": prompt }).to_string();
            let req = Request::post("/api/themes/generate")
                .header("Content-Type", "application/json")
                .body(body);

            match req {
                Ok(request) => match request.send().await {
                    Ok(resp) if resp.ok() => {
                        if let Ok(data) = resp.json::<ThemeGenerateResponse>().await {
                            theme_name.set(data.theme.name);
                            bg_color.set(normalize_to_hex(&data.theme.bg));
                            text_color.set(normalize_to_hex(&data.theme.text));
                            accent_color.set(normalize_to_hex(&data.theme.accent));
                            card_bg.set(normalize_to_hex(&data.theme.card_bg));
                            border_color.set(normalize_to_hex(&data.theme.border_color));
                            font_family.set(data.theme.font_family);
                        } else {
                            error_msg.set(Some("Failed to parse generated theme".to_string()));
                        }
                    }
                    _ => {
                        error_msg.set(Some("Failed to generate theme from server".to_string()));
                    }
                },
                Err(e) => {
                    error_msg.set(Some(format!("Request error: {e}")));
                }
            }
            generating.set(false);
        });
    };

    let on_apply = move |_| {
        let name = theme_name.get();
        let bg = bg_color.get();
        let text = text_color.get();
        let accent = accent_color.get();
        let card = card_bg.get();
        let border = border_color.get();
        let font = font_family.get();

        let dot = accent.clone();
        let swatch = format!("linear-gradient(135deg, {accent} 0%, {bg} 100%)");

        let tokens = CustomThemeTokens {
            name: name.clone(),
            bg: bg.clone(),
            text: text.clone(),
            accent: accent.clone(),
            font_family: font.clone(),
            card_bg: card.clone(),
            border_color: border.clone(),
            dot,
            swatch,
        };

        custom_theme.set(Some(tokens));
        design_system.set(DesignSystemId::Custom);
        preview_version.update(|v| *v += 1);
        open.set(false);

        // Directly apply custom theme variables to iframe if loaded
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Ok(Some(iframe_el)) = document.query_selector(".zd-preview-frame") {
                    if let Ok(iframe) = iframe_el.dyn_into::<web_sys::HtmlIFrameElement>() {
                        if let Some(content_doc) = iframe.content_document() {
                            if let Some(root) = content_doc.document_element() {
                                if let Ok(style) = root.dyn_into::<web_sys::HtmlElement>() {
                                    let s = style.style();
                                    let _ = s.set_property("--zd-bg", &bg);
                                    let _ = s.set_property("--zd-text", &text);
                                    let _ = s.set_property("--zd-accent", &accent);
                                    let _ = s.set_property("--zd-card-bg", &card);
                                    let _ = s.set_property("--zd-border", &border);
                                    let _ = s.set_property("--zd-font", &font);
                                    let _ = s.set_property("--bg", &bg);
                                    let _ = s.set_property("--text", &text);
                                    let _ = s.set_property("--accent", &accent);
                                    let _ = s.set_property("--card", &card);
                                    let _ = s.set_property("--border", &border);
                                }
                            }
                        }
                    }
                }
            }
        }

        push_activity(
            entries,
            ActivityKind::Status,
            format!("Custom theme '{name}' applied to session."),
        );
    };

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
                        class="zd-modal-window zd-theme-builder-dialog"
                        role="dialog"
                        aria-modal="true"
                        on:keydown=move |ev: KeyboardEvent| {
                            if ev.key() == "Escape" {
                                open.set(false);
                            }
                        }
                    >
                    <div class="zd-modal-header">
                        <div class="zd-modal-title-row">
                            <Icon data=LuPalette size=18 class="zd-modal-title-icon" />
                            <h2 class="zd-modal-title">
                                {move || t(language.get(), "Theme Builder", "سازنده تم سفارشی")}
                            </h2>
                        </div>
                        <button
                            type="button"
                            class="zd-btn-icon"
                            on:click=move |_| open.set(false)
                            title=move || t(language.get(), "Close", "بستن")
                        >
                            <Icon data=LuX size=16 />
                        </button>
                    </div>

                    <div class="zd-modal-body zd-theme-builder-body">
                        <!-- AI Prompt Generation Box -->
                        <div class="zd-theme-builder-prompt-card">
                            <div class="zd-theme-builder-prompt-header">
                                <label class="zd-form-label">
                                    <Icon data=LuWand size=13 />
                                    <span>{move || t(language.get(), "Describe Your Dream Theme", "توصیف تم مورد نظر شما")}</span>
                                </label>
                                <Show when=move || {
                                    settings.get().map(|s| s.provider == "template" || s.provider.is_empty()).unwrap_or(true)
                                }>
                                    <div class="zd-theme-ai-pill-row">
                                        <span class="zd-theme-ai-pill">
                                            <Icon data=LuZap size=11 />
                                            <span>{move || t(language.get(), "Offline Generator Active", "الگوریتم آفلاین فعال است")}</span>
                                        </span>
                                        <button
                                            type="button"
                                            class="zd-btn-pill-connect"
                                            on:click=move |_| settings_open.set(true)
                                            title=move || t(language.get(), "Connect real AI model in Settings", "اتصال مدل هوش مصنوعی در تنظیمات")
                                        >
                                            <Icon data=LuSlidersHorizontal size=11 />
                                            <span>{move || t(language.get(), "Connect AI for best creative results", "اتصال هوش مصنوعی برای نتیجه دقیق‌تر")}</span>
                                        </button>
                                    </div>
                                </Show>
                            </div>
                            <div class="zd-theme-builder-input-row">
                                <input
                                    type="text"
                                    class="zd-input zd-theme-builder-prompt-input"
                                    placeholder=move || t(
                                        language.get(),
                                        "e.g. Autumn forest with burnt orange, warm stone & pine slate…",
                                        "مثلاً: جنگل پاییزی با نارنجی سوخته، سنگ گرم و اسلیت کاج…"
                                    )
                                    prop:value=move || prompt_input.get()
                                    on:input=move |ev| prompt_input.set(event_target_value(&ev))
                                    on:keydown=move |ev: web_sys::KeyboardEvent| {
                                        if ev.key() == "Enter" {
                                            ev.prevent_default();
                                            on_generate(());
                                        }
                                    }
                                />
                                <button
                                    type="button"
                                    class="zd-btn-primary zd-theme-builder-gen-btn"
                                    disabled=move || generating.get() || prompt_input.get().trim().is_empty()
                                    on:click=move |_| on_generate(())
                                >
                                    <Show
                                        when=move || generating.get()
                                        fallback=move || view! {
                                            <Icon data=LuSparkles size=13 />
                                            <span>{move || t(language.get(), "Generate", "ساخت تم")}</span>
                                        }
                                    >
                                        <span>{move || t(language.get(), "Generating…", "در حال ساخت…")}</span>
                                    </Show>
                                </button>
                            </div>
                            <Show when=move || error_msg.get().is_some()>
                                <p class="zd-theme-builder-error">
                                    {move || error_msg.get().unwrap_or_default()}
                                </p>
                            </Show>
                        </div>

                        <!-- Split editor and preview -->
                        <div class="zd-theme-builder-columns">
                            <!-- Token Controls -->
                            <div class="zd-theme-builder-form">
                                <div class="zd-form-field">
                                    <label class="zd-form-label">{move || t(language.get(), "Theme Name", "نام تم")}</label>
                                    <input
                                        type="text"
                                        class="zd-input"
                                        prop:value=move || theme_name.get()
                                        on:input=move |ev| theme_name.set(event_target_value(&ev))
                                    />
                                </div>

                                <div class="zd-color-grid">
                                    <div class="zd-color-field">
                                        <label class="zd-color-label">{move || t(language.get(), "Background", "پس‌زمینه")}</label>
                                        <div class="zd-color-picker-wrap">
                                            <input
                                                type="color"
                                                class="zd-color-input"
                                                prop:value=move || bg_color.get()
                                                on:input=move |ev| bg_color.set(event_target_value(&ev))
                                            />
                                            <input
                                                type="text"
                                                class="zd-color-text"
                                                prop:value=move || bg_color.get()
                                                on:input=move |ev| bg_color.set(event_target_value(&ev))
                                            />
                                        </div>
                                    </div>

                                    <div class="zd-color-field">
                                        <label class="zd-color-label">{move || t(language.get(), "Text Color", "رنگ متن")}</label>
                                        <div class="zd-color-picker-wrap">
                                            <input
                                                type="color"
                                                class="zd-color-input"
                                                prop:value=move || text_color.get()
                                                on:input=move |ev| text_color.set(event_target_value(&ev))
                                            />
                                            <input
                                                type="text"
                                                class="zd-color-text"
                                                prop:value=move || text_color.get()
                                                on:input=move |ev| text_color.set(event_target_value(&ev))
                                            />
                                        </div>
                                    </div>

                                    <div class="zd-color-field">
                                        <label class="zd-color-label">{move || t(language.get(), "Accent", "رنگ شاخص")}</label>
                                        <div class="zd-color-picker-wrap">
                                            <input
                                                type="color"
                                                class="zd-color-input"
                                                prop:value=move || accent_color.get()
                                                on:input=move |ev| accent_color.set(event_target_value(&ev))
                                            />
                                            <input
                                                type="text"
                                                class="zd-color-text"
                                                prop:value=move || accent_color.get()
                                                on:input=move |ev| accent_color.set(event_target_value(&ev))
                                            />
                                        </div>
                                    </div>

                                    <div class="zd-color-field">
                                        <label class="zd-color-label">{move || t(language.get(), "Card Surface", "پس‌زمینه کارت")}</label>
                                        <div class="zd-color-picker-wrap">
                                            <input
                                                type="color"
                                                class="zd-color-input"
                                                prop:value=move || card_bg.get()
                                                on:input=move |ev| card_bg.set(event_target_value(&ev))
                                            />
                                            <input
                                                type="text"
                                                class="zd-color-text"
                                                prop:value=move || card_bg.get()
                                                on:input=move |ev| card_bg.set(event_target_value(&ev))
                                            />
                                        </div>
                                    </div>

                                    <div class="zd-color-field">
                                        <label class="zd-color-label">{move || t(language.get(), "Border", "حاشیه")}</label>
                                        <div class="zd-color-picker-wrap">
                                            <input
                                                type="color"
                                                class="zd-color-input"
                                                prop:value=move || border_color.get()
                                                on:input=move |ev| border_color.set(event_target_value(&ev))
                                            />
                                            <input
                                                type="text"
                                                class="zd-color-text"
                                                prop:value=move || border_color.get()
                                                on:input=move |ev| border_color.set(event_target_value(&ev))
                                            />
                                        </div>
                                    </div>
                                </div>

                                <div class="zd-form-field" style="margin-top: 12px;">
                                    <label class="zd-form-label">{move || t(language.get(), "Font Family", "خانواده فونت")}</label>
                                    <select
                                        class="zd-select"
                                        prop:value=move || font_family.get()
                                        on:change=move |ev| font_family.set(event_target_value(&ev))
                                    >
                                        <option value="'Vazirmatn', system-ui, sans-serif">"Vazirmatn (وزیرمتن - Modern Persian / Arabic)"</option>
                                        <option value="'Plus Jakarta Sans', system-ui, sans-serif">"Plus Jakarta Sans (Modern Editorial)"</option>
                                        <option value="'Inter', system-ui, sans-serif">"Inter (Clean Tech)"</option>
                                        <option value="'Outfit', system-ui, sans-serif">"Outfit (Warm Geometric)"</option>
                                        <option value="'JetBrains Mono', monospace">"JetBrains Mono (Developer)"</option>
                                        <option value="'Cinzel', 'Playfair Display', serif">"Cinzel (Luxury Serif)"</option>
                                        <option value="'Space Grotesk', Impact, sans-serif">"Space Grotesk (Brutalist)"</option>
                                        <option value="system-ui, -apple-system, sans-serif">"System Native Stack"</option>
                                    </select>
                                </div>
                            </div>

                            <!-- Live Preview Card -->
                            <div class="zd-theme-builder-preview">
                                <label class="zd-form-label">
                                    <span>{move || t(language.get(), "Live Preview", "پیش‌نمایش زنده تم")}</span>
                                </label>
                                <div
                                    class="zd-theme-preview-card"
                                    style=move || format!(
                                        "background-color: {}; color: {}; font-family: {}; border: 1px solid {};",
                                        bg_color.get(),
                                        text_color.get(),
                                        font_family.get(),
                                        border_color.get()
                                    )
                                >
                                    <div class="zd-preview-top-row">
                                        <span
                                            class="zd-preview-badge"
                                            style=move || format!(
                                                "background-color: {}; border: 1px solid {}; color: {};",
                                                card_bg.get(),
                                                border_color.get(),
                                                accent_color.get()
                                            )
                                        >
                                            {move || theme_name.get()}
                                        </span>
                                        <span
                                            class="zd-preview-dot"
                                            style=move || format!("background-color: {}; box-shadow: 0 0 10px {};", accent_color.get(), accent_color.get())
                                        ></span>
                                    </div>

                                    <h3 class="zd-preview-heading">
                                        "Crafting Next-Gen Interfaces"
                                    </h3>
                                    <p class="zd-preview-desc" style=move || format!("color: {}bb;", text_color.get())>
                                        "Synchronized tokens applied to every button, card, and motion timeline."
                                    </p>

                                    <div
                                        class="zd-preview-subcard"
                                        style=move || format!(
                                            "background-color: {}; border: 1px solid {};",
                                            card_bg.get(),
                                            border_color.get()
                                        )
                                    >
                                        <div class="zd-preview-stat-val" style=move || format!("color: {};", accent_color.get())>
                                            "99.8%"
                                        </div>
                                        <div class="zd-preview-stat-lbl">
                                            "Design Fidelity"
                                        </div>
                                    </div>

                                    <div class="zd-preview-actions">
                                        <button
                                            type="button"
                                            class="zd-preview-primary-btn"
                                            style=move || format!(
                                                "background-color: {}; color: #ffffff;",
                                                accent_color.get()
                                            )
                                        >
                                            "Explore"
                                        </button>
                                        <button
                                            type="button"
                                            class="zd-preview-secondary-btn"
                                            style=move || format!(
                                                "background-color: {}; border: 1px solid {}; color: {};",
                                                card_bg.get(),
                                                border_color.get(),
                                                text_color.get()
                                            )
                                        >
                                            "Inspect"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="zd-modal-footer">
                        <button
                            type="button"
                            class="zd-btn-secondary"
                            on:click=move |_| open.set(false)
                        >
                            {move || t(language.get(), "Cancel", "انصراف")}
                        </button>
                        <button
                            type="button"
                            class="zd-btn-primary"
                            on:click=on_apply
                        >
                            <Icon data=LuCheck size=14 />
                            <span>{move || t(language.get(), "Apply to Current Session", "اعمال به سشن فعلی")}</span>
                        </button>
                    </div>
                </div>
            </div>
        </Portal>
    </Show>
    }
}
