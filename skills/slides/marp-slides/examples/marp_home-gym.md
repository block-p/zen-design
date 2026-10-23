---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Rajdhani:wght@400;500;600;700&family=Rubik:wght@300;400;500;600;700&display=swap');

  :root {
    --accent: #00ff88;
    --accent-hover: #33ffaa;
    --dark: #0d0d0d;
    --card: #141414;
    --border: #222222;
    --muted: #666;
    --light: #ffffff;
  }

  section {
    background: var(--dark);
    color: var(--light);
    font-family: 'Rubik', sans-serif;
    font-weight: 400;
    padding: 50px 70px;
    line-height: 1.6;
  }

  h1 {
    font-family: 'Rajdhani', sans-serif;
    font-weight: 700;
    font-size: 4em;
    color: var(--accent);
    letter-spacing: 0.04em;
    line-height: 1;
    margin-bottom: 0.1em;
    text-transform: uppercase;
  }

  h2 {
    font-family: 'Rajdhani', sans-serif;
    font-weight: 700;
    font-size: 2em;
    color: var(--light);
    margin-bottom: 0.3em;
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }

  h3 {
    font-family: 'Rajdhani', sans-serif;
    font-weight: 600;
    font-size: 0.8em;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.2em;
    margin-bottom: 0.1em;
  }

  strong { color: var(--accent); font-weight: 600; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: radial-gradient(ellipse at 50% 80%, #001a0d 0%, var(--dark) 70%);
  }
  section.lead h1 { font-size: 5em; letter-spacing: 0.08em; }
  section.lead p { color: var(--muted); font-size: 1em; }

  .divider {
    width: 60px; height: 3px;
    background: var(--accent);
    border-radius: 2px;
    margin: 12px auto 16px;
    box-shadow: 0 0 12px rgba(0,255,136,0.4);
  }

  .pill-row { display: flex; gap: 10px; justify-content: center; margin-top: 16px; }
  .pill {
    display: inline-flex; align-items: center; gap: 6px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 24px;
    padding: 7px 18px;
    font-size: 0.65em;
    color: var(--muted);
    letter-spacing: 0.06em;
    text-transform: uppercase;
    font-weight: 500;
  }

  .diff-bar {
    display: flex; gap: 3px; align-items: center;
  }
  .diff-bar .bar-seg {
    width: 24px; height: 8px;
    border-radius: 2px;
    background: var(--border);
  }
  .diff-bar .bar-seg.active {
    background: var(--accent);
    box-shadow: 0 0 6px rgba(0,255,136,0.3);
  }

  .exercise-card {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 14px;
    padding: 16px 14px;
    text-align: center;
    flex: 1;
    position: relative;
    min-width: 0;
  }
  .exercise-card h4 {
    font-family: 'Rajdhani', sans-serif;
    font-weight: 700; font-size: 0.9em; color: var(--light);
    margin: 8px 0 6px; text-transform: uppercase; letter-spacing: 0.04em;
  }
  .exercise-card .sets-badge {
    display: inline-block;
    background: rgba(0,255,136,0.1);
    border: 1px solid rgba(0,255,136,0.3);
    border-radius: 8px;
    padding: 3px 10px;
    font-size: 0.7em;
    color: var(--accent);
    font-weight: 600;
    font-family: 'Rajdhani', sans-serif;
  }
  .exercise-card .rest {
    font-size: 0.6em; color: var(--muted); margin-top: 6px;
  }
  .exercise-card .overload-arrow {
    position: absolute;
    top: 8px; right: 10px;
    color: var(--accent);
    font-size: 0.7em;
    font-weight: 700;
  }

  .muscle-tag {
    display: inline-flex; align-items: center; gap: 4px;
    font-size: 0.55em; color: var(--accent); font-weight: 500;
    text-transform: uppercase; letter-spacing: 0.08em;
    background: rgba(0,255,136,0.06);
    border-radius: 6px;
    padding: 2px 8px;
    margin-top: 4px;
  }

  .day-col {
    flex: 1;
    text-align: center;
    min-width: 0;
  }
  .day-header {
    font-family: 'Rajdhani', sans-serif;
    font-size: 0.7em;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    padding: 6px 0;
    border-radius: 8px 8px 0 0;
    color: var(--dark);
  }
  .day-body {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 0 0 8px 8px;
    padding: 10px 6px;
    min-height: 120px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .day-lift {
    font-size: 0.58em;
    color: var(--light);
    font-weight: 500;
    line-height: 1.4;
  }
  .vol-bar-bg {
    width: 100%;
    height: 6px;
    background: var(--border);
    border-radius: 3px;
    margin-top: auto;
    overflow: hidden;
  }
  .vol-bar-fill {
    height: 100%;
    border-radius: 3px;
    background: linear-gradient(90deg, var(--accent), var(--accent-hover));
  }

  @keyframes glow {
    0%, 100% { box-shadow: 0 0 8px rgba(0,255,136,0.15); }
    50% { box-shadow: 0 0 20px rgba(0,255,136,0.35); }
  }

footer: ''
---

<!-- _class: lead -->

<svg width="80" height="80" viewBox="0 0 80 80" fill="none">
  <rect x="5" y="34" width="70" height="12" rx="6" fill="none" stroke="var(--accent)" stroke-width="2.5"/>
  <rect x="15" y="28" width="10" height="24" rx="3" fill="none" stroke="var(--accent)" stroke-width="2"/>
  <rect x="55" y="28" width="10" height="24" rx="3" fill="none" stroke="var(--accent)" stroke-width="2"/>
  <rect x="8" y="30" width="6" height="20" rx="2" fill="none" stroke="var(--accent)" stroke-width="2"/>
  <rect x="66" y="30" width="6" height="20" rx="2" fill="none" stroke="var(--accent)" stroke-width="2"/>
  <line x1="25" y1="40" x2="55" y2="40" stroke="var(--accent)" stroke-width="2" stroke-dasharray="4 3" opacity="0.4"/>
</svg>

# Push Pull Legs

<div class="divider"></div>

The proven split for hypertrophy and strength

<div class="pill-row">
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><rect x="3" y="4" width="18" height="18" rx="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg>
    6-Day Split
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
    60-75 Min
  </span>
  <span class="pill" style="display: inline-flex; align-items: center; gap: 8px;">
    Difficulty
    <span class="diff-bar">
      <span class="bar-seg active"></span>
      <span class="bar-seg active"></span>
      <span class="bar-seg active"></span>
      <span class="bar-seg active"></span>
      <span class="bar-seg"></span>
    </span>
  </span>
</div>

---

### Today's Session

## Push Day A

<div style="display: flex; gap: 14px; margin-top: 20px;">

  <div class="exercise-card" style="animation: glow 3s ease-in-out infinite;">
    <div class="overload-arrow">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2.5"><path d="M12 19V5M5 12l7-7 7 7"/></svg>
    </div>
    <svg width="36" height="36" viewBox="0 0 40 40" fill="none" stroke="var(--accent)" stroke-width="1.5">
      <circle cx="20" cy="8" r="4"/>
      <line x1="20" y1="12" x2="20" y2="26"/>
      <line x1="12" y1="18" x2="20" y2="14"/>
      <line x1="28" y1="18" x2="20" y2="14"/>
      <line x1="15" y1="36" x2="20" y2="26"/>
      <line x1="25" y1="36" x2="20" y2="26"/>
      <rect x="8" y="16" width="6" height="4" rx="1" opacity="0.5"/>
      <rect x="26" y="16" width="6" height="4" rx="1" opacity="0.5"/>
    </svg>
    <h4>Bench Press</h4>
    <div class="sets-badge">4 x 6-8</div>
    <div class="muscle-tag">
      <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><circle cx="12" cy="12" r="10"/></svg>
      Chest
    </div>
    <div class="rest">Rest: 3 min</div>
  </div>

  <div class="exercise-card" style="animation: glow 3s ease-in-out 0.5s infinite;">
    <div class="overload-arrow">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2.5"><path d="M12 19V5M5 12l7-7 7 7"/></svg>
    </div>
    <svg width="36" height="36" viewBox="0 0 40 40" fill="none" stroke="var(--accent)" stroke-width="1.5">
      <circle cx="20" cy="8" r="4"/>
      <line x1="20" y1="12" x2="20" y2="26"/>
      <line x1="10" y1="22" x2="20" y2="16"/>
      <line x1="30" y1="22" x2="20" y2="16"/>
      <line x1="15" y1="36" x2="20" y2="26"/>
      <line x1="25" y1="36" x2="20" y2="26"/>
      <line x1="10" y1="10" x2="10" y2="22" stroke-dasharray="2 1" opacity="0.4"/>
      <line x1="30" y1="10" x2="30" y2="22" stroke-dasharray="2 1" opacity="0.4"/>
    </svg>
    <h4>OHP</h4>
    <div class="sets-badge">4 x 8-10</div>
    <div class="muscle-tag">
      <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/></svg>
      Shoulders
    </div>
    <div class="rest">Rest: 2 min</div>
  </div>

  <div class="exercise-card" style="animation: glow 3s ease-in-out 1s infinite;">
    <svg width="36" height="36" viewBox="0 0 40 40" fill="none" stroke="var(--accent)" stroke-width="1.5">
      <circle cx="20" cy="8" r="4"/>
      <line x1="20" y1="12" x2="20" y2="26"/>
      <path d="M12 16 L20 14 L28 16 L28 20 L20 22 L12 20 Z" fill="none" opacity="0.6"/>
      <line x1="15" y1="36" x2="20" y2="26"/>
      <line x1="25" y1="36" x2="20" y2="26"/>
    </svg>
    <h4>Incline DB</h4>
    <div class="sets-badge">3 x 10-12</div>
    <div class="muscle-tag">
      <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><circle cx="12" cy="12" r="10"/></svg>
      Upper Chest
    </div>
    <div class="rest">Rest: 90s</div>
  </div>

  <div class="exercise-card" style="animation: glow 3s ease-in-out 1.5s infinite;">
    <svg width="36" height="36" viewBox="0 0 40 40" fill="none" stroke="var(--accent)" stroke-width="1.5">
      <circle cx="20" cy="8" r="4"/>
      <line x1="20" y1="12" x2="20" y2="26"/>
      <line x1="12" y1="20" x2="20" y2="16"/>
      <line x1="28" y1="20" x2="20" y2="16"/>
      <circle cx="12" cy="22" r="3" opacity="0.5"/>
      <circle cx="28" cy="22" r="3" opacity="0.5"/>
      <line x1="15" y1="36" x2="20" y2="26"/>
      <line x1="25" y1="36" x2="20" y2="26"/>
    </svg>
    <h4>Lateral Raise</h4>
    <div class="sets-badge">3 x 15-20</div>
    <div class="muscle-tag">
      <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/></svg>
      Side Delts
    </div>
    <div class="rest">Rest: 60s</div>
  </div>

  <div class="exercise-card" style="animation: glow 3s ease-in-out 2s infinite;">
    <svg width="36" height="36" viewBox="0 0 40 40" fill="none" stroke="var(--accent)" stroke-width="1.5">
      <circle cx="20" cy="8" r="4"/>
      <line x1="20" y1="12" x2="20" y2="26"/>
      <line x1="14" y1="18" x2="20" y2="14"/>
      <line x1="26" y1="18" x2="20" y2="14"/>
      <line x1="14" y1="18" x2="14" y2="24"/>
      <line x1="26" y1="18" x2="26" y2="24"/>
      <line x1="15" y1="36" x2="20" y2="26"/>
      <line x1="25" y1="36" x2="20" y2="26"/>
    </svg>
    <h4>Tricep Pushdown</h4>
    <div class="sets-badge">3 x 12-15</div>
    <div class="muscle-tag">
      <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77"/></svg>
      Triceps
    </div>
    <div class="rest">Rest: 60s</div>
  </div>

</div>

<div style="display: flex; justify-content: center; gap: 24px; margin-top: 16px;">
  <div style="display: flex; align-items: center; gap: 6px; font-size: 0.65em; color: var(--muted);">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2.5"><path d="M12 19V5M5 12l7-7 7 7"/></svg>
    <span>= Progressive overload target (add weight this week)</span>
  </div>
</div>

---

### Weekly Overview

## The 6-Day Split

<div style="display: flex; gap: 6px; margin-top: 18px;">

  <!-- Monday -->
  <div class="day-col">
    <div class="day-header" style="background: var(--accent);">Mon</div>
    <div class="day-body" style="border-top: 2px solid var(--accent);">
      <div style="font-size: 0.6em; font-weight: 700; color: var(--accent); text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 4px;">Push A</div>
      <div class="day-lift">Bench 4x6</div>
      <div class="day-lift">OHP 4x8</div>
      <div class="day-lift">Inc DB 3x10</div>
      <div class="day-lift">Lat Raise 3x15</div>
      <div class="day-lift">Tri Push 3x12</div>
      <div class="vol-bar-bg"><div class="vol-bar-fill" style="width: 75%;"></div></div>
    </div>
  </div>

  <!-- Tuesday -->
  <div class="day-col">
    <div class="day-header" style="background: #0088ff; color: #fff;">Tue</div>
    <div class="day-body" style="border-top: 2px solid #0088ff;">
      <div style="font-size: 0.6em; font-weight: 700; color: #0088ff; text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 4px;">Pull A</div>
      <div class="day-lift">Deadlift 3x5</div>
      <div class="day-lift">Pullups 4x8</div>
      <div class="day-lift">Row 4x10</div>
      <div class="day-lift">Face Pull 3x15</div>
      <div class="day-lift">Curl 3x12</div>
      <div class="vol-bar-bg"><div class="vol-bar-fill" style="width: 80%; background: linear-gradient(90deg, #0088ff, #33aaff);"></div></div>
    </div>
  </div>

  <!-- Wednesday -->
  <div class="day-col">
    <div class="day-header" style="background: #ff4444; color: #fff;">Wed</div>
    <div class="day-body" style="border-top: 2px solid #ff4444;">
      <div style="font-size: 0.6em; font-weight: 700; color: #ff4444; text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 4px;">Legs A</div>
      <div class="day-lift">Squat 4x6</div>
      <div class="day-lift">RDL 3x10</div>
      <div class="day-lift">Leg Press 3x12</div>
      <div class="day-lift">Leg Curl 3x12</div>
      <div class="day-lift">Calf Raise 4x15</div>
      <div class="vol-bar-bg"><div class="vol-bar-fill" style="width: 90%; background: linear-gradient(90deg, #ff4444, #ff6666);"></div></div>
    </div>
  </div>

  <!-- Thursday -->
  <div class="day-col">
    <div class="day-header" style="background: var(--accent);">Thu</div>
    <div class="day-body" style="border-top: 2px solid var(--accent);">
      <div style="font-size: 0.6em; font-weight: 700; color: var(--accent); text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 4px;">Push B</div>
      <div class="day-lift">Inc Bench 4x8</div>
      <div class="day-lift">DB Press 4x10</div>
      <div class="day-lift">Cable Fly 3x12</div>
      <div class="day-lift">Lat Raise 4x15</div>
      <div class="day-lift">Overhead Ext 3x12</div>
      <div class="vol-bar-bg"><div class="vol-bar-fill" style="width: 70%;"></div></div>
    </div>
  </div>

  <!-- Friday -->
  <div class="day-col">
    <div class="day-header" style="background: #0088ff; color: #fff;">Fri</div>
    <div class="day-body" style="border-top: 2px solid #0088ff;">
      <div style="font-size: 0.6em; font-weight: 700; color: #0088ff; text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 4px;">Pull B</div>
      <div class="day-lift">Barbell Row 4x8</div>
      <div class="day-lift">Lat Pull 4x10</div>
      <div class="day-lift">Cable Row 3x12</div>
      <div class="day-lift">Rear Delt 3x15</div>
      <div class="day-lift">Hammer Curl 3x12</div>
      <div class="vol-bar-bg"><div class="vol-bar-fill" style="width: 75%; background: linear-gradient(90deg, #0088ff, #33aaff);"></div></div>
    </div>
  </div>

  <!-- Saturday -->
  <div class="day-col">
    <div class="day-header" style="background: #ff4444; color: #fff;">Sat</div>
    <div class="day-body" style="border-top: 2px solid #ff4444;">
      <div style="font-size: 0.6em; font-weight: 700; color: #ff4444; text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 4px;">Legs B</div>
      <div class="day-lift">Front Squat 4x8</div>
      <div class="day-lift">Hip Thrust 3x10</div>
      <div class="day-lift">Lunge 3x12</div>
      <div class="day-lift">Leg Ext 3x15</div>
      <div class="day-lift">Calf Raise 4x15</div>
      <div class="vol-bar-bg"><div class="vol-bar-fill" style="width: 85%; background: linear-gradient(90deg, #ff4444, #ff6666);"></div></div>
    </div>
  </div>

  <!-- Sunday -->
  <div class="day-col">
    <div class="day-header" style="background: #333; color: #888;">Sun</div>
    <div class="day-body" style="border-top: 2px solid #333; justify-content: center; align-items: center;">
      <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#444" stroke-width="1.5">
        <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
        <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
      </svg>
      <div style="font-size: 0.6em; font-weight: 600; color: #555; text-transform: uppercase; letter-spacing: 0.1em; margin-top: 4px;">Rest</div>
      <div style="font-size: 0.5em; color: #444; margin-top: 2px;">Recovery</div>
      <div class="vol-bar-bg"><div class="vol-bar-fill" style="width: 0%;"></div></div>
    </div>
  </div>

</div>

<div style="display: flex; justify-content: center; gap: 20px; margin-top: 14px;">
  <div style="display: flex; align-items: center; gap: 6px; font-size: 0.6em; color: var(--muted);">
    <div style="width: 10px; height: 10px; border-radius: 2px; background: var(--accent);"></div> Push
  </div>
  <div style="display: flex; align-items: center; gap: 6px; font-size: 0.6em; color: var(--muted);">
    <div style="width: 10px; height: 10px; border-radius: 2px; background: #0088ff;"></div> Pull
  </div>
  <div style="display: flex; align-items: center; gap: 6px; font-size: 0.6em; color: var(--muted);">
    <div style="width: 10px; height: 10px; border-radius: 2px; background: #ff4444;"></div> Legs
  </div>
  <div style="display: flex; align-items: center; gap: 6px; font-size: 0.6em; color: var(--muted);">
    <div style="width: 10px; height: 10px; border-radius: 2px; background: #333;"></div> Rest
  </div>
  <div style="display: flex; align-items: center; gap: 6px; font-size: 0.6em; color: var(--muted);">
    <svg width="10" height="6" viewBox="0 0 40 10"><rect width="40" height="6" rx="3" fill="var(--border)"/><rect width="30" height="6" rx="3" fill="var(--accent)"/></svg>
    Volume Tracker
  </div>
</div>
