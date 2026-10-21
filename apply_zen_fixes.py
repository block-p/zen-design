#!/usr/bin/env python3
"""
ZenDesign Complete Upgrade & Anti-Slop Patcher
==============================================
Usage:
  Copy this file into the root directory of https://github.com/block-p/zen-design and run:
      python3 apply_zen_fixes.py

What this script fixes automatically (creating .bak backups first):
  1. server/src/agent.rs:
     - Fixes the contradiction between Google Fonts in `design_tokens` and strict offline CSP.
     - Separates `Prototype` mode (100dvh interactive App Shell) from `Editorial/Landing` mode.
     - Adds automatic Persian/RTL detection (`contains_rtl_script`) and native `Vazirmatn` +
       `dir="rtl"` + `letter-spacing: 0` rules across Prototype, Slides, and Motion prompts.
     - Adds a pre-HTML `<design_spec>` architectural planning step (transparently stripped by
       `extract_html`) so the LLM plans layout topology, derived semantic CSS tokens, and JS
       state machines before emitting HTML.
     - Mandates procedural inline visuals (multi-layer schematic SVGs, `<feTurbulence>` grain,
       and interactive `<canvas>` charts) instead of empty placeholder boxes.
     - Upgrades `build_user_prompt` & `build_user_prompt_from_request` to inject explicit RTL/Persian & interactivity directives.
     - Upgrades `DIRECTOR REVIEW` in `deep_review` from a passive no-op into an active 4-point
       Art Director & Front-End Engineering elevation pass while preserving `copy_guard` compatibility.
  2. studio/style.css:
     - Replaces the generic Tailwind Slate/Blue/Violet `:root` palette with an architectural
       monochrome & warm stone design system (inspired by Linear, Figma, and Raycast).
     - Eliminates AI-slop visual tropes: kills expanding glowing pulse rings (`.zd-agent-pulse-ping`),
       flattens nested card shadows, tightens puffy 12px-16px radii to 4px-6px studio precision,
       and fixes RTL Persian letter-spacing across the Studio UI.
  3. studio/src/components/header.rs & studio/src/components/agent_panel.rs:
     - Replaces overused `LuSparkles` magic icons in the Studio chrome with clean utilitarian
       icons (`LuCommand` / `LuZap`).
"""

from __future__ import annotations
import pathlib
import re
import shutil
import sys


RUST_HELPER_AND_PROMPTS = r'''/// Checks whether the user prompt contains Persian or Arabic characters.
pub fn contains_rtl_script(text: &str) -> bool {
    text.chars().any(|c| {
        let code = c as u32;
        (0x0600..=0x06FF).contains(&code)
            || (0x0750..=0x077F).contains(&code)
            || (0x08A0..=0x08FF).contains(&code)
            || (0xFB50..=0xFDFF).contains(&code)
            || (0xFE70..=0xFEFF).contains(&code)
    })
}

fn is_marketing_or_editorial_brief(prompt: &str) -> bool {
    let lower = prompt.to_lowercase();
    let landing_keywords = [
        "landing", "marketing", "portfolio", "agency", "editorial", "magazine",
        "blog", "article", "hero", "pricing page", "waitlist", "launch page",
        "لندینگ", "صفحه فرود", "معرفی محصول", "پورتفولیو", "وبلاگ", "مجله", "سایت شرکتی",
    ];
    let app_keywords = [
        "dashboard", "app", "studio", "editor", "kanban", "crm", "analytics",
        "terminal", "ide", "player", "synth", "calculator", "tracker", "panel",
        "داشبورد", "اپلیکیشن", "پنل", "مدیریت", "ابزار", "ادیتور", "پلیر", "سیستم",
    ];
    let has_landing = landing_keywords.iter().any(|k| lower.contains(k));
    let has_app = app_keywords.iter().any(|k| lower.contains(k));
    has_landing && !has_app
}

pub fn build_system_prompt(tokens: &DesignTokens, mode: &str) -> String {
    let req = PromptRequest {
        prompt: String::new(),
        design_system: String::from("linear"),
        mode: mode.to_string(),
        deep: false,
        session_id: None,
        custom_theme: None,
        aspect_ratio: None,
        duration_secs: None,
        sound_effects: None,
        tts_text: None,
        tts_lang: None,
    };
    build_system_prompt_with_request(tokens, &req)
}

pub fn build_system_prompt_with_request(tokens: &DesignTokens, req: &PromptRequest) -> String {
    if req.mode == "motion" {
        return build_motion_prompt_from_request(tokens, req);
    }
    if req.mode == "slides" {
        return build_slides_prompt(tokens);
    }

    let is_rtl = contains_rtl_script(&req.prompt);
    let is_editorial = is_marketing_or_editorial_brief(&req.prompt);

    let layout_architecture = if is_editorial {
        "- Layout Architecture (Editorial / Brand Surface):\n  * Build an asymmetrical editorial grid (12 columns, max-width 1200px, 24px gap) with crisp 1px structural hairlines (like Stripe Press, Linear, or Read.cv).\n  * Vary visual cadence across sections: pair a high-impact split hero (interactive live component preview or multi-layer schematic SVG on one side, tight typographic lockup on the other) with a bento/matrix specification grid, an interactive live demo/calculator section, and a structured technical spec table.\n  * Never stack three identical icon-title-paragraph cards in a row."
    } else {
        "- Layout Architecture (Interactive Product Screen / App Shell):\n  * Build a functional, high-density software workspace (like Linear, Figma, Raycast, or Vercel Dashboard), NOT a marketing landing page. Do NOT include marketing hero banners, testimonials, pricing tiers, or giant footers unless explicitly asked.\n  * Use a full-viewport application shell (`min-height: 100dvh; display: grid`) with a persistent navigation rail or command header, a primary workspace canvas/table/board, and a contextual inspector or detail drawer.\n  * Use tight, utilitarian spacing inside panels (4px, 8px, 12px, 16px, 24px) with `1px solid` structural borders separating panes."
    };

    let rtl_and_i18n_rules = if is_rtl {
        "- Persian / RTL Typography & Layout Rules (CRITICAL - Brief is in Persian):\n  * Set `<html lang=\"fa\" dir=\"rtl\">` on the root element.\n  * Prepend `'Vazirmatn', 'IRANSansX', 'Tahoma'` to the `--font-sans` stack so Persian glyphs render natively.\n  * NEVER apply negative `letter-spacing` (`tracking`) to Persian text or headings! Keep `letter-spacing: 0` on all Persian elements.\n  * Set body `line-height: 1.75` and heading `line-height: 1.35` to accommodate Persian ascenders/descenders.\n  * Use CSS logical properties everywhere (`margin-inline-start`, `padding-inline`, `border-inline-start`, `inset-inline-end`, `text-align: start`).\n  * Use `font-variant-numeric: tabular-nums` on aligned data columns."
    } else {
        "- Bidirectional & Typography Readiness:\n  * If the brief requests Persian/Arabic content, set `<html lang=\"fa\" dir=\"rtl\">`, include `'Vazirmatn'` first in the font stack, and set `letter-spacing: 0` on all RTL text.\n  * For Latin text, tighten display heading tracking (`-0.025em` to `-0.035em`) and use `font-variant-numeric: tabular-nums` on all numeric columns, counters, and timers."
    };

    format!(
        r#"You are ZenDesign, a principal product designer and staff design-systems engineer. You turn a brief into one self-contained, production-grade HTML artifact (`<!DOCTYPE html>`).

Design tokens for this brief (define these in `:root` and derive semantic surface/text tiers using `color-mix(in srgb, ...)`):
- background: {bg}
- text: {text}
- accent: {accent}
- font stack: {font}
- card background: {card_bg}
- border color: {border}

Required Derived Semantic CSS Variables in `:root`:
- `--bg-canvas`: {bg}
- `--bg-surface`: {card_bg}
- `--bg-elevated`: `color-mix(in srgb, {card_bg} 88%, {text} 12%)`
- `--bg-subtle`: `color-mix(in srgb, {bg} 92%, {text} 8%)`
- `--text-primary`: {text}
- `--text-secondary`: `color-mix(in srgb, {text} 72%, {bg} 28%)`
- `--text-muted`: `color-mix(in srgb, {text} 48%, {bg} 52%)`
- `--border-subtle`: {border}
- `--border-strong`: `color-mix(in srgb, {border} 65%, {text} 35%)`
- `--accent`: {accent}
- `--accent-soft`: `color-mix(in srgb, {accent} 14%, transparent)`
- `--font-sans`: 'Vazirmatn', {font}, system-ui, -apple-system, sans-serif

Craft standards – apply all of them:
- Direction: commit to a cohesive, unmistakable art direction (high-craft industrial utility, Swiss editorial grid, dark precision studio, or tactile brutalism) that matches the tokens.
{layout_architecture}
- Offline Font & Visual Asset Strategy (CSP-Safe):
  * External network requests, CDNs, and remote images are strictly blocked by Content-Security-Policy (`{csp}`).
  * NEVER leave empty colored boxes where illustrations or media belong. Instead, build rich, self-contained procedural visuals:
    1. Multi-layered technical/schematic inline `<svg>` illustrations with precise `1px–1.5px` strokes, `stroke-dasharray` grid lines, `<linearGradient>` fills, and subtle SVG `<filter>` grain (`<feTurbulence>`).
    2. Live, interactive `<canvas>` or `<svg>` data visualizations (sparklines, node graphs, waveforms, heatmaps, or real-time gauges) driven by JavaScript.
    3. Authentic, interactive UI micro-components (command palettes, live code/json inspectors, timeline scrubbers, sortable data tables).
- Spacing & Density: use a strict geometric scale (4, 8, 12, 16, 24, 32, 48, 64, 96px). Align edges along shared vertical axes; avoid centering every block.
- Color & Depth: keep `--accent` under 10% of total screen area. If `--text-primary` is a high-saturation neon hue (e.g. in cyberpunk/terminal themes), reserve the full-saturation hue for headings/accents and mix body text with neutral white (`color-mix(in srgb, var(--text-primary) 35%, #f4f4f5)`) so body copy clears 4.5:1 contrast.
{rtl_and_i18n_rules}
- Content Authenticity: write domain-specific, realistic data — real metric figures, realistic timestamps, technical identifiers, and believable Persian or English copy. Never use "Lorem ipsum", "Headline here", emoji icons, or generic AI buzzwords ("revolutionize", "seamless", "elevate", "unleash", "next-gen").
- Genuine Interactive State Machine (`<script>`):
  * Prototype mode: build an interactive product screen, not a marketing page. Use realistic data (names, numbers, statuses), working hover/click states, and tabs, filters or toggles where they help tell the story. Every control the design implies should do something visible.
  * Implement a clean vanilla JS state object driving real user interactions: working view/tab switchers, live search/filter input that filters rendered rows/cards in real time, interactive drawers/modals, or live sliders/calculators that dynamically recompute metrics.
  * Every interactive element (`<button>`, `<input>`, `[role="tab"]`) must have visible `:hover`, `:focus-visible`, and `:active` states with `150ms cubic-bezier(0.22, 1, 0.36, 1)` transitions.
- Responsive & Overflow Discipline: no horizontal scrolling at 360px. Check 390, 768, 1280 and 1440 widths mentally before finishing.
- Anti-Slop Ban List: NO glowing purple/pink floating gradient blobs; NO emoji used as UI icons; NO three-identical-card feature rows; NO heavy drop shadows on every card; NO broken external `<img>` tags.

Output contract:
- Before the HTML block, you MAY write a brief `<design_spec>` block (under 120 words) planning layout topology, interactive state, and custom SVG/Canvas visuals.
- Return ONE complete HTML document in a single ```html fenced code block starting with `<!DOCTYPE html>`.
- Everything inline: CSS in `<style>`, JS in `<script>`. No external requests whatsoever – no CDNs, no web fonts, no remote images. Inline SVG or data URIs only.
- If the brief cannot be satisfied, reply with a single line beginning with "ERROR:"."#,
        bg = tokens.bg,
        text = tokens.text,
        accent = tokens.accent,
        font = tokens.font_family,
        card_bg = tokens.card_bg,
        border = tokens.border_color,
        layout_architecture = layout_architecture,
        rtl_and_i18n_rules = rtl_and_i18n_rules,
        csp = CSP_META,
    )
}'''


RUST_USER_PROMPTS = r'''pub fn build_user_prompt(prompt: &str, design_system: &str, mode: &str) -> String {
    let dummy = PromptRequest {
        prompt: prompt.to_string(),
        design_system: design_system.to_string(),
        mode: mode.to_string(),
        deep: false,
        session_id: None,
        custom_theme: None,
        aspect_ratio: None,
        duration_secs: None,
        sound_effects: None,
        tts_text: None,
        tts_lang: None,
    };
    build_user_prompt_from_request(&dummy)
}

pub fn build_user_prompt_from_request(req: &PromptRequest) -> String {
    let is_rtl = contains_rtl_script(&req.prompt);
    let lang_directive = if is_rtl {
        "\nLanguage & Direction: Persian (فارسی) — MUST use `<html lang=\"fa\" dir=\"rtl\">`, `'Vazirmatn'` font priority, `letter-spacing: 0`, and natural Persian UI terminology."
    } else {
        ""
    };

    let duration = req.duration_secs.unwrap_or(10).clamp(5, 600);
    let artifact_type = match req.mode.as_str() {
        "motion" => {
            format!(
                "\nArtifact type: MOTION PIECE – a self-playing browser animation running for EXACTLY {duration}s. NOT a website: no nav, sections, cards, pricing, FAQ, CTA or footer."
            )
        }
        "slides" => {
            "\nArtifact type: SLIDE DECK (PRECISE SLIDE DECOMPOSITION) – full-viewport 16:9 presentation deck. Carefully respect any explicit slide separations, slide titles, numbered steps, or sections provided in the brief. Every designated slide must be faithfully represented as a separate, beautifully composed 16:9 slide.".to_string()
        }
        _ => {
            "\nArtifact type: INTERACTIVE PRODUCT / EDITORIAL ARTIFACT – include a working vanilla JS state machine (live filtering, tab switching, interactive charts/calculators, or stateful inspection) and bespoke inline SVG/Canvas graphics.".to_string()
        }
    };

    format!(
        "Brief:\n{}\n\nDesign system id: {}\nMode: {}{}{}\n\nBuild the artifact now.",
        req.prompt.trim(),
        req.design_system,
        req.mode,
        artifact_type,
        lang_directive
    )
}'''


NEW_DIRECTOR_REVIEW_TEXT = (
    "DIRECTOR REVIEW\\n\\n"
    "The current document is your own cut. Do not defend it, and do not describe the review inside the artifact. "
    "Audit and elevate the cut across four dimensions: "
    "(1) Replace any generic AI-slop patterns (empty placeholder boxes, floating gradient blobs, repetitive 3-card rows) with crisp 1px architectural borders, multi-layer schematic inline <svg> graphics, or live <canvas> data visualizations; "
    "(2) Fix any layout alignment, contrast, or overflow defects, and if Persian/RTL content is present ensure dir=\\\"rtl\\\", Vazirmatn font stack, and letter-spacing: 0 on headings; "
    "(3) Ensure every interactive tab, filter, toggle, or control is wired to working vanilla JS state in <script>; "
    "(4) Preserve the beat map, all user-specified copy/headlines/numbers, and the loop. "
    "Return only one complete fenced HTML document."
)


EMBEDDED_ANTI_SLOP_CSS = r"""
/* === ZEN DESIGN ANTI-SLOP ARCHITECTURAL OVERRIDES === */
:root {
  --zd-bg-canvas: #f3f2ee !important;
  --zd-bg-card: #ffffff !important;
  --zd-bg-muted: #ebe9e4 !important;
  --zd-bg-subtle: #e2e0d8 !important;

  --zd-border: #dfddd6 !important;
  --zd-border-muted: #ebe9e4 !important;
  --zd-border-strong: #c7c4bc !important;

  --zd-text: #141413 !important;
  --zd-text-secondary: #52514e !important;
  --zd-text-muted: #85837d !important;
  --zd-text-faint: #b5b3ad !important;

  --zd-primary: #18181b !important;
  --zd-primary-fg: #ffffff !important;
  --zd-primary-hover: #27272a !important;
  --zd-primary-soft: #e4e4e7 !important;
  --zd-primary-ghost: #f4f4f5 !important;

  --zd-emerald: #0d9488 !important;
  --zd-emerald-soft: #ccfbf1 !important;
  --zd-amber: #d97706 !important;
  --zd-amber-soft: #fef3c7 !important;
  --zd-red: #dc2626 !important;
  --zd-red-soft: #fee2e2 !important;
  --zd-violet: #18181b !important;
  --zd-violet-soft: #e4e4e7 !important;

  --zd-radius-sm: 4px !important;
  --zd-radius-md: 6px !important;
  --zd-radius-lg: 6px !important;
  --zd-radius-xl: 8px !important;

  --zd-shadow-sm: 0 1px 2px rgba(20, 20, 19, 0.04) !important;
  --zd-shadow-md: 0 2px 6px rgba(20, 20, 19, 0.06) !important;
  --zd-shadow-lg: 0 12px 32px rgba(20, 20, 19, 0.14) !important;

  --zd-ease: cubic-bezier(0.22, 1, 0.36, 1) !important;
  --zd-duration: 120ms !important;
}

html[data-theme="dark"],
html.dark,
body.dark {
  --zd-bg-canvas: #0c0c0d !important;
  --zd-bg-card: #141416 !important;
  --zd-bg-muted: #1c1c1f !important;
  --zd-bg-subtle: #26262a !important;
  --zd-border: #26262a !important;
  --zd-border-muted: #1c1c1f !important;
  --zd-border-strong: #38383e !important;
  --zd-text: #f4f4f5 !important;
  --zd-text-secondary: #a1a1aa !important;
  --zd-text-muted: #71717a !important;
  --zd-text-faint: #52525b !important;
  --zd-primary: #f4f4f5 !important;
  --zd-primary-fg: #0c0c0d !important;
  --zd-primary-hover: #ffffff !important;
  --zd-primary-soft: #27272a !important;
  --zd-primary-ghost: #1f1f22 !important;
}

html[dir="rtl"] *,
html[lang="fa"] * {
  letter-spacing: 0 !important;
}

.zd-header {
  background: var(--zd-bg-card) !important;
  border-bottom: 1px solid var(--zd-border) !important;
  box-shadow: none !important;
  backdrop-filter: none !important;
}

.zd-header-logo,
.zd-header-logo-icon {
  background: var(--zd-text) !important;
  color: var(--zd-bg-card) !important;
  border-radius: 5px !important;
  box-shadow: none !important;
}

.zd-sidebar {
  background: var(--zd-bg-card) !important;
  border-inline-end: 1px solid var(--zd-border) !important;
  box-shadow: none !important;
  border-radius: 0 !important;
}

.zd-agent-icon {
  background: var(--zd-bg-muted) !important;
  color: var(--zd-text) !important;
  border: 1px solid var(--zd-border-strong) !important;
  border-radius: 5px !important;
  box-shadow: none !important;
}

.zd-agent-pulse-ping,
.zd-agent-pulse-ring {
  display: none !important;
  animation: none !important;
}

.zd-agent-pulse {
  background: transparent !important;
  box-shadow: none !important;
}

.zd-agent-pulse-dot {
  width: 6px !important;
  height: 6px !important;
  border-radius: 50% !important;
  background: var(--zd-emerald) !important;
  box-shadow: none !important;
}

.zd-btn,
.zd-btn-soft,
.zd-btn-secondary,
.zd-header-btn,
.zd-picker-trigger {
  border-radius: 5px !important;
  border: 1px solid var(--zd-border) !important;
  background: var(--zd-bg-card) !important;
  color: var(--zd-text) !important;
  box-shadow: var(--zd-shadow-sm) !important;
}

.zd-btn-primary {
  background: var(--zd-primary) !important;
  color: var(--zd-primary-fg) !important;
  border: 1px solid var(--zd-primary) !important;
  border-radius: 5px !important;
  box-shadow: none !important;
}

.zd-btn-primary span,
.zd-btn-primary svg {
  color: var(--zd-primary-fg) !important;
}

.zd-btn-submit--active,
.zd-detected-connect-btn,
.zd-history-btn-open,
.zd-history-pill {
  background: var(--zd-primary) !important;
  color: var(--zd-primary-fg) !important;
  border: 1px solid var(--zd-primary) !important;
}

.zd-btn-submit--active span,
.zd-detected-connect-btn span,
.zd-history-btn-open span,
.zd-detected-connect-btn svg,
.zd-btn-submit--active svg,
.zd-history-btn-open svg {
  color: var(--zd-primary-fg) !important;
}

.zd-detected-connect-btn:hover:not(:disabled),
.zd-btn-submit--active:hover:not(:disabled),
.zd-history-btn-open:hover:not(:disabled),
.zd-btn-primary:hover:not(:disabled) {
  background: var(--zd-primary-hover) !important;
  color: var(--zd-primary-fg) !important;
  border-color: var(--zd-primary-hover) !important;
}

.zd-detected-connect-btn:disabled,
.zd-detected-connect-btn--loading {
  background: var(--zd-bg-muted) !important;
  color: var(--zd-text-muted) !important;
  border-color: var(--zd-border) !important;
}

.zd-detected-connect-btn:disabled span,
.zd-detected-connect-btn--loading span,
.zd-detected-connect-btn:disabled svg,
.zd-detected-connect-btn--loading svg {
  color: var(--zd-text-muted) !important;
}

.zd-preset-card {
  border-radius: 6px !important;
  border: 1px solid var(--zd-border) !important;
  background: var(--zd-bg-card) !important;
  box-shadow: none !important;
}

.zd-preset-card:hover {
  border-color: var(--zd-text) !important;
  background: var(--zd-bg-muted) !important;
  transform: none !important;
}
"""


def patch_agent_rs(repo_root: pathlib.Path) -> None:
    agent_path = repo_root / "server" / "src" / "agent.rs"
    if not agent_path.exists():
        print(f"[!] Could not find {agent_path}")
        return

    backup = agent_path.with_suffix(".rs.bak")
    shutil.copy2(agent_path, backup)
    content = agent_path.read_text(encoding="utf-8")

    # 1. Replace build_system_prompt + build_system_prompt_with_request
    pattern_sys = re.compile(
        r"pub fn build_system_prompt\s*\([^)]*\)\s*->\s*String\s*\{.*?"
        r"pub fn build_motion_prompt\s*\(",
        re.DOTALL,
    )
    if pattern_sys.search(content):
        content = pattern_sys.sub(
            RUST_HELPER_AND_PROMPTS + "\n\npub fn build_motion_prompt(",
            content,
            count=1,
        )
        print("[+] Upgraded build_system_prompt & build_system_prompt_with_request in server/src/agent.rs")
    else:
        print("[!] Warning: Could not locate build_system_prompt block in server/src/agent.rs")

    # 2. Replace build_user_prompt & build_user_prompt_from_request
    pattern_user = re.compile(
        r"pub fn build_user_prompt\s*\([^)]*\)\s*->\s*String\s*\{.*?"
        r"(?=/// The user turn for a deep-review pass|\nfn build_review_prompt)",
        re.DOTALL,
    )
    if pattern_user.search(content):
        content = pattern_user.sub(RUST_USER_PROMPTS + "\n\n", content, count=1)
        print("[+] Upgraded build_user_prompt & build_user_prompt_from_request in server/src/agent.rs")
    else:
        # Fallback to single function match
        single_user = re.compile(
            r"pub fn build_user_prompt\s*\([^)]*\)\s*->\s*String\s*\{.*?\n\}",
            re.DOTALL,
        )
        if single_user.search(content):
            content = single_user.sub(RUST_USER_PROMPTS, content, count=1)
            print("[+] Upgraded build_user_prompt in server/src/agent.rs")

    # 3. Upgrade DIRECTOR REVIEW prompt in deep_review
    old_review_pattern = re.compile(
        r"DIRECTOR REVIEW\s+The current document is your own cut\.[^\"]*?Return only one complete fenced HTML document\.",
        re.DOTALL,
    )
    if old_review_pattern.search(content):
        content = old_review_pattern.sub(NEW_DIRECTOR_REVIEW_TEXT, content, count=1)
        print("[+] Upgraded DIRECTOR REVIEW prompt in server/src/agent.rs")

    agent_path.write_text(content, encoding="utf-8")


def patch_style_css(repo_root: pathlib.Path) -> None:
    css_path = repo_root / "studio" / "style.css"
    if not css_path.exists():
        print(f"[!] Could not find {css_path}")
        return

    backup = css_path.with_suffix(".css.bak")
    shutil.copy2(css_path, backup)
    content = css_path.read_text(encoding="utf-8")

    marker = "/* === ZEN DESIGN ANTI-SLOP ARCHITECTURAL OVERRIDES === */"
    if marker not in content:
        content = content.rstrip() + "\n\n" + EMBEDDED_ANTI_SLOP_CSS.strip() + "\n"
        css_path.write_text(content, encoding="utf-8")
        print("[+] Applied Anti-Slop Architectural Design System to studio/style.css")
    else:
        print("[i] studio/style.css already contains Anti-Slop overrides.")


def patch_studio_components(repo_root: pathlib.Path) -> None:
    agent_panel_path = repo_root / "studio" / "src" / "components" / "agent_panel.rs"
    if agent_panel_path.exists():
        shutil.copy2(agent_panel_path, agent_panel_path.with_suffix(".rs.bak"))
        src = agent_panel_path.read_text(encoding="utf-8")
        src_updated = src.replace("data=LuSparkles", "data=LuZap")
        if "LuZap" in src_updated and "LuSparkles" in src_updated:
            src_updated = src_updated.replace("LuSparkles, ", "").replace("LuSparkles,", "")
        agent_panel_path.write_text(src_updated, encoding="utf-8")
        print("[+] Replaced LuSparkles with LuZap in studio/src/components/agent_panel.rs")

    header_path = repo_root / "studio" / "src" / "components" / "header.rs"
    if header_path.exists():
        shutil.copy2(header_path, header_path.with_suffix(".rs.bak"))
        src = header_path.read_text(encoding="utf-8")
        if "LuSparkles" in src:
            src_updated = src.replace("LuSparkles", "LuCommand")
            header_path.write_text(src_updated, encoding="utf-8")
            print("[+] Replaced LuSparkles with LuCommand in studio/src/components/header.rs")


def main() -> int:
    repo_root = pathlib.Path.cwd()
    print("=== ZenDesign Upgrade & Anti-Slop Patcher ===")
    patch_agent_rs(repo_root)
    patch_style_css(repo_root)
    patch_studio_components(repo_root)
    print("=== Done! Run `cargo test` and `trunk build` to verify. ===")
    return 0


if __name__ == "__main__":
    sys.exit(main())
