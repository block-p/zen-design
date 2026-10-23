# Typography Systems — Foundations

> **Source note:** Written from training-data knowledge (~Apr 2026). Stable concepts; refresh font catalog after session reset.

## Why type carries ~70% of perceived quality

Before color or motion register, the visitor has read a headline. If the type is wrong — wrong face, wrong size, wrong tracking — the site reads cheap no matter what else happens. Type is the single highest-leverage design decision.

## Picking a typeface in 2026

### When to use variable vs static
- **Variable** when the family supports it — one file, every weight, smaller payload than 2+ statics. Geist, Inter, Manrope, Recursive, Roboto Flex, Söhne (now variable in 2025+).
- **Static** when the family doesn't have variable cuts, or when you only need 1–2 weights and the static file is smaller.

### Sans categories (signal)
- **Geometric sans** (Geist, Manrope, Object Sans, Neue Haas Unica) — signals tech, efficiency, neutrality
- **Humanist sans** (Inter, Söhne, ABC Diatype, Suisse Int'l) — signals warmth, premium SaaS, editorial
- **Grotesque** (Helvetica Now, GT America, Akzidenz) — signals classic, corporate-but-confident
- **Neo-grotesque** (Söhne, Inter Display) — bridges humanist and grotesque

### Serif comeback (editorial)
- Editorial New, Reckless, Tobias, Söhne Schmal, GT Sectra, Ranger
- Use as display only — body serif on the web is still risky outside long-form reading
- Pairs beautifully with restrained sans body (Reckless display + Inter body is a 2024–2026 staple)

### Mono as accent
- Geist Mono, JetBrains Mono, Berkeley Mono (premium pick), IBM Plex Mono, Fragment Mono
- Use for: code samples, numbers in metrics, terminal motifs, accent in nav/footer
- Mono in body copy is rarely right outside developer docs

### Current popular faces with character (2025/2026)
- **Geist / Geist Mono** — Vercel's open-source family; geometric, generous spacing
- **Inter / Inter Display** — open default; Display variant is for sizes above 28pt
- **Söhne** (Klim) — premium humanist; Linear, Stripe, others
- **Suisse Int'l** (Swiss Typefaces) — premium grotesque; broad use
- **ABC Diatype / ABC Whyte / ABC Repro** (Dinamo) — high-craft commercial faces
- **PP Neue Montreal / PP Neue Machina / PP Editorial New** (Pangram Pangram) — accessible quality
- **Editorial New, Reckless, Tobias** (PP) — serif display
- **GT America, GT Walsheim, GT Sectra** (Grilli Type) — broad-use commercial
- **Tasa Orbiter, Tasa Explorer** (TASA) — character-rich, indie favorites
- **Berkeley Mono** — premium mono, distinctive
- **JetBrains Mono** — open mono, popular default

## Type scales

### Modular scales (ratio-based)
- 1.125 (Major Second) — extremely tight, near-prose
- 1.200 (Minor Third) — common for dense UIs
- 1.250 (Major Third) — default for most marketing
- 1.333 (Perfect Fourth) — bold but balanced
- 1.414 (Augmented Fourth) — display-heavy editorial
- 1.500 (Perfect Fifth) — bold, magazine
- 1.618 (Golden Ratio) — most dramatic

### Fluid type (clamp)
Use `clamp(min, preferred, max)` to scale type with viewport. Example:
```css
font-size: clamp(2.5rem, 6vw, 5rem); /* h1 fluid */
```
Pair with a fixed scale at certain breakpoints if needed.

### Display vs body sizing
- **Body:** 15–18px (1rem≈16px is the sweet spot for marketing)
- **Long-form reading:** 18–22px body
- **UI dense:** 13–15px body
- **Display:** clamp(2.5rem, 6–8vw, 5–8rem) for most heroes

## Optical sizing, tracking, leading

### Body
- Leading: 1.4–1.6 (1.5 is the default; 1.6 for long-form)
- Tracking: 0 to -0.005em (just slightly tighter than normal)
- Weight: 400 for body, 500 for emphasis (not bold-only)

### Display
- Leading: 0.95–1.1 (large type wants tighter leading)
- Tracking: -0.02em at 32px → -0.04em at 64px → -0.06em at 128px
- Weight: 500–600 default for impact, 700+ for aggressive
- Variable weight axis lets you push to e.g. 580 (between Medium and Semibold)

### Caps / all-caps
- Tracking: +0.05em to +0.1em (caps need spacing, not less)
- Weight: medium or semibold (regular caps reads weak)

### Mono
- Tracking: 0 (don't override)
- Leading: 1.5–1.6
- Use in slightly smaller size than surrounding body (mono characters are wider)

### When to break these rules
- Brutalist/anti-design — push tracking to extremes deliberately
- Editorial display moment — sometimes -0.08em tracking and 0.85 leading on a giant headline
- Subhead caps with extra letter-spacing as a "small caps" substitute

## Hierarchy patterns

- **Most sites use 2 weights total.** Body regular + body medium (for emphasis) + display semibold = three uses, two weights.
- **Color is a hierarchy tool stronger than size.** A 14px gray-9 caption next to 14px gray-12 body is hierarchy.
- **Weight is stronger than size at small scales.** 14/500 vs 14/400 reads more clearly than 14/400 vs 15/400.
- **Don't bold full sentences for "emphasis."** Use weight on the noun, color on the noun, or italic where appropriate.

## Pairings

### Sans + mono accent
- Linear: Inter body + Berkeley Mono accent
- Vercel: Geist body + Geist Mono accent
- Cursor: GeistMono throughout for terminal feel, with Inter for support

### Serif display + sans body (editorial)
- Reckless display + Inter body
- Tobias display + Söhne body
- Editorial New display + Suisse Int'l body

### All-sans with weight contrast
- Söhne Mittelschrift display + Söhne body
- Inter Display + Inter body
- GT America Expanded display + GT America body

### Pairing rules
- Never two display-grade serifs together
- One contrast axis at a time (face OR weight OR width — not all three)
- Test at the actual sizes you'll use, not Figma defaults

## Loading strategy

### Format
- WOFF2 only (universal support, smallest)
- Subset to Latin (or Latin-ext if needed) — saves 70%+
- Variable font as single file when family has it

### Loading
- `<link rel="preload" as="font" type="font/woff2" crossorigin>` for critical body face
- `font-display: swap` for fast paint (acceptable FOIT) OR `font-display: optional` for stricter perf
- Self-host (avoid Google Fonts CSS roundtrip — copy the woff2 and serve from same origin)
- For Next.js: `next/font/local` or `next/font/google` (the latter auto-self-hosts)

### Loading example (CSS)
```css
@font-face {
  font-family: 'Geist';
  src: url('/fonts/Geist-Variable.woff2') format('woff2-variations');
  font-weight: 100 900;
  font-display: swap;
}
```

## Anti-patterns

- Three or more typefaces on one site (display + body + mono is already the limit)
- Headings smaller than body (rare, almost never right)
- Body type below 14px on marketing (UI exception)
- Forgetting `lang` attribute (kerning + accent rendering depends on it)
- Using a free font that's missing characters (em dashes, curly quotes, ligatures)
- Letting line length exceed ~75ch for body (60–70ch is ideal)
- Justified text on the web (almost always wrong)
- Default `letter-spacing: normal` on display sizes (always go negative on large display)
- Not testing weights at actual size (variable font weight at 700 in display may look very different at body)

## Reference sites and their type setups

| Site | Display | Body | Mono | Notes |
|---|---|---|---|---|
| Linear | Inter Display | Inter | Berkeley Mono | Tight tracking on display, generous on body |
| Vercel | Geist | Geist | Geist Mono | Pure Geist family, variable, single file |
| Stripe | Sohne | Sohne | Sohne Mono | Klim-licensed; classic premium look |
| Cursor | Geist Mono (everywhere) | Inter | Geist Mono | Mono-as-identity |
| Apple | SF Pro Display | SF Pro | SF Mono | Native font stack, system speed |
| Anthropic | Tiempos / Styrene | Styrene | — | Editorial restraint |
| Family.co | ABC Diatype | ABC Diatype | — | Single-family discipline |
| paco.me | PP Editorial New | Inter | — | Serif-display indie portfolio |
| rauno.me | Geist | Geist | Geist Mono | Vercel stack, fluid scale |

## Sources

- practicaltypography.com (Matthew Butterick — opinions worth disagreeing with carefully)
- web.dev/learn/typography
- Klim, Pangram Pangram, Dinamo, Grilli Type type foundries
- variablefonts.io

---
*Last updated: 2026-05-28*
