use std::cell::Cell;

use icondata_lu::{
    LuDownload, LuExternalLink, LuGlobe, LuLoaderCircle, LuPresentation, LuRotateCw, LuTriangleAlert,
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
) -> impl IntoView {
    let frame_status = RwSignal::new(FrameStatus::Loading);
    let has_loaded = RwSignal::new(false);
    let iframe_ref = NodeRef::<leptos::html::Iframe>::new();

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
