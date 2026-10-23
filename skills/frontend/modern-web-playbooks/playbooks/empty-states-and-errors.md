# Empty States & Errors

**Captured:** 2026-05-29
**Scope:** What 55 designs taught us about error / empty / 404 / loading surfaces.
**Anchor:** #55 Nexto 404 — the single explicit error-state design in the batch. Everything else here is observed-by-absence.

## The batch's overall coverage

Across 55 design briefs, **exactly one** is a true error-state page: #55 Nexto 404. The other 54 are heroes, landings, feature walls, pricing blocks, footers, dashboards, and signup forms. This is the **defining structural fact** about this playbook — the batch optimizes heavily for the *front of the funnel* (impressions, conversions) and treats *the surfaces visitors didn't mean to land on* as out of scope.

That gap is the playbook's first warning: most batch-family designs ship without an error state. Productionization requires building these.

## The Nexto 404 anchor — full recipe

The design's emotional precision is the part to lift wholesale. The visitor didn't mean to be here, so:

1. **Single viewport, no scroll** — `html, body, #root { height: 100vh; overflow: hidden; }` plus `display: flex; flex-direction: column`. The whole apology + recovery fits in one screen. No scrolling escape, no long-form content, no funnel below.
2. **Friendly-warmth microcopy** — "Whoops!" (more sheepish than "Oops!"), "Nothing here yet" (the "yet" promises future-state possibility), "Seems you've wandered off..." (ellipsis invites the reader to complete the thought).
3. **Themed scenery, not stock illustration** — a spaceship PNG `background-position: center 40%; background-size: contain; background-attachment: fixed`. The metaphor is "you got lost in space" — themed to the page's narrative, not generic "broken-link" iconography.
4. **Two recovery cards, not one and not three** — Main Page + Showcase. One card would feel prescriptive; three+ would overwhelm. Two is the friendly-choice pattern.
5. **Decorative floating icons via Material Symbols Rounded** — `cloud` (top-left, 42px) + `favorite` heart (bottom-right, 32px), each gradient-filled via `background-clip: text` from `#F7B2FB` → `#786EF1` → `#5588FB` (pink → indigo → blue, sunset palette), each drifting on a 5-second `floatSlow` keyframe (translateY -10px + rotate 3deg + back).
6. **Inline pill highlights on action words** — "chat" and "define" wrapped in `<span class="highlight">` with `bg-#E0E2E7 px-3 py-0.5 rounded-md font-semibold text-[12.5px]`. Draws the eye to commitments without breaking inline reading.
7. **Standard nav as fallback recovery path** — the navbar's Our Team / Solutions / Showcase / News links remain available. Three escape routes: the two recovery cards, the standard nav, and the "Let's Connect" CTA.
8. **Subtle 2-step page gradient** — `linear-gradient(to top left, #F5F5F5, #F7F7F7)` — `oklch ~0.96 → ~0.97`, barely-1%-lightness delta. Reads as "a single near-white surface" with directional emphasis toward top-left.

### Concrete recipe to lift

```css
/* Page shell — single viewport, no scroll */
html, body, #root {
  height: 100vh;
  overflow: hidden;
}
body, #root {
  display: flex;
  flex-direction: column;
}

/* Layered background — gradient + themed scenery PNG */
body {
  background:
    url('/scenery.png') center 40% / contain no-repeat fixed,
    linear-gradient(to top left, #F5F5F5, #F7F7F7) center / 100% 100% no-repeat fixed;
}

/* Main flex pane — content centered, cards bottom-anchored */
main {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  max-width: 700px;
  margin: 0 auto;
  padding: 20px 20px 30px;
}
.navigation-cards {
  margin-top: auto;        /* pushes to bottom of the flex pane */
  max-width: 460px;
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

/* Gradient-filled decorative icon glyph */
.decoration-cloud,
.decoration-heart {
  background: linear-gradient(to bottom,
    #F7B2FB 50%,
    #786EF1 80%,
    #5588FB 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  filter: drop-shadow(0 0 1px #fff) drop-shadow(0 0 2px #fff);
}

/* Ambient drift animation */
@keyframes floatSlow {
  0%, 100% { transform: translateY(0px) rotate(0deg); }
  50%      { transform: translateY(-10px) rotate(3deg); }
}
.decoration-cloud { animation: floatSlow 5s ease-in-out 0.3s infinite; }
.decoration-heart { animation: floatSlow 4.5s ease-in-out 1s infinite; }

/* Inline keyword highlight pill */
.highlight {
  display: inline-flex;
  background-color: #E0E2E7;
  font-size: 12.5px;
  font-weight: 600;
  padding: 2px 12px;
  border-radius: 6px;
}

/* Recovery card */
.nav-card {
  background: #ffffff;
  border-radius: 18px;
  padding: 18px 22px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border: 1px solid rgba(0,0,0,0.05);
  box-shadow: 0 2px 12px rgba(0,0,0,0.04);
  transition: transform 0.2s, box-shadow 0.2s;
}
.nav-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 8px 28px rgba(0,0,0,0.08);
}
.nav-card:hover .icon-container { transform: scale(1.05); }
.nav-card:hover .chevron { transform: translateX(6px); }
```

### Copy recipe

- **Kicker:** "Seems you've wandered off..." (ellipsis, lowercase, slightly diminutive)
- **Title:** "Whoops! Nothing here yet" (apologetic, includes "yet" for future-state possibility)
- **Subtext:** *"Grab a 30-minute __chat__ to explore your ideas, scope, and vision. We'll find common ground, sync and __define__ a clear roadmap."* (informal verb "grab", specific time "30-minute", highlighted action words)
- **Card 1:** "Main Page" + subtitle "Back where it all begins..."
- **Card 2:** "Showcase" + subtitle "Where we walk the walk"
- **Nav CTA:** "Let's Connect" (inclusive "Let's", invitation framing)

### Type system

- **DM Sans** variable (`opsz 9..40, wght 100..1000`) for all text
- **Material Symbols Rounded** with `opsz,wght,FILL,GRAD@24,400,1,0` for cloud + heart decorations
- Title: `clamp(34px, 5vw, 52px)` weight 500, letter-spacing -1.5px, line-height 1.08, color `#0f0f0f`
- Body: 14px / 15px in `#888888` secondary
- Caps labels: 11px tracked +1.5px in the navbar

## Empty states elsewhere in the batch (what's MISSING)

The other 54 designs were scanned for empty-state coverage. **Almost nothing.** Specifically:

### Designs that should have empty states but don't
- **#22 Aura** — full inbox mockup, 6 fake messages. No "empty inbox" state. A real email client must handle "zero messages today."
- **#28 Aurora Sign-up** — form with First Name / Last Name / Email / Password / Submit. No empty-state for the success after submit, no error state for invalid email or "account already exists".
- **#41 Nexora** — dashboard with balance, transactions, accounts. No "no transactions yet" empty state.
- **#48 Convix** — Clicks card with 6,896 number. No "0 clicks this month" empty state.
- **#50 Behind the Lens** — Supabase-backed blog with `blog_posts` table. The spec is explicit: "No fallback for failed Supabase fetch — if the Supabase query fails (network error, RLS misconfiguration, table missing), the page shows an empty grid with no message. Production needs a loading state, an error state ('Posts couldn't be loaded; refresh to try again'), and an empty state ('No posts yet — check back soon')."
- **#51 AI Builder** — HLS Mux video hero. No fallback if the Mux asset is deleted or the HLS manifest fails.
- **#46 Stellar.ai** — auto-cycling tab UI with 4 conditional overlays. If a user clicks a tab during auto-cycle the behavior is unspecified. No empty-state for the tabs.

The pattern: **the batch builds the success state and stops**. Productionization needs to add the empty / loading / error states for every fetch surface.

### Designs that DO touch error-adjacent surfaces
- **#26 Mindloop "Search has changed. Have you?"** — Section 3 frames AI search as a meta-acknowledgment, but it's positioning copy, not a "no results" empty state. Still: the rhetorical move of *naming the problem the visitor came here to solve* is reusable for any empty-search-state copy.
- **#03 Prisma** — gracefully handles a single-page narrative where users who scroll past the hero get an About + Features bento. No explicit "you've reached the end" state, but the design ends on a content note rather than a footer.

## Loading states across the batch

Almost no design specifies a loading state. The pattern is:

- **Videos** autoplay-muted-loop with no `poster` attribute. Slow connections see a blank canvas.
- **Images** are direct URLs with no LQIP, no blur placeholder, no skeleton.
- **Fetches** (the few Supabase designs) don't specify a loading state — the page renders empty, fetches, then re-renders.

The few exceptions:
- **#15 Axion Studio shaders** — procedural backgrounds via `shaders/react` render immediately (no asset load). The shader pattern is the rare loading-state win.
- **#23 Animated Gradient CTA** — CSS `@property` gradients are computed at parse time, no load delay.

Recommended addition for all batch-family designs:

```jsx
{/* Image with skeleton fallback */}
<div className="relative aspect-video bg-gray-100 rounded-2xl overflow-hidden">
  {loaded ? null : <div className="absolute inset-0 bg-gradient-to-br from-gray-100 to-gray-50 animate-pulse" />}
  <img onLoad={() => setLoaded(true)} src={...} className="absolute inset-0 w-full h-full object-cover" />
</div>

{/* Video with poster */}
<video autoPlay muted loop playsInline poster="/poster.jpg" className="...">
  <source src={...} />
</video>
```

## Form error states (universal gap)

#28 Aurora Sign-up is the canonical form-error-gap design. Its brief is explicit: *"No error states specced for inputs. Real form needs invalid-email, password-too-short, account-already-exists, etc."*

For any batch-family design with a form, productionization needs:

- **Field-level validation** — invalid border color (e.g., `border-red-500` on the form's shadcn HSL system would be `border-destructive`), error message below the field
- **Submit-loading state** — spinner inside the button, button disabled, "Creating account…" copy
- **Submit-error state** — toast or inline message above/below the form, "Couldn't create account. Try again."
- **Submit-success state** — explicit confirmation, ideally with next-action affordance ("Check your email for the confirmation link.")

The motionsites cluster's universal pattern is to skip all of this — the form is rendered as the "happy path only" specimen. Always add error handling on production.

## Patterns from Nexto 404 worth lifting to other empty/error contexts

### 1. The `100vh + overflow: hidden` single-viewport commitment
Use for: error states, splash screens, single-action landing pages, any page that should feel **self-contained and unscrollable**. The constraint forces design discipline — every element must fit in one viewport, no escape via scroll.

### 2. The `margin-top: auto` bottom-anchored flex element
Use for: putting recovery cards / CTAs at the bottom of a flex-column container while keeping the apology centered. The CSS-canonical pattern for "push this to the end of the flex container."

### 3. Themed scenery PNG via fixed background
Use for: error states where the visual *is* the metaphor. Spaceship for "lost in space" (404). Could be a magnifying glass for "no search results". A folder for "empty inbox". An hourglass for "session timed out".

### 4. Gradient-filled icon glyphs via `background-clip: text`
Use for: any icon font glyph that should feel decorative-illustrated rather than UI-functional. The double-white-drop-shadow halo (`drop-shadow(0 0 1px #fff) drop-shadow(0 0 2px #fff)`) is the legibility guard against light backgrounds.

### 5. The 2-card recovery pattern
Use for: error states, empty inboxes, no-search-results, post-onboarding completion screens. Pick the two most-common visitor intents and offer both. One feels prescriptive; three+ overwhelms.

### 6. Inline-pill keyword highlighting
Use for: subtext where specific words are *commitments* or *actions* the brand is offering. Pills draw the eye to those words without breaking the reading flow. Smaller than the surrounding text (12.5px vs 14px), heavier weight (600 vs 400), `border-radius: 6px` for friendly tags (not pill-shaped which would read as actionable).

### 7. Ambient drift animation (`floatSlow`)
Use for: decorative elements that should feel alive without distracting. Slightly-different durations (5s + 4.5s, not synced) prevent mechanical pattern-spotting. `ease-in-out` only (linear feels mechanical). `prefers-reduced-motion` must disable.

### 8. Hand-drawn-style dashed border via CSS gradient
Use for: any context where `border-bottom: dashed` would render with browser-specific dashes. The CSS-gradient approach (`background-image: linear-gradient(to right, rgba(0,0,0,0.08) 2px, transparent 2px); background-size: 6px 1px;`) gives pixel-exact control across browsers.

### 9. The "Let's [Action]" inclusive CTA
"Let's Connect" beats "Connect" or "Contact Us" for friendly-warm voice. Implies *we're inviting you to do this together*, not *click here to fill out our form*. Especially appropriate for error states where the visitor is already mildly frustrated.

### 10. The "yet" suffix on apologetic copy
"Nothing here yet" beats "Nothing here". The "yet" suggests **future-state possibility** — the page might exist later, the gap is temporary, the brand is iterating. Tone-managers a 404 from dead-end to *not-built-this-far-yet*.

## When NOT to use the Nexto 404 recipe

- **High-stakes systems errors** (payment failure, data loss, security breach) — the friendly-warmth voice undermines severity. Use Apple/Stripe's neutral-clinical voice ("We couldn't process your payment. Try again or contact support.").
- **API/dashboard error states** — production tools need accurate technical messaging, not metaphorical scenery. Show the actual error code, the request ID, the retry button.
- **Empty inbox for productivity tools** — Linear, Notion, Superhuman all use minimal empty states that fade away on first action. Don't decorate empty inboxes with floating hearts.

The Nexto 404 recipe is for **marketing-page error states** where the visitor's mood matters more than the technical fault. For systems and tools, the recipe should be much quieter.

## Anti-patterns the batch (mostly) avoids

- **The crashed-robot illustration** (the cliché 404 trope from 2015-2020). Nexto uses a spaceship PNG that's specific to its narrative metaphor, not a generic broken-page mascot.
- **The "404 Page Not Found" technical label** — Nexto's "Whoops! Nothing here yet" replaces taxonomic language with conversational apology.
- **The dead-end with no escape** — Nexto provides three escape routes (2 recovery cards + standard nav + nav CTA). Production 404s should always offer multiple paths back.
- **The "back to home" only** — Nexto offers Main Page + Showcase, two distinct intents. Cards labeled "Home" alone are missed opportunities.
- **The hostile "Error 500" black-on-white** — Nexto's gentle gradient background (`#F5F5F5 → #F7F7F7`) keeps the page in the same visual register as the rest of the brand.

## Pattern: friendly-warmth copy vocabulary for error/empty states

A vocabulary specific to the *visitor-didn't-mean-to-be-here* surface. Drawn from Nexto 404 plus general industry conventions:

| Use | Recommended | Avoid |
|---|---|---|
| Page header | "Whoops! Nothing here yet" | "Page Not Found" / "404 Error" / "Oops!" |
| Soft kicker | "Seems you've wandered off..." | "Sorry, that's not a real URL." |
| Inviting CTA | "Let's Connect" | "Contact Us" / "Reach Out" / "Send a Message" |
| Recovery card title | "Main Page" / "Showcase" | "Home" / "Browse" |
| Recovery card subtitle | "Back where it all begins..." / "Where we walk the walk" | "View site" / "See projects" |
| Inviting verb | "Grab a 30-minute chat" | "Schedule a meeting" / "Book a call" |
| Future-state hedge | "Nothing here yet" | "This page doesn't exist." |
| Searching no-results | "We couldn't find that yet" | "0 results" / "No matches" |
| Empty state | "Nothing to show — yet." | "No data available" |
| Loading | "Hang tight..." (sometimes) | "Loading..." (often, fine for tools) |

## Recommendations distilled

For any batch-family designer building an error / empty / 404 state:

1. **Lift Nexto 404's recipe wholesale** for marketing-context errors. The `100vh + overflow: hidden` + `flex-column` + `margin-top: auto` + themed scenery PNG + 2 recovery cards + inline-pill subtext + ambient `floatSlow` decorations is a complete and self-consistent pattern.
2. **Use friendly-warmth copy** for marketing 404s. The "Whoops!", "yet", "Let's Connect", "Grab a chat" vocabulary moves the page from technical failure to brand moment.
3. **Use clinical-neutral copy** for systems errors (payment, auth, API). Don't decorate severity.
4. **Always include `prefers-reduced-motion` handling** — disable `floatSlow`, disable hover lifts, freeze decorations.
5. **Always include 404 analytics** — track which broken URL led the visitor here, so the source team can fix the link.
6. **Always include error states for any fetch surface** — Supabase fetch failures, image load failures, video manifest failures all need explicit handling.
7. **Build the empty inbox / no-search-results / no-transactions states explicitly** — the batch's success-state-only convention doesn't survive contact with real users.
8. **Don't add a hamburger menu over the error state** unless the standard nav has critical destinations. Nexto's choice (keep the standard nav available) is correct because the nav has fallback recovery; but a 404 with only "back to home" doesn't need a full nav above it.

## Open questions

- Where does **analytics tracking** for 404 hits live? Nexto's spec doesn't say. Production should at minimum log `window.location.pathname` on 404 page load so the team can find broken links pointing in.
- What's the **mobile experience** for Nexto 404? The `100vh` constraint works for mobile too, but `background-attachment: fixed` is known to glitch on iOS Safari. Production should test or replace with `background-attachment: scroll`.
- How do the **recovery cards adapt at <360px** viewports? The 460px max-width is fine for desktop and tablet; mobile needs smaller padding and tighter card stacking.
- Should the **navigation cards** show the recovery destinations' freshness ("Last updated 2 days ago") or current status ("Open today")? The 2-card pattern is a recovery affordance; if the destinations have state, that state could appear on the cards.
- For the broader batch, what should the **standardized empty/loading/error state component library** look like? A skeleton system that maps onto the shadcn HSL-triple convention would be the natural home for cross-design reuse.

---

*Captured: 2026-05-29. Nexto 404 is the only explicit error-state design in 55 — and the gap is the playbook's main warning. The recipe is complete and excellent for marketing-context 404s; the work of building error states for the rest of the batch (form errors, fetch failures, empty inboxes, no-search-results) is universal and unaddressed.*
