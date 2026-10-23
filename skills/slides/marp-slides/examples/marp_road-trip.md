---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Bebas+Neue&family=Nunito+Sans:wght@300;400;500;600;700&display=swap');

  :root {
    --accent: #fc5c7d;
    --accent-hover: #ff7e9a;
    --dark: #1a0a2e;
    --card: #251440;
    --border: #3a2060;
    --muted: #9b8ab5;
    --light: #f0c27f;
    --coral: #fc5c7d;
    --gold: #f0c27f;
  }

  section {
    background: var(--dark);
    color: var(--light);
    font-family: 'Nunito Sans', sans-serif;
    font-weight: 400;
    padding: 50px 70px;
    line-height: 1.6;
  }

  h1 {
    font-family: 'Bebas Neue', sans-serif;
    font-weight: 400;
    font-size: 4.5em;
    color: var(--light);
    letter-spacing: 0.06em;
    line-height: 0.95;
    margin-bottom: 0.1em;
  }

  h2 {
    font-family: 'Bebas Neue', sans-serif;
    font-weight: 400;
    font-size: 2.2em;
    color: var(--light);
    letter-spacing: 0.04em;
    margin-bottom: 0.25em;
  }

  h3 {
    font-family: 'Bebas Neue', sans-serif;
    font-weight: 400;
    font-size: 1em;
    color: var(--coral);
    letter-spacing: 0.2em;
    margin-bottom: 0.1em;
  }

  strong { color: var(--coral); font-weight: 700; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: radial-gradient(ellipse at 50% 100%, #3a1855 0%, #251440 40%, var(--dark) 80%);
  }
  section.lead h1 { font-size: 5.5em; }

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
  }

  .divider {
    width: 80px; height: 3px;
    background: linear-gradient(90deg, var(--coral), var(--gold));
    border-radius: 2px;
    margin: 10px auto 14px;
  }

  .day-card {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 14px;
    padding: 14px 16px;
    flex: 1;
  }
  .day-badge {
    display: inline-block;
    background: linear-gradient(135deg, var(--coral), #ff7e9a);
    color: #fff;
    font-family: 'Bebas Neue', sans-serif;
    font-size: 0.85em;
    letter-spacing: 0.12em;
    padding: 3px 12px;
    border-radius: 6px;
    margin-bottom: 6px;
  }
  .stop-name {
    font-weight: 700; color: var(--light); font-size: 0.85em; margin-bottom: 2px;
  }
  .stop-detail {
    color: var(--muted); font-size: 0.65em; line-height: 1.5;
  }
  .stop-icons {
    display: flex; gap: 8px; margin-top: 6px; align-items: center;
  }
  .icon-badge {
    display: inline-flex; align-items: center; gap: 4px;
    background: rgba(252,92,125,0.1); border: 1px solid rgba(252,92,125,0.2);
    border-radius: 6px; padding: 3px 8px; font-size: 0.6em; color: var(--coral);
  }
  .accom-badge {
    display: inline-flex; align-items: center; gap: 4px;
    background: rgba(240,194,127,0.1); border: 1px solid rgba(240,194,127,0.2);
    border-radius: 6px; padding: 3px 8px; font-size: 0.6em; color: var(--gold);
  }

  .drive-connector {
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    width: 50px; flex-shrink: 0;
  }
  .drive-bar {
    width: 4px; flex: 1; border-radius: 2px;
    background: linear-gradient(180deg, var(--coral), var(--gold));
    min-height: 30px;
  }
  .drive-time {
    font-size: 0.55em; color: var(--muted); text-align: center; padding: 2px 0;
    white-space: nowrap;
  }

  .summary-card {
    background: var(--card); border: 1px solid var(--border);
    border-radius: 14px; padding: 14px 16px; text-align: center;
  }

  .spend-tracker {
    display: flex; align-items: center; gap: 6px; margin-top: 4px;
  }
  .spend-bar {
    flex: 1; height: 8px; background: rgba(255,255,255,0.05); border-radius: 4px; overflow: hidden;
  }
  .spend-fill {
    height: 100%; border-radius: 4px;
    background: linear-gradient(90deg, var(--coral), var(--gold));
  }

  .highlight-card {
    flex: 1; background: rgba(252,92,125,0.08); border: 1px solid rgba(252,92,125,0.2);
    border-radius: 10px; padding: 10px 12px; text-align: center;
  }
footer: ''
---

<!-- _class: lead -->

<svg width="120" height="70" viewBox="0 0 120 70" fill="none">
  <path d="M10 60 Q20 20 40 40 Q60 60 75 30 Q85 10 110 25" stroke="url(#roadGrad)" stroke-width="4" stroke-linecap="round" fill="none" stroke-dasharray="8 4"/>
  <defs><linearGradient id="roadGrad" x1="0" y1="0" x2="120" y2="0"><stop offset="0%" stop-color="#fc5c7d"/><stop offset="100%" stop-color="#f0c27f"/></linearGradient></defs>
  <circle cx="10" cy="60" r="6" fill="#fc5c7d" stroke="#fff" stroke-width="2"/>
  <circle cx="110" cy="25" r="6" fill="#f0c27f" stroke="#fff" stroke-width="2"/>
  <text x="10" y="55" text-anchor="middle" fill="#fff" font-family="Bebas Neue" font-size="7">SF</text>
  <text x="110" y="20" text-anchor="middle" fill="#fff" font-family="Bebas Neue" font-size="7">LA</text>
</svg>

# Pacific Coast Highway

<div class="divider"></div>

<div style="color: #9b8ab5; font-size: 0.95em;">San Francisco to Los Angeles — the ultimate coastal drive</div>

<div class="pill-row">
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
    5 Days
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#f0c27f" stroke-width="2"><path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"/><circle cx="12" cy="10" r="3"/></svg>
    615 Miles
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/></svg>
    $2,400 Budget
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#f0c27f" stroke-width="2"><rect x="3" y="4" width="18" height="18" rx="2"/><path d="M16 2v4M8 2v4M3 10h18"/></svg>
    June 14 Departure
  </span>
</div>

---

### The Route

## Days 1 - 3

<div style="display: flex; gap: 0; align-items: stretch;">
  <div class="day-card">
    <div class="day-badge">DAY 1</div>
    <div class="stop-name">San Francisco → Half Moon Bay</div>
    <div class="stop-detail">Golden Gate Bridge at sunrise, Pacifica cliffs, HMB harbor lunch</div>
    <div class="stop-icons">
      <span class="icon-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><path d="M23 18V6a2 2 0 0 0-2-2H3a2 2 0 0 0-2 2v12"/><rect x="7" y="13" width="10" height="8" rx="1"/></svg> 45 mi</span>
      <span class="icon-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M14.31 8l5.74 9.94M9.69 8h11.48M7.38 12l5.74-9.94M9.69 16L3.95 6.06M14.31 16H2.83M16.62 12l-5.74 9.94"/></svg> 3 stops</span>
      <span class="accom-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#f0c27f" stroke-width="2"><path d="M3 21h18M3 7v14M21 7v14M6 7V4a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v3"/></svg> Ritz HMB</span>
    </div>
    <div class="spend-tracker">
      <span style="font-size: 0.6em; color: var(--coral);">$380</span>
      <div class="spend-bar"><div class="spend-fill" style="width: 76%;"></div></div>
      <span style="font-size: 0.55em; color: var(--muted);">/ $500</span>
    </div>
  </div>

  <div class="drive-connector">
    <div class="drive-time">1.5 hr</div>
    <div class="drive-bar"></div>
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9b8ab5" stroke-width="1.5"><path d="M12 5v14M19 12l-7 7-7-7"/></svg>
  </div>

  <div class="day-card">
    <div class="day-badge">DAY 2</div>
    <div class="stop-name">Half Moon Bay → Monterey</div>
    <div class="stop-detail">Santa Cruz boardwalk, Moss Landing otters, Cannery Row dinner</div>
    <div class="stop-icons">
      <span class="icon-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><path d="M23 18V6a2 2 0 0 0-2-2H3a2 2 0 0 0-2 2v12"/><rect x="7" y="13" width="10" height="8" rx="1"/></svg> 92 mi</span>
      <span class="icon-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M14.31 8l5.74 9.94M9.69 8h11.48M7.38 12l5.74-9.94M9.69 16L3.95 6.06M14.31 16H2.83M16.62 12l-5.74 9.94"/></svg> 4 stops</span>
      <span class="accom-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#f0c27f" stroke-width="2"><path d="M3 21h18M3 7v14M21 7v14M6 7V4a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v3"/></svg> Monterey Inn</span>
    </div>
    <div class="spend-tracker">
      <span style="font-size: 0.6em; color: var(--coral);">$420</span>
      <div class="spend-bar"><div class="spend-fill" style="width: 84%;"></div></div>
      <span style="font-size: 0.55em; color: var(--muted);">/ $500</span>
    </div>
  </div>

  <div class="drive-connector">
    <div class="drive-time">2.5 hr</div>
    <div class="drive-bar"></div>
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9b8ab5" stroke-width="1.5"><path d="M12 5v14M19 12l-7 7-7-7"/></svg>
  </div>

  <div class="day-card">
    <div class="day-badge">DAY 3</div>
    <div class="stop-name">Monterey → Big Sur</div>
    <div class="stop-detail">17-Mile Drive, Bixby Bridge, McWay Falls, Pfeiffer Beach sunset</div>
    <div class="stop-icons">
      <span class="icon-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><path d="M23 18V6a2 2 0 0 0-2-2H3a2 2 0 0 0-2 2v12"/><rect x="7" y="13" width="10" height="8" rx="1"/></svg> 65 mi</span>
      <span class="icon-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M14.31 8l5.74 9.94M9.69 8h11.48M7.38 12l5.74-9.94M9.69 16L3.95 6.06M14.31 16H2.83M16.62 12l-5.74 9.94"/></svg> 5 stops</span>
      <span class="accom-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#f0c27f" stroke-width="2"><path d="M3 21h18M3 7v14M21 7v14M6 7V4a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v3"/></svg> Glen Oaks</span>
    </div>
    <div class="spend-tracker">
      <span style="font-size: 0.6em; color: var(--coral);">$510</span>
      <div class="spend-bar"><div class="spend-fill" style="width: 100%;"></div></div>
      <span style="font-size: 0.55em; color: var(--muted);">/ $500</span>
    </div>
  </div>
</div>

---

### Finale

## Days 4 - 5 + Trip Summary

<div style="display: flex; gap: 16px;">
  <div style="flex: 1; display: flex; flex-direction: column; gap: 10px;">
    <div class="day-card">
      <div class="day-badge">DAY 4</div>
      <div class="stop-name">Big Sur → San Luis Obispo</div>
      <div class="stop-detail">Elephant seals at Piedras Blancas, Hearst Castle tour, SLO downtown</div>
      <div class="stop-icons">
        <span class="icon-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><path d="M23 18V6a2 2 0 0 0-2-2H3a2 2 0 0 0-2 2v12"/><rect x="7" y="13" width="10" height="8" rx="1"/></svg> 140 mi</span>
        <span class="icon-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M14.31 8l5.74 9.94M9.69 8h11.48"/></svg> 3 stops</span>
        <span class="accom-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#f0c27f" stroke-width="2"><path d="M3 21h18M3 7v14M21 7v14"/></svg> Madonna Inn</span>
      </div>
      <div class="spend-tracker">
        <span style="font-size: 0.6em; color: var(--coral);">$460</span>
        <div class="spend-bar"><div class="spend-fill" style="width: 92%;"></div></div>
        <span style="font-size: 0.55em; color: var(--muted);">/ $500</span>
      </div>
    </div>
    <div class="day-card">
      <div class="day-badge">DAY 5</div>
      <div class="stop-name">San Luis Obispo → Los Angeles</div>
      <div class="stop-detail">Pismo Beach stop, Santa Barbara wine, Malibu PCH, Santa Monica pier finish</div>
      <div class="stop-icons">
        <span class="icon-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><path d="M23 18V6a2 2 0 0 0-2-2H3a2 2 0 0 0-2 2v12"/><rect x="7" y="13" width="10" height="8" rx="1"/></svg> 273 mi</span>
        <span class="icon-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#fc5c7d" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M14.31 8l5.74 9.94M9.69 8h11.48"/></svg> 4 stops</span>
        <span class="accom-badge"><svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#f0c27f" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg> HOME!</span>
      </div>
      <div class="spend-tracker">
        <span style="font-size: 0.6em; color: var(--coral);">$350</span>
        <div class="spend-bar"><div class="spend-fill" style="width: 70%;"></div></div>
        <span style="font-size: 0.55em; color: var(--muted);">/ $500</span>
      </div>
    </div>
  </div>

  <div style="flex: 1.1; display: flex; flex-direction: column; gap: 10px;">
    <div class="summary-card">
      <div style="font-family: 'Bebas Neue', sans-serif; font-size: 1.1em; letter-spacing: 0.12em; color: var(--coral); margin-bottom: 8px;">TRIP TOTALS</div>
      <div style="display: flex; gap: 10px; margin-bottom: 10px;">
        <div style="flex: 1;">
          <div style="font-size: 0.6em; color: var(--muted); text-transform: uppercase; letter-spacing: 0.08em;">Miles Driven</div>
          <div style="background: rgba(255,255,255,0.05); border-radius: 6px; height: 18px; margin-top: 4px; overflow: hidden; position: relative;">
            <div style="background: linear-gradient(90deg, #fc5c7d, #f0c27f); width: 100%; height: 100%; border-radius: 6px;"></div>
            <span style="position: absolute; top: 1px; right: 6px; font-size: 0.65em; color: #fff; font-weight: 700;">615 mi</span>
          </div>
        </div>
      </div>
      <div style="display: flex; gap: 10px; margin-bottom: 6px;">
        <div style="flex: 1;">
          <div style="font-size: 0.6em; color: var(--muted); text-transform: uppercase; letter-spacing: 0.08em;">Budget: Spent vs Planned</div>
          <div style="background: rgba(255,255,255,0.05); border-radius: 6px; height: 18px; margin-top: 4px; overflow: hidden; display: flex;">
            <div style="background: var(--coral); width: 88%; height: 100%; display: flex; align-items: center; justify-content: center;">
              <span style="font-size: 0.6em; color: #fff; font-weight: 700;">$2,120 spent</span>
            </div>
            <div style="background: rgba(240,194,127,0.3); width: 12%; height: 100%; display: flex; align-items: center; justify-content: center;">
              <span style="font-size: 0.55em; color: var(--gold); font-weight: 600;">$280</span>
            </div>
          </div>
          <div style="font-size: 0.55em; color: var(--muted); margin-top: 2px; text-align: right;">$280 under budget</div>
        </div>
      </div>
    </div>

    <div style="font-family: 'Bebas Neue', sans-serif; font-size: 0.95em; letter-spacing: 0.12em; color: var(--gold); margin-top: 2px;">HIGHLIGHT REEL</div>
    <div style="display: flex; gap: 8px;">
      <div class="highlight-card">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#f0c27f" stroke-width="1.5"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
        <div style="font-size: 0.7em; font-weight: 700; color: var(--light); margin-top: 4px;">Bixby Bridge</div>
        <div style="font-size: 0.58em; color: var(--muted);">Sunset golden hour</div>
      </div>
      <div class="highlight-card">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#f0c27f" stroke-width="1.5"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
        <div style="font-size: 0.7em; font-weight: 700; color: var(--light); margin-top: 4px;">McWay Falls</div>
        <div style="font-size: 0.58em; color: var(--muted);">Turquoise cove magic</div>
      </div>
      <div class="highlight-card">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#f0c27f" stroke-width="1.5"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
        <div style="font-size: 0.7em; font-weight: 700; color: var(--light); margin-top: 4px;">SM Pier Finish</div>
        <div style="font-size: 0.58em; color: var(--muted);">End of Route 66</div>
      </div>
    </div>
  </div>
</div>
