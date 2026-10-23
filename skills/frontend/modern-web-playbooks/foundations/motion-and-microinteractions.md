# Motion & Microinteractions — Foundations

> **Source note:** Written from training-data knowledge (~Apr 2026). Stable concepts.

## Why motion matters

Motion communicates **causality** (this caused that), **hierarchy** (this is more important), and **state** (this changed). When motion is purposeful, it makes a site feel intelligent. When it's decorative, it makes the site feel cheap or desperate.

Rule: **if you removed this motion, would the user lose information?** If yes, keep it. If no, cut it.

## Principles

1. **Motion has a reason.** State change, focus shift, narrative beat — pick one.
2. **Duration: 150–400ms for UI, longer for narrative.** Faster than 100ms feels jarring; slower than 500ms feels sluggish (unless it's narrative).
3. **Easing is never linear** for UI (except progress bars). Out-cubic for entries, in-out-cubic for transitions, in-cubic for exits.
4. **Spring physics** for things that should feel "alive" — drags, dismissals, drawers.
5. **Always respect `prefers-reduced-motion`.** Not optional. The fallback should still communicate state, just without movement.
6. **Transform and opacity only.** GPU-composited. Avoid animating width/height/top/left.

## The motion vocabulary (catalog)

### Hover
- Duration: 120–200ms
- Properties: subtle scale (1.0 → 1.02), color shift, shadow lift, or background tint
- Easing: `cubic-bezier(0.16, 1, 0.3, 1)` (out-expo-ish, "decisive")
- Anti: scale to 1.1+ (clownish), or no hover at all

### Tap / press
- Duration: 80–120ms
- Properties: scale down (1.0 → 0.97), brightness shift
- Easing: faster easing (`ease-out` is fine)
- Counter-motion on release should be slightly slower to feel "natural"

### Focus
- Duration: 100–150ms
- Properties: outline / ring fade-in
- Use `:focus-visible` to avoid showing rings on mouse click
- Ring offset 2–4px, ring width 2px, ring color contrast 3:1 minimum

### Tooltip / popover
- Duration: 150–250ms
- Properties: opacity 0→1, optional translate-Y (4–8px from origin)
- Delay before show: 200–500ms (avoid trigger-happy tooltips)

### Drawer / sheet / modal
- Duration: 250–400ms open, 200–300ms close (close slightly faster)
- Easing: `cubic-bezier(0.32, 0.72, 0, 1)` (Emil Kowalski's drawer curve)
- Properties: transform Y (or X) + opacity + backdrop fade
- Backdrop click closes, escape closes, focus traps inside while open

### Toast
- Enter: 200–300ms slide-in from edge, slight overshoot
- Exit: 150–250ms slide-out + fade
- Auto-dismiss after 4–6 seconds (longer if it has action)

### Tab change
- 200–300ms
- Crossfade is acceptable but slide + fade feels more deliberate
- Indicator (underline / pill) animates between positions — FLIP technique

### Accordion expand/collapse
- 200–300ms
- Height auto requires JS (or `interpolate-size: allow-keywords` in 2026)
- For CSS-only: animate `grid-template-rows: 0fr → 1fr` (modern trick)

### List add/remove (FLIP technique)
- First, Last, Invert, Play
- Measure positions before/after, invert with transform, animate to identity
- Motion's `<AnimatePresence>` and `<Reorder>` handle this
- Critical for tables, sortable lists, dynamic grids

### Drag and drop
- Use spring physics (Motion's `useDragControls` + spring)
- Stiffness 300, damping 30 for "snappy" feel
- Visual cues: lifted shadow + scale 1.05 while dragging
- Drop zones light up; failed drops bounce back

### Loading states
- **Skeleton** for known-shape content (cards, lists) — feels fastest
- **Spinner** for short async (< 2s)
- **Progress bar** for measurable async
- **Shimmer** on skeleton — 1.5s loop, subtle gradient sweep
- Switch from skeleton to content with quick crossfade (not pop-in)

### Success / error feedback
- Inline next to the affected element (not a global toast for form errors)
- Color shift + icon + brief shake for errors (300ms shake, 8px amplitude, decreasing)
- Color shift + checkmark draw for success (400–600ms checkmark stroke animation)

## Page transitions

### View Transitions API (2026 baseline)
- Browser-native, cross-document AND same-document
- `document.startViewTransition(() => updateDOM())`
- Cross-document support is shipping across modern browsers in 2025+
- Pair with `view-transition-name: <name>` on persistent elements for shared-element transitions

### Astro view transitions
- `<ClientRouter />` enables same-document SPA-like nav
- Built on top of View Transitions API

### Next.js App Router
- Built-in loading.tsx and parallel routes give native-feeling transitions
- For richer effects: layout-level Motion components, or framer-motion's AnimatePresence at route level

### When NOT to use page transitions
- Documentation sites where users need to scan fast
- Sites where transitions delay critical content
- When you don't have time to test all edge cases (broken transitions are worse than none)

## Scroll-driven animations

### CSS scroll-driven (modern)
```css
@keyframes fade-up {
  from { opacity: 0; transform: translateY(40px); }
  to { opacity: 1; transform: translateY(0); }
}
.fade-in-on-scroll {
  animation: fade-up linear both;
  animation-timeline: view();
  animation-range: entry 0% cover 30%;
}
```
- No JS needed for many scroll effects
- Browser support: Chrome/Edge solid, Safari catching up (use feature query)

### JS-driven (when you need precision)
- IntersectionObserver for "enter viewport" triggers
- Motion's `useScroll` and `useTransform` for scroll-tied transforms
- GSAP ScrollTrigger for sophisticated timelines (pinning, scrubbing, complex)

### Patterns that work
- **Reveal on enter** — fade + slight translate-Y, 30–60px, once only
- **Sticky pin + reveal** — element pins while content scrolls behind/over
- **Scrub through animation** — scroll position drives animation progress
- **Parallax (subtle)** — background moves at 0.5x scroll speed; foreground at 1x

### Anti-patterns
- Parallax on every section (motion sickness inducement)
- Animations that block reading (no skip-to-content)
- Long pinned sections that feel like the page broke
- Heavy scroll JS that drops frames

## Easing in 2026

### Cubic Bezier classics
- `ease-out`: `cubic-bezier(0, 0, 0.2, 1)` — Material standard
- `ease-in-out`: `cubic-bezier(0.4, 0, 0.2, 1)`
- "Decisive arrival": `cubic-bezier(0.16, 1, 0.3, 1)` (out-expo-ish)
- "Quick departure": `cubic-bezier(0.7, 0, 0.84, 0)` (in-expo)

### The new linear() function
- Browser-native multi-stop easing
- Approximates springs without JS: `linear(0, 0.5 35%, 1.05 70%, 1)`
- Allows complex curves that cubic-bezier can't express
- Browser support: solid by 2025+

### Spring physics
- More natural for direct-manipulation UIs (drags, swipes, dismissals)
- Motion's defaults are sane: `{ type: 'spring', stiffness: 300, damping: 30 }`
- Stiffness up → faster; damping up → less bounce
- Use for drawers, modals, drag interactions; NOT for entrances

## Libraries

### Motion (formerly Framer Motion) — `motion` on npm
- React-first, spring physics, layout animations, gestures
- Heaviest of the popular options but most feature-complete
- Use for: rich React apps with state-driven motion

### Motion One — `motion` (smaller vanilla flavor)
- Vanilla JS, ~3.5kb, Web Animations API under the hood
- Use for: marketing sites that want a small motion lib without React

### GSAP
- Industry-standard for sophisticated scroll/timeline/SVG animation
- Free as of 2024+ (Webflow acquisition opened the license)
- Use for: scroll-trigger choreography, DrawSVG, MorphSVG, timelines

### CSS-only
- Hover, focus, simple transitions, basic scroll-driven
- No JS dependency, no jank
- Use first; reach for JS when CSS can't express what you need

### Lottie
- AE-exported JSON animations
- Use for: branded illustrated animations that loop or play on demand
- Avoid: where SVG + CSS would suffice (Lottie is heavier)

### Rive
- Interactive animations with state machines
- Use for: animated illustrations that respond to user input or app state

## Performance

- **Transform and opacity only** for animated properties (GPU-composited)
- **will-change sparingly** — only on elements about to animate; remove after
- **60fps target** = 16.6ms per frame budget for everything (paint, layout, composite, script)
- **Throttle scroll handlers** with rAF or passive listeners
- **Avoid layout thrashing** — read then write, never alternate

## Anti-patterns

- Motion that delays interactive elements (a button that animates for 600ms before clickable)
- Motion without `prefers-reduced-motion` fallback
- Bouncing CTAs to "draw attention" — desperate
- Parallax-everywhere syndrome
- Every section animates in (visitor fatigue by section 3)
- Heavy motion libraries pulled in for one fade animation
- Custom easings that ignore the design system (every motion should feel like the same character)

## Reference sites by motion category

### Motion-as-language
- **Linear** — every state transition has a deliberate motion
- **Vercel** — ship pages with kinetic type and gradient choreography
- **Cursor** — terminal-feel motion vocabulary

### Narrative scroll
- **Stripe payments** — long-scroll product narrative
- **Apple product pages** — section-by-section reveal choreography
- **Vercel ship pages** — scroll-driven feature reveals

### Micro-polish exemplars
- **Family.co** — subtle hovers, deliberate transitions
- **Raycast** — every microinteraction feels considered
- **Arc browser** — playful but purposeful

## Sources

- emilkowal.ski (Emil Kowalski's writings)
- motion.dev (formerly framer.com/motion)
- web.dev/learn/animations
- joshwcomeau.com (Josh Comeau)
- developer.mozilla.org/en-US/docs/Web/API/View_Transitions_API

---
*Last updated: 2026-05-28*
