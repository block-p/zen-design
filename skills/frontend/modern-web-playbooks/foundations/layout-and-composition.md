# Layout, Composition, IA & Navigation — Foundations

> **Source note:** Written from training-data knowledge (~Apr 2026). Stable concepts.

## Why layout is the unsung hero

Layout is what separates "I designed a page" from "I designed a system." A great layout gives every piece of content its right size and right neighbor, makes scanning effortless, and breathes. Layout failures are why most "modern" sites still feel cluttered.

## Grid systems in 2026

### 12-column
- Default for marketing. Flexible enough for most asymmetric needs.
- Gutter: 16–32px depending on container width.
- Max content width: typically 1280–1440px (wider feels editorial; tighter feels SaaS).

### 6-column / 4-column
- Used on smaller breakpoints, or for dense bento layouts.
- Container queries make these viable at component level.

### Asymmetric grids
- 8/4 split, 7/5, 9/3 — used to break monotony in long pages.
- Most premium sites alternate symmetric and asymmetric sections to create rhythm.

### CSS Grid + subgrid
- Native CSS Grid handles almost everything.
- Subgrid (now broadly supported in 2025+) lets children align to the parent's grid — solves nested grid alignment elegantly.
- Use `grid-template-columns: repeat(12, 1fr)` with `gap: var(--gutter)`.

### Container queries
- `@container (min-width: 600px)` — component-driven responsive, not viewport-driven
- A card knows its own width; styles itself accordingly
- Browser support: ~95% in 2026, safe to use
- Use for: cards, sidebars, anything that can appear in multiple contexts

## The Bento layout

### Origins
Apple's iPad Pro launch ads (~2022) popularized the modern bento. Earlier roots in dashboard design and magazine layouts.

### Why it works
- Communicates many things in parallel — the eye scans tiles, doesn't read top-down
- Each tile contains one idea, so density doesn't equal noise
- Visually rhythmic — varied tile sizes prevent monotony

### Variations
- **Rigid bento** — uniform tiles in a clean grid (Apple, Cursor features)
- **Modular bento** — varied tile sizes, magazine-like (Raycast extensions, Linear features)
- **Magazine-style** — one big tile + supporting smaller ones (editorial)

### Anti-patterns
- A bento that's just a feature list with rounded corners — no information density per tile
- Tiles of equal size with no rhythm — boring
- Tiles too small to contain meaningful content — gimmicky
- Animation in every tile — exhausting

## Asymmetric and editorial layouts

- Editorial sites borrow magazine grid intuition — pull-quotes, drop caps, asymmetric image placement
- Often combine narrow text columns (60–70ch) with full-bleed media moments
- Vertical rhythm matters more here than in dense product pages

## Composition principles

### Hierarchy through scale + spacing + color + weight
- Don't rely on size alone. A 14px gray-12 next to a 14px gray-9 is hierarchy.
- White space is the cheapest, strongest hierarchy tool.

### The 8pt grid (or 4pt)
- All spacing values are multiples of 8 (or 4 for tight UI)
- 4 / 8 / 12 / 16 / 24 / 32 / 48 / 64 / 96 / 128 / 192 / 256
- Tailwind ships this scale by default
- Stay on the grid 95% of the time; break it only with intention

### Vertical rhythm
- A consistent vertical beat (e.g., body lines on a 24px baseline) makes long pages feel composed
- Modern web has loosened the strict baseline-grid approach, but the *idea* of consistent vertical spacing remains

### Optical vs mathematical alignment
- The eye is fooled by perfect geometric centers — sometimes a button needs +1px on top to look centered
- Triangular shapes (play icons) need optical centering
- Text inside a circle is rarely centered mathematically

### Reading patterns
- Z-pattern: applies to sparse pages (hero with one CTA)
- F-pattern: applies to text-heavy pages (blog posts, docs)
- Modular: applies to bento and grid layouts
- Linear: applies to scrolling narratives

## Hero patterns 2026

### Big-type hero
- Massive headline (96–200px), generous spacing, one CTA below
- Example: Linear, Cursor, Plain, Vercel marketing
- Works when: brand confidence is established, message is short

### Product-screenshot hero
- Headline + subhead + a perfectly-shot product screenshot
- Example: Stripe, Linear, Vercel
- Works when: the product is visually distinct and you want trust fast

### Interactive hero
- The hero itself is the product (drag, type, play)
- Example: Figma, tldraw, Excalidraw, Vercel ship pages
- Works when: the product is interactive AND short to demonstrate
- Risk: performance, motion sickness, distraction from the message

### Video hero
- Background or inline video (always with motion-reduce fallback)
- Example: Apple product pages, many agency sites
- Works when: motion is part of the brand
- Risk: file size, autoplay policies, motion-reduce, loading

### Split hero
- 50/50 or 60/40 — type on one side, visual on the other
- Example: Stripe, classic SaaS
- Solid default; less exciting than alternatives

### Anti-hero
- Just text. Scroll to discover the product.
- Example: Plain, some indie portfolios, Anthropic
- Works when: brand confidence is *very* high; visitors expect this level of restraint

### Above the fold — what to include
- Logo + nav
- Headline (clear, specific, one idea)
- Subhead (1–2 lines, never long)
- Primary CTA (one)
- Optional secondary CTA (lower visual weight)
- Optional: small social proof (logo strip, badge, count)

### What to exclude above the fold
- Three CTAs
- Long paragraphs
- A feature list
- A cookie banner that takes more space than the hero

## Section rhythm

After the hero, sites breathe through varied section types:

1. **Hero** — sets intent
2. **Logos** — social proof (small, gray, not distracting)
3. **Value-prop section** — one core benefit, illustrated
4. **Bento** — many features at a glance
5. **Detailed feature deep-dive** — one feature, large screenshot, narrative
6. **Testimonial / case study** — proof
7. **CTA section** — repeat the action
8. **Footer** — comprehensive nav, trust, contact

The exact mix depends on length. A short marketing page might be hero → bento → CTA → footer. A long page might have 10+ sections.

**Density alternation matters.** Sparse hero → dense bento → sparse quote → dense feature shot → sparse CTA. Each section's job is to make the next one breathe.

## Navigation patterns

### Top nav
- **Sticky-on-scroll** — common, useful when the page is long
- **Transparent → solid on scroll** — gives the hero a clean canvas
- **Mega menu** — for sites with many product areas (Stripe, Apple)

### Bottom-only nav (mobile-first)
- Common in mobile apps; rarer on the web but used by some portfolios

### Command palette (cmd+k)
- Linear, Vercel, Cursor, Notion, Raycast — all reference points
- Powerful for power users; should never *replace* a visible nav

### Sidebar
- Default for docs sites (Stripe docs is gold standard)
- Often paired with right-side TOC for the current page

### Mobile nav
- Hamburger menu, full-screen overlay
- Often the same nav links as desktop, plus an explicit CTA
- Should be reachable with one thumb

## Information architecture

### Card sorting and tree testing
- Before building, sort content with users — what goes together, what should be siblings vs cousins
- Tree testing validates that the IA you chose is findable

### URL structure as IA
- `/product/<feature>/` mirrors nav
- Short, lowercase, hyphen-separated
- Avoid query parameters for navigable content

### Breadcrumbs
- Useful in docs and deep marketing sites
- Skip in marketing top-pages (visitors don't need them on the homepage)

## Footers — the forgotten section

### Big footer (Stripe, Vercel)
- Full sitemap
- Newsletter inline
- Social links
- Legal & compliance links
- Sometimes a brand statement or huge logo as visual anchor

### Minimal footer
- Single line: copyright + 3–5 essential links
- Works for portfolios and very-focused marketing pages

### Newsletter in footer
- Inline form, single field, clear value prop
- "Get one email a month" beats "Subscribe to our newsletter"

## Responsive strategy

### Mobile-first
- Design and build for mobile, layer up for larger screens
- Tailwind's `sm:`, `md:`, `lg:` prefixes encode this

### Container queries
- Component-driven responsive — a card knows its own width
- Replaces ~70% of cases where you'd previously use viewport breakpoints

### Breakpoint conventions
- Tailwind defaults: sm 640px / md 768px / lg 1024px / xl 1280px / 2xl 1536px
- Custom breakpoints often beat defaults — design at *your* content's break points

## Anti-patterns

- Three-column "feature/feature/feature" boxes with stock icons
- Centered everything (zero left-aligned text on a page = exhausting to read)
- Inconsistent gutters across sections
- Forcing 12-column grid on layouts that want 5 or 7
- Mobile nav that requires two-handed thumb reach
- Sidebars at desktop that don't collapse to a usable mobile pattern
- Footers as afterthought
- Hero that doesn't fit on common laptop screens (1366×768 still exists in 2026)

## Reference sites with annotation

| Site | Layout signature |
|---|---|
| Stripe | Long-scroll narrative + dense product proof + mega menu |
| Linear | Bento + dark mode + restrained sections, max-width disciplined |
| Vercel | Type-first hero + bento + dense logo wall + comprehensive footer |
| Cursor | Editorial hero + bento + terminal motif sections |
| Apple | Bento + kinetic scroll + video moments + comprehensive footer |
| Notion | Mixed editorial and bento + templates section as gallery |
| Stripe docs | Sidebar + right-rail TOC + inline code playgrounds |
| Family.co | Single-column editorial with breathing room |
| Plain | Minimalist single-column, mono-plus-red discipline |

## Sources

- web.dev/learn/css
- every-layout.dev (Heydon Pickering)
- ishadeed.com (Ahmad Shadeed)
- Smashing Magazine's layout writings

---
*Last updated: 2026-05-28*
