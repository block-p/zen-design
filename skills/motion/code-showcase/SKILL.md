---
name: code-showcase
title: Code Showcase
category: dev
tier: premium
engine: hyperframes
description: Typed code with syntax colors, highlights and a terminal run.
url: https://claude-motion.com/skills/code-showcase
---

# Code Showcase

Typed code with syntax colors, highlights and a terminal run.

### Code Showcase

- Page: https://claude-motion.com/skills/code-showcase · Install: https://claude-motion.com/skills/code-showcase/install.md
- premium · Developer · hyperframes · 16:9, 9:16, 1:1 · 14s
- Last checked: 2026-10-14

An editor types your snippet with syntax colors, dims everything but the lines that matter, then a terminal runs a command and prints results, ending on a headline. For SDKs, APIs and dev tools.

Good fit: SDKs, APIs and developer tools; Showing a short snippet and its output.
Not for: Long code files; Non-technical audiences.
Inputs: filename, code, highlight, command, output, caption, theme (ink | paper | electric | noir | sand), fontDisplay (serif | sans | grotesk | mono).

Example prompt: Use the code-showcase skill with this Python snippet from our README (paste it as `code`), filename "quickstart.py", highlight "3-6", command "python quickstart.py", output "✓ Connected | ✓ 1,204 rows synced", caption "Sync any database in 6 lines".

Claude Motion prompt: Make a 14-second 16:9 code demo for [who will watch, e.g. developers], playing [where, e.g. our docs and X]. Use only the code, command and headline I give below; don't change a character. Dark navy background.

A rounded editor window with three dots and a file tab rises in. The code types out character by character with a violet block caret and syntax colors (violet keywords, teal strings, gray italic comments, orange numbers). The key lines then get a translucent violet highlight with a left border while the other lines dim. A terminal below types the command and prints green check lines. End frame: the editor fades and the headline appears.

File name and code: [your snippet, 8 to 12 lines] (e.g. deploy.ts)
Lines to highlight: [e.g. 4–8]
Command: [e.g. "$ npx northwind deploy"]
Headline: [e.g. "From prompt to production in one call"]

