---
name: ui-morph-effects
title: UI Morph Effects
category: launch
tier: premium
engine: hyperframes
description: A button becomes a player, a search box fills with results: one object changing shape.
url: https://claude-motion.com/skills/ui-morph-effects
---

# UI Morph Effects

A button becomes a player, a search box fills with results: one object changing shape.

### UI Morph Effects

- Page: https://claude-motion.com/skills/ui-morph-effects · Install: https://claude-motion.com/skills/ui-morph-effects/install.md
- premium · Launch · hyperframes · 1:1, 9:16, 16:9 · 8s loop
- Last checked: 2026-10-14

Four product-UI moments as 8-second loops: a Preview button that grows into a video player, a search pill that opens, types a query and stages in results, a small card that unfolds into its editor while its title travels into the title bar, and a tab pill that stretches to the next tab as the panel swaps row by row. Each is one object changing shape, with a ghost of where it started and something small alive in the hold. Loop mode for posts and GIFs, hold mode for use under a voiceover.

Good fit: Feature announcements and landing-page moments; Product posts and looping GIFs.
Not for: Full product walkthroughs; Showing your real app's screens as they are.
Inputs: effect (player | search | workspace | tabs), mode (loop | hold), start, title, items, before, meta, tag, theme (ink | paper | electric | noir | sand), fontDisplay (serif | sans | grotesk | mono).

Example prompt: Use the ui-morph-effects skill with the tabs effect for Ledger: tabs "Monthly | Yearly | Lifetime", second tab title "Yearly plan", rows "Every template: included | Commercial use: included | Updates: 12 months", 1:1, my accent #2563eb, no Example tag.

Claude Motion prompt: Make an 8-second 16:9 seamless loop for [who will watch, e.g. designers], playing [where, e.g. the hero of our product page]. Use only the query and three results below. Near-black background.

A small search pill with a magnifier and the gray word "Search" widens to full width, a caret appears and the query types itself. Three result cards rise into place one after another, each with a colored tile, a bold title and one metric. An orange hover outline moves from the first result to the second. End frame: at 6.6 seconds everything reverses, so the last frame matches the first.

Query: [e.g. "spring campaign"]
Three results: [title · metric] (e.g. "Spring launch email · 48% opens")

---

# Guides

Short answer: Claude Motion is in beta on Claude **Team** and **Enterprise** plans. It isn't sold separately; it uses your plan's normal usage. It isn't listed for Free, Pro or Max.

## Which plans include Claude Motion

| Plan | Claude Motion |
|---|---|
| Free | Not included |
| Pro | Not included |
| Max | Not included |
| Team | Included in beta, **on by default** |
| Enterprise | Included in beta, **off until an owner turns it on** |

Availability can change while Motion is in beta. Anthropic's [help article](https://support.claude.com/en/articles/17454997-get-started-with-claude-motion) is the source of truth.

## How to turn it on (Enterprise)

An organization owner goes to **Organization settings → Artifacts** and enables Motion. Enterprise owners can also limit access to specific groups with custom roles. Admins control whether animations can be shared outside the organization.

## How to check if you have it

Open a chat and look for **Output → Motion** in the message box, or open the **Artifacts** tab and look for Motion templates. If neither is there, your plan doesn't include it or your admin hasn't enabled it.

## What it costs

There's no separate price. Motion counts toward your plan's usage limits, and longer or more complex animations use more of your limit than a typical message. In practice:

- Keep first drafts short (8–15 seconds), then extend.
- Ask for specific edits ("change this number") rather than regenerating the whole animation.
- Reuse a prompt that already works and swap in new words and numbers.

## If you don't have access

You don't need Team or Enterprise to make this kind of video. Motion skills give a coding agent the same ability:

- **Works with:** Claude Code (including on Pro and Max), Codex, Cursor, Gemini CLI and other agents that read skills.
- **How:** install a skill, give your agent your material, and it renders an MP4 on your computer with an open-source engine.
- **Cost:** free skills need only a Google sign-in, no credits; the only usage is your agent's own.

Start with [Motion Director](/skills/motion-director): it's free and makes a whole video from one sentence. For a specific kind of video, browse [Motion Launch](/launch), [Motion Creator](/creator) or [Motion Deck](/narrated-presentations), or copy a ready-made prompt from the [Claude Motion prompts page](/claude-motion-prompts). For a walkthrough of the feature itself, see [How to use Claude Motion](/learn/how-to-use-claude-motion).

*This guide is independent and not affiliated with Anthropic. Claude is a trademark of Anthropic, PBC. Checked against Anthropic's announcement and help article on October 12, 2026.*

Claude Motion is a beta feature Anthropic launched on October 8, 2026. You describe a short animated explainer, and Claude builds it as code that animates your text, charts, shapes and images. You can edit any word, number or timing, then export an MP4. It is not a video-generation model: there's no generated footage and no AI-generated people.

## Who can use it

- **Plans:** Claude Team and Enterprise, in beta. On Enterprise, an owner has to switch it on in the organization settings.
- **Not included (yet):** Free, Pro and Max plans.
- **Good for:** a 30-second explainer of a quarterly report, motion on a board-deck chart, a product walkthrough for onboarding.

The official announcement and help article are the source of truth for availability: [Anthropic's announcement](https://claude.com/resources/articles/dashboards-and-motion) and [Get started with Claude Motion](https://support.claude.com/en/articles/17454997-get-started-with-claude-motion).

Plan-by-plan details and how to turn it on: [Claude Motion access & pricing](/learn/claude-motion-access-pricing).

## Writing a good Claude Motion prompt

Anthropic's advice is short: say who will watch, where it will play and roughly how long it should be, and attach your own report, data or images. In practice that means four things:

1. **Who and where:** "for investors, in the monthly update" or "muted in the LinkedIn feed".
2. **Length and format:** "a 20-second 16:9 video".
3. **Your content only:** attach the report, data or images and say "use only this; don't change a number".
4. **Scenes in order, then the end frame:** what appears first, what moves, what the last frame shows.

Every prompt on this site follows that pattern, with [brackets] for your own words and an example in each. Step by step, from first prompt to MP4: [How to use Claude Motion](/learn/how-to-use-claude-motion). Ready-made prompts with previews: [Claude Motion prompts](/claude-motion-prompts).

## Three things people call "Claude motion"

1. **Claude Motion**, the feature: Team and Enterprise, in claude.ai, built from your attached content, exported as MP4.
2. **Opus 5.5 in a plain chat or in Claude Code**, asked to write an animation as an HTML, Canvas or Three.js page. Most "Claude motion prompts" shared on X are this kind, including the showreel prompts. It works on any plan, but you record the page yourself to get a video.
3. **Third-party tools** with similar names. They aren't Anthropic products, and their prompts may not behave the same in Claude Motion.

Our [prompts page](/claude-motion-prompts) labels each prompt with where it runs.

## If you don't have Team or Enterprise

You can get the same kind of result from a coding agent you already use:

1. Install a motion skill in Claude Code, Codex or Cursor.
2. Ask your agent for the video in plain language.
3. The agent fills the template, checks it, reviews still frames and renders an MP4 on your machine with an open-source engine such as [HyperFrames](https://github.com/heygen-com/hyperframes).

[Browse the skills](/skills) — free ones download after a free Google sign-in.

## Claude Motion vs. agent skills

| | Claude Motion | Agent skills |
|---|---|---|
| Where it runs | claude.ai | Your coding agent and computer |
| Plans | Team, Enterprise | Any agent that supports skills |
| Editing | In Claude, as code | Variables, or edit the HTML/CSS |
| Output | MP4 | MP4, MOV with transparency, GIF |
| Repeatable templates | Prompt by prompt | Same template, new values every time |

*This guide is independent and not affiliated with Anthropic. Claude is a trademark of Anthropic, PBC.*

Claude Motion turns a description into a short animated video that Claude builds as code. This tutorial walks through making your first one, writing prompts that work, editing the result and exporting an MP4. It also covers how to get the same kind of video in Claude Code or Codex if your plan doesn't include Motion.

## Before you start

Claude Motion is in beta on Claude **Team** and **Enterprise** plans. It's on by default for Team. On Enterprise, an owner has to turn it on first under **Organization settings → Artifacts**. If you're not sure whether you have it, see [Claude Motion access & pricing](/learn/claude-motion-access-pricing).

## 1. Start a motion

There are two ways in:

- **From a chat.** Describe the animation you want, for example "a 30-second explainer of our Q3 results for the all-hands". You can also pick **Output → Motion** in the message box so Claude knows you want an animation.
- **From the Artifacts tab.** Open Artifacts, choose a Motion template, then describe what you want.

Attach anything Claude should use: a report, a spreadsheet, a chart, screenshots or a logo.

## 2. Write a prompt Claude can follow

Anthropic's own tip is to say who the audience is, where the video will play and roughly how long it should run, and to attach the material Claude should use. Write the prompt in that order:

1. **Who and where:** "for investors, in the monthly update" or "muted in the LinkedIn feed".
2. **Length and format:** "a 20-second 16:9 video".
3. **Your content only:** attach the report, data or images and say "use only this; don't change a number".
4. **Scenes in order, then the end frame:** what appears first, what moves, what the last frame shows.

A weak prompt: *"Make a video about our growth."*

A prompt that works: *"Make a 20-second 16:9 video for investors, playing in our monthly update. Use only the numbers in the attached CSV; don't round them. A line chart of monthly active users draws on from left to right, and when it reaches June a callout pops in with the change since January. End frame: the June number, held for 2 seconds."*

Our [Claude Motion prompts page](/claude-motion-prompts) has prompts written this way for launches, charts, tutorials and social clips, each with a preview and fields for your own details.

## 3. Edit the result

Because the animation is code, you can change one detail without regenerating everything:

- **Ask in the chat:** "Make the second scene a second shorter", "Change 38% to 41%", "Use our brand green".
- **Edit directly** in the Motion editor.

Change one thing at a time. It's easier to see what moved, and easier to undo.

## 4. Export and share

Click **Export** and choose **MP4**. Animations are private by default; use **Share** to share one. Every animation is saved in your Artifacts tab, so you can come back and make a new version later.

Anthropic also lists integrations for taking an animation further in tools like Adobe, Descript, Runway and Luma AI.

## What Claude Motion can't do

Motion doesn't use a video-generation model. It won't create realistic footage or people. It's built for explainers, charts, product walkthroughs and launch clips: content made of text, numbers, shapes and images.

## No Claude Motion? Do it in Claude Code or Codex

If you're on Free, Pro or Max, or you work in another agent, you can get the same kind of video with a motion skill:

1. Start with [Motion Director](/skills/motion-director), which is free, or pick a skill for a specific video from [Motion Launch](/launch), [Motion Creator](/creator) or [Motion Deck](/narrated-presentations).
2. Copy its install prompt into Claude Code, Codex or Cursor.
3. Give your agent your material. It fills the template, checks frames and renders an MP4 on your computer.

Free skills download after a free Google sign-in. Each one also includes its matching Claude Motion prompt, so the same idea works in both places.

*This guide is independent and not affiliated with Anthropic. Claude is a trademark of Anthropic, PBC. Sources: [Anthropic's announcement](https://claude.com/resources/articles/dashboards-and-motion) and [Get started with Claude Motion](https://support.claude.com/en/articles/17454997-get-started-with-claude-motion).*

Both let an agent write video as code and render it to MP4. They differ in what the agent writes and in licensing.

## HyperFrames

- **What you write:** HTML, CSS and GSAP timelines. Agents already write this fluently.
- **Rendering:** a CLI (`npx hyperframes render`) drives headless Chrome and FFmpeg, frame by frame, deterministically.
- **Built for agents:** `check` runs lint, runtime, layout, motion and contrast audits; `snapshot` captures still frames so the agent can look at its own work before rendering.
- **License:** Apache 2.0, free for commercial use. [github.com/heygen-com/hyperframes](https://github.com/heygen-com/hyperframes)

## Remotion

- **What you write:** React components, with frame-based hooks (`useCurrentFrame`).
- **Strengths:** a mature ecosystem, great for data-driven and programmatic video at scale, and server-side rendering on Lambda.
- **License:** free for individuals and companies with up to three people; larger companies need a [company license](https://www.remotion.dev/license).

## Which one we use

Every skill in our first collection runs on HyperFrames. HTML and CSS keep templates readable and easy to edit by hand, the built-in audits catch layout problems early, and the Apache license means there's nothing extra to buy when you use a skill at work.

If your team already builds with React and Remotion, the same motion ideas carry over. The craft (pacing, typography, easing, restraint) is what makes the video, not the engine.

A skill is a folder with a `SKILL.md` file (instructions for the agent) plus templates and scripts. Install it once, and your agent uses it whenever a request matches.

## The fastest way: ask your agent

Copy the install prompt from any skill page and paste it into your agent:

```
Read https://<this site>/skills/<slug>/install.md and follow it to install the "<slug>" skill, then make a first video with it.
```

The agent reads the guide, asks you to download the archive in your browser, unzips it into the right folder and runs a first render.

## Manual install

1. Download the ZIP from the skill page.
2. Unzip it so the folder keeps its name (`<slug>/SKILL.md`):
   - **Claude Code:** `~/.claude/skills/<slug>/`, or `.claude/skills/<slug>/` inside a project
   - **Codex:** `~/.agents/skills/<slug>/`
   - **Cursor:** `.cursor/skills/<slug>/` inside the project
   - **Gemini CLI:** `~/.gemini/skills/<slug>/`
3. Start a new agent session so it picks up the skill.

```bash
mkdir -p ~/.claude/skills
unzip -o ~/Downloads/kinetic-type-title.zip -d ~/.claude/skills/
```

## What you need on your machine

- Node.js 22 or newer
- FFmpeg (`brew install ffmpeg` on macOS)
- Google Chrome (the renderer drives it headlessly)

The HyperFrames CLI installs itself through `npx` the first time a skill renders.

## Using a skill

Just ask: "Make a 9:16 title card that says *Ship faster, sleep better*." The agent copies the template, writes a `brief.json`, runs `hyperframes check`, looks at snapshot frames to catch layout problems, then renders the MP4 and tells you where it is.

Several tools can turn your material into a video. They suit different people, and the honest answer is sometimes "not us". Here is how they compare, and when to pick which.

## At a glance

| | Where it runs | What you get | Best for |
|---|---|---|---|
| **Motion Skills** | Your coding agent (Claude Code, Codex, Cursor), on your computer | Tested workflows and scenes that end in an MP4 | Animated explainers, launches, data stories and shorts from your own material |
| **Claude Motion** | Inside Claude, on Team and Enterprise plans (beta) | A short animated explainer from a prompt | Teams already on those plans |
| **HeyGen** | A web app | AI presenters, voices and translation | Videos with a talking presenter |
| **Remotion on its own** | Your own React project | A framework; you build every scene | Developers who want full control or render at scale |
| **Free open-source skills** | Your coding agent | Prompts and templates; quality varies | Trying an idea quickly, or a starting point to build on |

## HeyGen

HeyGen is a browser app built around AI presenters: a person or avatar on screen, speaking your script in many languages. If your video needs a face and a voice, that's the right kind of tool. Motion Skills makes motion graphics instead (titles, charts, diagrams, captions and screen callouts), and doesn't make avatars. HeyGen also maintains HyperFrames, the open-source engine our skills render with. We're not affiliated with HeyGen.

## Remotion on its own

Remotion lets you write video as React components, and it is excellent when you want total control or need to render thousands of personalised videos on a server. The cost is time: you design every scene, its timing and its layout for each format yourself. Larger companies also need a [company license](https://www.remotion.dev/license). Motion Skills hands your agent finished, tested scenes, so you describe the video instead of building it.

## Free open-source skills

There are free agent skills for motion on GitHub, and some are good. Check what a skill comes with before you rely on it. Things we found matter for a video you can post:

- **Tested themes and formats.** Layouts designed for 16:9, 9:16 and 1:1, not one frame squeezed into the others.
- **Checks before rendering.** The agent looks at still frames, and a director workflow shows you a storyboard first.
- **Sound.** Music and effects that fit the cut.
- **Workflows.** Going from an article, a spreadsheet or a repository to a finished video, not just one scene.
- **A clear license** for using the result at work.

Our [standard](/standard) lists what every skill here has to pass.

## When to pick which

- You need a talking presenter or dubbing: **HeyGen**.
- You're on Claude Team or Enterprise and want a quick explainer: **Claude Motion**.
- You're a developer building a video product or rendering at scale: **Remotion**.
- You want to see what an agent can do, for free: a **free open-source skill**, or one of [our free skills](/skills?tier=free).
- You want a finished, on-brand video from your own material, made in the agent you already use: **Motion Skills**. Start with a [workflow](/use-cases), and see [pricing](/pricing) for the premium scenes.

