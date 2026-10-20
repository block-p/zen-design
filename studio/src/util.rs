use wasm_bindgen::JsValue;

use crate::i18n::{localize_digits, t, Language};

/// Local wall-clock, 24-hour, matching the studio's activity timestamps.
pub fn format_clock(timestamp: f64, lang: Language) -> String {
    let date = js_sys::Date::new(&JsValue::from_f64(timestamp));
    localize_digits(
        &format!(
            "{:02}:{:02}:{:02}",
            date.get_hours(),
            date.get_minutes(),
            date.get_seconds()
        ),
        lang,
    )
}

pub fn format_sync_time(timestamp: Option<f64>, lang: Language) -> String {
    match timestamp {
        Some(timestamp) => format!(
            "{} {}",
            t(lang, "synced", "همگام‌سازی"),
            format_clock(timestamp, lang)
        ),
        None => t(lang, "awaiting first sync", "در انتظار اولین همگام‌سازی").to_string(),
    }
}
