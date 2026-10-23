# Glass and Surface
**Drawn from:** 55 design notes (01 cinematic-space-travel through 55 nexto-404)
**Last updated:** 2026-05-28

The single most identifiable cross-cutting pattern in this batch is **liquid-glass**. It appears in 20+ designs with byte-identical CSS — confirming a motionsites.ai shared utility — and varies in three documented dialects (light, strong, dark) plus several edge-case adaptations. Beyond glass, the batch reveals a consistent system of surface elevation that uses tint instead of shadow on dark mode, and three-tier near-white surface elevation on premium light mode. The most engineered surface moves (Vize #36's SVG glass filter, Convix #48's clip-and-bleed dashboard, Behind the Lens #50's L-bracket frames) reveal what's possible when you push past the default toolkit.

## Patterns observed

### Pattern: liquid-glass (the canonical)
**Used by:** #01 Cinematic, #05 Velorah, #06 Asme v1, #08 Vex Ventures (dark variant), #09 Asme v2 (7 surfaces), #14 MicroVisuals, #16 Asme v3 (13 surfaces), #17 Orbis (11+ surfaces), #22 Aura, #26 Mindloop, #29 CodeNest, #33 Bloom (two-tier), #34 Glowing Cards (custom variant), #36 Vize, #41 LUMINA, #42 Logoipsum, #45 Taskly (strong variant), and more. **20+ designs, ~40% of the batch.**

**Recipe (byte-identical across batch):**
```css
.liquid-glass {
  background: rgba(255, 255, 255, 0.01);
  background-blend-mode: luminosity;
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
  box-shadow: inset 0 1px 1px rgba(255, 255, 255, 0.1);
  position: relative;
  overflow: hidden;
}
.liquid-glass::before {
  content: '';
  position: absolute;
  inset: 0;
  padding: 1.4px;
  border-radius: inherit;
  background: linear-gradient(180deg,
    rgba(255, 255, 255, 0.45) 0%,
    rgba(255, 255, 255, 0.15) 25%,
    rgba(255, 255, 255, 0) 50%,
    rgba(255, 255, 255, 0) 50%,
    rgba(255, 255, 255, 0.15) 75%,
    rgba(255, 255, 255, 0.45) 100%);
  -webkit-mask: linear-gradient(#fff 0 0) content-box,
                linear-gradient(#fff 0 0);
  mask-composite: exclude;
  pointer-events: none;
}
```

**Key tuning values (all load-bearing):**
- `rgba(255, 255, 255, 0.01)` — 1% white tint. **Critical:** above 0.05 reads plasticky. The 1% is "barely there" by design.
- `backdrop-filter: blur(4px)` — light blur for chrome elements. Heavy enough to soften background detail without dissolving it.
- `background-blend-mode: luminosity` — preserves underlying color, only mixes lightness. This is what makes the glass *feel* like glass refracting light, not like a flat white tint.
- `inset 0 1px 1px rgba(255, 255, 255, 0.1)` — top-edge inner highlight. Simulates "light catching the upper rim of the glass surface."
- The `::before` rim-light gradient: 6 stops, symmetric (0.45 at top → 0.15 → 0 → 0 → 0.15 → 0.45 at bottom). Bright at top and bottom edges, transparent in middle. Combined with `mask-composite: exclude`, it produces a *ring of light around the pill's border* that simulates how real glass catches light at its edges.
- `padding: 1.4px` on the `::before` — sub-pixel border thickness for crispness at retina densities.

**When it works:** Over a strong background (video, mesh gradient, photo). Foundations explicitly state: "Glass on a flat white background — pointless." The batch confirms — all heavy-glass designs sit over video.
**When it fails:** On flat surfaces. On low-contrast backgrounds where the rim-light gradient doesn't catch enough light.
**Variations seen:** See `.liquid-glass-strong`, dark-glass, glass-pill, white-on-white strong glass — each below.

---

### Pattern: liquid-glass-strong (heavier blur for primary CTA)
**Used by:** #01 Cinematic, #14 MicroVisuals (nav-embedded CTA), #33 Bloom (hero panel + outer right-panel container), and ~10+ other designs as primary CTA chrome.
**Recipe:**
```css
.liquid-glass-strong {
  background: rgba(255, 255, 255, 0.01);
  background-blend-mode: luminosity;
  backdrop-filter: blur(50px);
  -webkit-backdrop-filter: blur(50px);
  box-shadow:
    4px 4px 4px rgba(0, 0, 0, 0.05),
    inset 0 1px 1px rgba(255, 255, 255, 0.15);
  position: relative;
  overflow: hidden;
}
.liquid-glass-strong::before {
  /* same as .liquid-glass but with brighter stops: */
  background: linear-gradient(180deg,
    rgba(255, 255, 255, 0.5) 0%,
    rgba(255, 255, 255, 0.2) 25%,
    rgba(255, 255, 255, 0) 50%,
    rgba(255, 255, 255, 0) 50%,
    rgba(255, 255, 255, 0.2) 75%,
    rgba(255, 255, 255, 0.5) 100%);
}
```
- `blur(50px)` is **12.5× the light variant** — at this radius the background completely dissolves into a smooth tinted wash.
- Additional `4px 4px 4px rgba(0, 0, 0, 0.05)` outer shadow (the light variant has none) — adds material presence.
- Inner highlight brightened to `0.15` (vs `0.1`).
- Rim-light stops brightened to `0.5 / 0.2` (vs `0.45 / 0.15`).

**Two-tier glass system in practice:**
- **`.liquid-glass`** (4px blur, subtle rim) — chrome, nav, social icons, secondary CTAs.
- **`.liquid-glass-strong`** (50px blur, brighter rim + outer shadow) — primary CTAs, hero panels, emphasis containers.

**Used together by:** #01 Cinematic, #14 MicroVisuals, #33 Bloom — confirmed two-tier discipline.

**When it works:** As a hierarchy signal. The strong tier reads as "physical material"; the light tier reads as "label printed on the underlying material."
**When it fails:** When over-used (more than 1 strong glass per visible viewport — competes with itself).

---

### Pattern: Dark liquid-glass variant
**Used by:** #08 Vex Ventures.
**Recipe:**
```css
.liquid-glass-dark {
  background: rgba(0, 0, 0, 0.4);
  background-blend-mode: luminosity;
  backdrop-filter: blur(4px);
  box-shadow: inset 0 1px 1px rgba(255, 255, 255, 0.1);
}
.liquid-glass-dark::before {
  /* softer stops: 0.3 / 0.1 / 0 / 0 / 0.1 / 0.3 */
}
```
- `rgba(0, 0, 0, 0.4)` instead of `rgba(255, 255, 255, 0.01)` — dark tint instead of light tint.
- Softer rim stops `0.3 / 0.1` (vs `0.45 / 0.15`) — the highlight is dimmer.
- Used specifically when the video background is **bright or varied** — the dark glass darkens the underside so headlines read against unpredictable luminance.

**When it works:** Bright or chromatically-varied background video where light glass would lose contrast. Vex Ventures' explicit choice — the video plays raw (no overlay), so the glass has to darken its underside.
**When it fails:** On dark video (loses contrast with surroundings). On flat dark surfaces (looks like an opaque grey panel).

---

### Pattern: glass-pill (distinct from liquid-glass)
**Used by:** #06 Asme v1 — specifically tuned for floating pills on video backgrounds.
**Recipe:**
```css
.glass-pill {
  background: rgba(255, 255, 255, 0.04);
  backdrop-filter: blur(16px) saturate(180%);
  -webkit-backdrop-filter: blur(16px) saturate(180%);
  border-radius: 9999px;
  box-shadow: none !important;
}
```
- 4% white background (vs liquid-glass's 1%) — more visible tint.
- `blur(16px)` (vs 4px) — heavier blur.
- `saturate(180%)` — boosts saturation of underlying frame to compensate for the heavy blur softening colors.
- `box-shadow: none !important` — explicitly removes any inherited shadow.

**When it works:** Floating pills on video backgrounds where shadows would muddy edges and saturation boost compensates for backdrop blur softening colors.
**When it fails:** On flat surfaces (saturation boost has nothing to boost). When mixed with `.liquid-glass` on the same page (inconsistent tinting).

---

### Pattern: Strong white-on-white glass
**Used by:** #45 Taskly — Liquid Glass adapted for light backgrounds.
**Recipe:**
```css
.glass-on-white {
  background: rgba(255, 255, 255, 0.3);
  backdrop-filter: blur(50px);
  -webkit-backdrop-filter: blur(50px);
  border-radius: 16px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  box-shadow: inset 0 4px 4px 0 rgba(255, 255, 255, 0.25);
}
```
- 30% white background (vs 1% — much more committed tint).
- `blur(50px)` — wide enough to pick up blurred ellipses behind and refract them.
- **`1px solid rgba(0, 0, 0, 0.1)` outer stroke** — the critical adaptation. Without this, 30%-white glass on pure white background disappears entirely.
- `inset 0 4px 4px rgba(255, 255, 255, 0.25)` — light-from-above material physics inner highlight.

**When it works:** Floating UI on light backgrounds. Pure-white pages where glass needs material identity. Apple Liquid Glass adapted for light context.
**When it fails:** Without the black-stroke (glass becomes invisible). Without blurred elements behind it (50px blur has nothing to refract).

---

### Pattern: bg-white/30 backdrop-blur-xl glass
**Used by:** #24 RIVR — heavier white glass for light-mode hero card.
**Recipe:** `bg-white/30 backdrop-blur-xl` (30% white tint + Tailwind's `blur-xl` = `backdrop-filter: blur(24px)`).
- More opaque than glass-pill (30% vs 4%) but lighter than Taskly's strong variant.
- Used for cards over light backgrounds where the underlying video is more atmospheric than chromatic.

**When it works:** Hero cards over softly-colored video backgrounds. Less committed than Taskly's strong variant.
**When it fails:** Over rapidly-changing video (the 30% tint reveals shifting underneath, can be distracting).

---

### Pattern: Two-tier glass system (light + strong)
**Used by:** #01 Cinematic, #14 MicroVisuals, #33 Bloom — most disciplined.
**Decision rule:**
- Use `.liquid-glass` for chrome (nav, social icons, chips, secondary CTAs).
- Use `.liquid-glass-strong` for primary CTAs and hero panels.
- Never more than 1 strong glass per visible viewport (otherwise compete).
- Never mix with other glass dialects (glass-pill, dark-glass) on the same page.

**When it works:** As a binary hierarchy signal. The strong tier always reads as "the action to take," the light tier always as "background chrome."
**When it fails:** When extended to a third tier (gets confusing).

---

### Pattern: Concentric radius rule
**Used by:** #36 Vize — explicit `inner-radius = outer-radius - margin` math.
**Recipe:**
- Outer container `rounded-[48px]` (48px radius).
- Inner card `m-2` (8px margin) + `rounded-[40px]` (40px radius).
- 48 - 8 = 40. Inner radius = outer radius minus margin.
- Result: the curves stay concentric. The inner card's corner traces parallel to the outer container's.

**When it works:** Card-in-card compositions where the visual relationship between containers should feel architectural. Apple's System Settings, Notion's document containers.
**When it fails:** When the margin is too small (< 4px — visual collapses). When the radii are too tight (< 16px — the parallel curves don't read).

---

### Pattern: Page-as-card outer frame
**Used by:** #24 RIVR (`min-h-screen bg-[#f0f0f0]` + `p-3 md:p-5` + `rounded-[1.5rem] md:rounded-[3rem]` inner card), #30 Foundation (`max-w-[1400px] mx-auto rounded-[48px]` card on `#f9fafb` page), #37 HAUL (footer card with `bg-white/95`), #44 Halo (`max-w-[88rem]` + `rounded-2xl` card on `#F5F5F5`), #48 Convix (`bg-[#ededed]` + `p-3 sm:p-4` + `rounded-2xl sm:rounded-3xl` card).
**Recipe:**
- Page bg in a neutral light tone (`#ededed`, `#f0f0f0`, `#f9fafb`).
- Outer padding/gutter (`p-3 md:p-5` or `p-4`).
- Single inner card at large radius (`rounded-2xl` to `rounded-[48px]`) with `overflow-hidden`.
- Card holds the entire hero content + video background.

**When it works:** When you want the hero to feel like a *physical object on a desk*, not a section of a page. The outer mat gives the card visual breathing room.
**When it fails:** On viewport heights < 700px (the card compresses). When the outer gutter is too small (< 12px — card touches viewport edges, defeats the framing).

---

### Pattern: Three-tier near-white surface elevation
**Used by:** #15 Axion (page `#EFEFEF` / about `#FFFFFF` / case studies `#F5F5F5`), #36 Vize (page `#F0F1F3` / outer card `#E9EBEE` / inner card `#FFFFFF`), #37 HAUL (page `#f8f9fa` / spacer `#FDFDFD` / card `bg-white/95`), #48 Convix (page `#ededed` / card `#d9d9d9` / dashboard tray `#f5f2ee` — warm contrast).
**Recipe:**
| Surface | Hex range | Role |
|---|---|---|
| Page | `#ededed`–`#f8f9fa` | Outer mat |
| Card | `#d9d9d9`–`#E9EBEE` | Card body |
| Tray / content | `#f5f2ee`–`#FFFFFF` | Held content |

**Convix's temperature contrast:** the dashboard tray (`#f5f2ee`) is *warmer* than the surrounding card (`#d9d9d9`) — reads as a different material (paper-on-aluminum vs aluminum-only).
**When it works:** Premium light-mode where the brand wants "card-on-mat-on-tray" physicality without going dark.
**When it fails:** When all three values are too close (no perceived elevation). When the tray is colder than the card (reads as flat).

---

### Pattern: Dark-mode tint elevation
**Used by:** #03 Prisma (`#000` bg / `#101010` card / `#212121` card-of-card), #41 LUMINA (`#000` page / glass card), #52 Design Rocket (`#050505` page / `#111111` card / `#080808` footer).
**Recipe:** Elevation = lighter tint, not shadow (shadows don't read on near-black).
- Step the lightness by 5-10% per tier.
- Always tint elevated surfaces *up* (lighter), never down (darker).
- The page is the most-recessed; cards step toward white.

**Design Rocket's three-tier dark recipe:**
- `#050505` page (~OKLCH 0.04)
- `#111111` card body (~OKLCH 0.11, 7% delta)
- `#080808` footer (between page and card — anchor below)

**When it works:** Any dark-mode design with multiple surface elevations. Foundations rule: "in dark mode, elevation = lighter tint."
**When it fails:** When using shadows on dark mode (invisible against near-black). When tinting too aggressively (lighter than ~30% feels like a panel).

---

### Pattern: Hand-tuned multi-layer card shadow
**Used by:** #40 Nexora (named custom CSS variable `--shadow-dashboard`).
**Recipe:**
```css
--shadow-dashboard:
  0 25px 80px -12px rgba(0, 0, 0, 0.08),
  0 0 0 1px rgba(0, 0, 0, 0.06);
```
- Layer 1: deep drop shadow (25px down, 80px blur, -12px spread, 8% black) — elevation.
- Layer 2: inset-style ring (`0 0 0 1px` with 6% black) — edge definition.

**When it works:** When a single shadow doesn't carry the elevation + edge simultaneously. Layer 1 = "this is lifted." Layer 2 = "this has a defined border."
**When it fails:** When over-used. Reserve for the page's most important held surface.

---

### Pattern: SVG glass filter (handcrafted feMerge)
**Used by:** #36 Vize footer — massive 400px wordmark "vize" rendered through a 9-primitive SVG filter chain.
**Recipe:**
- `feFlood` produces a white inner highlight bleeding from top.
- `feFlood` produces a black inner shadow rising from bottom.
- `feComposite` operations carve those into the wordmark shape.
- `feMerge` combines highlight + shadow + drop shadow (`feGaussianBlur` + `feOffset` for `dy=4, stdDeviation=6`).
- Result: depth fabricated entirely from filter primitives — no gradient, no image, no mesh, no backdrop-filter.

**When it works:** When the wordmark is the brand signature and needs material depth on a flat background. Type-foundry-grade craft.
**When it fails:** Performance on weak GPUs. Browser inconsistencies in SVG filter rendering.

---

### Pattern: Clip-and-bleed dashboard
**Used by:** #48 Convix.
**Recipe:**
- Hero card `overflow-hidden` + fixed height `calc(100vh - 24px)`.
- Inside: dashboard tray is taller than card height, *literally extends past the bottom*, gets clipped by the card's overflow.
- No gradient fade, no explicit scroll prompt — the visual incompleteness IS the prompt.

**When it works:** When the dashboard preview is the proof element. Clip says "there's more product than you can see, scroll for it." Same paradigm family as Halo (#44)'s clipped marquee, RIVR (#24)'s scroll-promise.
**When it fails:** Without the parent's `overflow-hidden` (dashboard spills below). On viewport heights too short (entire dashboard hides).

---

### Pattern: L-shaped corner brackets on cards
**Used by:** #50 Behind the Lens — every video tile in the photography grid.
**Recipe:**
- Four white L-shaped corner brackets at 15px inset from card edges.
- Each bracket ~12-15px on each leg (photographic-frame aesthetic).
- Visible always — they don't appear only on hover.

**When it works:** Photography / film / camera-related contexts where viewfinder-reticle aesthetic is on-brand.
**When it fails:** On busy backgrounds where the brackets disappear. On SaaS contexts where they read as "industrial decoration."

---

### Pattern: Global texture overlay
**Used by:** #02 TOONHUB (SVG `fractalNoise` baseFrequency=0.9, container opacity 0.4, 200×200px tile), #17 Orbis (`/texture.png` at `z-50, mix-blend-mode: lighten, opacity: 0.6`), #20 Max Reed (inline SVG data URI `feTurbulence` at `mix-blend-mode: soft-light, opacity: 0.55` on teal cards).
**Recipe variants:**
- **Subtle grain (foundations canonical):** SVG `feTurbulence` at 1-3% opacity, `multiply` or `overlay` blend, used to reduce banding on cheap displays.
- **Loud grain (Orbis variant):** Full PNG at 60% opacity, `lighten` blend, lifts dark regions without affecting highlights. The page reads "printed."
- **Card-specific grain (Max Reed):** SVG noise overlay only on accent-color cards (`#324444` teal). The noise turns flat color into film-grain warmth.

**When it works:** Subtle (1-3%) for any saturated background — reduces banding, adds analog warmth. Loud (60%) when the brand vertical is editorial/maximalist (Orbis) or studio-craft (Max Reed).
**When it fails:** Over text without contrast audit (texture can crush legibility). Without `prefers-reduced-motion` consideration if grain is animated.

---

### Pattern: Inverted-corner SVG mask
**Used by:** #24 RIVR (BottomRightCorner).
**Recipe:**
- Two absolutely-positioned SVG paths in the BG color (`#f0f0f0`) carve concave quarter-circles into the outer container.
- Path math: `30.9279 ≈ 56 * (1 - 1/√2)` — the inset distance for a quarter-circle that bulges concavely into a 56×56 square.
- Result: the inner solid card appears to *cut INTO* the rounded parent rather than sit on top.

**When it works:** Whenever a card sits in the corner of a rounded container and needs to feel "carved out." Bento corners, card-in-card compositions, modal corner notches.
**When it fails:** Without the SVG paths matching the BG color exactly (the cutout shows seams). At very small sizes (the math breaks).

---

### Pattern: Two-color disc pill button
**Used by:** #24 RIVR (both primary CTAs).
**Recipe:**
- Pill button with an icon-in-a-disc on the left and a text label on the right.
- The disc background sits *between* the button bg and the page bg.
- Navy button + `bg-white/20` disc + white icon — chromatic break that reads "this is the action target."
- Mirror variant: white button + `bg-[rgba(30,50,90,0.1)]` disc + navy icon.

**When it works:** As an upgrade to flat pill buttons. The disc provides a focal point within the button — the user's eye lands on the disc, not the button center.
**When it fails:** When the disc is the same color as the button (defeats the purpose). On small buttons (< 32px height — disc disappears).

---

### Pattern: Subtle radial gradient depth on dark
**Used by:** #16 Asme v3 — `bg-[radial-gradient(ellipse_at_top,_rgba(255,255,255,0.03)_0%,_transparent_70%)]` on About section; `bg-[radial-gradient(ellipse_at_center,_rgba(255,255,255,0.02)_0%,_transparent_60%)]` on Services section.
**Recipe:** 2-3% white at center, transparent at edge. Invisible but *felt* as soft luminance.
**When it works:** On long dark-mode pages to prevent uniform-void feeling. Same principle as foundations' "grain overlay on dark mode" — analog warmth without color.
**When it fails:** Above 5% opacity (becomes a visible halo).

---

## Anti-patterns observed

### Glass on flat surfaces
**Found in:** No clean offenders in the batch — all heavy-glass designs sit over video or gradient.
**Why it matters:** Glass needs underlying chromatic complexity to refract. On a flat surface, it's pointless.

### Liquid-glass dead CSS
**Example:** #15 Axion — 60+ lines of `.liquid-glass` and `.liquid-glass-strong` rules defined but never applied.
**Why it matters:** Bloat undermines the design's restraint argument.
**Fix:** Delete unused CSS.

### Mixed glass dialects on the same page
**Example:** Some batch designs include both `.liquid-glass` and `glass-pill` classes from inherited template scaffolding.
**Fix:** Pick one dialect per page. The two-tier `.liquid-glass` + `.liquid-glass-strong` is the cleanest hierarchy.

### Glass over rapidly-changing video without backdrop tint
**Found in:** Various designs that use light glass over high-saturation video frames.
**Why it matters:** When the video has bright frames, glass loses contrast and content beneath becomes illegible.
**Fix:** Use dark glass variant (`rgba(0,0,0,0.4)` background) for bright videos, or add `bg-black/15` scrim.

### Pure black bg without elevation
**Found in:** No clean offenders — most batch dark-mode designs step elevation with tint.
**Why it matters:** Foundations rule — on dark mode, elevation must use tint, not shadow.

### Glass corners not matching parent corners
**Found in:** Some early experiments — when a glass card with `rounded-3xl` (24px) sits inside a container with `rounded-2xl` (16px), the misaligned curves break visual coherence.
**Fix:** Use the concentric radius rule (`inner = outer - margin`).

---

## Decision rules

1. **Default to the two-tier system:** `.liquid-glass` (4px blur) for chrome, `.liquid-glass-strong` (50px blur + outer shadow) for primary CTAs.
2. **Glass works only over video, mesh gradients, or photos.** Don't use on flat surfaces.
3. **For dark video backgrounds, use light glass.** For bright/varied video, use dark glass variant.
4. **For light-mode glass (page is white), use the strong-white-glass recipe** — 30% white bg + 50px blur + `border 1px solid rgba(0,0,0,0.1)` + inset highlight.
5. **Apply concentric radius rule for card-in-card compositions.** `inner = outer - margin`.
6. **For dark-mode elevation, use lighter tint (5-10% per tier), not shadow.**
7. **For premium light-mode elevation, use three-tier near-white system** with optional temperature contrast on the deepest tier.
8. **Use subtle radial gradients (2-3% white at center) on long dark pages** to prevent uniform-void.
9. **Use texture overlays at 1-3% opacity for subtle grain, 50-60% for maximalist effect.** Always with appropriate blend mode (`overlay` for warmth, `lighten` for lift, `soft-light` for film grain).
10. **Build the rim-light gradient with 6 stops symmetric (0.45 / 0.15 / 0 / 0 / 0.15 / 0.45 at top/upper-mid/lower-mid/lower-mid/upper-mid/bottom).** Brighten to 0.5 / 0.2 for the strong variant.
11. **`padding: 1.4px` on the `::before` rim.** Sub-pixel for retina crispness.
12. **For "page-as-card" framing, use `p-3 md:p-5` gutter + `rounded-2xl` to `rounded-[48px]` inner card.** Page bg neutral light tone.

---

## Open questions / gaps

- Why is the canonical `.liquid-glass` recipe byte-identical across 20+ designs? Strong evidence of a shared utility file in motionsites.ai templates. Is this an emerging design-system primitive?
- The dark-glass variant (Vex #08) is uncommon — only one batch instance. Is this because most batch videos are calm enough that light glass survives, or because the pattern isn't yet widely understood?
- `glass-pill` (Asme v1 #06) with `saturate(180%)` is unusual. Are other batch designs missing a saturation boost that would help their backdrop blur?
- The strong-white-glass recipe (Taskly #45) requires three additions over canonical (30% white, black stroke, inner highlight). Why isn't this standardized? Is it a Light Liquid Glass v2 in waiting?
- Vize's SVG filter wordmark proves you can build glass without `backdrop-filter`. Is this performance-equivalent at scale, or is it only viable for static elements?
- Convix's clip-and-bleed is structurally elegant but uncommon. Is it a discoverable pattern or did the designer hand-craft it?
- The three-tier near-white system with *temperature* contrast (Convix's `#f5f2ee` warm tray) is novel. Most three-tier systems are pure lightness. Is temperature-as-elevation the next iteration?
- Several batch designs have `prefers-reduced-transparency` opportunities (glass collapses to solid fallback) but none implement it. Is this a missing accessibility consideration?
