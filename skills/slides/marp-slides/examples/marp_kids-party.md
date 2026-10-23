---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Baloo+2:wght@400;500;600;700;800&family=Poppins:wght@300;400;500;600;700&display=swap');

  :root {
    --accent: #ff6b9d;
    --accent-hover: #ff89b5;
    --secondary: #4ecdc4;
    --dark: #fff5e6;
    --card: #ffffff;
    --border: #ffe0c2;
    --muted: #888;
    --light: #333;
  }

  section {
    background: var(--dark);
    color: var(--light);
    font-family: 'Poppins', sans-serif;
    font-weight: 400;
    padding: 50px 70px;
    line-height: 1.6;
  }

  h1 {
    font-family: 'Baloo 2', cursive;
    font-weight: 800;
    font-size: 3.4em;
    color: var(--accent);
    letter-spacing: -0.01em;
    line-height: 1.1;
    margin-bottom: 0.1em;
  }

  h2 {
    font-family: 'Baloo 2', cursive;
    font-weight: 700;
    font-size: 1.8em;
    color: var(--light);
    margin-bottom: 0.3em;
  }

  h3 {
    font-family: 'Baloo 2', cursive;
    font-weight: 600;
    font-size: 0.85em;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.12em;
    margin-bottom: 0.1em;
  }

  strong { color: var(--accent); font-weight: 600; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: radial-gradient(ellipse at 50% 80%, #ffe8d0 0%, var(--dark) 70%);
  }
  section.lead h1 { font-size: 4em; }

  .badge-row { display: flex; gap: 14px; justify-content: center; margin-top: 18px; }
  .badge {
    display: inline-flex; align-items: center; gap: 8px;
    background: var(--card);
    border: 2px solid var(--border);
    border-radius: 24px;
    padding: 8px 20px;
    font-size: 0.72em;
    color: var(--light);
    font-weight: 600;
    box-shadow: 0 2px 8px rgba(0,0,0,0.06);
  }

  .timeline-row {
    display: flex; align-items: center; gap: 0; margin-top: 18px; justify-content: center;
  }
  .phase-card {
    background: var(--card);
    border: 2px solid var(--border);
    border-radius: 16px;
    padding: 14px 16px;
    text-align: center;
    min-width: 130px;
    box-shadow: 0 3px 10px rgba(0,0,0,0.05);
  }
  .phase-card h4 {
    font-family: 'Baloo 2', cursive;
    font-weight: 700; font-size: 0.9em; color: var(--light); margin: 6px 0 2px;
  }
  .phase-card .time { font-size: 0.68em; color: var(--accent); font-weight: 600; }
  .phase-card .dur-bar {
    margin-top: 6px; height: 6px; border-radius: 4px; background: #eee;
  }
  .phase-card .dur-fill { height: 100%; border-radius: 4px; }

  .wavy-connector {
    width: 40px; height: 30px; display: flex; align-items: center; justify-content: center;
  }

  .supply-budget { display: flex; gap: 30px; margin-top: 14px; }
  .supply-col { flex: 1; }
  .budget-col { flex: 1; }

  .checklist-item {
    display: flex; align-items: center; gap: 10px;
    padding: 8px 0; font-size: 0.82em; font-weight: 500;
    border-bottom: 1px dashed var(--border);
  }

  .budget-row {
    margin-bottom: 12px;
  }
  .budget-label {
    display: flex; justify-content: space-between; font-size: 0.72em; margin-bottom: 4px; font-weight: 500;
  }
  .budget-track {
    background: #f0e0cc; border-radius: 8px; height: 16px; overflow: hidden; position: relative;
  }
  .budget-fill { height: 100%; border-radius: 8px; }
  .budget-cap {
    position: absolute; top: 0; right: 0; height: 100%; width: 2px; background: var(--light);
  }

footer: ''
---

<!-- _class: lead -->
<!-- _paginate: false -->

<div style="display: flex; gap: 16px; justify-content: center; margin-bottom: -10px;">
  <svg width="50" height="70" viewBox="0 0 50 70">
    <ellipse cx="25" cy="55" rx="12" ry="14" fill="#ff6b9d" opacity="0.9"/>
    <line x1="25" y1="41" x2="25" y2="10" stroke="#ff6b9d" stroke-width="2"/>
    <path d="M25 10 Q25 5, 22 8" stroke="#ff6b9d" stroke-width="1.5" fill="none"/>
  </svg>
  <svg width="50" height="70" viewBox="0 0 50 70">
    <ellipse cx="25" cy="55" rx="12" ry="14" fill="#4ecdc4" opacity="0.9"/>
    <line x1="25" y1="41" x2="25" y2="10" stroke="#4ecdc4" stroke-width="2"/>
    <path d="M25 10 Q25 5, 28 8" stroke="#4ecdc4" stroke-width="1.5" fill="none"/>
  </svg>
  <svg width="50" height="70" viewBox="0 0 50 70">
    <ellipse cx="25" cy="55" rx="12" ry="14" fill="#ffd93d" opacity="0.9"/>
    <line x1="25" y1="41" x2="25" y2="10" stroke="#ffd93d" stroke-width="2"/>
    <path d="M25 10 Q25 5, 22 8" stroke="#ffd93d" stroke-width="1.5" fill="none"/>
  </svg>
</div>

# Mia's 7th Birthday

<div style="color: #888; font-size: 0.85em; margin-top: 4px;">The ultimate party plan</div>

<div class="badge-row">
  <span class="badge">
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#ff6b9d" stroke-width="2"><rect x="3" y="4" width="18" height="18" rx="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg>
    Saturday, May 10
  </span>
  <span class="badge">
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#4ecdc4" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
    15 Guests
  </span>
  <span class="badge">
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#ffd93d" stroke-width="2"><line x1="12" y1="1" x2="12" y2="23"/><path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/></svg>
    $350 Budget
  </span>
</div>

---

### Party Timeline

## The Big Day Schedule

<div class="timeline-row">
  <div class="phase-card">
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#ff6b9d" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
    <div class="time">2:00 PM</div>
    <h4>Arrive</h4>
    <div style="font-size: 0.62em; color: #888;">Welcome & settle in</div>
    <div class="dur-bar"><div class="dur-fill" style="width: 30%; background: linear-gradient(90deg, #ff6b9d, #ff89b5);"></div></div>
    <div style="font-size: 0.58em; color: #aaa; margin-top: 3px;">20 min</div>
  </div>

  <div class="wavy-connector">
    <svg width="40" height="20" viewBox="0 0 40 20"><path d="M0 10 Q10 2, 20 10 Q30 18, 40 10" stroke="#ffb8d0" stroke-width="2.5" fill="none" stroke-dasharray="4 3"/></svg>
  </div>

  <div class="phase-card">
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#4ecdc4" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
    <div class="time">2:20 PM</div>
    <h4>Games</h4>
    <div style="font-size: 0.62em; color: #888;">Musical chairs & relay</div>
    <div class="dur-bar"><div class="dur-fill" style="width: 70%; background: linear-gradient(90deg, #4ecdc4, #7eddd6);"></div></div>
    <div style="font-size: 0.58em; color: #aaa; margin-top: 3px;">45 min</div>
  </div>

  <div class="wavy-connector">
    <svg width="40" height="20" viewBox="0 0 40 20"><path d="M0 10 Q10 2, 20 10 Q30 18, 40 10" stroke="#ffb8d0" stroke-width="2.5" fill="none" stroke-dasharray="4 3"/></svg>
  </div>

  <div class="phase-card">
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#ffd93d" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
    <div class="time">3:05 PM</div>
    <h4>Cake</h4>
    <div style="font-size: 0.62em; color: #888;">Sing, blow candles, eat!</div>
    <div class="dur-bar"><div class="dur-fill" style="width: 50%; background: linear-gradient(90deg, #ffd93d, #ffe570);"></div></div>
    <div style="font-size: 0.58em; color: #aaa; margin-top: 3px;">30 min</div>
  </div>

  <div class="wavy-connector">
    <svg width="40" height="20" viewBox="0 0 40 20"><path d="M0 10 Q10 2, 20 10 Q30 18, 40 10" stroke="#ffb8d0" stroke-width="2.5" fill="none" stroke-dasharray="4 3"/></svg>
  </div>

  <div class="phase-card">
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#ff6b9d" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
    <div class="time">3:35 PM</div>
    <h4>Presents</h4>
    <div style="font-size: 0.62em; color: #888;">Unwrap the gifts</div>
    <div class="dur-bar"><div class="dur-fill" style="width: 40%; background: linear-gradient(90deg, #ff6b9d, #ff89b5);"></div></div>
    <div style="font-size: 0.58em; color: #aaa; margin-top: 3px;">25 min</div>
  </div>

  <div class="wavy-connector">
    <svg width="40" height="20" viewBox="0 0 40 20"><path d="M0 10 Q10 2, 20 10 Q30 18, 40 10" stroke="#ffb8d0" stroke-width="2.5" fill="none" stroke-dasharray="4 3"/></svg>
  </div>

  <div class="phase-card">
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#4ecdc4" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
    <div class="time">4:00 PM</div>
    <h4>Goodbye</h4>
    <div style="font-size: 0.62em; color: #888;">Party bags & pickup</div>
    <div class="dur-bar"><div class="dur-fill" style="width: 25%; background: linear-gradient(90deg, #4ecdc4, #7eddd6);"></div></div>
    <div style="font-size: 0.58em; color: #aaa; margin-top: 3px;">15 min</div>
  </div>
</div>

---

### Supplies & Budget

## Everything We Need

<div class="supply-budget">
  <div class="supply-col">
    <div style="font-family: 'Baloo 2', cursive; font-weight: 700; font-size: 0.9em; color: var(--accent); margin-bottom: 10px; display: flex; align-items: center; gap: 8px;">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#ff6b9d" stroke-width="2"><path d="M9 11l3 3L22 4"/><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/></svg>
      Supply Checklist
    </div>

    <div class="checklist-item">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="#4ecdc4" stroke="none"><rect x="3" y="3" width="18" height="18" rx="4"/><polyline points="9 12 11 14 16 9" fill="none" stroke="#fff" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
      Unicorn cake (ordered)
    </div>
    <div class="checklist-item">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="#4ecdc4" stroke="none"><rect x="3" y="3" width="18" height="18" rx="4"/><polyline points="9 12 11 14 16 9" fill="none" stroke="#fff" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
      Balloons & streamers
    </div>
    <div class="checklist-item">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="#ffe0c2" stroke="#ccc"><rect x="3" y="3" width="18" height="18" rx="4"/></svg>
      Party bags (x15)
    </div>
    <div class="checklist-item">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="#ffe0c2" stroke="#ccc"><rect x="3" y="3" width="18" height="18" rx="4"/></svg>
      Musical chairs music
    </div>
    <div class="checklist-item">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="#ffe0c2" stroke="#ccc"><rect x="3" y="3" width="18" height="18" rx="4"/></svg>
      Plates, cups & napkins
    </div>
    <div class="checklist-item">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="#4ecdc4" stroke="none"><rect x="3" y="3" width="18" height="18" rx="4"/><polyline points="9 12 11 14 16 9" fill="none" stroke="#fff" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
      Candles & matches
    </div>
  </div>

  <div class="budget-col">
    <div style="font-family: 'Baloo 2', cursive; font-weight: 700; font-size: 0.9em; color: var(--accent); margin-bottom: 10px; display: flex; align-items: center; gap: 8px;">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#ff6b9d" stroke-width="2"><line x1="12" y1="1" x2="12" y2="23"/><path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/></svg>
      Budget Tracker
    </div>

    <div class="budget-row">
      <div class="budget-label">
        <span><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#ffd93d" stroke-width="2" style="vertical-align: middle;"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78L12 21.23l8.84-8.84a5.5 5.5 0 0 0 0-7.78z"/></svg> Cake</span>
        <span style="color: #888;">$85 / $100</span>
      </div>
      <div class="budget-track">
        <div class="budget-fill" style="width: 85%; background: linear-gradient(90deg, #ff6b9d, #ff89b5);"></div>
      </div>
    </div>

    <div class="budget-row">
      <div class="budget-label">
        <span><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#4ecdc4" stroke-width="2" style="vertical-align: middle;"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="16"/><line x1="8" y1="12" x2="16" y2="12"/></svg> Decorations</span>
        <span style="color: #888;">$60 / $80</span>
      </div>
      <div class="budget-track">
        <div class="budget-fill" style="width: 75%; background: linear-gradient(90deg, #4ecdc4, #7eddd6);"></div>
      </div>
    </div>

    <div class="budget-row">
      <div class="budget-label">
        <span><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#ff6b9d" stroke-width="2" style="vertical-align: middle;"><path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"/><circle cx="12" cy="10" r="3"/></svg> Party Bags</span>
        <span style="color: #888;">$55 / $70</span>
      </div>
      <div class="budget-track">
        <div class="budget-fill" style="width: 79%; background: linear-gradient(90deg, #ffd93d, #ffe570);"></div>
      </div>
    </div>

    <div class="budget-row">
      <div class="budget-label">
        <span><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#4ecdc4" stroke-width="2" style="vertical-align: middle;"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg> Games & Prizes</span>
        <span style="color: #888;">$40 / $50</span>
      </div>
      <div class="budget-track">
        <div class="budget-fill" style="width: 80%; background: linear-gradient(90deg, #ff6b9d, #4ecdc4);"></div>
      </div>
    </div>

    <div style="margin-top: 14px; padding: 10px 16px; background: var(--card); border: 2px solid var(--border); border-radius: 12px; display: flex; justify-content: space-between; align-items: center;">
      <span style="font-family: 'Baloo 2', cursive; font-weight: 700; font-size: 0.85em;">Total Spent</span>
      <span style="font-family: 'Baloo 2', cursive; font-weight: 800; font-size: 1.2em; color: var(--accent);">$240 <span style="font-size: 0.6em; color: #888; font-weight: 400;">/ $350</span></span>
    </div>
  </div>
</div>
