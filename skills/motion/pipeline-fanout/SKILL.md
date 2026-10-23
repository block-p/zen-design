---
name: pipeline-fanout
title: Pipeline Fan-out
category: explainer
tier: premium
engine: hyperframes
description: One input flows through a process and fans out to many outputs.
url: https://claude-motion.com/skills/pipeline-fanout
---

# Pipeline Fan-out

One input flows through a process and fans out to many outputs.

### Pipeline Fan-out

- Page: https://claude-motion.com/skills/pipeline-fanout · Install: https://claude-motion.com/skills/pipeline-fanout/install.md
- premium · Explainer · hyperframes · 16:9, 9:16, 1:1 · 8s
- Last checked: 2026-10-14

Input, process and outputs appear in sequence, connected by curves. The process node lights up in the accent, then signal pulses travel in and fan out along every branch, outlining each output as they arrive. Horizontal in 16:9, vertical in 9:16 and 1:1.

Good fit: One input turning into several outputs; Explaining how a system distributes work.
Not for: Many inputs merging into one (use Converge to Result); Long multi-stage pipelines.
Inputs: title, input, process, outputs, theme (ink | paper | electric | noir | sand), fontDisplay (serif | sans | grotesk | mono).

Example prompt: Use the pipeline-fanout skill: input "Webhook: order.created", process "Router: rules", outputs "Slack: #sales | Email: customer | CRM: deal". Paper theme.

Claude Motion prompt: Make an 8-second 16:9 pipeline diagram for [who will watch, e.g. engineers evaluating us], playing [where, e.g. the docs overview]. Use only the labels below. Near-black background.

The title appears. A dark input card appears on the left, a gray line draws to the middle card which turns bright orange, then four curved lines fan out to four output cards on the right. Glowing orange dots travel from the input into the orange node and out along every branch, briefly outlining each output card in orange. End frame: the pulses repeat once and settle.

Title: [e.g. "One brief, every format"]
Input, middle step and four outputs: [e.g. "Brief — one paragraph" → "Agent + skill — renders" → "16:9 YouTube", "9:16 Reels", "1:1 Feed", "GIF Docs"]

