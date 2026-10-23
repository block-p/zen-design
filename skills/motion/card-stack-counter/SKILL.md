---
name: card-stack-counter
title: Card Stack Counter
category: explainer
tier: premium
engine: hyperframes
description: Cards drop in one by one while a big total adds them up.
url: https://claude-motion.com/skills/card-stack-counter
---

# Card Stack Counter

Cards drop in one by one while a big total adds them up.

### Card Stack Counter

- Page: https://claude-motion.com/skills/card-stack-counter · Install: https://claude-motion.com/skills/card-stack-counter/install.md
- premium · Explainer · hyperframes · 16:9, 9:16, 1:1 · 7s
- Last checked: 2026-10-14

Each card carries a label and a value. As each one lands in the column, the giant running total counts up to include it, and finally turns the accent color. Values sharing a unit are summed: hours, dollars, tickets, anything.

Good fit: Adding up costs, hours or savings; Building to one big total.
Not for: Values in different units; Comparisons where the total doesn't matter.
Inputs: cards, totalLabel, title, theme (ink | paper | electric | noir | sand), fontDisplay (serif | sans | grotesk | mono).

Example prompt: Use the card-stack-counter skill: cards "Design tool: $45 | Video editor: $30 | Stock music: $20 | Captions app: $15", total label "per month, replaced". Paper theme.

Claude Motion prompt: Make a 7-second 16:9 scene for [who will watch, e.g. managers we're selling to], playing [where, e.g. the sales deck]. Use only the items and hours below; the total must equal their sum. Near-black background.

The title sits at the top left. Dark rounded cards drop into a column one at a time, each with an item and its value in orange. Beside them a huge off-white number counts up as each card lands, with a gray caption beneath. End frame: when the last card lands, the total turns orange.

Title: [e.g. "Where the week goes"]
Items and values: [e.g. "Status meetings 6h", "Email triage 4h", "Chasing updates 3h", "Reporting 2h"]
Caption: [e.g. "lost every week"]

