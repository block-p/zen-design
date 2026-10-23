# Navigation Patterns

**Drawn from:** 55 design notes (01–55, excluding `_staging/` and `_inventory.md`)
**Last updated:** 2026-05-28

> **Batch context.** The dominant batch nav is the **floating liquid-glass pill at top-center**, sized to a `max-w-5xl` or smaller container, containing logo + nav links + 1–2 CTAs. It appears in 25+ designs. The pill is so canonical here that it's worth thinking about *which* pill variant a design uses (light/dark glass, rounded-full vs `rounded-[16px]`, embedded-CTA-on-right vs separate-CTA, asymmetric padding) rather than whether it has one. Mobile nav is the weakest part of the batch — half the designs don't ship a mobile menu pattern at all, and the ones that do split between three solves: full-screen overlay, slide-in-from-right drawer, and dropdown-from-top.

## Patterns observed

### Pattern: Floating liquid-glass pill nav at top
**Used by:** #01, #04, #05, #06, #09, #11, #12, #14, #16, #26, #30, #38, #40, #43, #45, #47, #48 — and many more
**Confidence:** **VERY HIGH — 25+ samples, the batch default.**
**Recipe:**
- Wrapper: `fixed top-4` to `top-6` (16–24px inset from top), `left-1/2 -translate-x-1/2`, `z-50`, `whitespace-nowrap`
- Pill: `rounded-full px-6 py-3` (or `rounded-[16px] px-4 py-2` for the "softer pill" variant)
- Material: `.liquid-glass` (or `.glass-pill`) — `rgba(255,255,255,0.01–0.05)` background, `backdrop-filter: blur(4–50px)`, `background-blend-mode: luminosity`, inset top-edge highlight, `::before` pseudo-element with masked gradient rim-light (stops `0.45 / 0.15 / 0 / 0 / 0.15 / 0.45` for `.liquid-glass`, `0.5 / 0.2` for `.liquid-glass-strong`)
- Contents (left to right): logo wordmark + nav links cluster + right-cluster (often Sign Up + Login OR a single embedded CTA pill)
- Max-width: `max-w-5xl mx-auto` (64rem) for nav-only container; some pages cap tighter at `max-w-[760px]` (Convix) or `max-w-3xl` (dot.) for intimacy
- The center pill is the visual interest; the chrome itself feels like proscenium framing

**When it works:** Premium dark-mode brand sites, single-page landings, anything with a video or busy bg behind it (glass needs variance to read). The pill commits the design to a single corner language.
**When it fails:** On flat-white backgrounds (foundations: "Glass on a flat white background — pointless"). Taskly (#45) is the exception — it tunes the glass with `backdrop-blur-[50px]` + `rgba(255,255,255,0.3)` fill + `1px solid rgba(0,0,0,0.1)` stroke + inner highlight to make glass work on pure white.
**Variations seen:**
- **Embedded-CTA pill:** the nav-end CTA is *inside* the pill with asymmetric padding `pl-6 pr-1` (LinkFlow #12) so the CTA hugs the right edge.
- **Two-pill nav:** logo as separate left-edge mark, glass pill in the center containing just nav links, CTA as another floating element on the right.
- **Solid-white pill on flat page:** Foundation (#30) and Convix (#48) use `bg-white rounded-full border border-neutral-200 shadow-sm` instead of glass — works on light bg without needing video.

---

### Pattern: Hanging black/dark pill nav from top edge
**Used by:** #03 Prisma
**Confidence:** **LOW — 1 sample, but distinctive.**
**Recipe:**
- Container: `bg-black rounded-b-2xl md:rounded-b-3xl px-4 py-2 md:px-8`
- Position: hangs from the top edge of the inset-rounded hero card (only bottom corners rounded, top corners flush with the section edge)
- Sits *over* an inset rounded hero (`p-4 md:p-6` outer padding + `rounded-2xl md:rounded-[2rem]` inner card) — the nav hangs *from* the card's top edge, not *floats above* it
- 5 nav items with extreme gap variation `gap-3 sm:gap-6 md:gap-12 lg:gap-14`
- Resting link color is alpha-reduced cream `rgba(225,224,204,0.8)`, hover is full opacity `#E1E0CC`

**When it works:** Editorial / studio sites where the hero is itself a card-on-stage. The hanging pill reads as proscenium — it tells you "the hero is a contained presented object, not a section."
**When it fails:** When the hero isn't inset (the hanging pill needs a top edge to hang from). On long pages where the nav should sticky-on-scroll — hanging-from-top doesn't translate.

---

### Pattern: Bottom-floating pill nav
**Used by:** #30 Foundation (`absolute bottom-10`)
**Confidence:** **LOW — 1 sample, but it inverts the convention.**
**Recipe:**
- Position: `absolute bottom-10 left-1/2 -translate-x-1/2`
- Same pill anatomy as top-floating pills (`bg-white/90 backdrop-blur-2xl rounded-full px-4 py-2`)
- Used inside the page-as-card hero (the nav lives inside the rounded card, not above the page)

**When it works:** When the hero card has its message in the upper-left "editorial corner" and the nav can occupy the lower-center without competing. Reads as confident — most sites would never put primary nav at the bottom.
**When it fails:** On long scroll pages — bottom-anchored nav stops being visible after the hero. Best for single-screen heroes only.

---

### Pattern: Sticky/fixed solid navbar (transparent or backdrop-blurred)
**Used by:** #10 SkyElite, #19 Cinematic Streaming, #22 Aura, #50 Behind the Lens, #53 Duolingo Style Guide, #55 Nexto 404
**Confidence:** **MEDIUM — 6 samples.**
**Recipe:**
- `fixed top-0 left-0 right-0 h-16 bg-white border-b border-[#E5E5E5] z-50` (Duolingo) OR `relative w-full transparent z-20` (Hotel)
- Contains: logo (left) + nav links (center or center-left, `hidden md:flex`) + action buttons (right)
- Heights: 48–80px range, with 64px as canonical for fixed-nav documentation
- Often paired with a `max-w-7xl mx-auto px-8 to px-28` content container
- Hover on links: color-only transition (`hover:text-foreground` or `hover:text-[brand-color]`), no underline, no scale, no pill highlight

**When it works:** When the page is long (>3 sections) and the nav must stay visible. When the surface design needs to feel "document-like" rather than "glass-floating" (Duolingo's style guide, Nexto's error page, Behind the Lens's blog index).
**When it fails:** On video backgrounds where the solid white bar fights the imagery — should switch to transparent or glass instead.
**Variations seen:** Aura (#22) uses an unusual "fixed vertical guide lines" pattern alongside the nav — two `fixed inset-y-0 left-1/2 -translate-x-[calc(50%+36rem)]` 1px white/10 hairlines that run the full viewport height as invisible-until-noticed alignment anchors.

---

### Pattern: No nav (or near-empty top bar)
**Used by:** #02 TOONHUB (just brand label top-left, "DISCOVER IT" bottom-right), #18 Bold Vision (logo + 3 nav links + hamburger only), #34 Glowing Feature Cards (no nav at all — section-only)
**Confidence:** **LOW-MEDIUM — 3 samples.**
**Recipe:**
- Brand label / wordmark at `absolute top-6 left-4 sm:left-8`, often uppercase with tracking
- No center nav, no CTA cluster — the page IS the navigation
- Sometimes paired with a single corner CTA or arrow link at the opposite side

**When it works:** Single-product hero pages (TOONHUB's figurine carousel), feature sections embedded in a larger page (Glowing Cards #34), brand statements where adding nav would betray the silence.
**When it fails:** When the page is a brand site and visitors expect nav — they assume the page is broken or partial.

---

### Pattern: Three-zone classic nav (logo / links / CTA pair)
**Used by:** #08 Vex Ventures, #10 SkyElite, #11 VaultShield, #15 Axion Studio, #20 Max Reed, #22 Aura, #30 Foundation, #41 LUMINA (in footer context), most non-pill batch designs
**Confidence:** **HIGH — 12+ samples, the second-most-common nav shape.**
**Recipe:**
- `flex items-center justify-between max-w-7xl mx-auto px-6 md:px-12 lg:px-20 py-5`
- Left: logo wordmark (24–28px Inter Semibold or display face) + sometimes a small tagline mono "the creative studio of ..." (Viktor Oddy)
- Center: nav links `hidden md:flex` (3–5 items, `text-sm font-medium text-white/70 hover:text-white` or near-equivalent)
- Right: action cluster — either Sign In + Sign Up (light/dark theme), Login + Start Free (asymmetric weight: text link + filled pill), or single primary CTA
- Hover: opacity dim or color shift, no scale, no pill background

**When it works:** Universal — works for marketing landings, SaaS products, studios, blogs. The "padding cascades down to hero" rule (use the same `px` value for nav and below-content) is what makes the page feel held together.
**When it fails:** When there are too many nav items (6+) or the action cluster has 3+ buttons of similar weight.

---

### Pattern: Right-cluster asymmetry (Sign Up plain + Login pill)
**Used by:** #06 Asme v1, #09 Asme v2, #14 MicroVisuals, #16 Asme v3
**Confidence:** **MEDIUM — 4 samples (one design family).**
**Recipe:**
- Right cluster: "Sign Up" as plain text link + "Login" as a liquid-glass pill
- The visual weight goes to *Login*, which is **inverted from waitlist convention** — usually Sign Up gets the pill
- Likely a deliberate inversion to make Sign Up read as the "casual" entry point and Login as the "members area"

**When it works:** On a real product with an active user base where most visitors are returning.
**When it fails:** On a pre-launch waitlist where there's no one to log in — the visual hierarchy lies about the conversion target.

---

### Pattern: Hamburger transformation to X (mobile menu trigger)
**Used by:** Almost every batch design with a mobile menu — #10 SkyElite, #11 VaultShield, #12 LinkFlow, #13 NHM, #30 Foundation, #47 Hotel Booking
**Confidence:** **HIGH — 6+ explicit samples.**
**Recipe (Vercel-style):**
- Two lines (`w-8 h-[1.5px]` each) inside a button
- Closed state: both horizontal
- Hover (optional): lines shrink/grow asymmetrically (first → `w-6`, second → `w-10`)
- Open state: both rotate (`+45deg` / `-45deg`) with `translateY` to overlap into an X
- Duration: `300ms` with `ease-out` or `ease-[cubic-bezier(0.22,1,0.36,1)]`

**Recipe (lucide-style):**
- Use lucide-react `Menu` and `X` icons, swap via `useState` toggle
- Often paired with `Menu ↔ X` with rotate-180 and opacity-scale-50 transitions for craft

**When it works:** Universal mobile-trigger pattern. NHM (#13) uses an inventive variant — two-line hamburger with asymmetric hover shrink/grow.
**When it fails:** When the hamburger is small (<36×36) — fails WCAG 2.5.5 target-size guidance.

---

### Pattern: Mobile slide-in-from-right drawer
**Used by:** #11 VaultShield, #12 LinkFlow, #28 Aurora (partial), #30 Foundation
**Confidence:** **HIGH — 4 samples.**
**Recipe:**
- Drawer: `fixed inset-y-0 right-0 z-50 w-[min(88vw, 360px)] h-[100dvh]` (note: `100dvh` not `100vh` for iOS Safari URL-bar correctness)
- Background: `backdrop-blur 4-8px` + tint + dramatic shadow `-12px 0 48px rgba(25,40,55,0.18)` (negative X-offset throws shadow leftward as drawer enters from right)
- Transition: `translate-x-full → translate-x-0`, duration `450–500ms`, ease `cubic-bezier(0.22, 1, 0.36, 1)` (the "decisive arrival" curve from the foundations) OR `cubic-bezier(0.32, 0.72, 0, 1)` (Emil Kowalski's drawer curve)
- Backdrop: separate `fixed inset-0 z-40 bg-black/30 backdrop-blur-sm` overlay, fades opacity 0→1 in 300ms
- **Per-item stagger inside the drawer:** each nav link's `transition-delay` is computed inline `style={{ transitionDelay: open ? \`${150 + i * 70}ms\` : '0ms' }}` so links cascade in 70ms apart, starting 150ms after the drawer opens. CTA group fires at 400ms.
- **Pure-CSS option (LinkFlow #12):** boolean `open` toggles classes, no AnimatePresence needed — same stagger achieved with inline `transitionDelay`

**When it works:** Premium mobile experience for hospitality, fintech, premium SaaS. The negative-shadow + backdrop-blur trio reads as physical material.
**When it fails:** When the drawer is full-width on mobile — feels heavy. `min(88vw, 360px)` is the right pocket. When the staggered delay total exceeds 700ms — feels sluggish.

---

### Pattern: Mobile full-screen overlay
**Used by:** #04 Aethera (implied), #18 Bold Vision, #30 Foundation, #47 Hotel Booking (full-screen black overlay)
**Confidence:** **MEDIUM — 3-4 samples.**
**Recipe:**
- Overlay: `fixed inset-0 z-40 bg-white/95 backdrop-blur-md` (light theme) or `bg-black` (dark theme)
- Contents: large nav links (`text-3xl` or `text-[28-32px]` — bigger than desktop counterparts)
- Optional: close button (X icon) in top-right at `top-6 right-6`
- Optional: secondary CTA cluster (Sign Up + Login) at the bottom of the overlay

**When it works:** When you need maximum-canvas mobile nav with rich content (sub-categories, search, language switcher). Hotel Booking (#47) commits to a full-screen *black* overlay even though the desktop is purple-themed — gives the mobile menu its own space, decoupled from the hero's identity.
**When it fails:** When the overlay is opaque white over a colorful site — feels like a takeover. When you don't include a clear close affordance — users get stuck.

---

### Pattern: Mobile dropdown-from-top
**Used by:** #19 Cinematic Streaming, #28 Aurora (form pane scrolls), #48 Convix
**Confidence:** **LOW-MEDIUM — 3 samples.**
**Recipe:**
- Dropdown: `absolute top-full left-2 right-2 mt-2 bg-white rounded-2xl shadow-lg` (Convix)
- Transition: `translate-y-0 opacity-100` (open) ↔ `-translate-y-4 opacity-0 pointer-events-none` (closed)
- Duration: 300–500ms `ease-out`
- Drops *from* the navbar, anchored to it rather than to the viewport edge

**When it works:** When the mobile menu is short (3–5 items) and the nav is meant to feel "attached" to the page. Anchored-to-nav dropdowns feel less aggressive than full-screen overlays.
**When it fails:** When the dropdown content is long (6+ items) — needs scroll inside the dropdown, awkward.

---

### Pattern: No mobile menu at all (just hides)
**Used by:** #31 SENTINEL AI Spline
**Confidence:** **LOW — 1 sample, but an explicit choice.**
**Recipe:**
- Desktop nav exists at `hidden lg:flex` or `md:flex`
- Below the breakpoint, the nav simply disappears — no hamburger, no replacement
- Logo and primary CTA still visible (the "Get Started" pill or similar)

**When it works:** Strict single-page hero where the nav is decorative (the visitor never needs to navigate anywhere because there's nowhere to go). SENTINEL is single-section.
**When it fails:** Almost always. Foundations: "Mobile nav should be reachable with one thumb." Hiding the nav entirely is the worst answer.

---

### Pattern: Nav-embedded CTA inside the pill
**Used by:** #01 Cinematic, #12 LinkFlow, #14 MicroVisuals, #43 dot.
**Confidence:** **MEDIUM-HIGH — 4 samples.**
**Recipe:**
- The pill nav has asymmetric padding: `pl-6 pr-1` (LinkFlow #12) or `pl-6 pr-1 py-1` (general)
- A separately-styled CTA button sits at the right edge of the pill, tucked in against the right padding
- The CTA button has its own padding (e.g., `py-3 px-5`) so the pill's `pr-1` just keeps it visually tight
- The CTA is part of the container's visual identity, not a separate floating button

**When it works:** When you want the "nav-as-container with welded CTA" feel — Linear, Vercel ship pages, Family.co all do this. The CTA reads as more committed than a separate-floating-button version.
**When it fails:** When the CTA is a different shape than the pill (e.g., square button inside rounded pill) — visual collision.

---

### Pattern: Nav reveals more on hover (logo gradient, brand recognition)
**Used by:** #30 Foundation (per-logo brand gradients on marquee), several with Magnet-style hover (#07 Jack)
**Confidence:** **LOW — 2-3 samples.**
**Recipe (Foundation):**
- Logo marquee items each get *their own* brand-tinted gradient that fades in on hover
- `group-hover:brightness-0 group-hover:invert` on the SVG to switch from gray to colored
- Procure → blue gradient, Shopify → yellow, Blender → blue, Figma → purple, Spotify → pink/red, etc.
- The hover *recognizes the logo it lives behind*

**When it works:** Brand strips, partner showcases, social-proof rows where each logo has known brand equity. Per-brand hover feels considered, not generic.
**When it fails:** When the logos are unfamiliar — the hover gradient reads as random color noise.

---

## Anti-patterns observed

- **No mobile nav at all (SENTINEL #31)** — explicit choice, but always wrong for non-anti-hero pages.
- **`whitespace-nowrap` pill that overflows on narrow viewports** — most floating-pill navs in this batch fail at 360px-wide screens because their content (logo + 5 nav links + 2 right-cluster items) is longer than the viewport. Drop to single icon or collapse to hamburger.
- **Hamburger smaller than 36×36** — fails WCAG 2.5.5 minimum target size. Common gap.
- **`:focus-visible` rings unspecified** — almost universal gap across the batch. Keyboard users get default browser focus rings, which look bad on glass-on-video designs.
- **Mobile menu open/close with no animation** — SkyElite (#10) is called out for this; instant show/hide breaks the otherwise-considered motion vocabulary. 200ms opacity + 8px translateY would be standard.
- **Nav links to features/pricing/about that don't anchor to in-page sections** — Asme v3 (#16) has nav with these labels but no in-page anchoring even though those sections exist.
- **Two near-identical glass surfaces with different blur** — `liquid-glass` (blur 4px) and `glass-pill` (blur 16px) coexist in Asme v1 with no documentation; an undisciplined design system smell.
- **`100vh` instead of `100dvh` on mobile drawers** — iOS Safari URL-bar collapse causes the drawer to "jump" 60px when the URL bar hides. Use `100dvh` (dynamic viewport).
- **No reduced-motion fallback for drawer animations** — the staggered 70ms-per-item entrance is delightful on first touch but tortuous if you toggle the menu repeatedly. Should respect `prefers-reduced-motion`.

---

## Decision rules

- **If the brand is dark + video,** use the floating liquid-glass pill at top with embedded CTA. That's the batch default.
- **If the brand is light + flat,** use a solid white pill nav (Foundation, Convix) OR a glass pill tuned for white (Taskly's `backdrop-blur-[50px] + rgba(255,255,255,0.3) + 1px black/10 stroke + inner highlight`).
- **If the page is long (3+ sections),** use a sticky/fixed solid nav so it stays visible. Anchor-link the nav items to in-page sections.
- **If the page is a single hero with no scroll,** the nav can be decorative (links don't have to go anywhere). Foundation can be bottom-floating; Aethera can be sparse.
- **If the brand voice is editorial / studio,** use the hanging-from-top dark pill over an inset rounded hero card (Prisma).
- **For mobile:** ship a hamburger that morphs into X with a 300ms rotate. Below 768px, choose one of: slide-in-from-right drawer (premium), full-screen overlay (high-touch), dropdown-from-top (lightweight). Don't ship nothing.
- **For the right-cluster:** asymmetric weight wins — text link + filled pill is the canonical pair. If both are the same shape, make Login the secondary and Sign Up the primary (or invert deliberately if your audience is mostly returning users).
- **For focus states:** always ship `:focus-visible` rings, even when the brief omits them. A `ring-2 ring-white/30` on dark or `ring-2 ring-blue-500/50` on light gets you to WCAG compliance with minimal visual cost.

---

## Open questions / gaps in our sample

- **No command-palette (cmd+k) navigation in any batch design.** Surprising given how many AI-adjacent products are in the batch. Linear, Vercel, Cursor would all use cmd+k as a "power-user shortcut" — these designs don't.
- **Mega-menu nav (Stripe/Apple-style) doesn't appear.** No design has multi-column dropdown panels with sub-product navigation. Suggests batch is mostly single-product, not portfolio-of-products.
- **Sidebar nav (docs-style) doesn't appear.** Foundations cites Stripe docs as the gold standard; no batch design is a docs site.
- **Breadcrumbs don't appear.** Marketing pages don't need them; the absence is correct but worth flagging.
- **Search affordance in nav is rare.** Only Cinematic Streaming (#19) ships an explicit Search button in the nav. Most batch designs omit it entirely.
- **Language/region switchers are absent.** None of the batch is internationalized. For real ship, every footer in the batch would need one.
- **Cart icon appears once (Convix #48, oddly — for a PR-agency SaaS).** No e-commerce designs in the batch.
- **Auth state in nav** — almost every nav has Sign Up + Login, but no design shows the logged-in variant (user avatar + dropdown). The batch is exclusively logged-out marketing.
- **Skip-to-content links** — never spec'd. Universal accessibility gap.

---
*Synthesizer: layout-family | 4 of 4 playbooks. Reads from: 55 design notes + foundations/layout-and-composition.md*
