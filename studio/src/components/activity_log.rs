use icondata_core::IconData;
use icondata_lu::{
    LuActivity, LuBrain, LuCircleCheckBig, LuLoaderCircle, LuMessageSquare, LuPenLine, LuRefreshCw,
    LuSend, LuTrash2, LuTriangleAlert,
};
use leptos::prelude::*;

use crate::data::{ActivityEntry, ActivityKind, ConnectionState};
use crate::i18n::{t, Language};
use crate::icons::Icon;
use crate::util::format_clock;

struct EntryMeta {
    label: &'static str,
    icon: &'static IconData,
    class_name: &'static str,
}

fn meta(kind: ActivityKind, lang: Language) -> EntryMeta {
    match kind {
        ActivityKind::User => EntryMeta {
            label: t(lang, "Brief", "درخواست"),
            icon: &LuMessageSquare,
            class_name: "zd-act-icon--user",
        },
        ActivityKind::Queued => EntryMeta {
            label: t(lang, "Dispatched", "ارسال شد"),
            icon: &LuSend,
            class_name: "zd-act-icon--queued",
        },
        ActivityKind::Thinking => EntryMeta {
            label: t(lang, "Thinking", "در حال تفکر"),
            icon: &LuBrain,
            class_name: "zd-act-icon--thinking",
        },
        ActivityKind::Writing => EntryMeta {
            label: t(lang, "Writing", "در حال نگارش"),
            icon: &LuPenLine,
            class_name: "zd-act-icon--writing",
        },
        ActivityKind::Done => EntryMeta {
            label: t(lang, "Finished", "تکمیل شد"),
            icon: &LuCircleCheckBig,
            class_name: "zd-act-icon--done",
        },
        ActivityKind::Reload => EntryMeta {
            label: t(lang, "Reloaded", "بازخوانی شد"),
            icon: &LuRefreshCw,
            class_name: "zd-act-icon--reload",
        },
        ActivityKind::Error => EntryMeta {
            label: t(lang, "Error", "خطا"),
            icon: &LuTriangleAlert,
            class_name: "zd-act-icon--error",
        },
        ActivityKind::Status => EntryMeta {
            label: t(lang, "Update", "وضعیت"),
            icon: &LuActivity,
            class_name: "zd-act-icon--status",
        },
    }
}

#[component]
pub fn ActivityLog(
    language: RwSignal<Language>,
    entries: RwSignal<Vec<ActivityEntry>>,
    running: RwSignal<bool>,
    connection: RwSignal<ConnectionState>,
) -> impl IntoView {
    let list_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        let _ = entries.with(|items| items.len());
        if let Some(element) = list_ref.get() {
            element.set_scroll_top(element.scroll_height());
        }
    });

    view! {
        <div class="zd-activity-log">
            <div class="zd-activity-header">
                <div class="zd-activity-heading">
                    <Icon data=LuActivity size=13 class="zd-activity-header-icon" />
                    <span class="zd-activity-title">
                        {move || t(language.get(), "Activity", "فعالیت")}
                    </span>
                    <Show when=move || running.get()>
                        <span class="zd-activity-badge zd-activity-badge--running">
                            <Icon data=LuLoaderCircle size=10 class="zd-spin" />
                            {move || t(language.get(), "running", "در حال اجرا")}
                        </span>
                    </Show>
                    <Show when=move || {
                        !running.get() && connection.get() == ConnectionState::Stalled
                    }>
                        <span class="zd-activity-badge zd-activity-badge--stalled">
                            {move || t(language.get(), "heartbeat lost", "قطع ضربان")}
                        </span>
                    </Show>
                </div>
                <button
                    type="button"
                    on:click=move |_| entries.set(Vec::new())
                    disabled=move || entries.with(|items| items.is_empty())
                    title=move || t(language.get(), "Clear activity", "پاکسازی فعالیت‌ها")
                    class="zd-activity-clear-btn"
                >
                    <Icon data=LuTrash2 size=11 />
                    <span>{move || t(language.get(), "Clear", "پاکسازی")}</span>
                </button>
            </div>

            <div
                node_ref=list_ref
                class="zd-activity-list"
            >
                <Show
                    when=move || entries.with(|items| items.is_empty())
                    fallback=move || view! {
                        <For
                            each=move || entries.get()
                            key=|entry| entry.id
                            children=move |entry| {
                                let lang = language.get();
                                let entry_meta = meta(entry.kind, lang);
                                let is_user = entry.kind == ActivityKind::User;
                                view! {
                                    <div class="zd-activity-item">
                                        <div class=format!("zd-act-icon-box {}", entry_meta.class_name)>
                                            <Icon
                                                data=entry_meta.icon
                                                size=11
                                            />
                                        </div>
                                        <div class="zd-activity-content">
                                            <div class="zd-activity-row">
                                                <span class="zd-activity-kind-label">
                                                    {entry_meta.label}
                                                </span>
                                                <span class="zd-activity-timestamp">
                                                    {format_clock(entry.at, language.get())}
                                                </span>
                                            </div>
                                            <p class=if is_user {
                                                "zd-activity-message zd-activity-message--user"
                                            } else {
                                                "zd-activity-message"
                                            }>
                                                {entry.message}
                                            </p>
                                        </div>
                                    </div>
                                }
                            }
                        />
                    }
                >
                    <div class="zd-activity-empty">
                        <p class="zd-activity-empty-text">
                            {move || t(language.get(), "No activity yet. Start with a prompt or preset.", "هنوز فعالیتی ثبت نشده است. با یک درخواست یا پیش‌فرض شروع کنید.")}
                        </p>
                    </div>
                </Show>
            </div>
        </div>
    }
}
