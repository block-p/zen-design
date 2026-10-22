use icondata_core::IconData;
use icondata_lu::{
    LuChartColumn, LuFilm, LuLayoutTemplate, LuMonitor, LuPresentation, LuRocket, LuSmartphone,
    LuTablet,
};

use crate::i18n::Language;

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum DesignSystemId {
    Stripe,
    Linear,
    Cyberpunk,
    Swiss,
    Nordic,
    Emerald,
    Terminal,
    Sunset,
    Brutalist,
    Glass,
    Custom,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CustomThemeTokens {
    pub name: String,
    pub bg: String,
    pub text: String,
    pub accent: String,
    pub font_family: String,
    pub card_bg: String,
    pub border_color: String,
    #[serde(default = "default_dot")]
    pub dot: String,
    #[serde(default = "default_swatch")]
    pub swatch: String,
}

fn default_dot() -> String {
    "#6366f1".to_string()
}

fn default_swatch() -> String {
    "linear-gradient(135deg, #6366f1 0%, #0f172a 100%)".to_string()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DesignSystemOption {
    pub id: DesignSystemId,
    pub label: &'static str,
    pub label_fa: &'static str,
    pub tagline: &'static str,
    pub tagline_fa: &'static str,
    pub dot: &'static str,
    pub swatch: &'static str,
}

pub const DESIGN_SYSTEMS: [DesignSystemOption; 10] = [
    DesignSystemOption {
        id: DesignSystemId::Stripe,
        label: "Stripe Clean",
        label_fa: "Stripe Clean",
        tagline: "Crisp violet on deep navy",
        tagline_fa: "بنفش شفاف روی سرمه‌ای عمیق",
        dot: "#635bff",
        swatch: "linear-gradient(135deg, #635bff 0%, #0a2540 100%)",
    },
    DesignSystemOption {
        id: DesignSystemId::Linear,
        label: "Linear Dark",
        label_fa: "Linear Dark",
        tagline: "Quiet contrast, precise lines",
        tagline_fa: "کنتراست آرام، خطوط دقیق",
        dot: "#5e6ad2",
        swatch: "linear-gradient(135deg, #5e6ad2 0%, #0d0e12 100%)",
    },
    DesignSystemOption {
        id: DesignSystemId::Cyberpunk,
        label: "Cyberpunk",
        label_fa: "Cyberpunk",
        tagline: "Neon voltage on black",
        tagline_fa: "ولتاژ نئون روی سیاه",
        dot: "#ff0055",
        swatch: "linear-gradient(135deg, #ff0055 0%, #00ffcc 100%)",
    },
    DesignSystemOption {
        id: DesignSystemId::Swiss,
        label: "Swiss Grid",
        label_fa: "Swiss Grid",
        tagline: "Editorial grid, honest type",
        tagline_fa: "شبکه ادیتوریال، تایپ صادق",
        dot: "#e63946",
        swatch: "linear-gradient(135deg, #e63946 0%, #f8f9fa 100%)",
    },
    DesignSystemOption {
        id: DesignSystemId::Nordic,
        label: "Nordic Frost",
        label_fa: "Nordic Frost",
        tagline: "Sage pine & warm stone slate",
        tagline_fa: "مریم‌گلی، درخت کاج و اسلیت ملایم",
        dot: "#10b981",
        swatch: "linear-gradient(135deg, #10b981 0%, #1e293b 100%)",
    },
    DesignSystemOption {
        id: DesignSystemId::Emerald,
        label: "Midnight Emerald",
        label_fa: "Midnight Emerald",
        tagline: "Deep obsidian with emerald luxury",
        tagline_fa: "مشکی آبسیدین با زمرد و طلای تاریک",
        dot: "#059669",
        swatch: "linear-gradient(135deg, #059669 0%, #022c22 100%)",
    },
    DesignSystemOption {
        id: DesignSystemId::Terminal,
        label: "Retro Terminal",
        label_fa: "Retro Terminal",
        tagline: "Amber & phosphor green CRT monospace",
        tagline_fa: "فسفر سبز و کهربایی روی مانیتور لامپی",
        dot: "#22c55e",
        swatch: "linear-gradient(135deg, #22c55e 0%, #050505 100%)",
    },
    DesignSystemOption {
        id: DesignSystemId::Sunset,
        label: "Sunset Warmth",
        label_fa: "Sunset Warmth",
        tagline: "Coral, tangerine & rich plum",
        tagline_fa: "مرجانی، نارنجی غروب و آلوی عمیق",
        dot: "#f97316",
        swatch: "linear-gradient(135deg, #f97316 0%, #4c0519 100%)",
    },
    DesignSystemOption {
        id: DesignSystemId::Brutalist,
        label: "Brutalist Stark",
        label_fa: "Brutalist Stark",
        tagline: "Bold industrial yellow & stark contrast",
        tagline_fa: "زرد بولد صنعتی و تایپوگرافی سنگین",
        dot: "#eab308",
        swatch: "linear-gradient(135deg, #eab308 0%, #000000 100%)",
    },
    DesignSystemOption {
        id: DesignSystemId::Glass,
        label: "Neumorphic Glass",
        label_fa: "Neumorphic Glass",
        tagline: "Frosted glassmorphism & soft violet",
        tagline_fa: "شیشه مات بلوری با هاله بنفش ملایم",
        dot: "#8b5cf6",
        swatch: "linear-gradient(135deg, #8b5cf6 0%, #18181b 100%)",
    },
];

pub fn system_meta(id: DesignSystemId) -> &'static DesignSystemOption {
    DESIGN_SYSTEMS
        .iter()
        .find(|system| system.id == id)
        .unwrap_or(&DESIGN_SYSTEMS[0])
}

impl DesignSystemOption {
    pub fn label(self, lang: Language) -> &'static str {
        match lang {
            Language::En => self.label,
            Language::Fa => self.label_fa,
        }
    }

    pub fn tagline(self, lang: Language) -> &'static str {
        match lang {
            Language::En => self.tagline,
            Language::Fa => self.tagline_fa,
        }
    }
}

impl DesignSystemId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stripe => "stripe",
            Self::Linear => "linear",
            Self::Cyberpunk => "cyberpunk",
            Self::Swiss => "swiss",
            Self::Nordic => "nordic",
            Self::Emerald => "emerald",
            Self::Terminal => "terminal",
            Self::Sunset => "sunset",
            Self::Brutalist => "brutalist",
            Self::Glass => "glass",
            Self::Custom => "custom",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "linear" => Self::Linear,
            "cyberpunk" => Self::Cyberpunk,
            "swiss" => Self::Swiss,
            "nordic" => Self::Nordic,
            "emerald" => Self::Emerald,
            "terminal" => Self::Terminal,
            "sunset" => Self::Sunset,
            "brutalist" => Self::Brutalist,
            "glass" => Self::Glass,
            "custom" => Self::Custom,
            _ => Self::Stripe,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum AspectRatio {
    R16_9,
    R9_16,
    R4_3,
    R1_1,
    Fill,
}

#[derive(Clone, Copy)]
pub struct AspectRatioOption {
    pub id: AspectRatio,
    pub label: &'static str,
    pub label_fa: &'static str,
    pub icon_name: &'static str,
    pub description: &'static str,
    pub description_fa: &'static str,
    pub css_ratio: &'static str,
    pub width: u32,
    pub height: u32,
}

pub const ASPECT_RATIOS: [AspectRatioOption; 5] = [
    AspectRatioOption {
        id: AspectRatio::R16_9,
        label: "16:9",
        label_fa: "۱۶:۹",
        icon_name: "monitor",
        description: "Landscape / Desktop",
        description_fa: "افقی / دسکتاپ و یوتیوب",
        css_ratio: "16 / 9",
        width: 1440,
        height: 810,
    },
    AspectRatioOption {
        id: AspectRatio::R9_16,
        label: "9:16",
        label_fa: "۹:۱۶",
        icon_name: "smartphone",
        description: "Vertical / Reels & Mobile",
        description_fa: "عمودی / استوری و ریلز گوشی",
        css_ratio: "9 / 16",
        width: 720,
        height: 1280,
    },
    AspectRatioOption {
        id: AspectRatio::R4_3,
        label: "4:3",
        label_fa: "۴:۳",
        icon_name: "tablet",
        description: "Classic Presentation",
        description_fa: "کلاسیک / ارائه و مانیتور قدیمی",
        css_ratio: "4 / 3",
        width: 1200,
        height: 900,
    },
    AspectRatioOption {
        id: AspectRatio::R1_1,
        label: "1:1",
        label_fa: "۱:۱",
        icon_name: "square",
        description: "Square / Feed",
        description_fa: "مربع / فید و پست",
        css_ratio: "1 / 1",
        width: 1080,
        height: 1080,
    },
    AspectRatioOption {
        id: AspectRatio::Fill,
        label: "Fill",
        label_fa: "تمام صفحه",
        icon_name: "maximize",
        description: "Responsive Full Viewport",
        description_fa: "واکنش‌گرا و هم‌اندازه صفحه",
        css_ratio: "100%",
        width: 1440,
        height: 900,
    },
];

pub fn aspect_ratio_meta(ratio: AspectRatio) -> &'static AspectRatioOption {
    ASPECT_RATIOS
        .iter()
        .find(|r| r.id == ratio)
        .unwrap_or(&ASPECT_RATIOS[0])
}

impl AspectRatioOption {
    pub fn label(self, lang: Language) -> &'static str {
        match lang {
            Language::En => self.label,
            Language::Fa => self.label_fa,
        }
    }

    pub fn description(self, lang: Language) -> &'static str {
        match lang {
            Language::En => self.description,
            Language::Fa => self.description_fa,
        }
    }
}

impl AspectRatio {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::R16_9 => "16:9",
            Self::R9_16 => "9:16",
            Self::R4_3 => "4:3",
            Self::R1_1 => "1:1",
            Self::Fill => "fill",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "9:16" => Self::R9_16,
            "4:3" => Self::R4_3,
            "1:1" => Self::R1_1,
            "fill" => Self::Fill,
            _ => Self::R16_9,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModeId {
    Prototype,
    Slides,
    Motion,
}

#[derive(Clone, Copy)]
pub struct ModeOption {
    pub id: ModeId,
    pub label: &'static str,
    pub label_fa: &'static str,
    pub icon: &'static IconData,
    pub hint: &'static str,
    pub hint_fa: &'static str,
}

pub const MODES: [ModeOption; 3] = [
    ModeOption {
        id: ModeId::Prototype,
        label: "Prototype",
        label_fa: "نمونه اولیه",
        icon: LuLayoutTemplate,
        hint: "Interactive screens",
        hint_fa: "صفحه‌های تعاملی",
    },
    ModeOption {
        id: ModeId::Slides,
        label: "Slides",
        label_fa: "اسلایدها",
        icon: LuPresentation,
        hint: "Narrative decks",
        hint_fa: "ارائه‌های روایی",
    },
    ModeOption {
        id: ModeId::Motion,
        label: "Motion",
        label_fa: "موشن",
        icon: LuFilm,
        hint: "Looping motion film",
        hint_fa: "فیلم موشن لوپ‌شونده",
    },
];

pub fn mode_meta(id: ModeId) -> &'static ModeOption {
    MODES.iter().find(|mode| mode.id == id).unwrap_or(&MODES[0])
}

impl ModeOption {
    pub fn label(self, lang: Language) -> &'static str {
        match lang {
            Language::En => self.label,
            Language::Fa => self.label_fa,
        }
    }

    pub fn hint(self, lang: Language) -> &'static str {
        match lang {
            Language::En => self.hint,
            Language::Fa => self.hint_fa,
        }
    }
}

impl ModeId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Prototype => "prototype",
            Self::Slides => "slides",
            Self::Motion => "motion",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "slides" => Self::Slides,
            "motion" => Self::Motion,
            _ => Self::Prototype,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Preset {
    pub title: &'static str,
    pub title_fa: &'static str,
    pub description: &'static str,
    pub description_fa: &'static str,
    pub prompt: &'static str,
    pub design_system: DesignSystemId,
    pub mode: ModeId,
    pub icon: &'static IconData,
}

pub const PRESETS: [Preset; 4] = [
    Preset {
        title: "SaaS landing",
        title_fa: "لندینگ SaaS",
        description: "Hero, proof, pricing",
        description_fa: "هیرو، اثبات، قیمت‌گذاری",
        prompt: "Design a polished SaaS landing page for an AI observability platform. Include a confident hero with a live metrics strip, three product capability cards, a testimonial, transparent pricing, and a crisp call to action. Make it feel fast, technical, and premium.",
        design_system: DesignSystemId::Stripe,
        mode: ModeId::Prototype,
        icon: LuRocket,
    },
    Preset {
        title: "Revenue dashboard",
        title_fa: "داشبورد درآمد",
        description: "KPI wall + chart",
        description_fa: "دیوار KPI + نمودار",
        prompt: "Build a dark analytics dashboard for a fintech revenue team. Include a compact sidebar, a welcome header, KPI cards for ARR, active accounts and churn, a revenue chart, a recent activity table, and a small forecast panel. Dense but calm, with strong visual hierarchy.",
        design_system: DesignSystemId::Linear,
        mode: ModeId::Prototype,
        icon: LuChartColumn,
    },
    Preset {
        title: "Launch deck",
        title_fa: "دک معرفی",
        description: "Six-slide narrative",
        description_fa: "روایت شش‌اسلایدی",
        prompt: "Create a six-slide product launch presentation for a developer productivity tool. Open with a bold title slide, frame the problem, reveal the product, show three proof points, include a customer quote, and close with a launch call to action. Keep each slide legible and editorial.",
        design_system: DesignSystemId::Swiss,
        mode: ModeId::Slides,
        icon: LuPresentation,
    },
    Preset {
        title: "Kinetic title",
        title_fa: "تیتر جنبشی",
        description: "10-second motion film",
        description_fa: "فیلم موشن ۱۰ ثانیه‌ای",
        prompt: "Create a 10-second kinetic title sequence for NOVA, a spatial computing headset. One fixed 16:9 artboard, four beats: masked letterforms assemble the word NOVA; a hairline horizon draws across the frame; the number 12ms snaps into place; the end card holds the mark and the line \"Space, folded.\" Hard cuts, clip-path wipes, a restrained navy-and-violet palette, seamless loop. This is a motion piece: no navigation, sections, cards, pricing or footer.",
        design_system: DesignSystemId::Stripe,
        mode: ModeId::Motion,
        icon: LuFilm,
    },
];

impl Preset {
    pub fn title(self, lang: Language) -> &'static str {
        match lang {
            Language::En => self.title,
            Language::Fa => self.title_fa,
        }
    }

    pub fn description(self, lang: Language) -> &'static str {
        match lang {
            Language::En => self.description,
            Language::Fa => self.description_fa,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceId {
    Desktop,
    Tablet,
    Mobile,
}

#[derive(Clone, Copy)]
pub struct DeviceOption {
    pub id: DeviceId,
    pub label: &'static str,
    pub label_fa: &'static str,
    pub icon: &'static IconData,
    pub frame_class: &'static str,
    pub width_label: &'static str,
    pub width_label_fa: &'static str,
}

pub const DEVICES: [DeviceOption; 3] = [
    DeviceOption {
        id: DeviceId::Desktop,
        label: "Desktop",
        label_fa: "دسکتاپ",
        icon: LuMonitor,
        frame_class: "zd-frame--desktop",
        width_label: "Fluid",
        width_label_fa: "سیال",
    },
    DeviceOption {
        id: DeviceId::Tablet,
        label: "Tablet",
        label_fa: "تبلت",
        icon: LuTablet,
        frame_class: "zd-frame--tablet",
        width_label: "820 px",
        width_label_fa: "۸۲۰ px",
    },
    DeviceOption {
        id: DeviceId::Mobile,
        label: "Mobile",
        label_fa: "موبایل",
        icon: LuSmartphone,
        frame_class: "zd-frame--mobile",
        width_label: "390 px",
        width_label_fa: "۳۹۰ px",
    },
];

pub fn device_meta(id: DeviceId) -> &'static DeviceOption {
    DEVICES
        .iter()
        .find(|item| item.id == id)
        .unwrap_or(&DEVICES[0])
}

impl DeviceOption {
    pub fn label(self, lang: Language) -> &'static str {
        match lang {
            Language::En => self.label,
            Language::Fa => self.label_fa,
        }
    }

    pub fn width_label(self, lang: Language) -> &'static str {
        match lang {
            Language::En => self.width_label,
            Language::Fa => self.width_label_fa,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConnectionState {
    Connecting,
    Live,
    Reconnecting,
    Stalled,
}

impl ConnectionState {
    pub fn label(self, lang: Language) -> &'static str {
        match (self, lang) {
            (Self::Connecting, Language::En) => "Connecting",
            (Self::Connecting, Language::Fa) => "در حال اتصال",
            (Self::Live, Language::En) => "Live",
            (Self::Live, Language::Fa) => "متصل",
            (Self::Reconnecting, Language::En) => "Reconnecting",
            (Self::Reconnecting, Language::Fa) => "اتصال مجدد",
            (Self::Stalled, Language::En) => "Heartbeat lost",
            (Self::Stalled, Language::Fa) => "ضربان قطع شد",
        }
    }

    /// Stable CSS tone names. The old studio stored Tailwind classes in this
    /// enum; the redesign keeps presentation in style.css instead.
    pub fn tone(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Stalled => "stalled",
            Self::Connecting | Self::Reconnecting => "pending",
        }
    }

    pub fn animates(self) -> bool {
        matches!(self, Self::Live | Self::Reconnecting)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActivityKind {
    User,
    Queued,
    Thinking,
    Writing,
    Done,
    Reload,
    Error,
    Status,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ActivityEntry {
    pub id: u64,
    pub kind: ActivityKind,
    pub message: String,
    pub at: f64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ChatHistoryItem {
    pub id: String,
    pub prompt: String,
    pub design_system: String,
    pub mode: String,
    #[serde(default)]
    pub deep: bool,
    pub timestamp: u64,
    #[serde(default)]
    pub has_artifact: bool,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct HistoryResponse {
    #[serde(default)]
    pub items: Vec<ChatHistoryItem>,
    #[serde(default)]
    pub current_mode: Option<String>,
}

