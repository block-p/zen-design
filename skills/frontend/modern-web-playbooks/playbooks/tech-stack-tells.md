# Tech Stack Tells — Frameworks, Libraries, Hosting, Fonts

**Captured:** 2026-05-29
**Scope:** Cross-cutting tech signals across 55 designs in Batch 1.
**Purpose:** Catalog the stacks, CDNs, font sources, and library versions seen — so the next analyzer can identify a brief's lineage in 30 seconds by scanning for the right fingerprints.

## The headline stack

Almost the entire batch resolves to one of two stacks. Naming them up front to anchor every other observation:

| Cluster | React | Vite | Tailwind | Motion lib | Fonts |
|---|---|---|---|---|---|
| **motionsites cluster (~40 designs)** | 18.3.1 | 5 | 3.4 (PostCSS, `@tailwind` directives) | framer-motion 11–12 (imported as `framer-motion`) | Google Fonts CDN, Instrument Serif + Inter |
| **NHM-style outlier (1 confirmed)** | 19.0.1 | 6.2.3 | 4.1.14 (`@import "tailwindcss"`, `@theme`) | `motion/react` (rebranded) | self-hosted via npm or `@fontsource/*` |

The Tailwind v3 / v4 split is the single sharpest tech fingerprint. A `<style>` block opening with `@import "tailwindcss"` means v4; `@tailwind base; @tailwind components; @tailwind utilities;` means v3. Likewise `motion/react` imports = v4 cluster; `framer-motion` imports = v3 cluster. The correlation is near-total.

## Frameworks

### React versions
- **React 18.3.1** is the canonical motionsites version. Confirmed across #01, #07, #09, #15, #18, #22, #26, #28 and many more.
- **React 19.0.1** appears only in #13 NHM and the newer v4 cluster (#43 dot. is "React 19" explicitly).
- **No React (vanilla HTML/CSS/JS)** in #32 Kresna — the only no-React design in the batch.
- **Single-file React (no Vite/Tailwind config)** in #52 Design Rocket — minimal scaffold for an email-style page.

### Vite vs Next.js
**Zero Next.js anywhere in the batch.** Every React-based design uses Vite. This is striking — Next.js is the dominant marketing-page framework in 2026, but the batch has none of it. Implications:
- No SSR. Every page is client-rendered SPA.
- No `next/font/google` auto-self-hosting. Every Google Fonts use is a CDN roundtrip.
- No App Router, no RSC. Every component is client-side.
- No Vercel-default image optimization. Every `<img>` is a raw URL.

The Vite-everywhere choice signals the batch is built for **prototyping and demos**, not production marketing surfaces. Real production sites in 2026 land on Next.js 16.2 or Astro 6 with content-fetching at build time.

### Build configurations
- **CDN-only React + Babel-in-browser (no build)** in #01 Cinematic Space-Travel. Babel standalone 7.29.0 parses JSX in the browser. Acceptable for a portfolio demo, dangerous for LCP-sensitive production.
- **Standard Vite + TypeScript + Tailwind** for everything else.
- **`@tailwindcss/vite` plugin (v4 native)** in NHM #13 and the v4 cluster.

## Styling

### Tailwind versions seen
- **Tailwind 3.4 with default config** — most of the batch. Sometimes extended with `borderRadius: { DEFAULT: '9999px' }` (#01, #14) or custom colors (`brand: '#3D81E3'` in Aura #22, `cream: '#EFF4FF', neon: '#6FFF00'` in Orbis #17).
- **Tailwind 3.4 with custom CSS namespace escapes** — `c1-` (#25), `c3-` (#22), `c5-` (#23). These add a hand-written CSS file alongside Tailwind for sections that need complex layered visuals.
- **Tailwind v4 with `@theme` blocks** — #13 NHM, #24 RIVR, #28 Aurora Sign-up, #30 Foundation, #34 Glowing Cards, #36 Vize, #41 LUMINA, #43 dot. — the modern revision cluster. Uses `--font-sans` / `--font-display` semantic tokens and `@import "tailwindcss"`.
- **Tailwind v4 `@utility` directive** in #43 dot. — replaces `@layer utilities` for custom utility classes. First batch appearance.

### CSS variable strategies
Three distinct strategies live in the batch:

1. **shadcn-style HSL triples** — `--background: 0 0% 0%` defined as space-separated HSL components without `hsl()` wrapping, consumed via `hsl(var(--background))`. Enables opacity composition like `bg-background/45`. Appears in #26 Mindloop, #28 Aurora, #40 Nexora, #49 Convix, #50 Neuralyn. Always paired with `tailwindcss-animate`.
2. **Named semantic tokens** — `--color-text`, `--color-accent`, `--color-login-bg` (#11 VaultShield), `--shadow-dashboard` (#41 Nexora), `--font-display` (#30 Foundation, Tailwind v4 `@theme`). Signals more bespoke design-system thinking.
3. **Hex-and-arbitrary** — `bg-[#0a1b33]`, `text-[80px]` everywhere, no token layer. Most of the v3 cluster (#44 Halo, #48 Convix, #51 AI Builder).

### Custom liquid-glass utility class
The most-shared CSS primitive in the batch. Defined in `index.css` (not Tailwind config) with byte-for-byte identical recipe across 20+ designs:

```css
.liquid-glass {
  background: rgba(255, 255, 255, 0.01);
  background-blend-mode: luminosity;
  backdrop-filter: blur(4px);
  box-shadow: inset 0 1px 1px rgba(255, 255, 255, 0.1);
  position: relative;
}
.liquid-glass::before {
  content: '';
  position: absolute;
  inset: 0;
  padding: 1.4px;
  background: linear-gradient(180deg,
    rgba(255,255,255,0.45) 0%,
    rgba(255,255,255,0.15) 25%,
    transparent 50%,
    transparent 75%,
    rgba(255,255,255,0.15) 100%,
    rgba(255,255,255,0.45) 100%);
  -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
  -webkit-mask-composite: xor;
  mask-composite: exclude;
  border-radius: inherit;
  pointer-events: none;
}
```

The `liquid-glass-strong` sibling boosts blur to 50px and the gradient stops to `0.5 / 0.2`. Recognizing this exact recipe is the fastest motionsites-family identifier after the CloudFront URL check.

## Component libraries

### shadcn/ui
Explicitly mentioned in #05 Velorah, #26 Mindloop, #28 Aurora, #30 Foundation, #36 Vize, #54 Estimation Calculator. Most often the only shadcn primitive actually used is `<Button>` and the CSS-variable convention (raw HSL triples). The library is treated as a *convention kit*, not a heavy dependency.

### Radix / React Aria
Not seen anywhere in the batch. Every drawer, dropdown, dialog is hand-built with `useState` + Tailwind classes. The motionsites cluster appears to be Radix-free by convention.

### Icons
- **lucide-react** dominates — appears in nearly every design. Tree-shakable, per-icon imports. Standard 2025-2026 default.
- **Material Symbols Rounded** in #55 Nexto 404 — the only Material Symbols appearance, used for the floating cloud + heart decorations with `FILL 1` for solid glyphs.
- **Inline SVG paths** for one-off icons (Material Icons paths in #01 Cinematic Space, custom Sunburst in #51 AI Builder).
- **No Heroicons, no Phosphor, no Tabler** anywhere in the batch.

### Animation libraries
- **framer-motion ~11–12** (imported as `framer-motion`) — v3 cluster. Used for `useInView`, `motion.div`, `whileInView`, `useScroll` + `useTransform`.
- **`motion/react`** — v4 cluster (post-2024 rebrand). Same library, new import path.
- **GSAP** — only in #14 MicroVisuals, used specifically as `gsap.set()` for transform performance, not as a tween engine.
- **`shaders/react`** — only in #15 Axion Studio (Swirl, ChromaFlow, FlutedGlass, FilmGrain procedural backgrounds).
- **`@splinetool/react-spline` + `@splinetool/runtime`** — only in #37 SENTINEL AI Spline (3D scene `Slk6b8kz3LRlKiyk`).
- **No GSAP, no Lottie, no Lenis** in the bulk of the batch. The motion vocabulary is framer-motion + custom CSS + occasional rAF loops.

## Hosting / CDNs

### Video hosting
Three distinct pipelines:

1. **CloudFront MP4** at `d8j0ntlcm91z4.cloudfront.net/user_38xzZboKViGWJOttwIXH07lWA1P/hf_<timestamp>_<uuid>.mp4` — the motionsites baseline. Used for ambient backgrounds, native `<video autoplay muted loop playsInline>`. ~40 designs.
2. **HLS Mux stream** at `stream.mux.com/<playback-id>.m3u8` — adaptive bitrate streaming for hero / CTA videos that benefit from quality scaling. Uses `hls.js` with a `canPlayType('application/vnd.apple.mpegurl')` fallback for Safari (native HLS). Used in #06 Asme v1, #26 Mindloop (CTA only), #29 CodeNest, #51 AI Builder. The Mindloop case is uniquely **hybrid** — CloudFront MP4 for ambient + Mux HLS for the moneymaker CTA.
3. **Cloudinary MP4** at `res.cloudinary.com/<account>/...` — #13 NHM (`dsdxaxkiz`), #21 thin entry (`dfonotyfb`).

The HLS subset is small but consistent: `hls.js` (latest v1.6.15+) + `Hls.isSupported()` check first, `canPlayType` fallback. The pattern from #29 CodeNest sets `enableWorker: false` for sandbox-environment compatibility (StackBlitz, CodeSandbox, Claude artifacts).

### Image hosting
- **CloudFront PNGs** wrapped by **Higgs.ai image proxy** — `images.higgs.ai/?default=1&output=webp&url=<urlencoded-cloudfront-path>&w=1280&q=85`. Converts PNGs to optimized WebP on the fly. Appears in #15 Axion Studio, #37 HAUL, and the broader motionsites cluster wherever non-video imagery is needed.
- **Cloudinary direct** at `res.cloudinary.com/dsdxaxkiz/...` — #13 NHM only. No transforms in the URL (no `f_auto,q_auto`).
- **figma.site** at `<hash>.figma.site/_components/...` — #02 TOONHUB characters, #07 Jack portrait. Figma's published-site CDN with hash-based asset URLs.
- **R2** at `pub-f170a2592d2c4a1485466404c36807be.r2.dev` for logo SVGs (#23, #25, #55) and `pub-e68758f43067417dba612b2371819aa1.r2.dev` for spaceship PNG (#55). Cloudflare's S3-compatible object storage. The `viktor/` subfolder appears in #23 and #25 — possibly an author or project codename.

### Webfont hosting
- **Google Fonts CDN via `<link>` or `@import url(...)`** — most designs. The standard easy choice, not the performance-best choice.
- **`@fontsource/*` npm packages** — #26 Mindloop and others in the v4 cluster. `import '@fontsource/inter/400.css'` self-hosts via npm, no Google Fonts handshake. Best performance/ergonomics balance for Vite apps.
- **`db.onlinewebfonts.com`** — font rehost service used in #11 VaultShield (Helvetica Now Display Bold), #12 LinkFlow (Neue Haas Grotesk), #24 RIVR (Helvetica), #41 LUMINA (Helvetica), #43 dot. (Nokia Cellphone FC Small), #53 Duolingo (Feather Bold). Loads with the legacy `eot/woff2/woff/ttf/svg` fallback chain (IE-era padding). Licensing is dubious — these are typically commercial faces (Helvetica Now, Neue Haas Grotesk are licensed-per-domain via Monotype/Commercial Type, not free to redistribute). Prototype-grade, not production.
- **Local woff2** — #27 Viktor Oddy ships `/PPMondwest-Regular.woff2` as a local file (PP Mondwest from Pangram Pangram). The proper move for licensed type.
- **Webflow CDN** at `assets.website-files.com` for PP Neue Montreal in #27 Viktor Oddy. Adds a third-party DNS lookup. Suggests the font was licensed via Webflow's font library and is restricted to Webflow-hosted use.
- **Pangram Pangram** — the foundry behind PP Neue Montreal and PP Mondwest, appears as the premium-accessible font source in #27.

## Fonts seen across the batch

Five font families dominate:

### Instrument Serif (free, Google Fonts)
The single most reused display face. Italic axis (`ital@0;1`) loaded for the per-design italic emphasis trick (one word per headline gets the italic treatment — see copy-voice playbook). Appears in #01, #03, #04, #05, #06, #09, #14, #16, #26, #27 (as accent), #28, #40, #43, #46, #49, #50, #51 — well over 20 designs. Treated as the "premium" alternative to Editorial New / Reckless / Tobias for italic editorial emphasis.

### Inter (free, Google Fonts)
The canonical body face. Weights 300/400/500/600/700 typically loaded; production builds rarely use 700 in practice. Used for body and UI text in every design that doesn't pick a specific alternative (#19, #20, #22, #28, #38, #46, etc.).

### Anton + Condiment (free, Google Fonts)
The outlier pair. Anton (condensed display sans) + Condiment (handwriting script) appear together in #17 Orbis NFT — the only handwriting-script appearance in the entire batch. Anton also appears in #02 TOONHUB at weight 900 for the giant ghost text.

### DM Sans + Material Symbols Rounded (free, Google Fonts)
The Nexto 404 (#55) pair. DM Sans variable axes (`opsz 9..40, wght 100..1000`) + Material Symbols Rounded with `opsz,wght,FILL,GRAD@24,400,1,0`. The `FILL 1` is the key choice — solid icon glyphs, not outlined. Friendly-modern stack.

### Kanit (free, Google Fonts)
Single appearance in #07 Jack 3D Creator. A Thai/Latin geometric-display sans, weights 300-900, used as the *single family for everything* (display + body + UI). Unusual — most designs use a sans+display pair, Jack commits to one family across all sizes.

### Plus Jakarta Sans (free, Google Fonts)
Used as the 11px eyebrow font in #29 CodeNest. Slightly humanist alternative to Inter at small sizes — its open apertures read more like editorial small-caps.

### Helvetica (commercial, rehosted)
Appears in #11 VaultShield (Helvetica Now Display Bold), #24 RIVR (Helvetica Regular), #41 LUMINA (Helvetica Regular). All loaded via `db.onlinewebfonts.com` — licensing-questionable.

### Neue Haas Grotesk (commercial, rehosted)
#12 LinkFlow only. Loaded via `db.onlinewebfonts.com`. Christian Schwartz redesign of original Haas Grotesk (Helvetica's ancestor). Signals deliberate type-engineer-grade choice but ships with dubious licensing.

### PP Neue Montreal + PP Mondwest (Pangram Pangram, premium-accessible)
#27 Viktor Oddy. The high-craft pairing — neo-grotesque sans + transitional-feel serif. PP Neue Montreal loaded via Webflow CDN, PP Mondwest as local woff2.

### Almarai (free, Google Fonts)
#03 Prisma. Humanist sans with bilingual roots, slightly warmer than Inter.

### Outfit (free, Google Fonts)
#30 Foundation. Geometric display sans by Smartsheet — first batch appearance, more contemporary than Inter Display.

### Barlow (free, Google Fonts)
#01 Cinematic Space, #14 MicroVisuals. Slightly condensed humanist grotesque used as the Inter substitute when paired with Instrument Serif italic.

### Fustat, Cabin, Manrope, TT Norms Pro, Sora, Söhne-likes
One-off appearances. Cabin in #48 booking site, Manrope in #48, Sora in #37 SENTINEL Spline, TT Norms Pro in #45 Halo USD.

## Backend / data layer

Almost no backend code in the batch. Two designs explicitly include Supabase:

- **#22 Aura** — declares `@supabase/supabase-js` as a dependency but the spec doesn't render any explicit data-fetching. Likely staged for a waitlist or auth flow.
- **#50 Behind the Lens** — actually uses Supabase as a content CMS, fetching from a `blog_posts` table with `display_order` field and an RLS public-read policy. First batch design with explicit content fetching. Treats Supabase the way a Ghost or Sanity install would be treated — read-only public blog.

No other database or backend integration in the batch. No Firebase, no MongoDB, no Convex, no payment integration, no auth provider (besides Aurora's social-button stub).

## Bundlers and tooling

- **Vite** is universal (no Webpack, no Parcel, no Rollup direct usage).
- **TypeScript** is the default for all React designs.
- **PostCSS** for Tailwind v3 cluster; **`@tailwindcss/vite` plugin** for v4 cluster (skips PostCSS).
- **clsx + tailwind-merge** appears in #30 Foundation — the modern `cn()` helper convention from the shadcn ecosystem.
- **No Biome, no Prettier configs** visible in any brief.

## Browser APIs leveraged

- **`requestVideoFrameCallback`** with `requestAnimationFrame` fallback — used in #12 LinkFlow and #14 MicroVisuals for the Boomerang frame-capture pattern. Modern API (Chrome 83+, Safari 15.4+, Firefox behind flag).
- **`mask-composite: exclude` / `-webkit-mask-composite: xor`** — universal in the `.liquid-glass::before` recipe. Modern CSS, broadly supported.
- **`backdrop-filter: blur(Xpx)`** — used in every glass surface. Sometimes paired with `background-blend-mode: luminosity`.
- **CSS `@property` declarations** — only in #23 Animated Gradient CTA (15 `@property` declarations for the animated gradient blobs). Chromium 85+, Firefox 128+, Safari 16.4+.
- **CSS scroll-driven animations (`animation-timeline: view()`)** — not used anywhere in the batch. Designs reach for `useScroll + useTransform` from framer-motion instead.
- **View Transitions API** — not used anywhere. No `view-transition-name` declarations.
- **`mix-blend-mode`** — used aggressively in #17 Orbis (`exclusion` on cursive overlays), #20 Max Reed (`soft-light` on noise overlays), #13 NHM (`lighten` on chapter images), #50 Neuralyn (`luminosity` on dashboard image).
- **`object-position` per-breakpoint** — `object-[65%] lg:object-center` (#24 RIVR) — responsive image/video framing.
- **`100dvh` (dynamic viewport height)** — only in #11 VaultShield mobile sheet. Most designs still use `100vh` despite the mobile-Safari URL-bar issue.

## Performance anti-patterns shared across the batch

- **No `prefers-reduced-motion` handling** in 90% of briefs. Specs ship motion (entrance staggers, video loops, scroll-driven reveals, hover scales) without media-query guards. Production-grade builds would add `@media (prefers-reduced-motion: reduce)` blocks for every animation.
- **No `:focus-visible` rings** specified in 95% of briefs. Keyboard users get default browser focus rings.
- **Google Fonts CDN roundtrip** in most designs. Modern best (self-hosted woff2 with `font-display: swap` and preload) is missing.
- **No video posters** in 80% of briefs. Slow connections see a blank canvas until the video buffers.
- **Unused font weights loaded** — #12 LinkFlow loads Inter 300-700 but uses zero of them. #14 MicroVisuals loads Dirtyline 36 Days of Type but never renders it. #10 SkyElite loads Inter 700 but never uses it. Dead payload across the batch.
- **No image optimization on Cloudinary URLs** — #13 NHM serves direct PNG/MP4 without `f_auto,q_auto` transforms. Page-weight risk.
- **CORS dependency on canvas-from-video patterns** — Boomerang (#12, #14) requires `Access-Control-Allow-Origin: *` on the video origin. Silent failure if the header is missing.
- **`overflow: hidden` everywhere** — single-section heroes commit to `h-screen overflow-hidden`, no scroll at all. Works for prototypes; doesn't scale to real products.

## Recipes worth lifting wholesale

### The HLS Mux + hls.js setup
```js
useEffect(() => {
  const video = videoRef.current;
  if (!video) return;
  const url = 'https://stream.mux.com/<playback-id>.m3u8';
  if (Hls.isSupported()) {
    const hls = new Hls({ enableWorker: false }); // disable for sandbox compat
    hls.loadSource(url);
    hls.attachMedia(video);
    return () => hls.destroy(); // important — prevents memory leak
  } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
    video.src = url;
  }
}, []);
```
~12 lines, works everywhere. The `enableWorker: false` flag is specifically for StackBlitz / CodeSandbox / Claude artifacts.

### The `useVideoFadeLoop` hook
```js
function useVideoFadeLoop(videoRef, { fadeMs = 500, leadTime = 0.55, resetDelay = 100 } = {}) {
  useEffect(() => {
    const video = videoRef.current;
    if (!video) return;
    const fadingOutRef = { current: false };
    let raf = null;
    const fade = (target, startTime) => {
      const now = performance.now();
      const progress = Math.min(1, (now - startTime) / fadeMs);
      const current = parseFloat(video.style.opacity || '0');
      video.style.opacity = String(current + (target - current) * progress);
      if (progress < 1) raf = requestAnimationFrame(() => fade(target, startTime));
    };
    const onTimeUpdate = () => {
      if (!fadingOutRef.current && video.duration - video.currentTime <= leadTime) {
        fadingOutRef.current = true;
        cancelAnimationFrame(raf);
        raf = requestAnimationFrame(() => fade(0, performance.now()));
      }
    };
    const onEnded = () => {
      video.style.opacity = '0';
      setTimeout(() => {
        video.currentTime = 0;
        video.play();
        fadingOutRef.current = false;
        cancelAnimationFrame(raf);
        raf = requestAnimationFrame(() => fade(1, performance.now()));
      }, resetDelay);
    };
    video.addEventListener('timeupdate', onTimeUpdate);
    video.addEventListener('ended', onEnded);
    return () => {
      video.removeEventListener('timeupdate', onTimeUpdate);
      video.removeEventListener('ended', onEnded);
      cancelAnimationFrame(raf);
    };
  }, [videoRef]);
}
```
The seamless-loop video pattern. ~40 lines. Hides the cut by fading out 0.55s before end, restarting on `ended` with a 100ms blackout. Better than the native `loop` attribute for any video where the source isn't perfectly cut.

### The shadcn HSL-triple pattern
```css
:root {
  --background: 0 0% 0%;       /* note: no hsl() wrap */
  --foreground: 0 0% 100%;
  --muted-foreground: 0 0% 65%;
}
```
```ts
// tailwind.config.ts
theme: {
  extend: {
    colors: {
      background: 'hsl(var(--background))',
      foreground: 'hsl(var(--foreground))',
    }
  }
}
```
Enables `bg-background/45` opacity composition. Standard shadcn convention.

## Stack recommendations distilled

If building a new design in this family from scratch:

- **Default to Vite + React 18 + TypeScript + Tailwind 3.4** for compatibility with the motionsites cluster. Switch to React 19 + Vite 6 + Tailwind 4 only when the design needs Tailwind v4 features (container queries, `@theme` semantic tokens, the new color palette).
- **Use Instrument Serif + Inter** as the default font pair. Italic axis on Instrument Serif (`ital@0;1`) is critical for the per-design italic emphasis trick.
- **Use `motion/react`** (not `framer-motion`) for new code — it's the future-supported name.
- **Use `@fontsource/*` for self-hosting** instead of Google Fonts CDN. Same fonts, no roundtrip.
- **Lift the `.liquid-glass` + `.liquid-glass-strong` CSS pair verbatim** if the design needs glass surfaces. Don't reinvent the rim-light gradient.
- **Lift the `useVideoFadeLoop` hook verbatim** for any seamless looping video background.
- **Always add `prefers-reduced-motion` handling** — the batch's biggest collective gap. Wrap any animation in `useReducedMotion()` checks (framer-motion provides this) and gate CSS animations behind `@media (prefers-reduced-motion: reduce)`.
- **Always add `:focus-visible` rings** — the second-biggest collective gap.
- **Always set a video `poster`** to a real first-frame screenshot.

## Open questions about the stack

- Why does the motionsites cluster avoid Next.js entirely? Vite-SPA-only suggests the generator pipeline doesn't support SSR.
- Is `motion/react` actually different at runtime from `framer-motion`, or just a renamed package? (It's the same library, but the cluster split tracks it precisely.)
- The `db.onlinewebfonts.com` rehost service is licensing-questionable but it appears in 6+ designs. Either the batch authors don't care about licensing, or they assume the demos won't ship as-is.
- Why hybrid HLS + MP4 video hosting in #26 Mindloop only? Cost-engineering (ambient on cheap MP4, hero on premium HLS) is a sophisticated move that no other design replicates.

---

*Captured: 2026-05-29. The stack tells are sharp enough that scanning any new brief for `d8j0ntlcm91z4.cloudfront.net`, `.liquid-glass`, `framer-motion` vs `motion/react`, `@import "tailwindcss"` vs `@tailwind base`, and `hf_*` video filenames will sort it into one of the three clusters within 30 seconds. The motionsites cluster is the dominant gravity well; everything else is an outlier with one or two confirmed members.*
