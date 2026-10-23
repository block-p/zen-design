# Performance & Accessibility — Foundations

> **Source note:** Written from training-data knowledge (~Apr 2026). Spec details (WCAG 2.2, Core Web Vitals thresholds) are stable.

# PART 1 — PERFORMANCE

## Why perf is part of design

Slow sites lose users before they see the design. In 2026, perf is a ranking factor (Core Web Vitals) and a competitive moat. Premium sites are fast. Period.

## Core Web Vitals in 2026

### LCP — Largest Contentful Paint
- **Target:** < 2.5s (good), < 4.0s (needs improvement), > 4.0s (poor)
- **What it measures:** time until the largest above-fold element renders
- **Usually:** hero image, hero text, or first big content block
- **How to improve:**
  - Identify the LCP element (Lighthouse, web-vitals lib)
  - `fetchpriority="high"` on the LCP image
  - `<link rel="preload">` for the LCP image and critical font
  - Inline critical CSS for above-fold
  - Server response time matters — slow TTFB makes LCP impossible

### INP — Interaction to Next Paint (replaced FID in 2024)
- **Target:** < 200ms (good), < 500ms (needs improvement)
- **What it measures:** worst-case input responsiveness across a session
- **How to improve:**
  - Break long JS tasks into smaller chunks (yield to main thread)
  - Use `scheduler.yield()` or `setTimeout(fn, 0)` in loops
  - Avoid synchronous third-party scripts on user input
  - Optimize React renders (memo, virtualize lists, defer non-critical state)

### CLS — Cumulative Layout Shift
- **Target:** < 0.1 (good), < 0.25 (needs improvement)
- **What it measures:** how much the page moves around during load
- **How to improve:**
  - **Always set width and height on images and embeds** (or use CSS aspect-ratio)
  - Reserve space for ads, embeds, late-loaded content
  - Use `font-display: optional` if FOUT is unacceptable (otherwise `swap`)
  - Don't insert content above existing content after load

## Image strategy

### Formats
- **AVIF first** — 30–50% smaller than WebP at equivalent quality
- **WebP fallback** — when AVIF is unavailable or unsupported
- **JPEG/PNG fallback** — for ancient browsers (rarely needed in 2026)

### Responsive
```html
<picture>
  <source type="image/avif" srcset="hero.avif 1x, hero@2x.avif 2x">
  <source type="image/webp" srcset="hero.webp 1x, hero@2x.webp 2x">
  <img src="hero.jpg" alt="..." width="1200" height="800" loading="lazy" fetchpriority="auto">
</picture>
```

### Loading hints
- `loading="lazy"` on below-fold images
- `loading="eager"` (default) on above-fold
- `fetchpriority="high"` on the LCP image
- `decoding="async"` is default in modern browsers

### Dimensions ALWAYS set
- `width` and `height` attributes on `<img>` even if styled with CSS — prevents CLS
- For responsive images, the ratio matters more than the absolute size

## Font strategy

- **WOFF2 only** (modern browsers, smallest format)
- **Subset for Latin** (or your specific charset) — saves 70%+
- **Preload critical fonts**:
  ```html
  <link rel="preload" href="/fonts/Geist-Variable.woff2" as="font" type="font/woff2" crossorigin>
  ```
- **`font-display: swap`** for fast paint (acceptable FOUT) OR `optional` for stricter perf
- **Variable font as single file** — every weight, one HTTP request
- **Self-host** — avoid Google Fonts CSS extra roundtrip

## JavaScript budget

- **Aim < 100KB gzipped initial JS** for marketing pages
- **Code split per route** (Next.js / Astro do this automatically)
- **Dynamic import non-critical**:
  ```js
  const Modal = dynamic(() => import('./Modal'))
  ```
- **Server components / islands** where possible — ship less JS
  - Next.js App Router server components
  - Astro islands
  - Qwik resumability
- **Audit dependencies** — heavy libs (Moment, Lodash full, etc.) often have lighter alternatives

## CSS strategy

- **Critical CSS inlined** in `<head>` for above-fold
- **Tailwind v4** uses Lightning CSS, JIT-by-default, no purge step needed
- **Container queries** reduce JS-based responsive
- **Avoid heavy CSS-in-JS** at runtime — prefer zero-runtime (Vanilla Extract, Linaria, CSS Modules, or just CSS)

## Caching, CDN, edge

- **HTTP cache headers**: `Cache-Control: public, max-age=31536000, immutable` for static assets with hashed filenames
- **Edge rendering** for personalized-but-fast (Vercel Edge, Cloudflare Workers, Netlify Edge)
- **SSG (static)** is fastest — use when content doesn't need per-request data
- **SSR (server-rendered)** for personalized content
- **CSR (client-rendered)** only when SEO/perf isn't critical (admin dashboards)
- **Service workers** for offline-first PWAs; selective use otherwise

## Measurement

### Tools
- **Lighthouse** — local + CI
- **PageSpeed Insights** — uses CrUX field data + Lighthouse lab
- **WebPageTest** — deep waterfall analysis, multi-location
- **Chrome DevTools Performance** — frame-by-frame, throttling
- **Real User Monitoring (RUM)** — Vercel Speed Insights, SpeedCurve, Calibre

### CrUX (Chrome User Experience Report)
- 28-day field data from real Chrome users
- The data Google uses for Core Web Vitals ranking
- Available in PSI, GSC, and BigQuery

---

# PART 2 — ACCESSIBILITY

## Why a11y is part of design

A site that's inaccessible isn't "broken for a minority" — it's broken for ~15–25% of users at any moment (visual, motor, cognitive, situational impairments). Plus: legal requirement in many jurisdictions (ADA, EAA in EU, etc.). Plus: better a11y = better SEO, better usability for everyone.

## WCAG 2.2 — what's new since 2.1

### New Level AA criteria
- **2.4.11 Focus Not Obscured** — focused element must not be hidden by sticky headers/footers
- **2.5.7 Dragging Movements** — drag operations must have a non-drag alternative
- **2.5.8 Target Size (Minimum)** — interactive targets must be 24×24 CSS pixels minimum (with exceptions)
- **3.2.6 Consistent Help** — help mechanisms (chat, contact) consistent across pages
- **3.3.7 Redundant Entry** — don't ask for the same info twice in a process
- **3.3.8 Accessible Authentication** — no cognitive function tests (puzzles) for auth

WCAG 3 is still in draft as of 2026 — don't optimize for it yet.

## Color contrast

### WCAG 2.2 minimums
- **4.5:1** for body text (small)
- **3:1** for large text (18pt regular / 14pt bold)
- **3:1** for UI components and graphical elements (icons, borders carrying meaning)

### APCA (modern perceptual)
- Better correlates with perceived contrast
- Use Lc 60+ for body, Lc 75+ for fluent reading
- Recommended for modern systems; will be incorporated in WCAG 3
- Tool: apcacontrast.com

## Keyboard navigation

- **Tab order = visual order** (DOM order matters)
- **`:focus-visible`** for keyboard-only focus rings (don't show on mouse click)
- **Skip-to-content link** at the top of every page
- **No keyboard traps** — every interactive thing has a way in AND out via keyboard
- **Custom components** (dropdowns, modals, tabs) must be fully keyboard-operable — use ARIA patterns or proven libraries (Radix UI, React Aria, Reach)

## Screen reader basics

### Rule 1: Semantic HTML first
- `<button>` for buttons, `<a>` for links, `<input>` for inputs
- Don't `<div onclick>` — invisible to screen readers and keyboards

### ARIA: the first rule
> "If you can use a native HTML element with the semantics and behavior you need, use it instead of repurposing an element and adding an ARIA role."

### Alt text
- **Descriptive, not redundant.** "Photo of a cat" is bad; "Tabby cat napping on a windowsill" is good.
- **Empty `alt=""`** for decorative images (NOT missing alt — that's worse)
- **Functional images** describe the function, not the picture ("Search" not "magnifying glass")

### Headings
- One `<h1>` per page
- Hierarchy intact (don't skip h2 → h4)
- Used for structure, not styling

### Landmarks
- `<main>`, `<nav>`, `<header>`, `<footer>`, `<aside>` — screen readers use these for quick navigation
- Multiple navs need `aria-label` to distinguish ("primary navigation" vs "footer navigation")

## Forms

- **Visible labels associated with inputs** (`<label for>` or wrapping)
- **Error messages programmatically associated** via `aria-describedby`
- **Required** marked in label text AND `aria-required="true"`
- **Input types correct**: `email`, `tel`, `number`, `url`, `search`, `date`
- **Autocomplete attributes**: `autocomplete="email"`, `name`, etc. — better UX + a11y
- **Inline validation** with clear messaging, not just red borders

## Motion accessibility

- **`prefers-reduced-motion: reduce`** respected for all decorative motion
- Fallback should still communicate state (a crossfade instead of slide is fine)
- **No flashes > 3 per second** (seizure risk — WCAG 2.3)
- **Parallax can cause vestibular discomfort** — disable on reduced-motion

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    transition-duration: 0.01ms !important;
  }
}
```

## Touch and pointer

- **Target size ≥ 24×24** (WCAG 2.2 minimum), 44×44 still recommended (Apple HIG)
- **Don't rely on hover alone** — touch users have no hover
- **Tap delay**: modern browsers have removed 300ms delay; ensure `<meta viewport>` is set
- **Honor `prefers-color-scheme`** — don't override system preference without consent

## Testing

### Automated (catches ~30% of issues)
- **axe DevTools** browser extension
- **Lighthouse a11y audit**
- **WAVE** (WebAIM)
- **eslint-plugin-jsx-a11y** in CI

### Manual (catches the remaining 70%)
- **Keyboard-only run-through** — tab through every interactive element
- **Screen reader test** — NVDA (Windows), VoiceOver (macOS/iOS), TalkBack (Android)
- **Zoom test** — 200% browser zoom, 400% browser zoom
- **High contrast mode** test (Windows High Contrast)

## Anti-patterns

- ARIA without understanding (broken ARIA is worse than no ARIA)
- Removing focus outlines without replacing them
- Placeholder as label (disappears on input)
- Color-only state indication (color blind users miss it)
- `tabindex` > 0 (breaks natural tab order)
- Inaccessible custom components (custom dropdown built from divs)
- Hover-only menus on touch devices
- Tiny touch targets (12px icons)
- Carousels without pause control
- Auto-playing video with sound

## Sources

- web.dev/vitals
- web.dev/learn/accessibility
- W3C WCAG 2.2 spec
- WebAIM, Adrian Roselli, Sara Soueidan, Léonie Watson, Scott O'Hara

---
*Last updated: 2026-05-28*
