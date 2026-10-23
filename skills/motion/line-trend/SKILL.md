---
name: line-trend
title: Line Trend
category: explainer
tier: free
engine: hyperframes
description: A line draws with a running value, and one moment gets annotated.
url: https://claude-motion.com/skills/line-trend
---

# Line Trend

A line draws with a running value, and one moment gets annotated.

### Line Trend

- Page: https://claude-motion.com/skills/line-trend · Install: https://claude-motion.com/skills/line-trend/install.md
- free · Explainer · hyperframes · 16:9, 9:16, 1:1 · 7s
- Last checked: 2026-10-14

The line draws left to right over a soft gradient fill while a glowing dot rides its tip and the big number tracks the value under it. When the line reaches the point you name, a ring and a pinned label annotate what happened there.

Good fit: A trend over time with one key moment; Growth stories and metric updates.
Not for: Several lines at once; Precise data reading with axes and gridlines.
Inputs: title, data, prefix, suffix, mark, note, theme (ink | paper | electric | noir | sand), fontDisplay (serif | sans | grotesk | mono).

Example prompt: Use the line-trend skill: title "MRR", data "Q1: 12 | Q2: 15 | Q3: 22 | Q4: 41", prefix "$", suffix "k", mark "Q3", note "Launched video ads". Paper theme, 1:1.

Claude Motion prompt: Make a 7-second 16:9 line chart for [who will watch, e.g. the board], playing [where, e.g. the quarterly review]. Use only the numbers I attach or list below; don't round or smooth them. Near-black background.

The title sits top left and a large white number top right. An orange line draws from left to right across the period over a fading orange gradient fill, while a white dot with an orange halo rides the tip and the big number counts along with it. At the marked point, a white ring appears on the line with a thin stem and a white label. End frame: the big number turns orange, held for 1.5 seconds.

Title: [e.g. "Weekly active users"]
Data: [your periods and values] (e.g. Jan to Jul, 1,200 to 5,200)
Marked point and label: [e.g. April, "Video launch"]

