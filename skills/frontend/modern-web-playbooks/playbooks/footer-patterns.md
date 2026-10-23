# Footer Patterns

**Drawn from:** 55 design notes (01–55, excluding `_staging/` and `_inventory.md`)
**Last updated:** 2026-05-28

> **Batch context.** Most of this batch (~35/55) ships **no footer at all** — the studio-confidence move where the page terminates on the last content section. The designs that *do* commit to a footer fall into four loud camps: liquid-glass-footer-over-video (LUMINA, Halo, Lumina-adjacent), two-card asymmetric footers with massive wordmark anchoring (Kresna, Vize), email-style single-column footers (Design Rocket Certificates), and minimal single-line legal bars (Mindloop, Foundation, Aurora). The most ambitious footer in the batch is also the one outside the React ecosystem — Kresna ships as a vanilla single-file HTML with inline `<style>` and `<script>`, with a hand-fit watermark via `getBBox()` after `document.fonts.ready`.

## Patterns observed

### Pattern: Liquid-glass footer over full-page background video
**Used by:** #41 LUMINA, #44 Halo (footer area), partial in #16 Asme v3 (Section 5 Services cards as footer-adjacent)
**Confidence:** **MEDIUM — 2-3 samples, but distinctive enough to lead the playbook.**
**Recipe:**
- Page setup: `min-h-[115vh] overflow-x-hidden` so there's a small deliberate scroll past the viewport (15% additional height in LUMINA's case)
- Background video: `fixed inset-0 w-full h-full object-cover z-0` (note: `fixed`, not `absolute` — video stays pinned to viewport while content scrolls over it)
- Footer card: `mt-32 md:mt-64` (128px mobile, 256px desktop) below the upper content — substantial gap so the footer arrives as a "scroll-reveal" moment, not a centerpiece
- Footer container: `liquid-glass w-full rounded-3xl p-6 md:p-10 text-white/70`
- Inner grid: `grid-cols-1 md:grid-cols-12 gap-10 md:gap-12 mb-10` with brand block at `md:col-span-5` and link section at `md:col-span-7` (the 5/7 editorial split, narrower brand)
- Bottom bar: `pt-6 border-t border-white/10 flex flex-col md:flex-row items-center justify-between gap-6 md:gap-4`
- Footer reveal animation: `initial={{ opacity: 0, y: 40 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 1, delay: 0.4, ease: "easeOut" }}`
- 6-stop opacity ramp on white: 0.1 / 0.2 / 0.5 / 0.7 / 1.0 — no second hue, no gradient text
- Glass-on-video earns its credit here (foundations: "Glass works only when the background has variance")

**When it works:** Editorial publications, premium brand sites where the video is the protagonist and the footer is the signature. LUMINA reads as "the video is the page; the footer is where to find us."
**When it fails:** When the video has strong directional motion that makes the static footer feel anchored against a tide. When the `mt-32 md:mt-64` ramp pushes the footer below the fold on every viewport — visitor never sees it.
**Variations seen:** LUMINA uses Music2 + Facebook/Twitter/Youtube/Instagram social icons (the Music2 implies podcast/playlists — broadcasts that the publication has an audio surface). Halo (#44) bridges hero + use cases + footer using the recursive card-with-video pattern (footer inherits the hero's visual primitive at smaller scale).

---

### Pattern: Two-card asymmetric footer (media + content)
**Used by:** #32 Kresna, #36 Vize, partial in #44 Halo
**Confidence:** **MEDIUM — 2 confident samples, but each is unusually rich.**
**Recipe:**
- **Outer wrapper:** `max-w-[1150-1200px] mx-auto`
- **Grid:** `grid-template-columns: 350px 1fr` with `gap: 16px` — asymmetric 1:2.3 ratio (Kresna). Vize uses `grid-cols-1 md:grid-cols-2 lg:grid-cols-5` with `lg:col-span-2` for brand block (40% weight) and 3 link columns at 1 col each (60% weight).
- **Left card (narrow, brand-heavy):** `bg: brand gradient` (Kresna `linear-gradient(135deg, #5b9ffb 0%, #1e5dd7 55%, #1448be 100%)`) with `rounded-[28px] padding: 32px min-height: 340px flex flex-col justify-between`. Holds logo at top, tagline floats `margin-top: auto`, social row at bottom. Often includes an autoplaying video at `z-0 pointer-events-none` doing ambient motion.
- **Right card (wide, content-rich):** `bg: light gray` (Kresna `#f0f1f5`) with `rounded-[28px] padding: 40px overflow: visible` (critical — `overflow: visible` lets a floating badge break the top edge). Holds 2-column nav links + bottom row with copyright + CTA mini.
- **Both cards** round to the same generous radius (28px Kresna, 40px Vize inner). Concentric radius rule: if outer is 48px and inner sits at m-2 (8px), inner is 40px.
- **Brand-tinted shadow** on left card: `0 12px 40px rgba(21,76,189,0.25)` — the shadow takes the brand hue, not generic gray. Difference between "designed" and "themed."
- **Right card shadow** is much lighter (`0 4px 20px rgba(0,0,0,0.04)`) because it's doing less visual work
- **Bottom legal bar** sits INSIDE the outer grey wrap but OUTSIDE the white inner box — visually one surface, hierarchically separate

**When it works:** Brand pages where the footer is meant to be the showpiece, not the housekeeping. Kresna's footer reads as "this is also the most memorable component on the page."
**When it fails:** When the asymmetry doesn't earn its keep — if the left card doesn't have a video, it just looks like a tall sidebar.

---

### Pattern: Floating "extra" badge that breaks the card top edge
**Used by:** #32 Kresna (the "Feeling lucky?" cube with Caveat handwriting + hand-drawn SVG arrow)
**Confidence:** **LOW — 1 sample, but distinctively memorable.**
**Recipe:**
- Parent card has `overflow: visible` (critical — otherwise the badge clips)
- Badge: absolute positioning with `top: -36px right: 36px z-index: 10` — overflows the card's top edge by 36px
- Cube anatomy: 96×96 rounded square with `rotate(-10deg)` containing a glyph rotated `+10deg` (counter-rotation makes the glyph "level" while the cube tilts)
- 3-layer shadow on the cube: inner white highlight + inner black depth + outer drop shadow with brand-tinted alpha
- Hand-drawn SVG arrow with 3 explicit subpaths: curved Bezier `M3 20 C 6 14, 10 9, 18 5` + two arrowhead lines `M18 5 L 12 5` and `M18 5 L 18 11` — feels handcrafted because the control points are intentional, not formulaic
- Caveat handwriting font for the "Feeling lucky?" label

**When it works:** When the rest of the footer is calm and structured. The badge is the punctuation — a wink at the end of a long page.
**When it fails:** When the badge is functional (a CTA) but visually decorative (no hover state, no click target). Kresna's "Feeling lucky?" writes a check the interaction doesn't cash — should link to a random feature page or stay purely decorative without the question.

---

### Pattern: Massive faded wordmark behind/below footer
**Used by:** #32 Kresna (320px DM Sans wordmark at 4% black), #36 Vize (`text-[min(25vw,400px)]` Inter wordmark with hand-built SVG glass filter)
**Confidence:** **MEDIUM — 2 samples.**
**Recipe (Kresna — simple):**
- `<svg class="watermark" aria-hidden="true">` with `<text>` element at `font-size: 320px font-weight: 700 letter-spacing: -0.03em fill: rgba(0,0,0,0.04)`
- Watermark sits outside `.footer-wrapper` but inside the section, with `margin: -60px auto 0` — negative margin pulls it up underneath the wrapper at z-index 0
- JS-fit via `getBBox()`: `document.fonts.ready.then(fitWatermark)` measures the rendered bounding box after fonts load, updates SVG viewBox so glyph edges sit flush against container. Window resize listener re-fits. The only JS in the file.
- Brand reinforcement without volume — peripheral vision picks up the wordmark as identity

**Recipe (Vize — sophisticated):**
- `<h1>` at `text-[min(25vw,400px)] font-bold leading-none text-white px-4`
- SVG filter chain renders the wordmark as glass: 9 primitives (feFlood for white inner highlight bleeding from top, feFlood for black inner shadow rising from bottom, feMerge composite, feGaussianBlur for outer drop shadow `dy=4 stdDeviation=6`)
- Color is `text-white` source — filter generates visible appearance. Any other source would contaminate inner-shadow blacks.
- Reveal animation: `initial={{ opacity: 0, scale: 0.98 }} whileInView={{ opacity: 1, scale: 1 }} transition={{ duration: 1.8, ease: [0.16, 1, 0.3, 1] }}` — 1.8s is firmly narrative-duration ("stop, look, this is the brand")

**When it works:** When the brand has a confident wordmark (Kresna, Vize). The watermark reads as a designer's monogram at the bottom of a printed poster.
**When it fails:** When the wordmark is generic ("Tools", "Pro", "Inc."). When the fade is too dark — competes with content. When the `getBBox()` fitter doesn't account for font swap (FOUT) — wordmark may briefly render in fallback font.

---

### Pattern: Email-style footer with social icons + unsubscribe
**Used by:** #52 Design Rocket Certificates
**Confidence:** **LOW — 1 sample, but a complete pattern.**
**Recipe:**
- Page constrained to `max-w-[640px] mx-auto` — the canonical email-render width
- Footer: `bg-[#080808]` (between outer page `#050505` and card body `#111111` — "footer sits structurally between page and card")
- Contents (top-down):
  - 4-column nav row at `text-[11px] tracking-wide uppercase` with 4 small caps section labels + 3-4 links per section
  - Social row: 4-6 social icons in `liquid-glass rounded-full p-2.5` pills with hover scale 1.05
  - Legal block: `text-[11px] text-white/40` with copyright + physical address (`660 4th Street #443, San Francisco, CA 94107 USA` — a real SoMa mailbox suite, the kind of address a real startup with real lawyers actually uses)
  - Disclaimer block: institutional partnership clarification (e.g., Microsoft collaboration disclaimer with explicit endorsement boundaries)
  - Unsubscribe link: `text-[10px] underline text-white/30` at the very bottom

**When it works:** Marketing email landing pages, course conversion pages, newsletter signup flows. The legal-trust footer with specific physical address is the credibility accelerator.
**When it fails:** When the footer copy is generic boilerplate. The whole point is that *real legal copy with real addresses* signals "this is a serious business."

---

### Pattern: Simple line footer (copyright + 3-5 essential links)
**Used by:** #26 Mindloop, #28 Aurora (footer link below form), #30 Foundation (none in card itself), #15 Axion (none — page terminates), #50 Behind the Lens (minimal)
**Confidence:** **MEDIUM — 4-5 samples.**
**Recipe:**
- Container: `py-12 px-8 md:px-28 flex justify-between items-center` (Mindloop)
- Left: `© 2026 Brand. All rights reserved.` at `text-[12.5px] font-medium text-gray-500`
- Right: 3 inline links separated by spaces or dots (Privacy / Terms / Contact)
- Optional `border-t border-border/30` (Mindloop uses `0 0% 20%` border at 30% opacity ≈ #333 at 30% alpha)
- No newsletter inline, no social row, no sitemap

**When it works:** Portfolios, single-page brand statements, very-focused marketing pages. When the brand doesn't have enough surfaces to warrant a big footer.
**When it fails:** When the page is long (5+ sections) and the visitor needs sitemap-level navigation. When you want newsletter subscribers (no inline form means lost conversion).

---

### Pattern: Footer-as-page (the footer IS the protagonist)
**Used by:** #37 HAUL! (truck parallax with footer card on top), #32 Kresna (single-file deliverable)
**Confidence:** **LOW — 2 samples, but a distinctive inversion.**
**Recipe (HAUL!):**
- Three vertical zones on `#f8f9fa` page:
  1. **Spacer** (`50vh`/`30vh`) — solid `#FDFDFD` with tiny `tracking-[0.5em]` caps "View Below" as the scroll cue
  2. **Parallax theater** (`h-screen overflow-hidden`) — background image fills container, truck PNG lives in `absolute inset-x-0 bottom-0 h-full` layer scrolling at different rate via `useScroll + useTransform`
  3. **Footer card** — pinned to `absolute top-0 w-full` *inside the parallax theater*, with `pt-12 lg:pt-24` so it floats off the top edge. The card overlaps the hero — when you arrive, the card is roughly where a nav would be; as you scroll, the truck rises into the frame while the card stays put.
- Footer entrance: `initial={{ opacity: 0, y: -20 }} animate={{ opacity: 1, y: 0 }} duration: 0.8s easeOut` — slides down FROM ABOVE, against convention. Tells you "this object belongs above, not below; the bottom is where everything resolves."
- One saturated accent (orange `#f97316`) in exactly 3 touchpoints: SVG mark holder, link hover, social icon hover

**When it works:** Single-page brand experiences where the entire page is footer-and-hero compressed. Especially for industrial / freight / heavy-equipment brands where the "object rising into frame" metaphor matches the product.
**When it fails:** When you have other things below the fold — the inverted footer-up-top reads as a layout bug.

---

### Pattern: No footer (page terminates on last content section)
**Used by:** #04 Aethera, #05 Velorah, #08 Vex, #10 SkyElite, #11 VaultShield, #13 NHM (DIGGING INTO OUR PLANET'S PAST as 1-line mono caption is the footer), #16 Asme v3, #29 CodeNest, #38 DesignPro, #40 Nexora, #44 Halo (Use Cases section is the terminus), #45 Taskly, and many more
**Confidence:** **VERY HIGH — 20+ samples, the dominant non-pattern.**
**Recipe:**
- The hero section, optional 1-2 content sections, then... nothing. The page literally ends at the bottom of the last section.
- Sometimes a 1-line legal mono caption replaces the footer entirely (NHM's "DIGGING INTO OUR PLANET'S PAST" at the bottom of Section 3)

**When it works:** Anti-hero brand statements, pre-launch waitlists, portfolios where the page is one cinematic poem. The absence of footer is itself the statement.
**When it fails:** When the brand is selling something — no contact link, no terms link, no copyright. Foundations: "Footers as afterthought" is an anti-pattern. For real-ship sites this is a defect.

---

### Pattern: Newsletter signup pill inside footer
**Used by:** #32 Kresna (right card bottom-right), #23 Animated Gradient (footer column)
**Confidence:** **LOW — 2 samples.**
**Recipe:**
- Input + button in a single pill: `width: 310px border-radius: 12px padding: 5px` on the container
- Input inside: `padding: 11px 14px` with placeholder text
- Button inside: `padding: 11px 22px border-radius: 8px` (nested radius — outer 12px, inner 8px, gives the button its own object identity inside the pill)
- Button shadow: `0 6px 20px rgba(0,0,0,0.28)` lifts it above the container
- No client-side validation called out (a real-ship gap)

**When it works:** When the brand actually has a newsletter cadence to back the form. The nested-radius button-in-pill pattern is the canonical email-capture geometry.
**When it fails:** Without validation feedback, the form feels like a placeholder. Without a clear value prop ("Get one email a month" beats "Subscribe to our newsletter"), the conversion rate is low.

---

## Anti-patterns observed

- **No footer at all on a brand site** — universal in this batch. For studios/waitlists it's defensible; for real-ship product sites it's a defect (no contact, no terms, no copyright, no GDPR/privacy/imprint for European visitors).
- **"Feeling lucky?" badge with no click target** — Kresna's decorative badge implies action but doesn't fire one. Either link it or rephrase.
- **Footer disclaimer in 10px text below the readable threshold** — Design Rocket's unsubscribe text at `text-[10px] text-white/30` is technically present but functionally invisible. APCA fluent reading threshold not met.
- **Massive watermark with no `aria-hidden`** — screen readers will read "Kresna" or "Vize" as standalone text, then the wordmark above, then everything else. Always `aria-hidden="true"` on decorative giant wordmarks.
- **Watermark JS that depends on `document.fonts.ready` without a fallback** — Kresna's `getBBox()` fitter assumes fonts will load. If they fail, the watermark renders in fallback metrics and the viewBox is wrong. Need a `setTimeout` fallback.
- **No `prefers-reduced-motion` handling for the autoplay video in the footer card** — Kresna's left card video loops continuously without a fallback. Footers are the second-best place to forget motion-reduce (after heroes).
- **Hover-only social icons** — when the only difference between "default" and "hover" is opacity 70 → 100, keyboard users get no `:focus-visible` state. Universal gap.
- **Brand wordmark + brand watermark + brand cube** — Kresna has the "K" letter in 3 places (logo mark, lucky cube, watermark). On a small footer this is reinforcement; on a page that already uses the K motif heavily, it becomes monogram fatigue.

---

## Decision rules

- **If the brand is editorial / publication,** use the liquid-glass footer over full-page video (LUMINA). Footer is the destination of the page; the video is the wait until the footer arrives.
- **If the brand is showpiece SaaS / studio,** use the two-card asymmetric footer (Kresna, Vize) with brand-tinted shadow on the media card and a massive faded wordmark anchoring the composition below.
- **If the brand is a marketing email / course conversion page,** use the email-style single-column footer at `max-w-[640px]` with real legal copy and a specific physical address. Institutional credibility = real institution.
- **If the brand is a portfolio / single-page statement,** ship a simple line footer (Mindloop) OR no footer. Pick deliberately — the absence of a footer is itself a brand statement.
- **For nav cluster vocabulary:** edit the labels. "Discover / The Mission / Concierge" (LUMINA) reads editorial. "Product / Company / Resources / Legal" reads SaaS. Match the brand voice.
- **For sitemap depth:** 3-5 links per column is the right pocket. Fewer feels sparse; more feels like a documentation site.
- **For copyright line:** `© <year> <Brand>. All rights reserved.` is the safe default. Variants: `© <year> <Brand>` (modern Linear-style restraint), `Crafted by <Brand> · <year>` (studio register).
- **For social icons:** 36×36 is the lower bound; 44×44 is WCAG-correct. Use brand SVG paths for fidelity (Kresna's pattern), not generic Lucide icons that all look the same.
- **For newsletter capture:** if your brand has actual editorial cadence, include it. If not, leave it out — empty newsletter forms read worse than no form at all.
- **Always include:** copyright, privacy link, terms link. For European visitors: GDPR / imprint contact. For real-ship: a physical address.

---

## Open questions / gaps in our sample

- **No big-footer Stripe/Vercel-style sitemaps** — none of the batch has the 4-6 column footer with full sitemap, social icons, newsletter, legal, region/currency selectors all in one. The batch is too marketing-page-only.
- **No animated footer reveals tied to scroll** — most footers just appear (or don't). No design uses scroll-driven scrubbing to "build" the footer as you scroll into it.
- **No multi-region / language selectors** in any footer. International is unrepresented.
- **No theme-toggle in footer** — never. Designs are dark-only or light-only.
- **No status-page link or system-status indicator** — Stripe footer convention, missing across the batch.
- **No B-corp / climate badge** — Patagonia-style mission badges absent.
- **No app store badges** — even on mobile-app-feeling designs (Mindloop, Aura), no "Download on the App Store" / "Get it on Google Play" badges.
- **No "Made with [tool]" attribution** — Webflow / Framer / Bolt attribution links absent (designs hide their toolchain).
- **No Easter egg footers** — Linear's secret footer messages, GitHub's old "Set sail!" Easter eggs — nothing playful.
- **No 4-column or 5-column footer at scale** — Vize's 5-column (`lg:grid-cols-5` with `lg:col-span-2` brand) is the closest, but it's still half-implemented (only 3 link columns).

---
*Synthesizer: layout-family | 4 of 4 playbooks. Reads from: 55 design notes + foundations/layout-and-composition.md*
