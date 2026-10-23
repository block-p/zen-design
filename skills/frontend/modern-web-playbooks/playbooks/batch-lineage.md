# Batch Lineage — How 55 Designs Resolve Into 3 Toolchains

**Captured:** 2026-05-29
**Scope:** Meta-analysis of 55 design briefs analyzed at max-depth in Batch 1.
**Purpose:** Document the discovery that the batch is not 55 independent designs from different teams. It's 3 toolchains plus a handful of one-offs, sharing CDNs, code primitives, fonts, and even raw asset files.

This file exists because synthesis kept hitting the same fingerprints. When the same CloudFront bucket (`d8j0ntlcm91z4`) supplied videos for what appeared to be 30+ unrelated brands, when the exact same `liquid-glass` CSS class appeared in 20+ briefs with byte-for-byte identical gradient stops, when the exact same `useVideoFadeLoop` rAF pattern showed up in #04 and #09 and #16 — at some point we stopped reading "shared inspiration" and started reading "shared toolchain". This playbook names what was shared, what was unique, and how to recognize batch-family membership at a glance.

## The three toolchains

### 1. motionsites.ai cluster (~40 designs)

**Primary tell:** CloudFront origin `d8j0ntlcm91z4.cloudfront.net` with user-keyed path `user_38xzZboKViGWJOttwIXH07lWA1P/` and `hf_<timestamp>_<uuid>.mp4` filenames.

**Direct evidence:** Design #07 (Jack 3D Creator) literally renders a marquee of preview GIFs whose URLs reference *other designs in this same batch*: `motionsites.ai/assets/hero-codenest-preview.gif`, `hero-asme-preview.gif`, `hero-aethera-preview.gif`, `hero-designpro-preview.gif`, `hero-stellar-ai-preview.gif`, `hero-terra-preview.gif`, `hero-nexora-preview.gif`, `hero-luminex-preview.gif`, `hero-celestia-preview.gif`. The marquee is the smoking gun — Jack's portfolio is a self-referential pointer into the generator's catalog.

**Stack signature:**
- React 18 (commonly 18.3.1) + Vite 5 + TypeScript
- Tailwind CSS 3.4 with `@tailwind base/components/utilities` (not v4 `@import`)
- framer-motion ~11–12 (still imported as `framer-motion`, not yet `motion/react`)
- lucide-react for icons
- Custom CSS for the `.liquid-glass` and `.liquid-glass-strong` utility pair, defined in `index.css` not in Tailwind config
- Google Fonts via `<link>` or `@import` — never self-hosted, never preloaded
- Default Tailwind config sometimes overrides `borderRadius.DEFAULT: '9999px'` (forces every `rounded` to pill)

**Font signature:** Instrument Serif (italic axis loaded) + Inter is the canonical pair (#01, #04, #05, #09, #14, #16, #26, #40 and many more). Barlow appears as the Inter substitute in #01 and #14. PP Neue Montreal + PP Mondwest from Pangram Pangram appears in #27 Viktor Oddy as the high-craft variant.

**Designs in this cluster (confirmed by CloudFront origin):** #01, #02 (partial), #04 Aethera, #05 Velorah, #06 Asme v1, #07 Jack, #08 Vex Ventures, #09 Asme v2, #10 SkyElite, #11 VaultShield, #12 LinkFlow, #14 MicroVisuals, #15 Axion Studio, #16 Asme v3, #17 Orbis NFT, #18 Bold Vision, #19 Cinematic Streaming, #20 Max Reed, #22 Aura, #26 Mindloop, #27 Viktor Oddy, #28 Aurora Sign-up, #29 CodeNest, #31 Sentinel HLS, #33 Bloom, #37 SENTINEL Spline, #38 HAUL Truck, #39 DesignPro, #40 Xero, #41 Nexora, #42 LUMINA, #43 Logoipsum, #44 dot. Nokia, #45 Halo USD, #46 Taskly, #47 Stellar.ai, #49 Convix, #50 Neuralyn, #51 AI Builder, #54 Estimation Calculator. That's ~40 of 55.

### 2. NHM-style outlier (rare, ~1–2 designs)

**Primary tell:** Cloudinary origin `res.cloudinary.com/dsdxaxkiz/` for all media, dramatically different stack.

**Stack signature:**
- React 19 (`^19.0.1`) + Vite 6 (`^6.2.3`) + TypeScript ~5.8.2
- Tailwind CSS 4 (`^4.1.14`) with `@import "tailwindcss"` and `@theme` block
- `@tailwindcss/vite` plugin (no PostCSS pipeline)
- `motion/react` (post-2024 rebrand of framer-motion)
- Lucide React 0.546.0 (much newer than the motionsites cluster's lucide pins)
- Self-built SVG primitives (the NHM wordmark is 10 hand-authored polygons)
- Custom SVG-filter components (SandTransitionImage's 5-stage `feTurbulence → feDisplacementMap → feOffset → feGaussianBlur → feColorMatrix` chain)

**Asset pattern:** All Cloudinary URLs are direct PNG/MP4 with no `f_auto,q_auto` transforms. Versioned `v1779624247` through `v1779625001` — uploaded within a single afternoon session (~127 seconds apart for the chapter images, 12 minutes later for the pterodactyl). The filenames betray AI synthesis: `magnific_use-img-2-as-the-exact-ba_*.mp4` (Magnific upscaler), `ChatGPT_Image_May_23_2026_*.png`.

**Designs in this cluster:** #13 NHM Natural History Museum. Only one confirmed instance. The Cloudinary fingerprint may extend to other yet-unseen designs.

### 3. Cloudinary `dfonotyfb` thin entry

**Primary tell:** Cloudinary origin `res.cloudinary.com/dfonotyfb/`.

**Designs in this cluster:** #21 only. The brief itself is thin (one video URL plus one sentence), so the stack inference is limited. The account fingerprint `dfonotyfb` is distinct from NHM's `dsdxaxkiz` — confirming it's a third toolchain, not a duplicate. Useful for provenance tracking if future batches reference the same `dfonotyfb` slug pool.

### One-off outliers

- **#32 Kresna** — vanilla HTML/CSS/JS, no framework, single-file. The only no-React design in the batch.
- **#52 Design Rocket Certificates** — single-file React, no Vite/Tailwind setup specified. Email-style 640px column layout.
- **#53 Duolingo Style Guide** — deliberate brand replica, loads real Duolingo logo SVG from production CDN. Not part of any generator pipeline — looks like a manual brand-replica exercise.
- **#55 Nexto 404** — uses R2 CDN (`pub-f170a2592d2c4a1485466404c36807be.r2.dev`) for both logo and spaceship PNG, shared with #23 and #25. Likely the same author across those three.

## The shared CDN map

| CDN host | Account / bucket | Designs |
|---|---|---|
| CloudFront `d8j0ntlcm91z4` | `user_38xzZboKViGWJOttwIXH07lWA1P` | ~40 motionsites designs |
| Cloudinary `dsdxaxkiz` | (NHM) | #13 only |
| Cloudinary `dfonotyfb` | (unknown) | #21 only |
| R2 `pub-f170a2592d2c4a1485466404c36807be.r2.dev` | (asset bucket, `viktor/` path) | #23, #25, #55 |
| R2 `pub-e68758f43067417dba612b2371819aa1.r2.dev` | (spaceship PNG) | #55 |
| Mux HLS `stream.mux.com` | various playback IDs | #06, #26, #29, #51 (HLS subset) |
| Spline `prod.spline.design` | scene `Slk6b8kz3LRlKiyk` | #37 SENTINEL 3D |
| figma.site | hash-named exports | #07 portrait, #02 character art |
| Webflow CDN `assets.website-files.com` | PP Neue Montreal | #27 Viktor Oddy |
| onlinewebfonts.com `db.onlinewebfonts.com` | font rehost | #11, #12, #24, #41, #43, #53 |
| Google Fonts | Instrument Serif, Inter, DM Sans, etc. | most designs |
| Higgs.ai proxy `images.higgs.ai/?default=1&output=webp&url=...` | wraps CloudFront | #37 HAUL and several others (image WebP conversion + resizing) |

The Higgs.ai proxy is the most diagnostic — it's a query-string wrapper that converts CloudFront-hosted PNGs to optimized WebP on the fly. The pattern `images.higgs.ai/?default=1&output=webp&url=<urlencoded-cloudfront-path>&w=1280&q=85` is unmistakable and only appears in the motionsites cluster.

## Shared code primitives (the "you built this once and reused it" tells)

### `useVideoFadeLoop` rAF pattern
The seamless-loop video hack. A `requestAnimationFrame` loop reads `video.currentTime` and `video.duration`; if `currentTime < 0.5s` it ramps opacity in, if `duration - currentTime <= 0.55s` it ramps opacity out; on `ended` event it sets opacity to 0, waits 100ms via `setTimeout`, resets `currentTime = 0`, calls `play()`. Constants: `FADE_MS = 500`, `FADE_OUT_LEAD = 0.55s`, `RESET_DELAY = 100ms`.

**Appears in:** #04 Aethera, #09 Asme v2, #16 Asme v3, #42 Logoipsum (250ms variant), and likely others.

### `BoomerangVideoBg` canvas frame-capture
The palindrome-loop trick. Plays the video once with `crossOrigin="anonymous"`, captures every frame into an `HTMLCanvasElement` array (max-width 960px), then hides the `<video>` and uses `requestAnimationFrame` at 30fps to walk the index forward to the last frame and back to the first, ping-pong forever. Uses `requestVideoFrameCallback` when available, falls back to rAF. Constants: `MAX_WIDTH = 960`, `interval = 1000/30 ≈ 33.3ms`.

**Appears in:** #12 LinkFlow, #14 MicroVisuals. Byte-for-byte identical implementation across both — `MAX_WIDTH`, fps cap, dedup guard, capture API, ping-pong index walker. Confirms shared component across the batch.

### `.liquid-glass` and `.liquid-glass-strong` CSS pair
Identical recipe in 20+ designs:
- `background: rgba(255,255,255,0.01)` — barely-there white tint
- `background-blend-mode: luminosity` — preserves underlying hue
- `backdrop-filter: blur(4px)` (or `50px` for the `-strong` variant)
- Inset `box-shadow: 0 1px 1px rgba(255,255,255,0.1)` — top edge highlight
- `::before` pseudo-element with `padding: 1.4px`, 6-stop linear gradient at `180deg` (stops `0.45 / 0.15 / 0 / 0 / 0.15 / 0.45` for the standard variant, `0.5 / 0.2 / 0 / 0 / 0.2 / 0.5` for `-strong`), `mask-composite: exclude` to carve out a thin rim-light

The gradient stops are **byte-for-byte identical across #01, #08, #14, #16, #41, and others**. This is not coincidence; it's a shared utility class.

**Latent / unused instances:** #15 Axion Studio defines `.liquid-glass` in `index.css` but never references it in markup. #36 Vize Footer defines `.text-glass` but never uses it. These are template scaffolding artifacts — the classes ship in the boilerplate, the design didn't end up needing them.

### Custom JS marquee patterns
- **Scroll-driven horizontal marquee** with passive listener + opposite-direction rows (Jack #07 — `(scrollY - sectionTop + vh) * 0.3` formula)
- **CSS-keyframe infinite marquee** with twice-rendered content + `mask-image` edge fade (Foundation #30, Max Reed #20, Halo #45 backers row, many footers)
- **GIF marquee** (Jack #07's 21 GIF set referencing other batch designs)

### Headline reveal primitives
- **WordsPullUp / WordsPullUpMultiStyle** — split text into spans, `motion.span` slides from `y:20` to `y:0` with 0.08s stagger (Prisma #03, others)
- **AnimatedLetter scroll-opacity** — per-character `useTransform` mapping scroll progress to opacity 0.2→1 (Prisma #03, Jack #07's About)
- **BlurText word-by-word** — 3-keyframe (blur 10px → 5px overshoot → 0) (#01 Cinematic Space)
- **AnimatedHeading char-by-char** — vanilla CSS transitions on inline-block spans triggered by `useState + setTimeout` (Vex Ventures #08)
- **clip-reveal stack** — words in `overflow-hidden` parents, `y: 110% → 0` (Bold Vision #18's "FEARLESS / VISION / DELIVERED")
- **Scroll-driven word reveal** — opacity 0.2→1 + color shift, anchor words land at full white, connective tissue at muted (Mindloop #26 mission, Neuralyn #50 testimonial)

### House easings
Two cubic-bezier curves appear across most of the motionsites cluster:
- `cubic-bezier(0.16, 1, 0.3, 1)` — out-expo-ish "decisive arrival"
- `cubic-bezier(0.22, 1, 0.36, 1)` — softer ease-out-quart variant

These two curves cover ~80% of all motion in the batch. Anything else (Emil Kowalski's `cubic-bezier(0.32, 0.72, 0, 1)` for drawers, Material's `cubic-bezier(0.4, 0, 0.2, 1)`) is an exception that flags a specific component lineage.

## Asset reuse across "different" brands

The smoking-gun finding: **multiple designs literally share the same MP4 file from CloudFront**.

| Asset | Used by | Implication |
|---|---|---|
| `hf_20260314_131748_f2ca2a28-fed7-44c8-b9a9-bd9acdd5ec31.mp4` | #05 Velorah hero + #16 Asme v3 Services Card 1 | Two "different brands" share their hero/feature video. Confirms CloudFront is a *shared asset library*, not brand-specific storage. |
| `hf_20260402_054547_9875cfc5-155a-4229-8ec8-b7ba7125cbf8.mp4` | #16 Asme v3 Featured Video + others | Reused inside same brand. |

The naming convention `hf_<YYYYMMDD>_<HHMMSS>_<uuid>.mp4` betrays both **the date the asset was generated** and **a HuggingFace-like AI pipeline** (the `hf_` prefix is consistent with HuggingFace generation IDs). Asset clusters around specific dates suggest **single-session batch generation** — e.g., all six videos for Orbis #17 share date `20260331` with consecutive timestamps within hours of each other.

## Template variants within the motionsites cluster

The batch isn't one template; it's a handful of templates that the generator picks between based on the prompt. After 40+ analyses, the visible variants are:

1. **Editorial / cinematic** (#04 Aethera, #05 Velorah, Asme #06/#09/#16, #43 dot.) — Instrument Serif + liquid-glass + monochrome
2. **Bento portfolio** (#20 Max Reed) — multi-card dark grid with mixed materials per cell
3. **NFT / space** (#17 Orbis) — Anton + Condiment cursive + neon + texture overlay
4. **Glassmorphism product** (#24 RIVR) — page-as-card + alpha-on-single-hue navy
5. **Animation-heavy** (#19 cinematic streaming, #02 TOONHUB) — `blurFadeUp` cascade + role-based carousel
6. **Studio / agency** (#15 Axion, #07 Jack, #27 Viktor Oddy) — shader heroes, magnet portraits, magazine spreads
7. **AI-native SaaS** (#22 Aura, #06 Asme v1, #51 AI Builder) — collapsing email forms, dashboard mockups, AI feature pills
8. **Stats hero** (#18 Bold Vision) — `+300/+200/+100` right-aligned + clip-reveal mega-headline
9. **Calculator / utility** (#54 Estimation Calculator) — dark calc form with comparison cards
10. **404 / error state** (#55 Nexto 404) — friendly-warmth single-viewport recovery card

Most designs fit one variant cleanly. A few hybridize (Mindloop #26 is editorial + AI-native).

## Tailwind v3 vs v4 split

The cluster splits roughly along **single-section vs multi-section** lines:

- **Tailwind v3 (PostCSS, `@tailwind` directives):** most single-section heroes, including the canonical motionsites family (#01, #04, #05, #06, #07, #08, #09, #10, #11, #12, #14, #15, #16, #18, #20, #22, #26, #27, #29, #38, #40 and many more)
- **Tailwind v4 (`@import "tailwindcss"`, `@theme` block):** the higher-craft multi-section pages, including #13 NHM, #24 RIVR, #28 Aurora Sign-up, #30 Foundation, #34 Glowing Cards, #36 Vize, #41 LUMINA, #43 dot. The v4 cluster correlates with `motion/react` (the post-2024 rebrand) and React 19 — these designs are generated by a *newer revision* of the generator.

The v4 cluster's adoption of `motion/react` is the cleanest tech-stack-tell: any design that imports `motion/react` (not `framer-motion`) is in the v4 / modern revision; any design that still imports `framer-motion` is in the original revision.

## CSS-variable strategy splits

- **shadcn-style HSL triples** — `--background: 0 0% 0%` (no `hsl()` wrapper), consumed via `hsl(var(--background))` for opacity composition like `bg-background/45`. Appears in #26 Mindloop, #28 Aurora, #40 Nexora, #49 Convix, #50 Neuralyn. Always paired with `tailwindcss-animate` plugin.
- **Named semantic tokens** — `--color-text`, `--color-accent`, `--color-login-bg` (#11 VaultShield), `--shadow-dashboard` (#41 Nexora), `--font-display` / `--font-sans` (#30 Foundation, Tailwind v4 `@theme`). Less common, signals more bespoke design-system thinking.
- **Direct arbitrary Tailwind values** — `bg-[#0a1b33]`, `text-[80px]` everywhere. Most of the v3 cluster.

## Custom CSS namespace escapes from Tailwind

A recurring pattern: designs with complex visuals add a custom CSS file with a class prefix to escape Tailwind utility-first conventions:
- **`c1-`** for #25 Core Features cards
- **`c3-`** for #22 Aura (pricing watermark section, traffic-light buttons, gradient headlines)
- **`c5-`** for #23 Animated Gradient CTA (the 15-`@property` declaration block + 10 keyframe animations)

The convention reads as **"this section escapes the global design system; treat its custom CSS as a quarantine zone."** Useful when you need `@property` declarations, complex SVG filters, or finely-tuned arbitrary values that don't fit Tailwind's grammar.

## Duplicates and mashes that needed cleanup

This batch arrived with several issues that the synthesizer had to resolve before analysis:

- **Deduped:** Original items #21+#22 (identical short Cloudinary stub), #31+#32 (identical Foundation paste), #55 (Duolingo style guide content pasted twice), positions #33+#39 (SENTINEL AI Spline content). The Foundation paste appearing twice and the SENTINEL Spline appearing twice suggest the user pasted some entries by mistake during transcription.
- **Mashed entries split:** Original #48 had Halo + Taskly bolted together (split into #45 + #46); original #49 had Stellar.ai + a hotel booking site (split into #47 + #48); original #56 had Estimation Calculator + Nexto 404 (split into #54 + #55).
- **Thin extractions kept but flagged:** #21 (one video URL only), #35 (one sentence only). These contribute almost no signal beyond CDN fingerprints.

## How to recognize a batch-family design at a glance

When evaluating a new design that might belong to this batch, these are the highest-signal tells:

1. **CDN check** — does any asset URL contain `d8j0ntlcm91z4.cloudfront.net/user_38xzZboKViGWJOttwIXH07lWA1P/`? If yes → motionsites family, ~95% confidence.
2. **`liquid-glass` class** — search the CSS for `.liquid-glass` with the exact `rgba(255,255,255,0.01)` + `backdrop-filter: blur(4px)` + `mask-composite: exclude` recipe. If present → motionsites or sibling.
3. **Font pair** — Instrument Serif + Inter is the canonical motionsites pair. PP Neue Montreal + PP Mondwest is the high-craft variant (Viktor Oddy #27). DM Sans + Material Symbols Rounded is the Nexto 404 outlier.
4. **`hf_<timestamp>_<uuid>.mp4`** filename pattern on any video — almost always motionsites-generated AI video.
5. **Easing curve** — `cubic-bezier(0.16, 1, 0.3, 1)` or `cubic-bezier(0.22, 1, 0.36, 1)` in motion configs — motionsites house easings.
6. **Single App.tsx file** with all components inline — motionsites prototype-tier organization (NHM #13 also has this even though it's a different toolchain).
7. **`borderRadius: { DEFAULT: '9999px' }`** override in Tailwind config — pill-everywhere brand decision, motionsites cluster signature (#01, #14).
8. **9 sections + macOS menu bar divider** (#22 Aura), **6-layer button shadow** (#27 Viktor Oddy), **`@property` animated gradient blobs** (#23), **per-tab semantic accent colors with auto-cycle** (#47 Stellar) — these are template-specific signatures, individually rare but each tightly bound to one variant.

## What this lineage means for synthesis

- **Treat the motionsites cluster as one design system being expressed across many brand identities.** The brand names (Aethera, Velorah, Asme, Mindloop, Nexora, Stellar, etc.) are decorative. The underlying material vocabulary is unified. When we extract a pattern from "Velorah," it's transferable to every other motionsites brand because they share the same primitives.
- **Cite design numbers, not brand names, when documenting reusable patterns.** Brand names are unstable; design numbers are the stable anchor in this knowledge base.
- **Acknowledge the AI-generated provenance.** The `hf_*` video filenames, the Cloudinary `Magnific_*` and `ChatGPT_Image_*` PNG filenames, the perfectly-timed asset clusters (Orbis's six videos all generated in one afternoon) — these are not stock footage purchased from a marketplace. They're synthesized assets shipped without disambiguation. Anyone shipping a batch-family design to production should plan to replace these with licensed or original assets.
- **The latent / unused code patterns (`.liquid-glass` defined-not-used in Axion #15, `.text-glass` defined-not-used in Vize #36, Inter weights 300-700 loaded-not-used in LinkFlow #12, Dirtyline font loaded-not-used in MicroVisuals #14) are template scaffolding.** They tell you the generator ships a standard boilerplate and the per-design author trims (or fails to trim) what's actually used. Treat these as performance-cleanup tasks for any production shipping.

## Open questions about the lineage

- Is motionsites.ai a publicly-accessible generator, a private internal tool, or a single designer's portfolio of generated outputs? The naming convention and asset-reuse patterns suggest a tool, not a person.
- The `user_38xzZboKViGWJOttwIXH07lWA1P` CloudFront path is the same across 40+ "brands." Either one user has generated all of these designs through a single account, or `user_38xzZ...` is the *tool's* asset bucket and the user-key is misleading.
- The split between Tailwind v3 (older cluster) and Tailwind v4 (newer cluster including NHM's React 19 / Vite 6 / Cloudinary outlier) is sharp enough that it suggests **the tool was upgraded between batch generation rounds**. Older designs in the batch (likely generated earlier) are v3; newer designs (generated later) are v4.
- Why does the `viktor/` R2 path show up across #23, #25, #55? Is "viktor" a designer's name, a project code, or unrelated? Could be a fingerprint that identifies a sub-cluster of the batch authored by one person who used a different CDN.
- What does the relationship between motionsites.ai (the marquee in Jack #07) and the CloudFront bucket (where all videos actually live) tell us about the generator's architecture? Is the marquee surfacing the same generator's catalog, or is motionsites.ai a separate gallery site that aggregates generator outputs?

---

*Captured: 2026-05-29. Most surprising finding: the same MP4 file `hf_20260314_131748_f2ca2a28-fed7-44c8-b9a9-bd9acdd5ec31.mp4` is Velorah's hero AND Asme v3 Service Card 1. Two "different brands" share the same brand-asset. Confirms CloudFront is a shared asset library across the cluster, not brand-specific storage — and demolishes any notion that these are independent design teams.*
