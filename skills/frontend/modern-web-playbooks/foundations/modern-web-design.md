# Modern Web Design — Foundations

> **Source note:** Written from training-data knowledge (cutoff ~Apr 2026). For the latest movements and reference-site URLs, ask the design-analyzer or site-architect to enrich this file with WebSearch after the session resets.

## Why this matters

When agents in this system look at a website, they need a shared mental model of *what modern means right now*. Without it, "make it look modern" collapses to a generic Tailwind landing page. With it, every decision can be traced back to a movement, a principle, or a reference.

The 2025/2026 web is unusually rich. The dominant aesthetic isn't one style — it's a vocabulary of 6–8 movements that craft-tier teams remix. Knowing the vocabulary lets you choose deliberately.

## Current visual movements

### 1. Big-type editorial (a.k.a. Swiss-influenced)
**Traits:** A single 120–200px display headline, generous negative space, restrained palette (often near-mono with one accent), grid-rigorous layout, body type that respects the same humanist family.
**Example sites:** Linear, Vercel marketing, Cursor, Family.co, Stripe Sessions, Plain.com
**Works when:** brand confidence is high, message is short, audience expects sophistication.
**Fails when:** the product needs to *show* itself (e-commerce, dense SaaS) — big type buries the offer.

### 2. Bento layouts
**Traits:** Modular tiles of varying size that show many features at once. Each tile self-contains a small story (icon + short copy + sometimes a mini-animation or screenshot).
**Example sites:** Apple iPad Pro features, Linear features, Raycast extensions, Cursor product page
**Works when:** you have 6–12 features you want to communicate in parallel, not sequentially.
**Fails when:** tiles are just feature lists with rounded corners — bento without information density is decoration.

### 3. Brutalism revival / anti-design
**Traits:** Default browser fonts (or Arial), heavy black bars, naked HTML feel, deliberate "ugliness," sometimes Geocities references.
**Example sites:** Are.na, certain indie devs, some agency portfolios, gallery sites.
**Works when:** the brand wants to signal anti-corporate, anti-AI-slop, raw and direct.
**Fails when:** done by anyone who hasn't earned the right — looks lazy, not brave.

### 4. Glass / depth / surface
**Traits:** Translucent surfaces, backdrop-filter blur, layered cards with subtle shadows, glassmorphism (post-Apple-Liquid-Glass era it has new life).
**Example sites:** Apple (Liquid Glass system), Arc browser, Cron (now Notion Calendar), some Linear views.
**Works when:** there's a strong background (gradient or image) the glass can rest on.
**Fails when:** glass on a flat white background — pointless.

### 5. Generative gradients / mesh
**Traits:** Multi-stop, often OKLCH-tuned, mesh-style gradients in heroes. Often paired with grain/noise overlay for organic feel.
**Example sites:** Stripe (long-time leader), Linear, Vercel, Cursor, Family
**Works when:** the brand wants warmth, energy, or to feel "AI-native."
**Fails when:** muddy middles (RGB linear interpolation through complementary hues), or used as filler with no editorial intent.

### 6. AI-native interfaces
**Traits:** Chat-as-primary-UI, command-bar centerpiece, streaming text effects, generative-looking gradients, sometimes a literal cursor blink as motif.
**Example sites:** Cursor, v0.dev, Claude, ChatGPT, Perplexity, Anthropic homepage
**Works when:** the product is AI (don't fake it).
**Fails when:** non-AI products imitate to look modern — feels parasitic.

### 7. Monochrome-plus-one-accent
**Traits:** All grayscale UI with exactly one saturated brand color for actions and accents.
**Example sites:** Vercel (black/white/blue), Linear (black/white/purple), Plain (black/white/red), Cron-now-Notion Calendar.
**Works when:** the team can hold the line — the moment a second accent creeps in, the spell breaks.
**Fails when:** "almost mono" — half-committed feels indecisive.

### 8. Kinetic typography
**Traits:** Text that animates on scroll, on hover, or on load — variable font weight transitions, character-by-character reveals, text-on-path, masking effects.
**Example sites:** Apple product pages, Vercel ship pages, agency portfolios (Active Theory, Resn, Active.com), Awwwards SOTD regulars.
**Works when:** the kinetic moment *means* something (the product page for a font foundry doing kinetic type — perfect).
**Fails when:** every section animates because the agency wanted to charge for the work.

### 9. Dense product-proof
**Traits:** After a quiet hero, sections cram in screenshots, customer logos, metric callouts, case-study cards. Information density signals product maturity.
**Example sites:** Stripe, Vercel, Notion, Figma, Linear
**Works when:** the product genuinely is mature and you have proof.
**Fails when:** padding with stock-photo testimonials.

### 10. Indie / portfolio craft
**Traits:** Personal sites by design engineers — meticulous spacing, custom animations, small experimental moments, often dark mode by default. Often built in Astro or vanilla.
**Example sites:** rauno.me, emilkowal.ski, paco.me, leerob.com, brittanychiang.com, hov.co, samuelkraft.com
**Works when:** every detail is a flex of craft.
**Fails when:** copying surface details without the underlying restraint.

## Principles that separate craft-tier from generic

1. **Restraint is the signal.** Generic sites add. Craft-tier sites remove. Five well-judged sections beat fifteen.
2. **Hierarchy lives in spacing, not size.** Doubling a heading's size is the cheapest hierarchy tool. Doubling the white space around it does more work.
3. **Motion communicates state, not decoration.** If a motion's removal would lose information, keep it. If not, cut it.
4. **One brand color held religiously.** Two brand colors becomes a "color system" — three becomes a clown costume unless you're a kids' brand.
5. **The grid is invisible, but felt.** 8px or 4px base unit. Everything snaps. Optical adjustments only where the math fights the eye.
6. **Body type is sacred.** 15–18px, 1.5–1.65 line height, near-neutral tracking. Get this wrong, the site reads cheap regardless of everything else.
7. **Dark mode is its own design, not an inversion.** Tints, contrasts, and saturations need re-tuning.
8. **The hero earns the rest.** A great hero buys 5 seconds of scrolling. If the next section is generic, the trust evaporates.
9. **Edges and corners are a fingerprint.** Are corners 4px, 8px, 12px, or pill? Are hairlines 1px gray-5, or 0.5px gray-7? Consistency here separates "designed" from "themed."
10. **Voice is part of design.** Body copy, button labels, error states — the voice is as visible as the type. Inconsistent voice undermines every visual choice.

## What "premium" feels like in 2026

- The first scroll is small. The hero takes its time.
- Typography is the protagonist. Color and motion are supporting.
- Every interactive element has a confident hover and a deliberate focus state.
- Page transitions exist but never get in the way.
- The footer is as designed as the hero.
- Dark mode is tested as carefully as light, often the default.
- Type is variable, fonts are subset, fonts are preloaded — no FOIT/FOUT on hero.
- Images are AVIF, dimensions are reserved, no CLS.
- The site loads with intent — LCP under 1.5s on a real connection.

## Rising patterns (next 12 months)

- **View Transitions API** broadly adopted — cross-document transitions without JS frameworks
- **OKLCH-based palettes** as default (Tailwind v4, Radix, custom systems)
- **Scroll-driven CSS animations** replacing JS scroll libraries where possible
- **AI-generated hero illustrations** — but only where they earn it (the illustration *is* the product story)
- **Spatial / 3D moments** (WebGPU emerging, R3F holding) used sparingly as scene-setters, not throughout
- **Reading mode as design constraint** — sites designed for skim AND deep read
- **AIEO-shaped content blocks** — direct-answer paragraphs near headings, dated facts, comparison tables

## Declining / dated patterns

- Heavy parallax on every section (peaked ~2018, lingering)
- Glassmorphism without depth or context (pre-Liquid-Glass; do better now)
- Generic 3-column "Features / Pricing / Testimonials" SaaS pages
- Big rotating gradient orbs as the only visual idea
- Cookie banners that take more design love than the hero
- "Loud" CTAs that bounce/pulse for attention
- Single-page narratives where every section autoplays a video
- Stock photography that screams stock

## Reference sites — top 15 to study

| Site | What's notable |
|---|---|
| linear.app | Mono-plus-accent done to perfection; motion vocabulary is the textbook |
| vercel.com | Type-first, dense product-proof, ship-page kinetic type, dark mode reference |
| stripe.com | Gradients, narrative scroll on payment pages, dense documentation as design |
| cursor.com / cursor.sh | AI-native hero, sparse to dense rhythm, kinetic terminal motif |
| family.co | Restraint as identity; nearly-mono with deliberate moments of warmth |
| raycast.com | Microinteraction polish; how to design a dense product surface |
| apple.com | Bento, kinetic scroll, View Transitions adoption; reference for everything |
| arc.net | Glass and depth done with intent; warmer-than-Apple Apple |
| rauno.me | Portfolio of a design engineer; every detail is a study |
| emilkowal.ski | Animation reasoning + written craft; the writing is part of the design |
| paco.me | Small, opinionated, fast; how a minimalist personal site can still feel rich |
| framer.com | Marketing for a design tool — performance is the demo |
| anthropic.com | AI-native quiet sophistication; restraint in an industry shouting |
| plain.com | Customer support tool with a maker's tone; mono-plus-accent for SaaS |
| figma.com | Editorial mixed with product-proof; the long page done well |

## Anti-patterns

- "Modern" = whatever's trending → always be 6 months late
- Adding glass + gradient + animation + kinetic type = soup
- Borrowing Linear's aesthetic without Linear's restraint
- Designing for the Dribbble screenshot, not for the scroll
- Inconsistent corner radii — 4px buttons next to 16px cards
- Ignoring dark mode and shipping a half-baked toggle at launch

## Sources (placeholder — refresh after session reset)

- Smashing Magazine (year-in-review issues, 2024–2026)
- web.dev/learn (Google Chrome team)
- Awwwards SOTD archives 2025–2026
- Hover States, Sidebar, SiteInspire
- Designer portfolios cited above

---
*Last updated: 2026-05-28 | Refresh sources needed*
