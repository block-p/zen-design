---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Noto+Sans+JP:wght@400;500;700;900&family=Inter:wght@300;400;500;600&display=swap');

  :root {
    --accent: #e74c3c;
    --accent-hover: #ff6b5a;
    --dark: #fff8f0;
    --card: #fff0e0;
    --border: #e8d5c0;
    --muted: #7a6b5e;
    --light: #2c2c2c;
  }

  section {
    background: var(--dark);
    color: var(--light);
    font-family: 'Inter', sans-serif;
    font-weight: 400;
    padding: 50px 70px;
    line-height: 1.6;
  }

  h1 {
    font-family: 'Noto Sans JP', sans-serif;
    font-weight: 900;
    font-size: 3.8em;
    color: var(--accent);
    letter-spacing: -0.02em;
    line-height: 1.05;
    margin-bottom: 0.15em;
  }

  h2 {
    font-family: 'Noto Sans JP', sans-serif;
    font-weight: 700;
    font-size: 1.8em;
    color: var(--light);
    margin-bottom: 0.3em;
  }

  h3 {
    font-family: 'Noto Sans JP', sans-serif;
    font-weight: 500;
    font-size: 0.8em;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.18em;
    margin-bottom: 0.1em;
  }

  strong { color: var(--accent); font-weight: 600; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: radial-gradient(ellipse at 50% 90%, #ffe8d0 0%, var(--dark) 70%);
  }
  section.lead h1 { font-size: 4.5em; }
  section.lead p { color: var(--muted); font-size: 1.05em; }

  .pill-row { display: flex; gap: 10px; justify-content: center; margin-top: 16px; }
  .pill {
    display: inline-flex; align-items: center; gap: 6px;
    background: #fff;
    border: 1px solid var(--border);
    border-radius: 24px;
    padding: 7px 18px;
    font-size: 0.7em;
    color: var(--muted);
    letter-spacing: 0.04em;
    font-weight: 500;
  }
  .pill svg { flex-shrink: 0; }

  .divider {
    width: 60px; height: 3px;
    background: var(--accent);
    border-radius: 2px;
    margin: 14px auto 18px;
  }

  .stop-card {
    background: #fff;
    border: 1px solid var(--border);
    border-radius: 14px;
    padding: 16px 18px;
    text-align: center;
    position: relative;
    min-width: 155px;
  }
  .stop-card .stop-num {
    position: absolute;
    top: -14px; left: 50%; transform: translateX(-50%);
    background: var(--accent);
    color: #fff;
    width: 28px; height: 28px;
    border-radius: 50%;
    display: flex; align-items: center; justify-content: center;
    font-weight: 700; font-size: 0.8em;
    box-shadow: 0 2px 8px rgba(231,76,60,0.3);
  }
  .stop-card h4 {
    font-family: 'Noto Sans JP', sans-serif;
    font-weight: 700; font-size: 0.95em; color: var(--light);
    margin: 8px 0 4px;
  }
  .stop-card p { color: var(--muted); font-size: 0.7em; margin: 0; line-height: 1.4; }

  .must-try {
    display: inline-block;
    background: var(--accent);
    color: #fff;
    font-size: 0.55em;
    font-weight: 600;
    padding: 2px 10px;
    border-radius: 10px;
    margin-top: 6px;
    letter-spacing: 0.05em;
    text-transform: uppercase;
  }

  .connector {
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    min-width: 60px;
  }
  .connector .walk-time {
    font-size: 0.6em; color: var(--muted); font-weight: 500;
    margin-bottom: 2px;
  }

  .summary-card {
    background: linear-gradient(135deg, var(--accent) 0%, #c0392b 100%);
    border-radius: 14px;
    padding: 20px 24px;
    color: #fff;
    text-align: center;
    min-width: 180px;
  }
  .summary-card h4 {
    font-family: 'Noto Sans JP', sans-serif;
    font-weight: 700; font-size: 1em; margin: 0 0 10px; color: #fff;
  }
  .summary-card .stat { font-size: 0.75em; margin: 4px 0; opacity: 0.9; }
  .summary-card .stat strong { color: #fff; }

  .photo-badge {
    display: inline-flex; align-items: center; gap: 5px;
    background: rgba(255,255,255,0.2);
    border-radius: 10px;
    padding: 4px 12px;
    font-size: 0.65em;
    margin-top: 8px;
    font-weight: 600;
  }

footer: ''
---

<!-- _class: lead -->

<svg width="90" height="90" viewBox="0 0 100 100" fill="none">
  <path d="M50 5 L55 40 H65 L50 55 L55 90 H45 L50 55 H35 L50 40 H45 Z" fill="none" stroke="var(--accent)" stroke-width="2"/>
  <path d="M30 90 C30 90 35 50 50 30 C65 50 70 90 70 90" fill="none" stroke="var(--accent)" stroke-width="3" stroke-linecap="round"/>
  <line x1="50" y1="30" x2="50" y2="10" stroke="var(--accent)" stroke-width="3" stroke-linecap="round"/>
  <path d="M38 90 L50 65 L62 90" fill="none" stroke="var(--accent)" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
  <rect x="44" y="82" width="12" height="8" rx="1" fill="var(--accent)" opacity="0.3"/>
</svg>

# Tokyo in a Day

<div class="divider"></div>

A curated walk through the heart of Tokyo — temples, streets, food, and city views

<div class="pill-row">
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"/><circle cx="12" cy="10" r="3"/></svg>
    8 Stops
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><path d="M18 6L6 18M6 6l12 12" stroke-width="0"/><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
    7-8 Hours
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
    18.5 km Total
  </span>
</div>

---

### Route Part 1

## Stops 1 through 4

<div style="display: flex; align-items: center; justify-content: center; gap: 0; margin-top: 28px;">

  <div class="stop-card">
    <div class="stop-num">1</div>
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"/><circle cx="12" cy="10" r="3"/></svg>
    <h4>Senso-ji</h4>
    <p>Asakusa's iconic temple<br>Thunder Gate entrance</p>
    <div class="must-try">Must See</div>
  </div>

  <div class="connector">
    <div class="walk-time">12 min</div>
    <svg width="60" height="8" viewBox="0 0 60 8">
      <line x1="0" y1="4" x2="55" y2="4" stroke="var(--accent)" stroke-width="2" stroke-dasharray="4 3"/>
      <polygon points="52,1 60,4 52,7" fill="var(--accent)"/>
    </svg>
  </div>

  <div class="stop-card">
    <div class="stop-num">2</div>
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><path d="M2 12h20"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
    <h4>Ueno Park</h4>
    <p>Museums, shrines<br>Cherry blossom paths</p>
  </div>

  <div class="connector">
    <div class="walk-time">20 min</div>
    <svg width="60" height="8" viewBox="0 0 60 8">
      <line x1="0" y1="4" x2="55" y2="4" stroke="var(--accent)" stroke-width="2" stroke-dasharray="4 3"/>
      <polygon points="52,1 60,4 52,7" fill="var(--accent)"/>
    </svg>
  </div>

  <div class="stop-card">
    <div class="stop-num">3</div>
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><rect x="2" y="7" width="20" height="14" rx="2"/><path d="M16 3l-4 4-4-4"/></svg>
    <h4>Akihabara</h4>
    <p>Electric Town<br>Anime & arcades</p>
  </div>

  <div class="connector">
    <div class="walk-time">15 min</div>
    <svg width="60" height="8" viewBox="0 0 60 8">
      <line x1="0" y1="4" x2="55" y2="4" stroke="var(--accent)" stroke-width="2" stroke-dasharray="4 3"/>
      <polygon points="52,1 60,4 52,7" fill="var(--accent)"/>
    </svg>
  </div>

  <div class="stop-card">
    <div class="stop-num">4</div>
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
    <h4>Imperial Palace</h4>
    <p>East Gardens walk<br>Moat & stone walls</p>
    <div class="must-try">Photo Spot</div>
  </div>

</div>

---

### Route Part 2

## Stops 5 through 8

<div style="display: flex; align-items: center; justify-content: center; gap: 0; margin-top: 28px;">

  <div class="stop-card">
    <div class="stop-num">5</div>
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
    <h4>Shibuya</h4>
    <p>Famous crossing<br>Hachiko statue</p>
  </div>

  <div class="connector">
    <div class="walk-time">18 min</div>
    <svg width="60" height="8" viewBox="0 0 60 8">
      <line x1="0" y1="4" x2="55" y2="4" stroke="var(--accent)" stroke-width="2" stroke-dasharray="4 3"/>
      <polygon points="52,1 60,4 52,7" fill="var(--accent)"/>
    </svg>
  </div>

  <div class="stop-card">
    <div class="stop-num">6</div>
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/></svg>
    <h4>Harajuku</h4>
    <p>Takeshita Street<br>Meiji Shrine nearby</p>
    <div class="must-try">Must Try: Crepes</div>
  </div>

  <div class="connector">
    <div class="walk-time">25 min</div>
    <svg width="60" height="8" viewBox="0 0 60 8">
      <line x1="0" y1="4" x2="55" y2="4" stroke="var(--accent)" stroke-width="2" stroke-dasharray="4 3"/>
      <polygon points="52,1 60,4 52,7" fill="var(--accent)"/>
    </svg>
  </div>

  <div class="stop-card">
    <div class="stop-num">7</div>
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
    <h4>Shinjuku</h4>
    <p>Omoide Yokocho<br>Golden Gai alley bars</p>
  </div>

  <div class="connector">
    <div class="walk-time">10 min</div>
    <svg width="60" height="8" viewBox="0 0 60 8">
      <line x1="0" y1="4" x2="55" y2="4" stroke="var(--accent)" stroke-width="2" stroke-dasharray="4 3"/>
      <polygon points="52,1 60,4 52,7" fill="var(--accent)"/>
    </svg>
  </div>

  <div class="stop-card">
    <div class="stop-num">8</div>
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
    <h4>Tokyo Tower</h4>
    <p>Night skyline views<br>Observation deck</p>
  </div>

</div>

<div style="display: flex; justify-content: center; margin-top: 28px;">
  <div class="summary-card">
    <h4>Route Complete</h4>
    <div class="stat"><strong>24,800</strong> steps estimated</div>
    <div class="stat"><strong>7.5 hrs</strong> total with stops</div>
    <div class="photo-badge">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#fff" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5"/><path d="M21 15l-5-5L5 21"/></svg>
      Best Photo: Imperial Palace at sunset
    </div>
  </div>
</div>
