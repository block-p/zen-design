use icondata_lu::{
    LuClock, LuCornerDownLeft, LuExternalLink, LuFolderOpen, LuHistory, LuPlay, LuSearch,
    LuSparkles, LuTrash2, LuX,
};
use leptos::portal::Portal;
use leptos::prelude::*;

use crate::actions::{activate_project, clear_all_history, delete_history_item, generate};
use crate::data::{
    mode_meta, system_meta, ActivityEntry, ChatHistoryItem, DesignSystemId, ModeId,
};
use crate::i18n::{localize_digits, t, Language};
use crate::icons::Icon;

fn format_relative_time(timestamp: u64, lang: Language) -> String {
    let now = js_sys::Date::now() / 1000.0;
    let diff = (now as i64) - (timestamp as i64);

    if diff < 60 {
        t(lang, "just now", "همین الان").to_string()
    } else if diff < 3600 {
        let mins = (diff / 60).max(1);
        let raw = format!("{mins}m ago");
        match lang {
            Language::En => raw,
            Language::Fa => format!("{} دقیقه پیش", localize_digits(&mins.to_string(), lang)),
        }
    } else if diff < 86400 {
        let hours = (diff / 3600).max(1);
        let raw = format!("{hours}h ago");
        match lang {
            Language::En => raw,
            Language::Fa => format!("{} ساعت پیش", localize_digits(&hours.to_string(), lang)),
        }
    } else {
        let days = (diff / 86400).max(1);
        let raw = format!("{days}d ago");
        match lang {
            Language::En => raw,
            Language::Fa => format!("{} روز پیش", localize_digits(&days.to_string(), lang)),
        }
    }
}

#[component]
pub fn HistoryModal(
    language: RwSignal<Language>,
    open: RwSignal<bool>,
    history: RwSignal<Vec<ChatHistoryItem>>,
    prompt: RwSignal<String>,
    design_system: RwSignal<DesignSystemId>,
    mode: RwSignal<ModeId>,
    deep: RwSignal<bool>,
    running: RwSignal<bool>,
    entries: RwSignal<Vec<ActivityEntry>>,
    preview_version: RwSignal<u64>,
) -> impl IntoView {
    let search = RwSignal::new(String::new());

    let filtered_items = Memo::new(move |_| {
        let q = search.get().trim().to_lowercase();
        let list = history.get();
        if q.is_empty() {
            list
        } else {
            list.into_iter()
                .filter(|item| {
                    item.prompt.to_lowercase().contains(&q)
                        || item.design_system.to_lowercase().contains(&q)
                        || item.mode.to_lowercase().contains(&q)
                })
                .collect()
        }
    });

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
                        class="zd-modal-window zd-history-modal"
                    >
                        <header class="zd-modal-header">
                            <div class="zd-modal-header-info">
                                <div class="zd-history-title-row">
                                    <Icon data=LuHistory size=16 class="zd-history-title-icon" />
                                    <h2 class="zd-modal-title">
                                        {move || t(language.get(), "Project & Chat History", "تاریخچه پروژه‌ها و سشن‌ها")}
                                    </h2>
                                    <span class="zd-history-count-badge">
                                        {move || {
                                            let count = history.get().len().to_string();
                                            localize_digits(&count, language.get())
                                        }}
                                    </span>
                                </div>
                                <p class="zd-modal-subtitle">
                                    {move || t(
                                        language.get(),
                                        "Click 'Open Project' to restore the design directly in the live preview canvas",
                                        "با کلیک روی «مشاهده پروژه»، طرح کامل مستقیماً در بوم پیش‌نمایش بارگذاری می‌شود"
                                    )}
                                </p>
                            </div>
                            <div class="zd-history-header-actions">
                                <Show when=move || !history.get().is_empty()>
                                    <button
                                        type="button"
                                        class="zd-btn-soft zd-history-clear-btn"
                                        on:click=move |_| clear_all_history(history)
                                        title=move || t(language.get(), "Clear all history", "پاکسازی کل تاریخچه")
                                    >
                                        <Icon data=LuTrash2 size=12 />
                                        <span>{move || t(language.get(), "Clear all", "پاکسازی")}</span>
                                    </button>
                                </Show>
                                <button
                                    type="button"
                                    on:click=move |_| open.set(false)
                                    title=move || t(language.get(), "Close", "بستن")
                                    class="zd-btn-icon"
                                >
                                    <Icon data=LuX size=14 />
                                </button>
                            </div>
                        </header>

                        <div class="zd-history-search-bar">
                            <Icon data=LuSearch size=13 class="zd-history-search-icon" />
                            <input
                                type="text"
                                class="zd-history-search-input"
                                placeholder=move || t(language.get(), "Search past projects & prompts…", "جستجو در پروژه‌ها و پرامپت‌ها…")
                                prop:value=move || search.get()
                                on:input=move |ev| search.set(event_target_value(&ev))
                            />
                            <Show when=move || !search.get().is_empty()>
                                <button
                                    type="button"
                                    class="zd-history-search-clear"
                                    on:click=move |_| search.set(String::new())
                                >
                                    <Icon data=LuX size=11 />
                                </button>
                            </Show>
                        </div>

                        <div class="zd-modal-body zd-history-list-body">
                            <Show
                                when=move || !filtered_items.get().is_empty()
                                fallback=move || view! {
                                    <div class="zd-history-empty">
                                        <div class="zd-history-empty-icon">
                                            <Icon data=LuClock size=28 />
                                        </div>
                                        <p class="zd-history-empty-title">
                                            {move || if search.get().is_empty() {
                                                t(language.get(), "No project history yet", "هنوز پروژه‌ای ثبت نشده است")
                                            } else {
                                                t(language.get(), "No matching projects found", "موردی با این جستجو پیدا نشد")
                                            }}
                                        </p>
                                        <p class="zd-history-empty-desc">
                                            {move || if search.get().is_empty() {
                                                t(language.get(), "Each generated design creates an isolated project session folder automatically.", "هر بار که طرحی تولید می‌کنید، یک پوشه پروژه مجزا برای آن ساخته می‌شود.")
                                            } else {
                                                t(language.get(), "Try searching for a different keyword.", "کلمه کلیدی دیگری را جستجو کنید.")
                                            }}
                                        </p>
                                    </div>
                                }
                            >
                                <div class="zd-history-items-grid">
                                    {move || {
                                        let lang = language.get();
                                        filtered_items.get().into_iter().map(|item| {
                                            let p_text = item.prompt.clone();
                                            let run_prompt = item.prompt.clone();
                                            let load_prompt = item.prompt.clone();
                                            let item_id = item.id.clone();
                                            let open_id = item.id.clone();
                                            let ext_id = item.id.clone();
                                            let ds = DesignSystemId::parse(&item.design_system);
                                            let md = ModeId::parse(&item.mode);
                                            let is_deep = item.deep;
                                            let time_str = format_relative_time(item.timestamp, lang);
                                            let ds_info = system_meta(ds);
                                            let md_info = mode_meta(md);

                                            view! {
                                                <div class="zd-history-card">
                                                    <div class="zd-history-card-header">
                                                        <div class="zd-history-badges">
                                                            <span
                                                                class="zd-history-tag zd-history-tag--system"
                                                                style=format!("--dot-color: {}", ds_info.dot)
                                                            >
                                                                <span class="zd-history-tag-dot" style=format!("background-color: {}", ds_info.dot)></span>
                                                                <span>{ds_info.label(lang)}</span>
                                                            </span>
                                                            <span class="zd-history-tag zd-history-tag--mode">
                                                                <span>{md_info.label(lang)}</span>
                                                            </span>
                                                            <Show when=move || is_deep>
                                                                <span class="zd-history-tag zd-history-tag--deep">
                                                                    <Icon data=LuSparkles size=9 />
                                                                    <span>{t(lang, "Deep", "عمیق")}</span>
                                                                </span>
                                                            </Show>
                                                        </div>
                                                        <span class="zd-history-time" title=time_str.clone()>
                                                            <Icon data=LuClock size=10 />
                                                            <span>{time_str.clone()}</span>
                                                        </span>
                                                    </div>

                                                    <p class="zd-history-prompt-text">{p_text}</p>

                                                    <div class="zd-history-card-footer">
                                                        <div class="zd-history-actions-left">
                                                            <button
                                                                type="button"
                                                                class="zd-history-btn-open"
                                                                on:click=move |_| {
                                                                    open.set(false);
                                                                    activate_project(
                                                                        open_id.clone(),
                                                                        prompt,
                                                                        design_system,
                                                                        mode,
                                                                        deep,
                                                                        preview_version,
                                                                        entries,
                                                                        lang,
                                                                        Some(running),
                                                                    );
                                                                }
                                                                title=move || t(lang, "Load this project into the live preview canvas", "بارگذاری و مشاهده این پروژه در بوم پیش‌نمایش")
                                                            >
                                                                <Icon data=LuFolderOpen size=11 />
                                                                <span>{t(lang, "Open Project", "مشاهده پروژه")}</span>
                                                            </button>

                                                            <a
                                                                href=format!("/preview/sessions/{ext_id}/index.html")
                                                                target="_blank"
                                                                rel="noopener noreferrer"
                                                                class="zd-history-btn-ext"
                                                                title=move || t(lang, "Open artifact standalone in new tab", "مشاهده طرح در تب مستقل جدید")
                                                            >
                                                                <Icon data=LuExternalLink size=10 />
                                                            </a>

                                                            <button
                                                                type="button"
                                                                class="zd-history-btn-run"
                                                                disabled=move || running.get()
                                                                on:click=move |_| {
                                                                    prompt.set(run_prompt.clone());
                                                                    design_system.set(ds);
                                                                    mode.set(md);
                                                                    deep.set(is_deep);
                                                                    open.set(false);
                                                                    generate(
                                                                        prompt, running, design_system, mode, deep, entries,
                                                                        Some(history), None, None, None, None, None, None,
                                                                    );
                                                                }
                                                                title=move || t(lang, "Re-run this prompt immediately", "اجرای مجدد این پرامپت")
                                                            >
                                                                <Icon data=LuPlay size=10 />
                                                                <span>{t(lang, "Re-run", "اجرای مجدد")}</span>
                                                            </button>

                                                            <button
                                                                type="button"
                                                                class="zd-history-btn-load"
                                                                on:click=move |_| {
                                                                    prompt.set(load_prompt.clone());
                                                                    design_system.set(ds);
                                                                    mode.set(md);
                                                                    deep.set(is_deep);
                                                                    open.set(false);
                                                                }
                                                                title=move || t(lang, "Load into composer to edit", "بارگذاری در ویرایشگر برای بازنویسی")
                                                            >
                                                                <Icon data=LuCornerDownLeft size=10 />
                                                                <span>{t(lang, "Edit prompt", "ویرایش")}</span>
                                                            </button>
                                                        </div>

                                                        <button
                                                            type="button"
                                                            class="zd-history-btn-delete"
                                                            on:click=move |_| delete_history_item(item_id.clone(), history)
                                                            title=move || t(lang, "Delete project and its session files", "حذف پروژه و فایل‌های آن")
                                                        >
                                                            <Icon data=LuTrash2 size=11 />
                                                        </button>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()
                                    }}
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </Portal>
        </Show>
    }
}
