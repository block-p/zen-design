---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Merriweather:wght@400;700;900&family=Roboto:wght@300;400;500;700&display=swap');

  :root {
    --accent: #8B4513;
    --accent-hover: #A0522D;
    --dark: #f5e6c8;
    --card: #ede0c0;
    --border: #c9a96e;
    --muted: #6b5b3e;
    --light: #3e2723;
    --red: #b71c1c;
    --green: #2e7d32;
    --blue: #1565c0;
    --gold: #c9951a;
  }

  section {
    background: var(--dark);
    background-image: url("data:image/svg+xml,%3Csvg width='60' height='60' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M30 5 L50 19 L50 41 L30 55 L10 41 L10 19 Z' fill='none' stroke='%23c9a96e' stroke-width='0.5' opacity='0.2'/%3E%3C/svg%3E");
    color: var(--light);
    font-family: 'Roboto', sans-serif;
    font-weight: 400;
    padding: 60px 80px;
    line-height: 1.6;
  }

  h1 {
    font-family: 'Merriweather', serif;
    font-weight: 900;
    font-size: 3.2em;
    color: var(--accent);
    letter-spacing: -0.01em;
    line-height: 1.1;
    margin-bottom: 0.2em;
  }

  h2 {
    font-family: 'Merriweather', serif;
    font-weight: 700;
    font-size: 1.8em;
    color: var(--light);
    margin-bottom: 0.4em;
  }

  h3 {
    font-family: 'Merriweather', serif;
    font-weight: 700;
    font-size: 0.85em;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.15em;
    margin-bottom: 0.1em;
  }

  strong { color: var(--accent); font-weight: 700; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: var(--dark);
    background-image: none;
  }
  section.lead h1 { font-size: 3.8em; margin-bottom: 0.1em; }
  section.lead p { color: var(--muted); font-size: 1em; }

  .badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: var(--card);
    border: 2px solid var(--border);
    border-radius: 24px;
    padding: 6px 18px;
    font-size: 0.72em;
    color: var(--muted);
    font-weight: 500;
    letter-spacing: 0.04em;
  }

  .badge-row { display: flex; gap: 12px; justify-content: center; margin-top: 16px; }

  .phase-box {
    flex: 1;
    background: var(--card);
    border: 2px solid var(--border);
    border-radius: 14px;
    padding: 18px 14px;
    text-align: center;
    position: relative;
  }
  .phase-box h4 {
    font-family: 'Merriweather', serif;
    font-weight: 700;
    font-size: 1em;
    color: var(--light);
    margin: 8px 0 4px;
  }
  .phase-box p {
    color: var(--muted);
    font-size: 0.7em;
    margin: 0;
    line-height: 1.4;
  }
  .phase-num {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    background: var(--accent);
    color: #fff;
    border-radius: 50%;
    font-size: 0.75em;
    font-weight: 700;
    font-family: 'Merriweather', serif;
  }

  .scoring-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 16px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 10px;
    margin-bottom: 8px;
  }
  .scoring-row .pts {
    font-family: 'Merriweather', serif;
    font-weight: 900;
    font-size: 1.6em;
    color: var(--accent);
    min-width: 44px;
    text-align: center;
  }
  .scoring-row .label {
    font-size: 0.82em;
    color: var(--light);
    font-weight: 500;
  }
  .scoring-row .desc {
    font-size: 0.68em;
    color: var(--muted);
  }

  .vp-bar {
    display: flex;
    height: 22px;
    border-radius: 8px;
    overflow: hidden;
    border: 1px solid var(--border);
    margin-top: 14px;
  }
  .vp-seg {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.55em;
    font-weight: 700;
    color: #fff;
  }

footer: ''
---

<!-- _class: lead -->

<svg width="280" height="240" viewBox="0 0 280 240" style="margin-bottom: 10px;">
  <!-- Hex grid pattern -->
  <g transform="translate(140,120)">
    <polygon points="0,-50 43,-25 43,25 0,50 -43,25 -43,-25" fill="none" stroke="#8B4513" stroke-width="2.5" opacity="0.9"/>
    <polygon points="0,-50 43,-25 43,25 0,50 -43,25 -43,-25" fill="#c9951a" opacity="0.15"/>
    <!-- Inner resource icon: wheat -->
    <line x1="-8" y1="10" x2="0" y2="-20" stroke="#c9951a" stroke-width="2" stroke-linecap="round"/>
    <line x1="8" y1="10" x2="0" y2="-20" stroke="#c9951a" stroke-width="2" stroke-linecap="round"/>
    <circle cx="0" cy="-22" r="4" fill="#c9951a"/>
    <circle cx="-6" cy="-14" r="3" fill="#c9951a" opacity="0.7"/>
    <circle cx="6" cy="-14" r="3" fill="#c9951a" opacity="0.7"/>
  </g>
  <g transform="translate(54,70)">
    <polygon points="0,-50 43,-25 43,25 0,50 -43,25 -43,-25" fill="none" stroke="#8B4513" stroke-width="2" opacity="0.5"/>
    <polygon points="0,-50 43,-25 43,25 0,50 -43,25 -43,-25" fill="#2e7d32" opacity="0.1"/>
    <!-- Tree icon: wood -->
    <line x1="0" y1="14" x2="0" y2="-6" stroke="#5d4037" stroke-width="3" stroke-linecap="round"/>
    <circle cx="0" cy="-14" r="12" fill="#2e7d32" opacity="0.6"/>
  </g>
  <g transform="translate(226,70)">
    <polygon points="0,-50 43,-25 43,25 0,50 -43,25 -43,-25" fill="none" stroke="#8B4513" stroke-width="2" opacity="0.5"/>
    <polygon points="0,-50 43,-25 43,25 0,50 -43,25 -43,-25" fill="#b71c1c" opacity="0.1"/>
    <!-- Brick icon -->
    <rect x="-14" y="-8" width="12" height="7" rx="1" fill="#b71c1c" opacity="0.6"/>
    <rect x="2" y="-8" width="12" height="7" rx="1" fill="#b71c1c" opacity="0.6"/>
    <rect x="-8" y="2" width="12" height="7" rx="1" fill="#b71c1c" opacity="0.5"/>
  </g>
  <g transform="translate(54,170)">
    <polygon points="0,-50 43,-25 43,25 0,50 -43,25 -43,-25" fill="none" stroke="#8B4513" stroke-width="2" opacity="0.5"/>
    <polygon points="0,-50 43,-25 43,25 0,50 -43,25 -43,-25" fill="#888" opacity="0.1"/>
    <!-- Ore icon -->
    <polygon points="0,-12 10,0 6,12 -6,12 -10,0" fill="#666" opacity="0.5" stroke="#444" stroke-width="1"/>
  </g>
  <g transform="translate(226,170)">
    <polygon points="0,-50 43,-25 43,25 0,50 -43,25 -43,-25" fill="none" stroke="#8B4513" stroke-width="2" opacity="0.5"/>
    <polygon points="0,-50 43,-25 43,25 0,50 -43,25 -43,-25" fill="#fff" opacity="0.15"/>
    <!-- Sheep icon -->
    <ellipse cx="0" cy="0" rx="12" ry="8" fill="#e8e0d0" stroke="#999" stroke-width="1"/>
    <circle cx="-8" cy="-4" r="5" fill="#e8e0d0" stroke="#999" stroke-width="0.8"/>
    <circle cx="8" cy="-4" r="5" fill="#e8e0d0" stroke="#999" stroke-width="0.8"/>
    <circle cx="0" cy="-7" r="4" fill="#e8e0d0" stroke="#999" stroke-width="0.8"/>
  </g>
</svg>

# Settlers of Catan

Quick Reference Guide

<div class="badge-row">
  <div class="badge">
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#8B4513" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
    3 - 4 Players
  </div>
  <div class="badge">
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#8B4513" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
    60 - 90 min
  </div>
  <div class="badge">
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#8B4513" stroke-width="2"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
    First to 10 VP Wins
  </div>
</div>

---

### Your Turn

## Turn Flow

<div style="display: flex; align-items: center; justify-content: center; gap: 0; margin-top: 24px;">
  <!-- Phase 1: Roll -->
  <div class="phase-box" style="min-width: 170px;">
    <div class="phase-num">1</div>
    <svg width="40" height="40" viewBox="0 0 40 40" style="display: block; margin: 8px auto 0;">
      <rect x="4" y="4" width="32" height="32" rx="6" fill="none" stroke="#8B4513" stroke-width="2"/>
      <circle cx="14" cy="14" r="3" fill="#3e2723"/>
      <circle cx="26" cy="14" r="3" fill="#3e2723"/>
      <circle cx="20" cy="20" r="3" fill="#3e2723"/>
      <circle cx="14" cy="26" r="3" fill="#3e2723"/>
      <circle cx="26" cy="26" r="3" fill="#3e2723"/>
    </svg>
    <h4>Roll</h4>
    <p>Roll 2 dice. All players collect resources from matching hexes.</p>
    <div style="display: flex; gap: 4px; justify-content: center; margin-top: 6px;">
      <svg width="16" height="16" viewBox="0 0 16 16"><circle cx="8" cy="8" r="7" fill="#2e7d32" opacity="0.5"/><text x="8" y="11" text-anchor="middle" fill="#fff" font-size="8" font-family="Roboto">W</text></svg>
      <svg width="16" height="16" viewBox="0 0 16 16"><circle cx="8" cy="8" r="7" fill="#b71c1c" opacity="0.5"/><text x="8" y="11" text-anchor="middle" fill="#fff" font-size="8" font-family="Roboto">B</text></svg>
      <svg width="16" height="16" viewBox="0 0 16 16"><circle cx="8" cy="8" r="7" fill="#e8e0d0" stroke="#999" stroke-width="0.5"/><text x="8" y="11" text-anchor="middle" fill="#666" font-size="8" font-family="Roboto">S</text></svg>
      <svg width="16" height="16" viewBox="0 0 16 16"><circle cx="8" cy="8" r="7" fill="#c9951a" opacity="0.5"/><text x="8" y="11" text-anchor="middle" fill="#fff" font-size="8" font-family="Roboto">G</text></svg>
      <svg width="16" height="16" viewBox="0 0 16 16"><circle cx="8" cy="8" r="7" fill="#666" opacity="0.5"/><text x="8" y="11" text-anchor="middle" fill="#fff" font-size="8" font-family="Roboto">O</text></svg>
    </div>
  </div>

  <!-- Arrow 1 -->
  <svg width="44" height="24" viewBox="0 0 44 24" style="flex-shrink: 0;">
    <line x1="2" y1="12" x2="32" y2="12" stroke="#8B4513" stroke-width="2.5" stroke-dasharray="4,3"/>
    <polygon points="32,6 44,12 32,18" fill="#8B4513"/>
  </svg>

  <!-- Phase 2: Trade -->
  <div class="phase-box" style="min-width: 170px;">
    <div class="phase-num">2</div>
    <svg width="40" height="40" viewBox="0 0 40 40" style="display: block; margin: 8px auto 0;">
      <path d="M8 20 L18 14 L18 26 Z" fill="none" stroke="#8B4513" stroke-width="2" stroke-linejoin="round"/>
      <path d="M32 20 L22 14 L22 26 Z" fill="none" stroke="#8B4513" stroke-width="2" stroke-linejoin="round"/>
      <line x1="12" y1="12" x2="28" y2="12" stroke="#8B4513" stroke-width="1.5" stroke-dasharray="2,2"/>
      <line x1="12" y1="28" x2="28" y2="28" stroke="#8B4513" stroke-width="1.5" stroke-dasharray="2,2"/>
    </svg>
    <h4>Trade</h4>
    <p>Trade with other players or use 4:1 port trades for resources.</p>
  </div>

  <!-- Arrow 2 -->
  <svg width="44" height="24" viewBox="0 0 44 24" style="flex-shrink: 0;">
    <line x1="2" y1="12" x2="32" y2="12" stroke="#8B4513" stroke-width="2.5" stroke-dasharray="4,3"/>
    <polygon points="32,6 44,12 32,18" fill="#8B4513"/>
  </svg>

  <!-- Phase 3: Build -->
  <div class="phase-box" style="min-width: 170px;">
    <div class="phase-num">3</div>
    <svg width="40" height="40" viewBox="0 0 40 40" style="display: block; margin: 8px auto 0;">
      <path d="M20 6 L32 18 L32 34 L8 34 L8 18 Z" fill="none" stroke="#8B4513" stroke-width="2" stroke-linejoin="round"/>
      <rect x="15" y="22" width="10" height="12" fill="none" stroke="#8B4513" stroke-width="1.5"/>
      <line x1="20" y1="22" x2="20" y2="34" stroke="#8B4513" stroke-width="1"/>
    </svg>
    <h4>Build</h4>
    <p>Spend resources to build roads, settlements, cities, or dev cards.</p>
  </div>

  <!-- Arrow 3 -->
  <svg width="44" height="24" viewBox="0 0 44 24" style="flex-shrink: 0;">
    <line x1="2" y1="12" x2="32" y2="12" stroke="#8B4513" stroke-width="2.5" stroke-dasharray="4,3"/>
    <polygon points="32,6 44,12 32,18" fill="#8B4513"/>
  </svg>

  <!-- Phase 4: Draw -->
  <div class="phase-box" style="min-width: 170px;">
    <div class="phase-num">4</div>
    <svg width="40" height="40" viewBox="0 0 40 40" style="display: block; margin: 8px auto 0;">
      <rect x="10" y="6" width="20" height="28" rx="3" fill="none" stroke="#8B4513" stroke-width="2"/>
      <line x1="14" y1="12" x2="26" y2="12" stroke="#8B4513" stroke-width="1.2"/>
      <line x1="14" y1="17" x2="26" y2="17" stroke="#8B4513" stroke-width="1.2"/>
      <circle cx="20" cy="26" r="4" fill="none" stroke="#8B4513" stroke-width="1.5"/>
      <text x="20" y="29" text-anchor="middle" fill="#8B4513" font-size="6" font-family="Merriweather" font-weight="700">?</text>
    </svg>
    <h4>Draw</h4>
    <p>If you bought a dev card, draw it. Play dev cards before rolling next turn.</p>
  </div>
</div>

<div style="text-align: center; margin-top: 18px; font-size: 0.72em; color: var(--muted);">
  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#b71c1c" stroke-width="2" style="vertical-align: middle;"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
  <strong style="color: #b71c1c;">Robber:</strong> On a roll of 7, move the robber. Players with 8+ cards discard half.
</div>

---

### Scoring Reference

## Victory Points

<div style="display: flex; gap: 24px; margin-top: 14px;">
  <div style="flex: 1.4;">
    <div class="scoring-row">
      <svg width="32" height="32" viewBox="0 0 32 32"><path d="M16 4 L24 12 L24 24 L8 24 L8 12 Z" fill="none" stroke="#2e7d32" stroke-width="2"/><circle cx="16" cy="16" r="2" fill="#2e7d32"/></svg>
      <div>
        <div class="label">Settlement</div>
        <div class="desc">Wood + Brick + Sheep + Wheat</div>
      </div>
      <div class="pts">1</div>
    </div>
    <div class="scoring-row">
      <svg width="32" height="32" viewBox="0 0 32 32"><path d="M16 2 L26 10 L26 26 L6 26 L6 10 Z" fill="none" stroke="#c9951a" stroke-width="2.5"/><path d="M11 26 L11 18 L21 18 L21 26" fill="none" stroke="#c9951a" stroke-width="1.5"/><rect x="14" y="6" width="4" height="6" fill="none" stroke="#c9951a" stroke-width="1.2"/></svg>
      <div>
        <div class="label">City (upgrade)</div>
        <div class="desc">3 Ore + 2 Wheat</div>
      </div>
      <div class="pts">2</div>
    </div>
    <div class="scoring-row">
      <svg width="32" height="32" viewBox="0 0 32 32"><line x1="4" y1="16" x2="28" y2="16" stroke="#8B4513" stroke-width="3" stroke-linecap="round"/><circle cx="4" cy="16" r="3" fill="#8B4513"/><circle cx="28" cy="16" r="3" fill="#8B4513"/><text x="16" y="12" text-anchor="middle" fill="#3e2723" font-size="7" font-weight="700" font-family="Merriweather">5+</text></svg>
      <div>
        <div class="label">Longest Road</div>
        <div class="desc">5+ continuous road segments</div>
      </div>
      <div class="pts">2</div>
    </div>
    <div class="scoring-row">
      <svg width="32" height="32" viewBox="0 0 32 32"><circle cx="16" cy="12" r="6" fill="none" stroke="#b71c1c" stroke-width="2"/><path d="M10 20 L16 16 L22 20 L22 28 L10 28 Z" fill="none" stroke="#b71c1c" stroke-width="2"/></svg>
      <div>
        <div class="label">Largest Army</div>
        <div class="desc">3+ knight cards played</div>
      </div>
      <div class="pts">2</div>
    </div>
    <div class="scoring-row">
      <svg width="32" height="32" viewBox="0 0 32 32"><rect x="8" y="6" width="16" height="20" rx="2" fill="none" stroke="#6b5b3e" stroke-width="2"/><polygon points="16,11 17.5,14 21,14.5 18.5,17 19,20.5 16,19 13,20.5 13.5,17 11,14.5 14.5,14" fill="#c9951a" stroke="#c9951a" stroke-width="0.5"/></svg>
      <div>
        <div class="label">Victory Point Card</div>
        <div class="desc">From development card deck</div>
      </div>
      <div class="pts">1</div>
    </div>
  </div>

  <div style="flex: 0.6; display: flex; flex-direction: column; align-items: center; justify-content: center;">
    <div style="font-family: 'Merriweather', serif; font-size: 0.7em; color: var(--muted); text-transform: uppercase; letter-spacing: 0.12em; margin-bottom: 8px;">Victory Track</div>
    <svg width="80" height="320" viewBox="0 0 80 320">
      <!-- Track background -->
      <rect x="30" y="10" width="20" height="300" rx="10" fill="#ede0c0" stroke="#c9a96e" stroke-width="1.5"/>
      <!-- Segments 1-10 from bottom to top -->
      <g transform="translate(40, 290)"><circle r="10" fill="#c9a96e" opacity="0.3"/><text text-anchor="middle" y="4" fill="#3e2723" font-size="10" font-weight="700" font-family="Merriweather">1</text></g>
      <g transform="translate(40, 260)"><circle r="10" fill="#c9a96e" opacity="0.3"/><text text-anchor="middle" y="4" fill="#3e2723" font-size="10" font-weight="700" font-family="Merriweather">2</text></g>
      <g transform="translate(40, 230)"><circle r="10" fill="#c9a96e" opacity="0.3"/><text text-anchor="middle" y="4" fill="#3e2723" font-size="10" font-weight="700" font-family="Merriweather">3</text></g>
      <g transform="translate(40, 200)"><circle r="10" fill="#c9a96e" opacity="0.3"/><text text-anchor="middle" y="4" fill="#3e2723" font-size="10" font-weight="700" font-family="Merriweather">4</text></g>
      <g transform="translate(40, 170)"><circle r="10" fill="#c9a96e" opacity="0.35"/><text text-anchor="middle" y="4" fill="#3e2723" font-size="10" font-weight="700" font-family="Merriweather">5</text></g>
      <g transform="translate(40, 140)"><circle r="10" fill="#c9a96e" opacity="0.4"/><text text-anchor="middle" y="4" fill="#3e2723" font-size="10" font-weight="700" font-family="Merriweather">6</text></g>
      <g transform="translate(40, 110)"><circle r="10" fill="#c9a96e" opacity="0.45"/><text text-anchor="middle" y="4" fill="#3e2723" font-size="10" font-weight="700" font-family="Merriweather">7</text></g>
      <g transform="translate(40, 80)"><circle r="10" fill="#c9a96e" opacity="0.5"/><text text-anchor="middle" y="4" fill="#3e2723" font-size="10" font-weight="700" font-family="Merriweather">8</text></g>
      <g transform="translate(40, 50)"><circle r="10" fill="#c9a96e" opacity="0.6"/><text text-anchor="middle" y="4" fill="#3e2723" font-size="10" font-weight="700" font-family="Merriweather">9</text></g>
      <g transform="translate(40, 20)"><circle r="12" fill="#c9951a" stroke="#8B4513" stroke-width="2"/><text text-anchor="middle" y="5" fill="#fff" font-size="11" font-weight="900" font-family="Merriweather">10</text></g>
      <!-- Crown at top -->
      <g transform="translate(40, 20)">
        <polygon points="0,-24 -8,-16 -4,-16 -4,-13 4,-13 4,-16 8,-16" fill="#c9951a" opacity="0.8"/>
      </g>
    </svg>
  </div>
</div>
