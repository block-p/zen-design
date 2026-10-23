# Hero Patterns

**Drawn from:** 55 design notes (01–55, excluding `_staging/` and `_inventory.md`)
**Last updated:** 2026-05-28

> **Batch context.** Most of the source designs come from a single shared lineage — the **motionsites.ai** generator pipeline. Tells: CloudFront videos on `d8j0ntlcm91z4.cloudfront.net/user_38xzZboKViGWJOttwIXH07lWA1P/...mp4`, a shared `useVideoFadeLoop` hook (500ms fade + 0.55s preempt + 100ms blackout reset), a shared `BoomerangVideoBg` component, an `Instrument Serif + Inter/Barlow` font cluster, a `.liquid-glass` + `.liquid-glass-strong` CSS pair, and a `borderRadius.DEFAULT: '9999px'` Tailwind override. Cross-brand asset reuse confirmed — Asme v3 (#16)'s Service Card 1 uses the exact same MP4 file as Velorah (#05)'s hero. Treat near-identical "design families" (Aethera/Velorah, Asme v1/v2/v3, SENTINEL HLS/Spline) as variants of one template when counting samples.

## Patterns observed

### Pattern: Centered cinematic stack over full-bleed video
**Used by:** #01 Cinematic Space-Travel, #04 Aethera, #05 Velorah, #06 Asme v1, #09 Asme v2, #14 MicroVisuals, #16 Asme v3, #19 Cinematic Streaming, #29 CodeNest, #41 LUMINA, #46 Stellar.ai
**Confidence:** **HIGH — 11+ samples, the dominant batch hero.**
**Recipe:**
- Shell: `relative min-h-screen overflow-hidden bg-black` (or saturated dark `--background`)
- Video: `<video autoPlay loop muted playsInline class="absolute inset-0 w-full h-full object-cover z-0">` — the four-attribute quartet required by iOS Safari
- Either native `loop` (#05, #10) OR the JS fade-loop (`useVideoFadeLoop`): rAF reads `currentTime`/`duration`, ramps opacity 0→1 over first 500ms, ramps 1→0 starting `duration - 0.55s`, on `ended` resets `currentTime = 0` after a 100ms blackout (#04, #09, #16, #42)
- Optional 1% white tint or `bg-black/30` overlay to keep type readable on bright frames
- Floating liquid-glass pill nav at `top-4` to `top-6`, centered, `max-w-5xl mx-auto`
- Centered content column (`flex flex-col items-center justify-center text-center`), often `-translate-y-[20%]` to bias above optical center
- Vertical stack: optional kicker badge → headline (Instrument Serif `text-5xl sm:text-7xl md:text-8xl`, `tracking-tight`, `font-normal`, leading 0.95–1.05) → subhead in muted opacity → 1–2 glass-pill CTAs → optional stat cards/social row pinned bottom
- Entry choreography: a 3-class CSS keyframe (`fade-rise` opacity 0→1 + translateY 24→0 + 0.8s ease-out), delayed `0 / 0.2 / 0.4s` — OR Framer Motion `fadeUp` with `[0.16, 1, 0.3, 1]` "decisive arrival" curve and `duration 0.6–1.0`

**When it works:** Brand-cinematic positioning, single-page anti-hero statements, video-as-mood where the type can hold center axis.
**When it fails:** When the source video has high-contrast bright frames in the loop — white text disappears. When you forget `prefers-reduced-motion` (a universal gap in this family). When the centered column wraps awkwardly on narrow viewports because of `whitespace-nowrap` headlines.
**Variations seen:** Aethera (#04) inverts the polarity — white bg, video inset as accent at `top: 300px` rather than full-bleed. Velorah (#05) keeps everything but uses native `loop` instead of JS fade. Asme v2 (#09) and v3 (#16) add a pill-stack vocabulary below the headline. Mindloop (#26) and #51 darken the video twice (60% video opacity + `bg-black/60` + `backdrop-blur-[2px]`) for maximum suppression.

---

### Pattern: Big-type editorial wordmark hero
**Used by:** #03 Prisma, #04 Aethera, #05 Velorah, #07 Jack 3D, #14 MicroVisuals, #16 Asme v3, #41 LUMINA, #51 AI Website Builder, #43 dot.
**Confidence:** **HIGH — 9 samples.**
**Recipe:**
- Single colossal display word or 4-word sentence at `clamp(96px, 18vw, 280px)` (MicroVisuals, the cap) or stepped `text-6xl md:text-7xl lg:text-[5.5rem]` (most others) or `text-[26vw]` for true wordmark-as-page (Prisma)
- Font: Instrument Serif italic at `font-normal` (400) — never bolder. Foundry says serif edges read tighter and trying for weight contrast collapses the elegance.
- Tracking: aggressive negative. Recurring values: `-2.46px` at `text-8xl`, `-0.04em` at 64px, `-0.06em` at 128px. Matches the foundations curve.
- Leading: crushed to `0.8–0.95` so two-line headlines visually fuse into one block.
- Often paired with a small uppercase kicker (`tracking-[0.2em] text-xs`) above and a single line of body underneath.
- For wordmark-as-page (Prisma): bottom-aligned, asymmetric 8/4 column split, with a hanging-pill nav from the top edge and an inset rounded-card video framing.
- AI Website Builder (#51) pushes furthest: 136px headline `Build Faster` clipped from white → lavender `#b4c0ff`, with the eyebrow line in italic Instrument *Serif* and the headline in Instrument *Sans* — a "pre-headline + headline" two-tier titling where the small line is poetic prose and the giant line is functional imperative.

**When it works:** Studios, brand-cinematic, anti-hero positions where the wordmark IS the brand statement. Confidence > evidence is the bet.
**When it fails:** When there's no proof below the hero. Aethera and Velorah both leave the brand promise un-discharged because nothing follows the headline. When `whitespace-nowrap` locks the headline width, any copy edit breaks the layout.
**Variations seen:** SkyElite (#10) uses two display words ("Premium. / Accessible.") with line 2 pulled up `marginTop: -12px` inline so the words *overlap*, with a light-gray → dark-navy color step that reads as depth. LinkFlow (#12) does the same trick with saturation rather than lightness ("Close the rift" saturated dark green, "linking signals and action" muted sage). Bloom (#33) and many others do single-italic-word emphasis ("spirit", "Inspired", "Smarter", "Overview", "Industry", "and", "all", "Agencies") — see "italic-word emphasis" sub-pattern below.

---

### Pattern: Card-as-hero (page-as-mat with rounded video card)
**Used by:** #24 RIVR, #30 Foundation, #44 Halo, #48 Convix, #50 Behind the Lens (partial), #39 Xero (rounded hero card on dark page)
**Confidence:** **HIGH — 5–6 samples.**
**Recipe:**
- Outer page background a calm light/dark mat (`#f0f0f0`, `#f9fafb`, `#F5F5F5`, `#ededed`, `#0a0a0f`)
- A `min-h-screen` flex container with `p-3 md:p-5` gutter padding
- Inside: a single `max-w-[1400-1600px]` card with `rounded-2xl md:rounded-3xl` (RIVR pushes to `rounded-[3rem]`) and `overflow-hidden`
- Fixed-height card via `h-[calc(100vh-Xpx)]` so the card frame is the viewport minus the gutter
- Background video inside the card at `absolute inset-0 object-cover`, often with `bg-white/10` or `bg-black/30` softening overlay
- Foreground nav floats above the card or sits *inside* it as a transparent strip at top
- Headline/CTA pinned to the upper-left or bottom-left "editorial corner" via `flex items-end` + `p-12 pt-36`
- Often a "scroll-promise" gesture: a marquee (Halo) or a dashboard tray (Convix) deliberately *bleeds off the bottom edge* via the card's `overflow-hidden`, signalling more content without an explicit `Scroll ↓` indicator

**When it works:** Premium fintech, DeFi, infrastructure brands that want "designed object" rather than "section of a page." The frame creates instant elevation and a deliberate physical-card metaphor.
**When it fails:** When the card height isn't tuned to the actual content — clipping kills important info. When the gutter is too thin and the card edges flatten against the viewport — needs at least `p-3` on mobile, `p-5` on desktop.
**Variations seen:** Foundation (#30) holds the card to a fixed 600px height — never fluid `100vh` — so it always sits whole on a 13"+ screen. Halo's `max-w-[88rem]` (1408px) is a deliberate "between sizes" choice — wider than Linear-disciplined 1280, narrower than oceanic 1536+. Behind the Lens (#50) is the counter-pattern — pure white canvas, no mat, magazine-grid as the entire shell.

---

### Pattern: Bottom-aligned anchor (content sinks to floor)
**Used by:** #08 Vex Ventures, #31 SENTINEL Spline, #15 Axion Studio, #35 Generic HLS (THIN), #19 Cinematic Streaming, #14 MicroVisuals (partial — bottom row), #12 LinkFlow (corners)
**Confidence:** **MEDIUM-HIGH — 5+ confirmed samples.**
**Recipe:**
- Outer hero `relative min-h-screen overflow-hidden`
- Outer container `flex flex-col` with `flex-1 flex flex-col justify-end` on the content wrapper — content pushed to the lower 30–40% of the viewport
- Nav at the top edge (`pt-6`, often transparent or glass)
- Middle empty (background carries it — usually a Spline 3D scene, ambient video, or shader stack)
- Hero block at bottom: 2-column on `lg` with copy left + small accent card or tag right (Vex), or single-column left-aligned headline (SENTINEL), or triangulated 3-zone (MicroVisuals: tagline-left / CTAs-center / tagline-right)
- Bottom padding `pb-10 md:pb-16` — never flush, always lifted off the absolute bottom edge
- Sometimes pairs with a top-anchored hero title and an asymmetric bottom-row content cluster (MicroVisuals: title fixed at `top: 126px`, bottom row at `bottom-12`)

**When it works:** When the background needs to perform (3D scenes, busy video frames, ambient renders, kinetic backdrops). The empty middle lets the background breathe, and the bottom-anchor reads as "I have so much going on visually that I can put the message in the corner."
**When it fails:** On short laptop viewports (1366×768) where the bottom band collapses against the navbar. Without `min-h-screen` discipline, vertical centering fights gravity.
**Variations seen:** Vex Ventures (#08) uses a dark `.liquid-glass` variant (`rgba(0,0,0,0.4)` + `blur 4px` + softer `0.3/0.1` rim stops) tuned for bright/varied video backgrounds. SENTINEL Spline (#31) wraps the content container in `pointer-events-none` so the visitor can drag the 3D scene anywhere except on the actual CTAs (the **"pass-through container, active buttons"** pattern).

---

### Pattern: Two-panel split hero (52/48 or 50/50)
**Used by:** #28 Aurora Sign Up, #33 Bloom, #45 Taskly
**Confidence:** **MEDIUM — 3 samples, but each pushes the 50/50 convention.**
**Recipe:**
- `flex min-h-screen` row with `lg:h-screen lg:overflow-hidden` so desktop locks to 100vh
- Left panel `w-[52%]` (Bloom, Aurora), right panel `flex-1` (~48%)
- The 4% asymmetry is the move — a 50/50 split reads ambivalent; 52/48 commits to a primary side
- Bloom: left is a `liquid-glass-strong` stage holding the hero stack, right is a sidebar of stacked glass cards (community, accounts, processing, growth archive) — **glass-on-glass-on-video** depth stack
- Aurora: left is a rounded video pane with bottom-aligned content (`pb-32 px-12`) including a 3-step onboarding indicator; right is the auth form pane
- Taskly: classical 50/50 with type+CTA left and a hue-rotated orb right — but uses `mix-blend-screen` + `hue-rotate(-55deg) saturate(250%)` to alchemize a borrowed purple WEBM orb into the brand's electric blue

**When it works:** Auth pages, brand spreads where you have both a message and a single hero visual. Below `lg` it should collapse to single column or `hidden lg:flex` on the visual pane.
**When it fails:** When the split is 50/50 (boring) or 40/60 with form dominant (form drowns the brand). Below 1024px, the visual pane should collapse cleanly or hide.
**Variations seen:** Aurora pulls the auth form to ~48% and *demotes* it visually so the brand video pane reads as protagonist — counterintuitive for a sign-up page. Taskly inverts the entire dark-mode-batch formula with a light theme, but keeps the glass material identity via `backdrop-blur-[50px]` + `rgba(255,255,255,0.3)` fill + `1px solid rgba(0,0,0,0.1)` stroke for navbars.

---

### Pattern: Hero with embedded product mockup/dashboard
**Used by:** #22 Aura, #40 Nexora, #48 Convix, #46 Stellar.ai (conditional overlays), #49 Neuralyn (luminosity-blended)
**Confidence:** **MEDIUM-HIGH — 5 samples.**
**Recipe:**
- Quiet hero stack on top: badge → headline (often italic-word emphasis) → 1-line subhead → 2 CTAs (`Book a demo` + Play circle)
- Below the CTAs (`mt-8`): a full-width or `max-w-5xl/6xl` rounded card holding a **fully coded** dashboard mockup
- Container is `rounded-2xl overflow-hidden p-3-4` with frosted-glass wrap: `bg: rgba(255,255,255,0.4)`, `border: 1px solid rgba(255,255,255,0.5)`, `boxShadow: var(--shadow-dashboard)` (a hand-tuned two-layer shadow `0 25px 80px -12px rgba(0,0,0,0.08), 0 0 0 1px rgba(0,0,0,0.06)`)
- The dashboard *exists* in real DOM — never a PNG. SVG cubic Bézier area chart, sidebar, search field with ⌘K hint, transactions table, two stat cards. All `pointer-events-none select-none` so users can't accidentally interact.
- Page wrapper is `h-screen overflow-hidden` so the dashboard *intentionally* clips at the bottom of the viewport — the visual incompleteness IS the scroll prompt
- Numbers in the mockup are *internally consistent* — 91.5% of 425,000 = 389,000, not lorem (Convix, Foundation, Aurora fintech-realism)

**When it works:** SaaS B2B where "we built this dashboard for you" is the claim. The willingness to ship the dashboard as real DOM is the flex.
**When it fails:** When the mockup numbers are obvious placeholders. When the dashboard's interaction surface looks live but isn't — users feel deceived.
**Variations seen:** Stellar.ai (#46) makes the dashboard *cycle* — 4-tab auto-advance every 4 seconds, with each tab swapping in a conditional overlay (Analyse/Train/Testing/Deploy) carrying its own progress color. Neuralyn (#49) `w-screen` escapes the parent's max-width via `marginLeft: calc(-50vw + 50%)`, then layers a dashboard PNG with `mixBlendMode: "luminosity"` so the dashboard's hue is inherited from the video beneath it.

---

### Pattern: Hero with 3D / shader / procedural background
**Used by:** #15 Axion Studio (4-shader stack), #31 SENTINEL Spline (Spline scene), #34 Glowing Feature Cards (gradient halos)
**Confidence:** **LOW-MEDIUM — 3 samples, but each defines a distinct recipe.**
**Recipe:**
- **Shader stack (Axion):** `Swirl + ChromaFlow + FlutedGlass + FilmGrain` from the `shaders/react` npm package, layered `absolute inset-0 pointer-events-none`. Key tuning: `FlutedGlass speed: 0.15` (slow enough to read as weather, not screensaver), `FilmGrain strength: 0.05` (just enough to kill banding). All four chromatic-aberration colors set to `#ff5f03` so the shader's color bleed harmonizes with the brand orange.
- **Spline (SENTINEL #31):** embed a Spline scene at `inset-0`, then a `bg-black/30` overlay to tame brightness, then content at `z-10`. Critical: the content wrapper carries `pointer-events-none` while the buttons inside carry `pointer-events-auto` — so the visitor can drag the 3D scene anywhere on the canvas except on actual clickable elements ("pass-through container, active buttons" pattern).
- **Gradient halo cards (#34):** each card sits on `#1A1A1C` (5% above page bg), with a `::after` pseudo-element blurred copy of a multi-stop gradient producing a colored *aura*. Border is itself a gradient via `mask-composite`. Three cards = three temperature moods (pink-orange / cyan / violet-magenta) communicated through chromatic light alone, no glassmorphism, no transparency.

**When it works:** When the brand wants "real-time / interactive / alive" rather than "recorded." A Spline scene is a system; a video is a recording. For security, AI, infrastructure brands, the implicit message of an interactive simulation is on-brand.
**When it fails:** Performance — shader stacks burn GPU continuously; Spline scenes are heavy. Mobile / integrated graphics are at risk. Always pair with `prefers-reduced-motion` fallback.

---

### Pattern: Stats hero (3-zone top/middle/bottom)
**Used by:** #18 Bold Vision (Fearless / Vision / Delivered), #38 DesignPro (Become Product Leader)
**Confidence:** **LOW — 2 samples, but a distinct pattern.**
**Recipe:**
- Outer `min-h-screen flex flex-col` with three zones via `flex-1` middle
- Top zone: nav (fixed height)
- Middle zone: `flex-1 flex items-center justify-end` — right-aligned 3-stat row pinned in the upper-middle (`+300 / +200 / +100` with Inter Semibold)
- Bottom zone: 2-row content stack — Row A is tagline + text-link CTA, Row B is description + 3-word billboard heading
- The 3-word headline uses clip-reveal: each word in its own `overflow-hidden` parent, `motion.span` starting at `y: "110%"` (fully clipped below) animating to `y: 0` with a 0.14s stagger — the agency-reveal tempo
- One saturated accent (purple `#5E0ED7` in #18, blue `#64CEFB` in #38), used in *exactly three places*: logo dot, "+" symbols, CTA color

**When it works:** Agency portfolios that lead with scale/proof. Stats-as-headline replaces marketing claims with quantified evidence.
**When it fails:** When the stats are vague ("+300" without a unit). When the background video has bright frames — black text on video is gambling.

---

### Pattern: AI search-as-hero (input is the CTA)
**Used by:** #42 Logoipsum (Transform Data Quickly), #06 Asme v1 (collapsing email form), #43 dot. (typing demo)
**Confidence:** **LOW — 3 samples.**
**Recipe:**
- Headline + subhead stack centered
- Below the subhead, a **translucent rounded box 728×200 with backdrop-blur** sitting where the CTA pair would normally go
- Box contains 3 rows: top meta (credit counter + "Powered by GPT-4o"), middle input + submit, bottom action chips (Attach / Voice / Prompts) + character counter
- The input IS the CTA — no separate "Try free" button
- For collapsing form (Asme v1): button → form morphs on click → typewriter placeholder cycles → submit → 4s auto-reset
- For typing demo (dot.): a `TypingMessages` component overlays the LCD screen of a Nokia phone *inside* the video, typing messages letter-by-letter in the actual Nokia Cellphone FC Small pixel font

**When it works:** AI-native products where "the demo is the demo." The cognitive load to understand "what is this?" drops to zero when you mirror the canonical ChatGPT prompt-box geometry.
**When it fails:** When you're not actually AI and you fake the affordance. Foundations: "AI-native interfaces — works when the product is AI (don't fake it)."

---

### Pattern: 404 / error-state hero
**Used by:** #55 Nexto 404
**Confidence:** **LOW — 1 sample, anecdote not pattern, but worth flagging since it's the only error-state in the batch.**
**Recipe:**
- `html, body, #root { height: 100vh; overflow: hidden }` — single-screen commitment, no scroll possible
- 2-step grey gradient page bg `linear-gradient(to top left, #F5F5F5, #F7F7F7)` (1% lightness delta, almost invisible)
- Centered themed background PNG (spaceship at `background-position: center 40%`, slightly above true center, `background-size: contain`)
- Apologetic title in `clamp(34px, 5vw, 52px)` DM Sans — "Whoops! Nothing here yet" (specifically `whoops` not `oops`, with `yet` added — implies "we might build this someday")
- Two recovery cards anchored at bottom via `margin-top: auto` within the flex column (no explicit footer position)
- Decorative Material Symbols glyphs (cloud + heart) gradient-clipped via `background-clip: text` drifting on a 5s `floatSlow` keyframe (`translateY -10px + rotate 3deg`)

**When it works:** When the brand has a unique metaphor (spaceship for "lost in space" software). When the voice is consistent across the rest of the site.
**When it fails:** When the 404 page is the only place the brand voice gets playful — feels off-brand by contrast.

---

## Italic-word emphasis (sub-pattern — appears across at least 9 designs)

**Where seen:** #04 Aethera ("silence," / "the eternal."), #05 Velorah ("dreams" / "through the silence."), #16 Asme v3 ("Innovation × *Vision*", "all"), #26 Mindloop ("Inspired", "changed.", "meaningful", "Journey"), #33 Bloom ("spirit"), #40 Nexora ("Smarter"), #43 dot. ("daily"), #47 Hotel ("and"), #48 Convix ("Agencies"), #49 Neuralyn ("Overview"), #51 AI Builder eyebrow ("Design at the speed of thought")
**Recipe:**
- Display headline in Inter or Instrument Serif at 48–96px
- One or two words inside the sentence wrapped in `<em>` (Instrument Serif italic) OR `<em className="not-italic text-muted-foreground">` (Velorah's refinement — strips the italic stylistic, keeps the color shift)
- Color shift to muted gray on light bg (`#6F6F6F`, `text-gray-500`) or muted white opacity on dark bg (`text-white/40` to `text-white/60`)
- The italic word is the **differentiation word** — the noun or operator that carries the actual brand claim (Smarter, Inspired, Industry, spirit, all, Vision, Agencies, Overview, Journey)
- Almost always *one or two words*, never a phrase — restraint is the rule

**When it works:** When the brand voice is contemplative or premium-restrained. When the rest of the type system is one sans family — the serif-italic switch reads as a single deliberate moment.
**When it fails:** When every other heading does it — loses meaning. When the italic word isn't load-bearing — emphasis on a connective (and, the) reads as fussy.

---

## Anti-patterns observed

- **No `prefers-reduced-motion` fallback** — universal gap. Every video hero, every fade-loop, every parallax, every char-stagger needs a `@media (prefers-reduced-motion: reduce)` branch that pauses the video, freezes the boomerang, or skips entrance animations to the final state. Documented as missing in >90% of the batch.
- **`whitespace-nowrap` on a headline** locks the layout to the current copy. Any copy edit ("Built for the curious" → "Built for the deeply curious") breaks the design. Seen in Asme v1/v2/v3, Axion.
- **Latent fonts** — Dirtyline loaded in MicroVisuals but never rendered, Inter 300–700 loaded in LinkFlow but never rendered, dead `liquid-glass` CSS in Axion. ~30–100KB of dead payload per design.
- **No video poster** — slow connections see a black field until first frame buffers. LCP suffers. Universal gap across the batch.
- **Centered everything on a 5-CTA hero** — when the eyebrow, headline, subhead, two CTAs, and stat row all center on one axis, the page reads as a "tower of text" and the eye has nowhere to rest. Seen in some over-stacked variants.
- **Cross-brand asset reuse** — Asme v3 Service Card 1 uses the *exact same* MP4 as Velorah's hero (`hf_20260314_131748_*.mp4`). For a brand page, reusing a sibling's hero asset as a service-card prop is a craft-tier red flag.
- **Glass on flat white** — pointless. Glass works only when the background has variance (video, gradient, image). Taskly (#45) is the exception — its `backdrop-blur-[50px]` is so wide it picks up the blurred atmospheric ellipses and refracts them; pair with a `1px solid rgba(0,0,0,0.1)` stroke + inner highlight or the surface vanishes.
- **Solo accent that goes everywhere** — when "monochrome-plus-one-accent" stops being disciplined and the accent appears in nav links, hovers, borders, focus rings, AND CTA, it stops being an accent and becomes a second brand color. CodeNest (#29) is the model — green appears in *exactly four* places: eyebrow, terminating period, CTA, link hover.

---

## Decision rules

- **If the brief is "single-page brand statement," reach for the centered-cinematic-stack-over-video.** That's the dominant pattern. Pair Instrument Serif headline + liquid-glass pill nav + 1–2 glass CTAs + fade-rise stagger. Hold yourself to 3-color discipline.
- **If the brief is "premium fintech / infrastructure" and you want a designed-object feel,** reach for the card-as-hero with `p-3-5` outer gutter and `rounded-2xl` clipped video card. Pin headline upper-left or upper-right. Use the clip as your scroll-promise.
- **If the brief is "SaaS B2B with a real dashboard,"** build the dashboard in React+SVG, wrap it in a frosted-glass card, let it deliberately clip at the bottom of the viewport. The visual incompleteness IS the scroll prompt. Audit the numbers for internal consistency.
- **If the background is busy (Spline, shaders, parallax),** anchor content bottom-left or in bottom corners. Empty middle = background performs.
- **If you have a 4-word brand position with antithesis,** consider color-as-depth two-line stacked overlap (SkyElite) OR saturation-as-depth (LinkFlow). Same size, same weight, different chroma on each word.
- **If the product is AI-native,** consider the input-as-hero where the prompt box mirrors ChatGPT's geometry. Don't fake it if you're not.
- **If you want "premium restrained voice,"** use italic-word emphasis on the differentiation noun. One face (Instrument Serif italic), one or two words, color shift to muted gray.
- **If the brand has multiple equal hues** (campaign, kids' product, seasonal), pick them at matched OKLCH lightness so one chrome color (white) reads on all. TOONHUB (#02) is the model.

---

## Open questions / gaps in our sample

- **No genuine interactive hero (drag/play/build) in the batch.** SENTINEL Spline (#31) allows camera-drag but the dragging isn't the product. The Figma/tldraw "hero IS the product" pattern from foundations doesn't appear.
- **Few asymmetric or magazine-grid heroes.** Most batch heroes commit to either centered or bottom-anchored. The 8/4 or 7/5 editorial split is rare — Prisma (#03) is the only confident sample.
- **No serif-default body hero.** Every body in the batch is sans (Inter, Barlow, DM Sans, Sora, Helvetica, Outfit, TT Norms Pro, Plus Jakarta, Schibsted Grotesk, Fustat, Kanit). Serif appears only as italic accent, never as body.
- **No anti-hero with literally just text and scroll.** Plain-style or Anthropic-style "no media at all" hero doesn't appear in the batch. Aethera (#04) is closest but cheats with the inset video.
- **Very few "no-video" heroes period.** Axion (#15, shaders), Glowing Cards (#34), Behind the Lens (#50), Duolingo (#53, gradient bg only), Nexto 404 (#55), Estimation Calculator (#54) are the only ones. The batch's mode is "video as canvas," which is itself a finding.
- **No long-form scroll-narrative heroes** where the hero transforms across multiple viewport heights (Apple AirPods Pro style). All batch heroes are single-screen.

---
*Synthesizer: layout-family | 4 of 4 playbooks. Reads from: 55 design notes + foundations/layout-and-composition.md*
