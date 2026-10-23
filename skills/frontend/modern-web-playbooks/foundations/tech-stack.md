# Modern Web Tech Stack — Foundations

## Why this matters

A modern site's tech is invisible when it works and obvious when it doesn't. The stack choices affect performance, DX, hiring, hosting cost, and how the site can evolve. This file is the catalog of what's current and what's appropriate when.

## Frameworks (meta-frameworks)

### Next.js
- **Current:** 16.2 (March 2026) [Source](https://nextjs.org/blog/next-16-2) — ~400% faster `next dev` startup, ~50% faster rendering, 200+ Turbopack fixes, stable Adapter API
- **Strengths:** Stable RSC (React Server Components) since React 19 [Source](https://react.dev/blog/2024/12/05/react-19), broad ecosystem, Vercel-tuned, App Router for nested layouts and parallel routes
- **Best for:** product sites with complex routing, dashboards, marketing+app hybrid
- **Watch:** RSC mental model is a learning curve; underlying RSC bundler APIs do not follow semver [Source](https://react.dev/reference/rsc/server-components); May 2026 security release patched 13 advisories [Source](https://vercel.com/changelog/next-js-may-2026-security-release) — keep current

### Astro
- **Current:** 6.x (upgrade guide live) [Source](https://docs.astro.build/en/guides/upgrade-to/v6/); Astro 5 introduced Content Layer API, Server Islands, native View Transitions [Source](https://astro.build/blog/astro-5-beta/)
- **Strengths:** Islands architecture, zero-JS by default, multi-framework friendly (React, Vue, Svelte, Solid in one project), Content Collections 2.0 with type-safe loaders from any source (CMS, API, DB)
- **Best for:** marketing sites, blogs, content-heavy sites, documentation
- **Watch:** Less suited for highly interactive apps

### SvelteKit
- **Current:** SvelteKit 2+, Svelte 5 shipped Oct 2024 with runes ($state, $derived, $effect, $props) [Source](https://svelte.dev/blog/runes)
- **Strengths:** Less code than React, fast, compiled (no virtual DOM), excellent DX, backward-compatible — Svelte 4 components still run unchanged [Source](https://svelte.dev/docs/svelte/v5-migration-guide)
- **Best for:** marketing sites, dashboards, anywhere the team can ship Svelte
- **Watch:** Smaller ecosystem than React; hiring is harder; new code should use rune-based `$app/state` over legacy `$app/stores`

### Remix → React Router (merger)
- Remix merged into React Router v7 — now one library [Source](https://reactrouter.com/start/modes)
- Two modes: **Data Mode** (drop-in upgrade) and **Framework Mode** (Vite plugin adds SSR, file routing, server functions — the Remix successor)
- Early RSC support landed in v7.9.2 but not production-ready [Source](https://react.dev/reference/rsc/server-components)
- **Best for:** apps that want server-rendered React without Next.js opinions; teams migrating from Remix v2

### TanStack Start
- Full-stack framework (React **and** Solid) currently at RC, feature-complete [Source](https://tanstack.com/start/latest/docs/framework/solid/overview)
- Type-safe routing/server functions inherited from TanStack Router, streaming SSR, universal deploy
- **Best for:** teams already on TanStack Query/Router wanting end-to-end type safety

### Qwik
- Resumability (no hydration)
- **Best for:** marketing sites where TTI is critical
- **Watch:** smaller community, fewer libraries

### Solid / SolidStart
- Fine-grained reactivity (signals), React-like syntax
- **Best for:** performance-sensitive apps where the team can adopt new mental models

### Vue / Nuxt
- Strong outside-US adoption, excellent DX
- Nuxt 3+ is competitive with Next.js
- **Best for:** teams already on Vue or building Vue-friendly tools

### Webflow / Framer (no-code)
- **Framer** has matured into a serious site builder; many premium marketing sites ship on it
- **Webflow** still strong for marketing+CMS
- **Best for:** teams without an engineer, or when speed-to-launch trumps customization

## Styling

### Tailwind CSS v4
- **v4.0 stable Jan 2025, v4.2 Feb 2026** [Source](https://github.com/tailwindlabs/tailwindcss/releases) — Rust-based **Oxide engine** unifies content scanning, compilation, optimization in one pass
- 3.5x faster full builds, 8x faster incremental, 100x+ for no-new-CSS changes; OKLCH-default colors, CSS-native `@theme` config, single `@import "tailwindcss"` entry
- Surpassed Bootstrap as the #1 CSS framework in State of CSS 2024 [Source](https://2024.stateofcss.com/en-US/tools/)
- **Use with:** any framework; works everywhere

### CSS Modules
- Scoped CSS, no runtime, baseline for many
- **Best for:** teams that want plain CSS with scoping

### Vanilla Extract
- Zero-runtime CSS-in-TS
- Type-safe styles
- **Best for:** design system authoring with type safety

### Linaria, Panda CSS
- Zero-runtime CSS-in-JS, with Panda having design-token-first DX
- **Best for:** teams wanting CSS-in-JS DX without runtime cost

### styled-components / Emotion (legacy)
- Runtime cost; falling out of favor for new projects
- Use only if existing codebase

### Stylex (Meta)
- Atomic CSS, atomic class output, fast
- **Use for:** scale (Meta-size apps)

## Component libraries (headless)

### Radix UI
- Unstyled, accessible primitives
- React-only
- Now the de-facto pick for design systems built on React

### React Aria (Adobe)
- Hooks for accessible behavior; bring your own DOM
- More powerful but more verbose than Radix
- Cross-platform aspirations (React Native, etc.)

### shadcn/ui
- Not a library — a copy-paste collection of Radix + Tailwind components [Source](https://ui.shadcn.com/docs/components)
- **The 2024+ default** for new React projects that want speed without library lock-in
- CLI v4 (March 2026) [Source](https://ui.shadcn.com/docs/changelog/2026-03-cli-v4) added `registry:base` to install entire design systems in one shot, fonts as first-class registry items, target aliases
- Owns its code, modifies freely; thriving third-party registry ecosystem [Source](https://ui.shadcn.com/docs/directory)

### Park UI / Ark UI
- Cross-framework headless components
- **Best for:** multi-framework projects

### Headless UI (Tailwind Labs)
- Smaller scope than Radix; sufficient for many sites

## Animation libraries

### Motion (formerly Framer Motion)
- Rebranded to **Motion** in 2025 — independent from Framer [Source](https://motion.dev/docs/react-upgrade-guide); import from `motion/react` not `framer-motion`
- Latest 12.40.x (May 2026) [Source](https://motion.dev/changelog) — adds repeatType/repeatDelay in animation sequences, React 19 drag fixes
- React, vanilla JS, and Vue flavors; strong layout animations, gestures, springs
- **Use for:** rich React apps

### GSAP
- **100% free for everyone since April 2025** — Webflow acquired GreenSock fall 2024 and unlocked all Club plugins (ScrollTrigger, SplitText, MorphSVG, etc.) including commercial use [Source](https://webflow.com/blog/gsap-becomes-free)
- Industry-standard for sophisticated timeline/scroll work
- **Use for:** scroll-driven narrative, SVG manipulation

### Auto-Animate (FormKit)
- Drop-in for list animations
- Tiny, dead simple
- **Use for:** quick polish without learning Motion

### Lottie / dotLottie
- After Effects → JSON workflow
- **Use for:** branded illustrated animations

### Rive
- Interactive animations with state machines
- **Use for:** UI illustrations that respond to interaction

## Headless CMS

### Sanity
- Real-time, customizable studio, GROQ query language
- **Best for:** teams that want flexibility and a serious editorial workflow

### Contentful
- Mature, enterprise-friendly, generous free tier
- **Best for:** marketing teams with non-technical editors

### Strapi
- Open source, self-hostable
- **Best for:** teams that want control of the CMS

### Storyblok
- Visual editor, modular blocks
- **Best for:** marketing teams that want WYSIWYG

### Payload CMS
- Code-first, TypeScript, self-hostable; installs directly into Next.js — CMS and app are the same project
- **Acquired by Figma in 2025** [Source](https://www.digitalapplied.com/blog/headless-cms-2026-sanity-contentful-payload-comparison); stars grew 197% Jan 2024 → Apr 2026
- Payload 3.0 cut deps from 88 to 27; local API runs in-process (no network, no rate limits)
- **Best for:** developer-first teams who want CMS without external service — strongest momentum of any headless CMS in 2026

### Hygraph (GraphCMS)
- Schema-driven, GraphQL-native
- **Best for:** GraphQL-first teams

### Builder.io
- Visual editing + headless
- **Use for:** drag-and-drop marketing pages

### Notion as CMS
- Increasingly common for blogs (via Notion API + custom layer)
- **Use for:** small teams that already live in Notion

## Hosting

### Vercel
- Tightly integrated with Next.js (same company)
- Edge functions, image optimization, analytics built-in
- **Best for:** Next.js projects

### Netlify
- Multi-framework, edge functions, generous free tier
- **Best for:** static sites and non-Next.js projects

### Cloudflare Pages / Workers
- Global edge network, Workers for compute, R2 for storage
- Cheap at scale
- **Best for:** cost-conscious global apps

### Fly.io
- Run apps near users, dockerized
- **Best for:** apps that need server-side compute beyond serverless

### Railway, Render
- Heroku-style PaaS
- **Best for:** full-stack apps with databases included

### AWS, GCP, Azure
- For when you've outgrown PaaS or have enterprise requirements

## Databases

### Postgres-based
- **Supabase** — Postgres + auth + storage + realtime; Firebase-killer
- **Neon** — serverless Postgres, branching; best default serverless Postgres for new projects in 2026 [Source](https://encore.dev/articles/supabase-alternatives)
- **PlanetScale** — killed free Hobby tier April 2024, dropped Vitess maintenance Feb 2024 [Source](https://planetscale.com/docs/plans/hobby-plan-deprecation-faq); reintroduced a $5/mo tier in 2025 but trust is shaken — not recommended for new indie projects
- **Xata** — Postgres + search + analytics

### Other
- **Turso** — SQLite at the edge, distributed
- **Convex** — TS-native backend with reactive queries
- **Firebase** — still strong for mobile and quick prototypes
- **MongoDB Atlas** — document store, mature

## Auth

- **Better Auth** — TypeScript-native open source library; 2FA, passkeys, RBAC, impersonation, plugin system — the **strongest default for new self-hosted Next.js projects in 2026** [Source](https://blog.logrocket.com/best-auth-library-nextjs-2026/)
- **Clerk** — best DX, fastest time-to-prod; common pattern is ship on Clerk → migrate to Better Auth/Supabase around 50K MAU when bills bite
- **Auth0** — enterprise standby
- **Supabase Auth** — bundled with Supabase DB; pairs naturally with RLS
- **NextAuth.js / Auth.js** — open source, framework-agnostic; v5 shipped but fewer built-in features than Better Auth
- **WorkOS** — enterprise SSO/SAML focus
- **Lucia** — **deprecated March 2025** [Source](https://www.wisp.blog/blog/lucia-auth-is-dead-whats-next-for-auth); now a learning resource only — migrate to Better Auth

## Email

- **Resend** — modern email API, great DX, React Email templates (JSX → HTML+text rendered automatically); 3K/mo free tier, no auto-overage [Source](https://resend.com/)
- **Postmark** — transactional focus
- **SendGrid, Mailgun** — legacy but reliable

## Payments

- **Stripe** — default; **Stripe Managed Payments** is now Stripe's own merchant-of-record offering [Source](https://www.lemonsqueezy.com/blog/2026-update)
- **Polar** — open source, dev-focused, 4% + $0.40 (cheapest of major MoRs); fastest-growing pick for indie devs [Source](https://devtoolpicks.com/blog/polar-vs-lemon-squeezy-vs-creem-2026)
- **Lemon Squeezy** — **acquired by Stripe July 2024**; roadmap quiet, 5% + $0.50 (now the most expensive option); avoid for new builds unless already integrated
- **Paddle** — merchant of record alternative
- **Creem** — newer MoR entrant, 3.9% + $0.40

## Analytics

- **Plausible** — privacy-friendly, lightweight, EU-friendly
- **Fathom** — similar to Plausible
- **PostHog** — full product analytics + feature flags + session replay
- **Vercel Analytics** — built-in for Vercel deploys
- **Google Analytics 4** — free, comprehensive, controversial privacy
- **Mixpanel, Amplitude** — product analytics at scale

## Forms

- **React Hook Form** — most flexible, performant for React
- **Conform** — progressive enhancement focus
- **Formspree, Basin** — backend-as-a-service for static sites
- **Tally** — drag-and-drop forms

## Search

- **Algolia** — best-in-class hosted search
- **Typesense** — open-source Algolia alternative
- **Meilisearch** — open source, fast, easy
- **Pagefind** — static site full-text search (no backend)

## Image / media

- **Cloudinary** — image and video CDN with transformations
- **Imgix** — image CDN
- **ImageKit** — alternative
- Built-in: **Next.js Image**, **Vercel Image Optimization**, **Cloudflare Images**

## Icons

- **Lucide** — community-driven, replaces Heroicons for many
- **Heroicons** — Tailwind Labs
- **Phosphor Icons** — large set, multiple weights
- **Radix Icons** — minimal set, pairs with Radix UI
- **Tabler Icons** — large open set

## Developer tooling

- **TypeScript** — default for modern projects
- **pnpm** — fast package manager, monorepo-friendly, content-addressable store; the safe pick for large/long-term codebases
- **Bun** — runtime + package manager + bundler + test runner in one binary; 4–5x faster cold installs than pnpm 10, ~95% Node compat [Source](https://bun.com/package-manager)
- **Turborepo / Nx** — monorepo task runners
- **Biome v2.x** — Rust-based formatter+linter, 10–100x faster than ESLint+Prettier; **500+ rules, type-aware linting** in v2 "Biotype" release [Source](https://biomejs.dev/)
- **Vite 8** (March 2026) [Source](https://vite.dev/blog/announcing-vite8) — ships **Rolldown** (Rust bundler) as default; 10–30x faster builds. Rolldown 1.0 stable May 2026 [Source](https://voidzero.dev/posts/announcing-rolldown-1-0)
- **tsup** — small library bundling

## ORMs

- **Drizzle** — code-first TypeScript ORM, SQL-close, tiny bundle, fast serverless cold starts [Source](https://makerkit.dev/blog/tutorials/drizzle-vs-prisma)
- **Prisma 7** (late 2025) — Rust query engine removed, client is now pure TypeScript — simpler edge/Workers deploys; still the schema-first ergonomics champion
- **Choose:** Drizzle for edge/performance, Prisma for abstraction + ecosystem maturity

## When to choose what — quick rules

| If you want... | Reach for... |
|---|---|
| Marketing site, fast launch | Astro + Tailwind + headless CMS (Sanity/Storyblok) |
| Marketing site, no code | Framer or Webflow |
| SaaS marketing + product | Next.js App Router + Tailwind + shadcn/ui + Vercel |
| Portfolio | Astro or Next.js static + custom |
| Documentation | Astro Starlight, Mintlify, or Nextra |
| Blog | Astro Content Collections, or Next.js + MDX |
| Dashboard / app | Next.js + shadcn/ui + Supabase/Convex |
| Multi-tenant SaaS | Next.js + Postgres + Clerk + Stripe |
| Mobile + web | React Native + Next.js sharing components, or Expo Router |

## Anti-patterns

- Picking the most popular tool without checking fit
- React for a 5-page brochure site
- Custom CMS when Sanity would take a day
- Self-hosting Postgres at startup phase (use Supabase/Neon)
- Resume-driven development — adopting tech because it's new

## Sources

- [Next.js blog & changelog](https://nextjs.org/blog) — current 16.2
- [Astro releases](https://github.com/withastro/astro/releases) and [v6 upgrade guide](https://docs.astro.build/en/guides/upgrade-to/v6/)
- [Svelte 5 migration guide](https://svelte.dev/docs/svelte/v5-migration-guide), [runes intro](https://svelte.dev/blog/runes)
- [Tailwind CSS releases](https://github.com/tailwindlabs/tailwindcss/releases)
- [React 19 release post](https://react.dev/blog/2024/12/05/react-19), [Server Components reference](https://react.dev/reference/rsc/server-components)
- [React Router modes](https://reactrouter.com/start/modes)
- [Vite 8 announcement](https://vite.dev/blog/announcing-vite8), [Rolldown 1.0](https://voidzero.dev/posts/announcing-rolldown-1-0)
- [State of JS 2024 — frameworks](https://2024.stateofjs.com/en-US/libraries/front-end-frameworks/)
- [State of CSS 2024 — tools](https://2024.stateofcss.com/en-US/tools/) ([CSS-Tricks recap](https://css-tricks.com/state-of-css-2024-results/))
- [Webflow makes GSAP free](https://webflow.com/blog/gsap-becomes-free)
- [Motion changelog](https://motion.dev/changelog)
- [shadcn/ui changelog](https://ui.shadcn.com/docs/changelog)
- [Better Auth vs Clerk vs NextAuth (LogRocket, 2026)](https://blog.logrocket.com/best-auth-library-nextjs-2026/)
- [Lucia auth deprecation](https://www.wisp.blog/blog/lucia-auth-is-dead-whats-next-for-auth)
- [Headless CMS comparison 2026](https://www.digitalapplied.com/blog/headless-cms-2026-sanity-contentful-payload-comparison)
- [PlanetScale Hobby deprecation FAQ](https://planetscale.com/docs/plans/hobby-plan-deprecation-faq)
- [Polar vs Lemon Squeezy vs Creem 2026](https://devtoolpicks.com/blog/polar-vs-lemon-squeezy-vs-creem-2026)
- [Biome](https://biomejs.dev/)

---
*Last updated: 2026-05-28 | Verified against live sources 2026-05-28 | Volatile — refresh quarterly*
