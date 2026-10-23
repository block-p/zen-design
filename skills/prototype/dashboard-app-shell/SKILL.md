---
name: dashboard-app-shell
title: High-Density Dashboard App Shell
category: prototype
description: Production-grade SaaS product shell with persistent navigation rail, global search, sortable data grids, live metrics, and contextual inspectors.
---

# High-Density Dashboard App Shell

Build sophisticated, high-density SaaS product workspaces inspired by Linear, Raycast, Vercel Dashboard, and Supabase Studio.

## Layout Blueprint

1. **Full-Viewport Application Shell**:
   - `min-height: 100dvh; display: grid; grid-template-columns: var(--nav-width, 240px) 1fr;`
   - Fixed persistent sidebar / navigation rail on the inline-start.
   - Main content viewport with sticky top command header and scrollable body.

2. **Persistent Navigation Rail**:
   - Workspace/Organization switcher dropdown at the top with status avatar.
   - Categorized navigation links (`Overview`, `Projects`, `Analytics`, `Settings`, etc.) with active indicators.
   - Keyboard navigation badges (e.g. `1`, `2`, `3` or `G then P`).
   - Bottom user profile pill with status dot and plan badge.

3. **Global Command Header**:
   - Breadcrumb navigation (`Workspace / Telemetry / Real-time Ingestion`).
   - Global search input / command bar shortcut trigger (`⌘K`).
   - Right-aligned quick action buttons (e.g. `+ New Resource`, `Export CSV`, `Live Sync Indicator`).

4. **Live Metrics Banner (KPI Bento Cards)**:
   - 3 to 4 metric cards displaying headline numbers in tabular digits (`font-variant-numeric: tabular-nums`).
   - Micro inline sparklines or trend badges (`+14.8% vs last week`, colored green or amber).
   - Clean card hierarchy with 1px border and hover glow.

5. **Sortable, Interactive Data Grid**:
   - Column headers with click-to-sort indicators (`▲`, `▼`).
   - Client-side search and category dropdown filtering rows instantaneously.
   - Status pills with subtle pulsing indicator dots (`Active`, `Processing`, `Suspended`).
   - Row click opens a slide-over contextual drawer showing JSON payload, metadata, and edit controls.

6. **Contextual Inspector / Detail Drawer**:
   - Slides smoothly from the right (`transform: translateX(0)`) on item selection.
   - Displays full item properties, timeline audit trail, and quick action buttons.
   - Close button and `Escape` key dismiss listener.
