# Motion Vocabulary
**Drawn from:** 55 design notes (01 cinematic-space-travel through 55 nexto-404)
**Last updated:** 2026-05-28

The batch's motion is dominated by two house easing curves and a small set of choreographed primitives. The motionsites.ai lineage uses Framer Motion (now `motion`/`motion/react`) for most entrance choreography, but multiple designs deliberately opt out — using pure CSS transitions, vanilla rAF loops, or no motion at all. The pattern split tracks page scope: single-section sites use CSS keyframes, multi-section editorials reach for `useInView` + `whileInView`, and the most engineered designs (LinkFlow #12 boomerang, NHM #13 sand transition) hand-roll their own systems.

## Patterns observed

### Pattern: House easing curves — two cubic-beziers do 80% of the work
**Used by:** Confirmed in 15+ designs.
- **`cubic-bezier(0.16, 1, 0.3, 1)`** ("decisive arrival" / out-expo-ish): #03 Prisma, #06 Asme v1, #13 NHM (wordmark + CTA), #31 SENTINEL AI (fade-up keyframe), Mindloop, others.
- **`cubic-bezier(0.22, 1, 0.36, 1)`** ("modern UI ease-out-quart"): #03 Prisma (card entrance), #11 VaultShield (fadeUp variant), #12 LinkFlow (mobile drawer), #18 Bold Vision (every animation — single-curve discipline), Asme v3, others.
- Emil Kowalski's drawer curve `cubic-bezier(0.32, 0.72, 0, 1)` — referenced in motion foundations doc, used in #15 Axion Studio mobile sheet.

**When it works:** A single curve across the page makes the motion feel authored by one hand. Bold Vision #18 uses `[0.22, 1, 0.36, 1]` on every animation — the page feels like one character moving rather than multiple effects firing.
**When it fails:** Mixing 3+ curves dilutes the motion identity. Defaulting to Tailwind's `ease` (cubic-bezier(0.25, 0.1, 0.25, 1)) on everything reads as generic.

**Decision rule:** Pick ONE curve for the page, use it for hover + entrance + sheet animations. Use the other only when narrative duration demands a different feel.

---

### Pattern: useVideoFadeLoop hook
**Used by:** #04 Aethera, #09 Asme v2, #16 Asme v3. **Three batch confirmations of identical recipe.**
**Recipe:**
- Custom `requestAnimationFrame` loop reads `video.currentTime` and `video.duration` on every frame.
- 500ms fade-in on load: `opacity = currentTime / 0.5` while `currentTime < 0.5s`.
- 500ms fade-out preempt: triggered when `duration - currentTime <= 0.55s` (the lead time).
- On `ended` event: opacity → 0, `setTimeout(100ms)`, then `currentTime = 0`, call `play()`, fade back in.
- `cancelAnimationFrame` on cleanup; new fades resume from current opacity, not snap back to 0/1.
- **Critical:** native HTML5 `loop` attribute is OFF — looping reimplemented manually so the 100ms blackout hides the seam.

**Config knobs:** `fadeDurationMs: 500`, `leadTimeS: 0.55`, `resetDelayMs: 100`.

**When it works:** Any single-video hero where the loop seam is visible at the cut. The 100ms dead-zone makes the eye expect a brief darkening, not a stutter.
**When it fails:** When the source video already loops seamlessly (SkyElite #10 uses native `loop` and trusts the source). When you can afford HLS streaming (which provides gapless loops at the cost of ~30KB hls.js library — Asme v1 #06 chose this path).

---

### Pattern: BoomerangVideoBg component
**Used by:** #12 LinkFlow, #14 MicroVisuals. **Two batch confirmations of identical implementation.**
**Recipe:**
**Phase 1 — Capture (one play-through):**
1. Video plays once with `crossOrigin="anonymous"`, `muted`, `playsInline`.
2. Every frame captured into `HTMLCanvasElement[]` via `requestVideoFrameCallback` (with `requestAnimationFrame` fallback).
3. `MAX_WIDTH = 960` cap; `scale = Math.min(1, 960 / videoWidth)`.
4. Deduplication: skip writes when `video.currentTime === lastTime`.
5. On `ended`: store frames, `setFramesReady(true)`.

**Phase 2 — Playback (30fps, ping-pong, infinite):**
1. Video element hidden via `display: none`, canvas shown.
2. rAF loop walks index 0 → length-1 → 0 → length-1 at 30fps (`interval = 1000/30`).
3. At boundary, `direction` flips.
4. Canvas draws `frames[index]` — no interpolation.

**The palindrome trick:** the eye never sees a "restart at frame 0" cut — playback reverses to frame N-2, N-3, etc. The boundary transition is two identical-looking frames (the video's last frame replayed once), so direction reversal is imperceptible.

**Memory cost:** 960×540×4 bytes per frame × 30fps × 30s source = ~300MB canvas memory. Use `MAX_WIDTH` to tune.
**CORS dependency:** CloudFront must serve `Access-Control-Allow-Origin: *`. Without it, the canvas taints silently and the entire boomerang fails.
**Works only for direction-agnostic content:** walking, water flow, smoke (one-directional motion) look wrong reversed. Source video must be abstract/ambient.

**When it works:** When you need genuinely seam-free looping and direction-reversal isn't perceptible.
**When it fails:** On low-RAM devices (300MB canvas memory risks tab eviction). On directional video content. Without CORS.

---

### Pattern: fade-rise CSS animation (3-class delay stagger)
**Used by:** #04 Aethera, #05 Velorah, #09 Asme v2 (entrance choreography).
**Recipe:**
```css
@keyframes fade-rise {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}
.animate-fade-rise { animation: fade-rise 0.8s ease-out both; }
.animate-fade-rise-delay { animation: fade-rise 0.8s ease-out 0.2s both; }
.animate-fade-rise-delay-2 { animation: fade-rise 0.8s ease-out 0.4s both; }
```
- 24px translate distance (Velorah) or 20px (Aethera).
- 0.8s duration is slow for UI — appropriate because this is **narrative motion** (curtain-up), not stateful.
- Three classes for stagger: 0 / 0.2s / 0.4s delays cover headline, description, CTA.
- ease-out arrives and settles, doesn't bounce.

**When it works:** Single-section hero with a 3-element vertical stack. Cumulative 1.2s reveal is the right window for "page assembling itself."
**When it fails:** On multi-section pages — use `whileInView` instead.

---

### Pattern: blurFadeUp (opacity + filter blur + translateY)
**Used by:** #19 Cinematic Streaming (11-element ladder, 0ms to 900ms delays), #31 SENTINEL AI (`fade-up` keyframe with same three-property recipe).
**Recipe:**
```css
@keyframes blurFadeUp {
  from { opacity: 0; filter: blur(20px); transform: translateY(40px); }
  to { opacity: 1; filter: blur(0); transform: translateY(0); }
}
animation: blurFadeUp 1s ease-out forwards;
```
The blur(20px → 0) is the differentiator. Normal fade-rise = opacity + translate. blurFadeUp adds filter:blur for *photographic depth-of-field* — content "focuses in" rather than just "appears."

**Performance note:** `filter: blur()` is NOT GPU-composited the same way transform/opacity are. Triggers paint per frame. Acceptable on small content elements, expensive on large.

**Cinematic Streaming's 11-element ladder:** logo 0ms / nav 100-300ms / metadata 300ms / hero text 400-700ms / prev-next 800-900ms. Two parallel staggers (top nav + bottom hero) overlapping by 100ms feels like one event.

**When it works:** Cinematic / film / luxury heroes where the entrance should feel theatrical (1-2s total) rather than functional (300-600ms SaaS).
**When it fails:** On performance-strict builds (blur paint cost). Without `prefers-reduced-motion` fallback — the 20px blur is exactly the kind of motion that triggers vestibular discomfort.

---

### Pattern: Char-by-char animations
**Used by:** #03 Prisma (AnimatedLetter — scroll-linked opacity per character), #08 Vex Ventures (entrance char-stagger — 30ms per char, 500ms duration), #26 Mindloop (word-by-word scroll reveal).
**Recipes:**
- **Vex Ventures' vanilla recipe** (~40 lines, zero deps): split string into chars → inline-block spans → React `useState` + `setTimeout` toggle visibility → CSS `transition: opacity 500ms, transform 500ms`. Per-char delay = `(lineIndex * lineLength * charDelay) + (charIndex * charDelay)` where `charDelay = 30ms`. `&nbsp;` for spaces to preserve tracking.
- **Prisma's AnimatedLetter** (Framer Motion): per-character `useTransform` mapping scroll progress to `[0.2, 1]` opacity. Math: `charProgress = i / total`, range `[charProgress - 0.1, charProgress + 0.05]`. Produces "page reading itself" effect tied to scroll.

**When it works:** Hero headlines where the entrance choreography *is* the first scene. Scroll-driven on body paragraphs that want narrative reveal.
**When it fails:** On more than ~30 chars (animation feels endless). Without `prefers-reduced-motion` fallback.

---

### Pattern: Scroll-driven word reveal
**Used by:** #26 Mindloop (mission section — words split into spans, each gets opacity 0.15→1 as it crosses scroll threshold, plus anchor words get full white while connective tissue stays muted gray), #49 Neuralyn (testimonial section — each word gets two simultaneous transitions: opacity 0.2→1 AND color hsl(0 0% 35%)→hsl(0 0% 100%) across its scroll slice via `useTransform`).
**Recipe:**
- Wrap testimonial / mission sentence in container with `useScroll({ target: containerRef, offset: ["start end", "end center"] })`.
- Split sentence into `<motion.span>` words.
- Each word gets `useTransform(scrollYProgress, [i/total, (i+1)/total], [0.2, 1])` for opacity.
- Optionally also `useTransform` for color.
- Pre-trigger via offset configuration (read-along mapping).

**Neuralyn's offset semantics:**
- `["start start", "end start"]` — hero exit (run animation while section is exiting upward).
- `["start end", "end center"]` — testimonial entry (run animation while section is entering/settling).

**When it works:** Mission statements, testimonials, manifesto paragraphs that the visitor *should read at scroll-pace*. Teleprompter-style read-along.
**When it fails:** On long-form body (visitor wants to scan, not be read to). Without reduced-motion fallback (scroll-tied opacity can trigger vestibular issues).

---

### Pattern: Sticky-stacking project cards
**Used by:** #07 Jack — 3D Creator.
**Recipe:**
```jsx
// per-card formula:
const targetScale = 1 - (totalCards - 1 - index) * 0.03;
// per-card offset:
top: `${index * 28}px`;
// per-card container:
className="h-[85vh]"
// per-card position:
className="sticky top-24 md:top-32"
```
Each card pins for ~85vh of scroll. As you scroll into the next card, the previous card scales down by 0.03 and nests beneath. Cards visibly stack with 28px peeks.

**When it works:** Portfolio sections with 3-5 project cards. Pure scroll storytelling.
**When it fails:** With more than ~5 cards (scroll feels endless). Without reduced-motion fallback.

---

### Pattern: Scroll-driven horizontal marquee
**Used by:** #07 Jack (21 GIFs in two opposite-direction rows, scroll-controlled via passive scroll listener).
**Recipe:**
```js
// passive scroll listener
const offset = (window.scrollY - sectionTop + window.innerHeight) * 0.3;
// row 1
translateX(offset - 200)
// row 2 (opposite direction)
translateX(-(offset - 200))
// willChange: 'transform'
```
Multiplied by 0.3 to keep speed approachable. Deterministic per scroll position (scrub back and forth works). Compare to typical CSS `@keyframes marquee` which runs at fixed speed regardless of scroll.

**When it works:** When the marquee content *is* what the visitor scrolls through. Project portfolios, logo walls that respond to scroll.
**When it fails:** When `willChange: 'transform'` is left always-on (memory cost). Without speed-delta between rows (mechanical-feeling).

---

### Pattern: CSS @property animated gradients
**Used by:** #23 Animated Gradient CTA — 5 blobs + 5 size animations + 15 `@property` declarations.
**Recipe:**
- Declare each custom property with explicit syntax: `@property --x1 { syntax: '<percentage>'; inherits: false; initial-value: 10%; }`.
- Browser can now interpolate the percentage value frame-by-frame.
- Animate `var(--x1)`, `var(--y1)`, `var(--size1)` via `@keyframes`.
- **Coprime durations** so phase alignment doesn't reveal mechanical looping: 3.0 / 3.5 / 3.8 / 4.0 / 4.2 / 4.6 / 5.0 / 5.5 / 6.0 / 6.5 seconds.
- `ease-in-out` on every animation.
- `prefers-reduced-motion: reduce { animation: none }` — at-rest values render as static gradient.

**When it works:** Consumer-friendly CTA cards, hero gradients that need to feel alive without committing to video.
**When it fails:** Without `prefers-reduced-motion`. With same-duration animations (mechanical).

---

### Pattern: Shiny gradient sweep on text
**Used by:** #22 Aura ("Revitalized" — 7-stop gradient sweeps via `background-position` animation, 6s linear infinite), #38 DesignPro ("Product Leader." — `#64CEFB` light sky blue sweep), often combined with `background-clip: text`.
**Recipe (Aura):**
```css
@keyframes shiny {
  0%   { background-position: -200% center; }
  100% { background-position:  200% center; }
}
.shiny-text {
  background: linear-gradient(90deg, #091020 0%, #0B2551 12.5%, #A4F4FD 32.5%, #00d2ff 50%, #0B2551 67.5%, #091020 87.5%, #091020 100%);
  background-size: 200% auto;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: shiny 6s linear infinite;
}
```
Key tuning: 200% background-size + 200% start/end positions means the bright peak fully exits the right edge before returning. Linear easing (not eased) for ambient looping. 7 stops with asymmetric bookends — bright cyan peak at 50%, dark navy bookends.

**Aura's noise overlay:** SVG `feTurbulence` baseFrequency 0.9 at `multiply` blend, alpha 0.35, overlaid on top of the gradient — reduces banding on cheap displays, adds film grain warmth.

**When it works:** Single-word hero accent. Continuous ambient effect that announces brand without competing for attention.
**When it fails:** On more than ~12 characters (sweep feels slow). On bright backgrounds where the dark gradient hexes don't contrast.

---

### Pattern: Mouse-tracking Magnet
**Used by:** #07 Jack (portrait), #14 MicroVisuals (GSAP-driven video parallax — adjacent technique).
**Recipe (Magnet):**
```js
// on mousemove within radius:
const dist = sqrt((mx - cx)² + (my - cy)²);
if (dist < padding) {
  translate3d(dx/strength, dy/strength, 0);
}
// Settings: padding=150, strength=3
// Transitions: transform 0.3s ease-out (active), 0.6s ease-in-out (release)
// willChange: 'transform'
```
Tracks mouse position vs element center, applies translation when cursor is within `padding` (150px) of the element edge. The release transition is slower (0.6s vs 0.3s) so the element catches up to mouse fast and releases slow.

**MicroVisuals' GSAP parallax variant:** Same idea but applied to fixed video background. `strength: 20` pixels, `lerp: 0.06` smoothing. Background drifts toward cursor with elastic delay.

**When it works:** Hero portraits, CTAs, marquee items where you want passive ambient response to cursor. Signals "this is a system, not a recording."
**When it fails:** On touch devices (no cursor). Without `willChange: 'transform'` (jank). With strength > 5 or radius > 200 (clownish).

---

### Pattern: SandTransitionImage (custom SVG filter chain)
**Used by:** #13 NHM Natural History Museum (chapter image dissolves).
**Recipe:**
Five-stage SVG filter chain driven by rAF over 900ms:
1. `feTurbulence` — fractal noise, baseFrequency 1.8 (high = sand-grain size), numOctaves 4.
2. `feDisplacementMap` — scale ramps 0 → 150 with progress; displaces pixels.
3. `feOffset` — dy ramps to -80 (enter — particles rise) or 120 (exit — particles fall); dx ±30.
4. `feGaussianBlur` — stdDeviation ramps to 6px; softens displaced pixels.
5. `feColorMatrix` — opacity ramps via `1 - progress * 1.2` (disappears at progress ≈ 0.83, gives an empty frame before next image arrives).

Dual easings:
- **Entering**: `1 - Math.pow(1 - t, 4)` — quartic ease-out. Image assembles fast, settles slowly.
- **Exiting**: `Math.pow(t, 3)` — cubic ease-in. Image dissolves slow, then disperses fast.

`usePresence()` from `motion/react` branches the easing function. `crossOrigin="anonymous"` required for canvas filter ops on cross-origin images.

**When it works:** Image-swap contexts where a crossfade feels too cheap. Editorial / museum / archive contexts where "the past reconstructs from fragments" is the metaphor.
**When it fails:** Without CORS. On performance-strict builds. With abstract/non-photographic content.

---

### Pattern: Clip-reveal word stack
**Used by:** #18 Bold Vision (three-word heading "FEARLESS / VISION / DELIVERED" — each word in `overflow-hidden` parent, animates `y: "110%" → 0`).
**Recipe:**
```jsx
<span className="block overflow-hidden">
  <motion.span
    initial={{ y: "110%" }}
    animate={{ y: 0 }}
    transition={{ duration: 0.7, delay: 0.4 + i * 0.14, ease: [0.22, 1, 0.36, 1] }}>
    {word}
  </motion.span>
</span>
```
Each word starts at `y: 110%` (110% of its own height below the wrapper's bottom edge — fully clipped, invisible). 110% (not 100%) over-corrects to ensure descenders don't peek above the bottom edge before animation begins.
- 0.14s stagger between words — agency-reveal tempo (faster reads simultaneous, slower reads sluggish).
- Total reveal: 0.4s → 1.38s.

**Why clip-reveal beats fade-up for headlines:** A faded letter is visible the entire time it animates (just at lower alpha). A clip-revealed letter is *not there*, then *is there*. Binary perception. Reads as performed, not animated.

**When it works:** Hero headlines where the brand voice is "FEARLESS / DELIVERED" — words that should *arrive*, not *appear*.
**When it fails:** On more than ~5 words (eye loses the pattern). On lowercase prose copy.

---

### Pattern: Text-roll hover
**Used by:** #15 Axion Studio — every CTA on the page.
**Recipe:**
```jsx
<button className="group">
  <div className="overflow-hidden h-[20px]">
    <div className="transition-transform duration-500 ease-[cubic-bezier(0.25,0.1,0.25,1)]
                    group-hover:-translate-y-1/2">
      <span className="block h-[20px]">Start a project</span>
      <span className="block h-[20px]">Start a project</span>
    </div>
  </div>
</button>
```
Duplicated text in a `flex-col` parent with `overflow-hidden h-[20px]`. On `group-hover` the inner stack translates `-50%` on Y, swapping the visible copy with its identical twin. Word *rolls* into place.

**When it works:** Agency portfolio / studio CTAs where you want a deliberate hover (more considered than color shift, less aggressive than scale).
**When it fails:** When overused across many elements on a single page — the gesture loses uniqueness.

---

### Pattern: Hover-expanding pill (circle → wide pill)
**Used by:** #15 Axion Studio — case-study cards.
**Recipe:**
- Initial: `h-9 w-9 rounded-full` absolute-positioned circle in bottom-left of card.
- On `group-hover`: width transitions to `w-[148px]` (or `w-[168px]` per-card), revealing the label.
- `duration-300 ease-in-out` width transition.
- Inner text fades from `opacity-0` to `opacity-100` with `delay-100` so width animation leads.
- Icon rotates from `-rotate-45` to `rotate-0` simultaneously.

**When it works:** Card grids where you want space-efficient action affordances. The expansion teaches the user "this is interactive."
**When it fails:** When the icon-only state doesn't telegraph interactivity (`cursor-pointer` is critical).

---

### Pattern: Collapsing email capture
**Used by:** #06 Asme v1.
**Recipe:**
- Primary CTA *is* the email field.
- Click "Get early access" → button morphs into a pill-shaped form via `AnimatePresence mode="wait"` + scale 0.95→1.
- Submit → ArrowRight icon swaps to Check via same AnimatePresence pattern.
- 4-second auto-reset back to the button state.
- Typewriter placeholder runs at 60ms/char with two-state machine (pre-submit / post-submit).

**When it works:** Single-CTA waitlist pages where the form is the only conversion.
**When it fails:** The 4s auto-reset is unusual — a user who submits and looks away may wonder if it worked. Persistent success state is the safer pattern.

---

### Pattern: Typewriter placeholder
**Used by:** #06 Asme v1 (form placeholder), #43 dot. (TypingMessages component overlaying Nokia phone screen in video).
**Recipe:**
- Hand-coded `useEffect` + `setInterval` at 60ms per character.
- State machine with multiple strings (Asme v1: "Enter Your Email Here For Early Access" / "You Will Receive Notifications By Email"; dot.: "Are you here?" / "Yes, I am." / "Speak soon.").
- Pause 2s with blinking `motion.span` cursor, then delete char-by-char, then type next.

**When it works:** Idle empty states that need a heartbeat. dot.'s overlay on a phone screen in a video is the most creative use — "the product enacts itself in the hero."
**When it fails:** Without `prefers-reduced-motion` fallback. With brittle alignment (dot.'s typing requires the video file to never change).

---

## Anti-patterns observed

### `willChange` permanently set on idle elements
**Found in:** #02 TOONHUB, #07 Jack — `willChange: 'transform, filter, opacity'` on always-on animated targets.
**Why it matters:** Foundations explicitly warn against this. `will-change` is meant to be applied just-in-time. Always-on is a sustained memory cost.
**Fix:** Apply only when an animation is about to fire; remove after.

### No `prefers-reduced-motion` fallback
**Found in:** Virtually every design in the batch. Universal accessibility gap.
**Foundations rule:** "Always respect `prefers-reduced-motion`. Not optional."
**Fix:** At minimum, freeze loops and skip entrance animations under `@media (prefers-reduced-motion: reduce)`.

### Parallax-everywhere
**Found in:** No clean offenders — most batch designs use parallax once (HAUL #37's truck) or not at all.

### Motion that delays interactive elements
**Found in:** #04 Aethera (CTA fades up at 1.1s delay — clicking before then does nothing visible).
**Fix:** Make elements clickable immediately; let only opacity/transform animate.

### 12px hover scale (too aggressive)
**Found in:** #17 Orbis purple chevron `hover:scale-110` — 10% scale is louder than foundations' recommended 1.02-1.05.
**Fix:** Default to 1.02-1.05 unless the element is tiny (then 1.08-1.10 is acceptable).

---

## Decision rules

1. **Pick ONE house easing curve.** Default to `cubic-bezier(0.16, 1, 0.3, 1)` for cinematic / brand, `cubic-bezier(0.22, 1, 0.36, 1)` for modern UI / SaaS.
2. **For video loops, default to native `loop` attribute.** Reach for `useVideoFadeLoop` only when the seam is visible. Reach for `BoomerangVideoBg` only when the content is direction-agnostic and you need invisible-cut polish.
3. **For entrance choreography, use CSS keyframes on single-section pages, `useInView` on multi-section.**
4. **For scroll-driven word reveal, use `useScroll` with offset `["start end", "end center"]` for read-along mapping.** Use `["start start", "end start"]` for exit-as-you-scroll-past.
5. **Stagger ladder values:**
   - **Asme/Aethera/Velorah cinematic:** 0 / 0.2s / 0.4s (200ms gaps, 0.8s duration each, ~1.2s total).
   - **Bold Vision agency reveal:** 0.4 / 0.54 / 0.68s (140ms gaps, 0.7s duration each, ~1.38s total).
   - **Cinematic Streaming film:** 0 / 100 / 150 / 200 / 250 / 300 / 350 / 400 / 500 / 600 / 700 / 800 / 900ms (11 elements, 1s duration, ~1.9s total).
   - **SaaS standard:** `delay = i * 0.1s`, `duration = 0.6s`.
6. **For hover, default to `scale: 1.02-1.05` + `whileTap: scale 0.96-0.98`.** Push to 1.08 only on tiny circular buttons.
7. **For drawer/sheet, use `cubic-bezier(0.32, 0.72, 0, 1)` (Emil's drawer curve).** Duration 350-500ms.
8. **For mobile nav stagger, inline `style={{ transitionDelay: \`${base + i * step}ms\` }}`** — pure CSS, no Framer Motion needed (LinkFlow #12's recipe).
9. **`useInView { once: true, margin: "-100px" }`** — pre-trigger entrance 100px before section enters viewport so animation has settled when user is reading.
10. **`prefers-reduced-motion: reduce`** — at minimum: `{ animation: none; opacity: 1; filter: none; transform: none; }`.

---

## Open questions / gaps

- Why is `useVideoFadeLoop` shared across designs but `BoomerangVideoBg` only used twice? Boomerang is more expensive (memory) but produces invisible loops. Is the cost the limiting factor or the source-video constraint (direction-agnostic only)?
- The motion-foundations doc recommends `linear()` for spring-without-JS. No batch design uses it. Is the browser support gap (Safari) holding it back, or has the tool not caught up?
- View Transitions API is the "modern default" per foundations. **Zero batch designs use it.** Why? Tool not generating it, or designs are pre-View-Transitions?
- The pattern of "1-second narrative duration" appears throughout cinematic heroes — is this the new SaaS-baseline expectation, or is it specific to brand pages?
- Multiple designs use `cancelAnimationFrame` correctly in `useEffect` cleanup. Is this universal in the batch's React patterns, or are some designs leaking rAF loops?
- Why does no batch design use spring physics for drawers/dismissals? Foundations recommend springs for direct-manipulation. Is this a tool limitation or aesthetic choice?
