---
name: marp-slides
description: Create beautiful MARP presentation decks with SVG charts, interactive elements, dashboard components, animations, dark/light themes. Triggers on 'marp', 'slides', 'presentation', 'deck'.
version: 2.0
updated: 2026-04-08
---

# MARP Slides v2

## Prerequisites
- VS Code extension "Marp for VS Code"
- Settings: `markdown.marp.enableHtml: true` + `markdown.marp.allowLocalFiles: true`
- Export: `npx @marp-team/marp-cli slides.md --pdf --allow-local-files`

## Example Decks (CRITICAL — read before generating)

The `examples/` folder contains 22 curated reference decks. **Before generating any deck, read 2-3 examples that match the requested style.** These are the quality bar — match their composition, spacing, and visual density. Key examples by category:

| Category | Examples to read |
|---|---|
| Data / dashboard | `marp_facebook-ads.md`, `marp_fitness.md`, `marp_comparison.md` |
| Lifestyle / editorial | `marp_coffee.md`, `marp_wine-tasting.md`, `marp_cocktail.md` |
| Guide / how-to | `marp_garden.md`, `marp_houseplant.md`, `marp_home-gym.md` |
| Fun / creative | `marp_kids-party.md`, `marp_board-game.md`, `marp_film-director.md` |
| Travel / location | `marp_travel.md`, `marp_walking-tour.md`, `marp_road-trip.md` |
| Showcase / hero | `marp_hero.md`, `marp_apartment.md`, `marp_wardrobe.md` |
| Reference / sampler | `marp_sample.md` (component showcase) |

## Core Rules
- Slides separated by `---`
- YAML frontmatter controls theme/pagination/styles
- `enableHtml` unlocks SVG, cards, charts, animations, interactive elements
- Default 16:9 (1280x720)

## Dark Starter Template

The dark template uses Outfit 800 for headings and Raleway 100-200 for body text. CSS variables:

```css
@import url('https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700;800&family=Raleway:wght@100;200;300&display=swap');

:root {
  --accent: #ff6b1a; --accent-hover: #ff8c4a;
  --dark: #000; --card: #080808; --border: #111;
  --body: #999; --label: #666; --muted: #555; --light: #fff;
  --green: #22c55e; --red: #ef4444; --yellow: #f5a623;
}
section { background: var(--dark); color: var(--light); font-family: 'Raleway', sans-serif; font-weight: 200; padding: 56px 72px; }
h1 { font-family: 'Outfit'; font-weight: 800; font-size: 3em; color: var(--light); }
h2 { font-family: 'Raleway'; font-weight: 100; font-size: 1.3em; color: #888; }
h3 { font-family: 'Outfit'; font-weight: 600; font-size: 0.6em; color: var(--muted); text-transform: uppercase; letter-spacing: 0.2em; }
strong { color: var(--accent); font-weight: 300; }
section.lead { display: flex; flex-direction: column; justify-content: center; align-items: center; text-align: center; }
header { text-align: right; } header img { margin: 0; }
.row:hover { background: #0c0c0c; } .row { transition: background 0.2s; border-radius: 6px; }
details { background: var(--card); border: 1px solid var(--border); border-radius: 10px; padding: 14px 18px; margin-top: 8px; }
details summary { color: var(--accent); font-family: 'Outfit'; font-weight: 600; font-size: 0.8em; cursor: pointer; }
details p { color: var(--body); font-size: 0.78em; margin-top: 8px; }
.tag { font-family: 'Outfit'; font-weight: 600; font-size: 0.55em; letter-spacing: 0.12em; text-transform: uppercase; padding: 3px 10px; border-radius: 4px; }
abbr { text-decoration: none; border-bottom: 1px dotted #333; cursor: help; }
```

Frontmatter includes: `header: '![w:100](./logo.png)'`

## Light Theme
Swap vars: `--accent: #2563eb; --dark: #fafafa; --card: #fff; --border: #eee; --body: #666; --label: #bbb; --light: #1a1a1a;`
Use Space Grotesk + IBM Plex Mono or Plus Jakarta Sans.

## Heading Hierarchy
- h1 = title slides (white, extra large)
- h2 = subtitle (grey, thin)
- h3 = section label (muted, uppercase, small)

## Font Pairings (Tested)

| Heading | Body | Use |
|---|---|---|
| Outfit 800 | Raleway 100 | Dashboard, data (default) |
| DM Serif Display | DM Sans 300 | Recipes, editorial |
| Space Grotesk 700 | IBM Plex Mono 300 | Travel, light themes |
| Sora 700 | Sora 200 | Product comparisons |
| Urbanist 800 | Urbanist 100 | Music, Spotify-style |
| Plus Jakarta Sans 800 | Plus Jakarta Sans 200 | Retros, team decks |

## Images

CRITICAL: Relative paths only. `./image.png` works. Absolute paths break in preview.

- Logo header: `header: '![w:100](./logo.png)'` — hide per slide: `<!-- _header: '' -->`
- Photo bg: `![bg brightness:0.15](https://unsplash.com/photo-ID?w=1400)`
- Split: `![bg right:35% brightness:0.2 blur:3px](url)` or `![bg left:30%](url)`
- CDN logos: `<img src="https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/name.png" style="width:200px;" />`
- Centered inline: wrap img in `<div style="display:flex; justify-content:center;">` with border-radius and border

## Dashboard Components

### Metric Card (gradient top border)
Card with `position:relative; overflow:hidden;` and absolute div at top with `background:linear-gradient(90deg, var(--accent), transparent); height:2px;`. Icon + label + big number + trend arrow.

### Status Dots
Inline SVG circles: green (#22c55e) = active, yellow (#f5a623) = learning, red (#ef4444) = paused.
`<svg width="8" height="8" viewBox="0 0 8 8"><circle cx="4" cy="4" r="4" fill="#22c55e"/></svg>`

### Verdict Tags
`<span class="tag" style="background:#22c55e12; color:var(--green); border:1px solid #22c55e22;">Scale</span>`
Swap colors for red (kill) and yellow (review).

### Hover Rows
Wrap content in `<div class="row">` for hover highlight effect.

## SVG Charts

### Line / Area Chart
SVG polyline for the line + polygon with linearGradient fill for area under the line. Add grid lines, dashed target line, circle data points. Use viewBox="0 0 900 240" with preserveAspectRatio="none".

### Pie / Donut Chart
Each segment = separate circle with stroke-dasharray and stroke-dashoffset. Math: circumference = 2*pi*r. For r=110: ~691. Segment = (pct/100)*691. Offsets accumulate negatively. stroke-width controls ring thickness. Always transform="rotate(-90 cx cy)".

### Gauge / Half-Circle Meter
SVG path arc for background + colored value arc. Needle line from center + circle pivot. For scores 0-100. stroke-linecap="round".

### Donut Ring
Single circle stroke-dasharray. circ = 2*pi*r. Offset = circ - (circ * pct/100). For r=74: circ=465. 89%: offset=51.

### Sparkline (inline mini)
`<svg width="50" height="16"><polyline points="0,14 8,12 16,10 24,8 50,2" fill="none" stroke="#22c55e" stroke-width="1.2"/></svg>`

### Stacked Bar
Flex div with colored width-percent segments, border-radius, overflow:hidden.

### Vertical Bar Chart
Flex container align-items:flex-end. Gradient bars: `background:linear-gradient(180deg, var(--accent), #cc5515); border-radius:3px 3px 0 0;`

### Radar / Spider
SVG polygon for hexagonal grid + data shape with fill-opacity:0.1 and stroke outline.

## Interactive Elements

- Collapsible: `<details><summary>Title</summary><p>Content</p></details>`
- Tooltip: `<abbr title="Full text">TERM</abbr>`
- Slider: `<input type="range" style="accent-color:var(--accent);" />`
- Checkbox: `<input type="checkbox" checked style="accent-color:var(--accent);" />`
- Progress: `<progress value="76" max="100" style="accent-color:var(--accent);"></progress>`

## Layout Components

- Before/After Split — two flex panels, border-top red vs green
- Terminal Mockup — traffic light dots + monospace body
- Browser Mockup — dots + URL bar div
- Chat Bubbles — user (left) + agent (right, orange-tinted)
- Flowchart — boxes + SVG arrow connectors
- Timeline — vertical border-left + dot circles
- Card Row — display:flex; gap:14px; with flex:1 children

## SVG Icons

All wrapped in `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5">`. Sizes: inline=16, cards=44, features=32.

Dollar: path d="M12 2v20M17 5H9.5a3.5 3.5 0 1 0 0 7h5a3.5 3.5 0 1 1 0 7H6"
Heartbeat: polyline points="22 12 18 12 15 21 9 3 6 12 2 12"
Check (#22c55e): path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" + polyline points="22 4 12 14.01 9 11.01"
Arrow up (#22c55e): polyline points="18 15 12 9 6 15"
Arrow down (#ef4444): polyline points="18 9 12 15 6 9"
X circle (#ef4444): circle cx=12 cy=12 r=10 + two crossing lines
Clock: circle cx=12 cy=12 r=10 + polyline points="12 6 12 12 16 14"
Eye: path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" + circle cx=12 cy=12 r=3
Lightning: path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"
Warning (#f5a623): triangle path
Search: circle cx=11 cy=11 r=8 + line to corner
Bars: three vertical paths
Users: user silhouette path + circle
Globe: circle + horizontal line + vertical ellipse path
Lock: rect + arch path
Book: two paths for book shape

## Animations (HTML export + preview only)

float: translateY(0) to -8px and back
glow: box-shadow pulse with accent color
blink: border-color toggle for cursors

Stagger with delay: animation: float 4s ease-in-out 0.5s infinite;

## Export

npx @marp-team/marp-cli slides.md --pdf --allow-local-files
npx @marp-team/marp-cli slides.md --pptx --allow-local-files
npx @marp-team/marp-cli slides.md --html --allow-local-files
--pptx-editable needs LibreOffice. Animations + details only in HTML.

## Design Rules

1. One idea per slide. Overflow clips silently.
2. h1 = white. Accent for data highlights only.
3. Body text #999, labels #666. Never darker than #555.
4. Max 6 rows per list slide.
5. Charts over numbers. Mix visual types across slides.
6. Relative paths only for images.
7. Always preview — no overflow warnings in source.
8. Per-slide overrides: _backgroundColor, _header, _paginate, _footer

Custom dimensions: section { width: 540px; height: 720px; } (CSS not size: frontmatter).
Portrait: stack vertically, scale down 15-20%.
