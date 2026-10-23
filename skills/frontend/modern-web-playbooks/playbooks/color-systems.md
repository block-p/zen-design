# Color Systems
**Drawn from:** 55 design notes (01 cinematic-space-travel through 55 nexto-404)
**Last updated:** 2026-05-28

Color across the batch is overwhelmingly disciplined. The dominant pattern is **monochrome-plus-one-accent** (per the foundations), but the batch reveals several specific dialects of that pattern: opacity-on-near-black for video heroes, single-hue alpha-blends as a complete UI palette, three-tier near-white surface elevation for premium light-mode, and saturated-dark bases that replace the "neutral near-black" convention. The outliers (Orbis #17, Duolingo #53, Animated Gradient #23) commit to multi-color palettes through *strict role assignment*, not through palette expansion.

## Patterns observed

### Pattern: liquid-glass class as system surface
**Used by:** #01 Cinematic, #05 Velorah, #06 Asme v1, #08 Vex Ventures (dark variant), #09 Asme v2 (7 surfaces on hero), #14 MicroVisuals, #16 Asme v3 (13 surfaces), #17 Orbis (11+ surfaces), #19 Cinematic Streaming, #22 Aura, #26 Mindloop, #29 CodeNest, #33 Bloom (two-tier), #34 Glowing Cards (custom variant), #36 Vize, #41 LUMINA, #42 Logoipsum, #45 Taskly (strong variant), and more. **20+ designs, ~40% of the batch.**
**Recipe (canonical):**
```css
.liquid-glass {
  background: rgba(255, 255, 255, 0.01);
  background-blend-mode: luminosity;
  backdrop-filter: blur(4px);
  box-shadow: inset 0 1px 1px rgba(255, 255, 255, 0.1);
  position: relative;
}
.liquid-glass::before {
  content: '';
  position: absolute;
  inset: 0;
  padding: 1.4px;
  border-radius: inherit;
  background: linear-gradient(180deg,
    rgba(255,255,255,0.45) 0%,
    rgba(255,255,255,0.15) 25%,
    transparent 50%,
    transparent 50%,
    rgba(255,255,255,0.15) 75%,
    rgba(255,255,255,0.45) 100%);
  -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
  mask-composite: exclude;
  pointer-events: none;
}
```
Key tuning:
- `rgba(255,255,255,0.01)` — 1% white tint. Barely visible. **0.05+ reads plasticky.**
- `backdrop-filter: blur(4px)` — light blur for chrome. `blur(50px)` is the `liquid-glass-strong` variant for primary CTAs.
- `background-blend-mode: luminosity` — preserves underlying color, only mixes lightness.
- The masked `::before` with 6-stop gradient produces a "rim light" effect — bright at top/bottom edges, transparent middle. This is what gives glass its "wet" look on video backgrounds.

**When it works:** Over a strong background (video, mesh gradient, photo). Foundations' rule: "Glass on a flat white background — pointless." Velorah and Asme variants prove the inverse — glass on dark cinematic video is the canonical use.
**When it fails:** On flat surfaces. On low-contrast backgrounds where the rim-light gradient doesn't catch enough light.
**Variations seen:**
- **`.liquid-glass-strong`** (heavier blur 50px + outer shadow + brighter rim stops 0.5/0.2) — used for primary CTAs in many designs.
- **Dark liquid-glass** (#08 Vex Ventures) — `rgba(0,0,0,0.4)` + softer rim stops 0.3/0.1 — for bright/varied background video.
- **`bg-white/30 backdrop-blur-xl`** (#24 RIVR) — heavier-tinted white glass for light-mode contexts.
- **Strong white-on-white glass** (#45 Taskly) — `backdrop-blur-[50px]` + `rgba(255,255,255,0.3)` + `1px solid rgba(0,0,0,0.1)` outer stroke + `inset 0 4px 4px 0 rgba(255,255,255,0.25)` highlight — the critical adaptation for glass on pure white.
- **`glass-pill` class distinct from `liquid-glass`** (#06 Asme v1) — `rgba(255,255,255,0.04)` + `backdrop-filter: blur(16px) saturate(180%)` + `box-shadow: none !important` — tuned for floating pills on video backgrounds.

---

### Pattern: Pure black bg + opacity ramp (strict 0% saturation HSL)
**Used by:** #33 Bloom (most strict — every CSS var is `0 0% X%`), #16 Asme v3, #26 Mindloop, #49 Neuralyn (`0 0% 0%` pure black for luminosity-blend requirement), #51 AI Website Builder (pure `#000000`).
**Recipe:**
- Background: `#000000` or `hsl(0 0% 0%)`
- Text: white with opacity steps (white/100, white/80, white/60, white/40)
- No tinted near-blacks (no `#0d0d10`, no `#0a0a0e`)
- Justification: when running luminosity blends or strict monochrome chrome, the saturation-zero HSL prevents any hue contamination.

**When it works:** When the video supplies all color complexity (Bloom, Asme v3) or when material blend modes depend on pure black (Neuralyn's luminosity-blend dashboard).
**When it fails:** When the background is exposed (no video) and the pure-black-on-pure-white text combo creates harsh contrast. Foundations cite this exact anti-pattern.

---

### Pattern: Three-tier near-white surface elevation
**Used by:** #37 HAUL (page `#f8f9fa` / spacer `#FDFDFD` / card `bg-white/95`), #36 Vize (page `#F0F1F3` / outer card `#E9EBEE` / inner card `#FFFFFF`), #48 Convix (page `#ededed` / card `#d9d9d9` / dashboard tray `#f5f2ee` — warm-on-cool contrast), #15 Axion (page `#EFEFEF` / about `#FFFFFF` / case studies `#F5F5F5`), #50 Behind the Lens (pure white, no mat).
**Recipe (three steps with deliberate temperature contrast):**
| Surface | Hex | Role | Temperature |
|---|---|---|---|
| Page | `#ededed` or `#f0f1f3` or `#f8f9fa` | Outer mat | Neutral cool |
| Card | `#d9d9d9` or `#E9EBEE` | Card body | Neutral cool, darker |
| Tray/dashboard | `#f5f2ee` or `#FFFFFF` | Held content | Warm contrast |

**Vize's concentric radius rule:** `inner-radius = outer-radius - margin`. 48px outer + 8px margin = 40px inner, keeping the curves concentric.
**When it works:** Premium light-mode where the brand wants "card-on-mat" physicality without committing to dark mode. The temperature step makes the cream tray feel like a different material (paper-on-aluminum).
**When it fails:** When all three values are too close (no perceived elevation). When the bottom-most tray is colder than the card (reads as flat).

---

### Pattern: Saturated dark base instead of near-black
**Used by:** #05 Velorah (HSL `201 100% 13%` — deep navy at full chroma), #43 dot. (warm cream page `#F3F4ED`), #17 Orbis (`#010828` deep navy ground), #52 Design Rocket (`#050505` near-pure-black with subtle warmth).
**Recipe:** Pick a brand hue, set HSL at 100% saturation and 10-15% lightness. White foreground. No accent. Deep navy here, but burgundy, forest, indigo, charcoal all work the same way.
**Velorah's commitment:** `--background: 201 100% 13%` is approximately `#003954` — a midnight blue with full chroma. White-on-saturated-blue reads softer than white-on-black because blue absorbs more in the eye than black.
**When it works:** When the brand needs single-hue identity at saturation that survives only in low-lightness territory. Reads cinematic, not digital.
**When it fails:** When the saturated bg fights the video underneath (the video's natural colors clash with the brand hue).

---

### Pattern: Single-hue alpha-blend palette
**Used by:** #24 RIVR (entire UI built from `rgba(30,50,90,*)` at 9 alpha steps), #12 LinkFlow (5-step single-hue green ramp `#85AB8B / #4b5b47 / #3d5638 / #336443 / #2d3a2a / #1f2a1d`).
**Recipe:** Sample alpha values from one rgb root to manufacture an entire scale.
RIVR's 9-step alpha ramp:
- `rgba(30,50,90,0.05)` — icon disc bg
- `rgba(30,50,90,0.1)` — borders
- `rgba(30,50,90,0.2)` — icon circle border
- `rgba(30,50,90,0.6)` — captions/sublabels
- `rgba(30,50,90,0.8)` — buttons, icons
- `rgba(30,50,90,0.9)` — badge/stat numeric text
- `rgba(30,50,90,0.95)` — documentation title
- `rgba(30,50,90,1.0)` — button hover (full)

**When it works:** When brand identity is one hue and you want chromatic unity without the rigidity of a fixed 12-step ramp. Especially for products where the brand has one hero color (navy, deep green, burgundy).
**When it fails:** When the page needs semantic state colors (red error, green success) — single-hue can't represent state diversity.

---

### Pattern: Strict accent discipline — 2 or 3 touchpoints
**Used by:** #11 VaultShield (`#7342E2` purple: 3 touchpoints — logo dot, plus symbols, CTA), #18 Bold Vision (`#5E0ED7` purple: 3 touchpoints — logo dot, + symbols, CTA), #29 CodeNest (`#5ed29c` green: 4 touchpoints — eyebrow, terminating period, CTA, link hover), #37 HAUL (`#f97316` orange: 3 touchpoints — logo square, link hover, social icon hover), #36 Vize (`#31A8FF` blue: 2 touchpoints — wordmark, CTA), #48 Convix (`#ef4d23` orange: 3 touchpoints — logo, eyebrow accents, "Pages" link with dropdown), #54 Calculator (`#FF5656` red-pink: 4 touchpoints — radio dot, slider thumb, checkbox interior, price annotations).
**Recipe:** Pick one saturated hue. Hold it to ≤4 surfaces. Every accent placement must be load-bearing. The discipline is what keeps it premium.
**CodeNest's "green period" move:** Coloring the *terminating period* of `LAUNCH YOUR CODING CAREER.` as the single brand-color punctuation. Punctuation-as-branding — the eye treats the green dot as a sigil.
**When it works:** When the brand vertical demands premium restraint. Linear (purple), Vercel (blue), Plain (red) all hold this line religiously.
**When it fails:** When tempted to extend to a fifth surface "for accessibility" or "for the empty state" — every new placement degrades the existing ones.

---

### Pattern: Vivid green family for AI / dev brands
**Confirmed instances:**
- **#29 CodeNest** — `#5ed29c` (sea-green, OKLCH ~0.78 0.16 158) — softer
- **#31 SENTINEL AI** — HSL `119 99% 46%` ≈ `#00ed05` (vivid acid green) — harder
- **#34 Glowing Cards Studio variant** — `#7DD3FC / #06B6D4` (cyan rather than pure green, but adjacent)

**Recipe:** Hue near 119-160, saturation 80-99%, lightness 46-50% (on dark) or 70-80% (on light).
**When it works:** Security/AI vocabulary ("active," "live," "monitored") signals "system online." Used sparingly on dark backgrounds where saturation reads vibrant without being eye-burning.
**When it fails:** On white backgrounds (retina-burning) or in more than ~3 surfaces (overwhelming).

---

### Pattern: Per-card gradient hues with shared structure
**Used by:** #34 Glowing Cards (3 cards, each with its own 137° linear-gradient: Hardware pink-orange, Studio white-cyan, Motion blue-magenta), #25 Core Features (3 cards, same radial-gradient structure with different hue pairs converging to shared `#F4F8F9`).
**Recipe (Glowing Cards):**
- Same gradient angle (137° not 135° — the 2° nudge breaks the corner-to-corner mathematical line into something atmospheric).
- Same stop positions (0/45/100% — not 0/50/100, pushes the second hue past midpoint).
- Different hue pairs per card.
- Same gradient appears in *two roles*: sharp border (via padding-box + border-box trick) AND blurred halo (45px blur, 0.6 opacity, behind the card).

**Recipe (Core Features):**
- Same radial-gradient origin (`circle at 50% 0%`).
- Same stop percentages (0% → 30% → 60% → 100%).
- All gradients converge to **the same surface color** (`#F4F8F9`).
- Hues rotate per card; structure stays constant.

**When it works:** When showing N parallel features that each need a chromatic identity. Color rotation system, not three separate palettes.
**When it fails:** When the gradients don't share a structural anchor — N bespoke gradients reads as chaos.

---

### Pattern: Animated `@property` gradient blobs
**Used by:** #23 Animated Gradient CTA (5 blobs + 5 size animations, 15 `@property` declarations).
**Recipe:**
- Declare custom properties as `<percentage>` via `@property` so the browser can interpolate.
- 5 blobs as overlapping radial-gradients.
- Animate position via `--x1`/`--y1` etc.
- Animate size via `--size1` etc.
- **Critical:** durations must be coprime (3.0, 3.5, 3.8, 4.0, 4.2, 4.6, 5.0, 5.5, 6.0, 6.5 seconds) so phase alignment doesn't reveal mechanical looping.
- `ease-in-out` on every animation (mimics fluid dynamics).

**When it works:** Consumer-friendly brands wanting fully-saturated multi-hue gradients without static-gradient banality. The animation hides any single-frame "gradient picker" feeling.
**When it fails:** Without coprime durations (eye catches synchronicity within seconds). Without `prefers-reduced-motion` fallback (continuous saturated motion).

---

### Pattern: Three-color palette with strict role assignment
**Used by:** #17 Orbis.Nft (navy ground / cream type / neon for cursive only / purple gradient for one button class).
**Recipe:** Pick N hues, assign each to *exactly one role*, refuse all overlap.
Orbis's discipline:
- `#010828` deep navy = ground
- `#EFF4FF` cream = all primary text
- `#6FFF00` neon = Condiment cursive overlays + one underline bar
- `#b724ff → #7c3aed` purple gradient = Section 3 card chevron button (48x48 circles)

Total purple surface area: ~3% of one section. Each color has one job, zero overlap.
**When it works:** When the brand vertical needs more chromatic identity than monochrome-plus-one allows. Breaks the foundations' "three becomes a clown costume" rule by giving each color exactly one role.
**When it fails:** When a fourth color creeps in (an "info blue" for a tooltip, a "warn orange" for an alert) — the system collapses.

---

### Pattern: Mix-blend-mode as color generator
**Used by:** #17 Orbis (`mix-blend-exclusion` on Condiment cursive overlays — per-frame color inversion against video), #41 LUMINA (selection styling `selection:bg-white/20`), #45 Taskly (`mix-blend-screen` on hue-rotated WEBM orb), #49 Neuralyn (`mixBlendMode: "luminosity"` on dashboard PNG over video).
**Recipes:**
- **`exclusion`** (Orbis): inverts the underlying pixel against source color per channel. Cursive text picks up a different color every video frame. Source color must be saturated to survive the inversion.
- **`screen`** (Taskly): adds light values. Black underneath stays black; anything else gets brighter and shifted toward source hue. Used to drop a WEBM's black background through white canvas.
- **`luminosity`** (Neuralyn): preserves underlying hue/saturation, applies source's lightness. Dashboard image takes its hue from the video beneath while keeping its own structural shapes.
- **`lighten`** (#02 TOONHUB grain, #17 Orbis texture): only the lighter of source/dest per channel — used for global texture overlays that lift dark regions without affecting highlights.

**When it works:** When the design needs per-frame color variation without JS, or material-physical compositing (luminosity = real material; exclusion = optical illusion).
**When it fails:** Without controlled background colors. Without pure black anchors. On low-contrast displays where the math gets crushed.

---

## Anti-patterns observed

### Pure white text on pure black (#000 on #FFF)
**Foundations warn:** Use gray-12 on gray-1 instead.
**Found in:** Many batch designs that use `text-white` on `bg-black`. Justified on video backgrounds (the video softens the contrast). Problematic when video is muted/absent.

### Inverting light mode for dark mode
**Found in:** No batch examples — every design that supports dark mode designs it natively. The batch respects the foundations rule.

### Two-cream-token bug
**Found in:** #03 Prisma — `#E1E0CC` inline and `#DEDBC8` Tailwind both as "the brand cream." Inevitable adjacent-use shows lightness step.
**Fix:** Pick one, delete the other, refactor.

### Brand color used for both error and CTA
**Found in:** No clean examples in the batch (most use single accent only). Foundations warn against red-for-everything overload.

### Forgetting focus-ring contrast
**Found in:** ~all batch designs — no `:focus-visible` ring is specified in *any* brief. Universal accessibility gap.

### Dim-text contrast cliff
**Examples:** #04 Aethera (`#6F6F6F` body sneaks past 4.5:1 only because text is 18px+), #45 Taskly (orange stars #FF801E inside a white-glass pill), #41 LUMINA (white/40 metadata on video).
**Fix:** Always audit at smallest size on darkest video frame.

### Liquid-glass dead CSS
**Example:** #15 Axion — 60+ lines of `.liquid-glass` and `.liquid-glass-strong` rules defined but never applied. Bloat the design's restraint argument.
**Fix:** Delete unused CSS.

---

## Decision rules

1. **Default to monochrome-plus-one-accent.** Match Linear, Vercel, Stripe — pick one brand hue, hold it religiously to 2-4 surfaces.
2. **On dark video heroes, default to opacity ladder, not gray ramp.** White at 100/80/70/60/50/40 covers all hierarchy needs.
3. **For "premium light-mode," build three-tier near-white surface elevation.** Page → card → tray, with the tray warmer than the card+page (`#f5f2ee` warm vs `#d9d9d9` cool).
4. **For liquid-glass on video, use canonical `rgba(255,255,255,0.01)` + 4px blur + 6-stop mask-composite rim.** Bump to `blur(50px)` + outer shadow for `liquid-glass-strong` primary CTAs.
5. **For liquid-glass on flat white, add `border 1px solid rgba(0,0,0,0.1)` + `inset 0 4px 4px rgba(255,255,255,0.25)` highlight.** The stroke is non-negotiable on white — without it, 30%-white glass disappears.
6. **Restrict accent to ≤4 touchpoints.** Bold Vision's 3-touchpoint discipline is the model.
7. **For saturated dark bases, use HSL `<hue> 100% 13%`.** Velorah's recipe transfers to burgundy, forest, indigo.
8. **For single-hue alpha palettes, ramp at 0.05 / 0.1 / 0.2 / 0.6 / 0.8 / 0.9 / 0.95 / 1.0.** RIVR's 9-step is the model.
9. **For animated gradients, use `@property` + coprime durations.** Three blobs minimum, durations at 3-7 second range.
10. **Use OKLCH for ramps and gradients.** Avoid muddy middles. When sRGB is required, place explicit mid-stops (Glowing Cards' Motion gradient passes through lavender at 45% to avoid blue-to-magenta gray middle).
11. **Selection styling is a fingerprint.** Default `selection:bg-white/20` on dark, `selection:bg-black/10` on light. Most sites inherit OS default.
12. **Brand-tinted shadow on primary CTA.** `box-shadow: 0 4px 24px rgba(<brand-rgb>, 0.28)` — colored shadow at ~25% alpha of the brand color. VaultShield's recipe transfers.

---

## Open questions / gaps

- Why does the motionsites.ai batch default to `9999px` border-radius (pill everywhere) when other premium sites (Linear, Vercel) use 12-16px softer corners? Is this a tool-default or a stylistic choice?
- Multiple batch designs use commercial-licensed fonts via `db.onlinewebfonts.com` (legally gray) — is this a tool-template issue?
- The "warm `#FF801E` stars on white-glass" pattern in #45 Taskly is genuinely novel — the only orange on the page is a 5-pixel-each rating row. Is this an emerging "tiny chroma at the trust signal" technique?
- Why does Foundation #30 use `#0a1b33` (near-navy) rather than pure black for headline text on white? Multiple light-mode designs default to near-blue-blacks rather than pure black. Is this dark-mode-inversion aesthetic seeping into light mode?
- The two batch designs that use the most saturated three-or-more-color palettes (Orbis #17, Duolingo #53) both signal "playful" / "loud" vertical. Is saturated-multi-color a vertical signal now?
- Aurora #28's `#324444` teal is described as "muted retro-industrial." How many batch members would benefit from one-off character hues like this vs the default Linear-purple / Stripe-purple / generic-blue?
