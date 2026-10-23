# Section Rhythm

**Drawn from:** 55 design notes (01–55, excluding `_staging/` and `_inventory.md`)
**Last updated:** 2026-05-28

> **Batch context.** Roughly 35 of 55 designs in this corpus are single-section heroes — the dominant shape is "one full-viewport statement, then nothing." Only about 12 designs run a true multi-section landing with 3+ sections; another handful (e.g., Aura #22 at 9 sections) push toward standard SaaS marketing depth. The single-section bias matters: it means most batch designs *don't earn the rest*, which the foundations doc explicitly flags ("the hero earns the rest"). When a design *does* commit to a multi-section page, the choices made are unusually informative because they're rare.

## Patterns observed

### Pattern: Single-section anti-hero (zero scroll story)
**Used by:** #01, #02, #04 Aethera, #05 Velorah, #06 Asme v1, #08 Vex, #09 Asme v2, #10 SkyElite, #11 VaultShield, #12 LinkFlow, #14 MicroVisuals, #18 Bold Vision, #19 Cinematic Streaming, #21 (THIN), #29 CodeNest, #31 SENTINEL Spline, #33 Bloom, #35 (THIN), #38 DesignPro, #39 Xero, #40 Nexora, #42 Logoipsum, #43 dot., #45 Taskly, #46 Stellar.ai, #47 Hotel Booking, #48 Convix, #51 AI Builder
**Confidence:** **VERY HIGH — 25+ samples, the dominant batch architecture.**
**Recipe:**
- `relative min-h-screen` (or `h-screen overflow-hidden` for hard ceilings) outer container
- All content stacked vertically inside the single section: nav at top, headline block centered or bottom-aligned, optional dashboard/mockup/CTA below
- No footer in spec — the page literally ends at the section's bottom edge, or wraps in an `overflow-hidden` that swallows anything below
- Variants commit to either:
  - **Fluid `min-h-screen`** — page is *at least* viewport tall, content can extend (rare in this group)
  - **Fixed `h-screen`** + outer `overflow-hidden` — single viewport, no scroll possible (Nexora #40, Stellar #46, Calculator section embedded versions)

**When it works:** Brand-cinematic moments where the message IS the page. Studios with high brand confidence ("Studio confidence: visitors expect this level of restraint"). Pre-launch waitlists where there is no product to describe yet.
**When it fails:** When the brand promises depth (a studio with "Journal" in the nav, an enterprise SaaS with "Pricing/Docs/About") and never delivers it. Aethera (#04), Velorah (#05), Asme v1/v2 (#06/#09) all leave the brand promise un-discharged because nothing follows the headline — visitors scroll, find nothing, bounce.
**Variations seen:** Aurora (#28) is single-section but split-pane, so the page reads as 2 columns × 1 row rather than 1 row × 1 column. SENTINEL Spline (#31) hides the absence with `flex items-end` bottom-anchored content + drag-the-3D-scene affordance.

---

### Pattern: 2-3 section landing (sparse → dense → terminate)
**Used by:** #01 Cinematic Space-Travel, #03 Prisma, #15 Axion Studio, #50 Behind the Lens, #49 Neuralyn, #44 Halo (4 sections), #22 Aura (9 sections — outlier)
**Confidence:** **HIGH — 7 samples (excluding the 9-section outlier).**
**Recipe:**
- **Section 1: Hero.** Sets intent, sparse.
- **Section 2: One of three roles** — Bento/Capabilities (3 cards horizontally, density gradient), About/Mission, or Featured Work/Case Studies.
- **(Optional) Section 3: Closing role** — Pricing, Services Bento, or CTA card.
- Section padding alternates: tight `pt-6` between paired sections (so heading reads "into" the next), generous `py-28/40` between unpaired sections
- Use radial gradients at 2–3% white alpha to give long-dark pages "felt depth" — invisible-but-felt anchors (Asme v3 #16 places them at top of opening sections, at center of anchor sections)
- Often skip the footer — page terminates on the last content section

**When it works:** When 2-3 sections is genuinely all you need. Axion (#15) commits to Hero + About + Case Studies; Prisma (#03) commits to Hero + About + Features. Both are studios that can rely on the work doing the convincing.
**When it fails:** When the third section is filler (a marquee that doesn't earn its place, a testimonial that contradicts the hero's restraint). When you have 3 sections but the rhythm is dense-dense-dense — no breathing.
**Variations seen:** Prisma alternates with `dark → medium → dense` (sparse hero → medium About → 4-card grid). Axion is `sparse-dense-tighten` (shader hero → asymmetric About → tighter case-study grid). Behind the Lens (#50) is the magazine-grid variant — header → featured-post 2-col → 3-col grid → archive sections (it's a blog index, not a marketing page).

---

### Pattern: 5–6 section editorial multi-section landing
**Used by:** #13 NHM, #16 Asme v3, #26 Mindloop, #27 Viktor Oddy, #44 Halo (4 sections + recursive cards)
**Confidence:** **MEDIUM-HIGH — 5 samples, but each pushes the format meaningfully.**
**Recipe:**
- **Hero** (full viewport, sets the brand voice)
- **Section 2 — About / Identity** (sparse, often a manifesto paragraph or founder quote, sometimes with two-tone headline via italic-word emphasis)
- **Section 3 — Featured Video / Show-don't-tell** — full-width rounded `aspect-video` card with a `bg-gradient-to-t from-black/60` overlay and a bottom-overlay liquid-glass caption card
- **Section 4 — Philosophy / Split** — 2-column grid with media on one side, text on the other; often uses mixed-axis Framer Motion entrance (`x: -40` left, `x: 40` right meeting in center). Asme v3's `Innovation × Vision` brand-equation pattern lives here.
- **Section 5 — Services / Bento** — 2 to 4 liquid-glass cards, each with media on top + body below, slow `group-hover:scale-105 duration-700` on the media area
- **(Optional) Section 6 — CTA section** with HLS video bg and 2 buttons (Subscribe / Start Writing)
- **Footer** — minimal single line OR absent
- Section entrances use Framer Motion `useInView { once: true, margin: "-100px" }` so animations pre-trigger before viewport entry and have settled when the user is reading
- Heading scale steps *down* by section: `text-9xl` hero, `text-8xl` Section 2, `text-7xl` Section 3, `text-5xl` Services header — hierarchy across sections, not just within them

**When it works:** Brand expansion past the single-page-poem format. Asme v3 (#16) is the model — it stretches the Asme brand from a 1-section waitlist (v2) to a 5-section studio without losing the voice. Mindloop (#26) layers a hybrid-hosted video strategy (3 ambient CloudFront MP4s + 1 Mux HLS for the CTA).
**When it fails:** When the last section before the footer is the strongest (most users bounce mid-page in this format). When the nav doesn't anchor-link to the in-page sections (Asme v3's `Features / Pricing / About` nav doesn't actually anchor to its in-page About section — a missed-IA opportunity).
**Variations seen:** NHM (#13) uses 3 sections (light hero → light explore → dark collection) but layers an overlapping image (pterodactyl whose lower body sits in Section 3 and whose wings sit in Section 2 via `whileInView` y-percentage offsets) — turning 3 stacked sections into a continuous magazine spread.

---

### Pattern: Rounded-top section stacking with negative-margin overlap
**Used by:** #07 Jack 3D Creator
**Confidence:** **LOW — 1 sample, anecdote.**
**Recipe:**
- All sections after the hero get `rounded-t-[40px] sm:rounded-t-[50px] md:rounded-t-[60px]` (responsive corner radius scaling 40→60px)
- Each subsequent section gets `-mt-10 sm:-mt-12 md:-mt-14` (negative top margin) so it visually rests *on top of* the prior section
- Alternates `bg-#0C0C0C` ↔ `bg-#FFFFFF` to make each section a distinct surface
- Produces a "stacked card stack" page architecture where each section is a separate held object, not a flat band
- Jack's specific stack: Hero (dark) → Marquee → About → Services (white, rounded-top, overlaps) → Projects (dark, rounded-top, overlaps Services)

**When it works:** Personal portfolios, indie sites, design-engineer signature pages where each section is a self-contained chapter. The negative-margin overlap reads as deliberate craft.
**When it fails:** When you have more than 4-5 sections — the stacking becomes monotonous. When section heights are dramatically different — the rounded tops look ragged.

---

### Pattern: Bento section (cards as the body)
**Used by:** #16 Asme v3 Services, #20 Max Reed Portfolio, #25 Core Features (3 gradient cards), #30 Foundation, #34 Glowing Feature Cards, #44 Halo info row, #50 Behind the Lens grid, #54 Estimation Calculator output column
**Confidence:** **HIGH — 8 samples.**
**Recipe:**
- Section container `max-w-7xl mx-auto` (or `max-w-6xl` for tighter editorial)
- Header row: heading + optional right-aligned action
- Grid `grid-cols-1 md:grid-cols-2 lg:grid-cols-3` or `grid-cols-4` with `gap-4 to gap-8`
- Each card: `liquid-glass rounded-3xl overflow-hidden group` (dark mode) OR white card with `rounded-2xl border border-slate-200 shadow-sm` (light mode)
- **Card composition rule:** icon top-left (44×44 nested glass square `rounded-[0.75rem]`) + tag chips top-right (`justify-end`) + title-and-body bottom. Asymmetric internal layout in a symmetric grid — repeatable formula.
- Stagger card entrances by 0.15s with Framer Motion `whileInView { once: true }`
- Optional `group-hover:scale-105 transition-transform duration-700` on card media areas (slow 700ms — fast enough to feel responsive, slow enough to feel premium)

**When it works:** When you have 3–6 features that each deserve a tile. Max Reed (#20) is the master class — mixed-material bento where each cell picks its own substrate (video card, noise-textured quote card, billboard stat, marquee, contact). Cards 1-2-2 distribution (1 tall anchor card + 4 stacked cards) is a cheap modular-bento that reads magazine-rich.
**When it fails:** Foundations anti-pattern: "A bento that's just a feature list with rounded corners — no information density per tile." Equal-size tiles with no rhythm — boring. Animation in every tile — exhausting. When a tile is so small it can't contain meaningful content.
**Variations seen:** Glowing Feature Cards (#34) commits to 3 cards with chromatic identity per card — pink-orange / cyan / violet-magenta halos via blurred lower-opacity gradient copies. Asme v3 Services (#16) is a strict 2-card bento with deliberate visual rhyme. Foundation (#30) uses bento for the brand logo marquee — each brand gets *its own* hover gradient.

---

### Pattern: Layered card-in-card-in-page (concentric surfaces)
**Used by:** #28 Aurora, #30 Foundation, #36 Vize footer, #52 Design Rocket Certificates, #54 Calculator section
**Confidence:** **HIGH — 5 samples.**
**Recipe:**
- **Tier 1: Page bg** — neutral (`#F0F1F3`, `#050505`, `#f9fafb`)
- **Tier 2: Outer card body** — `rounded-[28-48px]`, slightly different lightness (`#E9EBEE`, `#111111`, `#FFFFFF`)
- **Tier 3: Inner card** — `m-2` (8px margin from outer), `rounded-[40px]` (concentric radius: `inner-radius = outer-radius - margin`), different lightness again
- The **concentric radius rule** is the craft tell — if the outer card is `rounded-[48px]` and the inner sits at `m-2` (8px), the inner must be `rounded-[40px]` (48 − 8) so the curves stay parallel
- Combined with `shadow-sm` or `shadow-2xl` and a `ring-1 ring-white/5` hairline, this produces unmistakable "card physically lifted off page" elevation in dark mode (where shadows don't read alone)

**When it works:** Anytime you want a single bounded "object" of content — settings panel, dashboard preview, contact card, email-format landing — and want it to feel like a physical object. Vize (#36) is the model: `#F0F1F3 → #E9EBEE → #FFFFFF` three-tier light stack stepping ~3% lightness each.
**When it fails:** When the lightness deltas are too big (the layers stop reading as "paper layers" and start reading as "panels"). Apple-tier: keep deltas small.

---

### Pattern: Density alternation (sparse → dense → sparse → dense)
**Used by:** #01 (sparse hero → dense 3-card bento), #22 Aura (sparse hero → 40px menu bar divider → dense inbox mockup → sparse feature → quiet logos → dense testimonial cards → loud pricing → sparse final CTA), #26 Mindloop (sparse → dense → sparse → narrative → dense → sparse → minimal), #27 Viktor Oddy (sparse hero → dense marquee → sparse quote → medium pricing → dense carousel → medium projects → sparse partner → dense footer)
**Confidence:** **MEDIUM — 4 explicit samples, but every multi-section design implicitly does this.**
**Recipe:**
- Foundations rule: "Each section's job is to make the next one breathe."
- After every dense section (bento, mockup, carousel), insert a sparse section (quote, logo strip, single-CTA card)
- Use the macOS menu bar trick (Aura #22) — a 40px-tall full-width strip as a *structural pivot* that resets the page's rhythm without taking real estate. It says "now we enter the app" with maximum semantic weight in minimum vertical space.
- Loudest sections should be 6+ sections apart (Aura's gradient hero and watermarked pricing rhyme without competing because they're 8 sections apart)

**When it works:** Long-scroll narratives, SaaS marketing pages with 6+ sections. The rhythm is what makes a long page feel composed instead of padded.
**When it fails:** When every section tries to be loud. When the dense sections cluster without rest. When "rest" is just empty white space — rest needs to have a *role* (quote, single CTA, logo strip).

---

### Pattern: Recursive card-with-video (same primitive at multiple scales)
**Used by:** #44 Halo (hero card + Use Cases right column both rounded video cards), #16 Asme v3 (hero pattern + Featured Video section + Services cards)
**Confidence:** **LOW — 2 samples, but the pattern is structurally distinctive.**
**Recipe:**
- Establish a primary visual primitive at hero scale — `rounded-2xl overflow-hidden + background video + foreground content`
- Reuse the same primitive at smaller scale for content sections — `rounded-3xl overflow-hidden + section background video + section content` (radius scales up as the card scales down, signaling "softer at smaller sizes")
- Each instance keeps the same composition rules: media full-bleed inside the card, foreground content padded `p-6 md:p-10`

**When it works:** When the brand's primary visual mode is "card-with-video" — committing to it across scales makes the page feel authored by a single hand. Halo (#44) uses the recursion to signal "this is a system, not a one-off hero."
**When it fails:** When the recursive card is the only place a video appears below the fold — feels like "the hero, again, smaller." Needs to genuinely demonstrate something the hero couldn't (e.g., Halo's Use Cases card shows a different mode of the product).

---

### Pattern: Overlapping image bridges section boundary
**Used by:** #13 NHM (pterodactyl), partial in #27 Viktor Oddy
**Confidence:** **LOW — 1 confident sample.**
**Recipe:**
- Place an image at the top of section N+1 with `z-0` (negative-relative z to its parent's higher z)
- Use Framer Motion `whileInView` to animate `y: "-65%" → y: "-78%"` as the section scrolls into view — the image rises *further* up into section N's space
- Section N must have a deliberately-sized spacer (220–450px) at its bottom as the runway for the bleed
- Section N+1's content sits at `z-30` (higher than the image), so the image lives behind it
- Net effect: the image's lower body is in section N+1, its upper body is in section N — the section boundary cuts through the creature/object

**When it works:** When you have a single editorial bridge moment between two visually distinct sections (light section → dark section). Reads as magazine-spread continuity.
**When it fails:** When you do this on every section transition — feels gimmicky. The spacer needs to be there or the image lands on top of section N's content.

---

### Pattern: Sticky-stacking section cards
**Used by:** #07 Jack 3D Creator
**Confidence:** **LOW — 1 sample.**
**Recipe:**
- Container: `h-[85vh]` per card
- Each card has `position: sticky; top: 96-128px`
- Card scale formula: `targetScale = 1 - (totalCards - 1 - index) * 0.03` — each older card shrinks 3% per slot below the current
- `top: ${index * 28}px` offset gives each card a 28px peek of the prior cards
- Uses `useScroll` + `useTransform` to drive the scale per card

**When it works:** Project galleries, portfolios where each section is a self-contained case study. Borrowed from Active Theory / Tom Bogner.
**When it fails:** When you have more than 5 cards — the stack becomes a wall. When the cards aren't visually distinct — feels like a slideshow stuck in mud.

---

## Anti-patterns observed

- **Sections without footers** — the studio-confidence move (Aethera, Velorah, Asme v1/v2/v3, Halo, Bloom) gambles that the visitor will read the hero and leave. Often the bet is wrong — for a real brand site, leaving without a contact link, social handle, or legal page is a defect, not a feature. Foundations: "The footer is the forgotten section, but it's as designed as the hero."
- **Three sections of the same type in a row** — three feature cards then three bento then three testimonials. The eye learns the format after the first row and the third one feels like padding. Alternate types.
- **A single section that tries to be 5 sections** — when one section has a hero AND a bento AND a testimonial AND a CTA stacked vertically, the section is actually a page and should be broken up. Bold Vision (#18) gets away with it because the elements share a single composition (top/middle/bottom zones), but adding a 4th element would collapse it.
- **No anchor-linking from nav to sections** — Asme v3 has nav links (Features/Pricing/About) that don't anchor to the in-page sections, even though those sections exist. Trivially fixed with `href="#about"`; commonly missed.
- **Latent footer space** — when the page ends with a 200px gap before the viewport bottom because the last section was sized for desktop. Audit at multiple viewport heights.

---

## Decision rules

- **If you have 1 section,** make sure the brand actually warrants it — restraint requires confidence. Pair with at least nav, hero, and signature visual material (video, glass, mockup). No footer is fine for waitlists; for a real product, add at least a 1-line footer.
- **If you have 2-3 sections,** plan the rhythm: sparse hero → medium middle → terminate or sparse close. The 3rd section's job is to *close*, not to add a 4th feature.
- **If you have 5-6 sections,** follow Asme v3's spine: Hero → About → Featured Video → Philosophy split → Services bento → CTA. Each section earns its place by doing one job the prior section can't do.
- **If you have 8+ sections (long SaaS marketing page),** alternate density rigorously. Use structural pivots (Aura's macOS menu bar trick) to reset the page's rhythm.
- **If you want the page to feel like one continuous spread,** use overlapping images or rounded-top section stacking with negative-margin overlap to bridge section boundaries.
- **If sections share a visual primitive** (card-with-video, glass-on-glass), commit to it across scales (recursive cards). The radius should scale up as the scale scales down.

---

## Open questions / gaps in our sample

- **Mega-menu nav backed by a deep section tree** — no batch design ships this. Suggests batch is mostly marketing/landing, not product surface with broad IA.
- **Long-form editorial scroll (Stripe-style narrative)** — partially present in Aura (#22) at 9 sections, but no design commits to a 12–15-section deep-product story.
- **Section reveals tied to a sticky horizontal scroll** — no Apple AirPods Pro / Cursor multi-act hero in the batch. All scroll is vertical, all sections are full-width.
- **Sections with subgrid alignment** — CSS subgrid is foundations-blessed but doesn't show up in the briefs. Most multi-section designs use independent grids per section, not parent-aligned subgrids.
- **Inline pricing tables as their own section** — only #54 Estimation Calculator has it, and it's a different format (interactive calculator, not a static pricing table).

---
*Synthesizer: layout-family | 4 of 4 playbooks. Reads from: 55 design notes + foundations/layout-and-composition.md*
