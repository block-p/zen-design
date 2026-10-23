# Color Systems — Foundations

> **Source note:** Written from training-data knowledge (~Apr 2026). Stable concepts; refresh palette references after session reset.

## Why most palettes fail

- They start from inspiration, not from a system
- They use one of three default blues
- They invert for dark mode instead of redesigning it
- They have no semantic layer (the same hue is used for "brand", "info", and "primary action")
- They mix color spaces inconsistently (sRGB for some, eyeballed OKLCH for others)
- They ignore that perceptual lightness ≠ HSL lightness

## Color spaces in 2026

### sRGB (legacy)
- The default for the web for two decades
- Hex notation is sRGB
- Limited gamut, perceptually non-uniform

### Display-P3
- ~25% wider gamut than sRGB
- Supported by all modern browsers via `color(display-p3 r g b)`
- Most consumer screens since ~2020 can display it
- Use for: vivid brand accents, gradients that pop

### OKLCH / OKLab
- Perceptually uniform — equal L value = equal perceived brightness across hues
- Much better for: ramps, gradients (no muddy middles), hover/active states
- Browser support: ~95% global (Chrome, Safari, Firefox all good since 2023)
- Syntax: `oklch(0.6 0.15 240)` — L (0-1), C (chroma, 0–0.4ish), H (hue 0–360)
- **The modern default.** Tailwind v4 uses OKLCH internally.

### Fallback strategy
```css
.brand { color: #2563eb; /* sRGB fallback */ }
@supports (color: oklch(0% 0 0)) {
  .brand { color: oklch(58.5% 0.18 263); }
}
```
In practice, OKLCH support is wide enough that you only need fallback for ancient browsers — most sites just use OKLCH directly.

## The 12-step scale model (Radix-style)

Radix Colors popularized a 12-step semantic ramp per hue. Adopt it.

| Step | Use |
|---|---|
| 1 | App background |
| 2 | Subtle background (cards on bg) |
| 3 | UI element background (hover for transparent) |
| 4 | UI element background (active for transparent) |
| 5 | Subtle borders, separators |
| 6 | UI borders |
| 7 | Hovered UI borders |
| 8 | Solid backgrounds (low emphasis) |
| 9 | Solid backgrounds (brand) — the "main" brand color |
| 10 | Hovered solid backgrounds |
| 11 | Low-contrast text |
| 12 | High-contrast text |

**Light and dark versions are paired by step**, not by hex. `gray-9` light = `gray-9` dark — both are "solid brand color" in their context.

## Palette construction

A good palette has:
1. **One neutral ramp** (gray, slate, stone, mauve — pick one)
2. **One brand hue ramp** (full 12 steps)
3. **One accent hue ramp** (optional — but if added, used sparingly)
4. **Semantic ramps** for state: success (green), warn (amber), danger (red), info (blue)

### Restraint principle
The most premium sites use one brand color and never deviate. Linear is purple. Vercel is black/white with blue accent. Stripe is purple. Plain is red. Notion is black-and-white with no real brand color.

### Black-and-white-plus-one-accent
The most underused pattern. A grayscale UI with one saturated hue for actions only. Forces every choice to earn its color.

Examples: Vercel, Linear (purple as the single accent), Cursor (terminal green/cyan on near-black), Plain (red).

## Dark mode

### Don't invert
Light mode and dark mode have **different perceptual demands**:
- Dark mode: reduce chroma 5–15% (saturated colors glow uncomfortably on dark backgrounds)
- Dark mode: reduce contrast slightly (pure white on pure black is harsh; use gray-12 on gray-1)
- Dark mode: lighter surfaces step up by a smaller amount

### True black vs near-black
- **#000 (true black)** — OLED-friendly, dramatic, but elevations need shadows or borders to register
- **#0a0a0a / #111** — slightly elevated, allows tints for depth
- **#0d0d10 (cool near-black)** — most premium sites
- **#0a0a0e (slight blue tint)** — Apple Dark
- Use one base near-black for `--bg`, then step up: `--bg-subtle: #16161a`, `--card: #1c1c20`, etc.

### Surface elevation
- In light mode, elevation = shadow
- In dark mode, elevation = lighter tint (shadows don't work on dark bg)
- This is why dark mode needs more bg steps than light mode

## Contrast

### WCAG 2.2 minimums
- 4.5:1 for body text (small text)
- 3:1 for large text (18pt+ regular, 14pt+ bold)
- 3:1 for UI components and graphical elements (icons, borders that carry meaning)

### APCA (Accessible Perceptual Contrast Algorithm)
- Modern perceptual contrast measurement (vs WCAG's relative luminance)
- Use Lc 60+ for body, Lc 75+ for fluent reading
- Recommended for modern systems; WCAG 3 will incorporate APCA-like measure when ratified
- Tool: apcacontrast.com

### When to break rules
- Placeholders (gray-9 on gray-2 typically below 4.5:1 — acceptable per ARIA)
- Disabled states (intentional reduced contrast)
- Decorative SVGs / backgrounds
- Brand colors at hover states that get darker

## Gradients in 2026

### Mesh gradients
- Multi-stop, soft, often hero-spanning
- Stripe and Linear are reference points
- Tools: meshy.dev, css-doodle, hand-built radial-gradient compositions

### OKLCH gradients
- Use `linear-gradient(in oklch, ...)` for perceptually clean transitions
- Avoid muddy middles when blending complementary hues
- Example: `linear-gradient(in oklch, oklch(60% 0.2 30) 0%, oklch(60% 0.2 270) 100%)` — same lightness, hue rotation, no muddy gray middle

### Conic and radial
- Conic for AI-shimmer effects (`conic-gradient(from ...)`)
- Radial for hero focus / spotlight
- Combine layers (`background: radial-gradient(...), linear-gradient(...)`) for complexity

### Grain / noise overlay
- 1–3% opacity SVG noise filter on top of gradients
- Adds analog warmth, reduces banding on cheap displays
- Example:
```css
.gradient-hero {
  background:
    url(/noise.svg),
    linear-gradient(in oklch, var(--brand-3), var(--brand-9));
  background-blend-mode: overlay;
}
```

## Accent and semantic colors

### Don't overload
- Red for both errors AND CTAs is confusing
- If brand is red, use a desaturated red for errors and a stronger red for brand
- Better: brand is its own hue; semantic colors are distinct

### Color-coding by category
- **State:** info / success / warn / danger
- **Identity:** brand / accent
- **Surface:** background / card / overlay
- Keep these layers separated. A "primary button" should be `--brand-solid`, not `--blue-9`.

## Anti-patterns

- Inverting light mode for dark mode (chroma stays too high, contrast too sharp)
- Using `#000` and `#fff` for text — too harsh
- Gradient between two hues 180° apart in RGB (muddy gray middle; use OKLCH)
- Too many brand colors (rainbow brand = no brand)
- Semantic color overload (red for "important" everything)
- No accessibility check on hover states
- Forgetting focus-ring contrast (it must hit 3:1 against adjacent colors)

## Reference palettes

### Linear
- Bg: ~#08090a (near-black, cool)
- Surface: ~#16171a
- Text high: ~#fafafa
- Text low: ~#888
- Brand: #5e6ad2 (their purple) — used sparingly
- Accent: gradient mixes purple/blue/teal in headers

### Vercel
- Bg light: #fff / Bg dark: #000
- Geist-grey 12-step ramp (open-source)
- Brand: #0070f3 (blue) — buttons, links, accents
- Highly disciplined mono-plus-accent

### Stripe
- Brand: #635bff (purple)
- Background: white in marketing, near-black in docs
- Gradients: signature multi-stop in heroes
- Lots of color in product UI for status differentiation

### Cursor
- Near-black background
- Terminal green / cyan accent
- Heavy gradient mesh in hero
- Mono type throughout for terminal feel

### Family
- Near-mono with warm beige neutrals
- Single accent that varies by campaign
- Restraint as identity

### Apple
- White / near-white in marketing
- Black / near-black in dark mode
- Brand colors per product (red for AppleCare, blue for App Store, etc.)
- Display-P3 throughout

### Raycast
- Dark by default
- Magenta/pink as brand accent
- Vibrant gradients in hero
- Rich semantic color in product UI

## Sources

- radix-ui.com/colors — the canonical 12-step model
- oklch.com — interactive OKLCH picker
- evilmartians.com (blog on OKLCH and modern color)
- web.dev/learn/css/color
- Tailwind CSS v4 color docs

---
*Last updated: 2026-05-28*
