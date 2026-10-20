use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, RwLock};
use tracing::{info, warn};

use crate::frames;
use crate::providers::{self, Generated};
use crate::settings::{Settings, PROVIDER_ANTHROPIC, PROVIDER_OPENAI, PROVIDER_TEMPLATE};

/// Tailwind utilities compiled offline from `preview.input.css`.
/// Inlining them keeps generated artifacts fully local: the preview makes no
/// network requests at all, not even to a CDN.
const PREVIEW_CSS: &str = include_str!("../assets/preview.css");

/// Hard ceiling for a single artifact. Anything above this is almost always a
/// runaway generation rather than a design.
const MAX_ARTIFACT_BYTES: usize = 512 * 1024;

/// Deep review: render the cut, let the model look at four frames, rewrite it,
/// repeat. This is deliberately time-boxed so a slow reasoning model cannot
/// hold the daemon forever.
const DEEP_PASSES: usize = 3;
const DEEP_BUDGET: Duration = Duration::from_secs(45 * 60);

/// Inline CSP injected into generated artifacts so model output cannot call
/// home or reach back into the daemon API.
pub const CSP_META: &str = r#"<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline'; img-src data: blob:; font-src data:; media-src data: blob:; connect-src 'none'; base-uri 'none'; form-action 'none'; frame-src 'none'">"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
    pub design_system: String, // e.g. "stripe", "linear", "cyberpunk", "swiss"
    pub mode: String,          // e.g. "prototype", "slides", "motion"
    /// Render, review and rewrite the artifact up to `DEEP_PASSES` times.
    #[serde(default)]
    pub deep: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    pub event_type: String, // "status", "token", "writing", "done", "error"
    pub message: String,
}

/// Design-system tokens shared by the offline template and the model prompt.
#[derive(Debug, Clone, Copy)]
pub struct DesignTokens {
    pub bg: &'static str,
    pub text: &'static str,
    pub accent: &'static str,
    pub font_family: &'static str,
    pub card_bg: &'static str,
    pub border_color: &'static str,
}

pub fn design_tokens(design_system: &str) -> DesignTokens {
    match design_system {
        "linear" => DesignTokens {
            bg: "#0d0e12",
            text: "#f3f4f6",
            accent: "#5e6ad2",
            font_family: "system-ui, -apple-system, sans-serif",
            card_bg: "#161822",
            border_color: "#2b2d3d",
        },
        "cyberpunk" => DesignTokens {
            bg: "#0a0a14",
            text: "#00ffcc",
            accent: "#ff0055",
            font_family: "'Courier New', monospace",
            card_bg: "#14142b",
            border_color: "#ff0055",
        },
        "swiss" => DesignTokens {
            bg: "#ffffff",
            text: "#111111",
            accent: "#e63946",
            font_family: "Helvetica Neue, Arial, sans-serif",
            card_bg: "#f8f9fa",
            border_color: "#111111",
        },
        _ => DesignTokens {
            // Stripe default
            bg: "#0a2540",
            text: "#ffffff",
            accent: "#635bff",
            font_family: "'Inter', system-ui, sans-serif",
            card_bg: "#ffffff10",
            border_color: "#ffffff20",
        },
    }
}

pub struct AgentOrchestrator {
    workspace_dir: String,
    event_sender: broadcast::Sender<String>,
    settings: Arc<RwLock<Settings>>,
    http: reqwest::Client,
}

impl AgentOrchestrator {
    pub fn new(
        workspace_dir: String,
        event_sender: broadcast::Sender<String>,
        settings: Arc<RwLock<Settings>>,
        http: reqwest::Client,
    ) -> Self {
        Self {
            workspace_dir,
            event_sender,
            settings,
            http,
        }
    }

    pub async fn execute_prompt(&self, req: PromptRequest) -> Result<(), String> {
        let settings = self.settings.read().await.clone();
        if settings.provider == PROVIDER_TEMPLATE {
            return self.render_template(req).await;
        }
        self.run_provider(req, &settings).await
    }

    /// Deterministic offline path: no network, no model.
    async fn render_template(&self, req: PromptRequest) -> Result<(), String> {
        let sender = &self.event_sender;
        providers::emit_event(
            sender,
            "status",
            format!(
                "Agent analyzing prompt: '{}' with Design System [{}]",
                req.prompt, req.design_system
            ),
        );

        tokio::time::sleep(Duration::from_millis(600)).await;

        providers::emit_event(
            sender,
            "status",
            "Applying design system tokens & composing HTML/CSS artifact...",
        );

        tokio::time::sleep(Duration::from_millis(500)).await;

        let generated_html = generate_prototype_html(&req.prompt, &req.design_system, &req.mode);
        self.write_artifact(&generated_html)?;
        providers::emit_event(sender, "done", "Artifact rendered successfully.");
        Ok(())
    }

    async fn run_provider(&self, req: PromptRequest, settings: &Settings) -> Result<(), String> {
        let tokens = design_tokens(&req.design_system);
        let system = build_system_prompt(&tokens, &req.mode);
        let user = build_user_prompt(&req.prompt, &req.design_system, &req.mode);

        let label = settings.provider_label();
        let model = settings.active_profile().model.trim().to_string();
        providers::emit_event(
            &self.event_sender,
            "status",
            format!(
                "Thinking with {label}{}…",
                if model.is_empty() {
                    String::new()
                } else {
                    format!(" · {model}")
                }
            ),
        );

        let result = providers::generate(
            &self.http,
            settings,
            &system,
            &user,
            &self.event_sender,
            Path::new(&self.workspace_dir),
        )
        .await;

        match result {
            Ok(Generated::Text(raw)) => {
                let html = extract_html(&raw).ok_or_else(|| {
                    "The model did not return an HTML artifact. Try again or pick another model."
                        .to_string()
                })?;
                let mut html = ensure_csp(&html);
                let mut reviewed = 0;
                let mut deep_attempted = false;

                if req.deep {
                    if matches!(
                        settings.provider.as_str(),
                        PROVIDER_OPENAI | PROVIDER_ANTHROPIC
                    ) {
                        deep_attempted = true;
                        let (next, passes) = self.deep_review(&req, html, settings, &system).await;
                        html = next;
                        reviewed = passes;
                    } else {
                        providers::emit_event(
                            &self.event_sender,
                            "status",
                            "Deep review needs an OpenAI-compatible or Anthropic API provider; running once.",
                        );
                    }
                }

                if html.len() > MAX_ARTIFACT_BYTES {
                    return Err(format!(
                        "The model returned {} KB, above the {} KB limit.",
                        html.len() / 1024,
                        MAX_ARTIFACT_BYTES / 1024
                    ));
                }
                providers::emit_event(
                    &self.event_sender,
                    "writing",
                    format!("Applying model output ({} KB)…", html.len() / 1024),
                );
                self.write_artifact(&html)?;
                let message = if reviewed > 0 {
                    format!(
                        "Artifact refreshed via {label} after {reviewed} review pass{}.",
                        if reviewed == 1 { "" } else { "es" }
                    )
                } else if deep_attempted {
                    format!("Artifact refreshed via {label}. Deep review kept the one-shot cut.")
                } else {
                    format!("Artifact refreshed via {label}.")
                };
                providers::emit_event(&self.event_sender, "done", message);
                Ok(())
            }
            Ok(Generated::ArtifactWritten) => {
                match self.harden_agent_artifact() {
                    Ok(Some(message)) => {
                        providers::emit_event(&self.event_sender, "status", message)
                    }
                    Ok(None) => {}
                    Err(error) => warn!("{error}"),
                }
                providers::emit_event(
                    &self.event_sender,
                    "done",
                    format!("Artifact written by the agent ({label})."),
                );
                Ok(())
            }
            Err(error) => {
                warn!("Provider run failed: {error}");
                Err(error)
            }
        }
    }

    /// Render → review → rewrite, with guardrails. Every candidate revision is
    /// checked against the required copy and a rendered luma profile before it
    /// can replace the current cut. A rejected revision gets one correction
    /// attempt; a second failure keeps the current cut.
    async fn deep_review(
        &self,
        req: &PromptRequest,
        mut html: String,
        settings: &Settings,
        system: &str,
    ) -> (String, usize) {
        let deadline = Instant::now() + DEEP_BUDGET;
        let workspace = Path::new(&self.workspace_dir);
        let quoted = quoted_phrases(&req.prompt);
        let mut applied = 0usize;

        providers::emit_event(
            &self.event_sender,
            "status",
            "Deep review: rendering the opening cut…",
        );
        let mut current_frames = match frames::capture(&html, 0).await {
            Ok(captured) => captured,
            Err(error) => {
                warn!("Deep opening capture failed: {error}");
                providers::emit_event(
                    &self.event_sender,
                    "status",
                    format!(
                        "Deep review skipped: the current cut could not be rendered ({}).",
                        truncate(&error, 200)
                    ),
                );
                return (html, 0);
            }
        };

        let review_system = format!(
            "{system}\n\n---\n\nDIRECTOR REVIEW\n\
The current document is your own cut. Do not defend it, and do not describe \
the review inside the artifact. Find any material problems. If the cut is \
already strong, return it unchanged; do not invent work. If you rewrite, make \
the smallest change that fixes the material problems. Preserve the beat map, \
the copy and the loop. No new sections, features, words or numbers. Return \
only one complete fenced HTML document."
        );

        'passes: for pass in 1..=DEEP_PASSES {
            if Instant::now() >= deadline {
                providers::emit_event(
                    &self.event_sender,
                    "status",
                    "Deep review hit its 45-minute budget; keeping the last cut.",
                );
                break;
            }

            let mut rejection: Option<String> = None;
            for attempt in 0..2 {
                if Instant::now() >= deadline {
                    providers::emit_event(
                        &self.event_sender,
                        "status",
                        "Deep review hit its 45-minute budget; keeping the last cut.",
                    );
                    break 'passes;
                }

                let frame_note = format!(
                    "Four stills from the current cut are attached in order at {}. {}",
                    current_frames.labels.join(", "),
                    current_frames.report()
                );
                let review_user = build_review_prompt(
                    &req.prompt,
                    &req.mode,
                    &html,
                    pass,
                    DEEP_PASSES,
                    &frame_note,
                    rejection.as_deref(),
                    &quoted,
                );
                let quiet = broadcast::channel(16).0;

                providers::emit_event(
                    &self.event_sender,
                    "status",
                    if attempt == 0 {
                        format!("Deep pass {pass}/{DEEP_PASSES}: director review…")
                    } else {
                        format!("Deep pass {pass}/{DEEP_PASSES}: correction attempt…")
                    },
                );

                let result = providers::generate_with_images(
                    &self.http,
                    settings,
                    &review_system,
                    &review_user,
                    &current_frames.pngs,
                    &quiet,
                    workspace,
                )
                .await;

                let raw = match result {
                    Ok(Generated::Text(text)) => text,
                    Ok(Generated::ArtifactWritten) => {
                        providers::emit_event(
                            &self.event_sender,
                            "status",
                            "The agent wrote the artifact during review; stopping the loop.",
                        );
                        break 'passes;
                    }
                    Err(error) if !current_frames.pngs.is_empty() => {
                        providers::emit_event(
                            &self.event_sender,
                            "status",
                            format!(
                                "Visual review failed ({}); retrying as a code review…",
                                truncate(&error, 160)
                            ),
                        );
                        match providers::generate(
                            &self.http,
                            settings,
                            &review_system,
                            &review_user,
                            &quiet,
                            workspace,
                        )
                        .await
                        {
                            Ok(Generated::Text(text)) => text,
                            Ok(Generated::ArtifactWritten) => break 'passes,
                            Err(error) => {
                                providers::emit_event(
                                    &self.event_sender,
                                    "status",
                                    format!("Deep review stopped: {}", truncate(&error, 200)),
                                );
                                break 'passes;
                            }
                        }
                    }
                    Err(error) => {
                        providers::emit_event(
                            &self.event_sender,
                            "status",
                            format!("Deep review stopped: {}", truncate(&error, 200)),
                        );
                        break 'passes;
                    }
                };

                let Some(next) = extract_html(&raw) else {
                    providers::emit_event(
                        &self.event_sender,
                        "status",
                        format!(
                            "Deep pass {pass}/{DEEP_PASSES} returned no document; keeping the last cut."
                        ),
                    );
                    break 'passes;
                };

                let notes = raw.split("```").next().unwrap_or("").trim();
                if !notes.is_empty() {
                    providers::emit_event(
                        &self.event_sender,
                        "status",
                        format!("Deep pass {pass} notes: {}", truncate(notes, 240)),
                    );
                }

                let next = ensure_csp(&next);

                if next.len() > MAX_ARTIFACT_BYTES {
                    providers::emit_event(
                        &self.event_sender,
                        "status",
                        format!(
                            "Deep pass {pass}/{DEEP_PASSES} returned an oversized cut; keeping the last one."
                        ),
                    );
                    break 'passes;
                }
                if !html_is_self_contained(&next) {
                    providers::emit_event(
                        &self.event_sender,
                        "status",
                        format!(
                            "Deep pass {pass}/{DEEP_PASSES} introduced external references; keeping the last cut."
                        ),
                    );
                    break 'passes;
                }
                if next.trim() == html.trim() {
                    providers::emit_event(
                        &self.event_sender,
                        "status",
                        format!("Deep pass {pass}/{DEEP_PASSES} found no changes; stopping."),
                    );
                    break 'passes;
                }

                if let Err(reason) = copy_guard(&req.prompt, &html, &next) {
                    providers::emit_event(
                        &self.event_sender,
                        "status",
                        format!("Deep pass {pass}/{DEEP_PASSES} rejected: {reason}."),
                    );
                    if attempt == 0 {
                        rejection = Some(reason);
                        continue;
                    }
                    break 'passes;
                }

                providers::emit_event(
                    &self.event_sender,
                    "status",
                    format!("Deep pass {pass}/{DEEP_PASSES}: verifying the revised cut…"),
                );
                let candidate_frames = match frames::capture(&next, pass * 10 + attempt + 1).await {
                    Ok(captured) => captured,
                    Err(error) => {
                        warn!("Deep verification capture failed: {error}");
                        providers::emit_event(
                            &self.event_sender,
                            "status",
                            format!(
                                "Deep pass {pass}/{DEEP_PASSES} could not be verified ({}); keeping the last cut.",
                                truncate(&error, 160)
                            ),
                        );
                        break 'passes;
                    }
                };

                if let Err(reason) = legibility_guard(&current_frames, &candidate_frames) {
                    providers::emit_event(
                        &self.event_sender,
                        "status",
                        format!("Deep pass {pass}/{DEEP_PASSES} rejected: {reason}."),
                    );
                    if attempt == 0 {
                        rejection = Some(reason);
                        continue;
                    }
                    break 'passes;
                }

                html = next;
                current_frames = candidate_frames;
                applied += 1;
                providers::emit_event(
                    &self.event_sender,
                    "status",
                    format!(
                        "Deep pass {pass}/{DEEP_PASSES} applied · {} KB.",
                        html.len() / 1024
                    ),
                );
                continue 'passes;
            }
        }

        (html, applied)
    }

    fn write_artifact(&self, html: &str) -> Result<(), String> {
        let target = Path::new(&self.workspace_dir).join("index.html");
        fs::write(&target, html).map_err(|e| format!("Failed to write to workspace: {}", e))?;
        info!("Wrote artifact to {:?} ({} bytes)", target, html.len());
        Ok(())
    }

    /// When the agent writes `workspace/index.html` itself, it bypasses the
    /// model-text path (and therefore `ensure_csp`). Add the same offline CSP
    /// here — but only for self-contained documents, because `default-src
    /// 'none'` would break a deliberately multi-file artifact.
    fn harden_agent_artifact(&self) -> Result<Option<String>, String> {
        let target = Path::new(&self.workspace_dir).join("index.html");
        let html = fs::read_to_string(&target)
            .map_err(|error| format!("Failed to read the agent artifact: {error}"))?;
        if html
            .to_ascii_lowercase()
            .contains("content-security-policy")
        {
            return Ok(Some("Agent artifact already declares a CSP.".to_string()));
        }
        if !html_is_self_contained(&html) {
            info!("Agent artifact references local files; skipping CSP injection");
            return Ok(Some(
                "Agent wrote a multi-file artifact — inline CSP skipped.".to_string(),
            ));
        }
        let size = html.len();
        fs::write(&target, ensure_csp(&html))
            .map_err(|error| format!("Failed to harden the agent artifact: {error}"))?;
        info!("Hardened agent artifact with offline CSP ({size} bytes)");
        Ok(Some(format!(
            "Agent artifact hardened with offline CSP ({} KB).",
            size / 1024
        )))
    }
}

/// Heuristic used before injecting an inline CSP into an agent-written file:
/// inline styles and scripts plus `data:`/`blob:` assets are safe under
/// `default-src 'none'`; anything loaded from disk would break.
pub fn html_is_self_contained(html: &str) -> bool {
    let lower = html.to_ascii_lowercase();
    for tag in ["<script", "<img", "<video", "<audio", "<source", "<iframe"] {
        let mut rest = lower.as_str();
        while let Some(index) = rest.find(tag) {
            let element = &rest[index..];
            let element = &element[..element.find('>').unwrap_or(element.len())];
            if let Some(reference) = attribute_value(element, "src") {
                if !is_inline_reference(reference) {
                    return false;
                }
            }
            if tag == "<video" {
                if let Some(reference) = attribute_value(element, "poster") {
                    if !is_inline_reference(reference) {
                        return false;
                    }
                }
            }
            rest = &rest[index + tag.len()..];
        }
    }

    let mut rest = lower.as_str();
    while let Some(index) = rest.find("<link") {
        let element = &rest[index..];
        let element = &element[..element.find('>').unwrap_or(element.len())];
        let is_stylesheet = attribute_value(element, "rel")
            .map(|value| value.contains("stylesheet"))
            .unwrap_or(false);
        if is_stylesheet {
            if let Some(reference) = attribute_value(element, "href") {
                if !is_inline_reference(reference) {
                    return false;
                }
            }
        }
        rest = &rest[index + "<link".len()..];
    }

    true
}

/// Value of `name=` inside a single tag, honouring quotes and ignoring
/// lookalikes such as `data-src=`.
fn attribute_value<'a>(element: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!("{name}=");
    let mut search = element;
    while let Some(index) = search.find(&needle) {
        let boundary = index == 0 || search.as_bytes()[index - 1].is_ascii_whitespace();
        if boundary {
            let after = &search[index + needle.len()..];
            let (quote, rest) = match after.as_bytes().first() {
                Some(b'"') | Some(b'\'') => (after.as_bytes()[0], &after[1..]),
                _ => (0, after),
            };
            let value = if quote == 0 {
                let end = rest
                    .find(|c: char| c.is_ascii_whitespace() || c == '>')
                    .unwrap_or(rest.len());
                &rest[..end]
            } else {
                let end = rest.find(quote as char).unwrap_or(rest.len());
                &rest[..end]
            };
            return Some(value.trim());
        }
        search = &search[index + needle.len()..];
    }
    None
}

fn is_inline_reference(value: &str) -> bool {
    value.is_empty()
        || value.starts_with("data:")
        || value.starts_with("blob:")
        || value.starts_with('#')
}

/// System prompt for model-backed generation. The contract is deliberately
/// strict: one self-contained document, no network, motion means real motion.
///
/// The craft section exists because "make it beautiful" produces generic
/// output. Concrete constraints — a spacing scale, a type scale, contrast
/// floors, anti-patterns — are what separate a designed page from a template.
pub fn build_system_prompt(tokens: &DesignTokens, mode: &str) -> String {
    if mode == "motion" {
        return build_motion_prompt(tokens);
    }
    if mode == "slides" {
        return build_slides_prompt(tokens);
    }

    let mode_rules = match mode {
        _ => {
            "- Prototype mode: build an interactive product screen, not a marketing page. \
Use realistic data (names, numbers, statuses), working hover/click states, and tabs, \
filters or toggles where they help tell the story. Every control the design implies \
should do something visible."
        }
    };

    format!(
        r#"You are ZenDesign, a senior product designer and front-end engineer with an
editorial eye. You turn a design brief into one production-quality HTML artifact.

Design tokens for this brief (they are the palette — derive tints with color-mix() or
alpha, never invent new hues):
- background: {bg}
- text: {text}
- accent: {accent}
- font stack: {font}
- card background: {card_bg}
- border color: {border}

Craft standards — apply all of them:
- Direction: choose one clear art direction (editorial, soft-tech, brutalist, warm
  minimal, etc.) that fits the brand, and carry it through type, spacing, color and
  motion. When the brief is silent, default to restrained editorial minimalism.
- Composition: one idea per section and an obvious focal point. Use a 12-column grid
  inside a max-width container (1120–1240px) with 24px gutters. Generous whitespace;
  align edges; do not center every section. Vary density instead of repeating cards.
  (Slides mode: compose each 16:9 slide instead of a page.)
- Spacing: use one scale only — 4, 8, 12, 16, 24, 32, 48, 64, 96px. Section padding
  64–96px desktop / 40–56px mobile. No arbitrary values.
- Typography: at most two families from the token stack. Use a modular scale with
  clear jumps (e.g. 13/15/18/24/32/48/72). Display line-height 1.02–1.15, body
  1.5–1.65, measure 60–72ch. Tighten tracking on large headings (-0.02em to -0.04em).
  Create hierarchy with weight and space, not by using many sizes.
- Color and depth: the accent covers under 10% of the surface. Body text must clear
  4.5:1 contrast, large text 3:1. Build depth with hairline borders, one surface tint
  and at most one soft shadow level. No neon glow, rainbow gradients or glassmorphism
  unless the tokens themselves are neon.
- Content: write specific, confident copy — real product names, numbers, prices,
  dates, testimonial names, useful labels. No lorem ipsum, no "Your headline here",
  no emoji used as icons. Avoid clichés: revolutionize, seamless, unlock, elevate,
  next-generation.
- Interaction: every link, button, tab and input needs hover, focus-visible and active
  states. Use 150–250ms ease-out transitions. Controls must be keyboard reachable and
  use the correct element. Nothing may autoplay aggressively or trap focus.
- Responsive: no horizontal scrolling at 360px. Use clamp() for display type, collapse
  multi-column grids to one column, keep tap targets at least 44px. Check 390, 768,
  1280 and 1440 widths mentally before finishing.
- Code: valid semantic HTML5 with <title>, meta viewport and lang. One <style> and one
  <script>. Inline SVG icons with ~1.5px strokes. No libraries, no external requests,
  no console errors, keep the file under ~120KB.
- Avoid: a whole page of three identical feature cards; gradient blobs behind
  everything; shadows on every card; emoji as product icons; stock-photo placeholders;
  fixed heights that clip content; text over busy imagery.
- Finish: include only the sections the brief needs, but each must earn its place.
  A considered footer, one working interaction, and a coherent whole beat extra sections.

Output contract:
- Return ONE complete HTML document in a single ```html fenced code block. No commentary
  before or after the block.
- Everything inline: CSS in <style>, JS in <script>. No external requests whatsoever —
  no CDNs, no web fonts, no remote images. Inline SVG or data URIs only.
- If the brief cannot be satisfied, reply with a single line beginning with "ERROR:".

{mode_rules}"#,
        bg = tokens.bg,
        text = tokens.text,
        accent = tokens.accent,
        font = tokens.font_family,
        card_bg = tokens.card_bg,
        border = tokens.border_color,
        mode_rules = mode_rules,
    )
}

/// Motion mode is a different artifact type, not a website with animations. This
/// prompt replaces the page-composition contract entirely: no nav, sections,
/// pricing, FAQ, CTA or footer — a timed film on a fixed artboard instead.
fn build_motion_prompt(tokens: &DesignTokens) -> String {
    format!(
        r#"You are ZenDesign Motion: a motion designer and creative technologist. You turn a
brief into one self-playing motion piece that runs in the browser. You are not building a
website. You are building a short film made of HTML, CSS, SVG and JavaScript.

Design tokens (derive tints with color-mix() or alpha; never invent new hues):
- background: {bg}
- text: {text}
- accent: {accent}
- font stack: {font}
- card background: {card_bg}
- border: {border}

ARTIFACT — this overrides every website instinct:
- One fixed 16:9 artboard at 1440×900, centred in the viewport and scaled as a unit with a
  resize handler: transform: translate(-50%, -50%) scale(min(vw/1440, vh/900)). No page
  scroll. The composition never reflows; the frame scales like a film.
- NO nav bar, header, footer, hero, feature grid, pricing, testimonials, FAQ, contact form,
  CTA row or "sections". If the file contains these, it is the wrong artifact.
- The only chrome allowed: a tiny replay button and pause toggle in one corner plus a
  hairline progress bar. Understated, low contrast, no labels competing with the film.
- Autoplay immediately, muted, with no click required. Total runtime 8–12 seconds. Loop
  seamlessly, or hold the end card and restart cleanly. The first frame must work as a
  poster.
- 3–5 beats. Each beat has one focal element with a clear entrance and exit. Cut between
  beats with a mask, wipe, scale-through or a single hard cut — not just opacity fades.

STORY AND TYPE:
- One idea, 10–30 words of real copy total: a product name, a launch line, one number, a
  final mark. No lorem ipsum, no paragraphs, no feature lists, no fake UI.
- Kinetic typography is required. Use at least three techniques: masked line or word
  reveals, split-letter stagger, tracking that tightens or opens, outlined-to-filled type,
  vertical counter-scroll, or type that scales through the camera.
- Display type is the protagonist: 96–180px on the 1440×900 artboard, tight tracking,
  line-height 0.9–1.05. Keep every word inside a 64px safe margin.
- Include at least one non-type graphic event: an SVG path drawing itself, a counting
  statistic, bars growing, a dot field, a shape morph, a wireframe horizon or particle drift.

MOTION GRAMMAR:
- Build the timeline with CSS @keyframes and animation-delay, or the Web Animations API.
  Everything shares one clock so the loop has no visible seam.
- Entrances 400–800ms with cubic-bezier(0.16, 1, 0.3, 1); exits 250–450ms with
  cubic-bezier(0.7, 0, 0.84, 0); holds 500–1200ms; stagger siblings 40–80ms.
- Animate transform, opacity, clip-path and filter only. Never animate top/left/width/
  height. No huge blur() surfaces. Hold 60fps.
- Ambient motion (slow drift, a travelling hairline) may run underneath, but must not
  fight the beats. No full-frame grain, animated noise or canvas texture: it lowers type
  contrast and reads as a filter over the film. If a static texture is essential, keep it
  at or below 3% opacity and never over type. No flashing above 3Hz; no random per-frame
  jitter except a deliberate particle field.

COLOUR, TYPE, TASTE:
- One art direction, one accent moment. The background may invert once as a beat. Body
  text clears 4.5:1, display text 3:1. Hairline rules, generous negative space, optical
  centring.
- No glassmorphism, gradient blobs, neon glow unless the tokens are neon, and no emoji.
- Use only the token font stack. Inline SVG for marks and graphics. No external requests.

TECHNICAL:
- One complete HTML5 document, one <style> and one <script>. No libraries, CDNs, web fonts
  or remote images. Under ~100KB. No console errors.
- Autoplay must survive the preview iframe: no user-gesture APIs, no audio.
- prefers-reduced-motion: show the composed end frame as a static poster and keep replay
  working. Keyboard: Space pauses/resumes, R replays.

SELF-CHECK — do this before replying. If any answer is wrong, rewrite the file:
1. Does the HTML contain <nav>, <header>, <footer>, "pricing", "features", "FAQ" or a CTA
   row? Then it is a website. Delete those and rebuild it as a film.
2. Are there at least 3 distinct beats and 8+ animated elements on one timeline?
3. Does it autoplay, run 8–12s, and loop or hold the end card cleanly?
4. Would the first frame read as a poster for a motion piece, not a homepage?
5. Is every word legible at 1280×720?

Output contract:
- Return ONE complete HTML document in a single ```html fenced code block. No commentary.
- If the brief cannot be satisfied, reply with a single line beginning with "ERROR:"."#,
        bg = tokens.bg,
        text = tokens.text,
        accent = tokens.accent,
        font = tokens.font_family,
        card_bg = tokens.card_bg,
        border = tokens.border_color,
    )
}

/// Slides mode is a first-class presentation deck with strict content fidelity.
/// It respects the user's explicit slide count, sections, language direction (LTR/RTL),
/// and structures each slide with high-end editorial layouts.
fn build_slides_prompt(tokens: &DesignTokens) -> String {
    format!(
        r#"You are ZenDesign Slides: an elite presentation designer and front-end architect.
You turn the user's brief into a world-class, interactive 16:9 slide deck.

Design tokens (palette — derive tints with color-mix() or opacity; never invent conflicting hues):
- background: {bg}
- text: {text}
- accent: {accent}
- font stack: {font}
- card background: {card_bg}
- border: {border}

CORE PRESENTATION ARCHITECTURE:
1. STRICT SLIDE COUNT & STRUCTURE:
   - Carefully examine the user's brief for slide separation cues: numbered slides (e.g. "اسلاید ۱", "Slide 1"),
     bullet sections, paragraphs, dividers (---), or explicit slide counts.
   - If the user provides a specific text divided into N slides, you MUST produce EXACTLY N slides.
     DO NOT merge slides together and DO NOT skip any slide content.
   - If no explicit count is requested, parse the distinct ideas into a coherent 5–8 slide narrative.
   - Every single slide must have a distinct purpose and layout tailored to its content.

2. STAGE & SCALING (16:9 Full Viewport):
   - Fixed 16:9 canvas (1440×810 or 1280×720) centered in viewport and cleanly scaled with CSS:
     transform: translate(-50%, -50%) scale(min(vw / 1280, vh / 720));
   - Never allow any slide to vertically or horizontally overflow. If content is long, structure it into concise,
     punchy bullet cards or split columns, but PRESERVE all key facts, numbers, and messages.

3. EDITORIAL SLIDE LAYOUT DIVERSITY:
   Never make all slides look identical. Dynamically select the best layout for each slide:
   - Cover / Title Slide: Bold oversized display heading, subtle category badge, presenter / subtitle.
   - Metric & Big Stat Slide: Massive stat numbers (e.g. "+148%", "2.4M", "۰ تا ۱۰۰") with concise explanatory labels.
   - Split Comparison / Two-Column: Balanced juxtaposition (Before vs After, Problem vs Solution, Feature comparison).
   - Card Grid (3-4 items): Key takeaways or feature pillars with clean hairline borders and subtle hover lift.
   - Quote / Focal Statement: Large editorial quotation with highlighted accent keywords.
   - Summary / Conclusion: Clear roadmap, next steps or parting message.

4. PERSIAN & BILINGUAL (RTL/LTR) PERFECTION:
   - If the brief contains Persian / Arabic text, set `dir="rtl"` on the deck container or html.
   - When RTL is active:
     * Right arrow (ArrowRight) moves to the PREVIOUS slide, Left arrow (ArrowLeft) moves to the NEXT slide
       (or support intuitive natural flow with dual arrows and space bar).
     * Progress bar and slide counters must align naturally with RTL reading direction.
     * Use appropriate Persian typography weights and line heights (1.6–1.8 for Persian body text).

5. INTERACTION, KEYBOARD & CONTROLS:
   - Keyboard navigation: ArrowRight / ArrowLeft / Space / PageUp / PageDown.
   - Touch / Mouse navigation: Sleek, non-intrusive bottom navigation bar showing:
     * Slide indicator counter: e.g. "03 / 08" or "۳ از ۸"
     * Prev / Next subtle icon buttons
     * Hairline animated progress rail at the bottom edge.
     * Fullscreen toggle on key "f" or clicking a small fullscreen button.
   - Slide transitions: Fluid 250–350ms transitions (clean fade + subtle 12px translateX/Y slide).

6. TECHNICAL & OUTPUT CONTRACT:
   - Exactly ONE complete self-contained HTML5 file with all CSS in `<style>` and JavaScript in `<script>`.
   - NO external dependencies, no CDNs, no web fonts, no external image links. Use inline SVG icons where needed.
   - Return ONLY the ```html fenced code block with zero preambles or chat commentary."#,
        bg = tokens.bg,
        text = tokens.text,
        accent = tokens.accent,
        font = tokens.font_family,
        card_bg = tokens.card_bg,
        border = tokens.border_color,
    )
}

pub fn build_user_prompt(prompt: &str, design_system: &str, mode: &str) -> String {
    let artifact_type = match mode {
        "motion" => {
            "\nArtifact type: MOTION PIECE — an 8–12s self-playing film on a 16:9 artboard. \
NOT a website: no nav, sections, cards, pricing, FAQ, CTA or footer."
        }
        "slides" => {
            "\nArtifact type: SLIDE DECK (PRECISE SLIDE DECOMPOSITION) — full-viewport 16:9 presentation deck. \
Carefully respect any explicit slide separations, slide titles, numbered steps, or sections provided in the brief. \
Every designated slide must be faithfully represented as a separate, beautifully composed 16:9 slide."
        }
        _ => "",
    };

    format!(
        "Brief:\n{}\n\nDesign system id: {}\nMode: {}{}\n\nBuild the artifact now.",
        prompt.trim(),
        design_system,
        mode,
        artifact_type
    )
}

/// The user turn for a deep-review pass. It carries the original brief, the
/// current document, what the attached stills represent, and any reason the
/// previous revision was rejected.
fn build_review_prompt(
    brief: &str,
    mode: &str,
    current: &str,
    pass: usize,
    total: usize,
    frame_note: &str,
    rejection: Option<&str>,
    quoted: &[String],
) -> String {
    let lens = match mode {
        "motion" => {
            "Judge it as a motion director. The first frame must work as a poster; there must be \
3–5 distinct beats, kinetic typography, at least one graphic event, a clean loop seam, and no \
website chrome. Check whether the type is composed, not merely placed. Full-frame grain, \
animated noise or a canvas texture that lowers type contrast is a material problem: remove it."
        }
        "slides" => {
            "Judge it as a presentation designer. Each 16:9 slide should make one point; \
navigation, counter and transitions should stay quiet; no slide may overflow."
        }
        _ => {
            "Judge it as a product design director. Check hierarchy, spacing, real data, \
interaction states, contrast, and whether it reads as a real product rather than a template."
        }
    };

    let required = if quoted.is_empty() {
        "No extra copy. Keep every existing word, number, label and beat.".to_string()
    } else {
        format!(
            "Required copy (must remain verbatim): {}.",
            quoted
                .iter()
                .map(|phrase| format!("“{phrase}”"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let rejection = rejection
        .map(|reason| {
            format!(
                "\nThe previous revision was rejected because {reason}. Fix that specifically in \
this attempt; do not regress the rest of the cut.\n"
            )
        })
        .unwrap_or_default();

    format!(
        "Original brief:\n{brief}\n\nMode: {mode}\nRefinement pass {pass} of {total}.\n\
{frame_note}\n\n{lens}\n\n<current_cut>\n{current}\n</current_cut>\n\n\
NON-NEGOTIABLE:\n\
- {required}\n\
- Keep the same beat count and beat order. Do not add counters, stats, subtitles or a new brand \
line. Do not restructure the film.\n\
- Every frame must have visible contrast. If a frame reads near zero in the luma report, that \
beat is blank or white-on-white — fix it.\n\
- The last frame must return to the first composition for a clean loop.\n\
{rejection}\n\
First decide whether the cut has a material problem. Material means a beat is unreadable or \
blank, required copy is missing or invisible, the loop jumps, layout collides, the brief is \
contradicted, or a texture/filter is obscuring the type. Taste is not material. If there is no \
material problem, return the current document unchanged.\n\n\
Do this in order:\n\
1. Name up to three material problems with the current cut. One line each. Be specific about \
what is on screen, not generic advice.\n\
2. If there are material problems, rewrite the entire document to fix them with the smallest \
change that works. Keep the parts that already work. If there are none, return the current \
document unchanged. Do not add navigation, sections, cards, pricing, FAQ, CTA, footer or extra \
copy. Keep it self-contained and inline-only.\n\n\
Before returning, verify: all required copy is present; no new words or numbers appeared; every \
frame has readable contrast; the loop returns to the first beat.\n\n\
Return only the complete revised HTML in one ```html fenced block. No commentary before or after it."
    )
}

/// Phrases the brief puts in quotes are locked copy. If a revision drops one,
/// it is rejected before it can overwrite the preview.
fn quoted_phrases(brief: &str) -> Vec<String> {
    let mut phrases = Vec::new();
    for (open, close) in [('"', '"'), ('“', '”'), ('‘', '’')] {
        let mut rest = brief;
        while let Some(start) = rest.find(open) {
            let after = &rest[start + open.len_utf8()..];
            let Some(end) = after.find(close) else { break };
            let phrase = after[..end].trim();
            if phrase.chars().count() >= 4 && phrase.chars().any(|c| c.is_alphabetic()) {
                phrases.push(phrase.to_string());
            }
            rest = &after[end + close.len_utf8()..];
        }
    }
    phrases.sort();
    phrases.dedup();
    phrases
}

/// Remove an element and its contents so scripts and styles do not pollute the
/// visible-copy comparison. ASCII lowercasing preserves byte offsets.
fn strip_tag_block(html: &str, tag: &str) -> String {
    let lower = html.to_ascii_lowercase();
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let mut out = String::with_capacity(html.len());
    let mut cursor = 0usize;

    while let Some(relative) = lower[cursor..].find(&open) {
        let start = cursor + relative;
        let boundary = lower[start + open.len()..].chars().next();
        if !matches!(
            boundary,
            Some('>') | Some('/') | Some(' ') | Some('\n') | Some('\r') | Some('\t')
        ) {
            out.push_str(&html[cursor..start + 1]);
            cursor = start + 1;
            continue;
        }
        out.push_str(&html[cursor..start]);
        if let Some(end_relative) = lower[start..].find(&close) {
            cursor = start + end_relative + close.len();
        } else {
            cursor = html.len();
            break;
        }
    }
    out.push_str(&html[cursor..]);
    out
}

fn strip_comments(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(start) = rest.find("<!--") {
        out.push_str(&rest[..start]);
        if let Some(end) = rest[start..].find("-->") {
            rest = &rest[start + end + 3..];
        } else {
            rest = "";
            break;
        }
    }
    out.push_str(rest);
    out
}

fn flush_text_token(
    token: &mut String,
    words: &mut HashSet<String>,
    numbers: &mut HashSet<String>,
) {
    if token.is_empty() {
        return;
    }
    let lower = token.to_ascii_lowercase();
    if lower.chars().all(|c| c.is_ascii_digit()) {
        if lower.len() >= 2 {
            numbers.insert(lower);
        }
    } else if lower.len() >= 2 && lower.chars().any(|c| c.is_ascii_alphabetic()) {
        words.insert(lower);
    }
    token.clear();
}

/// Visible words and numbers, ignoring tags, scripts, styles and comments.
fn visible_tokens(html: &str) -> (HashSet<String>, HashSet<String>) {
    let mut text = strip_tag_block(html, "script");
    text = strip_tag_block(&text, "style");
    text = strip_comments(&text);

    let mut words = HashSet::new();
    let mut numbers = HashSet::new();
    let mut token = String::new();
    let mut in_tag = false;

    for ch in text.chars() {
        match ch {
            '<' => {
                flush_text_token(&mut token, &mut words, &mut numbers);
                in_tag = true;
            }
            '>' => in_tag = false,
            _ if in_tag => {}
            _ if ch.is_ascii_alphanumeric() => token.push(ch.to_ascii_lowercase()),
            _ => flush_text_token(&mut token, &mut words, &mut numbers),
        }
    }
    flush_text_token(&mut token, &mut words, &mut numbers);
    (words, numbers)
}

/// Reject a revision that drops locked copy, invents words or numbers, or
/// removes a large share of the existing copy. A refinement should change
/// composition and timing, not the script.
fn copy_guard(brief: &str, baseline: &str, candidate: &str) -> Result<(), String> {
    let candidate_lower = candidate.to_ascii_lowercase();
    for phrase in quoted_phrases(brief) {
        if !candidate_lower.contains(&phrase.to_ascii_lowercase()) {
            return Err(format!("it drops the required line “{phrase}”"));
        }
    }

    let (base_words, base_numbers) = visible_tokens(baseline);
    let (candidate_words, candidate_numbers) = visible_tokens(candidate);
    let (brief_words, brief_numbers) = visible_tokens(brief);

    let mut allowed_words = base_words.clone();
    allowed_words.extend(brief_words);
    let mut new_words: Vec<&String> = candidate_words
        .iter()
        .filter(|word| word.chars().count() >= 4 && !allowed_words.contains(*word))
        .collect();
    new_words.sort();
    // One connective word can be a legitimate fix; a second is a rewrite of the
    // script. Required copy is checked separately above.
    if new_words.len() > 1 {
        let invented = new_words
            .iter()
            .map(|word| word.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!("it invents the words “{invented}”"));
    }

    let mut allowed_numbers = base_numbers;
    allowed_numbers.extend(brief_numbers);
    let mut new_numbers: Vec<&String> = candidate_numbers
        .iter()
        .filter(|number| !allowed_numbers.contains(*number))
        .collect();
    new_numbers.sort();
    if let Some(number) = new_numbers.first() {
        return Err(format!("it invents the number “{number}”"));
    }

    let mut missing: Vec<&String> = base_words
        .iter()
        .filter(|word| !candidate_words.contains(*word))
        .collect();
    missing.sort();
    let threshold = (base_words.len() / 3).max(2);
    if !missing.is_empty() && missing.len() >= threshold {
        let missing_text = missing
            .iter()
            .map(|word| word.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        return Err(format!("it drops the existing copy “{missing_text}”"));
    }

    Ok(())
}

/// A frame whose luma standard deviation is tiny is effectively blank. Also
/// reject a revision whose flattest frame is much flatter than the current
/// cut's flattest frame, which catches a title fading into its background.
fn legibility_guard(
    previous: &frames::CapturedFrames,
    candidate: &frames::CapturedFrames,
) -> Result<(), String> {
    let next = candidate.min_contrast();
    if next < 8.0 {
        return Err(format!(
            "the flattest frame has contrast {next:.1}/255, which is nearly blank"
        ));
    }
    let prior = previous.min_contrast();
    if prior > 12.0 && next < prior * 0.6 {
        return Err(format!(
            "contrast collapses from {prior:.1}/255 to {next:.1}/255"
        ));
    }
    Ok(())
}

/// Pull the HTML document out of a model reply: first fenced block wins, then
/// the first `<!DOCTYPE`/`<html>` … `</html>` span.
pub fn extract_html(raw: &str) -> Option<String> {
    let lower = raw.to_ascii_lowercase();
    let mut best_doc: Option<String> = None;
    let mut largest: Option<String> = None;
    let mut cursor = 0usize;

    while let Some(relative) = raw[cursor..].find("```") {
        let fence = cursor + relative;
        let Some(line_end) = raw[fence + 3..].find('\n') else {
            break;
        };
        let body_start = fence + 3 + line_end + 1;
        let Some(end) = raw[body_start..].find("```") else {
            break;
        };
        let block = raw[body_start..body_start + end].trim().to_string();
        if !block.is_empty() {
            let doc = block.to_ascii_lowercase();
            if (doc.contains("<html") || doc.contains("<!doctype")) && best_doc.is_none() {
                best_doc = Some(block.clone());
            }
            if largest
                .as_ref()
                .map_or(true, |current| block.len() > current.len())
            {
                largest = Some(block);
            }
        }
        cursor = body_start + end + 3;
    }

    if best_doc.is_some() {
        return best_doc;
    }
    if let Some(block) = largest {
        let doc = block.to_ascii_lowercase();
        if doc.contains("<html") || doc.contains("<!doctype") {
            return Some(block);
        }
    }

    let start = lower.find("<!doctype").or_else(|| lower.find("<html"))?;
    let end = lower
        .rfind("</html>")
        .map(|index| index + 7)
        .unwrap_or(raw.len());
    if end <= start {
        return None;
    }
    Some(raw[start..end].trim().to_string())
}

/// Add a CSP meta tag if the model did not provide one. `connect-src 'none'`
/// blocks fetch/EventSource/WebSocket from the artifact, so model output cannot
/// call back into the daemon or anywhere else.
pub fn ensure_csp(html: &str) -> String {
    if html
        .to_ascii_lowercase()
        .contains("content-security-policy")
    {
        return html.to_string();
    }
    let lower = html.to_ascii_lowercase();
    if let Some(position) = lower.find("<head>") {
        let at = position + "<head>".len();
        return format!("{}{}\n{}", &html[..at], CSP_META, &html[at..]);
    }
    if let Some(position) = lower.find("<html") {
        if let Some(offset) = html[position..].find('>') {
            let at = position + offset + 1;
            return format!("{}{}\n{}", &html[..at], CSP_META, &html[at..]);
        }
    }
    format!("{CSP_META}\n{html}")
}

pub fn generate_prototype_html(prompt: &str, design_system: &str, mode: &str) -> String {
    let tokens = design_tokens(design_system);
    let bg = tokens.bg;
    let text = tokens.text;
    let accent = tokens.accent;
    let font_family = tokens.font_family;
    let card_bg = tokens.card_bg;
    let border_color = tokens.border_color;

    let headline = escape_html(&headline_from_prompt(prompt));
    let brief = escape_html(&truncate_words(
        if prompt.trim().is_empty() {
            "A modern collaborative workspace, rendered live by the ZenDesign agent."
        } else {
            prompt.trim()
        },
        260,
    ));
    let design_system = escape_html(design_system);
    let mode = escape_html(mode);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    {csp}
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ZenDesign Live Prototype</title>
    <style>{preview_css}</style>
    <style>
        body {{
            background-color: {bg};
            color: {text};
            font-family: {font_family};
        }}
        .custom-card {{
            background-color: {card_bg};
            border: 1px solid {border_color};
            backdrop-filter: blur(12px);
        }}
        .custom-btn {{
            background-color: {accent};
            color: #ffffff;
            transition: all 0.2s ease;
        }}
        .custom-btn:hover {{
            opacity: 0.9;
            transform: translateY(-2px);
        }}
    </style>
</head>
<body class="min-h-screen flex flex-col justify-between p-6 md:p-12 selection:bg-purple-500 selection:text-white">
    <!-- Header -->
    <header class="flex flex-wrap justify-between items-center gap-4 pb-8 border-b" style="border-color: {border_color};">
        <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-lg flex items-center justify-center font-bold text-lg shadow-lg" style="background-color: {accent}; color: white;">
                Z
            </div>
            <span class="font-bold text-xl tracking-tight">ZenDesign AI</span>
        </div>
        <div class="flex flex-wrap items-center gap-2">
            <span class="text-xs px-3 py-1 rounded-full uppercase tracking-wider font-semibold custom-card">
                DS: {design_system}
            </span>
            <span class="text-xs px-3 py-1 rounded-full uppercase tracking-wider font-semibold custom-card">
                Mode: {mode}
            </span>
        </div>
    </header>

    <!-- Hero Content -->
    <main class="max-w-4xl mx-auto my-12 text-center flex flex-col items-center">
        <div class="inline-block px-4 py-1.5 rounded-full text-xs font-semibold uppercase tracking-widest mb-6 custom-card" style="color: {accent};">
            Generated with ZenDesign Fast Engine
        </div>
        <h1 class="text-4xl md:text-5xl font-extrabold tracking-tight mb-6 leading-tight">
            {headline}
        </h1>
        <p class="text-base md:text-lg opacity-75 max-w-2xl mb-10 leading-relaxed">
            {brief}
        </p>

        <div class="flex flex-wrap gap-4 justify-center">
            <button class="custom-btn px-8 py-3.5 rounded-xl font-semibold shadow-lg text-sm">
                Explore Prototype
            </button>
            <button class="custom-card px-8 py-3.5 rounded-xl font-semibold text-sm hover:opacity-80 transition">
                Inspect Tokens
            </button>
        </div>

        <!-- Metric Grid -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 w-full mt-16 text-left">
            <div class="custom-card p-6 rounded-2xl">
                <div class="text-3xl font-black mb-1" style="color: {accent};">15 MB</div>
                <div class="font-medium text-sm">Daemon Memory Footprint</div>
                <div class="text-xs opacity-60 mt-2">Compiled native Rust without Node.js GC overhead.</div>
            </div>
            <div class="custom-card p-6 rounded-2xl">
                <div class="text-3xl font-black mb-1" style="color: {accent};">300 ms</div>
                <div class="font-medium text-sm">Debounced Reload</div>
                <div class="text-xs opacity-60 mt-2">Smooth, flicker-free updates when saving files.</div>
            </div>
            <div class="custom-card p-6 rounded-2xl">
                <div class="text-3xl font-black mb-1" style="color: {accent};">0 ms</div>
                <div class="font-medium text-sm">Reasoning Timeout</div>
                <div class="text-xs opacity-60 mt-2">SSE keep-alive prevents any browser disconnect.</div>
            </div>
        </div>
    </main>

    <!-- Footer -->
    <footer class="text-center text-xs opacity-50 pt-8 border-t" style="border-color: {border_color};">
        ZenDesign Local Workspace &bull; Real-time synchronized with Rust Daemon
    </footer>
</body>
</html>"#,
        bg = bg,
        text = text,
        accent = accent,
        font_family = font_family,
        card_bg = card_bg,
        border_color = border_color,
        csp = CSP_META,
        headline = headline,
        brief = brief,
        design_system = design_system,
        mode = mode,
        preview_css = PREVIEW_CSS
    )
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Turn an imperative design brief into a short, display-worthy headline.
fn headline_from_prompt(prompt: &str) -> String {
    let trimmed = prompt.trim();
    if trimmed.is_empty() {
        return "Modern Collaborative Agent Workspace".to_string();
    }

    let first_sentence = trimmed
        .split(['.', '!', '?', '\n'])
        .next()
        .unwrap_or(trimmed)
        .trim();

    let lower = first_sentence.to_lowercase();
    let mut core = first_sentence;
    for prefix in [
        "design a ",
        "design an ",
        "create a ",
        "create an ",
        "build a ",
        "build an ",
        "make a ",
        "make an ",
        "generate a ",
        "generate an ",
    ] {
        if lower.starts_with(prefix) {
            core = first_sentence[prefix.len()..].trim();
            break;
        }
    }

    let core = if core.is_empty() {
        first_sentence
    } else {
        core
    };
    let mut headline = truncate_words(core, 76);

    if let Some(first) = headline.chars().next() {
        let upper: String = first.to_uppercase().collect();
        headline.replace_range(0..first.len_utf8(), &upper);
    }

    headline
}

fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let cut: String = text.chars().take(max_chars).collect();
    format!("{cut}…")
}

fn truncate_words(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }

    let mut out = String::new();
    for word in text.split_whitespace() {
        let separator = usize::from(!out.is_empty());
        if out.chars().count() + word.chars().count() + separator > max_chars {
            break;
        }
        if separator == 1 {
            out.push(' ');
        }
        out.push_str(word);
    }

    if out.is_empty() {
        out = text.chars().take(max_chars).collect();
    }

    format!("{out}…")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_artifact_is_self_contained() {
        let html = generate_prototype_html("Revenue dashboard", "stripe", "prototype");
        assert!(html.contains("<style>"));
        assert!(!html.contains("cdn.tailwindcss.com"));
        assert!(!html.contains("<script src="));
    }

    #[test]
    fn generated_artifact_escapes_prompt_html() {
        let html = generate_prototype_html(
            "Build a <script>alert(1)</script> dashboard for my account",
            "stripe",
            "prototype",
        );
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script>alert(1)</script>"));
    }

    #[test]
    fn generated_artifact_uses_selected_tokens() {
        let html = generate_prototype_html("Motion launch", "cyberpunk", "motion");
        assert!(html.contains("#ff0055"));
        assert!(html.contains("DS: cyberpunk"));
        assert!(html.contains("Mode: motion"));
    }

    #[test]
    fn extracts_fenced_document_from_chatty_reply() {
        let raw = "Sure! Here is the artifact:\n\n```html\n<!DOCTYPE html>\n<html><body><h1>Hi</h1></body></html>\n```\nEnjoy!";
        let html = extract_html(raw).expect("fenced html");
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.ends_with("</html>"));
    }

    #[test]
    fn extracts_raw_document_without_fences() {
        let raw = "preamble\n<!DOCTYPE html>\n<html><body>ok</body></html>\ntrailing";
        let html = extract_html(raw).expect("raw html");
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.ends_with("</html>"));
    }

    #[test]
    fn rejects_replies_without_html() {
        assert!(extract_html("I cannot build that.").is_none());
    }

    #[test]
    fn csp_is_added_once_and_blocks_network() {
        let html = ensure_csp("<html><head></head><body></body></html>");
        assert!(html.contains("connect-src 'none'"));
        assert_eq!(html.matches("Content-Security-Policy").count(), 1);
        let again = ensure_csp(&html);
        assert_eq!(again.matches("Content-Security-Policy").count(), 1);
    }

    #[test]
    fn self_contained_docs_are_detected() {
        let inline = "<html><head><style>body{}</style></head><body><img src=\"data:image/png;base64,AA\"><script>ok()</script></body></html>";
        assert!(html_is_self_contained(inline));

        // Attribute lookalikes must not fool the scanner.
        assert!(html_is_self_contained(
            "<html><body><img data-src=\"hero.png\" src=\"data:image/png;base64,AA\"></body></html>"
        ));
    }

    #[test]
    fn multi_file_docs_skip_csp_injection() {
        let linked =
            "<html><head><link rel=\"stylesheet\" href=\"style.css\"></head><body></body></html>";
        assert!(!html_is_self_contained(linked));

        let scripted = "<html><head></head><body><script src=\"app.js\"></script></body></html>";
        assert!(!html_is_self_contained(scripted));

        let pictured = "<html><body><img src=\"assets/hero.png\"></body></html>";
        assert!(!html_is_self_contained(pictured));
    }

    #[test]
    fn hardened_docs_stay_self_contained() {
        let inline = "<html><head><style>body{}</style></head><body>hi</body></html>";
        let hardened = ensure_csp(inline);
        assert!(html_is_self_contained(&hardened));
    }

    #[test]
    fn system_prompt_for_motion_demands_animation() {
        let tokens = design_tokens("stripe");
        let prompt = build_system_prompt(&tokens, "motion");
        assert!(prompt.contains("@keyframes"));
        assert!(prompt.contains("prefers-reduced-motion"));
        assert!(prompt.contains("#635bff"));
        assert!(prompt.contains("No full-frame grain"));
        assert!(prompt.contains("3% opacity"));
    }

    #[test]
    fn motion_prompt_replaces_the_website_contract() {
        let prompt = build_system_prompt(&design_tokens("stripe"), "motion");
        assert!(prompt.contains("not building a\nwebsite"));
        assert!(prompt.contains("16:9"));
        assert!(prompt.contains("NO nav bar"));
        assert!(prompt.contains("8–12 seconds"));
        assert!(!prompt.contains("A considered footer"));

        let user = build_user_prompt("NOVA launch", "stripe", "motion");
        assert!(user.contains("MOTION PIECE"));
        assert!(user.contains("NOT a website"));
    }

    #[test]
    fn deep_review_defaults_to_one_shot() {
        let one_shot: PromptRequest =
            serde_json::from_str(r#"{"prompt":"NOVA","design_system":"stripe","mode":"motion"}"#)
                .unwrap();
        assert!(!one_shot.deep);

        let deep: PromptRequest = serde_json::from_str(
            r#"{"prompt":"NOVA","design_system":"stripe","mode":"motion","deep":true}"#,
        )
        .unwrap();
        assert!(deep.deep);
    }

    #[test]
    fn review_prompt_is_grounded_in_the_current_cut() {
        let prompt = build_review_prompt(
            "NOVA title film",
            "motion",
            "<html><body>cut</body></html>",
            2,
            3,
            "Four stills attached at ~0.9s, ~3.5s, ~6.5s, ~9.4s.",
            None,
            &["Space, folded.".to_string()],
        );
        assert!(prompt.contains("NOVA title film"));
        assert!(prompt.contains("Refinement pass 2 of 3"));
        assert!(prompt.contains("material problem"));
        assert!(prompt.contains("<current_cut>"));
        assert!(prompt.contains("no website chrome"));
        assert!(prompt.contains("Required copy"));
        assert!(prompt.contains("Space, folded."));
        assert!(prompt.contains("Return only the complete revised HTML"));
    }

    #[test]
    fn locked_copy_rejects_a_replacement() {
        let brief = r#"A film. The line "Space, folded." appears at the end."#;
        let baseline = "<html><body><h1>NOVA</h1><p>Space, folded.</p></body></html>";
        let candidate = "<html><body><h1>NOVA</h1><p>VELOCITY</p></body></html>";
        let error = copy_guard(brief, baseline, candidate).unwrap_err();
        assert!(error.contains("Space, folded"), "{error}");
    }

    #[test]
    fn copy_guard_allows_a_style_only_revision() {
        let brief = r#"A film. The line "Space, folded." appears."#;
        let baseline = "<html><body><h1>NOVA</h1><p>Space, folded.</p></body></html>";
        let candidate =
            "<html><body><h1 style=\"color:red\">NOVA</h1><p>Space, folded.</p></body></html>";
        assert!(copy_guard(brief, baseline, candidate).is_ok());
    }

    #[test]
    fn copy_guard_rejects_invented_numbers() {
        let baseline = "<div>NOVA</div><div>Space, folded.</div>";
        let candidate = "<div>NOVA</div><div>Space, folded.</div><div>41</div>";
        let error = copy_guard("A film", baseline, candidate).unwrap_err();
        assert!(error.contains("41"), "{error}");
    }

    #[test]
    fn copy_guard_allows_one_connective_word() {
        let baseline = "<div>NOVA</div><div>Space, folded.</div>";
        let candidate = "<div>NOVA</div><div>arrive. Space, folded.</div>";
        assert!(copy_guard("A film", baseline, candidate).is_ok());
    }

    #[test]
    fn copy_guard_rejects_two_invented_words() {
        let baseline = "<div>NOVA</div><div>Space, folded.</div>";
        let candidate = "<div>NOVA</div><div>Ideas arrive. Space, folded.</div><div>Velocity</div>";
        let error = copy_guard("A film", baseline, candidate).unwrap_err();
        assert!(error.contains("invents the words"), "{error}");
    }

    #[test]
    fn visible_tokens_ignore_scripts_and_tags() {
        let (words, numbers) = visible_tokens(
            "<style>.x{color:red}</style><div data-x=\"9\">NOVA 2026</div>\
             <script>const velocity=1</script>",
        );
        assert!(words.contains("nova"));
        assert!(numbers.contains("2026"));
        assert!(!words.contains("velocity"));
        assert!(!words.contains("color"));
    }

    #[test]
    fn legibility_guard_rejects_a_blank_frame() {
        let frames = |stddev: f64| frames::CapturedFrames {
            pngs: Vec::new(),
            labels: Vec::new(),
            stats: vec![frames::FrameStats { mean: 20.0, stddev }; 4],
        };
        assert!(legibility_guard(&frames(30.0), &frames(2.0)).is_err());
        assert!(legibility_guard(&frames(30.0), &frames(25.0)).is_ok());
    }
}
