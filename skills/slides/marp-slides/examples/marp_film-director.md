---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Playfair+Display:wght@400;600;700;800;900&family=DM+Sans:wght@300;400;500;600;700&display=swap');

  :root {
    --accent: #e8a87c;
    --accent-hover: #f0c4a8;
    --secondary: #85cdca;
    --tertiary: #d4a5a5;
    --dark: #fdf0e0;
    --card: #f5e4d0;
    --border: #e0cdb8;
    --muted: #7a7a7a;
    --light: #4a4a4a;
  }

  section {
    background: var(--dark);
    color: var(--light);
    font-family: 'DM Sans', sans-serif;
    font-weight: 400;
    padding: 60px 80px;
    line-height: 1.6;
  }

  h1 {
    font-family: 'Playfair Display', serif;
    font-weight: 900;
    font-size: 4em;
    color: var(--light);
    letter-spacing: -0.02em;
    line-height: 1.05;
    margin-bottom: 0.15em;
  }

  h2 {
    font-family: 'Playfair Display', serif;
    font-weight: 700;
    font-size: 1.9em;
    color: var(--light);
    margin-bottom: 0.4em;
  }

  h3 {
    font-family: 'DM Sans', sans-serif;
    font-weight: 600;
    font-size: 0.75em;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.2em;
    margin-bottom: 0.1em;
  }

  strong { color: var(--light); font-weight: 700; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: var(--dark);
  }
  section.lead h1 {
    font-size: 5em;
    font-style: italic;
  }
  section.lead p { color: var(--muted); font-size: 1em; }

  .tag {
    display: inline-block;
    background: transparent;
    border: 1.5px solid var(--border);
    border-radius: 20px;
    padding: 5px 16px;
    font-size: 0.68em;
    color: var(--muted);
    letter-spacing: 0.06em;
  }
  .tag-row { display: flex; gap: 8px; justify-content: center; margin-top: 16px; flex-wrap: wrap; }

  .stat-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 8px 16px;
    font-size: 0.72em;
    color: var(--muted);
  }
  .stat-badge strong {
    font-family: 'Playfair Display', serif;
    font-size: 1.4em;
    color: var(--light);
  }

  .film-strip {
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin-top: 16px;
  }
  .film-entry {
    display: flex;
    align-items: center;
    gap: 16px;
  }
  .film-title {
    font-family: 'Playfair Display', serif;
    font-weight: 600;
    font-size: 0.8em;
    color: var(--light);
    min-width: 160px;
    text-align: right;
  }
  .film-year {
    font-size: 0.65em;
    color: var(--muted);
    min-width: 40px;
  }
  .swatch-row { display: flex; gap: 3px; }
  .swatch {
    width: 80px;
    height: 36px;
    border-radius: 4px;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    padding-bottom: 3px;
  }
  .swatch-label {
    font-size: 0.45em;
    color: rgba(255,255,255,0.8);
    text-shadow: 0 1px 2px rgba(0,0,0,0.5);
    font-weight: 600;
    letter-spacing: 0.04em;
  }
  .swatch-label-dark {
    font-size: 0.45em;
    color: rgba(0,0,0,0.6);
    font-weight: 600;
    letter-spacing: 0.04em;
  }

  .technique-col {
    flex: 1;
    text-align: center;
    padding: 20px 16px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 14px;
  }
  .technique-col h4 {
    font-family: 'Playfair Display', serif;
    font-weight: 700;
    font-size: 1em;
    color: var(--light);
    margin: 12px 0 6px;
  }
  .technique-col p {
    font-size: 0.7em;
    color: var(--muted);
    line-height: 1.5;
    margin: 0;
  }

  .divider-line {
    width: 50px;
    height: 2px;
    background: var(--accent);
    margin: 12px auto 16px;
    border-radius: 2px;
  }

footer: ''
---

<!-- _class: lead -->

<div style="margin-bottom: 8px;">
  <svg width="64" height="64" viewBox="0 0 64 64">
    <!-- Film camera icon -->
    <rect x="8" y="18" width="36" height="28" rx="4" fill="none" stroke="#e8a87c" stroke-width="2.5"/>
    <polygon points="44,24 58,16 58,48 44,40" fill="none" stroke="#e8a87c" stroke-width="2.5" stroke-linejoin="round"/>
    <circle cx="22" cy="14" r="6" fill="none" stroke="#85cdca" stroke-width="2"/>
    <circle cx="36" cy="14" r="6" fill="none" stroke="#85cdca" stroke-width="2"/>
  </svg>
</div>

# Wes Anderson

<div class="divider-line"></div>

A Visual Study

<div style="display: flex; gap: 12px; justify-content: center; margin-top: 20px;">
  <div class="stat-badge">
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#e8a87c" stroke-width="1.5"><rect x="2" y="2" width="20" height="20" rx="2"/><path d="M10 2v20"/><path d="M2 10h20"/></svg>
    <strong>11</strong> Feature Films
  </div>
  <div class="stat-badge">
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#85cdca" stroke-width="1.5"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
    <strong>7</strong> Oscar Noms
  </div>
  <div class="stat-badge">
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#d4a5a5" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
    <strong>1996</strong> - Present
  </div>
</div>

<div class="tag-row">
  <span class="tag">Symmetry</span>
  <span class="tag">Pastels</span>
  <span class="tag">Deadpan Humor</span>
  <span class="tag">Ensemble Casts</span>
  <span class="tag">Miniatures</span>
</div>

---

### Color Palette Analysis

## Signature Palettes by Film

<div class="film-strip">
  <!-- The Grand Budapest Hotel -->
  <div class="film-entry">
    <div class="film-year">2014</div>
    <div class="film-title">The Grand Budapest Hotel</div>
    <div class="swatch-row">
      <div class="swatch" style="background: #d4508b;"><span class="swatch-label">#D4508B</span></div>
      <div class="swatch" style="background: #7b2d8e;"><span class="swatch-label">#7B2D8E</span></div>
      <div class="swatch" style="background: #f5d0c5;"><span class="swatch-label-dark">#F5D0C5</span></div>
      <div class="swatch" style="background: #c41e3a;"><span class="swatch-label">#C41E3A</span></div>
      <div class="swatch" style="background: #2c1810;"><span class="swatch-label">#2C1810</span></div>
    </div>
  </div>

  <!-- Moonrise Kingdom -->
  <div class="film-entry">
    <div class="film-year">2012</div>
    <div class="film-title">Moonrise Kingdom</div>
    <div class="swatch-row">
      <div class="swatch" style="background: #e8c170;"><span class="swatch-label-dark">#E8C170</span></div>
      <div class="swatch" style="background: #5b7f5e;"><span class="swatch-label">#5B7F5E</span></div>
      <div class="swatch" style="background: #c9884b;"><span class="swatch-label">#C9884B</span></div>
      <div class="swatch" style="background: #8b6f47;"><span class="swatch-label">#8B6F47</span></div>
      <div class="swatch" style="background: #d4a574;"><span class="swatch-label-dark">#D4A574</span></div>
    </div>
  </div>

  <!-- The Royal Tenenbaums -->
  <div class="film-entry">
    <div class="film-year">2001</div>
    <div class="film-title">The Royal Tenenbaums</div>
    <div class="swatch-row">
      <div class="swatch" style="background: #c41e3a;"><span class="swatch-label">#C41E3A</span></div>
      <div class="swatch" style="background: #e8d5b7;"><span class="swatch-label-dark">#E8D5B7</span></div>
      <div class="swatch" style="background: #4a6741;"><span class="swatch-label">#4A6741</span></div>
      <div class="swatch" style="background: #8b4513;"><span class="swatch-label">#8B4513</span></div>
      <div class="swatch" style="background: #d4a07a;"><span class="swatch-label-dark">#D4A07A</span></div>
    </div>
  </div>
</div>

<div style="display: flex; gap: 16px; margin-top: 20px;">
  <div style="flex: 1; text-align: center; padding: 10px; background: var(--card); border-radius: 8px; border: 1px solid var(--border);">
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#e8a87c" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><path d="M12 8v4l3 3"/></svg>
    <div style="font-size: 0.65em; color: var(--muted); margin-top: 4px;">Each palette restricts to <strong style="color: var(--light);">3-5 dominant hues</strong> per film</div>
  </div>
  <div style="flex: 1; text-align: center; padding: 10px; background: var(--card); border-radius: 8px; border: 1px solid var(--border);">
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#85cdca" stroke-width="1.5"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
    <div style="font-size: 0.65em; color: var(--muted); margin-top: 4px;">Wardrobe, sets, and props all conform to the <strong style="color: var(--light);">same restricted palette</strong></div>
  </div>
</div>

---

### Signature Techniques

## The Anderson Toolkit

<div style="display: flex; gap: 20px; margin-top: 18px;">

  <!-- Framing Column -->
  <div class="technique-col">
    <svg width="120" height="90" viewBox="0 0 120 90">
      <!-- Frame border -->
      <rect x="5" y="5" width="110" height="80" rx="3" fill="none" stroke="#e8a87c" stroke-width="2"/>
      <!-- Center crosshair (symmetry axis) -->
      <line x1="60" y1="5" x2="60" y2="85" stroke="#e8a87c" stroke-width="1" stroke-dasharray="4,3" opacity="0.6"/>
      <line x1="5" y1="45" x2="115" y2="45" stroke="#e8a87c" stroke-width="1" stroke-dasharray="4,3" opacity="0.6"/>
      <!-- Centered figure silhouette -->
      <circle cx="60" cy="32" r="8" fill="none" stroke="#4a4a4a" stroke-width="1.5"/>
      <line x1="60" y1="40" x2="60" y2="60" stroke="#4a4a4a" stroke-width="1.5"/>
      <line x1="50" y1="50" x2="70" y2="50" stroke="#4a4a4a" stroke-width="1.5"/>
      <line x1="60" y1="60" x2="52" y2="72" stroke="#4a4a4a" stroke-width="1.5"/>
      <line x1="60" y1="60" x2="68" y2="72" stroke="#4a4a4a" stroke-width="1.5"/>
      <!-- Symmetry arrows -->
      <polygon points="22,45 28,41 28,49" fill="#e8a87c" opacity="0.5"/>
      <polygon points="98,45 92,41 92,49" fill="#e8a87c" opacity="0.5"/>
    </svg>
    <h4>Framing</h4>
    <div class="divider-line"></div>
    <p><strong style="color: var(--accent);">Dead-center symmetry</strong> is the hallmark. Subjects placed on the exact vertical axis. Doorways, hallways, and windows used as nested frames within frames.</p>
  </div>

  <!-- Color Column -->
  <div class="technique-col">
    <svg width="120" height="90" viewBox="0 0 120 90">
      <!-- Color wheel segments -->
      <circle cx="60" cy="45" r="35" fill="none" stroke="#e0cdb8" stroke-width="1"/>
      <path d="M60 10 A35 35 0 0 1 90.3 27.5 L60 45 Z" fill="#e8a87c" opacity="0.7"/>
      <path d="M90.3 27.5 A35 35 0 0 1 90.3 62.5 L60 45 Z" fill="#d4a5a5" opacity="0.7"/>
      <path d="M90.3 62.5 A35 35 0 0 1 60 80 L60 45 Z" fill="#85cdca" opacity="0.7"/>
      <path d="M60 80 A35 35 0 0 1 29.7 62.5 L60 45 Z" fill="#f5d0c5" opacity="0.7"/>
      <path d="M29.7 62.5 A35 35 0 0 1 29.7 27.5 L60 45 Z" fill="#e8c170" opacity="0.7"/>
      <path d="M29.7 27.5 A35 35 0 0 1 60 10 L60 45 Z" fill="#d4508b" opacity="0.5"/>
      <!-- Center dot -->
      <circle cx="60" cy="45" r="8" fill="var(--dark)" stroke="#e0cdb8" stroke-width="1.5"/>
      <circle cx="60" cy="45" r="3" fill="#e8a87c"/>
    </svg>
    <h4>Color</h4>
    <div class="divider-line"></div>
    <p><strong style="color: var(--secondary);">Restricted palettes</strong> per film. Every prop, wall, costume, and vehicle is color-coordinated. Pastels dominate, with strategic saturated accents for contrast.</p>
  </div>

  <!-- Movement Column -->
  <div class="technique-col">
    <svg width="120" height="90" viewBox="0 0 120 90">
      <!-- Lateral tracking line -->
      <line x1="10" y1="45" x2="110" y2="45" stroke="#85cdca" stroke-width="2" stroke-dasharray="6,4"/>
      <!-- Camera icon moving along line -->
      <g transform="translate(60, 45)">
        <rect x="-14" y="-10" width="28" height="20" rx="3" fill="none" stroke="#4a4a4a" stroke-width="1.5"/>
        <circle cx="0" cy="0" r="6" fill="none" stroke="#85cdca" stroke-width="1.5"/>
        <circle cx="0" cy="0" r="2" fill="#85cdca"/>
      </g>
      <!-- Direction arrows -->
      <polygon points="15,45 22,40 22,50" fill="#85cdca" opacity="0.6"/>
      <polygon points="105,45 98,40 98,50" fill="#85cdca" opacity="0.6"/>
      <!-- Vertical whip pan indicator -->
      <line x1="38" y1="15" x2="38" y2="75" stroke="#e8a87c" stroke-width="1.5" stroke-dasharray="3,3" opacity="0.5"/>
      <polygon points="38,15 34,22 42,22" fill="#e8a87c" opacity="0.5"/>
      <polygon points="38,75 34,68 42,68" fill="#e8a87c" opacity="0.5"/>
      <text x="44" y="18" fill="#e8a87c" font-size="6" font-family="DM Sans" opacity="0.7">WHIP</text>
      <!-- Slow-mo indicator -->
      <g transform="translate(85, 22)">
        <circle r="10" fill="none" stroke="#d4a5a5" stroke-width="1" opacity="0.6"/>
        <path d="M0,-6 L0,0 L4,2" fill="none" stroke="#d4a5a5" stroke-width="1.2"/>
        <text x="0" y="16" text-anchor="middle" fill="#d4a5a5" font-size="5" font-family="DM Sans" opacity="0.7">SLO-MO</text>
      </g>
    </svg>
    <h4>Movement</h4>
    <div class="divider-line"></div>
    <p><strong style="color: var(--tertiary);">Lateral tracking shots</strong> and whip pans. Camera moves side-to-side like a dollhouse. Slow motion for emotional peaks. Minimal handheld work.</p>
  </div>

</div>
