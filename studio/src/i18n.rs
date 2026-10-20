use web_sys::Storage;

/// The studio ships with two first-class interface languages. Persian flips
/// the document to RTL and swaps the type stack to Vazirmatn; English uses
/// Inter. The choice is kept in localStorage so a reload never loses it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Language {
    En,
    Fa,
}

impl Language {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Fa => "fa",
        }
    }

    pub fn dir(self) -> &'static str {
        match self {
            Self::En => "ltr",
            Self::Fa => "rtl",
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::En => Self::Fa,
            Self::Fa => Self::En,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::En => "Switch to Persian",
            Self::Fa => "تغییر به انگلیسی",
        }
    }
}

/// Tiny inline translation helper. Keeps every string next to the markup that
/// uses it instead of turning a visual redesign into a key-management layer.
pub fn t(lang: Language, en: &'static str, fa: &'static str) -> &'static str {
    match lang {
        Language::En => en,
        Language::Fa => fa,
    }
}

/// Persian UI uses Persian-Indic digits. Only plain ASCII digits are touched,
/// so model names, paths and endpoint hosts stay copy-pasteable.
pub fn localize_digits(value: &str, lang: Language) -> String {
    if lang == Language::En {
        return value.to_string();
    }
    value
        .chars()
        .map(|ch| match ch {
            '0' => '۰',
            '1' => '۱',
            '2' => '۲',
            '3' => '۳',
            '4' => '۴',
            '5' => '۵',
            '6' => '۶',
            '7' => '۷',
            '8' => '۸',
            '9' => '۹',
            _ => ch,
        })
        .collect()
}

fn storage() -> Option<Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

pub fn load_language() -> Language {
    match storage()
        .and_then(|storage| storage.get_item("zen-language").ok().flatten())
        .as_deref()
    {
        Some("fa") => Language::Fa,
        _ => Language::En,
    }
}

pub fn store_language(lang: Language) {
    if let Some(storage) = storage() {
        let _ = storage.set_item("zen-language", lang.as_str());
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

pub fn load_theme() -> Theme {
    if let Some(storage) = storage() {
        if let Ok(Some(val)) = storage.get_item("zen-theme") {
            if val == "dark" {
                return Theme::Dark;
            } else if val == "light" {
                return Theme::Light;
            }
        }
    }
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(root) = document.document_element() {
            if root.get_attribute("data-theme").as_deref() == Some("dark") {
                return Theme::Dark;
            }
        }
    }
    Theme::Light
}

pub fn store_theme(theme: Theme) {
    if let Some(storage) = storage() {
        let _ = storage.set_item("zen-theme", theme.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persian_digits_leave_latin_and_paths_alone() {
        assert_eq!(
            localize_digits("12/2000 · 3 events", Language::Fa),
            "۱۲/۲۰۰۰ · ۳ events"
        );
        assert_eq!(
            localize_digits("ag/claude-4-6", Language::Fa),
            "ag/claude-۴-۶"
        );
        assert_eq!(localize_digits("12/2000", Language::En), "12/2000");
    }

    #[test]
    fn directions_flip() {
        assert_eq!(Language::En.dir(), "ltr");
        assert_eq!(Language::Fa.dir(), "rtl");
    }
}
