---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Bitter:wght@300;400;500;600;700;800;900&family=Fira+Sans:wght@300;400;500;600&display=swap');

  :root {
    --accent: #6b8e23;
    --accent-hover: #8fbc3e;
    --dark: #2a1f14;
    --card: #3a2e22;
    --border: #4d3f30;
    --muted: #a89880;
    --light: #e8dcc8;
    --terra: #cd853f;
    --terra-hover: #daa06d;
  }

  section {
    background: var(--dark);
    color: var(--light);
    font-family: 'Fira Sans', sans-serif;
    font-weight: 400;
    padding: 50px 70px;
    line-height: 1.6;
  }

  h1 {
    font-family: 'Bitter', serif;
    font-weight: 900;
    font-size: 3.4em;
    color: var(--light);
    letter-spacing: -0.02em;
    line-height: 1.05;
    margin-bottom: 0.15em;
  }

  h2 {
    font-family: 'Bitter', serif;
    font-weight: 700;
    font-size: 1.7em;
    color: var(--light);
    margin-bottom: 0.3em;
  }

  h3 {
    font-family: 'Bitter', serif;
    font-weight: 600;
    font-size: 0.8em;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.15em;
    margin-bottom: 0.1em;
  }

  strong { color: var(--terra); font-weight: 600; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: radial-gradient(ellipse at 50% 90%, #3a2e22 0%, var(--dark) 70%);
  }
  section.lead h1 { font-size: 4em; }

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
    width: 60px; height: 3px;
    background: var(--accent);
    border-radius: 2px;
    margin: 14px auto 16px;
  }

  .gantt-row {
    display: flex; align-items: center; gap: 0; margin-bottom: 6px; font-size: 0.65em;
  }
  .gantt-label {
    width: 80px; flex-shrink: 0; font-weight: 600; color: var(--light); text-align: right; padding-right: 10px;
  }
  .gantt-track {
    flex: 1; height: 20px; background: var(--card); border-radius: 4px; position: relative; overflow: hidden;
    border: 1px solid var(--border);
  }
  .gantt-bar {
    position: absolute; top: 2px; height: 16px; border-radius: 3px; display: flex; align-items: center; justify-content: center;
    font-size: 0.85em; font-weight: 500; color: #fff;
  }

  .month-header {
    display: flex; gap: 0; margin-bottom: 4px; font-size: 0.55em; color: var(--muted); text-transform: uppercase; letter-spacing: 0.08em;
  }
  .month-header span {
    flex: 1; text-align: center; padding: 3px 0;
    border-right: 1px solid var(--border);
  }
  .month-header span:last-child { border-right: none; }

  .companion-pair {
    display: flex; align-items: center; gap: 8px; margin-bottom: 8px;
    background: var(--card); border: 1px solid var(--border); border-radius: 10px; padding: 8px 14px;
  }

  .checklist-item {
    display: flex; align-items: center; gap: 8px; margin-bottom: 6px; font-size: 0.75em;
  }

  .harvest-bar-container {
    flex: 1; background: var(--card); border: 1px solid var(--border); border-radius: 8px;
    height: 28px; display: flex; overflow: hidden;
  }
  .harvest-seg {
    height: 100%; display: flex; align-items: center; justify-content: center;
    font-size: 0.6em; font-weight: 600; color: #fff;
  }
footer: ''
---

<!-- _class: lead -->

<svg width="90" height="90" viewBox="0 0 24 24" fill="none" stroke-linecap="round" stroke-linejoin="round">
  <path d="M12 22V12" stroke="#6b8e23" stroke-width="2"/>
  <path d="M12 12C12 12 8 8 8 5a4 4 0 0 1 8 0c0 3-4 7-4 7z" fill="rgba(107,142,35,0.2)" stroke="#6b8e23" stroke-width="1.5"/>
  <path d="M12 15C12 15 16 12 18 10" stroke="#8fbc3e" stroke-width="1.5"/>
  <path d="M18 10c1-2 0-4-2-4s-3 2-4 4" fill="rgba(107,142,35,0.15)" stroke="#8fbc3e" stroke-width="1"/>
  <path d="M12 17C12 17 8 14 6 12" stroke="#8fbc3e" stroke-width="1.5"/>
  <path d="M6 12c-1-2 0-4 2-4s3 2 4 4" fill="rgba(107,142,35,0.15)" stroke="#8fbc3e" stroke-width="1"/>
  <path d="M8 22h8" stroke="#cd853f" stroke-width="2"/>
  <path d="M6 22c0-2 2-3 6-3s6 1 6 3" stroke="#cd853f" stroke-width="1.5" fill="rgba(205,133,63,0.15)"/>
</svg>

# 2026 Garden Plan

<div class="divider"></div>

From seed to harvest — a full season mapped out

<div class="pill-row">
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6b8e23" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
    Zone 8b
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#cd853f" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18M9 3v18"/></svg>
    4 x 12 ft Plot
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6b8e23" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/></svg>
    8 Crops Planned
  </span>
</div>

---

### Planting Calendar

## Seed to Harvest Timeline

<div style="margin-left: 80px;">
  <div class="month-header">
    <span>Jan</span><span>Feb</span><span>Mar</span><span>Apr</span><span>May</span><span>Jun</span><span>Jul</span><span>Aug</span><span>Sep</span><span>Oct</span><span>Nov</span><span>Dec</span>
  </div>
</div>

<div class="gantt-row">
  <div class="gantt-label"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="5" fill="#e74c3c"/></svg> Tomatoes</div>
  <div class="gantt-track">
    <div class="gantt-bar" style="left: 16.6%; width: 8.3%; background: rgba(107,142,35,0.4); border: 1px dashed #6b8e23;">
      <svg width="8" height="8" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#6b8e23"/></svg>
    </div>
    <div class="gantt-bar" style="left: 25%; width: 50%; background: linear-gradient(90deg, #6b8e23, #8fbc3e);">plant &rarr; harvest</div>
  </div>
</div>
<div class="gantt-row">
  <div class="gantt-label"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="5" fill="#27ae60"/></svg> Peppers</div>
  <div class="gantt-track">
    <div class="gantt-bar" style="left: 16.6%; width: 8.3%; background: rgba(107,142,35,0.4); border: 1px dashed #6b8e23;">
      <svg width="8" height="8" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#6b8e23"/></svg>
    </div>
    <div class="gantt-bar" style="left: 25%; width: 50%; background: linear-gradient(90deg, #8fbc3e, #6b8e23);">plant &rarr; harvest</div>
  </div>
</div>
<div class="gantt-row">
  <div class="gantt-label"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="5" fill="#8fbc3e"/></svg> Lettuce</div>
  <div class="gantt-track">
    <div class="gantt-bar" style="left: 8.3%; width: 25%; background: linear-gradient(90deg, #6b8e23, #8fbc3e);">spring</div>
    <div class="gantt-bar" style="left: 66.6%; width: 25%; background: linear-gradient(90deg, #6b8e23, #8fbc3e);">fall</div>
  </div>
</div>
<div class="gantt-row">
  <div class="gantt-label"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="5" fill="#cd853f"/></svg> Herbs</div>
  <div class="gantt-track">
    <div class="gantt-bar" style="left: 25%; width: 58.3%; background: linear-gradient(90deg, #cd853f, #daa06d);">continuous harvest</div>
  </div>
</div>
<div class="gantt-row">
  <div class="gantt-label"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="5" fill="#f39c12"/></svg> Squash</div>
  <div class="gantt-track">
    <div class="gantt-bar" style="left: 33.3%; width: 41.6%; background: linear-gradient(90deg, #f39c12, #f1c40f);">plant &rarr; harvest</div>
  </div>
</div>
<div class="gantt-row">
  <div class="gantt-label"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="5" fill="#27ae60"/></svg> Beans</div>
  <div class="gantt-track">
    <div class="gantt-bar" style="left: 33.3%; width: 33.3%; background: linear-gradient(90deg, #27ae60, #2ecc71);">plant &rarr; harvest</div>
  </div>
</div>
<div class="gantt-row">
  <div class="gantt-label"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="5" fill="#e67e22"/></svg> Carrots</div>
  <div class="gantt-track">
    <div class="gantt-bar" style="left: 16.6%; width: 33.3%; background: linear-gradient(90deg, #e67e22, #d35400);">spring sow</div>
    <div class="gantt-bar" style="left: 58.3%; width: 25%; background: linear-gradient(90deg, #e67e22, #d35400);">fall sow</div>
  </div>
</div>
<div class="gantt-row">
  <div class="gantt-label"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="5" fill="#2ecc71"/></svg> Kale</div>
  <div class="gantt-track">
    <div class="gantt-bar" style="left: 8.3%; width: 33.3%; background: linear-gradient(90deg, #2ecc71, #27ae60);">spring</div>
    <div class="gantt-bar" style="left: 58.3%; width: 33.3%; background: linear-gradient(90deg, #2ecc71, #27ae60);">fall/winter</div>
  </div>
</div>

<div style="display: flex; gap: 16px; margin-top: 8px; font-size: 0.6em; color: var(--muted);">
  <span><svg width="8" height="8" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#6b8e23"/></svg> = seed indoors</span>
  <span style="display: inline-block; width: 20px; height: 8px; background: #6b8e23; border-radius: 2px; vertical-align: middle;"></span> = active growing/harvest window
</div>

---

### Harvest Forecast

## Monthly Yields & Companion Planting

<div style="display: flex; gap: 20px;">
  <div style="flex: 1.4;">
    <div style="font-size: 0.7em; font-weight: 600; color: var(--terra); text-transform: uppercase; letter-spacing: 0.1em; margin-bottom: 8px;">Estimated Monthly Yield (lbs)</div>
    <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 5px; font-size: 0.62em;">
      <span style="width: 32px; text-align: right; color: var(--muted);">Jun</span>
      <div class="harvest-bar-container">
        <div class="harvest-seg" style="width: 30%; background: #8fbc3e;">Lettuce</div>
        <div class="harvest-seg" style="width: 20%; background: #cd853f;">Herbs</div>
        <div class="harvest-seg" style="width: 10%; background: #27ae60;">Beans</div>
      </div>
      <span style="width: 28px; color: var(--muted);">8 lb</span>
    </div>
    <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 5px; font-size: 0.62em;">
      <span style="width: 32px; text-align: right; color: var(--muted);">Jul</span>
      <div class="harvest-bar-container">
        <div class="harvest-seg" style="width: 35%; background: #e74c3c;">Tomato</div>
        <div class="harvest-seg" style="width: 20%; background: #27ae60;">Beans</div>
        <div class="harvest-seg" style="width: 15%; background: #cd853f;">Herbs</div>
        <div class="harvest-seg" style="width: 10%; background: #f39c12;">Squash</div>
      </div>
      <span style="width: 28px; color: var(--muted);">18 lb</span>
    </div>
    <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 5px; font-size: 0.62em;">
      <span style="width: 32px; text-align: right; color: var(--muted);">Aug</span>
      <div class="harvest-bar-container">
        <div class="harvest-seg" style="width: 30%; background: #e74c3c;">Tomato</div>
        <div class="harvest-seg" style="width: 20%; background: #27ae60;">Pepper</div>
        <div class="harvest-seg" style="width: 20%; background: #f39c12;">Squash</div>
        <div class="harvest-seg" style="width: 15%; background: #e67e22;">Carrot</div>
      </div>
      <span style="width: 28px; color: var(--muted);">24 lb</span>
    </div>
    <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 5px; font-size: 0.62em;">
      <span style="width: 32px; text-align: right; color: var(--muted);">Sep</span>
      <div class="harvest-bar-container">
        <div class="harvest-seg" style="width: 25%; background: #e74c3c;">Tomato</div>
        <div class="harvest-seg" style="width: 20%; background: #27ae60;">Pepper</div>
        <div class="harvest-seg" style="width: 15%; background: #2ecc71;">Kale</div>
        <div class="harvest-seg" style="width: 15%; background: #e67e22;">Carrot</div>
      </div>
      <span style="width: 28px; color: var(--muted);">20 lb</span>
    </div>
    <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 5px; font-size: 0.62em;">
      <span style="width: 32px; text-align: right; color: var(--muted);">Oct</span>
      <div class="harvest-bar-container">
        <div class="harvest-seg" style="width: 30%; background: #2ecc71;">Kale</div>
        <div class="harvest-seg" style="width: 25%; background: #8fbc3e;">Lettuce</div>
        <div class="harvest-seg" style="width: 15%; background: #e67e22;">Carrot</div>
      </div>
      <span style="width: 28px; color: var(--muted);">12 lb</span>
    </div>
  </div>

  <div style="flex: 1;">
    <div style="font-size: 0.7em; font-weight: 600; color: var(--terra); text-transform: uppercase; letter-spacing: 0.1em; margin-bottom: 8px;">Companion Pairs</div>
    <div class="companion-pair">
      <svg width="18" height="18" viewBox="0 0 10 10"><circle cx="5" cy="5" r="5" fill="#e74c3c"/></svg>
      <span style="font-size: 0.72em; color: var(--light);">Tomatoes</span>
      <svg width="20" height="12" viewBox="0 0 20 12"><line x1="0" y1="6" x2="14" y2="6" stroke="#cd853f" stroke-width="1.5"/><path d="M8 3 L11 6 L8 9" fill="none" stroke="#cd853f" stroke-width="1.5"/><svg x="14" y="1" width="6" height="10" viewBox="0 0 24 24"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78L12 21.23l8.84-8.84a5.5 5.5 0 0 0 0-7.78z" fill="#cd853f" stroke="none"/></svg></svg>
      <span style="font-size: 0.72em; color: var(--light);">Basil</span>
    </div>
    <div class="companion-pair">
      <svg width="18" height="18" viewBox="0 0 10 10"><circle cx="5" cy="5" r="5" fill="#e67e22"/></svg>
      <span style="font-size: 0.72em; color: var(--light);">Carrots</span>
      <svg width="20" height="12" viewBox="0 0 20 12"><line x1="0" y1="6" x2="14" y2="6" stroke="#cd853f" stroke-width="1.5"/><path d="M8 3 L11 6 L8 9" fill="none" stroke="#cd853f" stroke-width="1.5"/><svg x="14" y="1" width="6" height="10" viewBox="0 0 24 24"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78L12 21.23l8.84-8.84a5.5 5.5 0 0 0 0-7.78z" fill="#cd853f" stroke="none"/></svg></svg>
      <span style="font-size: 0.72em; color: var(--light);">Beans</span>
    </div>
    <div class="companion-pair">
      <svg width="18" height="18" viewBox="0 0 10 10"><circle cx="5" cy="5" r="5" fill="#f39c12"/></svg>
      <span style="font-size: 0.72em; color: var(--light);">Squash</span>
      <svg width="20" height="12" viewBox="0 0 20 12"><line x1="0" y1="6" x2="14" y2="6" stroke="#cd853f" stroke-width="1.5"/><path d="M8 3 L11 6 L8 9" fill="none" stroke="#cd853f" stroke-width="1.5"/><svg x="14" y="1" width="6" height="10" viewBox="0 0 24 24"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78L12 21.23l8.84-8.84a5.5 5.5 0 0 0 0-7.78z" fill="#cd853f" stroke="none"/></svg></svg>
      <span style="font-size: 0.72em; color: var(--light);">Beans</span>
    </div>

    <div style="font-size: 0.7em; font-weight: 600; color: var(--terra); text-transform: uppercase; letter-spacing: 0.1em; margin: 14px 0 8px;">April Tasks</div>
    <div class="checklist-item">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#6b8e23" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="3"/><polyline points="9 12 11.5 14.5 16 9"/></svg>
      <span style="color: var(--light);">Start tomato + pepper seeds indoors</span>
    </div>
    <div class="checklist-item">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#6b8e23" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="3"/><polyline points="9 12 11.5 14.5 16 9"/></svg>
      <span style="color: var(--light);">Direct sow lettuce + kale outdoors</span>
    </div>
    <div class="checklist-item">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#4d3f30" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="3"/></svg>
      <span style="color: var(--muted);">Amend beds with compost</span>
    </div>
    <div class="checklist-item">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#4d3f30" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="3"/></svg>
      <span style="color: var(--muted);">Set up drip irrigation</span>
    </div>
    <div class="checklist-item">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#4d3f30" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="3"/></svg>
      <span style="color: var(--muted);">Order bean + squash seeds</span>
    </div>
  </div>
</div>
