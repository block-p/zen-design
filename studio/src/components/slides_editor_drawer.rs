use gloo_net::http::Request;
use icondata_lu::{
    LuArrowDown, LuArrowUp, LuCheck, LuCode, LuColumns2, LuCopy, LuEye, LuLayoutGrid,
    LuLoaderCircle, LuMic, LuPenLine, LuPlus, LuSave, LuSparkles, LuTrash2, LuWand, LuX,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::json;
use wasm_bindgen::JsCast;

use crate::data::{ActivityEntry, ActivityKind};
use crate::i18n::{t, Language};
use crate::icons::Icon;
use crate::stream::push_activity;

#[derive(Clone, Debug, PartialEq)]
pub struct SlideCardItem {
    pub index: usize,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SlideItem {
    pub id: String,
    pub index: usize,
    pub badge: String,
    pub title: String,
    pub subtitle: String,
    pub layout: String,
    pub speaker_notes: String,
    pub items: Vec<SlideCardItem>,
}

fn parse_doc(html: &str) -> Option<web_sys::Document> {
    let parser = web_sys::DomParser::new().ok()?;
    parser.parse_from_string(html, web_sys::SupportedType::TextHtml).ok()
}

fn serialize_doc(doc: &web_sys::Document) -> String {
    if let Some(doc_elem) = doc.document_element() {
        format!("<!DOCTYPE html>\n{}", doc_elem.outer_html())
    } else {
        String::new()
    }
}

fn extract_slides(html: &str) -> Vec<SlideItem> {
    let Some(doc) = parse_doc(html) else { return Vec::new(); };
    let Ok(nodes) = doc.query_selector_all(".slide, section[id^='s'], [data-slide-id]") else {
        return Vec::new();
    };
    let mut slides = Vec::new();
    let len = nodes.length();
    for i in 0..len {
        let Some(node) = nodes.item(i) else { continue; };
        let Ok(el) = node.dyn_into::<web_sys::Element>() else { continue; };
        let id = el.id();
        let slide_id = if id.is_empty() {
            el.get_attribute("data-slide-id").unwrap_or_else(|| format!("s{}", i + 1))
        } else {
            id
        };

        let layout = el
            .get_attribute("data-layout")
            .unwrap_or_else(|| "bento".to_string());

        let speaker_notes = el
            .get_attribute("data-speaker-notes")
            .or_else(|| {
                el.query_selector(".speaker-notes, aside[class*='notes']")
                    .ok()
                    .flatten()
                    .and_then(|e| e.text_content())
            })
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let badge = el
            .query_selector(".badge, .category-badge, [class*='badge']")
            .ok()
            .flatten()
            .and_then(|e| e.text_content())
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let title = el
            .query_selector("h1, h2, h3, .display-title, [class*='title']")
            .ok()
            .flatten()
            .and_then(|e| e.text_content())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| format!("Slide {}", i + 1));

        let subtitle = el
            .query_selector(".sub, .body-text, p")
            .ok()
            .flatten()
            .and_then(|e| e.text_content())
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let mut items = Vec::new();
        if let Ok(card_nodes) = el.query_selector_all(".card, .node, .point, .item, li, div[class*='card']") {
            let card_len = card_nodes.length();
            for j in 0..card_len {
                if let Some(card_node) = card_nodes.item(j) {
                    if let Ok(card_el) = card_node.dyn_into::<web_sys::Element>() {
                        let txt = card_el.text_content().unwrap_or_default().trim().to_string();
                        if !txt.is_empty() && txt.len() < 300 {
                            items.push(SlideCardItem {
                                index: j as usize,
                                text: txt,
                            });
                        }
                    }
                }
            }
        }

        slides.push(SlideItem {
            id: slide_id,
            index: i as usize,
            badge,
            title,
            subtitle,
            layout,
            speaker_notes,
            items,
        });
    }
    slides
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
    let ai_instruction = RwSignal::new(String::new());
    let is_ai_loading = RwSignal::new(false);

    // Derived signal: parsed slides
    let slides = Memo::new(move |_| extract_slides(&raw_html.get()));

    // Jump the live preview iframe to a specific slide index
    let jump_preview = move |idx: usize| {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Ok(Some(frame)) = doc.query_selector("iframe") {
                if let Ok(iframe) = frame.dyn_into::<web_sys::HtmlIFrameElement>() {
                    if let Some(fdoc) = iframe.content_document() {
                        let code = format!(
                            "(function() {{ if (typeof window.__zd_go === 'function') {{ window.__zd_go({}); }} else if (typeof go === 'function') {{ go({}); }} }})();",
                            idx, idx
                        );
                        if let Ok(s) = fdoc.create_element("script") {
                            s.set_text_content(Some(&code));
                            if let Some(b) = fdoc.body() {
                                let _ = b.append_child(&s);
                                let _ = s.remove();
                            }
                        }
                    }
                }
            }
        }
    };

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

    let update_speaker_notes = move |idx: usize, val: String| {
        let current = raw_html.get();
        if let Some(doc) = parse_doc(&current) {
            if let Ok(nodes) = doc.query_selector_all(".slide, section[id^='s'], [data-slide-id]") {
                if let Some(node) = nodes.item(idx as u32) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        let _ = el.set_attribute("data-speaker-notes", &val);
                        raw_html.set(serialize_doc(&doc));
                    }
                }
            }
        }
    };

    let run_ai_action = move |action_type: &'static str| {
        if is_ai_loading.get() {
            return;
        }
        is_ai_loading.set(true);
        let cur_idx = selected_slide_idx.get();
        let current_slides = slides.get();
        let maybe_slide = current_slides.get(cur_idx).cloned();

        let (title, badge, subtitle, items_str, layout_str) = if let Some(s) = maybe_slide {
            (
                s.title,
                s.badge,
                s.subtitle,
                s.items.into_iter().map(|it| it.text).collect::<Vec<_>>(),
                s.layout,
            )
        } else {
            (
                String::new(),
                String::new(),
                String::new(),
                Vec::new(),
                "bento".to_string(),
            )
        };

        let user_instruction = ai_instruction.get();
        let lang = language.get();

        spawn_local(async move {
            let body = json!({
                "action": action_type,
                "instruction": user_instruction,
                "title": title,
                "badge": badge,
                "subtitle": subtitle,
                "items": items_str,
                "target_layout": layout_str,
            });

            match Request::post("/api/slides/ai-action")
                .header("Content-Type", "application/json")
                .body(body.to_string())
            {
                Ok(req) => match req.send().await {
                    Ok(resp) if resp.ok() => {
                        if let Ok(data) = resp.json::<serde_json::Value>().await {
                            let html_snippet = data
                                .get("html_snippet")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default();
                            let notes_out = data
                                .get("speaker_notes")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default();

                            if action_type == "add_slide" {
                                let current_html = raw_html.get();
                                let updated = if let Some(pos) = current_html.find("</main>") {
                                    let mut s = current_html.clone();
                                    s.insert_str(pos, &format!("\n{html_snippet}\n"));
                                    s
                                } else if let Some(pos) = current_html.find("</body>") {
                                    let mut s = current_html.clone();
                                    s.insert_str(pos, &format!("\n{html_snippet}\n"));
                                    s
                                } else {
                                    format!("{}\n{}", current_html, html_snippet)
                                };
                                raw_html.set(updated);
                                let new_total = slides.get().len();
                                selected_slide_idx.set(new_total.saturating_sub(1));
                                jump_preview(new_total.saturating_sub(1));
                                push_activity(
                                    entries,
                                    ActivityKind::Done,
                                    t(
                                        lang,
                                        "New AI slide generated and added to deck",
                                        "اسلاید جدید با هوش مصنوعی ایجاد و اضافه شد",
                                    ),
                                );
                            } else {
                                let current_html = raw_html.get();
                                if let Some(doc) = parse_doc(&current_html) {
                                    if let Ok(nodes) = doc.query_selector_all(
                                        ".slide, section[id^='s'], [data-slide-id]",
                                    ) {
                                        if let Some(node) = nodes.item(cur_idx as u32) {
                                            if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                                                if !html_snippet.is_empty() {
                                                    if let Some(temp_doc) = parse_doc(&format!(
                                                        "<div>{html_snippet}</div>"
                                                    )) {
                                                        if let Ok(Some(new_el)) =
                                                            temp_doc.query_selector(".slide, section")
                                                        {
                                                            if let Some(parent) = el.parent_node() {
                                                                if let Ok(imported) =
                                                                    doc.import_node_with_deep(&new_el, true)
                                                                {
                                                                    let _ = parent
                                                                        .replace_child(&imported, &el);
                                                                    raw_html.set(serialize_doc(&doc));
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    let _ = el.set_attribute(
                                                        "data-speaker-notes",
                                                        notes_out,
                                                    );
                                                    raw_html.set(serialize_doc(&doc));
                                                }
                                            }
                                        }
                                    }
                                }
                                jump_preview(cur_idx);
                                push_activity(
                                    entries,
                                    ActivityKind::Done,
                                    t(
                                        lang,
                                        "AI slide modification completed",
                                        "تغییرات هوشمند اسلاید با موفقیت اعمال شد",
                                    ),
                                );
                            }
                        }
                        is_ai_loading.set(false);
                        ai_instruction.set(String::new());
                    }
                    _ => {
                        is_ai_loading.set(false);
                        push_activity(
                            entries,
                            ActivityKind::Error,
                            t(
                                lang,
                                "AI action request failed",
                                "خطا در برقراری ارتباط با هوش مصنوعی",
                            ),
                        );
                    }
                },
                Err(_) => {
                    is_ai_loading.set(false);
                }
            }
        });
    };

    let apply_layout_to_slide = move |cur_idx: usize, target_layout: &'static str| {
        if is_ai_loading.get() {
            return;
        }
        is_ai_loading.set(true);
        let current_slides = slides.get();
        let Some(slide) = current_slides.get(cur_idx).cloned() else {
            is_ai_loading.set(false);
            return;
        };
        let items_vec: Vec<String> = slide.items.into_iter().map(|it| it.text).collect();
        let lang = language.get();

        spawn_local(async move {
            let body = json!({
                "action": "transform_layout",
                "title": slide.title,
                "badge": slide.badge,
                "subtitle": slide.subtitle,
                "items": items_vec,
                "target_layout": target_layout,
            });

            match Request::post("/api/slides/ai-action")
                .header("Content-Type", "application/json")
                .body(body.to_string())
            {
                Ok(req) => match req.send().await {
                    Ok(resp) if resp.ok() => {
                        if let Ok(data) = resp.json::<serde_json::Value>().await {
                            if let Some(html_snippet) =
                                data.get("html_snippet").and_then(|v| v.as_str())
                            {
                                let current_html = raw_html.get();
                                if let Some(doc) = parse_doc(&current_html) {
                                    if let Ok(nodes) = doc.query_selector_all(
                                        ".slide, section[id^='s'], [data-slide-id]",
                                    ) {
                                        if let Some(node) = nodes.item(cur_idx as u32) {
                                            if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                                                if let Some(temp_doc) = parse_doc(&format!(
                                                    "<div>{html_snippet}</div>"
                                                )) {
                                                    if let Ok(Some(new_el)) =
                                                        temp_doc.query_selector(".slide, section")
                                                    {
                                                        if let Some(parent) = el.parent_node() {
                                                            if let Ok(imported) =
                                                                doc.import_node_with_deep(&new_el, true)
                                                            {
                                                                let _ = parent
                                                                    .replace_child(&imported, &el);
                                                                raw_html.set(serialize_doc(&doc));
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                jump_preview(cur_idx);
                                push_activity(
                                    entries,
                                    ActivityKind::Done,
                                    t(
                                        lang,
                                        "Layout applied to slide",
                                        "طرح‌بندی جدید روی اسلاید اعمال گردید",
                                    ),
                                );
                            }
                        }
                        is_ai_loading.set(false);
                    }
                    _ => {
                        is_ai_loading.set(false);
                    }
                },
                Err(_) => {
                    is_ai_loading.set(false);
                }
            }
        });
    };

    // Slide Mutations
    let update_title = move |idx: usize, val: String| {
        let current = raw_html.get();
        if let Some(doc) = parse_doc(&current) {
            if let Ok(nodes) = doc.query_selector_all(".slide, section[id^='s'], [data-slide-id]") {
                if let Some(node) = nodes.item(idx as u32) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        if let Ok(Some(title_el)) = el.query_selector("h1, h2, h3, .display-title, [class*='title']") {
                            title_el.set_text_content(Some(&val));
                            raw_html.set(serialize_doc(&doc));
                        }
                    }
                }
            }
        }
    };

    let update_badge = move |idx: usize, val: String| {
        let current = raw_html.get();
        if let Some(doc) = parse_doc(&current) {
            if let Ok(nodes) = doc.query_selector_all(".slide, section[id^='s'], [data-slide-id]") {
                if let Some(node) = nodes.item(idx as u32) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        if let Ok(Some(badge_el)) = el.query_selector(".badge, .category-badge, [class*='badge']") {
                            badge_el.set_text_content(Some(&val));
                            raw_html.set(serialize_doc(&doc));
                        }
                    }
                }
            }
        }
    };

    let update_subtitle = move |idx: usize, val: String| {
        let current = raw_html.get();
        if let Some(doc) = parse_doc(&current) {
            if let Ok(nodes) = doc.query_selector_all(".slide, section[id^='s'], [data-slide-id]") {
                if let Some(node) = nodes.item(idx as u32) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        if let Ok(Some(sub_el)) = el.query_selector(".sub, .body-text, p") {
                            sub_el.set_text_content(Some(&val));
                            raw_html.set(serialize_doc(&doc));
                        }
                    }
                }
            }
        }
    };

    let update_item_text = move |slide_idx: usize, item_idx: usize, val: String| {
        let current = raw_html.get();
        if let Some(doc) = parse_doc(&current) {
            if let Ok(nodes) = doc.query_selector_all(".slide, section[id^='s'], [data-slide-id]") {
                if let Some(node) = nodes.item(slide_idx as u32) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        if let Ok(card_nodes) = el.query_selector_all(".card, .node, .point, .item, li, div[class*='card']") {
                            if let Some(card_node) = card_nodes.item(item_idx as u32) {
                                if let Ok(card_el) = card_node.dyn_into::<web_sys::Element>() {
                                    card_el.set_text_content(Some(&val));
                                    raw_html.set(serialize_doc(&doc));
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    let delete_item = move |slide_idx: usize, item_idx: usize| {
        let current = raw_html.get();
        if let Some(doc) = parse_doc(&current) {
            if let Ok(nodes) = doc.query_selector_all(".slide, section[id^='s'], [data-slide-id]") {
                if let Some(node) = nodes.item(slide_idx as u32) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        if let Ok(card_nodes) = el.query_selector_all(".card, .node, .point, .item, li, div[class*='card']") {
                            if let Some(card_node) = card_nodes.item(item_idx as u32) {
                                if let Ok(card_el) = card_node.dyn_into::<web_sys::Element>() {
                                    card_el.remove();
                                    raw_html.set(serialize_doc(&doc));
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    let add_item = move |slide_idx: usize| {
        let current = raw_html.get();
        if let Some(doc) = parse_doc(&current) {
            if let Ok(nodes) = doc.query_selector_all(".slide, section[id^='s'], [data-slide-id]") {
                if let Some(node) = nodes.item(slide_idx as u32) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        // Find container or fallback to slide
                        let target_container = el.query_selector(".cards-grid, .grid, .slide-content, .cards")
                            .ok()
                            .flatten()
                            .unwrap_or_else(|| el.clone());

                        if let Ok(new_card) = doc.create_element("div") {
                            let _ = new_card.set_attribute("class", "card");
                            new_card.set_inner_html("<p>New bullet item or card description</p>");
                            let _ = target_container.append_child(&new_card);
                            raw_html.set(serialize_doc(&doc));
                        }
                    }
                }
            }
        }
    };

    let delete_slide = move |idx: usize| {
        let current = raw_html.get();
        if let Some(doc) = parse_doc(&current) {
            if let Ok(nodes) = doc.query_selector_all(".slide, section[id^='s'], [data-slide-id]") {
                let total = nodes.length();
                if total > 1 {
                    if let Some(node) = nodes.item(idx as u32) {
                        if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                            el.remove();
                            raw_html.set(serialize_doc(&doc));
                            let next_idx = idx.saturating_sub(1);
                            selected_slide_idx.set(next_idx);
                            jump_preview(next_idx);
                        }
                    }
                }
            }
        }
    };

    let duplicate_slide = move |idx: usize| {
        let current = raw_html.get();
        if let Some(doc) = parse_doc(&current) {
            if let Ok(nodes) = doc.query_selector_all(".slide, section[id^='s'], [data-slide-id]") {
                if let Some(node) = nodes.item(idx as u32) {
                    if let Ok(cloned) = node.clone_node_with_deep(true) {
                        if let Ok(cloned_el) = cloned.dyn_into::<web_sys::Element>() {
                            let old_id = cloned_el.id();
                            cloned_el.set_id(&format!("{}_copy", if old_id.is_empty() { "s" } else { &old_id }));
                            if let Some(parent) = node.parent_node() {
                                let next_sibling = node.next_sibling();
                                let _ = parent.insert_before(&cloned_el, next_sibling.as_ref());
                                raw_html.set(serialize_doc(&doc));
                                selected_slide_idx.set(idx + 1);
                                jump_preview(idx + 1);
                            }
                        }
                    }
                }
            }
        }
    };

    let move_slide = move |idx: usize, dir: i32| {
        let current = raw_html.get();
        if let Some(doc) = parse_doc(&current) {
            if let Ok(nodes) = doc.query_selector_all(".slide, section[id^='s'], [data-slide-id]") {
                let total = nodes.length() as i32;
                let target = idx as i32 + dir;
                if target >= 0 && target < total {
                    let Some(node_curr) = nodes.item(idx as u32) else { return; };
                    let Some(node_targ) = nodes.item(target as u32) else { return; };
                    if let Some(parent) = node_curr.parent_node() {
                        if dir < 0 {
                            let _ = parent.insert_before(&node_curr, Some(&node_targ));
                        } else {
                            let targ_next = node_targ.next_sibling();
                            let _ = parent.insert_before(&node_curr, targ_next.as_ref());
                        }
                        raw_html.set(serialize_doc(&doc));
                        selected_slide_idx.set(target as usize);
                        jump_preview(target as usize);
                    }
                }
            }
        }
    };

    let add_new_slide = move || {
        let current = raw_html.get();
        let new_slide_markup = r#"
    <!-- New Slide -->
    <section class="slide" data-slide-id="new_slide">
        <div class="slide-content">
            <span class="category-badge">NEW TOPIC</span>
            <h2 class="display-title">New Slide Title</h2>
            <p class="body-text">Add your detailed slide notes and bullet points here.</p>
            <div class="cards-grid">
                <div class="card"><p>Key takeaway item 1</p></div>
                <div class="card"><p>Key takeaway item 2</p></div>
            </div>
        </div>
    </section>
"#;
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
        let count = slides.get().len();
        selected_slide_idx.set(count);
        jump_preview(count);
    };

    view! {
        <Show when=move || open.get()>
            <div class="zd-slide-drawer-backdrop" on:click=move |_| open.set(false)></div>
            <aside class="zd-slide-drawer">
                <div class="zd-slide-drawer-header">
                    <div class="zd-slide-drawer-title-row">
                        <div class="zd-slide-drawer-icon">
                            <Icon data=LuPenLine size=16 />
                        </div>
                        <div>
                            <h2 class="zd-slide-drawer-title">
                                {move || t(language.get(), "Slide Deck Studio", "استودیوی مدیریت اسلایدها")}
                            </h2>
                            <p class="zd-slide-drawer-subtitle">
                                {move || t(language.get(), "Deep editing for titles, content, cards & sequence", "ویرایش عمیق تیترها، متون، کارت‌ها و ترتیب اسلایدها")}
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
                                    <Icon data=LuSave size=13 />
                                    <span>{move || t(language.get(), "Save Deck", "ذخیره اسلایدها")}</span>
                                }
                            >
                                <Icon data=LuCheck size=13 />
                                <span>{move || t(language.get(), "Saving…", "در حال ذخیره…")}</span>
                            </Show>
                        </button>
                        <button
                            type="button"
                            class="zd-btn-icon"
                            on:click=move |_| open.set(false)
                            title=move || t(language.get(), "Close", "بستن")
                        >
                            <Icon data=LuX size=15 />
                        </button>
                    </div>
                </div>

                <div class="zd-slide-drawer-tabs">
                    <button
                        type="button"
                        class=move || if active_tab.get() == "cards" { "zd-slide-tab zd-slide-tab--active" } else { "zd-slide-tab" }
                        on:click=move |_| active_tab.set("cards")
                    >
                        <Icon data=LuColumns2 size=13 />
                        <span>{move || t(language.get(), "Visual Slide Studio", "استودیوی بصری اسلایدها")}</span>
                    </button>
                    <button
                        type="button"
                        class=move || if active_tab.get() == "code" { "zd-slide-tab zd-slide-tab--active" } else { "zd-slide-tab" }
                        on:click=move |_| active_tab.set("code")
                    >
                        <Icon data=LuCode size=13 />
                        <span>{move || t(language.get(), "Full Source / HTML", "کد منبع HTML")}</span>
                    </button>
                </div>

                <div class="zd-slide-drawer-body">
                    <Show
                        when=move || active_tab.get() == "cards"
                        fallback=move || view! {
                            <div class="zd-slide-code-panel">
                                <div class="zd-slide-code-hint">
                                    <Icon data=LuCode size=13 />
                                    <span>{move || t(language.get(), "Direct HTML edits apply instantly to your workspace index.html", "ویرایش مستقیم کد HTML بی‌درنگ ذخیره می‌شود")}</span>
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
                        <div class="zd-slide-studio-container">
                            <!-- Left: Slides Navigator -->
                            <div class="zd-slide-studio-sidebar">
                                <div class="zd-slide-sidebar-header">
                                    <span class="zd-slide-count-badge">
                                        {move || format!("{} {}", slides.get().len(), t(language.get(), "Slides", "اسلاید"))}
                                    </span>
                                    <button
                                        type="button"
                                        class="zd-btn-soft zd-btn-add-slide-mini"
                                        on:click=move |_| add_new_slide()
                                        title=move || t(language.get(), "Add new slide", "افزودن اسلاید جدید")
                                    >
                                        <Icon data=LuPlus size=12 />
                                        <span>{move || t(language.get(), "Add", "افزودن")}</span>
                                    </button>
                                </div>

                                <div class="zd-slide-deck-list">
                                    {move || {
                                        let current_list = slides.get();
                                        let cur_sel = selected_slide_idx.get();
                                        current_list.into_iter().enumerate().map(|(idx, slide)| {
                                            let is_selected = idx == cur_sel;
                                            let badge_txt = slide.badge.clone();
                                            let badge_show = badge_txt.clone();
                                            let title_txt = slide.title.clone();
                                            let count_txt = format!("{} items", slide.items.len());

                                            view! {
                                                <div
                                                    class=if is_selected {
                                                        "zd-slide-nav-item zd-slide-nav-item--active"
                                                    } else {
                                                        "zd-slide-nav-item"
                                                    }
                                                    on:click=move |_| {
                                                        selected_slide_idx.set(idx);
                                                        jump_preview(idx);
                                                    }
                                                >
                                                    <div class="zd-slide-nav-item-top">
                                                        <span class="zd-slide-num">{format!("{:02}", idx + 1)}</span>
                                                        <Show when=move || !badge_show.is_empty()>
                                                            <span class="zd-slide-mini-badge">{badge_txt.clone()}</span>
                                                        </Show>
                                                        <div class="zd-slide-item-actions">
                                                            <button
                                                                type="button"
                                                                class="zd-btn-icon-mini"
                                                                title="Move Up"
                                                                disabled=idx == 0
                                                                on:click=move |e| {
                                                                    e.stop_propagation();
                                                                    move_slide(idx, -1);
                                                                }
                                                            >
                                                                <Icon data=LuArrowUp size=11 />
                                                            </button>
                                                            <button
                                                                type="button"
                                                                class="zd-btn-icon-mini"
                                                                title="Move Down"
                                                                on:click=move |e| {
                                                                    e.stop_propagation();
                                                                    move_slide(idx, 1);
                                                                }
                                                            >
                                                                <Icon data=LuArrowDown size=11 />
                                                            </button>
                                                        </div>
                                                    </div>
                                                    <div class="zd-slide-nav-item-title">
                                                        {title_txt}
                                                    </div>
                                                    <div class="zd-slide-nav-item-footer">
                                                        <span class="zd-slide-items-count">{count_txt}</span>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()
                                    }}
                                </div>
                            </div>

                            <!-- Right: Deep Slide Editor -->
                            <div class="zd-slide-studio-editor">
                                {move || {
                                    let cur_idx = selected_slide_idx.get();
                                    let all_slides = slides.get();
                                    if let Some(slide) = all_slides.get(cur_idx) {
                                        let current_slide = slide.clone();
                                        view! {
                                            <div class="zd-slide-editor-inner">
                                                <!-- Slide Editor Header & Actions -->
                                                <div class="zd-slide-editor-header">
                                                    <div class="zd-slide-editor-heading">
                                                        <span class="zd-slide-editor-tag">{format!("Slide {:02}", cur_idx + 1)}</span>
                                                        <h3 class="zd-slide-editor-cur-title">{current_slide.title.clone()}</h3>
                                                    </div>
                                                    <div class="zd-slide-editor-btn-row">
                                                        <button
                                                            type="button"
                                                            class="zd-btn-soft zd-btn-preview-slide"
                                                            on:click=move |_| jump_preview(cur_idx)
                                                            title=move || t(language.get(), "Jump preview to this slide", "انتقال پیش‌نمایش به این اسلاید")
                                                        >
                                                            <Icon data=LuEye size=12 />
                                                            <span>{move || t(language.get(), "Preview", "نمایش")}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="zd-btn-soft"
                                                            on:click=move |_| duplicate_slide(cur_idx)
                                                            title=move || t(language.get(), "Duplicate this slide", "تکثیر این اسلاید")
                                                        >
                                                            <Icon data=LuCopy size=12 />
                                                            <span>{move || t(language.get(), "Duplicate", "تکثیر")}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="zd-btn-soft zd-btn-delete-slide"
                                                            disabled=all_slides.len() <= 1
                                                            on:click=move |_| delete_slide(cur_idx)
                                                            title=move || t(language.get(), "Delete this slide", "حذف این اسلاید")
                                                        >
                                                            <Icon data=LuTrash2 size=12 />
                                                            <span>{move || t(language.get(), "Delete", "حذف")}</span>
                                                        </button>
                                                    </div>
                                                </div>

                                                <!-- AI Slide Copilot -->
                                                <div class="zd-slide-ai-card">
                                                    <div class="zd-slide-ai-header">
                                                        <div class="zd-slide-ai-title-row">
                                                            <Icon data=LuSparkles size=14 />
                                                            <span class="zd-slide-ai-title">
                                                                {move || t(language.get(), "ZenSlides AI Copilot", "دستیار هوش مصنوعی اسلاید")}
                                                            </span>
                                                        </div>
                                                        <Show when=move || is_ai_loading.get()>
                                                            <div class="zd-slide-ai-spin">
                                                                <Icon data=LuLoaderCircle size=12 />
                                                                <span>{move || t(language.get(), "Processing…", "در حال پردازش…")}</span>
                                                            </div>
                                                        </Show>
                                                    </div>
                                                    <div class="zd-slide-ai-prompt-row">
                                                        <input
                                                            type="text"
                                                            class="zd-input zd-slide-ai-input"
                                                            placeholder=move || t(
                                                                language.get(),
                                                                "Ask AI: rewrite, summarize, turn into KPIs, or create next slide...",
                                                                "دستور به هوش مصنوعی: بازنویسی، خلاصه‌سازی، تبدیل به شاخص عددی...",
                                                            )
                                                            prop:value=move || ai_instruction.get()
                                                            on:input=move |ev| ai_instruction.set(event_target_value(&ev))
                                                            on:keydown=move |ev: web_sys::KeyboardEvent| {
                                                                if ev.key() == "Enter" {
                                                                    run_ai_action("rewrite_slide");
                                                                }
                                                            }
                                                        />
                                                        <button
                                                            type="button"
                                                            class="zd-btn-primary zd-btn-ai-run"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| run_ai_action("rewrite_slide")
                                                        >
                                                            <Icon data=LuWand size=12 />
                                                            <span>{move || t(language.get(), "Generate", "اجرا")}</span>
                                                        </button>
                                                    </div>
                                                    <div class="zd-slide-ai-chips">
                                                        <button
                                                            type="button"
                                                            class="zd-slide-chip"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| run_ai_action("rewrite_slide")
                                                            title="Rewrite and polish slide text"
                                                        >
                                                            <span>{"✍️ "}{move || t(language.get(), "Rewrite & Polish", "بازنویسی حرفه‌ای")}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="zd-slide-chip"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| run_ai_action("summarize")
                                                            title="Summarize into concise key points"
                                                        >
                                                            <span>{"⚡ "}{move || t(language.get(), "Summarize", "خلاصه‌سازی نکات")}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="zd-slide-chip"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| run_ai_action("kpi_stats")
                                                            title="Convert bullet points into bold KPI numbers"
                                                        >
                                                            <span>{"📊 "}{move || t(language.get(), "Turn to KPIs", "تبدیل به آمار و ارقام")}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="zd-slide-chip"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| run_ai_action("speaker_notes")
                                                            title="Generate speech talking points"
                                                        >
                                                            <span>{"🎤 "}{move || t(language.get(), "Speaker Notes", "تولید یادداشت ارائه")}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="zd-slide-chip zd-slide-chip--accent"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| run_ai_action("add_slide")
                                                            title="Generate brand new slide on current topic"
                                                        >
                                                            <span>{"➕ "}{move || t(language.get(), "New AI Slide", "اسلاید جدید با AI")}</span>
                                                        </button>
                                                    </div>
                                                </div>

                                                <!-- Visual Layout Switcher -->
                                                <div class="zd-slide-layout-bar">
                                                    <div class="zd-slide-layout-label">
                                                        <Icon data=LuLayoutGrid size=13 />
                                                        <span>{move || t(language.get(), "Slide Layout Template", "طرح‌بندی و ساختار بصری اسلاید")}</span>
                                                    </div>
                                                    <div class="zd-slide-layout-options">
                                                        <button
                                                            type="button"
                                                            class="zd-layout-btn"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| apply_layout_to_slide(cur_idx, "bento")
                                                            title="Bento Grid Layout"
                                                        >
                                                            <span>{"🍱 "}{move || t(language.get(), "Bento Grid", "بنتو گرید")}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="zd-layout-btn"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| apply_layout_to_slide(cur_idx, "stats")
                                                            title="KPI Big Numbers"
                                                        >
                                                            <span>{"📊 "}{move || t(language.get(), "KPI Stats", "شاخص‌های کلیدی")}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="zd-layout-btn"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| apply_layout_to_slide(cur_idx, "split")
                                                            title="2-Column Split"
                                                        >
                                                            <span>{"🌓 "}{move || t(language.get(), "Split 2-Col", "دو ستونه")}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="zd-layout-btn"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| apply_layout_to_slide(cur_idx, "timeline")
                                                            title="Timeline Steps"
                                                        >
                                                            <span>{"⏳ "}{move || t(language.get(), "Timeline Steps", "گام‌های زمانی")}</span>
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="zd-layout-btn"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| apply_layout_to_slide(cur_idx, "comparison")
                                                            title="Comparison Matrix"
                                                        >
                                                            <span>{"⚖️ "}{move || t(language.get(), "Comparison", "مقایسه")}</span>
                                                        </button>
                                                    </div>
                                                </div>

                                                <!-- Slide Meta Inputs -->
                                                <div class="zd-slide-form-grid">
                                                    <div class="zd-form-field">
                                                        <label class="zd-form-label">
                                                            {move || t(language.get(), "Slide Title", "عنوان اسلاید")}
                                                        </label>
                                                        <input
                                                            type="text"
                                                            class="zd-input zd-slide-title-input"
                                                            prop:value=current_slide.title.clone()
                                                            on:change=move |ev| update_title(cur_idx, event_target_value(&ev))
                                                        />
                                                    </div>

                                                    <div class="zd-form-field">
                                                        <label class="zd-form-label">
                                                            {move || t(language.get(), "Badge / Category Tag", "برچسب موضوعی")}
                                                        </label>
                                                        <input
                                                            type="text"
                                                            class="zd-input"
                                                            prop:value=current_slide.badge.clone()
                                                            on:change=move |ev| update_badge(cur_idx, event_target_value(&ev))
                                                        />
                                                    </div>

                                                    <div class="zd-form-field zd-form-field--full">
                                                        <label class="zd-form-label">
                                                            {move || t(language.get(), "Subtitle / Body Paragraph", "توضیح یا زیرعنوان اسلاید")}
                                                        </label>
                                                        <textarea
                                                            class="zd-textarea zd-slide-sub-input"
                                                            prop:value=current_slide.subtitle.clone()
                                                            on:change=move |ev| update_subtitle(cur_idx, event_target_value(&ev))
                                                            rows=2
                                                        ></textarea>
                                                    </div>
                                                </div>

                                                <!-- Speaker Notes / Presentation Script -->
                                                <div class="zd-slide-speaker-notes-box">
                                                    <div class="zd-slide-notes-header">
                                                        <div class="zd-slide-notes-title-row">
                                                            <Icon data=LuMic size=13 />
                                                            <span class="zd-slide-notes-title">
                                                                {move || t(language.get(), "Speaker Notes & Script", "یادداشت‌های ارائه‌دهنده و متن سخنرانی")}
                                                            </span>
                                                        </div>
                                                        <button
                                                            type="button"
                                                            class="zd-btn-soft zd-btn-gen-notes-mini"
                                                            disabled=move || is_ai_loading.get()
                                                            on:click=move |_| run_ai_action("speaker_notes")
                                                            title=move || t(language.get(), "Auto-generate talking points with AI", "تولید خودکار متن سخنرانی با هوش مصنوعی")
                                                        >
                                                            <Icon data=LuSparkles size=11 />
                                                            <span>{move || t(language.get(), "AI Script", "متن هوشمند")}</span>
                                                        </button>
                                                    </div>
                                                    <p class="zd-slide-notes-desc">
                                                        {move || t(
                                                            language.get(),
                                                            "Private cues displayed only on presenter view during presentation",
                                                            "یادداشت‌های اختصاصی که فقط در حالت ارائه (Presenter Mode) برای شما نمایش داده می‌شود",
                                                        )}
                                                    </p>
                                                    <textarea
                                                        class="zd-textarea zd-slide-notes-textarea"
                                                        placeholder=move || t(
                                                            language.get(),
                                                            "Type speaker notes or click 'AI Script' to draft talking points...",
                                                            "متن یا نکات سخنرانی این اسلاید را اینجا یادداشت کنید...",
                                                        )
                                                        prop:value=current_slide.speaker_notes.clone()
                                                        on:change=move |ev| update_speaker_notes(cur_idx, event_target_value(&ev))
                                                        rows=3
                                                    ></textarea>
                                                </div>

                                                <!-- Slide Content Items / Cards -->
                                                <div class="zd-slide-items-box">
                                                    <div class="zd-slide-items-header">
                                                        <div>
                                                            <h4 class="zd-slide-items-title">
                                                                {move || t(language.get(), "Slide Cards & Bullet Items", "کارت‌ها و نکات کلیدی اسلاید")}
                                                            </h4>
                                                            <p class="zd-slide-items-desc">
                                                                {move || t(language.get(), "Edit each individual point or card in this slide", "ویرایش مجزای هر کارت یا نکته در این اسلاید")}
                                                            </p>
                                                        </div>
                                                        <button
                                                            type="button"
                                                            class="zd-btn-soft zd-btn-add-item"
                                                            on:click=move |_| add_item(cur_idx)
                                                        >
                                                            <Icon data=LuPlus size=12 />
                                                            <span>{move || t(language.get(), "Add Card / Item", "افزودن کارت")}</span>
                                                        </button>
                                                    </div>

                                                    <div class="zd-slide-items-list">
                                                        {if current_slide.items.is_empty() {
                                                            view! {
                                                                <div class="zd-slide-items-empty">
                                                                    <span>{move || t(language.get(), "No separate cards detected in this slide. Click 'Add Card / Item' to insert one, or edit the text directly.", "کارتی در این اسلاید نیست. می‌توانید با دکمه بالا کارت جدید اضافه کنید یا مستقیماً روی بوم تایپ کنید.")}</span>
                                                                </div>
                                                            }.into_any()
                                                        } else {
                                                            current_slide.items.into_iter().enumerate().map(|(item_i, item)| {
                                                                let item_idx = item.index;
                                                                let text_val = item.text.clone();
                                                                view! {
                                                                    <div class="zd-slide-item-card">
                                                                        <div class="zd-slide-item-card-header">
                                                                            <span class="zd-slide-item-card-num">{format!("Item {}", item_i + 1)}</span>
                                                                            <button
                                                                                type="button"
                                                                                class="zd-btn-icon-mini zd-btn-delete-item"
                                                                                on:click=move |_| delete_item(cur_idx, item_idx)
                                                                                title=move || t(language.get(), "Delete item", "حذف آیتم")
                                                                            >
                                                                                <Icon data=LuTrash2 size=12 />
                                                                            </button>
                                                                        </div>
                                                                        <textarea
                                                                            class="zd-textarea zd-slide-item-textarea"
                                                                            prop:value=text_val
                                                                            on:change=move |ev| update_item_text(cur_idx, item_idx, event_target_value(&ev))
                                                                            rows=2
                                                                        ></textarea>
                                                                    </div>
                                                                }
                                                            }).collect_view().into_any()}
                                                        }
                                                    </div>
                                                </div>

                                                <!-- Hint Banner -->
                                                <div class="zd-slide-footer-hint">
                                                    <Icon data=LuPenLine size=13 />
                                                    <span>
                                                        {move || t(
                                                            language.get(),
                                                            "Tip: You can also click 'Edit In-Place' in the top preview toolbar to type directly on any element in the live slide canvas with instant Ctrl+S save!",
                                                            "نکته: با کلیک روی 'ویرایش روی بوم' در نوار بالای پیش‌نمایش، می‌توانید مستقیماً روی متن‌های اسلاید کلیک کرده، تایپ کنید و با Ctrl+S ذخیره نمایید!"
                                                        )}
                                                    </span>
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="zd-slide-empty-state">
                                                <span>{move || t(language.get(), "No slide selected. Click a slide on the left or add a new one.", "اسلایدی انتخاب نشده است. از ستون سمت چپ یک اسلاید را انتخاب کنید.")}</span>
                                            </div>
                                        }.into_any()
                                    }
                                }}
                            </div>
                        </div>
                    </Show>
                </div>
            </aside>
        </Show>
    }
}
