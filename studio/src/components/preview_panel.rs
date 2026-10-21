use std::cell::Cell;

use icondata_lu::{
    LuChevronLeft, LuChevronRight, LuDownload, LuExternalLink, LuGlobe, LuLoaderCircle, LuPenLine,
    LuPresentation, LuRotateCw, LuSave, LuTriangleAlert,
};
use leptos::prelude::*;

use crate::actions::export_video;
use crate::data::{
    device_meta, ActivityEntry, AspectRatio, ConnectionState, DeviceId, ModeId, ASPECT_RATIOS,
    DEVICES,
};
use crate::i18n::{t, Language};
use crate::icons::Icon;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FrameStatus {
    Loading,
    Ready,
    Stalled,
}

const LOAD_TIMEOUT_MS: u32 = 12_000;
/// A file being written right now can briefly serve as an empty document.
/// Retry transparently a few times before showing any fallback panel.
const EMPTY_RETRY_LIMIT: u32 = 3;
const EMPTY_RETRY_DELAY_MS: u32 = 550;

thread_local! {
    /// Transparent retries for mid-write empty documents.
    static EMPTY_RETRIES: Cell<u32> = const { Cell::new(0) };
}

#[component]
#[allow(unused_variables)]
pub fn PreviewPanel(
    language: RwSignal<Language>,
    device: RwSignal<DeviceId>,
    preview_version: RwSignal<u64>,
    last_reload_at: RwSignal<Option<f64>>,
    event_count: RwSignal<u32>,
    connection: RwSignal<ConnectionState>,
    running: RwSignal<bool>,
    exporting: RwSignal<bool>,
    entries: RwSignal<Vec<ActivityEntry>>,
    mode: RwSignal<ModeId>,
    aspect_ratio: RwSignal<AspectRatio>,
    slides_drawer_open: RwSignal<bool>,
) -> impl IntoView {
    let frame_status = RwSignal::new(FrameStatus::Loading);
    let has_loaded = RwSignal::new(false);
    let iframe_ref = NodeRef::<leptos::html::Iframe>::new();
    let is_editing_inplace = RwSignal::new(false);
    let is_saving_inplace = RwSignal::new(false);
    let trigger_save = RwSignal::new(false);

    // Slide step helper for live iframe
    let step_slide = move |dir: i32| {
        if let Some(frame) = iframe_ref.get_untracked() {
            if let Some(doc) = frame.content_document() {
                let code = format!(
                    "(function() {{ if (typeof window.__zd_slide_step === 'function') {{ window.__zd_slide_step({}); }} else if (typeof go === 'function') {{ const s = document.querySelectorAll('.slide'); let c = 0; s.forEach((el, i) => {{ if (el.classList.contains('active')) c = i; }}); go(c + ({})); }} }})();",
                    dir, dir
                );
                if let Ok(script) = doc.create_element("script") {
                    script.set_text_content(Some(&code));
                    if let Some(body) = doc.body() {
                        let _ = body.append_child(&script);
                        let _ = script.remove();
                    }
                }
            }
        }
    };

    // Toggle direct in-place text editing on canvas
    let toggle_inplace_edit = move || {
        let next = !is_editing_inplace.get();
        is_editing_inplace.set(next);

        if let Some(frame) = iframe_ref.get_untracked() {
            if let Some(doc) = frame.content_document() {
                if next {
                    // Inject or enable the visual editing helper
                    let script_id = "zd-visual-editor-runtime";
                    if doc.get_element_by_id(script_id).is_none() {
                        if let Ok(script_elem) = doc.create_element("script") {
                            let _ = script_elem.set_id(script_id);
                            let script_code = r#"
(function() {
    window.__zd_editor_active = true;
    
    // 1. Force enable user selection & edit styles
    let style = document.getElementById('zd-editor-styles');
    if (!style) {
        style = document.createElement('style');
        style.id = 'zd-editor-styles';
        style.textContent = `
            *, *::before, *::after {
                user-select: text !important;
                -webkit-user-select: text !important;
            }
            .zd-text-editable {
                outline: 1.5px dashed rgba(99, 102, 241, 0.45) !important;
                outline-offset: 3px !important;
                border-radius: 4px !important;
                cursor: text !important;
                transition: outline 0.15s ease, background-color 0.15s ease !important;
            }
            .zd-text-editable:hover {
                outline: 2px solid #6366f1 !important;
                background-color: rgba(99, 102, 241, 0.08) !important;
            }
            .zd-text-editable:focus, [contenteditable="true"]:focus {
                outline: 2px solid #10b981 !important;
                outline-offset: 2px !important;
                background-color: rgba(16, 185, 129, 0.10) !important;
                box-shadow: 0 0 10px rgba(16, 185, 129, 0.3) !important;
                cursor: text !important;
            }
        `;
        document.head.appendChild(style);
    }

    // 2. Mark all text elements as directly editable
    const sel = 'h1, h2, h3, h4, h5, h6, p, span, li, td, th, a, button, blockquote, [class*="title"], [class*="sub"], [class*="badge"], [class*="desc"], [class*="item"], [class*="val"], [class*="label"], [class*="num"], [class*="stat"], [class*="card"], em, strong, b, code';
    const elements = document.querySelectorAll(sel);
    elements.forEach(el => {
        el.setAttribute('contenteditable', 'true');
        el.setAttribute('spellcheck', 'false');
        el.classList.add('zd-text-editable');
    });

    // 3. Listen for Ctrl+S / Cmd+S inside iframe
    if (!window.__zd_key_listener) {
        window.__zd_key_listener = function(e) {
            if ((e.ctrlKey || e.metaKey) && (e.key === 's' || e.key === 'S')) {
                e.preventDefault();
                window.parent.postMessage({ type: 'zd-save-inplace' }, '*');
            }
        };
        document.addEventListener('keydown', window.__zd_key_listener);
    }

    // 4. Slide navigation helpers
    window.__zd_slide_step = function(dir) {
        if (dir > 0) {
            const nextBtn = document.getElementById('nextBtn') || document.querySelector('.next-btn, [data-action="next"]');
            if (nextBtn) { nextBtn.click(); return; }
        } else {
            const prevBtn = document.getElementById('prevBtn') || document.querySelector('.prev-btn, [data-action="prev"]');
            if (prevBtn) { prevBtn.click(); return; }
        }
        if (typeof window.go === 'function') {
            const slides = document.querySelectorAll('.slide');
            let cur = 0;
            slides.forEach((s, idx) => { if (s.classList.contains('active')) cur = idx; });
            let target = cur + dir;
            if (target >= 0 && target < slides.length) {
                window.go(target);
                return;
            }
        }
        const key = dir > 0 ? (document.dir === 'rtl' ? 'ArrowLeft' : 'ArrowRight') : (document.dir === 'rtl' ? 'ArrowRight' : 'ArrowLeft');
        document.dispatchEvent(new KeyboardEvent('keydown', { key: key, bubbles: true }));
    };

    window.__zd_go = function(targetIdx) {
        if (typeof window.go === 'function') {
            window.go(targetIdx);
            return;
        }
        const slides = document.querySelectorAll('.slide');
        if (slides.length > targetIdx && targetIdx >= 0) {
            slides.forEach((s, idx) => {
                if (idx === targetIdx) {
                    s.classList.add('active');
                    s.classList.remove('exit');
                } else {
                    s.classList.remove('active');
                }
            });
        }
    };
})();
"#;
                            script_elem.set_text_content(Some(script_code));
                            if let Some(head) = doc.head() {
                                let _ = head.append_child(&script_elem);
                            }
                        }
                    }
                } else {
                    // Turn off visual editor inside iframe
                    if let Ok(clean_script) = doc.create_element("script") {
                        clean_script.set_text_content(Some(r#"
(function() {
    window.__zd_editor_active = false;
    const style = document.getElementById('zd-editor-styles');
    if (style) style.remove();
    const script = document.getElementById('zd-visual-editor-runtime');
    if (script) script.remove();
    document.querySelectorAll('.zd-text-editable, [contenteditable="true"]').forEach(el => {
        el.classList.remove('zd-text-editable', 'zd-editable-highlight', 'zd-editing-text');
        el.removeAttribute('contenteditable');
        el.removeAttribute('spellcheck');
    });
})();
"#));
                        if let Some(body) = doc.body() {
                            let _ = body.append_child(&clean_script);
                            let _ = clean_script.remove();
                        }
                    }
                }
            }
        }
    };

    // Save in-place edited HTML back to server
    let save_inplace_content = move || {
        if is_saving_inplace.get() {
            return;
        }
        let Some(frame) = iframe_ref.get_untracked() else { return; };
        let Some(doc) = frame.content_document() else { return; };
        let Some(doc_elem) = doc.document_element() else { return; };

        // Clean up editing attributes and runtime helper tags before saving
        if let Ok(clean_script) = doc.create_element("script") {
            clean_script.set_text_content(Some(r#"
(function() {
    const style = document.getElementById('zd-editor-styles');
    if (style) style.remove();
    const script = document.getElementById('zd-visual-editor-runtime');
    if (script) script.remove();
    document.querySelectorAll('.zd-editable-highlight, .zd-editing-text, .zd-text-editable, [contenteditable="true"]').forEach(el => {
        el.classList.remove('zd-editable-highlight', 'zd-editing-text', 'zd-text-editable');
        el.removeAttribute('contenteditable');
        el.removeAttribute('spellcheck');
    });
})();
"#));
            if let Some(body) = doc.body() {
                let _ = body.append_child(&clean_script);
                let _ = clean_script.remove();
                let _ = body.remove_attribute("contenteditable");
                let _ = body.remove_attribute("style");
            }
        }
        is_editing_inplace.set(false);

        let html = format!("<!DOCTYPE html>\n{}", doc_elem.outer_html());
        is_saving_inplace.set(true);
        let lang = language.get();

        leptos::task::spawn_local(async move {
            let body = serde_json::json!({ "html": html });
            match gloo_net::http::Request::put("/api/artifact")
                .header("Content-Type", "application/json")
                .body(body.to_string())
            {
                Ok(req) => match req.send().await {
                    Ok(resp) if resp.ok() => {
                        is_saving_inplace.set(false);
                        preview_version.update(|v| *v = v.saturating_add(1));
                        crate::stream::push_activity(
                            entries,
                            crate::data::ActivityKind::Done,
                            t(lang, "In-place slide edits saved.", "تغییرات درون‌متنی اسلاید ذخیره شد."),
                        );
                    }
                    _ => {
                        is_saving_inplace.set(false);
                    }
                },
                Err(_) => {
                    is_saving_inplace.set(false);
                }
            }
        });
    };

    // Watch for shortcut triggers from iframe message
    Effect::new(move |_| {
        if trigger_save.get() {
            trigger_save.set(false);
            save_inplace_content();
        }
    });

    // Listen for Ctrl+S message event from inside iframe
    Effect::new(move |_| {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        if let Some(win) = web_sys::window() {
            let cb = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
                if let Ok(val) = event.data().dyn_into::<js_sys::Object>() {
                    let type_val = js_sys::Reflect::get(&val, &wasm_bindgen::JsValue::from_str("type")).ok();
                    if let Some(t_str) = type_val.and_then(|v| v.as_string()) {
                        if t_str == "zd-save-inplace" {
                            trigger_save.set(true);
                        }
                    }
                }
            }) as Box<dyn FnMut(web_sys::MessageEvent)>);
            let _ = win.add_event_listener_with_callback("message", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    // A fresh version begins a fresh watchdog. It must never overwrite a
    // successfully loaded frame: before this guard, an idle studio (no reload
    // events for 12s) showed the fallback on top of a perfectly rendered
    // artifact.
    Effect::new(move |_| {
        let current = preview_version.get();
        frame_status.set(FrameStatus::Loading);

        let status = frame_status;
        let versions = preview_version;
        gloo_timers::callback::Timeout::new(LOAD_TIMEOUT_MS, move || {
            if versions.get_untracked() == current && !has_loaded.get_untracked() {
                status.set(FrameStatus::Stalled);
            }
        })
        .forget();
    });

    // Self-healing: if the status says "not ready" but the iframe visibly has
    // a document, trust the document. This covers a load event that fires
    // before the document commits (it reads the old blank document and would
    // otherwise leave the fallback covering real content).
    gloo_timers::callback::Interval::new(2_000, move || {
        if frame_status.get_untracked() == FrameStatus::Ready {
            return;
        }
        let Some(frame) = iframe_ref.get_untracked() else {
            return;
        };
        let has_content = frame
            .content_document()
            .and_then(|document| document.body())
            .map(|body| {
                body.child_element_count() > 0
                    || !body.text_content().unwrap_or_default().trim().is_empty()
            })
            .unwrap_or(false);
        if has_content {
            EMPTY_RETRIES.with(|counter| counter.set(0));
            frame_status.set(FrameStatus::Ready);
            has_loaded.set(true);
        }
    })
    .forget();

    // One transparent retry budget shared by empty loads and navigation errors.
    let retry_or_stall = move || {
        let attempts = EMPTY_RETRIES.with(Cell::get);
        if attempts < EMPTY_RETRY_LIMIT {
            EMPTY_RETRIES.with(|counter| counter.set(attempts + 1));
            frame_status.set(FrameStatus::Loading);
            let versions = preview_version;
            gloo_timers::callback::Timeout::new(EMPTY_RETRY_DELAY_MS, move || {
                versions.update(|version| *version = version.saturating_add(1));
            })
            .forget();
        } else {
            frame_status.set(FrameStatus::Stalled);
        }
    };

    let handle_frame_load = move || {
        let Some(frame) = iframe_ref.get_untracked() else {
            return;
        };

        let href = frame
            .content_window()
            .and_then(|window| window.location().href().ok())
            .unwrap_or_default();

        // about:blank fires before the real navigation begins.
        if href == "about:blank" {
            return;
        }

        let is_empty = frame
            .content_document()
            .and_then(|document| document.body())
            .map(|body| {
                body.child_element_count() == 0
                    && body.text_content().unwrap_or_default().trim().is_empty()
            })
            .unwrap_or(false);

        if is_empty {
            // The load event can fire before the new document commits, so give
            // the frame a beat and re-read it before spending a retry. A file
            // that is genuinely mid-write still gets the quiet retry treatment.
            gloo_timers::callback::Timeout::new(250, move || {
                let still_empty = iframe_ref
                    .get_untracked()
                    .and_then(|frame| frame.content_document())
                    .and_then(|document| document.body())
                    .map(|body| {
                        body.child_element_count() == 0
                            && body.text_content().unwrap_or_default().trim().is_empty()
                    })
                    .unwrap_or(true);

                if still_empty {
                    retry_or_stall();
                } else {
                    EMPTY_RETRIES.with(|counter| counter.set(0));
                    frame_status.set(FrameStatus::Ready);
                    has_loaded.set(true);
                }
            })
            .forget();
            return;
        }

        EMPTY_RETRIES.with(|counter| counter.set(0));
        frame_status.set(FrameStatus::Ready);
        has_loaded.set(true);
    };

    // A manual reload starts a fresh retry budget.
    let bump_preview = move || {
        EMPTY_RETRIES.with(|counter| counter.set(0));
        preview_version.update(|version| *version = version.saturating_add(1));
        last_reload_at.set(Some(js_sys::Date::now()));
    };

    view! {
        <section class="zd-preview">
            <div class="zd-preview-toolbar">
                <div class="zd-preview-info">
                    <div class="zd-preview-globe-icon">
                        <Icon data=LuGlobe size=14 stroke_width=1.9 />
                    </div>
                    <div class="zd-preview-info-text">
                        <div class="zd-preview-title-row">
                            <span class="zd-preview-title">
                                {move || t(language.get(), "Preview", "پیش‌نمایش")}
                            </span>
                        </div>
                    </div>
                </div>

                <div class="zd-preview-actions">
                    <Show
                        when=move || mode.get() == ModeId::Motion
                        fallback=move || view! {
                            <div class="zd-segmented">
                                {DEVICES
                                    .iter()
                                    .map(|item| {
                                        let item = *item;
                                        view! {
                                            <button
                                                type="button"
                                                on:click=move |_| device.set(item.id)
                                                aria-pressed=move || (device.get() == item.id).to_string()
                                                title=move || format!("{} — {}", item.label(language.get()), item.width_label(language.get()))
                                                class=move || {
                                                    if device.get() == item.id {
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
                        }
                    >
                        <div class="zd-segmented">
                            {ASPECT_RATIOS
                                .iter()
                                .map(|item| {
                                    let item = *item;
                                    view! {
                                        <button
                                            type="button"
                                            on:click=move |_| aspect_ratio.set(item.id)
                                            aria-pressed=move || (aspect_ratio.get() == item.id).to_string()
                                            title=move || format!("{} — {}", item.label(language.get()), item.description(language.get()))
                                            class=move || {
                                                if aspect_ratio.get() == item.id {
                                                    "zd-segmented-btn zd-segmented-btn--active"
                                                } else {
                                                    "zd-segmented-btn"
                                                }
                                            }
                                        >
                                            <span class="zd-segmented-btn-label">{move || item.label(language.get())}</span>
                                        </button>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </Show>

                    <!-- In-place visual edit toggle (Move blocks & edit text) -->
                    <div class="zd-preview-divider"></div>

                        <!-- In-place visual edit toggle -->
                        <button
                            type="button"
                            class=move || {
                                if is_editing_inplace.get() {
                                    "zd-btn-soft zd-btn-slide-edit zd-btn-slide-edit--active"
                                } else {
                                    "zd-btn-soft zd-btn-slide-edit"
                                }
                            }
                            on:click=move |_| toggle_inplace_edit()
                            title=move || {
                                if is_editing_inplace.get() {
                                    t(language.get(), "Finish in-place editing", "پایان ویرایش درون‌متنی")
                                } else {
                                    t(language.get(), "Edit slide text directly on canvas", "ویرایش مستقیم متن اسلاید روی صفحه")
                                }
                            }
                        >
                            <Icon data=LuPenLine size=12 />
                            <span>
                                {move || {
                                    if is_editing_inplace.get() {
                                        t(language.get(), "Editing…", "در حال ویرایش…")
                                    } else {
                                        t(language.get(), "Edit In-Place", "ویرایش روی بوم")
                                    }
                                }}
                            </span>
                        </button>

                        <!-- Save In-place changes button (shown when editing) -->
                        <Show when=move || is_editing_inplace.get()>
                            <button
                                type="button"
                                class="zd-btn-primary zd-btn-slide-save"
                                disabled=move || is_saving_inplace.get()
                                on:click=move |_| save_inplace_content()
                                title=move || t(language.get(), "Save your in-place edits to disk", "ذخیره تغییرات روی دیسک")
                            >
                                <Icon data=LuSave size=12 />
                                <span>{move || t(language.get(), "Save", "ذخیره")}</span>
                            </button>
                        </Show>

                        <!-- Slide Navigation (for Slides mode) -->
                        <Show when=move || mode.get() == ModeId::Slides>
                            <div class="zd-slide-nav-group">
                                <button
                                    type="button"
                                    class="zd-btn-icon zd-btn-slide-nav"
                                    on:click=move |_| step_slide(-1)
                                    title=move || t(language.get(), "Previous slide", "اسلاید قبلی")
                                >
                                    <Icon data=LuChevronRight size=13 />
                                </button>
                                <span class="zd-slide-nav-badge">
                                    {move || t(language.get(), "Slide", "اسلاید")}
                                </span>
                                <button
                                    type="button"
                                    class="zd-btn-icon zd-btn-slide-nav"
                                    on:click=move |_| step_slide(1)
                                    title=move || t(language.get(), "Next slide", "اسلاید بعدی")
                                >
                                    <Icon data=LuChevronLeft size=13 />
                                </button>
                            </div>

                            <button
                                type="button"
                                class="zd-btn-soft zd-btn-slide-deck"
                                on:click=move |_| slides_drawer_open.set(true)
                                title=move || t(language.get(), "Open Slides Deck Inspector & Source Editor", "باز کردن پنل مدیریت اسلایدها و ویرایشگر کد")
                            >
                                <Icon data=LuPresentation size=12 />
                                <span>{move || t(language.get(), "Deck Editor", "مدیریت اسلایدها")}</span>
                            </button>
                        </Show>

                    <div class="zd-preview-divider"></div>

                    <button
                        type="button"
                        on:click=move |_| bump_preview()
                        title=move || t(language.get(), "Reload preview", "بازخوانی پیش‌نمایش")
                        class="zd-btn-icon"
                    >
                        <Icon data=LuRotateCw size=13 />
                    </button>

                    <button
                        type="button"
                        on:click=move |_| export_video(exporting, entries)
                        disabled=move || exporting.get() || running.get()
                        title=move || {
                            let lang = language.get();
                            if exporting.get() {
                                t(lang, "Recording the preview — the MP4 downloads when it finishes", "در حال ضبط پیش‌نمایش — پس از اتمام MP4 دانلود خواهد شد").to_string()
                            } else if running.get() {
                                t(lang, "Wait for the current run to finish before exporting", "قبل از خروجی گرفتن منتظر پایان اجرای فعلی بمانید").to_string()
                            } else {
                                t(lang, "Export an 8-second MP4 of the preview", "خروجی فیلم ۸ ثانیه‌ای MP4 از پیش‌نمایش").to_string()
                            }
                        }
                        class=move || {
                            if exporting.get() {
                                "zd-btn-icon zd-btn-icon--exporting"
                            } else {
                                "zd-btn-icon"
                            }
                        }
                    >
                        <Show
                            when=move || exporting.get()
                            fallback=move || view! { <Icon data=LuDownload size=13 /> }
                        >
                            <Icon data=LuLoaderCircle size=13 class="zd-spin" />
                        </Show>
                    </button>

                    <a
                        href="/api/export/pptx"
                        download="presentation.pptx"
                        title=move || t(language.get(), "Export PowerPoint (.pptx)", "خروجی پاورپوینت (PPTX)").to_string()
                        class="zd-btn-icon"
                    >
                        <Icon data=LuPresentation size=13 />
                    </a>

                    <a
                        href="/preview/index.html"
                        target="_blank"
                        rel="noreferrer"
                        title=move || t(language.get(), "Open preview in a new tab", "باز کردن پیش‌نمایش در تب جدید")
                        class="zd-btn-icon"
                    >
                        <Icon data=LuExternalLink size=13 />
                    </a>
                </div>
            </div>

            <div class="zd-preview-canvas">
                <div class=move || {
                    if mode.get() == ModeId::Motion {
                        let ratio_class = match aspect_ratio.get() {
                            AspectRatio::R16_9 => "zd-frame--aspect-16-9",
                            AspectRatio::R9_16 => "zd-frame--aspect-9-16",
                            AspectRatio::R4_3 => "zd-frame--aspect-4-3",
                            AspectRatio::R1_1 => "zd-frame--aspect-1-1",
                            AspectRatio::Fill => "zd-frame--aspect-fill",
                        };
                        format!("zd-device-frame {ratio_class}")
                    } else {
                        format!("zd-device-frame {}", device_meta(device.get()).frame_class)
                    }
                }>
                    <div class="zd-device-viewport">
                        <iframe
                            node_ref=iframe_ref
                            src=move || format!("/preview/index.html?v={}", preview_version.get())
                            title="ZenDesign live preview"
                            on:load=move |_| handle_frame_load()
                            on:error=move |_| retry_or_stall()
                            sandbox="allow-scripts allow-same-origin allow-forms allow-modals allow-popups allow-downloads"
                            class="zd-device-iframe"
                        ></iframe>

                        <Show when=move || frame_status.get() != FrameStatus::Ready>
                            <div class="zd-preview-overlay">
                                <Show
                                    when=move || frame_status.get() == FrameStatus::Loading
                                    fallback=move || view! {
                                        <div class="zd-preview-overlay-content">
                                            <div class="zd-preview-overlay-icon zd-preview-overlay-icon--warning">
                                                <Icon data=LuTriangleAlert size=20 />
                                            </div>
                                            <p class="zd-preview-overlay-title">
                                                {move || t(language.get(), "Preview is waiting for the artifact", "پیش‌نمایش در انتظار آرتیفکت است")}
                                            </p>
                                            <p class="zd-preview-overlay-desc">
                                                {move || t(language.get(), "The workspace file looks empty right now — it may still be mid-write. Retry, or wait for the agent to finish.", "فایل محیط کار در حال حاضر خالی است — ممکن است در حین نوشتن باشد. مجدداً تلاش کنید یا منتظر پایان عامل بمانید.")}
                                            </p>
                                            <button
                                                type="button"
                                                on:click=move |_| bump_preview()
                                                class="zd-btn-overlay-retry"
                                            >
                                                <Icon data=LuRotateCw size=12 />
                                                <span>{move || t(language.get(), "Retry preview", "تلاش مجدد")}</span>
                                            </button>
                                        </div>
                                    }
                                >
                                    <div class="zd-preview-overlay-content">
                                        <div class="zd-preview-overlay-icon zd-preview-overlay-icon--loading">
                                            <Icon data=LuLoaderCircle size=20 class="zd-spin" />
                                        </div>
                                        <p class="zd-preview-overlay-title">
                                            {move || {
                                                let lang = language.get();
                                                if has_loaded.get() {
                                                    t(lang, "Syncing workspace…", "همگام‌سازی محیط کار…")
                                                } else {
                                                    t(lang, "Rendering workspace…", "رندر محیط کار…")
                                                }
                                            }}
                                        </p>
                                        <p class="zd-preview-overlay-desc">
                                            {move || {
                                                let lang = language.get();
                                                if has_loaded.get() {
                                                    t(lang, "Applying the latest debounced file changes", "اعمال تغییرات ذخیره شده فایل")
                                                } else {
                                                    t(lang, "Starting the ZenDesign artifact preview", "شروع پیش‌نمایش آرتیفکت زن‌دیزاین")
                                                }
                                            }}
                                        </p>
                                        <div class="zd-preview-sweep-wrap">
                                            <div class="zd-preview-sweep-bar"></div>
                                        </div>
                                    </div>
                                </Show>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
        </section>
    }
}
