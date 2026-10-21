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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomThemeTokens {
    pub name: String,
    pub bg: String,
    pub text: String,
    pub accent: String,
    pub font_family: String,
    pub card_bg: String,
    pub border_color: String,
    #[serde(default = "default_theme_dot")]
    pub dot: String,
    #[serde(default = "default_theme_swatch")]
    pub swatch: String,
}

fn default_theme_dot() -> String {
    "#6366f1".to_string()
}

fn default_theme_swatch() -> String {
    "linear-gradient(135deg, #6366f1 0%, #0f172a 100%)".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
    pub design_system: String, // e.g. "stripe", "linear", "cyberpunk", "swiss", "custom"
    pub mode: String,          // e.g. "prototype", "slides", "motion"
    /// Render, review and rewrite the artifact up to `DEEP_PASSES` times.
    #[serde(default)]
    pub deep: bool,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub custom_theme: Option<CustomThemeTokens>,
    #[serde(default)]
    pub aspect_ratio: Option<String>,
    #[serde(default)]
    pub duration_secs: Option<u32>,
    #[serde(default)]
    pub sound_effects: Option<bool>,
    #[serde(default)]
    pub tts_text: Option<String>,
    #[serde(default)]
    pub tts_lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    pub event_type: String, // "status", "token", "writing", "done", "error"
    pub message: String,
}

/// Design-system tokens shared by the offline template and the model prompt.
#[derive(Debug, Clone)]
pub struct DesignTokens {
    pub bg: String,
    pub text: String,
    pub accent: String,
    pub font_family: String,
    pub card_bg: String,
    pub border_color: String,
}

pub fn design_tokens(design_system: &str) -> DesignTokens {
    match design_system {
        "linear" => DesignTokens {
            bg: "#0d0e12".to_string(),
            text: "#f3f4f6".to_string(),
            accent: "#5e6ad2".to_string(),
            font_family: "system-ui, -apple-system, sans-serif".to_string(),
            card_bg: "#161822".to_string(),
            border_color: "#2b2d3d".to_string(),
        },
        "cyberpunk" => DesignTokens {
            bg: "#0a0a14".to_string(),
            text: "#00ffcc".to_string(),
            accent: "#ff0055".to_string(),
            font_family: "'Courier New', monospace".to_string(),
            card_bg: "#14142b".to_string(),
            border_color: "#ff0055".to_string(),
        },
        "swiss" => DesignTokens {
            bg: "#ffffff".to_string(),
            text: "#111111".to_string(),
            accent: "#e63946".to_string(),
            font_family: "Helvetica Neue, Arial, sans-serif".to_string(),
            card_bg: "#f8f9fa".to_string(),
            border_color: "#111111".to_string(),
        },
        "nordic" => DesignTokens {
            bg: "#0f172a".to_string(),
            text: "#e2e8f0".to_string(),
            accent: "#10b981".to_string(),
            font_family: "'Plus Jakarta Sans', system-ui, sans-serif".to_string(),
            card_bg: "#1e293b".to_string(),
            border_color: "#334155".to_string(),
        },
        "emerald" => DesignTokens {
            bg: "#022c22".to_string(),
            text: "#ecfdf5".to_string(),
            accent: "#059669".to_string(),
            font_family: "'Cinzel', 'Playfair Display', serif".to_string(),
            card_bg: "#064e3b55".to_string(),
            border_color: "#047857".to_string(),
        },
        "terminal" => DesignTokens {
            bg: "#050505".to_string(),
            text: "#22c55e".to_string(),
            accent: "#eab308".to_string(),
            font_family: "'JetBrains Mono', 'Fira Code', monospace".to_string(),
            card_bg: "#0d1117".to_string(),
            border_color: "#16a34a40".to_string(),
        },
        "sunset" => DesignTokens {
            bg: "#2b0938".to_string(),
            text: "#fef2f2".to_string(),
            accent: "#f97316".to_string(),
            font_family: "'Outfit', system-ui, sans-serif".to_string(),
            card_bg: "#4c0519aa".to_string(),
            border_color: "#f43f5e55".to_string(),
        },
        "brutalist" => DesignTokens {
            bg: "#fffbeb".to_string(),
            text: "#000000".to_string(),
            accent: "#eab308".to_string(),
            font_family: "'Space Grotesk', Impact, sans-serif".to_string(),
            card_bg: "#ffffff".to_string(),
            border_color: "#000000".to_string(),
        },
        "glass" => DesignTokens {
            bg: "#09090b".to_string(),
            text: "#fafafa".to_string(),
            accent: "#8b5cf6".to_string(),
            font_family: "'Inter', system-ui, sans-serif".to_string(),
            card_bg: "rgba(255, 255, 255, 0.05)".to_string(),
            border_color: "rgba(255, 255, 255, 0.15)".to_string(),
        },
        _ => DesignTokens {
            // Stripe default
            bg: "#0a2540".to_string(),
            text: "#ffffff".to_string(),
            accent: "#635bff".to_string(),
            font_family: "'Inter', system-ui, sans-serif".to_string(),
            card_bg: "#ffffff10".to_string(),
            border_color: "#ffffff20".to_string(),
        },
    }
}

pub fn design_tokens_with_custom(
    design_system: &str,
    custom: Option<&CustomThemeTokens>,
) -> DesignTokens {
    if design_system == "custom" {
        if let Some(c) = custom {
            return DesignTokens {
                bg: c.bg.clone(),
                text: c.text.clone(),
                accent: c.accent.clone(),
                font_family: c.font_family.clone(),
                card_bg: c.card_bg.clone(),
                border_color: c.border_color.clone(),
            };
        }
    }
    design_tokens(design_system)
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

    pub async fn execute_prompt(&self, mut req: PromptRequest) -> Result<(), String> {
        let settings = self.settings.read().await.clone();

        // 1. SMART WEB & GITHUB INSPECTOR: Detect any URLs in the prompt and fetch live content
        if let Some(url) = extract_first_url(&req.prompt) {
            providers::emit_event(
                &self.event_sender,
                "status",
                format!("🔍 Detecting URL in prompt: {url} — fetching live repository/site context…"),
            );
            if let Ok(web_context) = fetch_url_context(&self.http, &url).await {
                providers::emit_event(
                    &self.event_sender,
                    "status",
                    format!("📄 Extracted context from {url} ({} bytes). Integrating into prompt…", web_context.len()),
                );
                req.prompt = format!(
                    "{}\n\n[INSPECTED LIVE CONTEXT FROM {}]:\n{}",
                    req.prompt, url, web_context
                );
            }
        }

        // 2. CONCEPT & NARRATIVE EXPANSION (Deep Research Agent):
        // If prompt is short/conceptual (e.g. "Presentation on Event-Driven Architecture", "SaaS for accountants"),
        // or specifically in Slides/Motion mode, expand it into a comprehensive structured breakdown.
        if settings.provider == PROVIDER_OPENAI || settings.provider == PROVIDER_ANTHROPIC {
            let is_short_brief = req.prompt.lines().count() <= 3 && req.prompt.len() < 220;
            let needs_expansion = is_short_brief || req.mode == "slides" || req.mode == "motion";
            
            if needs_expansion {
                if req.mode == "motion" {
                    providers::emit_event(
                        &self.event_sender,
                        "status",
                        "🎬 Motion Creative Director: Choreographing cinematic storyboard, visual beats & kinetic typography…",
                    );
                } else {
                    providers::emit_event(
                        &self.event_sender,
                        "status",
                        "🧠 Autonomous Research Agent: Analyzing domain, extracting core concepts, metrics & narrative structure…",
                    );
                }
                if let Ok(expanded) = expand_brief_with_research(&self.http, &settings, &req.prompt, &req.mode, req.duration_secs.unwrap_or(10)).await {
                    providers::emit_event(
                        &self.event_sender,
                        "status",
                        if req.mode == "motion" {
                            "✨ Cinematic storyboard & kinetic choreography ready. Composing motion film…"
                        } else {
                            "✨ Deep outline & structural concepts generated. Composing final artifact…"
                        },
                    );
                    if req.mode == "motion" {
                        req.prompt = format!(
                            "User Creative Brief: {}\n\nCINEMATIC MOTION STORYBOARD & ART TREATMENT (STRICTLY NO SLIDES, NO CARDS - PURE KINETIC MOTION GRAPHICS):\n{}",
                            req.prompt, expanded
                        );
                    } else {
                        req.prompt = format!(
                            "User Request: {}\n\nDETAILED RESEARCH & STRUCTURED SPECIFICATION:\n{}",
                            req.prompt, expanded
                        );
                    }
                }
            }
        }

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

        let generated_html = generate_prototype_html_from_request(&req);
        self.write_artifact(&generated_html, req.session_id.as_deref())?;
        providers::emit_event(sender, "done", "Artifact rendered successfully.");
        Ok(())
    }

    async fn run_provider(&self, req: PromptRequest, settings: &Settings) -> Result<(), String> {
        let tokens = design_tokens_with_custom(&req.design_system, req.custom_theme.as_ref());
        let system = build_system_prompt_with_request(&tokens, &req);
        let user = build_user_prompt_from_request(&req);

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
                self.write_artifact(&html, req.session_id.as_deref())?;
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
                match self.harden_agent_artifact(req.session_id.as_deref()) {
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

    fn write_artifact(&self, html: &str, session_id: Option<&str>) -> Result<(), String> {
        let workspace = Path::new(&self.workspace_dir);
        let target = workspace.join("index.html");
        fs::write(&target, html).map_err(|e| format!("Failed to write to workspace: {}", e))?;
        info!("Wrote artifact to {:?} ({} bytes)", target, html.len());

        if let Some(sid) = session_id {
            let session_dir = workspace.join("sessions").join(sid);
            if let Err(e) = fs::create_dir_all(&session_dir) {
                warn!("Failed to create session dir {:?}: {e}", session_dir);
            } else {
                let session_file = session_dir.join("index.html");
                if let Err(e) = fs::write(&session_file, html) {
                    warn!("Failed to write session file {:?}: {e}", session_file);
                } else {
                    info!("Saved session copy to {:?} ({} bytes)", session_file, html.len());
                }
            }
        }
        Ok(())
    }

    /// When the agent writes `workspace/index.html` itself, it bypasses the
    /// model-text path (and therefore `ensure_csp`). Add the same offline CSP
    /// here — but only for self-contained documents, because `default-src
    /// 'none'` would break a deliberately multi-file artifact.
    fn harden_agent_artifact(&self, session_id: Option<&str>) -> Result<Option<String>, String> {
        let workspace = Path::new(&self.workspace_dir);
        let target = workspace.join("index.html");
        let html = fs::read_to_string(&target)
            .map_err(|error| format!("Failed to read the agent artifact: {error}"))?;
        let final_html = if html
            .to_ascii_lowercase()
            .contains("content-security-policy")
        {
            html
        } else if !html_is_self_contained(&html) {
            info!("Agent artifact references local files; skipping CSP injection");
            html
        } else {
            let hardened = ensure_csp(&html);
            fs::write(&target, &hardened)
                .map_err(|error| format!("Failed to harden the agent artifact: {error}"))?;
            hardened
        };

        if let Some(sid) = session_id {
            let session_dir = workspace.join("sessions").join(sid);
            let _ = fs::create_dir_all(&session_dir);
            let session_file = session_dir.join("index.html");
            let _ = fs::write(&session_file, &final_html);
        }

        let size = final_html.len();
        info!("Processed agent artifact ({size} bytes)");
        Ok(Some(format!(
            "Agent artifact ready ({} KB).",
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
/// Checks whether the user prompt contains Persian or Arabic characters.
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
        "- Layout Architecture (Editorial / Brand Surface):
  * Build an asymmetrical editorial grid (12 columns, max-width 1200px, 24px gap) with crisp 1px structural hairlines (like Stripe Press, Linear, or Read.cv).
  * Vary visual cadence across sections: pair a high-impact split hero (interactive live component preview or multi-layer schematic SVG on one side, tight typographic lockup on the other) with a bento/matrix specification grid, an interactive live demo/calculator section, and a structured technical spec table.
  * Never stack three identical icon-title-paragraph cards in a row."
    } else {
        "- Layout Architecture (Interactive Product Screen / App Shell):
  * Build a functional, high-density software workspace (like Linear, Figma, Raycast, or Vercel Dashboard), NOT a marketing landing page. Do NOT include marketing hero banners, testimonials, pricing tiers, or giant footers unless explicitly asked.
  * Use a full-viewport application shell (`min-height: 100dvh; display: grid`) with a persistent navigation rail or command header, a primary workspace canvas/table/board, and a contextual inspector or detail drawer.
  * Use tight, utilitarian spacing inside panels (4px, 8px, 12px, 16px, 24px) with `1px solid` structural borders separating panes."
    };

    let rtl_and_i18n_rules = if is_rtl {
        "- Persian / RTL Typography & Layout Rules (CRITICAL - Brief is in Persian):
  * Set `<html lang=\"fa\" dir=\"rtl\">` on the root element.
  * Prepend `'Vazirmatn', 'IRANSansX', 'Tahoma'` to the `--font-sans` stack so Persian glyphs render natively.
  * NEVER apply negative `letter-spacing` (`tracking`) to Persian text or headings! Keep `letter-spacing: 0` on all Persian elements.
  * Set body `line-height: 1.75` and heading `line-height: 1.35` to accommodate Persian ascenders/descenders.
  * Use CSS logical properties everywhere (`margin-inline-start`, `padding-inline`, `border-inline-start`, `inset-inline-end`, `text-align: start`).
  * Use `font-variant-numeric: tabular-nums` on aligned data columns."
    } else {
        "- Bidirectional & Typography Readiness:
  * If the brief requests Persian/Arabic content, set `<html lang=\"fa\" dir=\"rtl\">`, include `'Vazirmatn'` first in the font stack, and set `letter-spacing: 0` on all RTL text.
  * For Latin text, tighten display heading tracking (`-0.025em` to `-0.035em`) and use `font-variant-numeric: tabular-nums` on all numeric columns, counters, and timers."
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
}

pub fn build_motion_prompt(tokens: &DesignTokens) -> String {
    let dummy = PromptRequest {
        prompt: String::new(),
        design_system: "stripe".to_string(),
        mode: "motion".to_string(),
        deep: false,
        session_id: None,
        custom_theme: None,
        aspect_ratio: None,
        duration_secs: None,
        sound_effects: None,
        tts_text: None,
        tts_lang: None,
    };
    build_motion_prompt_from_request(tokens, &dummy)
}

fn build_motion_prompt_from_request(tokens: &DesignTokens, req: &PromptRequest) -> String {
    let aspect_ratio = req.aspect_ratio.as_deref().unwrap_or("16:9");
    let duration = req.duration_secs.unwrap_or(10).clamp(5, 600);
    let sfx = req.sound_effects.unwrap_or(false);
    let tts = req.tts_text.as_deref();
    let tts_lang = req.tts_lang.as_deref().unwrap_or("fa-IR");

    let artboard_desc = match aspect_ratio {
        "9:16" => "One fixed 9:16 vertical artboard at 720×1280 (optimised for mobile phones, Reels, TikTok and Stories), centred in the viewport and scaled as a unit with a resize handler: transform: translate(-50%, -50%) scale(min(vw/720, vh/1280)); aspect-ratio: 9/16. No page scroll. The composition never reflows; the frame scales like a phone screen.",
        "4:3" => "One fixed 4:3 artboard at 1200×900 (classic presentation format), centred in the viewport and scaled as a unit with a resize handler: transform: translate(-50%, -50%) scale(min(vw/1200, vh/900)); aspect-ratio: 4/3. No page scroll.",
        "1:1" => "One fixed 1:1 square artboard at 1080×1080 (square feed post format), centred in the viewport and scaled as a unit with a resize handler: transform: translate(-50%, -50%) scale(min(vw/1080, vh/1080)); aspect-ratio: 1/1. No page scroll.",
        "fill" => "Full viewport fluid responsive artboard (width: 100vw; height: 100vh; overflow: hidden). The composition adapts dynamically to the entire viewport without letterboxing.",
        _ => "One fixed 16:9 artboard at 1440×900, centred in the viewport and scaled as a unit with a resize handler: transform: translate(-50%, -50%) scale(min(vw/1440, vh/900)). No page scroll. The composition never reflows; the frame scales like a film.",
    };

    let sfx_prompt = if sfx {
        r#"
AUDIO & CINEMATIC PROCEDURAL SOUND DESIGN (Web Audio API):
- The piece MUST include high-production procedural audio using browser Web Audio API inside a <script> block.
- NO external .mp3/.wav files or CDNs (all network requests are blocked by CSP). Synthesize sounds mathematically via AudioContext, OscillatorNode, GainNode, ConvolverNode/Delay, and BiquadFilterNode.
- To sound professional, cinematic, and sleek (NEVER harsh, robotic, or cheap 8-bit beeps):
  1. Continuous Generative Ambient Soundtrack / Drone Pad:
     A lush, warm polyphonic drone chord (e.g. D minor 9th / Pentatonic: 73.4Hz, 110Hz, 174.6Hz, 261.6Hz) using low-pass filtered sawtooth/sine oscillators (cutoff 280Hz–380Hz) with gentle LFO modulation and soft gain (0.05–0.08) playing continuously throughout the film so the film NEVER feels silent or empty.
  2. Cinematic Sub-Bass Impact (Bass Drop / Scene Entry):
     Clean sine wave starting at 90Hz dropping smoothly to 32Hz over 0.8s, passed through a lowpass filter (140Hz), with a soft 15ms attack and logarithmic decay (Gain 0.4 -> 0.001). Add a subtle stereo detuned layer (+3 cents) for stereo warmth.
  3. Harmonic Glass Chimes (Data / Metric / Title reveals):
     Layered sine oscillators tuned to musical intervals (Pentatonic: E5 659Hz, B5 987Hz) with fast 5ms attack, sweet resonant bandpass filter, and 350ms gentle exponential decay.
  4. Cinematic Ambient Whoosh / Air Riser (Scene wipes):
     Filtered noise or multi-oscillator pitch sweep with soft low-pass filter (sweeping 180Hz -> 1200Hz -> 100Hz) and gentle stereo panning.
- Autoplay & User Gesture:
  Provide a sleek, subtle floating audio button (e.g. "♪ صدا" / "✕ بی‌صدا") and automatically resume AudioContext on first click/key or show an unobtrusive start HUD."#
    } else {
        ""
    };

    let tts_prompt = if let Some(text) = tts {
        format!(
            r#"
TEXT-TO-SPEECH (TTS) NARRATION ENABLED:
- Voiceover narration script: "{text}"
- Narration language code: "{tts_lang}"
- Implement Web Speech API in the <script> block:
  const utterance = new SpeechSynthesisUtterance("{text}");
  utterance.lang = "{tts_lang}";
- Coordinate kinetic typography and subtitles with the spoken narration so words appear in rhythm.
- Provide an inviting "Play Voiceover & Film" button or trigger speech synthesis on user interaction.
"#,
            text = escape_html(text),
            tts_lang = tts_lang,
        )
    } else {
        String::new()
    };

    format!(
        r#"You are ZenDesign Motion: an elite motion designer and creative technologist (calibre of Apple Keynote product films, Studio Freight, Buck, ManvsMachine, and Claude Opus 4.6 art direction).
You turn the user's brief into a jaw-dropping, cinema-grade kinetic motion piece that runs in the browser.
You are NOT building a website. You are NOT building a presentation slide deck.
You are directing a high-production motion graphics film composed purely of HTML5, CSS3, dynamic SVG and JavaScript.

Design tokens (palette — derive tints with color-mix() or alpha; never invent conflicting hues):
- background: {bg}
- text: {text}
- accent: {accent}
- font stack: {font}

ABSOLUTE PROHIBITIONS (CATASTROPHIC FAILURE CRITERIA):
- ZERO CARDS OR BOXES: Absolutely NO rectangular cards, NO rounded containers with borders, NO presentation slide cards, NO card grids, NO dashboard panels. If your output contains cards or boxes with titles and descriptions, IT IS AN IMMEDIATE FAILURE.
- ZERO BULLET POINTS OR PARAGRAPHS: Do NOT display paragraphs of text or bulleted lists. A motion film communicates visually through bold kinetic typography (2 to 5 words max per beat) and synchronized graphic events.
- ZERO WEBSITE CHROME: No headers with pill badges, no navbars, no buttons competing with the film.

THE CANVAS & ATMOSPHERE:
- {artboard_desc}
- Deep cinematic atmosphere: the stage is boundless. Use subtle ambient radial lighting (e.g. radial-gradient(circle at 50% 50%, color-mix(in srgb, {accent} 16%, transparent), transparent 75%)), travelling light rays, or interactive HTML5 Canvas micro-particles drifting in deep space.
- Spatial depth & parallax: structure the scene across 3 depth planes:
  * Background: Ambient lighting or floating constellation/dust.
  * Midground: Intricate animated SVG geometry and massive kinetic typography.
  * Foreground: Floating optical coordinates, hairline crosshairs or parallax particles.

KINETIC TYPOGRAPHY AS PROTAGONIST (ZERO CLIPPING DISCIPLINE):
- Kinetic typography is the hero visual art. Treat typography as physical, kinetic sculpture:
  * Giant display scale: clamp(48px, 8vw, 130px) (or 96px–180px on 1440×900).
  * Line-Height & Diacritic Headroom Safety (MANDATORY TO PREVENT CLIPPING):
    - For Persian / Arabic script: MUST use `line-height: 1.25 to 1.35` and `letter-spacing: 0` (NEVER use negative letter-spacing, and NEVER use line-height under 1.2 which cuts off the hat of 'آ' and dots of 'ش/چ/پ/ژ').
    - In masked reveal containers (`.typo-mask`), ALWAYS provide vertical overflow clearance:
      `.typo-mask {{ overflow: hidden; display: inline-block; padding: 0.18em 0.12em; margin: -0.18em -0.12em; vertical-align: bottom; }}`
      This ensures that glyph ascenders, hats (کلاه آ), and bottom descenders/dots are 100% visible with ZERO clipping from top or bottom!
    - For English script: line-height: 1.05 to 1.15; letter-spacing: -0.03em.
  * Mandatory masked reveals: Wrap lines or words in `.typo-mask` containers. Animate inner lines with translateY(120%) -> translateY(0%) using snappy easing cubic-bezier(0.16, 1, 0.3, 1), or clip-path.
  * Stagger siblings: delay each word or letter by 40–80ms for rhythmic entry.
  * Dynamic typographic behavior: tracking expansion/compression, chromatic text shadow, or filled-to-stroked outline shift.

INTRICATE HERO GRAPHIC ARTIFACT IN EVERY BEAT:
- Text is NEVER alone. Every single beat MUST feature an intricate inline SVG or HTML5 Canvas visual centerpiece:
  * Concentric 3D isometric rings rotating on multiple axes (transform: rotateX(60deg) rotateZ(...)) with glowing gradient accents and dashed tick marks.
  * Dynamic SVG spline curves drawing in real-time using stroke-dasharray and stroke-dashoffset (e.g. glowing metrics graph, biometric waveforms).
  * Futuristic coordinate reticles, rotating degree dials, orbital satellite dots, or laser sweeps.
  * Morphing organic vectors or particle explosions.

TIMELINE, DURATION & CHOREOGRAPHY (STRICT PACING MANDATE):
- Autoplay immediately, muted by default or with slick audio start overlay. Total runtime: strictly {duration}s dynamic duration.
- Loop seamlessly, or hold the final cinematic mark and restart cleanly. The first frame must work as a film poster.
- STRICT PACING MANDATE (NO DEAD TIME):
  * NEVER leave a single visual graphic or camera angle static on screen for more than 3 to 4.5 seconds!
  * Divide the {duration}s duration into a rich sequence of fast-paced rhythmic beats (each beat lasting 2.5s to 4.5s max).
  * Within EVERY beat, there must be continuous sub-second momentum: oscillating waveforms, flowing particle vectors, rotating 3D wireframe geometries, orbiting electrons, laser scan lines, traveling light bursts, or camera perspective tilt.
  * Transitions between beats MUST be visceral and cinematic: 3D scale-throughs (diving through a geometric portal or letter aperture), horizontal velocity wipes, glitch/displacement flashes, or camera roll — NEVER a sluggish 20-second static hold!
{sfx_prompt}
{tts_prompt}

MOTION GRAMMAR & EASING:
- Build the timeline with CSS @keyframes and animation-delay, or the Web Animations API / requestAnimationFrame. Everything shares one clock.
- Easing:
  * Entrances: 400–800ms with cubic-bezier(0.16, 1, 0.3, 1) [expo out].
  * Exits: 250–450ms with cubic-bezier(0.7, 0, 0.84, 0) or scale-through zoom into camera.
  * Holds: 800–1800ms.
- Animate transform, opacity, clip-path and filter only. Never animate top/left/width/height. Hold 60fps.
- Continuous ambient momentum: subtle micro-rotation or particle drift so the composition is NEVER static or frozen.

MINIMALIST CONTROLS CHROME:
- Ultra-understated: a single hairline 2px progress bar pinned to the bottom edge with a glowing accent tip.
- Sleek elapsed counter (e.g. 00:04 / {duration}s) in monospace 11px font at low opacity.
- Minimalist floating replay and sound toggle buttons in the corner with no card or box background.

TECHNICAL:
- One complete HTML5 document, one <style> and one <script>. No external libraries, CDNs, web fonts or remote images. Under ~100KB. No console errors.
- prefers-reduced-motion: show the composed end frame as a static poster and keep replay working. Keyboard: Space pauses/resumes, R replays.

SELF-CHECK — verify before replying:
1. Does the HTML contain ANY cards, bordered boxes, slide containers, or presentation card grids? If yes, DELETE them immediately.
2. Is the typography massive, masked, and kinetic, rather than small body copy in boxes?
3. Is there an intricate animated SVG or Canvas graphic centerpiece in every beat?
4. Does it autoplay, run the full dynamic requested duration of {duration}s, and loop or hold cleanly?

Output contract:
- Return ONE complete HTML document in a single ```html fenced code block. No commentary.
- If the brief cannot be satisfied, reply with a single line beginning with "ERROR:"."#,
        bg = tokens.bg,
        text = tokens.text,
        accent = tokens.accent,
        font = tokens.font_family,
        artboard_desc = artboard_desc,
        duration = duration,
        sfx_prompt = sfx_prompt,
        tts_prompt = tts_prompt,
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

fn generate_motion_storyboard_plan(req: &PromptRequest) -> String {
    let duration = req.duration_secs.unwrap_or(10).clamp(5, 600);
    let prompt = req.prompt.trim();
    let is_rtl = contains_rtl_script(prompt);

    let beat_duration = if duration <= 15 {
        3.0
    } else if duration <= 30 {
        3.5
    } else if duration <= 60 {
        4.0
    } else {
        4.5
    };

    let num_beats = ((duration as f64) / beat_duration).round() as usize;
    let num_beats = num_beats.clamp(3, 16);
    let actual_beat_len = (duration as f64) / (num_beats as f64);

    let mut plan = String::new();
    plan.push_str("\n\n════════════════════════════════════════════════════════════════\n");
    plan.push_str("DIRECTOR'S STORYBOARD & TIMELINE IMPLEMENTATION PLAN\n");
    plan.push_str("════════════════════════════════════════════════════════════════\n");
    plan.push_str(&format!(
        "Total Film Duration: {duration} seconds | Planned Rhythmic Beats: {num_beats} beats (~{:.1}s per beat)\n",
        actual_beat_len
    ));
    plan.push_str("MANDATORY PACING MANDATE (NO DEAD TIME):\n- NEVER leave any single graphic or camera angle static on screen for more than 3 to 4.5 seconds!\n- Execute this concrete storyboard beat-by-beat with continuous momentum and visceral transitions:\n\n");

    for i in 0..num_beats {
        let t_start = (i as f64) * actual_beat_len;
        let t_end = ((i + 1) as f64) * actual_beat_len;
        let pct_start = ((i as f64) / (num_beats as f64) * 100.0).round() as u32;
        let pct_end = (((i + 1) as f64) / (num_beats as f64) * 100.0).round() as u32;

        let beat_role = if i == 0 {
            "THE GENESIS & HOOK (Origins, cosmic/microscopic ignition, bold title statement)"
        } else if i == 1 {
            "THE ARCHITECTURE (Structural grid, isometric wireframe synthesis, blueprint vectors)"
        } else if i == num_beats - 1 {
            "THE FINALE & SIGNATURE (Culmination, iconic unified emblem, continuous kinetic horizon)"
        } else if i == num_beats - 2 {
            "THE SURGE & VELOCITY (High-frequency clock burst, maximum visual speed, data torrent)"
        } else if i % 2 == 0 {
            "THE DEEP PROCESS (Complex procedural SVG/Canvas centerpiece, metric evolution)"
        } else {
            "THE BREAKTHROUGH (Camera scale-through, kinetic typographic impact, frequency shift)"
        };

        plan.push_str(&format!(
            "• Beat {:02} [{:.1}s – {:.1}s | {}%–{}%] — {}\n  - Camera & Motion: Active camera movement (zoom-through / 3D tilt / velocity wipe). Zero static stillness.\n  - Kinetic Typo: Massive masked reveal (2–4 words maximum in {}).\n  - Centerpiece Visual: Intricate animated SVG / Canvas procedural geometry synchronized with the beat.\n  - Audio Event: Sub-bass impact & synced harmonic riser/whoosh.\n\n",
            i + 1,
            t_start,
            t_end,
            pct_start,
            pct_end,
            beat_role,
            if is_rtl { "natural Persian (فارسی)" } else { "concise English" }
        ));
    }

    plan.push_str("TIMELINE CLOCK SPECIFICATION:
- Driven by a unified `requestAnimationFrame(tick)` loop using `performance.now()`.
- Calculates `elapsed = (now - startTime) / 1000`.
- Continuously renders 60fps ambient background Canvas (micro-particles / light sweeps).
- Evaluates beat triggers at the exact designated timestamps.
- Hairline bottom progress bar smoothly interpolates `elapsed / TOTAL * 100%`.
- Monospace counter displays `fmt(elapsed) + ' / ' + fmt(TOTAL)`.
- At `elapsed >= TOTAL`, holds signature mark cleanly and seamlessly restarts after 2.5s.");

    plan
}

fn generate_slides_implementation_plan(req: &PromptRequest) -> String {
    let prompt = req.prompt.trim();
    let is_rtl = contains_rtl_script(prompt);

    let mut plan = String::new();
    plan.push_str("\n\n════════════════════════════════════════════════════════════════\n");
    plan.push_str("SLIDE DECK ARCHITECTURE & CONTENT IMPLEMENTATION PLAN\n");
    plan.push_str("════════════════════════════════════════════════════════════════\n");
    plan.push_str("Carefully structure the presentation with distinct, high-impact 16:9 slides:\n");
    plan.push_str("1. Slide 1 (Cover): Bold title lockup, category badge, subtitle, and an authentic animated SVG/Canvas backdrop.\n");
    plan.push_str("2. Slide 2 (Problem / Context): Core challenge, contextual metrics, and structured comparison cards.\n");
    plan.push_str("3. Slide 3 (Core Architecture / Solution): Deep technical breakdown, procedural SVG node-diagram or schematic layout.\n");
    plan.push_str("4. Slide 4 (Features / Capabilities): Bento grid matrix with distinct, non-repetitive feature specifications.\n");
    plan.push_str("5. Slide 5 (Metrics & Impact): Key quantitative metrics, data visualization chart/bars, and growth indicators.\n");
    plan.push_str("6. Slide 6 (Conclusion / Roadmap): Future vision, next steps, and unified signature contact/cta lockup.\n");
    plan.push_str(&format!(
        "Language & Direction: {}\n",
        if is_rtl {
            "Persian (فارسی) with `dir=\"rtl\"` and Vazirmatn font."
        } else {
            "English with Space Grotesk / Inter."
        }
    ));
    plan.push_str("Each slide MUST be a fully fleshed out `<section class=\"slide\" id=\"sN\">` with unique content, clear hierarchy, and smooth slide navigation (`function go(n)`).");
    plan
}

fn generate_prototype_implementation_plan(_req: &PromptRequest) -> String {
    let mut plan = String::new();
    plan.push_str("\n\n════════════════════════════════════════════════════════════════\n");
    plan.push_str("PRODUCT PROTOTYPE SPECIFICATION & STATE MACHINE PLAN\n");
    plan.push_str("════════════════════════════════════════════════════════════════\n");
    plan.push_str("Implement a genuine, living software application shell with:\n");
    plan.push_str("1. Application Shell: Persistent sidebar / top navigation, workspace toolbar, and responsive content canvas.\n");
    plan.push_str("2. Living State Machine: Working tab switcher, live search/filtering input that filters list/grid items in real-time, interactive modal/drawer inspectors, and toggleable filters.\n");
    plan.push_str("3. Authentic Domain Mock Data: Realistic names, timestamps, identifiers, and quantitative values — zero 'Lorem Ipsum'.\n");
    plan.push_str("4. Bespoke Visual Centerpiece: Interactive SVG/Canvas data visualization or live metric sparklines.\n");
    plan
}

pub fn build_user_prompt_from_request(req: &PromptRequest) -> String {
    let is_rtl = contains_rtl_script(&req.prompt);
    let lang_directive = if is_rtl {
        "
Language & Direction: Persian (فارسی) — MUST use `<html lang=\"fa\" dir=\"rtl\">`, `'Vazirmatn'` font priority, `letter-spacing: 0`, and natural Persian UI terminology."
    } else {
        ""
    };

    let duration = req.duration_secs.unwrap_or(10).clamp(5, 600);
    let artifact_type = match req.mode.as_str() {
        "motion" => {
            format!(
                "
Artifact type: MOTION PIECE – a self-playing browser animation running for EXACTLY {duration}s. NOT a website: no nav, sections, cards, pricing, FAQ, CTA or footer."
            )
        }
        "slides" => {
            "
Artifact type: SLIDE DECK (PRECISE SLIDE DECOMPOSITION) – full-viewport 16:9 presentation deck. Carefully respect any explicit slide separations, slide titles, numbered steps, or sections provided in the brief. Every designated slide must be faithfully represented as a separate, beautifully composed 16:9 slide.".to_string()
        }
        _ => {
            "
Artifact type: INTERACTIVE PRODUCT / EDITORIAL ARTIFACT – include a working vanilla JS state machine (live filtering, tab switching, interactive charts/calculators, or stateful inspection) and bespoke inline SVG/Canvas graphics.".to_string()
        }
    };

    let implementation_plan = match req.mode.as_str() {
        "motion" => generate_motion_storyboard_plan(req),
        "slides" => generate_slides_implementation_plan(req),
        _ => generate_prototype_implementation_plan(req),
    };

    format!(
        "Brief:
{}

Design system id: {}
Mode: {}{}{}{}

Build the artifact now.",
        req.prompt.trim(),
        req.design_system,
        req.mode,
        artifact_type,
        lang_directive,
        implementation_plan
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
    generate_prototype_html_from_request(&dummy)
}

pub fn generate_prototype_html_from_request(req: &PromptRequest) -> String {
    let tokens = design_tokens_with_custom(&req.design_system, req.custom_theme.as_ref());
    if req.mode == "motion" {
        return generate_motion_html(req, &tokens);
    }

    let bg = &tokens.bg;
    let text = &tokens.text;
    let accent = &tokens.accent;
    let font_family = &tokens.font_family;
    let card_bg = &tokens.card_bg;
    let border_color = &tokens.border_color;

    let headline = escape_html(&headline_from_prompt(&req.prompt));
    let brief = escape_html(&truncate_words(
        if req.prompt.trim().is_empty() {
            "A modern collaborative workspace, rendered live by the ZenDesign agent."
        } else {
            req.prompt.trim()
        },
        260,
    ));
    let design_system = escape_html(&req.design_system);
    let mode = escape_html(&req.mode);

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

fn generate_motion_html(req: &PromptRequest, tokens: &DesignTokens) -> String {
    let aspect_ratio = req.aspect_ratio.as_deref().unwrap_or("16:9");
    let duration = req.duration_secs.unwrap_or(10).clamp(5, 600);
    let sfx = req.sound_effects.unwrap_or(false);
    let tts_text = req.tts_text.as_deref().unwrap_or("").trim();
    let tts_lang = req.tts_lang.as_deref().unwrap_or("fa-IR");

    let (artboard_style, _artboard_dims) = match aspect_ratio {
        "9:16" => (
            "width: 720px; height: 1280px; aspect-ratio: 9/16; transform: translate(-50%, -50%) scale(min(calc(100vw / 750), calc(100vh / 1320)));",
            "720 × 1280 (Mobile / Reels)",
        ),
        "4:3" => (
            "width: 1200px; height: 900px; aspect-ratio: 4/3; transform: translate(-50%, -50%) scale(min(calc(100vw / 1240), calc(100vh / 940)));",
            "1200 × 900 (4:3 Classic)",
        ),
        "1:1" => (
            "width: 1080px; height: 1080px; aspect-ratio: 1/1; transform: translate(-50%, -50%) scale(min(calc(100vw / 1120), calc(100vh / 1120)));",
            "1080 × 1080 (Square)",
        ),
        "fill" => (
            "width: 100vw; height: 100vh; transform: none; position: relative; top: 0; left: 0;",
            "Fluid Fullscreen",
        ),
        _ => (
            "width: 1440px; height: 810px; aspect-ratio: 16/9; transform: translate(-50%, -50%) scale(min(calc(100vw / 1480), calc(100vh / 850)));",
            "1440 × 810 (16:9 Landscape)",
        ),
    };

    let headline = escape_html(&headline_from_prompt(&req.prompt));
    let tts_clean = escape_html(tts_text);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    {csp}
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ZenDesign Cinematic Motion Piece</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            background: #050608;
            color: {text};
            font-family: {font_family};
            width: 100vw;
            height: 100vh;
            overflow: hidden;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .motion-artboard {{
            position: absolute;
            top: 50%;
            left: 50%;
            transform-origin: center center;
            background: radial-gradient(circle at 50% 50%, color-mix(in srgb, {accent} 14%, {bg}) 0%, {bg} 85%);
            overflow: hidden;
            box-shadow: 0 40px 100px -20px rgba(0, 0, 0, 0.85);
            {artboard_style}
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            perspective: 1200px;
        }}
        #ambientCanvas {{
            position: absolute;
            inset: 0;
            width: 100%;
            height: 100%;
            pointer-events: none;
            opacity: 0.65;
            z-index: 1;
        }}

        /* Ambient Lighting Beam */
        .light-beam {{
            position: absolute;
            width: 600px;
            height: 600px;
            border-radius: 50%;
            background: radial-gradient(circle, {accent}35 0%, transparent 70%);
            filter: blur(80px);
            pointer-events: none;
            z-index: 1;
            animation: beamFloat {duration}s infinite alternate ease-in-out;
        }}
        @keyframes beamFloat {{
            0% {{ transform: translate(-20%, -20%) scale(0.9); }}
            100% {{ transform: translate(120%, 70%) scale(1.3); }}
        }}

        /* Stage & Kinetic Choreography */
        .stage {{
            position: relative;
            flex: 1;
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 2;
        }}
        .beat {{
            position: absolute;
            inset: 0;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            text-align: center;
            padding: 0 64px;
            opacity: 0;
            pointer-events: none;
        }}

        /* Dynamic keyframes synced to {duration}s */
        .beat-1 {{ animation: beat1Motion {duration}s infinite cubic-bezier(0.16, 1, 0.3, 1); }}
        .beat-2 {{ animation: beat2Motion {duration}s infinite cubic-bezier(0.16, 1, 0.3, 1); }}
        .beat-3 {{ animation: beat3Motion {duration}s infinite cubic-bezier(0.16, 1, 0.3, 1); }}

        @keyframes beat1Motion {{
            0% {{ opacity: 0; transform: scale(0.92) translateZ(-60px); }}
            5% {{ opacity: 1; transform: scale(1) translateZ(0); }}
            28% {{ opacity: 1; transform: scale(1.02) translateZ(10px); }}
            34% {{ opacity: 0; transform: scale(1.15) translateZ(80px); filter: blur(8px); }}
            100% {{ opacity: 0; }}
        }}

        @keyframes beat2Motion {{
            0%, 32% {{ opacity: 0; transform: scale(0.85) translateZ(-80px); filter: blur(6px); }}
            37% {{ opacity: 1; transform: scale(1) translateZ(0); filter: blur(0); }}
            62% {{ opacity: 1; transform: scale(1.02) translateZ(15px); }}
            68% {{ opacity: 0; transform: scale(1.18) translateZ(90px); filter: blur(8px); }}
            100% {{ opacity: 0; }}
        }}

        @keyframes beat3Motion {{
            0%, 66% {{ opacity: 0; transform: scale(0.88) translateZ(-60px); filter: blur(6px); }}
            71% {{ opacity: 1; transform: scale(1) translateZ(0); filter: blur(0); }}
            94% {{ opacity: 1; transform: scale(1.01) translateZ(5px); }}
            99% {{ opacity: 0; transform: scale(1.08) translateZ(40px); }}
            100% {{ opacity: 0; }}
        }}

        /* Kinetic Typography Masking */
        .mask-container {{
            overflow: hidden;
            display: block;
            margin-bottom: 8px;
        }}
        .mask-child {{
            display: inline-block;
            transform: translateY(115%);
            animation: textRise {duration}s infinite cubic-bezier(0.16, 1, 0.3, 1);
        }}
        .beat-1 .mask-child {{ animation-name: textRise1; }}
        .beat-2 .mask-child {{ animation-name: textRise2; }}
        .beat-3 .mask-child {{ animation-name: textRise3; }}

        @keyframes textRise1 {{
            0% {{ transform: translateY(115%); }}
            6%, 28% {{ transform: translateY(0); }}
            34%, 100% {{ transform: translateY(-115%); }}
        }}
        @keyframes textRise2 {{
            0%, 33% {{ transform: translateY(115%); }}
            38%, 62% {{ transform: translateY(0); }}
            68%, 100% {{ transform: translateY(-115%); }}
        }}
        @keyframes textRise3 {{
            0%, 67% {{ transform: translateY(115%); }}
            72%, 95% {{ transform: translateY(0); }}
            100% {{ transform: translateY(-115%); }}
        }}

        .mono-tag {{
            font-size: 13px;
            font-weight: 700;
            letter-spacing: 0.35em;
            text-transform: uppercase;
            color: {accent};
            margin-bottom: 20px;
            opacity: 0.9;
        }}
        .kinetic-headline {{
            font-size: clamp(48px, 7.5vw, 108px);
            font-weight: 900;
            letter-spacing: -0.045em;
            line-height: 0.95;
            text-wrap: balance;
            max-width: 1100px;
        }}
        .kinetic-subtext {{
            font-size: clamp(16px, 2.2vw, 24px);
            font-weight: 400;
            letter-spacing: -0.01em;
            opacity: 0.75;
            margin-top: 24px;
            max-width: 680px;
            line-height: 1.45;
        }}
        .accent-gradient {{
            background: linear-gradient(135deg, {text} 20%, {accent} 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}

        /* Hero Animated SVG Centerpiece */
        .gyro-container {{
            width: 220px;
            height: 220px;
            position: relative;
            margin-bottom: 28px;
            transform-style: preserve-3d;
        }}
        .gyro-ring {{
            position: absolute;
            inset: 0;
            border-radius: 50%;
            border: 2px dashed {accent}66;
            animation: gyroSpin 14s infinite linear;
        }}
        .gyro-ring-1 {{
            border-style: solid;
            border-color: {accent}99;
            transform: rotateX(65deg) rotateY(15deg);
            animation-duration: 9s;
        }}
        .gyro-ring-2 {{
            transform: rotateX(-45deg) rotateY(55deg);
            animation-direction: reverse;
            animation-duration: 13s;
        }}
        .gyro-ring-3 {{
            transform: rotateX(20deg) rotateZ(45deg);
            animation-duration: 18s;
            border-width: 1px;
            border-color: {accent}44;
        }}
        .gyro-core {{
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            width: 32px;
            height: 32px;
            border-radius: 50%;
            background: {accent};
            box-shadow: 0 0 40px 10px {accent};
            animation: corePulse 2s infinite ease-in-out;
        }}
        @keyframes gyroSpin {{
            0% {{ transform: rotateX(65deg) rotateY(15deg) rotateZ(0deg); }}
            100% {{ transform: rotateX(65deg) rotateY(15deg) rotateZ(360deg); }}
        }}
        @keyframes corePulse {{
            0%, 100% {{ transform: translate(-50%, -50%) scale(1); opacity: 0.9; }}
            50% {{ transform: translate(-50%, -50%) scale(1.35); opacity: 1; }}
        }}

        /* Beat 2 SVG Waveform */
        .wave-svg {{
            width: 580px;
            height: 140px;
            margin: 20px 0;
            overflow: visible;
        }}
        .wave-path {{
            stroke-dasharray: 800;
            stroke-dashoffset: 800;
            animation: drawWave {duration}s infinite cubic-bezier(0.16, 1, 0.3, 1);
        }}
        @keyframes drawWave {{
            0%, 33% {{ stroke-dashoffset: 800; }}
            42%, 62% {{ stroke-dashoffset: 0; }}
            68%, 100% {{ stroke-dashoffset: -800; }}
        }}
        .stat-mega {{
            font-size: clamp(64px, 10vw, 130px);
            font-weight: 900;
            letter-spacing: -0.05em;
            line-height: 0.9;
            color: {accent};
            text-shadow: 0 0 50px {accent}66;
        }}

        /* Minimalist Controls & Timeline Chrome */
        .bottom-hud {{
            position: relative;
            z-index: 10;
            padding: 24px 36px;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }}
        .time-counter {{
            font-family: monospace;
            font-size: 12px;
            letter-spacing: 0.1em;
            opacity: 0.5;
        }}
        .hud-actions {{
            display: flex;
            align-items: center;
            gap: 16px;
        }}
        .hud-btn {{
            background: transparent;
            border: none;
            color: {text};
            cursor: pointer;
            font-size: 12px;
            font-weight: 600;
            letter-spacing: 0.05em;
            opacity: 0.6;
            transition: all 0.2s;
        }}
        .hud-btn:hover {{
            opacity: 1;
            color: {accent};
        }}
        .hairline-progress {{
            position: absolute;
            bottom: 0;
            left: 0;
            width: 100%;
            height: 2px;
            background: rgba(255, 255, 255, 0.08);
            z-index: 20;
        }}
        .hairline-fill {{
            height: 100%;
            width: 100%;
            background: {accent};
            box-shadow: 0 0 12px {accent};
            transform-origin: left center;
            animation: lineProgress {duration}s linear infinite;
        }}
        @keyframes lineProgress {{
            0% {{ transform: scaleX(0); }}
            100% {{ transform: scaleX(1); }}
        }}

        .subtitles-banner {{
            position: absolute;
            bottom: 56px;
            left: 50%;
            transform: translateX(-50%);
            z-index: 10;
            font-size: 15px;
            font-weight: 500;
            color: {text};
            opacity: 0.8;
            letter-spacing: 0.02em;
            text-align: center;
            text-shadow: 0 2px 10px rgba(0,0,0,0.8);
            max-width: 800px;
        }}
    </style>
</head>
<body>
    <div class="motion-artboard" id="artboard">
        <canvas id="ambientCanvas"></canvas>
        <div class="light-beam"></div>

        <div class="stage">
            <!-- Beat 1: The Kinetic Launch Hook -->
            <div class="beat beat-1">
                <div class="gyro-container">
                    <div class="gyro-ring gyro-ring-1"></div>
                    <div class="gyro-ring gyro-ring-2"></div>
                    <div class="gyro-ring gyro-ring-3"></div>
                    <div class="gyro-core"></div>
                </div>
                <div class="mono-tag">KINETIC SEQUENCE &bull; {duration}S RUNTIME</div>
                <div class="mask-container">
                    <h1 class="kinetic-headline mask-child">
                        {headline}
                    </h1>
                </div>
                <p class="kinetic-subtext">
                    Engineered with native procedural audio synthesis and 60 FPS choreography.
                </p>
            </div>

            <!-- Beat 2: Kinetic Data & Dynamic Spline -->
            <div class="beat beat-2">
                <div class="mono-tag">DYNAMIC COMPUTATIONAL VELOCITY</div>
                <div class="mask-container">
                    <div class="stat-mega mask-child">99.9%</div>
                </div>
                <svg class="wave-svg" viewBox="0 0 580 140" fill="none">
                    <defs>
                        <linearGradient id="waveGrad" x1="0" y1="0" x2="580" y2="0" gradientUnits="userSpaceOnUse">
                            <stop offset="0%" stop-color="{accent}" stop-opacity="0.2"/>
                            <stop offset="50%" stop-color="{accent}" stop-opacity="1"/>
                            <stop offset="100%" stop-color="{text}" stop-opacity="0.8"/>
                        </linearGradient>
                    </defs>
                    <path class="wave-path" d="M 10 70 Q 150 10, 290 70 T 570 70" stroke="url(#waveGrad)" stroke-width="4" stroke-linecap="round"/>
                </svg>
                <div class="mask-container">
                    <div class="kinetic-subtext mask-child" style="margin-top: 0;">
                        Zero Overhead &bull; Pure Native Web Technology &bull; Unbounded Execution
                    </div>
                </div>
            </div>

            <!-- Beat 3: Grand Finale & Hero Signature -->
            <div class="beat beat-3">
                <div style="width: 80px; height: 80px; border-radius: 50%; border: 2px solid {accent}; display: flex; align-items: center; justify-content: center; margin-bottom: 24px; box-shadow: 0 0 60px {accent}88; background: {accent}18;">
                    <span style="font-size: 38px; font-weight: 900; color: {accent};">Z</span>
                </div>
                <div class="mask-container">
                    <h2 class="kinetic-headline mask-child accent-gradient">
                        DESIGNED FOR DISTINCTION
                    </h2>
                </div>
                <p class="kinetic-subtext">
                    Continuous Autonomous Intelligence &bull; ZenDesign Motion
                </p>
            </div>
        </div>

        <div class="subtitles-banner" id="subtitleBox" style="{subtitle_display}">
            "{tts_clean}"
        </div>

        <div class="bottom-hud">
            <div class="time-counter" id="timeCounter">00:00 / 00:{duration}</div>
            <div class="hud-actions">
                <button class="hud-btn" id="voiceBtn" title="Play Voiceover">🎙 Voiceover</button>
                <button class="hud-btn" id="sfxBtn" title="Toggle SFX"><span id="sfxIcon">🔊 SFX</span></button>
                <button class="hud-btn" id="restartBtn" title="Replay">⟳ Replay</button>
            </div>
        </div>

        <div class="hairline-progress">
            <div class="hairline-fill" id="timelineProgress"></div>
        </div>
    </div>

    <script>
        const DURATION = {duration};
        const HAS_SFX = {sfx_enabled};
        const TTS_TEXT = "{tts_clean_js}";
        const TTS_LANG = "{tts_lang}";

        // Canvas ambient starfield
        (function initCanvas() {{
            const canvas = document.getElementById('ambientCanvas');
            const ctx = canvas.getContext('2d');
            let w, h, particles = [];

            function resize() {{
                w = canvas.width = canvas.offsetWidth;
                h = canvas.height = canvas.offsetHeight;
                particles = [];
                for (let i = 0; i < 45; i++) {{
                    particles.push({{
                        x: Math.random() * w,
                        y: Math.random() * h,
                        r: Math.random() * 1.5 + 0.5,
                        dx: (Math.random() - 0.5) * 0.4,
                        dy: (Math.random() - 0.5) * 0.4,
                        alpha: Math.random() * 0.5 + 0.2
                    }});
                }}
            }}
            window.addEventListener('resize', resize);
            resize();

            function draw() {{
                ctx.clearRect(0, 0, w, h);
                particles.forEach(p => {{
                    p.x += p.dx;
                    p.y += p.dy;
                    if (p.x < 0) p.x = w;
                    if (p.x > w) p.x = 0;
                    if (p.y < 0) p.y = h;
                    if (p.y > h) p.y = 0;

                    ctx.beginPath();
                    ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
                    ctx.fillStyle = `rgba(255, 255, 255, ${{p.alpha}})`;
                    ctx.fill();
                }});
                requestAnimationFrame(draw);
            }}
            draw();
        }})();

        // Elapsed time counter
        (function initTimer() {{
            const el = document.getElementById('timeCounter');
            const start = performance.now();
            function pad(n) {{ return n < 10 ? '0' + n : n; }}
            function tick(now) {{
                const elapsed = ((now - start) / 1000) % DURATION;
                const secs = Math.floor(elapsed);
                const totalSecs = DURATION;
                el.textContent = `00:${{pad(secs)}} / ${{pad(Math.floor(totalSecs / 60))}}:${{pad(totalSecs % 60)}}`;
                requestAnimationFrame(tick);
            }}
            requestAnimationFrame(tick);
        }})();

        let audioCtx = null;
        let sfxMuted = !HAS_SFX;

        function getAudioContext() {{
            if (!audioCtx) {{
                audioCtx = new (window.AudioContext || window.webkitAudioContext)();
            }}
            if (audioCtx.state === 'suspended') {{
                audioCtx.resume();
            }}
            return audioCtx;
        }}

        function playPunch() {{
            if (sfxMuted) return;
            try {{
                const ctx = getAudioContext();
                const now = ctx.currentTime;
                const osc = ctx.createOscillator();
                const filter = ctx.createBiquadFilter();
                const gain = ctx.createGain();

                filter.type = 'lowpass';
                filter.frequency.setValueAtTime(140, now);
                filter.frequency.exponentialRampToValueAtTime(45, now + 0.6);

                osc.type = 'sine';
                osc.frequency.setValueAtTime(85, now);
                osc.frequency.exponentialRampToValueAtTime(32, now + 0.6);

                gain.gain.setValueAtTime(0.001, now);
                gain.gain.linearRampToValueAtTime(0.4, now + 0.02);
                gain.gain.exponentialRampToValueAtTime(0.001, now + 0.6);

                osc.connect(filter);
                filter.connect(gain);
                gain.connect(ctx.destination);
                osc.start(now);
                osc.stop(now + 0.62);
            }} catch(e) {{}}
        }}

        function playTick() {{
            if (sfxMuted) return;
            try {{
                const ctx = getAudioContext();
                const now = ctx.currentTime;
                [659.25, 987.77].forEach((freq, idx) => {{
                    const osc = ctx.createOscillator();
                    const gain = ctx.createGain();
                    const filter = ctx.createBiquadFilter();

                    filter.type = 'bandpass';
                    filter.frequency.setValueAtTime(freq, now);
                    filter.Q.setValueAtTime(4.0, now);

                    osc.type = 'sine';
                    osc.frequency.setValueAtTime(freq, now);

                    gain.gain.setValueAtTime(0.001, now);
                    gain.gain.linearRampToValueAtTime(0.12 / (idx + 1), now + 0.006);
                    gain.gain.exponentialRampToValueAtTime(0.001, now + 0.35);

                    osc.connect(filter);
                    filter.connect(gain);
                    gain.connect(ctx.destination);
                    osc.start(now);
                    osc.stop(now + 0.36);
                }});
            }} catch(e) {{}}
        }}

        function playWhoosh() {{
            if (sfxMuted) return;
            try {{
                const ctx = getAudioContext();
                const now = ctx.currentTime;
                const osc = ctx.createOscillator();
                const filter = ctx.createBiquadFilter();
                const gain = ctx.createGain();

                filter.type = 'lowpass';
                filter.frequency.setValueAtTime(220, now);
                filter.frequency.exponentialRampToValueAtTime(1200, now + 0.22);
                filter.frequency.exponentialRampToValueAtTime(150, now + 0.48);

                osc.type = 'triangle';
                osc.frequency.setValueAtTime(120, now);
                osc.frequency.linearRampToValueAtTime(260, now + 0.22);
                osc.frequency.linearRampToValueAtTime(80, now + 0.48);

                gain.gain.setValueAtTime(0.001, now);
                gain.gain.linearRampToValueAtTime(0.18, now + 0.18);
                gain.gain.exponentialRampToValueAtTime(0.001, now + 0.48);

                osc.connect(filter);
                filter.connect(gain);
                gain.connect(ctx.destination);
                osc.start(now);
                osc.stop(now + 0.5);
            }} catch(e) {{}}
        }}

        function speakNarration() {{
            if (!window.speechSynthesis || !TTS_TEXT) return;
            try {{
                window.speechSynthesis.cancel();
                const utterance = new SpeechSynthesisUtterance(TTS_TEXT);
                utterance.lang = TTS_LANG;
                utterance.rate = 1.0;
                window.speechSynthesis.speak(utterance);
            }} catch(e) {{}}
        }}

        function scheduleAudioBeats() {{
            if (sfxMuted) return;
            playPunch();
            setTimeout(() => playWhoosh(), (DURATION * 0.33) * 1000);
            setTimeout(() => playTick(), (DURATION * 0.45) * 1000);
            setTimeout(() => playPunch(), (DURATION * 0.68) * 1000);
        }}

        const sfxBtn = document.getElementById('sfxBtn');
        const sfxIcon = document.getElementById('sfxIcon');
        const voiceBtn = document.getElementById('voiceBtn');
        const restartBtn = document.getElementById('restartBtn');

        sfxBtn.addEventListener('click', () => {{
            sfxMuted = !sfxMuted;
            sfxIcon.textContent = sfxMuted ? '🔇 SFX Off' : '🔊 SFX On';
            if (!sfxMuted) {{
                getAudioContext();
                playPunch();
            }}
        }});

        voiceBtn.addEventListener('click', () => {{
            getAudioContext();
            speakNarration();
            if (!sfxMuted) scheduleAudioBeats();
        }});

        restartBtn.addEventListener('click', () => {{
            const artboard = document.getElementById('artboard');
            artboard.style.animation = 'none';
            artboard.offsetHeight;
            artboard.style.animation = '';
            const progress = document.getElementById('timelineProgress');
            progress.style.animation = 'none';
            progress.offsetHeight;
            progress.style.animation = '';
            if (TTS_TEXT) speakNarration();
            scheduleAudioBeats();
        }});

        if (HAS_SFX) {{
            setInterval(() => {{
                scheduleAudioBeats();
            }}, DURATION * 1000);
        }}
    </script>
</body>
</html>"#,
        csp = CSP_META,
        bg = tokens.bg,
        text = tokens.text,
        accent = tokens.accent,
        font_family = tokens.font_family,
        artboard_style = artboard_style,
        duration = duration,
        headline = headline,
        tts_clean = tts_clean,
        tts_clean_js = tts_clean.replace('"', "\\\"").replace('\n', " "),
        tts_lang = tts_lang,
        subtitle_display = if tts_text.is_empty() { "display: none;" } else { "display: block;" },
        sfx_enabled = if sfx { "true" } else { "false" },
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

pub fn truncate_words(text: &str, max_chars: usize) -> String {
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

pub fn extract_first_url(text: &str) -> Option<String> {
    for word in text.split_whitespace() {
        let clean = word.trim_matches(|c| c == '(' || c == ')' || c == '[' || c == ']' || c == '<' || c == '>' || c == '"' || c == '\'');
        if clean.starts_with("http://") || clean.starts_with("https://") {
            return Some(clean.to_string());
        }
        if clean.starts_with("github.com/") {
            return Some(format!("https://{clean}"));
        }
    }
    None
}

pub async fn fetch_url_context(client: &reqwest::Client, url: &str) -> Result<String, String> {
    let target = url.trim();
    // GitHub repository specific handling: directly fetch README.md for maximum signal
    if target.contains("github.com/") && !target.contains("/raw/") {
        let trimmed = target.trim_end_matches('/').trim_end_matches(".git");
        let parts: Vec<&str> = trimmed.split("github.com/").collect();
        if parts.len() == 2 {
            let repo_path = parts[1].trim_matches('/');
            let segments: Vec<&str> = repo_path.split('/').collect();
            if segments.len() >= 2 {
                let owner = segments[0];
                let repo = segments[1];
                let raw_candidates = [
                    format!("https://raw.githubusercontent.com/{owner}/{repo}/main/README.md"),
                    format!("https://raw.githubusercontent.com/{owner}/{repo}/master/README.md"),
                    format!("https://raw.githubusercontent.com/{owner}/{repo}/main/readme.md"),
                    format!("https://raw.githubusercontent.com/{owner}/{repo}/master/readme.md"),
                ];
                for raw_url in raw_candidates {
                    if let Ok(resp) = client.get(&raw_url).timeout(Duration::from_secs(6)).send().await {
                        if resp.status().is_success() {
                            if let Ok(body) = resp.text().await {
                                if !body.trim().is_empty() {
                                    let preview = body.chars().take(5000).collect::<String>();
                                    return Ok(format!("GitHub Repository: {}/{}\n\nREADME:\n{}", owner, repo, preview));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Generic Web Page Fetcher: fetch and strip HTML tags
    let resp = client
        .get(target)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) ZenDesign-Agent/1.0")
        .timeout(Duration::from_secs(7))
        .send()
        .await
        .map_err(|e| format!("Fetch failed: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("HTTP error {}", resp.status()));
    }

    let text = resp.text().await.map_err(|e| e.to_string())?;
    let stripped = strip_html_tags(&text);
    let sample = stripped.split_whitespace().take(800).collect::<Vec<&str>>().join(" ");
    Ok(sample)
}

fn strip_html_tags(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    let mut in_script_or_style = false;
    let lower = html.to_lowercase();
    let chars: Vec<char> = html.chars().collect();
    let lower_chars: Vec<char> = lower.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if !in_tag && chars[i] == '<' {
            in_tag = true;
            // Check if entering script or style
            let rem: String = lower_chars[i..].iter().take(8).collect();
            if rem.starts_with("<script") || rem.starts_with("<style") {
                in_script_or_style = true;
            }
            if rem.starts_with("</script") || rem.starts_with("</style") {
                in_script_or_style = false;
            }
        } else if in_tag && chars[i] == '>' {
            in_tag = false;
            out.push(' ');
        } else if !in_tag && !in_script_or_style {
            out.push(chars[i]);
        }
        i += 1;
    }
    out
}

pub async fn expand_brief_with_research(
    client: &reqwest::Client,
    settings: &Settings,
    prompt: &str,
    mode: &str,
    duration_secs: u32,
) -> Result<String, String> {
    let is_persian = prompt.chars().any(|c| ('\u{0600}'..='\u{06FF}').contains(&c));
    let lang_instruction = if is_persian {
        "Respond in fluent, professional Persian (Farsi) with modern terminology."
    } else {
        "Respond in concise, professional English."
    };

    let (system_role, research_prompt) = if mode == "motion" {
        let structure_guide = if duration_secs >= 60 {
            let minutes = duration_secs / 60;
            format!(
                "LONG-FORM EPISODIC MOTION FILM ({minutes} min / {duration_secs}s TOTAL RUNTIME):\n\
                - Design 6 to 10 sequential cinematic acts spanning exactly {duration_secs} seconds.\n\
                - For each act: specify Exact Timestamp range (e.g. 00:00-00:30), Hero Visual Graphic / Particle / SVG metaphor, Punchy Kinetic Typography (MAX 2-4 words per beat, NO paragraphs), and Camera transition.\n\
                - The motion timeline must sustain dynamic energy continuously throughout all {duration_secs} seconds."
            )
        } else {
            format!(
                "SHORT-FORM MOTION PIECE ({duration_secs}s runtime):\n\
                - 4 cinematic beats (Hook, Friction/Velocity, Breakthrough, Climax/Signature mark).\n\
                - In each beat: specify the visual centerpiece (e.g., 3D SVG isometric rings, glowing spline curve drawing, particle constellation), the massive kinetic headline (2-4 words max), and dynamic camera move (scale-through, directional wipe)."
            )
        };

        (
            "You are an elite Motion Film Creative Director (Apple Keynote, Buck, Studio Freight). You choreograph cinematic visual storyboards with kinetic typography and vector geometry. You strictly forbid slides, cards, or paragraphs.",
            format!(
                r#"You are ZenDesign's World-Class Motion Film Director (calibre of Apple Keynote product reveals, Buck, Studio Freight, ManvsMachine).
The user wants a cinema-grade kinetic motion piece for:
"{prompt}"

Your mission is to construct a CHOREOGRAPHED VISUAL STORYBOARD & MOTION TREATMENT for the frontend animation engine.

STRICT MOTION DIRECTIVES (OVERRIDE ALL PRESENTATION/SLIDE INSTINCTS):
1. THIS IS A FILM, NOT A SLIDE PRESENTATION.
   - Absolutely NO slides, NO UI cards, NO rectangular bordered boxes, NO bulleted lists, NO paragraphs of body text.
   - Every single beat is high-impact visual art: massive kinetic typography, animated vector geometry, optical depth, camera scale-throughs, and particle drift.

2. STORYBOARD & SHOT CHOREOGRAPHY:
{structure_guide}

3. VISUAL ARTIFACTS & HERO GRAPHICS:
   - Detail the intricate inline SVG geometry, 3D rotating rings, procedural waveforms, or particle effects rendered in each scene.
   - Detail the kinetic typography behavior (masked line reveals, split-word stagger, tracking compression, scale-through).

4. COLOR, MOOD & ATMOSPHERE:
   - Deep cinematic backdrop, lighting halos, accent energy pulses.

{lang_instruction}
Deliver a crisp, director-level motion storyboard ready for instantaneous animation."#
            )
        )
    } else {
        let duration_note = if mode == "slides" {
            "For slides mode, construct an exact slide-by-slide deck (Cover, Problem, Architecture, Key Metrics, Deep Dive, Case Study, Conclusion) with specific content, titles, and stat callouts for every slide.".to_string()
        } else {
            "For prototype mode, construct a full Product Requirements Document (PRD) with user journey, dashboard cards, table columns, interactive controls, and realistic dataset.".to_string()
        };

        (
            "You are an elite Research & Product Architect. You turn brief inputs into deep, comprehensive domain breakdowns.",
            format!(
                r#"You are ZenDesign's Lead Autonomous Product Architect & Creative Director.
The user gave this brief for a {mode}:
"{prompt}"

Your job is NOT to write HTML or code.
Your job is to generate the COMPLETE, EXHAUSTIVE IMPLEMENTATION PLAN & DETAILED DESIGN SPECIFICATION that will be fed into the frontend generation engine.

Create a high-impact, professional architectural plan:
1. Executive Summary & Core Value Proposition
   - Define the product/topic clearly with sharp, professional terminology.
   - Target audience, user problem, and core solution.

2. Domain Breakdown & Realistic Data Assets
   - Key industry metrics, real numbers, benchmarks, and technical concepts.
   - Real labels, product names, component breakdown (never use lorem ipsum or generic placeholders).

3. Blueprint & Structural Breakdown:
   {duration_note}

4. Visual Language & Editorial Tone
   - Creative art direction, visual metaphors, font emphasis, and layout dynamics.

{lang_instruction}
Make it deeply comprehensive, structured, and immediately actionable for production."#
            )
        )
    };

    match settings.provider.as_str() {
        PROVIDER_OPENAI => {
            let base = settings.openai.base_url.trim().trim_end_matches('/');
            let model = settings.openai.model.trim();
            if base.is_empty() || model.is_empty() {
                return Err("OpenAI provider unconfigured".to_string());
            }
            let key = settings.openai_key();
            let body = serde_json::json!({
                "model": model,
                "temperature": 0.6,
                "messages": [
                    { "role": "system", "content": system_role },
                    { "role": "user", "content": research_prompt }
                ]
            });
            let mut req = client.post(format!("{base}/chat/completions")).json(&body);
            if !key.is_empty() {
                req = req.bearer_auth(key);
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            if !resp.status().is_success() {
                return Err(format!("OpenAI returned status {}", resp.status()));
            }
            let res_json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
            let text = res_json["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            if text.trim().is_empty() {
                return Err("Empty response from research provider".to_string());
            }
            Ok(text)
        }
        PROVIDER_ANTHROPIC => {
            let base = settings.anthropic.base_url.trim().trim_end_matches('/');
            let model = if settings.anthropic.model.trim().is_empty() {
                "claude-opus-4-6"
            } else {
                settings.anthropic.model.trim()
            };
            let key = settings.anthropic_key();
            if key.is_empty() {
                return Err("Anthropic API key missing".to_string());
            }
            let body = serde_json::json!({
                "model": model,
                "system": system_role,
                "max_tokens": 2048,
                "messages": [
                    { "role": "user", "content": research_prompt }
                ]
            });
            let resp = client
                .post(format!("{base}/v1/messages"))
                .header("x-api-key", key.trim())
                .header("anthropic-version", "2023-06-01")
                .json(&body)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            if !resp.status().is_success() {
                return Err(format!("Anthropic returned status {}", resp.status()));
            }
            let res_json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
            let text = res_json["content"][0]["text"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            if text.trim().is_empty() {
                return Err("Empty response from research provider".to_string());
            }
            Ok(text)
        }
        _ => Err("Provider does not support autonomous research".to_string()),
    }
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
