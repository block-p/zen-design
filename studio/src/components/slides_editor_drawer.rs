use gloo_net::http::Request;
use icondata_lu::{LuCheck, LuCode, LuColumns2, LuPenLine, LuPlus, LuSave, LuX};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::json;

use crate::data::{ActivityEntry, ActivityKind};
use crate::i18n::{t, Language};
use crate::icons::Icon;
use crate::stream::push_activity;

#[derive(Clone, Debug, PartialEq)]
pub struct SlideCardItem {
    pub index: usize,
    pub title: String,
    pub content: String,
    pub layout: String,
    pub raw_html: String,
}

#[component]
pub fn SlidesEditorDrawer(
    language: RwSignal<Language>,
    open: RwSignal<bool>,
    preview_version: RwSignal<u64>,
    entries: RwSignal<Vec<ActivityEntry>>,
) -> impl IntoView {
    let raw_html = RwSignal::new(String::new());
    let is_loading = RwSignal::new(false);
    let is_saving = RwSignal::new(false);
    let active_tab = RwSignal::new("cards"); // "cards" or "code"
    let selected_slide_idx = RwSignal::new(0_usize);

    // Fetch workspace artifact HTML
    let load_artifact = move || {
        is_loading.set(true);
        spawn_local(async move {
            match Request::get("/api/artifact").send().await {
                Ok(resp) if resp.ok() => {
                    if let Ok(data) = resp.json::<serde_json::Value>().await {
                        if let Some(html_str) = data.get("html").and_then(|v| v.as_str()) {
                            raw_html.set(html_str.to_string());
                        }
                    }
                    is_loading.set(false);
                }
                _ => {
                    is_loading.set(false);
                }
            }
        });
    };

    // Auto-load on open
    Effect::new(move |_| {
        if open.get() {
            load_artifact();
        }
    });

    // Save HTML back to workspace
    let save_artifact = move || {
        if is_saving.get() {
            return;
        }
        is_saving.set(true);
        let content = raw_html.get();
        let lang = language.get();

        spawn_local(async move {
            let body = json!({ "html": content });
            match Request::put("/api/artifact")
                .header("Content-Type", "application/json")
                .body(body.to_string())
            {
                Ok(req) => match req.send().await {
                    Ok(resp) if resp.ok() => {
                        is_saving.set(false);
                        preview_version.update(|v| *v = v.saturating_add(1));
                        push_activity(
                            entries,
                            ActivityKind::Done,
                            t(lang, "Slides updated and saved successfully.", "تغییرات اسلایدها با موفقیت ذخیره شد."),
                        );
                    }
                    _ => {
                        is_saving.set(false);
                        push_activity(
                            entries,
                            ActivityKind::Error,
                            t(lang, "Failed to save slides.", "خطا در ذخیره‌سازی اسلایدها."),
                        );
                    }
                },
                Err(err) => {
                    is_saving.set(false);
                    push_activity(entries, ActivityKind::Error, format!("Save error: {err}"));
                }
            }
        });
    };

    // Quick helper to insert new slide into raw HTML
    let add_new_slide = move || {
        let current = raw_html.get();
        let new_slide_markup = r#"
    <!-- Slide New -->
    <section class="slide" data-slide-id="new">
        <div class="slide-content">
            <span class="category-badge">NEW TOPIC</span>
            <h2 class="display-title">New Slide Title</h2>
            <p class="body-text">Add your detailed slide notes and bullet points here.</p>
        </div>
    </section>
"#;
        // If there is a container like `<main class="deck">` or `</body>`
        let updated = if let Some(pos) = current.find("</main>") {
            let mut s = current.clone();
            s.insert_str(pos, new_slide_markup);
            s
        } else if let Some(pos) = current.find("</body>") {
            let mut s = current.clone();
            s.insert_str(pos, new_slide_markup);
            s
        } else {
            format!("{}\n{}", current, new_slide_markup)
        };
        raw_html.set(updated);
        save_artifact();
    };

    view! {
        <Show when=move || open.get()>
            <div class="zd-slide-drawer-backdrop" on:click=move |_| open.set(false)></div>
            <aside class="zd-slide-drawer">
                <div class="zd-slide-drawer-header">
                    <div class="zd-slide-drawer-title-row">
                        <div class="zd-slide-drawer-icon">
                            <Icon data=LuPenLine size=15 />
                        </div>
                        <div>
                            <h2 class="zd-slide-drawer-title">
                                {move || t(language.get(), "Slides Deck Editor", "ویرایشگر دستی اسلایدها")}
                            </h2>
                            <p class="zd-slide-drawer-subtitle">
                                {move || t(language.get(), "Edit titles, text, structure or raw code", "ویرایش تیترها، متون، چیدمان و کد منبع اسلایدها")}
                            </p>
                        </div>
                    </div>
                    <div class="zd-slide-drawer-actions">
                        <button
                            type="button"
                            class="zd-btn-primary zd-slide-save-btn"
                            disabled=move || is_saving.get()
                            on:click=move |_| save_artifact()
                        >
                            <Show
                                when=move || is_saving.get()
                                fallback=move || view! {
                                    <Icon data=LuSave size=12 />
                                    <span>{move || t(language.get(), "Save Deck", "ذخیره اسلایدها")}</span>
                                }
                            >
                                <Icon data=LuCheck size=12 />
                                <span>{move || t(language.get(), "Saving…", "در حال ذخیره…")}</span>
                            </Show>
                        </button>
                        <button
                            type="button"
                            class="zd-btn-icon"
                            on:click=move |_| open.set(false)
                            title=move || t(language.get(), "Close", "بستن")
                        >
                            <Icon data=LuX size=14 />
                        </button>
                    </div>
                </div>

                <div class="zd-slide-drawer-tabs">
                    <button
                        type="button"
                        class=move || if active_tab.get() == "cards" { "zd-slide-tab zd-slide-tab--active" } else { "zd-slide-tab" }
                        on:click=move |_| active_tab.set("cards")
                    >
                        <Icon data=LuColumns2 size=12 />
                        <span>{move || t(language.get(), "Visual Deck Inspector", "مدیریت بصری اسلایدها")}</span>
                    </button>
                    <button
                        type="button"
                        class=move || if active_tab.get() == "code" { "zd-slide-tab zd-slide-tab--active" } else { "zd-slide-tab" }
                        on:click=move |_| active_tab.set("code")
                    >
                        <Icon data=LuCode size=12 />
                        <span>{move || t(language.get(), "Full Source / HTML", "کد منبع HTML")}</span>
                    </button>
                </div>

                <div class="zd-slide-drawer-body">
                    <Show
                        when=move || active_tab.get() == "cards"
                        fallback=move || view! {
                            <div class="zd-slide-code-panel">
                                <div class="zd-slide-code-hint">
                                    <Icon data=LuCode size=12 />
                                    <span>{move || t(language.get(), "Direct HTML edits apply instantly to your workspace index.html", "ویرایش مستقیم کد HTML بی‌درنگ در پروژه ذخیره می‌شود")}</span>
                                </div>
                                <textarea
                                    class="zd-slide-raw-textarea"
                                    prop:value=move || raw_html.get()
                                    on:input=move |ev| raw_html.set(event_target_value(&ev))
                                    spellcheck="false"
                                ></textarea>
                            </div>
                        }
                    >
                        <div class="zd-slide-visual-panel">
                            <div class="zd-slide-toolbar">
                                <button
                                    type="button"
                                    class="zd-btn-soft zd-slide-add-btn"
                                    on:click=move |_| add_new_slide()
                                >
                                    <Icon data=LuPlus size=12 />
                                    <span>{move || t(language.get(), "Add New Slide", "افزودن اسلاید جدید")}</span>
                                </button>
                                <span class="zd-slide-hint">
                                    {move || t(language.get(), "Tip: You can also click directly on any slide in the preview canvas to edit in-place!", "نکته: می‌توانید روی خود بوم پیش‌نمایش نیز مستقیماً متن‌ها را ادیت کنید!")}
                                </span>
                            </div>

                            <div class="zd-slide-quick-list">
                                <div class="zd-slide-quick-card">
                                    <div class="zd-slide-quick-badge">"SLIDE OVERVIEW"</div>
                                    <p class="zd-slide-quick-info">
                                        {move || t(
                                            language.get(),
                                            "In-place editing is enabled on the live preview iframe. Use the Quick Editor toolbar above the preview canvas to format headings, body copy, and toggle contenteditable mode.",
                                            "حالت ویرایش زنده روی بوم فعال است. از دکمه‌های نوار ابزار بالای بوم برای تغییر تیترها، محتوا و ذخیره آسان استفاده کنید."
                                        )}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </Show>
                </div>
            </aside>
        </Show>
    }
}
