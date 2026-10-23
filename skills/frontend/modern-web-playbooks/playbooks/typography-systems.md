# Typography Systems
**Drawn from:** 55 design notes (01 cinematic-space-travel through 55 nexto-404)
**Last updated:** 2026-05-28

A cross-design synthesis of the typographic moves that recur — and the small set that genuinely separate the craft-tier work from competent template assembly. The batch is dominated by a shared "motionsites.ai" lineage; that lineage has an obvious typographic dialect, but the outliers (NHM #13, Orbis #17, Behind the Lens #50, Duolingo #53) reveal the dialect's edges.

## Patterns observed

### Pattern: Instrument Serif italic as single-word accent
**Used by:** #04 Aethera, #05 Velorah, #06 Asme v1, #09 Asme v2, #14 MicroVisuals, #16 Asme v3, #26 Mindloop, #33 Bloom, #40 Nexora, #43 dot., #46 Stellar.ai, #47 booking site, #48 Convix, #49 Neuralyn, #51 AI Website Builder, #52 Design Rocket. **15+ designs, ~30% of the batch.**
**Recipe:**
- Load Instrument Serif via Google Fonts with `ital@0;1` (both regular and italic axes).
- Wrap exactly *one word* per headline in `<em>` or `<span>` with `font-family: 'Instrument Serif', serif` + `font-style: italic` while the surrounding sentence stays sans (Inter or Barlow).
- Headline scale: `text-5xl` / `text-6xl` / `text-7xl` ladder (~48-96px).
- Mindloop holds the strictest discipline: **exactly four italic words on the entire page** (Inspired / changed / meaningful / Journey), one per section.
- Pick the italic word for **semantic weight**: the noun that names the page's promise (Inspired, Smarter, meaningful, Overview, Journey, perfect, Agencies).

**When it works:** When the italic word is the *thematic noun* of the sentence and the rest is utility prose. The italic functions as a hand-set emphasis the eye reads as "this is the word that matters."
**When it fails:** When italicized on a verb (reads as accidental emphasis), or when used more than ~once per visible section (loses signal — see #43 dot. which italicizes "and" — borderline).
**Variations seen:**
- Velorah #05 inverts the italic stylistically (`<em className="not-italic text-muted-foreground">`) — keeps the gray, drops the slant. Same emphasis-by-quieting idea, on dark mode.
- Asme v3 #16 weaves italic + `text-white/40` into structural multi-word phrases ("Innovation × Vision" with × in italic-muted).
- LinkFlow #12 substitutes saturation for italic-color: same word stays the same face but desaturates (`#85AB8B` muted vs `#336443` saturated for the same headline).

---

### Pattern: Inter + Instrument Serif as the default pair
**Used by:** ~25 designs (most motionsites.ai batch members default to this combo).
**Recipe:**
- Inter for body, nav, microcopy at weights 300/400/500/600 (700 often loaded but unused — see anti-patterns).
- Instrument Serif italic for one mid-sentence word in the hero headline.
- The pair is structurally inseparable: most of the batch never uses Instrument Serif for body, never uses Inter for the italic-emphasis moment.

**When it works:** When the brand voice is contemplative-editorial and the product is AI-native, premium SaaS, studio/portfolio, or hospitality. The Inter-as-utility / Instrument-as-poetry split mirrors the brand's split between feature/tool and aspiration/promise.
**When it fails:** When the brand vertical is *playful* (TOONHUB #02, Duolingo #53) or *industrial-loud* (Orbis #17, HAUL #37) — both abandon this pair entirely.

---

### Pattern: Display fonts surveyed (the secondary face decisions)
**Confirmed appearances across batch:**
- **Fustat** — #45 Taskly (75px headline), #42 Logoipsum (80px). Free Egyptian humanist Latin/Arabic — has wider apertures than Inter, more contemporary character. Two appearances suggest emerging niche.
- **Outfit** — #30 Foundation (56px), #50 Behind the Lens (64px). Geometric display sans by Smartsheet, free on Google Fonts. Quieter than Inter Display, more editorial than Geist.
- **Sora** — #31 SENTINEL AI Spline. Geometric humanist, warmer than Inter, more graphical than Geist.
- **Kanit** — #07 Jack. Thai/Latin geometric-leaning sans, very heavy at weights 800-900 — indie portfolio.
- **DM Sans** — #32 Kresna, #55 Nexto 404. Geometric humanist; pairs with handwriting accent (Caveat).
- **TT Norms Pro** — #44 Halo. Commercial geometric sans, fintech-standard (Revolut). First commercial-license-required face in batch.
- **Helvetica Now Display Bold** — #11 VaultShield (loaded from db.onlinewebfonts.com — licensing gray area).
- **Helvetica Regular only** — #24 RIVR (80px), #41 LUMINA. "No-design design" Swiss restraint.
- **PP Neue Montreal + PP Mondwest** — #27 Viktor Oddy. Pangram-Pangram premium pair: humanist sans body + transitional serif italic accent on *exactly four words* (next wave / bold way / Apple / builders).
- **Anton + Condiment** — #02 TOONHUB, #17 Orbis.Nft. Geometric condensed uppercase + brush-script cursive. The two loudest designs in the batch.
- **Plus Jakarta Sans** — #29 CodeNest (only on 11px eyebrow — single-element role).
- **Schibsted Grotesk + Fustat + Inter + Noto Sans** — #42 Logoipsum. Four-face role-assignment system (nav / display / body / fallback).
- **Manrope + Cabin + Inter + Instrument Serif** — #47 booking site. Four-face role assignment (nav / button / body / headline italic).
- **Nokia Cellphone FC Small** — #43 dot. Pixel/LCD font for typed-message overlay on retro Nokia phone in video. One-element use.
- **Neue Haas Grotesk** — #12 LinkFlow. Christian Schwartz premium grotesque (commercial license required).
- **Feather Bold + Nunito** — #53 Duolingo style guide. Duolingo's actual production fonts pulled from db.onlinewebfonts.com (brand-replica).
- **Almarai** — #03 Prisma. Humanist sans with bilingual roots — warmer than Inter.
- **Source Serif 4** — #33 Bloom. Used italic-only for one word inside a Poppins headline.
- **System default only** — #15 Axion Studio. Deliberate restraint as statement.

**Decision tree:**
- Default to Inter + Instrument Serif unless brand vertical demands otherwise.
- For fintech: TT Norms Pro (Halo) signals premium banking.
- For Swiss-neutrality: Helvetica Regular only (RIVR, LUMINA).
- For maximalist poster: Anton + Condiment (Orbis, TOONHUB).
- For agency-craft: PP Neue Montreal + PP Mondwest (Viktor Oddy).
- For "I have taste": Neue Haas Grotesk (LinkFlow).
- For consumer/playful: DM Sans + Caveat (Kresna), or Nunito + Feather (Duolingo).
- For brutalist-restraint: system font (Axion).

---

### Pattern: Gradient-clipped text
**Used by:** #07 Jack (hero headline `linear-gradient(180deg, #646973 0%, #BBCCD7 100%)`), #06 Asme v1 (`from-white via-white/95 to-white/70`), #22 Aura ("Revitalized" with 7-stop shiny gradient), #38 DesignPro ("Product Leader." with `#64CEFB` sweep), #39 Xero (headline white-to-purple), #51 AI Website Builder ("Build Faster" white-to-`#b4c0ff` lavender), #55 Nexto 404 (Material Symbols glyphs with multi-stop gradient).
**Recipe:**
```css
background: linear-gradient(<angle>, <stops>);
-webkit-background-clip: text;
-webkit-text-fill-color: transparent;
```
**When it works:** When the gradient hues match the page's chromatic field (Jack's cool steel matches the dark-canvas portrait, Aura's cyan ramp matches the brand). Best on a single word inside a longer sentence.
**When it fails:** On more than ~10 characters at small sizes (gradient becomes invisible). When the gradient banding shows on cheap displays — add 1-3% noise overlay.

---

### Pattern: Massive headlines — the text-[80-136px] cluster
**Confirmed cluster:**
- #14 MicroVisuals — `clamp(96px, 18vw, 280px)` — biggest in batch
- #38 DesignPro — `text-9xl` (~128px) at xl breakpoint
- #51 AI Website Builder — 136px Instrument Sans
- #50 Behind the Lens — 64px Outfit (editorial-magazine scale)
- #02 TOONHUB — `clamp(90px, 28vw, 380px)` Anton-900 ghost text
- #03 Prisma — `text-[26vw]` cascade scaling to `text-[19vw]` (massive wordmark)
- #36 Vize — `text-[min(25vw,400px)]` (~400px wordmark below footer)
- #44 Halo — 88rem container, headline at hero-scale
- #45 Taskly — 75px Fustat at hero
- #46 Stellar.ai — `text-[80px]` lg display
- #42 Logoipsum — 80px Fustat
- #18 Bold Vision — `clamp(2rem, 9vw, 9rem)` ~144px ceiling
- #29 CodeNest — `text-5xl md:text-7xl lg:text-[80px]` (80px ceiling, Inter Extra Bold)

**Recipe (display):**
- `font-weight: 400-500` — restraint at scale is more premium than 700+
- `line-height: 0.85-1.05` — crushed leading
- `letter-spacing: -0.02em to -0.06em` per the foundations curve, or pixel-tuned (`-2.46px` in Aethera/Velorah, `-4.8px` in Logoipsum)
- Either `clamp()` for fluid scaling OR a 3-step Tailwind breakpoint ramp; `clamp()` is more sophisticated for one-screen heroes

**When it works:** When the page commits to letting the headline be the loudest object and the surrounding chrome shrinks to support. Background must be visually quiet (single-hue dark, near-white, or video softened by overlay).
**When it fails:** When competing motion (video without overlay, animated background) prevents the headline from being the focal anchor. When weight is also pushed to 700+ (reads as shouting, not commanding).

---

### Pattern: clamp() fluid type
**Used by:** #14 MicroVisuals (`clamp(96px, 18vw, 280px)`), #18 Bold Vision (`clamp(2rem, 9vw, 9rem)`), #25 Core Features, #31 SENTINEL AI (entire scale `clamp(3rem, 8vw, 6rem)` etc. — zero breakpoint changes), #34 Glowing Cards, many others.
**Recipe:** `font-size: clamp(<floor>, <viewport-relative>, <cap>)`. SENTINEL goes furthest — entire type scale runs on `clamp()`, no breakpoint utility classes.
**When it works:** On single-screen heroes where the headline must scale smoothly across all viewports. Eliminates the "375px = small, 376px = jump to medium" discontinuity.
**When it fails:** On complex multi-section pages where breakpoint-tied size changes serve different reading rhythms per section.

---

### Pattern: Italic emphasis as soft-emphasis (gray-on-light or white/60 on dark)
**Used by:** #04 Aethera (gray italic on white), #05 Velorah (gray on dark, not-italic em — stripped slant), #16 Asme v3 (text-white/40 italic mixed into structural headline), #33 Bloom (text-white/80 italic).
**Recipe:** Italic phrases get demoted in **color**, not promoted — inverts the conventional "italic = louder" relationship.
- Light mode: italic + `text-[#6F6F6F]` (a notch above body gray)
- Dark mode: italic + `text-white/40` to `text-white/60` (Bloom uses /80, gentler)
- Or: strip italic, keep color (Velorah's `not-italic text-muted-foreground`)

**When it works:** When the brand voice is contemplative; the demoted phrases function as the *backdrop concepts* against a foregrounded verb. Reads as "watch the verb act on the noun."
**When it fails:** When the italic phrases carry more meaning than the surrounding text (then they need to be promoted, not demoted).

---

### Pattern: Opacity ladder as type hierarchy (white-on-black variant)
**Used by:** #01 Cinematic, #14 MicroVisuals, #16 Asme v3, #41 LUMINA, others.
**Recipe (Asme v3's six-tier ladder):**
- `text-white` — primary surface (headlines, button text)
- `text-white/80` — nav links, supporting copy
- `text-white/70` — body paragraphs
- `text-white/60` — soft body, muted headline portions
- `text-white/50` — descriptions, secondary
- `text-white/40` — metadata, captions, muted accents
- `rgba(255,255,255,0.1)` — borders/dividers

Six tiers of white-on-black, semantically associated with copy roles. **The opacity ladder is the type hierarchy** — replaces a gray ramp entirely.
**When it works:** On pure black or near-black with video underneath. Background varies in tone, so opacity gives different reads on different frames.
**When it fails:** On flat white surfaces where opacity reads as faded. Switch to gray ramp.

---

### Pattern: Mono accent for metadata
**Used by:** #13 NHM (JetBrains Mono at 10px / `tracking-[0.2em]` for all metadata — section index, chapter counter, captions), #27 Viktor Oddy (system mono for tagline below wordmark), #18 Bold Vision (`tracking-widest` caps approximation), #42 Logoipsum (Schibsted Grotesk Medium 14px for nav).
**Recipe:**
- `text-[10px]` to `text-[12px]`
- `tracking-[0.2em]` to `tracking-widest`
- `UPPERCASE`
- Either JetBrains Mono, Berkeley Mono, IBM Plex Mono, or system monospace
- Restricted to: section indices, chapter counters, page captions, footer ledger lines, sub-nav metadata

**When it works:** As a single recurring token applied across all metadata. NHM's discipline is the model — *one CSS class, four jobs*, immediately recognizable as "metadata."
**When it fails:** Mixed with display serif on body copy. Mono in body copy is rarely right outside developer docs.

---

### Pattern: All-caps + tracking-widest + single weight as page-wide voice
**Used by:** #07 Jack, #08 Vex Ventures, #15 Axion Studio, #18 Bold Vision (most committed), #50 Behind the Lens.
**Recipe:** Inter Semibold + UPPERCASE + `tracking-widest` (or wider). Hierarchy via *size only*. Multi-size scale: nav (14px) → eyebrow (12px) → headline (48-144px). Editorial-poster discipline.
**When it works:** Agency portfolios, fashion lookbooks, brutalist-adjacent sites. Bold Vision pulls this off most rigorously — caps + tracking-widest across every text block on the page.
**When it fails:** Long-form body copy in caps is illegible. Restrict caps to labels, headlines, CTAs.

---

### Pattern: Two-tone headline split (color/saturation)
**Used by:** #10 SkyElite (lightness step — `gray-500` "Premium." behind `#202A36` "Accessible."), #12 LinkFlow (saturation step — `#85AB8B` muted behind `#336443` saturated for the same headline), #16 Asme v3 (italic-muted mixed), #17 Orbis (cursive-accent on one word in Anton headline), #18 Bold Vision (clip-revealed words at same scale).
**Recipe variants:**
- **Lightness split** (SkyElite): same size, same weight, same tracking — only color separates. Light tier (`gray-500`) reads as recessed; dark tier (`navy #202A36`) reads as forward. Line 2 pulled up 12px with inline style for visual overlap.
- **Saturation split** (LinkFlow): same hue family, saturated tier in front, muted tier behind. Works only if your brand color has a saturation ramp.
- **Loud-accent split** (Orbis): cursive-neon swap on one word inside an Anton headline — the inverse of the gray-italic emphasis pattern.

**When it works:** When the headline is a two-clause antithesis ("Premium / Accessible") or two-phrase metaphor ("Close the rift / linking signals and action").
**When it fails:** On more than two clauses (reads as 5-color noise). When the contrast is too low to register the depth effect.

---

## Anti-patterns observed

### Dead-payload fonts (loaded but unused)
**Examples:** #07 Jack (21 GIFs in marquee plus 4 Inter weights only 3 used), #11 VaultShield (700 weight loaded, never used), #12 LinkFlow (Inter weights 300-700 loaded, never used), #14 MicroVisuals (Dirtyline 36 Days of Type loaded, never used), #36 Vize (300 and 600 Inter weights unused).
**Why it matters:** Each weight = 25-60KB of payload. Strip from `<link>` URL before production.

### Three-cream-token bug
**Example:** #03 Prisma — inline `#E1E0CC` AND Tailwind `primary: #DEDBC8` are both "the brand cream." Two near-identical colors will inevitably get used adjacent, creating a 2-3 step OKLCH lightness step that shows on bright displays.
**Why it matters:** Pick one cream, delete the other, refactor all inline-style overrides to the token.

### Headline tracking left at default
**Examples:** #06 Asme v1 (-0.01em at 64px when foundations call for -0.02 to -0.04em). Looser than ideal but face widths compensate.
**Why it matters:** At 64px+ display, default tracking reads loose/amateur. Push to -0.02em minimum, -0.04em ideally, hand-tune per face.

### Onlinewebfonts.com source
**Examples:** #11 VaultShield (Helvetica Now), #12 LinkFlow (Neue Haas Grotesk), #24 RIVR (Helvetica with legacy `.eot` fallback chain), #41 LUMINA (Helvetica), #43 dot. (Nokia Cellphone FC Small), #53 Duolingo (Feather Bold).
**Why it matters:** Licensing gray area. Will likely 404 or rate-limit in production. Use properly licensed commercial sources or open-source equivalents (Inter, PP Neue Montreal, ABC Diatype).

### British spelling inside a US brand
**Example:** #25 Core Features ("centred" in the prompt-box body of an otherwise US-default brand).
**Why it matters:** Detail-level inconsistency reveals copy was not reviewed.

### Headline writing checks the design can't cash
**Example:** #09 Asme v2 — "Built for the curious" headline followed by generic newsletter boilerplate. Same subtitle copied verbatim into #16 Asme v3.
**Why it matters:** Headline raises the brand voice; subtext must match the register or the voice collapses.

---

## Decision rules

1. **Default to Inter + Instrument Serif italic for one word per headline.** If the brand vertical demands otherwise (fintech: TT Norms Pro; Swiss-neutrality: Helvetica only; playful: DM Sans + Caveat; agency-craft: PP Mondwest), switch — but commit fully, don't half-do.
2. **Restrict italic to nouns and adverbial phrases.** Verbs and pronouns stay roman.
3. **Hold the italic count to 1-4 per visible section.** Mindloop's "exactly four italics on the entire page" is the disciplined ceiling.
4. **For display sizes 64px+, push tracking to -0.02em minimum, -0.04em ideally.** At 96px+, go to -0.05em or hand-tune per face.
5. **For display weight, default to 400-500 not 600-700.** Restraint at scale is the premium signal.
6. **Use opacity ladders on video/dark backgrounds, gray ramps on flat surfaces.**
7. **Mono accent is for metadata only.** 10-12px / `tracking-[0.2em]` / UPPERCASE. One token, multiple jobs.
8. **Caps need spacing.** `tracking-wider` (+0.05em) minimum, `tracking-widest` (+0.1em) for very small caps.
9. **Three faces is the ceiling — except when each gets a single role.** Logoipsum #42 (4 faces) and booking site #47 (4 faces) work because each face stays in one swimlane. Most batch designs hold to 2 faces.
10. **Avoid `whitespace-nowrap` on headlines without a tested overflow strategy.** Asme v2 / v3 both lock "Built for the curious" / "Know it then all" — any copy edit breaks the layout.

---

## Open questions / gaps

- Is the motionsites.ai dialect a tool's default or a designer's preference? The Inter + Instrument Serif + Barlow + liquid-glass + 9999px-pill stack appears 30+ times. Unlikely to be coincidence.
- Why is Fustat showing up (#42, #45) as the new display sans? Two appearances suggest emerging trend. Worth tracking.
- The two pages that load four typefaces (#42 Logoipsum, #47 booking site) both have *role-specific font assignments* — is this an emerging counter-pattern to the foundations' "3-face ceiling"?
- Webfonts loaded but unused appear in **5+ designs** — is this a sign of generative-design pipelines auto-scaffolding fonts that get pruned later, or do these pages ship with the dead payload?
- Designs #45 Taskly and #47 booking site both use *purple* as primary CTA. Is this an emerging "premium consumer-app" convention vs the SaaS-blue default?
- NHM #13 (the outlier from a different toolchain) uses JetBrains Mono + Inter only and gets more typographic mileage per pixel than most batch members. Is the metadata-mono discipline portable to motionsites-batch designs?
