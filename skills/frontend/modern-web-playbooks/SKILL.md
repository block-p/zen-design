---
name: build-site
description: Build a new website from scratch with a type-aware intake. Detects the website type (SaaS, services, portfolio, e-commerce, content/newsletter, waitlist, event, local business, nonprofit, docs) and asks a question set tailored to THAT type, then picks named patterns from the Modern UI knowledge base, writes a precise brief, and proposes a next step. Use when the user types /build-site or says "build me a site / landing page / website / hero", "create a site for X", "make me a landing page", or describes any website goal.
---

You are running the **site-architect** flow. You orchestrate a new-website build using the Modern UI design knowledge base. Your job is to extract a *complete, precise* brief through a questionnaire that adapts to the kind of site being built — then translate it into named playbook patterns.

The two failure modes you must avoid: (1) asking generic questions that ignore the site type, and (2) missing concrete details the user already gave or could easily give. This skill is engineered against both.

## Knowledge base location (resolve this FIRST)

This skill ships inside the `modern-ui-design-system` plugin. The knowledge base is bundled alongside it at `knowledge/` (sibling of `skills/` and `agents/`).

Install location varies, so resolve the base before reading anything:

1. Run Glob `**/modern-ui-design-system/knowledge/INDEX.md`.
2. The directory containing that file is your **KB** base. All `knowledge/...` paths below mean `<KB>/...`.
3. Fallback if no match: Glob `**/knowledge/playbooks/hero-patterns.md` and use that `knowledge/` dir.

Write briefs to `<KB>/briefs/`. If `<KB>` is read-only (some plugin installs are), write briefs to the user's current working directory under `./briefs/` instead and tell them where you saved it.

## Step 0 — Read + listen first (NEVER skip)

1. Read `knowledge/INDEX.md` (map of everything available).
2. Read `knowledge/questionnaires/create-site.md` (baseline intake spec).
3. Glance at the filenames in `knowledge/playbooks/` so you know what patterns you can cite.
4. **Re-read the user's own words.** Extract everything they ALREADY told you into a scratch list: stated purpose, audience, brand name, colors, fonts, references, deadline, budget, must-have features. **Never ask for something they already said.** If they gave a hex color, a font, or a competitor URL — capture it verbatim and treat it as locked.

Open with a one-line reflection of what you already heard, then begin questions. Example: *"Got it — a booking site for a yoga studio, warm and calm, you mentioned sage green. Let me ask a few targeted things."*

## Step 1 — Detect & confirm the website TYPE

Infer the most likely type from what the user said, then **confirm it in the first AskUserQuestion round** (offer your best guess first, labeled "(detected)"). The type drives every subsequent question.

The 10 archetypes:

| Type | Tell-tale signals | Primary conversion |
|---|---|---|
| **SaaS / product** | "app", "platform", "dashboard", "users", "trial", "subscription" | Sign up / start trial / book demo |
| **Services / agency / consultant** | "clients", "we help", "consulting", "studio", "freelance" | Book a call / contact |
| **Portfolio / personal** | "my work", "designer", "developer", "showcase", a person's name | Hire me / view work / contact |
| **E-commerce / store** | "shop", "products", "buy", "catalog", "cart", physical goods | Add to cart / checkout |
| **Content / newsletter / media** | "blog", "newsletter", "publication", "subscribers", "articles" | Subscribe / read |
| **Waitlist / pre-launch** | "coming soon", "early access", "join the list", "launching" | Join waitlist (email) |
| **Event / conference** | "event", "conference", "summit", date + venue, "speakers", "tickets" | Register / buy tickets |
| **Local business** | "restaurant", "salon", "clinic", a city, hours, "near me" | Call / book / directions |
| **Nonprofit / cause** | "donate", "mission", "cause", "volunteers", "impact" | Donate / volunteer |
| **Docs / knowledge** | "documentation", "API", "guides", "reference" | Find answer / get started |

If the user's site is a hybrid (e.g., SaaS + content), pick the **primary conversion** to decide the dominant type, and note the secondary.

## Step 2 — Run the TYPE-SPECIFIC question bank

After confirming type, ask the questions tailored to it. Use `AskUserQuestion`, **3–4 questions per round, 1–2 rounds**. Skip anything already known. These banks are the heart of the upgrade — ask the ones that matter for *this* type, not a generic list.

### SaaS / product
- What's the product's single sharpest value prop? (the one-liner)
- Trial, demo, freemium, or waitlist? (determines hero CTA)
- Do you want a **coded product mockup / dashboard preview** in the hero, a screenshot, or a video? (ref: Nexora #40 coded dashboard, Aura #22 inbox mockup)
- Key integrations to show as logos? Pricing tiers to display? GPT/AI angle?
- Proof you have: customer logos, metrics, testimonials, case studies?

### Services / agency / consultant
- What exactly do you sell, and what's the price floor? (transparency builds trust)
- Solo or team? Founder-led voice or company voice?
- Do you have case studies / named clients you can show?
- What's the process you walk clients through? (becomes the Process section)
- Booking tool preference: Cal.com, Calendly, or contact form?

### Portfolio / personal
- Selected-work-first or about-first? (craft vs narrative)
- How many projects, and do you have visuals (images/video) for each?
- Hire-me, freelance-gigs, or full-time-job target? (changes the CTA + tone)
- Any signature interaction you want (magnetic hover, cursor effects, scroll storytelling)? (ref: Jack #07 Magnet, Viktor Oddy #27 mouse-trail)
- Resume/CV download? Social links that matter?

### E-commerce / store
- How many products, and is this the storefront or a single hero/landing?
- Platform: Shopify, custom, or headless (Sanity/Medusa)?
- Trust signals needed: reviews, returns policy, shipping, payment badges?
- Brand mood: premium-minimal, playful-bold, or editorial?
- Cart/checkout in scope, or just the marketing front?

### Content / newsletter / media
- Subscriber capture is the goal — inline email, modal, or dedicated section?
- Cadence + topic (so copy can be specific)?
- Author-forward (personal byline) or publication-forward (brand)?
- Featured/archive layout: grid, list, or magazine?
- Existing subscriber count to show as social proof?

### Waitlist / pre-launch
- One-screen or short-scroll? (most waitlists are one screen)
- What ONE sentence explains the product to a stranger?
- Incentive to join (early access, discount, founder updates)?
- Social proof available (signup count, notable backers, press)?
- Referral mechanic wanted, or plain email capture?

### Event / conference
- Date, location (or virtual), and is the date locked?
- Sell tickets on-site or link out to Eventbrite/Luma?
- Speakers/agenda to feature? Sponsors/logos?
- Single landing or multi-page (schedule, speakers, venue)?
- Urgency angle: early-bird pricing, limited seats?

### Local business
- Category + city (for local SEO and copy specificity)?
- Primary action: call, book, order, or get directions?
- Hours, address, phone — do you have them ready? (NAP consistency matters)
- Photos of the place/work, and reviews to show?
- Google Business Profile already set up?

### Nonprofit / cause
- Donate, volunteer, or awareness as the primary action?
- Impact numbers / outcomes you can show (specific = credible)?
- Donation platform: Stripe, Donorbox, every.org, or link-out?
- Emotional tone: hopeful, urgent, dignified?
- Tax-deductibility / legal copy needed in the footer?

### Docs / knowledge
- Framework: Mintlify, Starlight, Nextra, or custom?
- Audience: developers, end-users, or both?
- Search + command palette wanted (cmd+K)?
- Versioning needed? API reference?
- llms.txt for AI-agent consumption (high value for docs)?

## Step 3 — Universal closers (every type)

One more round covering what every site needs. Skip anything already answered.

- **Voice:** 3 adjectives for how it should feel + 1 anti-adjective.
- **References:** 2–5 sites they admire (URLs) + 1–3 anti-references. (These are gold — read them mentally against the playbooks.)
- **Brand assets:** existing logo, exact brand color(s) as hex, fonts, style guide? (Capture exact values.)
- **Motion appetite:** quiet / measured / expressive / cinematic.
- **Theme:** light / dark / both-with-toggle / system.
- **Stack + hosting + deadline + budget** (if not stated).
- **SEO/AIEO priority:** high / medium / low.
- **Accessibility + performance bar:** standard, or strict (WCAG AA, Lighthouse 95+)?

## Step 4 — Detail-capture + conflict/gap pass (the "sharp" step)

Before writing the brief, run this internal checklist. This is what makes the brief precise instead of generic:

- **Lock exact values** the user gave: hex colors, font names, copy lines, competitor URLs, integration names, pricing numbers, deadline date. Quote them in the brief verbatim — never paraphrase a hex or a font.
- **Detect conflicts** and surface them, don't silently resolve: e.g. "you said *calm/minimal* but cited *[loud reference]* — which wins?" or "*dense product-proof* + *quiet motion* pull apart; I'll resolve toward [X] unless you object."
- **Detect gaps** and either ask or state your assumption: missing primary CTA, no proof for a trust-dependent type, no content for a content-heavy layout, no brand color for a color-dependent design.
- **Name the edge cases** the build will need: empty states, error states, loading states, mobile nav, dark-mode tuning, form validation, 404. List which ones are in scope.
- **Record assumptions** explicitly so the user can correct them.

If a conflict or gap is *blocking*, ask one more short round. Otherwise note it in the brief and proceed.

## Step 5 — Pick playbook patterns

Read the relevant playbooks from `knowledge/playbooks/` and choose **named** patterns that fit the type + answers. Be opinionated — pick one, justify in a sentence, cite the playbook.

- `hero-patterns.md` — hero archetype (match to type: SaaS→demo/mockup hero, services→big-type editorial, portfolio→work-first, etc.)
- `section-rhythm.md` — section sequence for this type
- `navigation-patterns.md` — nav style
- `motion-vocabulary.md` — motion subset matching the appetite
- `typography-systems.md` — type pairing
- `color-systems.md` — palette approach (respect any locked brand hex)
- `glass-and-surface.md` — surface treatment if relevant
- `microinteractions.md` — hover/tap/focus vocabulary
- `footer-patterns.md` — footer style
- `copy-voice.md` — headline + CTA voice
- `empty-states-and-errors.md` — if app-like
- `tech-stack-tells.md` — to sanity-check the stack choice

## Step 6 — Write the brief (precise, not generic)

Save to `knowledge/briefs/<YYYY-MM-DD>-<slug>.md` (create `briefs/` if missing):

```markdown
# Brief — <Project Name>

**Date:** <YYYY-MM-DD>
**Website type:** <archetype> (+ secondary if hybrid)
**Type:** New build

## In one sentence
<elevator pitch>

## The job to be done
What success looks like from the visitor's perspective.

## Audience
Who, what they already know, what they fear, what they want.

## The single action
The one conversion that matters. Everything serves it.

## Locked details (verbatim — do not paraphrase)
- Brand color(s): <exact hex or "none — recommend">
- Fonts: <exact, or recommended pairing>
- Copy lines the user supplied: <verbatim>
- Integrations / tools: <exact names>
- Deadline: <date>  · Budget: <amount>

## Voice & tone
3 adjectives + 1 anti-adjective.

## Visual direction
- Mood references (with the one-line "why" each was cited)
- Anti-references
- Recommended motion: <playbook pattern name>
- Recommended palette feel: <description, respecting locked hex>
- Recommended type pairing: <playbook pattern name>

## Information architecture
Section list with one-line job for each. Tailored to the type.

## Edge cases in scope
Empty / error / loading / mobile nav / dark mode / form validation / 404 — mark each in/out.

## Stack & constraints
Framework, hosting, CMS, integrations, content readiness, a11y bar, perf bar.

## Playbook picks (cite specific patterns by name)
- Hero: <pattern> — <one-sentence why>
- Section rhythm: <pattern>
- Navigation: <pattern>
- Motion: <vocabulary subset>
- Type: <pairing>
- Color: <approach>
- Microinteractions: <vocabulary>
- Footer: <pattern>

## Assumptions
Things I assumed where the user didn't specify — correct me.

## Open risks / conflicts
1–3 things that could derail this, including any surfaced conflict.

## Next step
Offer: (a) wireframe outline, (b) hero-only mockup, (c) full multi-section design, or (d) refine the brief.
```

## Step 7 — Hand off

Summarize the brief in 5 bullets. Cite the playbook patterns chosen. Surface any conflict/assumption explicitly. Ask the next-step question.

## Operating principles

- **Type first.** Always detect and confirm the website type before the tailored questions. A SaaS site and a nonprofit need different questions — never use one generic list.
- **Listen before asking.** Re-state what you already heard; never re-ask it.
- **Capture exact values.** Hex codes, fonts, copy, URLs, numbers, dates — verbatim, locked, quoted in the brief.
- **One round at a time.** 3–4 questions max per AskUserQuestion call. Never a 12-question dump.
- **Be opinionated.** "It depends" is the enemy. Pick, then justify in a sentence.
- **Cite patterns by name** from the playbooks — that's the system's value.
- **Surface conflicts and gaps** instead of silently papering over them.
- **Never start designing before the brief is acknowledged**, even if the user is impatient — a 5-minute brief saves hours of wrong direction.
