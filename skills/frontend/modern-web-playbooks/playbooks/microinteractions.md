# Microinteractions
**Drawn from:** 55 design notes (01 cinematic-space-travel through 55 nexto-404)
**Last updated:** 2026-05-28

The micro-feedback layer of the batch. Most hover, tap, and focus behaviors converge on a small set of values — `scale: 1.02-1.05` on hover, `scale: 0.96-0.98` on tap, 200ms `transition-colors` for color shifts. The interesting work is in the *compound* gestures: text-roll, pill-expansion, collapsing forms, character-stagger entrances. The pattern split tracks intent: marketing pages use confident-restrained microinteractions; portfolio sites use playful ones; product UIs use precise functional feedback.

## Patterns observed

### Pattern: Hover scale 1.02-1.05
**Used by:** Vast majority of CTAs across the batch. Confirmed in #11 VaultShield (1.04 with brightness 1.1), #16 Asme v3 (`whileHover scale 1.05`), #18 Bold Vision, #29 CodeNest, #33 Bloom, and ~30+ other designs.
**Recipe:**
- Framer Motion: `whileHover={{ scale: 1.02 }}` or `whileHover={{ scale: 1.04, filter: 'brightness(1.1)' }}`.
- Tailwind: `hover:scale-105 transition-transform`.
- Default duration ~150-200ms via Tailwind's `transition-transform` or Motion's spring defaults.

**The 1.02 vs 1.05 split:**
- **1.02-1.03** for editorial / cinematic / luxury contexts (Aethera #04, Velorah #05 — subtle).
- **1.04-1.05** for SaaS / product / consumer contexts (VaultShield #11, Bloom #33 — confident).
- **1.08-1.10** for tiny circular buttons (Orbis #17 chevron, TOONHUB #02 nav arrows — necessary because small targets need visible feedback).

**When it works:** When applied consistently across all interactive elements. The user learns "this is hoverable" via one repeated gesture.
**When it fails:** Above 1.1 reads clownish (foundations rule). Inconsistent values across the same page break the gesture's signal.

---

### Pattern: whileTap scale 0.96-0.98
**Used by:** #11 VaultShield (`whileTap scale 0.96` — proper press feedback), #16 Asme v3 (`whileTap scale 0.95` on Explore More), #33 Bloom (`active:scale-95` on CTA), and across the batch wherever Framer Motion buttons appear.
**Recipe:**
- Framer Motion: `whileTap={{ scale: 0.96 }}`.
- Tailwind: `active:scale-[0.97]` or `active:scale-95`.
- Faster than hover (~80-120ms).

**The 0.96 vs 0.98 split:**
- **0.95-0.96** for confident-press feedback (most batch).
- **0.97-0.98** for subtle confirmation (Aether-tier sites).

**When it works:** As a counterpart to hover scale. The compression-on-press makes the button *feel* like it's been pressed, not just clicked.
**When it fails:** Without paired hover (the compression has no setup). On non-button elements (cards, links).

---

### Pattern: Text-roll hover (duplicate text in overflow-hidden)
**Used by:** #15 Axion Studio — every CTA on the page.
**Recipe:**
```jsx
<button className="group">
  <div className="overflow-hidden h-[20px]">
    <div className="transition-transform duration-500 ease-[cubic-bezier(0.25,0.1,0.25,1)] group-hover:-translate-y-1/2">
      <span className="block h-[20px]">Start a project</span>
      <span className="block h-[20px]">Start a project</span>
    </div>
  </div>
</button>
```
The duplicated text in `flex-col` parent with `overflow-hidden h-[20px]` produces a "rolling word" effect on hover. Inner stack translates `-50%` Y, swapping the visible copy with its identical twin.
- 500ms duration with house easing.
- `block h-[20px]` on each child enforces precise alignment.

**When it works:** Agency / studio CTAs where the hover should feel *deliberate*, more considered than color shift but less aggressive than scale.
**When it fails:** When overused across the whole page (gesture loses uniqueness). On variable-height text (must enforce fixed line-height).

---

### Pattern: Hover-expanding pill button (circle → wide pill)
**Used by:** #15 Axion Studio — case-study card CTAs.
**Recipe:**
- Initial state: `h-9 w-9 rounded-full` absolutely-positioned circle in bottom-left of card.
- On `group-hover`: width transitions to fixed pixel width (`w-[148px]` or `w-[168px]` per-card) over `duration-300 ease-in-out`.
- Inner text fades in with `opacity-0 → opacity-100` + `delay-100ms` so width animation leads (~30% complete) before text appears.
- Icon (chain link, ArrowRight) rotates from `-rotate-45` to `rotate-0` simultaneously.

**When it works:** Card-grid CTAs that need to telegraph "interactive" without dominating the static state. The expansion teaches the user the action.
**When it fails:** Without `cursor-pointer` on parent (the icon-only state may not telegraph interactivity). On touch devices (no hover trigger).

---

### Pattern: Collapsing email capture (button → form → submit → reset)
**Used by:** #06 Asme v1.
**Recipe:**
- Primary CTA *is* the email field.
- Click "Get early access" → button morphs to pill-shaped form via `AnimatePresence mode="wait"` + scale 0.95→1.
- Submit → ArrowRight icon swaps to Check via same AnimatePresence pattern.
- 4-second auto-reset back to button state.
- Typewriter placeholder runs on the empty form.

**When it works:** Single-CTA waitlist / pre-launch pages where the form is the only conversion. The morph is more sophisticated than a separate "Sign up" link + modal.
**When it fails:** The 4s auto-reset is opinionated — a user who submits and looks away may wonder if it worked. Persistent success state is safer for real builds. The pattern also assumes the form has no validation errors.

---

### Pattern: Typewriter placeholder
**Used by:** #06 Asme v1 (email form), #43 dot. (TypingMessages component overlaying Nokia phone screen).
**Recipe:**
- Hand-coded `useEffect` + `setInterval` at 60ms per character.
- Two-state machine for pre-submit / post-submit copy.
- After fully typed: pause 2s with blinking `motion.span` cursor, then delete char-by-char, then type next.
- dot.'s variant: cycles through 3 messages indefinitely ("Are you here?" → "Yes, I am." → "Speak soon."), each typed in pixel-accurate Nokia Cellphone FC Small font, positioned at `left-[48.5%] bottom-[32%]` to overlap the phone screen visible in the video.

**When it works:** Idle empty states that need a heartbeat. Best when the typed content *demonstrates the product* (dot.'s typed messages literally enact the product's one-message-a-day model).
**When it fails:** Without `prefers-reduced-motion` fallback. With brittle viewport alignment (dot.'s pixel-tuned overlap breaks if the video changes).

---

### Pattern: L-shaped corner brackets on cards
**Used by:** #50 Behind the Lens — every video tile in the photography grid.
**Recipe:**
- Four white L-shaped corner brackets at 15px inset from card edges.
- Each bracket is ~12-15px on each leg (photographic-frame aesthetic).
- Visible always — they don't appear only on hover.
- Combine with `mix-blend` or pure white (depending on card background).

**When it works:** Photography / film / camera-related editorial contexts where the viewfinder-reticle aesthetic is on-brand.
**When it fails:** On busy backgrounds where the brackets disappear. On SaaS contexts where they read as "industrial decoration."

---

### Pattern: 4-effect simultaneous hover (scale + overlay + icon + brackets)
**Used by:** #50 Behind the Lens — video tiles in photography grid.
**Recipe (all 4 fire on hover):**
1. Video scales to `1.08`.
2. `rgba(0,0,0,0.25)` overlay fades in.
3. 70px translucent circle with centered `+` icon scales from `0.7` to `1.0`.
4. Corner brackets remain visible.

All on the same easing, same duration (~300-400ms), starting at the same moment. The cumulative effect converts a static grid into a viewfinder gallery.

**When it works:** When the four properties are coordinated and tell one story. Behind the Lens's story: "tile recedes (overlay) while the focal target (+ icon) advances."
**When it fails:** When the four effects fire at different durations (chaos). Without a coordinated narrative.

---

### Pattern: + icon hover reveal
**Used by:** #13 NHM specimen card ("View Details" plus circle), #16 Asme v3 Service Cards (ArrowUpRight in glass-pill circle), #20 Max Reed (Reach Me card), #50 Behind the Lens (70px translucent + circle on tile hover).
**Recipe:**
- Circle of varying size (24-70px) holding a `+` or `ArrowUpRight` icon.
- Border, fill, or glass treatment depending on context.
- Optional hover: scale from `0.7 → 1.0`, or rotation, or color shift.

**When it works:** As a "see more / expand / open record" affordance. The diagonal arrow (ArrowUpRight) is the universal 2026 signal for "more here" or "external link" — replaced the older underline-and-blue-text convention.
**When it fails:** Without an explicit interaction (looks decorative). When inconsistent across the page (some cards have +, some have ArrowUpRight, some have nothing).

---

### Pattern: Nav-embedded CTA inside the pill
**Used by:** #01 Cinematic, #06 Asme v1, #12 LinkFlow (asymmetric `pl-6 pr-1` to hug the embedded button), #14 MicroVisuals, #16 Asme v3, others.
**Recipe:**
- Nav pill `rounded-full px-6 py-2.5` with `liquid-glass` background.
- Embedded CTA on the right edge: `bg-white text-black rounded-full px-4 py-2`.
- The CTA is *inside* the nav pill, not adjacent.
- The nav pill's right padding is reduced (`pr-1` instead of `pr-6`) to hug the CTA.

**When it works:** When the nav is the only persistent UI on the page and you want one element to carry both navigation and primary CTA. Family.co's pattern, ported across the batch.
**When it fails:** When the CTA dominates the nav (the user reads the CTA as the nav). When mobile collapses break the asymmetric padding.

---

### Pattern: Same CTA copy twice (deliberate redundancy)
**Used by:** #04 Aethera ("Begin Journey" in nav + hero), #05 Velorah (same), #08 Vex Ventures ("Start a Chat" in nav + hero, identical styling), #18 Bold Vision ("Work With Us" in nav + hero + mobile menu footer).
**Recipe:** Same label, same color, identical or near-identical styling across two or three placements. No "Sign up" vs "Get started" variation.
**Why it works:** The repetition is the cue. Users who scroll past one will click the other. The lack of variation makes the redundancy feel intentional, not accidental.
**When it fails:** When the labels differ slightly ("Sign up" vs "Get started") — reads as confusion, not redundancy. When the CTAs lead to different destinations.

---

### Pattern: Glow-without-color hover shadows
**Used by:** #14 MicroVisuals — all three CTAs use white luminance glows at varying intensity:
- Primary "Start generating": `hover:shadow-[0_0_24px_4px_rgba(255,255,255,0.25)]` — 25% white glow.
- Nav "Try it free": `hover:shadow-[0_0_16px_2px_rgba(255,255,255,0.12)]` — 12% glow.
- Secondary "See templates": `hover:shadow-[inset_0_1px_1px_rgba(255,255,255,0.2),0_0_20px_2px_rgba(255,255,255,0.07)]` — 7% glow.

**Recipe:** White luminance glow at varying opacity (25% / 12% / 7%) as CTA hierarchy. Avoids the cliché of brand-color glow on hover.
**When it works:** On dark backgrounds where pure white glow reads as ambient illumination. The opacity variance is the hierarchy.
**When it fails:** On light backgrounds (white glow disappears). With brand-color glow on the same page (mixed messaging).

---

### Pattern: Brand-tinted CTA shadow
**Used by:** #11 VaultShield (`box-shadow: 0 4px 24px rgba(115,66,226,0.28)` on the purple CTA — the shadow IS the brand color at 28% alpha).
**Recipe:** `box-shadow: 0 4px 24px rgba(<brand-rgb>, 0.28)` — colored shadow at ~25-30% alpha of the brand color.
**When it works:** Primary CTAs where the brand color is saturated and the surrounding palette is restrained. The colored shadow says "this button glows in its own color."
**When it fails:** With a neutral grey shadow (cheap-feeling). On low-saturation brand colors (washes out).

---

### Pattern: Submit icon swap (Arrow → Check)
**Used by:** #06 Asme v1 — ArrowRight → Check via AnimatePresence inside the email form.
**Recipe:**
- Inside the submit button: `<AnimatePresence mode="wait">` containing either `<ArrowRight />` or `<Check />` based on submit state.
- 0.2s scale 0.95→1 swap.
- Critically: `mode="wait"` ensures the exiting icon fully animates out before the new one animates in.

**When it works:** Form submissions where success state needs visible confirmation in <200ms. The icon swap reads as "your action was received."
**When it fails:** When success is async and the icon needs to swap back on error. Without a paired text-state change (some users miss the icon).

---

### Pattern: Compound CTA hover (multi-property)
**Used by:** #03 Prisma — "Join the lab" CTA with two-property hover:
- `hover:gap-3` (gap between label and embedded circle widens).
- `group-hover:scale-110` (embedded black circle pops).

**Recipe:** Multiple Tailwind utilities composed via `group` / `group-hover` to coordinate two simultaneous transforms.
**When it works:** Compound transforms feel richer than single-property hover. The gap-widening is the more novel half — most "compound hovers" stick to scale + color.
**When it fails:** When the two properties have different durations (chaos). When the compound feels like compensation for weak base styling.

---

### Pattern: Hamburger ↔ X morph
**Used by:** Many batch designs with mobile menus. Vercel-style swap with `rotate-180 + opacity + scale-50`.
**Recipe:**
```jsx
<button onClick={toggle}>
  <Menu className={isOpen ? "opacity-0 rotate-90 scale-50" : "opacity-100 rotate-0 scale-100"} />
  <X className={isOpen ? "opacity-100 rotate-0 scale-100" : "opacity-0 -rotate-90 scale-50"} />
</button>
```
- `transition-all duration-300` (or duration-500 for cinematic pacing).
- Counter-rotated entry/exit on each icon.

**When it works:** As the universal "menu opens / closes" affordance. The morph signals state change in 300ms.
**When it fails:** Without paired menu animation (the icon morphs but the menu pops abruptly). With `duration-500+` on UI-critical actions (feels sluggish).

---

### Pattern: Inline-icon-as-glyph headline
**Used by:** #11 VaultShield — Lucide `Zap`, `LockKeyhole`, `Fingerprint` sitting inline between words in the headline at 24px (~0.5× cap-height), `top: -2px` for baseline alignment.
**Recipe:**
- Icons at 0.5× cap-height of surrounding text.
- `top: -2px` optical adjustment.
- `vertical-align: middle`.
- Position icons *between* words, not beside — semantic relationship to surrounding text.

**When it works:** When the icons are semantically tied to the words around them ("Zap" near "Lock", "Fingerprint" near "Identity"). The visual rhythm beats illustration.
**When it fails:** With decorative icons (reads as cluttered). At sizes outside 0.4-0.6× cap-height (looks like a typo or asterisk).

---

### Pattern: Booking-window microcopy + live timezone clock
**Used by:** #15 Axion Studio.
**Recipe:**
- "Taking on projects for Q1 2026" — present-progressive, low-pressure text.
- Live clock: `setInterval(1000)` with `toLocaleTimeString('en-GB', { timeZone: 'Europe/London', hour: '2-digit', minute: '2-digit' })`.
- Both at `text-xs text-gray-600` in nav status row.

**When it works:** Service businesses (studios, consultancies, agencies) signaling availability + location without a "Hire us" button. The clock is the credibility — it announces the team is alive (live update) and where they are (timezone).
**When it fails:** Without SSR fallback (the clock flashes 00:00 on first paint). When the booking window is stale (Q1 2024 in mid-2024 reads as abandoned).

---

### Pattern: Auto-cycling tab bar with conditional overlays
**Used by:** #46 Stellar.ai — 4-tab bar (Analyse / Train / Testing / Deploy) auto-cycles every 4s, each tab swapping the in-product overlay on the video container below.
**Recipe:**
- `useState('analyse')` tab state.
- `useEffect` with `setInterval(4000)` walks through tabs.
- Each tab change crossfades the new overlay (`animate-fade-in-overlay` 0.4s opacity-only).
- Inner card slides up (`animate-slide-up-overlay` 0.5s translate(-50%, -50%)) into position.
- Click handler resets the interval (manual selection wins).

**When it works:** Self-playing demos for products with 3-5 discrete phases. The page does the work of a feature tour without ever leaving the hero.
**When it fails:** Without manual interaction (users feel hijacked). When the overlays don't visually differ enough (cycling reads as the same thing).

---

### Pattern: Auto-cycling chapter list
**Used by:** #13 NHM Natural History Museum — `setInterval(() => setActiveChapter(p => (p + 1) % 5), 3500)`.
**Recipe:**
- 3.5s interval auto-advances chapter index.
- Click handlers reset the interval (manual override).
- Active chapter triggers `SandTransitionImage` (SVG filter dissolve) on left panel.
- Active row shows `ArrowUpRight` icon.

**Variant:** active chapter starts at index `2` ("Reptiles of the Mesozoic"), not 0 — deliberate "you walked into the middle of an exhibit" feel.

**When it works:** Kiosk-mode demos baked into a homepage. The page is self-narrating; passive visitors still see all 5 chapters cycle.
**When it fails:** Without pause-on-hover (visitor clicks a chapter to read it, page advances anyway after 3.5s). Without manual control (users feel hijacked).

---

### Pattern: 4-quadrant logo / geometric brand mark
**Used by:** #41 LUMINA (4 quadrant-arches in a 256×256 SVG), #44 Halo (two interlocking rounded squares — halo metaphor), #29 CodeNest (concentric circles), #38 DesignPro (same concentric circles — likely template).
**Recipe:** Custom 256×256 SVG with geometric primitives — quadrant arches, interlocking squares, concentric circles. Used at 24-32px in lockup with wordmark.
**When it works:** When the geometric primitive carries semantic weight (Halo's interlocking squares = peg + reward; LUMINA's quadrants = compass / horizon). Reads more confident than abstract logos.
**When it fails:** When the geometry is decoration without meaning. When the wordmark dominates and the mark becomes vestigial.

---

### Pattern: Pause-icon scroll cue
**Used by:** #13 NHM Natural History Museum.
**Recipe:** `w-12 h-12 rounded-full border-gray-400` circle with **two vertical lines** (`w-[1px] h-[12px]`) inside, instead of a chevron or down-arrow.
**When it works:** Museum / exhibit / gallery contexts — reads as "the page is on pause, scroll to play." Different from every other scroll-cue.
**When it fails:** When the user doesn't recognize the pause-icon convention. On non-narrative pages.

---

### Pattern: Floating bottom navbar
**Used by:** #30 Foundation of New Digital Epoch — pill nav at `absolute bottom-10 left-1/2 -translate-x-1/2`.
**Recipe:**
- `bg-white/90 backdrop-blur-2xl` (90% white with 40px blur).
- `border border-slate-200`.
- Positioned at bottom-center of hero card.

**When it works:** When the hero composition has top-anchored content and the nav can occupy the bottom band without competing.
**When it fails:** When the nav scrolls with the page (loses persistence). On long pages where the bottom navbar disappears beneath content.

---

## Anti-patterns observed

### Bouncing CTAs to draw attention
**Found in:** No batch examples — most designs use restrained 1.02-1.05 hover scales.
**Why it matters:** Foundations call this "desperate." Avoid spring-physics CTAs on marketing pages.

### Same-property hover transitions at different durations
**Found in:** Various designs. When color transitions are 200ms but transform is 300ms, the hover feels disjointed.
**Fix:** Match durations across all hover properties unless deliberately asymmetric.

### Hover states without focus-visible counterparts
**Found in:** Virtually every design in the batch. Universal accessibility gap.
**Fix:** Every `hover:` should have an equivalent `focus-visible:` style. Keyboard users need the same feedback.

### `transition: all`
**Found in:** Some batch designs. `transition: all` cascades to every animatable property, including ones that shouldn't animate (e.g., `display`).
**Fix:** Specify properties explicitly (`transition: transform 200ms, background-color 200ms`).

### Inverted Sign Up / Login hierarchy
**Found in:** #09 Asme v2 — "Sign Up" is plain text, "Login" is the prominent glass pill. On a waitlist page where presumably no one has an account, this is backwards.
**Fix:** Make Sign Up the prominent CTA on waitlist pages. Login can be plain text or a small ghost button.

### Hover state but no cursor: pointer
**Found in:** Some custom hover affordances without explicit `cursor: pointer`. The hover changes appearance but the OS doesn't telegraph clickability.
**Fix:** Always add `cursor-pointer` to non-button hover-enabled elements.

### 4s auto-reset on success state
**Found in:** #06 Asme v1.
**Why it matters:** User submits, looks away for 4s, returns to find form reset. Wonders if it submitted at all.
**Fix:** Persistent success state, or reset only on input focus.

---

## Decision rules

1. **Default to `hover:scale-105` + `active:scale-95` for primary CTAs.** Pair Framer Motion `whileHover` with `whileTap` always.
2. **For nav links, use `transition-colors` only.** No transform, no scale — just color shift.
3. **For text-link CTAs (no button chrome), use arrow translate.** `group-hover:translate-x-0.5` on a paired ArrowRight icon — the agency-portfolio standard.
4. **Match all hover durations to 200ms unless cinematic context (500ms+).**
5. **Use `cursor-pointer` on every non-button hover-enabled element.**
6. **For mobile menu toggle, use Menu ↔ X morph with `rotate + opacity + scale-50`.** Duration 300ms standard, 500ms for cinematic pacing.
7. **For form submit confirmation, use AnimatePresence icon swap (Arrow → Check) in `mode="wait"`.** 0.2s duration. Pair with persistent text-state change.
8. **For ambient idle states, use typewriter placeholder at 60ms/char.** Always provide reduced-motion fallback.
9. **For card-grid CTAs, use hover-expanding pill (circle → wide pill).** 300ms width transition, text fade with 100ms delay.
10. **For studio / agency CTAs, use text-roll hover.** Duplicate text + `overflow-hidden h-[20px]` + `group-hover:-translate-y-1/2`.
11. **For service-business credibility, use live timezone clock + booking-window microcopy.** Two pieces of unanimated text outperform a testimonials section.
12. **For self-playing demos, use auto-cycle with manual override.** `setInterval` + `useState`, click resets the interval.

---

## Open questions / gaps

- Most batch designs lack explicit `:focus-visible` styles. Is this a tool limitation or a designer-blind-spot pattern?
- The 4-effect hover on Behind the Lens (#50) is rare in the batch. Is single-property hover (just color shift, or just scale) the new restrained default?
- Compound-property hovers (Prisma's gap-widen + scale-circle) work but are uncommon. Why?
- Are auto-cycling demos (NHM #13 chapters, Stellar.ai #46 tabs) emerging as a new "hero-as-demo" pattern, or are they isolated experiments?
- The typewriter pattern is used twice (#06, #43) but with different intent (form placeholder vs product-enactment overlay). Is the latter use scalable beyond the dot. concept?
- Nav-embedded CTA (Family.co's pattern) appears in 5+ batch designs but never in B2B SaaS (Stellar.ai #46, Nexora #40). Is the pattern restricted to brand / portfolio / consumer contexts?
- No batch design uses spring physics for any microinteraction. Foundations recommend springs for direct-manipulation UIs. Why the universal opt-out?
