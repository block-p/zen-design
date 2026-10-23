---
name: grouped-bars
title: Grouped Bars
category: explainer
tier: premium
engine: hyperframes
description: Before-and-after bars per group, with the change popping above each pair.
url: https://claude-motion.com/skills/grouped-bars
---

# Grouped Bars

Before-and-after bars per group, with the change popping above each pair.

### Grouped Bars

- Page: https://claude-motion.com/skills/grouped-bars · Install: https://claude-motion.com/skills/grouped-bars/install.md
- premium · Explainer · hyperframes · 16:9, 9:16, 1:1 · 7s
- Last checked: 2026-10-14

The first series grows across all groups, then the second series grows beside it, and a pill with the percentage change pops above each pair. Ideal for before/after, year-over-year or us/them comparisons.

Good fit: Before/after and year-over-year comparisons; Two series across a few groups.
Not for: More than two series; Long time series.
Inputs: title, series, groups, suffix, showChange (yes | no), theme (ink | paper | electric | noir | sand), fontDisplay (serif | sans | grotesk | mono).

Example prompt: Use the grouped-bars skill: series "2025 | 2026", groups "EU: 1.2, 2.1 | US: 3.4, 4.0 | APAC: 0.6, 1.8", suffix "M". Electric theme.

Claude Motion prompt: Make a 7-second 16:9 bar chart for [who will watch, e.g. clients we're pitching], playing [where, e.g. the case study]. Use only the numbers I attach or list below; the percentage changes must match them. Sand background.

The title sits top left with a small legend top right (before in gray, after in orange). Gray bars grow up from a baseline for each item. Then shorter orange bars grow beside each one. End frame: dark pills pop above each pair with the change.

Title: [e.g. "Hours spent per video"]
Items with before and after: [e.g. Script 4h → 2h, Design 10h → 3h, Animation 16h → 2h, Revisions 8h → 1h]

