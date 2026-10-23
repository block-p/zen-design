# AIEO / GEO — Foundations

## What is AIEO / GEO

**AIEO** (AI Engine Optimization) — also called **GEO** (Generative Engine Optimization) — is the discipline of getting your brand, content, and product cited *inside the answers* of AI search engines: ChatGPT search, Claude, Perplexity, Gemini, Google AI Overviews, Bing Copilot. The term was formally introduced in the Princeton/Georgia Tech/Allen Institute paper "GEO: Generative Engine Optimization" published at KDD 2024, which demonstrated GEO methods can lift citation visibility up to 40% [Source](https://collaborate.princeton.edu/en/publications/geo-generative-engine-optimization/).

It overlaps with SEO but has different mechanics:
- **SEO** optimizes for ranking in a list of links a user chooses from
- **AIEO** optimizes for being cited inside a synthesized answer the user reads

## Why it now matters more than SEO for some businesses

1. **Users don't always click.** AI answers absorb informational queries.
2. **Citations inside AI answers drive traffic that bypasses SERPs.** A Perplexity citation can convert better than a Google #1.
3. **Brand mentions inside AI responses build awareness** without a click.
4. **New KPIs emerging**: citation rate, share-of-voice in AI answers, sentiment in AI responses, branded-query consistency.

## How AI engines retrieve and cite

### Two modes

1. **Training-data knowledge** (closed) — what the model "knows" from its training cutoff
   - Frozen at cutoff; updates require re-training or fine-tuning
   - Brands established in pre-training data have an advantage
2. **Live retrieval / RAG** (Retrieval-Augmented Generation) — fetches sources at query time
   - ChatGPT Search, Perplexity, Gemini, Claude (when web search is enabled), Bing Copilot, Google AI Overviews
   - Citations link back to sources

### Which engines crawl live (2026)
Major providers now separate **training** crawlers, **search-index** crawlers, and **user-triggered** fetchers — each controllable independently in robots.txt.

- **OpenAI** — three crawlers: `GPTBot` (training), `OAI-SearchBot` (ChatGPT search index), `ChatGPT-User` (user-initiated fetches), plus newer `OAI-AdsBot` for ad landing-page validation [Source](https://developers.openai.com/api/docs/bots)
- **Anthropic** — three crawlers: `ClaudeBot` (training), `Claude-SearchBot` (search index, added 2025), `Claude-User` (user-triggered) — blocking one does NOT block the others [Source](https://searchengineland.com/anthropic-claude-bots-470171)
- **Perplexity** — `PerplexityBot` (indexing) and `Perplexity-User` (user-triggered)
- **Google** — `Google-Extended` is an opt-out token (not a crawler) for Gemini/Vertex training; AI Overviews and AI Mode use the normal Googlebot index
- **Apple** — `Applebot-Extended` is an opt-out token for Apple Intelligence training; `Applebot` itself crawls for Siri/Spotlight
- **Bing Copilot** — `Bingbot` index plus Microsoft-specific AI fetchers
- **Meta AI** — `Meta-ExternalAgent` (climbed to ~17% of AI bot traffic by March 2026) [Source](https://websearchapi.ai/blog/monthly-ai-crawler-report)
- **ByteDance** — `Bytespider` (undocumented; grew 138% Nov 2025–Feb 2026 — now ~⅓ of AI training crawl activity)
- **Amazon** — `Amazonbot`
- **Mistral** — `MistralAI-User`
- **DuckDuckGo** — `DuckAssistBot`
- **You.com** — `YouBot`

Key 2026 shift: Applebot-Extended traffic jumped +124% in March 2026 (2.6% → 5.8% of AI bot traffic) as Apple expanded Apple Intelligence; xAI's Grok crawler still operates without a declared user-agent [Source](https://nohacks.co/blog/ai-user-agents-landscape-2026).

### Citation factors (empirical, 2026)
Cyrus Shepard's analysis of AI citation correlations found **URL accessibility (9.5/10)** and **search rank (9.4/10)** are the two strongest predictors — meaning traditional SEO fundamentals still underpin AI visibility [Source](https://ppc.land/google-traffic-winners-in-2026-5-data-backed-traits-that-set-them-apart/). His follow-up data showed only **2% of cited URLs appear across AI Overviews, ChatGPT, AND Perplexity** — **91% of citations are engine-specific**, so per-engine optimization matters.

- **Authority + search rank** — high correlation with citation; AI surfaces lean heavily on existing ranking signals
- **URL accessibility** — crawlable, indexable, no JS-only rendering
- **Direct-answer paragraphs** in BLUF (bottom-line-up-front) format — first 2–4 sentences after a heading
- **Question-shaped headings** that mirror likely user queries
- **Specific numbers, dates, named entities** (extractable facts)
- **Content freshness** — a "3-month citation cliff" is emerging; refresh dates matter [Source](https://aithinkerlab.com/generative-engine-optimization-2026/)
- **Structured data** (schema.org) — FAQ schema pages are ~3.2× more likely to be cited in Perplexity [Source](https://www.trysight.ai/blog/how-perplexity-ai-selects-sources)
- **Wikipedia presence** — outsize influence on training data and retrieval
- **Off-domain brand mentions** — Reddit and LinkedIn are now the most-cited domains in AI answers (see below)

### Per-engine: Perplexity citation weighting (2026)
Empirical breakdown of Perplexity's reranking signals: content relevance ~30%, visual placement ~20%, domain authority ~15%, content freshness ~15%, source diversity ~10%, structured data ~10%. **94% of Perplexity answers contain at least one inline citation**; average 8.2 sources per answer (range 5–12) [Source](https://www.stackmatix.com/blog/perplexity-ai-optimization-strategy).

## llms.txt and emerging standards

### llms.txt
- Proposed by Jeremy Howard (Answer.AI) in September 2024 at [llmstxt.org](https://llmstxt.org/) — a markdown file at `/llms.txt` and optionally `/llms-full.txt`
- Tells LLMs how to consume your site (similar in spirit to robots.txt for crawlers, sitemap.xml for indexers)
- **Adoption status as of 2026:** ~10.13% of 300K domains have one per SE Ranking — but the practical impact is limited. Studies show **GPTBot, ClaudeBot, PerplexityBot, OAI-SearchBot, and Google-Extended overwhelmingly skip the file** and crawl HTML directly [Source](https://presenc.ai/research/state-of-llms-txt-2026)
- **Who actually uses it:** Anthropic confirmed support late 2024, Perplexity mid-2025. Google's Gary Illyes confirmed July 2025 that Google does NOT support llms.txt and isn't planning to. No major LLM provider (OpenAI, Anthropic, Google, Meta, Mistral) has committed to using it as a *production* answer-surface signal [Source](https://codersera.com/blog/llms-txt-complete-guide-2026/)
- **Where it does matter:** IDE coding agents (Cursor, Continue, Cline, Aider), MCP servers for docs, dev-doc platforms (Mintlify, GitBook, Vercel Docs, Supabase) — Mintlify auto-generated llms.txt for Anthropic, Cursor, Coinbase, Pinecone, Windsurf starting Nov 2024 [Source](https://www.mintlify.com/library/best-llms-txt-platforms)
- Format:
  ```
  # Project Name
  
  > One-sentence project description.
  
  Longer paragraph explaining what the project is, who it's for, what makes it different.
  
  ## Docs
  - [Quickstart](https://example.com/docs/quickstart): One-line description
  - [API Reference](https://example.com/docs/api): One-line description
  
  ## Optional
  - [Changelog](https://example.com/changelog): Updates
  ```
- `llms-full.txt` is the same idea but contains the full content (so the LLM doesn't need to fetch each link)

### robots.txt for AI crawlers
- You can selectively allow/block each AI crawler — crucially, **blocking a training bot does NOT block the search-index or user-fetcher bot** from the same provider
- **The trade-off:** blocking the search/user crawlers = invisible in AI answers. Most brands should allow them
- Recommended split: allow `OAI-SearchBot` / `Claude-SearchBot` / `PerplexityBot` (search) while choosing per-brand whether to allow `GPTBot` / `ClaudeBot` (training)

### Crawler list — verified 2026
Token strings as they should appear in `robots.txt` (`User-agent:`), with full UA fragments where useful.

| Token | Provider | Purpose | Notes |
|---|---|---|---|
| `GPTBot` | OpenAI | Training | UA: `...GPTBot/1.3; +https://openai.com/gptbot` [Source](https://developers.openai.com/api/docs/bots) |
| `OAI-SearchBot` | OpenAI | ChatGPT search index | UA: `...OAI-SearchBot/1.3; +https://openai.com/searchbot` |
| `ChatGPT-User` | OpenAI | User-initiated fetch | robots.txt may not apply (user-triggered) |
| `OAI-AdsBot` | OpenAI | Ad landing-page check | Not used for training |
| `ClaudeBot` | Anthropic | Training | UA: `Mozilla/5.0 (compatible; ClaudeBot/1.0; ...)` [Source](https://searchengineland.com/anthropic-claude-bots-470171) |
| `Claude-SearchBot` | Anthropic | Claude search index | Added 2025; independent of ClaudeBot |
| `Claude-User` | Anthropic | User-triggered fetch | Independent of the other two |
| `PerplexityBot` | Perplexity | Indexing | UA: `...PerplexityBot/1.0; +https://perplexity.ai/perplexitybot` |
| `Perplexity-User` | Perplexity | User-triggered fetch | |
| `Google-Extended` | Google | Opt-out token (Gemini/Vertex training) | NOT a crawler; UA token only |
| `Googlebot` | Google | Indexing (powers AI Overviews/AI Mode) | Blocking this kills AI Overviews citations |
| `Applebot-Extended` | Apple | Opt-out token (Apple Intelligence training) | NOT a crawler |
| `Applebot` | Apple | Siri/Spotlight indexing | |
| `Bingbot` | Microsoft | Indexing (Copilot) | |
| `Meta-ExternalAgent` | Meta | Training | UA: `meta-externalagent/1.1` |
| `Amazonbot` | Amazon | Indexing/Alexa | |
| `Bytespider` | ByteDance | Training (undocumented) | Often blocked by default |
| `MistralAI-User` | Mistral | User-triggered fetch | |
| `DuckAssistBot` | DuckDuckGo | DuckAssist answers | |
| `CCBot` | Common Crawl | Open dataset used by many LLMs | |

**Sensible 2026 default for most brands:**
```
User-agent: GPTBot
User-agent: OAI-SearchBot
User-agent: ChatGPT-User
User-agent: ClaudeBot
User-agent: Claude-SearchBot
User-agent: Claude-User
User-agent: PerplexityBot
User-agent: Perplexity-User
User-agent: Google-Extended
User-agent: Applebot-Extended
User-agent: Meta-ExternalAgent
User-agent: Amazonbot
Allow: /

User-agent: Bytespider
User-agent: CCBot
Disallow: /
```
Source for crawler strings: [No Hacks — AI User-Agent Landscape 2026](https://nohacks.co/blog/ai-user-agents-landscape-2026), [Anthropic docs](https://searchengineland.com/anthropic-claude-bots-470171), [OpenAI bots page](https://developers.openai.com/api/docs/bots).

## Content patterns that get cited

### Direct answers near the top
For each section, the first 2–3 sentences should be a self-contained answer. AI engines pull these.

❌ "Many factors influence pricing strategy in B2B SaaS. There's no one-size-fits-all approach. Let's explore..."

✅ "Most early-stage B2B SaaS startups should price between $50–500/month per seat for their main plan. Below $50, the unit economics rarely work; above $500, you need enterprise sales motion."

### Question-shaped headings
- "What is X?" beats "X overview"
- "How to do Y" beats "Y guide"
- Headings mirror queries

### Specific over vague
- "Founded in 2021 by ex-Stripe engineers" > "Founded recently by veteran founders"
- "85% of Fortune 500 use X" > "Many enterprises use X"
- "Updated October 2025" > "Recent updates"

### Q&A and FAQ structures
- AI engines love these — pre-extracted question/answer pairs
- Use FAQPage schema (still helps even if Google deprecated the rich result)

### Comparison pages
- "X vs Y" pages get cited disproportionately
- Cover the comparison fairly — biased pages don't get cited

### Lists and tables
- Structured data the model can extract cleanly

### Recency signals
- `<time>` elements, "Last updated" dates, regular content refreshes
- AI engines weight recent sources for time-sensitive topics

## Structured data for AI

### High-value schemas for AIEO
- **Organization** — homepage; pulls into knowledge panels
- **Person** — author pages, About pages with credentials
- **Product** — products with price, availability, ratings
- **FAQPage** — question/answer pairs
- **HowTo** — step-by-step content
- **Article / BlogPosting** — with `author`, `datePublished`, `dateModified`
- **BreadcrumbList**

### Knowledge graph alignment
- **Wikipedia article** about your brand or founders has outsize impact on AI knowledge
- **Wikidata entry** linked from Wikipedia
- **Crunchbase, LinkedIn** profiles for the company and people
- **Consistent NAP** (name, address, phone) across the web

## Brand presence outside your domain

This matters more for AIEO than for SEO.

### Wikipedia / Wikidata
- If notable enough, a Wikipedia article significantly boosts AI visibility
- Don't write your own — disclose conflict of interest; submit drafts via legitimate channels

### Podcasts and interviews
- Audio transcripts get indexed
- Mentions in interview podcasts (Acquired, Lenny's, Lex Fridman, etc.) carry weight

### Reddit and Stack Overflow
- **Reddit is the #1 most-cited domain across ChatGPT, AI Mode, Gemini, Perplexity, and AI Overviews** — 3M+ mentions in Profound's 2026 dataset [Source](https://searchengineland.com/ai-search-engines-cite-reddit-youtube-and-linkedin-most-study-473138)
- Reddit struck a data deal with OpenAI and Google
- Genuine community presence (not astroturf) gets cited

### LinkedIn (new — major 2026 shift)
- **#1 most-cited domain for professional queries across all six major AI platforms**, citation frequency doubled between Nov 2025 and Feb 2026 [Source](https://www.tryprofound.com/blog/linkedin-is-the-most-cited-domain-for-professional-queries-in-ai-search)
- 11% of AI responses on average reference a LinkedIn URL (ChatGPT Search 14.3%, Google AI Mode 13.5%, Perplexity 5.3%)
- Feed posts + long-form articles overtook profile pages as the cited content type (profile-page citations fell 33.9% → 14.5% between Nov 2025 and Feb 2026) [Source](https://mediacopilot.ai/linkedin-top-cited-ai-search-profound/)
- Implication: founders/execs publishing long-form LinkedIn posts now compounds AI visibility

### YouTube
- Transcripts are indexed by Google (which feeds AI Overviews and Gemini)
- Video chapters with question-shaped titles help

### LinkedIn presence for personnel
- Helps validate E-E-A-T claims
- Founders, leadership, key engineers — visible profiles

## Measurement

### Manual sampling (best current method)
Weekly: ask 5 AI engines (ChatGPT, Claude, Perplexity, Gemini, Copilot) a rotating set of branded and topic queries. Record:
- Was your brand cited? (yes/no per engine)
- Was the citation accurate? (yes/no)
- Sentiment of the citation (positive/neutral/negative)
- Which sources were cited *alongside* or *instead of* you

### Tools — verified active 2026
Category collectively raised $200M+ by early 2026. Per HubSpot/Capston/Rankability comparisons [Source](https://capston.ai/geo-tools/), [Source](https://www.rankability.com/blog/best-ai-search-visibility-tracking-tools/):
- **Profound** ([tryprofound.com](https://www.tryprofound.com)) — enterprise, $399+/mo; appears in 74% of buyer queries
- **AthenaHQ** ([athenahq.ai](https://athenahq.ai)) — enterprise/mid-market, $270/mo; outperformed Profound in a 30-day head-to-head answer-share test
- **Peec AI** — mid-market, €85/mo
- **Otterly.ai** — SMB, $29/mo; simple mention tracking
- **Scrunch AI**
- **Brandlight**
- **Evertune**
- **AirOps**
- **BrandRank.AI** — still alive; brand-focused
- **Conductor GPT Visibility** — enterprise
- **HubSpot AI Search Grader** — free
- **Semrush AI Toolkit** and **Ahrefs Brand Radar** — built into existing SEO suites
- *Note: "Goodie" and "peecai" from earlier industry lists appear to have been renamed/absorbed — Peec AI is the current entity*

### Inside Google Analytics
- AI-engine traffic appears as referral from `chatgpt.com`, `perplexity.ai`, `gemini.google.com`, `claude.ai`, `copilot.microsoft.com`
- Set up custom segments to track

## Strategy by content type

### Marketing site / homepage
- Crystal-clear one-line description (gets used in "what is X" answers)
- Founder credentials, founding year, company stage visible
- Use cases listed concretely

### Documentation
- Question-shaped page titles
- Direct answers per page
- llms.txt to surface key pages

### Blog
- First paragraph is a direct answer to the title's implicit question
- Date published + date modified
- Named author with credentials

### Glossary / definitions
- HUGE AIEO wins — AI engines love clean definitions
- One concept per page, definition in the first sentence

### Comparison pages
- "Brand vs Competitor" pages get cited disproportionately
- Be fair — biased comparisons hurt long-term

### Case studies
- Specific outcomes, named clients, real numbers
- AI engines extract these as evidence

## llms.txt example for a SaaS

```
# Acme — Customer feedback for product teams

> Acme helps product teams collect, organize, and act on customer feedback from every channel.

Acme is a customer feedback platform for B2B SaaS product teams. Founded in 2023 by ex-Linear engineers, used by ~500 SaaS companies including [Notable Customer 1] and [Notable Customer 2]. Acme integrates with Slack, Linear, Jira, and Intercom.

## Product
- [Overview](https://acme.com/): What Acme does and who it's for
- [Features](https://acme.com/features): Full feature list
- [Pricing](https://acme.com/pricing): $29/mo Starter, $99/mo Team, custom Enterprise
- [Changelog](https://acme.com/changelog): Recent updates

## Docs
- [Quickstart](https://acme.com/docs/quickstart): 5-minute setup
- [API Reference](https://acme.com/docs/api): REST and GraphQL
- [Integrations](https://acme.com/docs/integrations): All supported tools

## Compare
- [Acme vs Linear](https://acme.com/vs/linear): How we differ
- [Acme vs Productboard](https://acme.com/vs/productboard): How we differ

## Resources
- [Blog](https://acme.com/blog): Product feedback best practices
- [Templates](https://acme.com/templates): Feedback workflow templates
```

## Anti-patterns

- **Blocking all AI crawlers** if you want citations
- **Wall-of-text content** without headings or structure
- **No named author / no credentials** on E-E-A-T-sensitive topics
- **Outdated content** (last updated 2022) on time-sensitive topics
- **Cookie banner blocking content** for the crawler (some crawlers don't accept cookies)
- **Heavy JS rendering** without SSR — many AI crawlers don't execute JS
- **Astroturf / fake reviews** — fragile, eventually backfires
- **Keyword stuffing for AI** — same penalty mechanics as SEO

## Open questions

- Will AI engines weight content behind paywalls?
- Will citation rates stabilize as AI search matures, or keep churning?
- How much weight do non-textual signals (video transcripts, podcasts) carry?
- Will brands need to publish to specific "trusted" sources to be cited?

## Sources

**Foundational research**
- [GEO: Generative Engine Optimization — Princeton/Georgia Tech/Allen Institute, KDD 2024](https://collaborate.princeton.edu/en/publications/geo-generative-engine-optimization/)
- [llmstxt.org — Jeremy Howard's original proposal](https://llmstxt.org/)

**Crawler specs (official)**
- [OpenAI bots overview](https://developers.openai.com/api/docs/bots)
- [Anthropic Claude bots — Search Engine Land coverage of official docs](https://searchengineland.com/anthropic-claude-bots-470171)
- [No Hacks — AI User-Agent Landscape 2026 (consolidated reference)](https://nohacks.co/blog/ai-user-agents-landscape-2026)

**2026 citation behavior research**
- [Profound: LinkedIn #1 cited for professional queries](https://www.tryprofound.com/blog/linkedin-is-the-most-cited-domain-for-professional-queries-in-ai-search)
- [Search Engine Land: AI engines cite Reddit, YouTube, LinkedIn most](https://searchengineland.com/ai-search-engines-cite-reddit-youtube-and-linkedin-most-study-473138)
- [Monthly AI Crawler Report — March 2026 (Bytespider, Meta, Applebot growth)](https://websearchapi.ai/blog/monthly-ai-crawler-report)
- [Presenc AI: State of llms.txt 2026](https://presenc.ai/research/state-of-llms-txt-2026)
- [Codersera: llms.txt honest guide (May 2026)](https://codersera.com/blog/llms-txt-complete-guide-2026/)

**Practitioner analysis**
- [Mike King (iPullRank) on Google's AI search guidance, May 2026](https://ppc.land/mike-king-googles-ai-search-guide-serves-platform-over-the-open-web/)
- [Mike King — Beyond RAG / Agentic-RAG content engineering](https://ipullrank.com/agentic-rag)
- [Cyrus Shepard — Google traffic winners 2026 (5 features)](https://ppc.land/google-traffic-winners-in-2026-5-data-backed-traits-that-set-them-apart/)
- [Lily Ray — what the SEO industry gets wrong about AI search](https://ppc.land/lily-ray-what-the-seo-industry-is-getting-dangerously-wrong-about-ai-search/)
- [Perplexity citation factors breakdown — Stackmatix](https://www.stackmatix.com/blog/perplexity-ai-optimization-strategy)
- [How Perplexity selects sources — Trysight](https://www.trysight.ai/blog/how-perplexity-ai-selects-sources)

**Tools / market**
- [Capston: Best GEO tools 2026 comparison](https://capston.ai/geo-tools/)
- [Rankability: AI visibility tools 2026](https://www.rankability.com/blog/best-ai-search-visibility-tracking-tools/)
- [HubSpot: Profound vs AthenaHQ](https://blog.hubspot.com/marketing/profound-vs-athenahq)

---
*Last updated: 2026-05-28 | Verified against live sources 2026-05-28 | Highly volatile — refresh monthly*
