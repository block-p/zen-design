---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Montserrat:wght@300;400;500;600;700;800;900&family=Open+Sans:wght@300;400;500;600&display=swap');

  :root {
    --accent: #3498db;
    --accent-hover: #5dade2;
    --dark: #f8f8f8;
    --card: #ffffff;
    --border: #e0e0e0;
    --muted: #7f8c8d;
    --light: #2c3e50;
    --red: #e74c3c;
    --green: #27ae60;
    --yellow: #f39c12;
  }

  section {
    background: var(--dark);
    color: var(--light);
    font-family: 'Open Sans', sans-serif;
    font-weight: 400;
    padding: 50px 70px;
    line-height: 1.6;
  }

  h1 {
    font-family: 'Montserrat', sans-serif;
    font-weight: 900;
    font-size: 3.2em;
    color: var(--light);
    letter-spacing: -0.03em;
    line-height: 1.05;
    margin-bottom: 0.15em;
  }

  h2 {
    font-family: 'Montserrat', sans-serif;
    font-weight: 700;
    font-size: 1.8em;
    color: var(--light);
    margin-bottom: 0.3em;
  }

  h3 {
    font-family: 'Montserrat', sans-serif;
    font-weight: 600;
    font-size: 0.8em;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.15em;
    margin-bottom: 0.1em;
  }

  strong { color: var(--accent); font-weight: 600; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: linear-gradient(135deg, #f8f8f8 0%, #ecf0f1 50%, #dfe6e9 100%);
  }
  section.lead h1 { font-size: 3.8em; }

  .pill-row { display: flex; gap: 10px; justify-content: center; margin-top: 16px; flex-wrap: wrap; }
  .pill {
    display: inline-flex; align-items: center; gap: 6px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 24px;
    padding: 8px 18px;
    font-size: 0.7em;
    color: var(--muted);
    letter-spacing: 0.04em;
    box-shadow: 0 2px 8px rgba(0,0,0,0.06);
  }
  .pill svg { flex-shrink: 0; }

  .divider {
    width: 60px; height: 3px;
    background: var(--accent);
    border-radius: 2px;
    margin: 14px auto 16px;
  }

  .score-bar {
    height: 18px; border-radius: 9px; display: inline-block; vertical-align: middle;
  }
  .bar-green { background: linear-gradient(90deg, #27ae60, #2ecc71); }
  .bar-yellow { background: linear-gradient(90deg, #f39c12, #f1c40f); }
  .bar-red { background: linear-gradient(90deg, #e74c3c, #ec7063); }

  table {
    width: 100%;
    border-collapse: separate;
    border-spacing: 0 5px;
    font-size: 0.72em;
  }
  table th {
    background: var(--accent);
    color: #fff;
    font-family: 'Montserrat', sans-serif;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    font-size: 0.85em;
    padding: 10px 14px;
    border: none;
  }
  table th:first-child { border-radius: 8px 0 0 8px; }
  table th:last-child { border-radius: 0 8px 8px 0; }
  table td {
    background: var(--card);
    padding: 8px 14px;
    border: none;
    text-align: center;
  }
  table td:first-child { border-radius: 8px 0 0 8px; text-align: left; font-weight: 600; color: var(--light); }
  table td:last-child { border-radius: 0 8px 8px 0; }

  .pros-cons { display: flex; gap: 20px; margin-top: 12px; }
  .pros-panel, .cons-panel {
    flex: 1; background: var(--card); border-radius: 14px; padding: 18px 20px;
    box-shadow: 0 2px 12px rgba(0,0,0,0.05);
  }
  .pros-panel { border-left: 4px solid var(--green); }
  .cons-panel { border-left: 4px solid var(--red); }

  .stat-card {
    flex: 1; background: var(--card); border: 1px solid var(--border);
    border-radius: 14px; padding: 16px 14px; text-align: center;
    box-shadow: 0 2px 8px rgba(0,0,0,0.05);
  }

  .verdict-badge {
    display: inline-block; background: var(--accent); color: #fff;
    font-family: 'Montserrat', sans-serif; font-weight: 700;
    padding: 10px 28px; border-radius: 30px; font-size: 0.9em;
    box-shadow: 0 4px 16px rgba(52,152,219,0.3);
    letter-spacing: 0.04em;
  }
footer: ''
---

<!-- _class: lead -->

<svg width="80" height="80" viewBox="0 0 24 24" fill="none" stroke="#3498db" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round">
  <rect x="3" y="10" width="18" height="12" rx="1"/>
  <path d="M3 10L12 3l9 7"/>
  <rect x="9" y="15" width="6" height="7"/>
  <rect x="5" y="13" width="3" height="3"/>
  <rect x="16" y="13" width="3" height="3"/>
</svg>

# Apartment Hunt 2026

<div class="divider"></div>

Finding the perfect place in Brooklyn

<div class="pill-row">
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#3498db" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18M9 3v18"/></svg>
    4 Apartments Shortlisted
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2"><path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/></svg>
    $2,200 - $2,800/mo
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#f39c12" stroke-width="2"><rect x="3" y="4" width="18" height="18" rx="2"/><path d="M16 2v4M8 2v4M3 10h18"/></svg>
    Move-in: June 1
  </span>
</div>

---

### Scoring Matrix

## Side-by-Side Comparison

<div style="overflow-x: auto;">

| | <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#fff" stroke-width="2"><rect x="3" y="10" width="18" height="12" rx="1"/><path d="M3 10L12 3l9 7"/></svg> **Unit 2A — Bushwick** | <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#fff" stroke-width="2"><rect x="3" y="10" width="18" height="12" rx="1"/><path d="M3 10L12 3l9 7"/></svg> **Unit 3B — Greenpoint** | <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#fff" stroke-width="2"><rect x="3" y="10" width="18" height="12" rx="1"/><path d="M3 10L12 3l9 7"/></svg> **Unit 1C — Williamsburg** | <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#fff" stroke-width="2"><rect x="3" y="10" width="18" height="12" rx="1"/><path d="M3 10L12 3l9 7"/></svg> **Unit 4D — Bed-Stuy** |
|---|---|---|---|---|
| **Price** (25%) | <div class="score-bar bar-green" style="width: 85%;"></div> $2,200 | <div class="score-bar bar-green" style="width: 70%;"></div> $2,500 | <div class="score-bar bar-red" style="width: 40%;"></div> $2,800 | <div class="score-bar bar-green" style="width: 90%;"></div> $2,100 |
| **Location** (20%) | <div class="score-bar bar-yellow" style="width: 60%;"></div> 6/10 | <div class="score-bar bar-green" style="width: 90%;"></div> 9/10 | <div class="score-bar bar-green" style="width: 85%;"></div> 8/10 | <div class="score-bar bar-yellow" style="width: 55%;"></div> 5/10 |
| **Size** (20%) | <div class="score-bar bar-yellow" style="width: 55%;"></div> 550 sqft | <div class="score-bar bar-green" style="width: 80%;"></div> 700 sqft | <div class="score-bar bar-yellow" style="width: 60%;"></div> 580 sqft | <div class="score-bar bar-green" style="width: 85%;"></div> 720 sqft |
| **Light** (15%) | <div class="score-bar bar-red" style="width: 35%;"></div> North | <div class="score-bar bar-green" style="width: 90%;"></div> South | <div class="score-bar bar-green" style="width: 75%;"></div> East | <div class="score-bar bar-yellow" style="width: 50%;"></div> West |
| **Noise** (10%) | <div class="score-bar bar-red" style="width: 30%;"></div> Loud | <div class="score-bar bar-green" style="width: 80%;"></div> Quiet | <div class="score-bar bar-red" style="width: 35%;"></div> Loud | <div class="score-bar bar-green" style="width: 75%;"></div> Quiet |
| **Transit** (10%) | <div class="score-bar bar-green" style="width: 75%;"></div> L,M | <div class="score-bar bar-green" style="width: 85%;"></div> G,7 | <div class="score-bar bar-green" style="width: 90%;"></div> L,G,J | <div class="score-bar bar-yellow" style="width: 60%;"></div> A,C |
| **TOTAL** | <div style="font-size: 1.3em; font-weight: 700; color: #7f8c8d;">68</div> | <div style="font-size: 1.3em; font-weight: 700; color: #27ae60;">87</div> | <div style="font-size: 1.3em; font-weight: 700; color: #f39c12;">72</div> | <div style="font-size: 1.3em; font-weight: 700; color: #7f8c8d;">70</div> |

</div>

---

### Winner

## Unit 3B: Greenpoint

<div class="pros-cons">
  <div class="pros-panel">
    <div style="font-family: 'Montserrat', sans-serif; font-weight: 700; color: #27ae60; font-size: 0.85em; margin-bottom: 10px; text-transform: uppercase; letter-spacing: 0.1em;">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2.5"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
      Pros
    </div>
    <div style="font-size: 0.78em; line-height: 2.2; color: #2c3e50;">
      <div><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2.5" style="vertical-align: middle;"><polyline points="20 6 9 17 4 12"/></svg> South-facing, floor-to-ceiling windows</div>
      <div><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2.5" style="vertical-align: middle;"><polyline points="20 6 9 17 4 12"/></svg> 700 sqft — dedicated office nook</div>
      <div><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2.5" style="vertical-align: middle;"><polyline points="20 6 9 17 4 12"/></svg> Tree-lined block, very quiet</div>
      <div><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2.5" style="vertical-align: middle;"><polyline points="20 6 9 17 4 12"/></svg> In-unit washer/dryer</div>
      <div><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2.5" style="vertical-align: middle;"><polyline points="20 6 9 17 4 12"/></svg> 8 min walk to G train</div>
    </div>
  </div>
  <div class="cons-panel">
    <div style="font-family: 'Montserrat', sans-serif; font-weight: 700; color: #e74c3c; font-size: 0.85em; margin-bottom: 10px; text-transform: uppercase; letter-spacing: 0.1em;">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#e74c3c" stroke-width="2.5"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>
      Cons
    </div>
    <div style="font-size: 0.78em; line-height: 2.2; color: #2c3e50;">
      <div><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#e74c3c" stroke-width="2.5" style="vertical-align: middle;"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg> $300/mo above budget center</div>
      <div><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#e74c3c" stroke-width="2.5" style="vertical-align: middle;"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg> No dishwasher</div>
      <div><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#e74c3c" stroke-width="2.5" style="vertical-align: middle;"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg> 3rd floor walk-up (no elevator)</div>
    </div>
  </div>
</div>

<div style="display: flex; gap: 14px; margin-top: 16px;">
  <div class="stat-card">
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#3498db" stroke-width="1.5" style="margin-bottom: 4px;"><path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/><line x1="7" y1="7" x2="7.01" y2="7"/></svg>
    <div style="font-family: 'Montserrat', sans-serif; font-size: 1.8em; font-weight: 800; color: #3498db; line-height: 1;">92</div>
    <div style="color: #7f8c8d; font-size: 0.65em; text-transform: uppercase; letter-spacing: 0.1em;">Walk Score</div>
  </div>
  <div class="stat-card">
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#3498db" stroke-width="1.5" style="margin-bottom: 4px;"><rect x="4" y="3" width="16" height="18" rx="2"/><path d="M4 11h16"/><path d="M12 3v18"/></svg>
    <div style="font-family: 'Montserrat', sans-serif; font-size: 1.8em; font-weight: 800; color: #3498db; line-height: 1;">88</div>
    <div style="color: #7f8c8d; font-size: 0.65em; text-transform: uppercase; letter-spacing: 0.1em;">Transit Score</div>
  </div>
  <div class="stat-card">
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#3498db" stroke-width="1.5" style="margin-bottom: 4px;"><path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/></svg>
    <div style="font-family: 'Montserrat', sans-serif; font-size: 1.8em; font-weight: 800; color: #3498db; line-height: 1;">$2,500</div>
    <div style="color: #7f8c8d; font-size: 0.65em; text-transform: uppercase; letter-spacing: 0.1em;">Avg Rent Area</div>
  </div>
</div>

<div style="text-align: center; margin-top: 18px;">
  <span class="verdict-badge">
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#fff" stroke-width="2.5" style="vertical-align: middle; margin-right: 6px;"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
    TOP PICK — APPLY BY MAY 15
  </span>
</div>
