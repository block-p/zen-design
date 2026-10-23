---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Cinzel:wght@400;500;600;700;800;900&family=Lora:ital,wght@0,400;0,500;0,600;0,700;1,400&display=swap');

  :root {
    --accent: #d4af37;
    --accent-hover: #e8c84a;
    --dark: #0f0f0f;
    --card: #1a1a1a;
    --border: #2a2a2a;
    --muted: #8a8070;
    --light: #f0e6d2;
  }

  section {
    background: var(--dark);
    color: var(--light);
    font-family: 'Lora', serif;
    font-weight: 400;
    padding: 50px 70px;
    line-height: 1.6;
  }

  h1 {
    font-family: 'Cinzel', serif;
    font-weight: 900;
    font-size: 3.6em;
    color: var(--accent);
    letter-spacing: 0.06em;
    line-height: 1.1;
    margin-bottom: 0.1em;
  }

  h2 {
    font-family: 'Cinzel', serif;
    font-weight: 700;
    font-size: 1.7em;
    color: var(--light);
    margin-bottom: 0.3em;
    letter-spacing: 0.03em;
  }

  h3 {
    font-family: 'Cinzel', serif;
    font-weight: 500;
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
    background: radial-gradient(ellipse at 50% 80%, #1f1a0e 0%, var(--dark) 70%);
  }
  section.lead h1 { font-size: 4.2em; }

  .gold-line {
    width: 80px; height: 2px;
    background: linear-gradient(90deg, transparent, var(--accent), transparent);
    margin: 14px auto 18px;
  }

  .menu-badge {
    display: inline-flex; align-items: center; gap: 8px;
    background: var(--card);
    border: 1px solid var(--accent);
    border-radius: 20px;
    padding: 6px 18px;
    font-size: 0.65em;
    color: var(--accent);
    letter-spacing: 0.1em;
    text-transform: uppercase;
    font-family: 'Cinzel', serif;
  }

  .cocktail-row { display: flex; gap: 24px; margin-top: 14px; }
  .cocktail-card {
    flex: 1;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 16px;
    padding: 22px 18px;
    text-align: center;
    position: relative;
    overflow: hidden;
  }
  .cocktail-card::before {
    content: '';
    position: absolute; top: 0; left: 0; right: 0; height: 2px;
    background: linear-gradient(90deg, transparent, var(--accent), transparent);
  }
  .cocktail-card h4 {
    font-family: 'Cinzel', serif;
    font-weight: 700; font-size: 1em; color: var(--light); margin: 10px 0 4px;
    letter-spacing: 0.04em;
  }
  .cocktail-card .recipe {
    font-size: 0.68em; color: var(--muted); line-height: 1.7; margin-top: 8px;
    text-align: left; padding: 0 4px;
  }

  .layer-stack {
    display: flex; flex-direction: column; align-items: center; gap: 0; margin: 10px auto 0;
    width: 70px;
  }
  .layer {
    width: 100%; height: 14px; border-radius: 2px;
  }

  .diff-dots { display: flex; gap: 4px; justify-content: center; margin-top: 10px; }

  .spirit-grid {
    display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; margin-top: 16px;
  }
  .spirit-card {
    background: var(--card); border: 1px solid var(--border); border-radius: 12px;
    padding: 16px 10px; text-align: center;
  }
  .spirit-card:hover { border-color: var(--accent); }
  .spirit-card h5 {
    font-family: 'Cinzel', serif; font-weight: 600; font-size: 0.75em;
    color: var(--light); margin: 8px 0 0; letter-spacing: 0.04em;
  }

  .mixer-grid {
    display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px; margin-top: 10px;
  }
  .mixer-pill {
    background: var(--card); border: 1px solid var(--border); border-radius: 8px;
    padding: 6px 10px; font-size: 0.62em; color: var(--muted); text-align: center;
    display: flex; align-items: center; gap: 6px; justify-content: center;
  }

  .garnish-row { display: flex; gap: 10px; margin-top: 10px; flex-wrap: wrap; }
  .garnish-item {
    display: flex; align-items: center; gap: 6px;
    background: var(--card); border: 1px solid var(--border); border-radius: 20px;
    padding: 5px 12px; font-size: 0.6em; color: var(--muted);
  }

footer: ''
---

<!-- _class: lead -->
<!-- _paginate: false -->

<svg width="70" height="90" viewBox="0 0 70 90">
  <polygon points="20,10 50,10 58,70 12,70" fill="none" stroke="#d4af37" stroke-width="1.5"/>
  <line x1="20" y1="10" x2="10" y2="2" stroke="#d4af37" stroke-width="1.5"/>
  <line x1="50" y1="10" x2="60" y2="2" stroke="#d4af37" stroke-width="1.5"/>
  <rect x="24" y="35" width="22" height="12" fill="#d4af37" opacity="0.15" rx="1"/>
  <rect x="22" y="48" width="26" height="10" fill="#d4af37" opacity="0.25" rx="1"/>
  <rect x="18" y="59" width="34" height="10" fill="#d4af37" opacity="0.35" rx="1"/>
  <line x1="28" y1="70" x2="28" y2="80" stroke="#d4af37" stroke-width="2"/>
  <line x1="42" y1="70" x2="42" y2="80" stroke="#d4af37" stroke-width="2"/>
  <line x1="22" y1="80" x2="48" y2="80" stroke="#d4af37" stroke-width="2"/>
  <circle cx="55" cy="18" r="5" fill="#d4af37" opacity="0.3"/>
  <circle cx="55" cy="18" r="3" fill="none" stroke="#d4af37" stroke-width="0.5"/>
</svg>

# The Craft Bar

<div class="gold-line"></div>

<div style="color: #8a8070; font-size: 0.9em; font-style: italic; margin-bottom: 14px;">Tonight's Specials</div>

<div style="display: flex; gap: 12px; justify-content: center;">
  <span class="menu-badge">3 Cocktails</span>
  <span class="menu-badge">Curated</span>
  <span class="menu-badge">Handcrafted</span>
</div>

---

### The Menu

## Signature Cocktails

<div class="cocktail-row">
  <div class="cocktail-card">
    <!-- Martini glass SVG -->
    <svg width="50" height="60" viewBox="0 0 50 60">
      <polygon points="10,8 40,8 25,32" fill="none" stroke="#d4af37" stroke-width="1.2"/>
      <line x1="25" y1="32" x2="25" y2="50" stroke="#d4af37" stroke-width="1.2"/>
      <line x1="17" y1="50" x2="33" y2="50" stroke="#d4af37" stroke-width="1.2"/>
      <rect x="14" y="14" width="22" height="6" fill="#c8e6c9" opacity="0.3" rx="1"/>
      <circle cx="18" cy="12" r="3" fill="#8bc34a" opacity="0.4"/>
    </svg>
    <h4>Classic Martini</h4>
    <!-- Ingredient layers -->
    <div class="layer-stack">
      <div class="layer" style="background: #e0e0e0; opacity: 0.6; border-radius: 2px 2px 0 0;"></div>
      <div class="layer" style="background: #c8e6c9; opacity: 0.7;"></div>
      <div class="layer" style="background: #a5d6a7; opacity: 0.8; border-radius: 0 0 2px 2px;"></div>
    </div>
    <div class="recipe">
      60ml London Dry Gin<br>
      10ml Dry Vermouth<br>
      Olive or lemon twist
    </div>
    <!-- Difficulty: 1 dot -->
    <div class="diff-dots">
      <svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#d4af37"/></svg>
      <svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#2a2a2a" stroke="#3a3a3a" stroke-width="1"/></svg>
      <svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#2a2a2a" stroke="#3a3a3a" stroke-width="1"/></svg>
    </div>
    <div style="font-size: 0.55em; color: #5a5040; margin-top: 4px; letter-spacing: 0.1em; text-transform: uppercase;">Easy</div>
  </div>

  <div class="cocktail-card">
    <!-- Highball glass SVG -->
    <svg width="50" height="60" viewBox="0 0 50 60">
      <rect x="13" y="4" width="24" height="48" rx="3" fill="none" stroke="#d4af37" stroke-width="1.2"/>
      <rect x="15" y="28" width="20" height="8" fill="#ffcc80" opacity="0.4" rx="1"/>
      <rect x="15" y="36" width="20" height="8" fill="#ff8a65" opacity="0.5" rx="1"/>
      <rect x="15" y="44" width="20" height="6" fill="#e65100" opacity="0.5" rx="1"/>
      <circle cx="20" cy="14" r="4" fill="#fff9c4" opacity="0.3"/>
      <circle cx="30" cy="18" r="3" fill="#fff9c4" opacity="0.25"/>
    </svg>
    <h4>Tequila Sunrise</h4>
    <!-- Ingredient layers -->
    <div class="layer-stack">
      <div class="layer" style="background: #ffcc80; opacity: 0.7; border-radius: 2px 2px 0 0;"></div>
      <div class="layer" style="background: #ff8a65; opacity: 0.8;"></div>
      <div class="layer" style="background: #e65100; opacity: 0.8;"></div>
      <div class="layer" style="background: #b71c1c; opacity: 0.7; border-radius: 0 0 2px 2px;"></div>
    </div>
    <div class="recipe">
      45ml Tequila Blanco<br>
      90ml Orange Juice<br>
      15ml Grenadine<br>
      Orange slice garnish
    </div>
    <!-- Difficulty: 2 dots -->
    <div class="diff-dots">
      <svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#d4af37"/></svg>
      <svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#d4af37"/></svg>
      <svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#2a2a2a" stroke="#3a3a3a" stroke-width="1"/></svg>
    </div>
    <div style="font-size: 0.55em; color: #5a5040; margin-top: 4px; letter-spacing: 0.1em; text-transform: uppercase;">Medium</div>
  </div>

  <div class="cocktail-card">
    <!-- Coupe glass SVG -->
    <svg width="50" height="60" viewBox="0 0 50 60">
      <path d="M12,22 Q12,6 25,6 Q38,6 38,22" fill="none" stroke="#d4af37" stroke-width="1.2"/>
      <line x1="12" y1="22" x2="38" y2="22" stroke="#d4af37" stroke-width="1.2"/>
      <line x1="25" y1="22" x2="25" y2="46" stroke="#d4af37" stroke-width="1.2"/>
      <ellipse cx="25" cy="48" rx="10" ry="3" fill="none" stroke="#d4af37" stroke-width="1.2"/>
      <rect x="16" y="14" width="18" height="4" fill="#e1bee7" opacity="0.4" rx="1"/>
      <rect x="14" y="18" width="22" height="3" fill="#ce93d8" opacity="0.35" rx="1"/>
    </svg>
    <h4>Aviation</h4>
    <!-- Ingredient layers -->
    <div class="layer-stack">
      <div class="layer" style="background: #e1bee7; opacity: 0.6; border-radius: 2px 2px 0 0;"></div>
      <div class="layer" style="background: #ce93d8; opacity: 0.7;"></div>
      <div class="layer" style="background: #ab47bc; opacity: 0.6; border-radius: 0 0 2px 2px;"></div>
    </div>
    <div class="recipe">
      45ml Gin<br>
      15ml Maraschino Liqueur<br>
      15ml Lemon Juice<br>
      5ml Creme de Violette
    </div>
    <!-- Difficulty: 3 dots -->
    <div class="diff-dots">
      <svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#d4af37"/></svg>
      <svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#d4af37"/></svg>
      <svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#d4af37"/></svg>
    </div>
    <div style="font-size: 0.55em; color: #5a5040; margin-top: 4px; letter-spacing: 0.1em; text-transform: uppercase;">Advanced</div>
  </div>
</div>

---

### Build Your Own

## Create Your Signature

<div style="display: flex; gap: 24px; margin-top: 10px;">
  <div style="flex: 1.3;">
    <div style="font-family: 'Cinzel', serif; font-weight: 600; font-size: 0.75em; color: var(--accent); letter-spacing: 0.12em; text-transform: uppercase; margin-bottom: 10px;">Base Spirit</div>
    <div class="spirit-grid" style="grid-template-columns: repeat(4, 1fr); gap: 10px;">
      <div class="spirit-card">
        <svg width="28" height="44" viewBox="0 0 28 44">
          <rect x="9" y="14" width="10" height="26" rx="2" fill="none" stroke="#d4af37" stroke-width="1"/>
          <rect x="11" y="4" width="6" height="10" rx="1" fill="none" stroke="#d4af37" stroke-width="1"/>
          <rect x="11" y="24" width="6" height="10" fill="#d4af37" opacity="0.2"/>
        </svg>
        <h5>Gin</h5>
      </div>
      <div class="spirit-card">
        <svg width="28" height="44" viewBox="0 0 28 44">
          <rect x="9" y="14" width="10" height="26" rx="2" fill="none" stroke="#d4af37" stroke-width="1"/>
          <rect x="11" y="4" width="6" height="10" rx="1" fill="none" stroke="#d4af37" stroke-width="1"/>
          <rect x="11" y="24" width="6" height="10" fill="#ff8a65" opacity="0.3"/>
        </svg>
        <h5>Vodka</h5>
      </div>
      <div class="spirit-card">
        <svg width="28" height="44" viewBox="0 0 28 44">
          <rect x="9" y="14" width="10" height="26" rx="2" fill="none" stroke="#d4af37" stroke-width="1"/>
          <rect x="11" y="4" width="6" height="10" rx="1" fill="none" stroke="#d4af37" stroke-width="1"/>
          <rect x="11" y="24" width="6" height="10" fill="#a1887f" opacity="0.35"/>
        </svg>
        <h5>Whiskey</h5>
      </div>
      <div class="spirit-card">
        <svg width="28" height="44" viewBox="0 0 28 44">
          <rect x="9" y="14" width="10" height="26" rx="2" fill="none" stroke="#d4af37" stroke-width="1"/>
          <rect x="11" y="4" width="6" height="10" rx="1" fill="none" stroke="#d4af37" stroke-width="1"/>
          <rect x="11" y="24" width="6" height="10" fill="#c8e6c9" opacity="0.3"/>
        </svg>
        <h5>Tequila</h5>
      </div>
    </div>

    <div style="font-family: 'Cinzel', serif; font-weight: 600; font-size: 0.72em; color: var(--accent); letter-spacing: 0.12em; text-transform: uppercase; margin: 14px 0 8px;">Mixers</div>
    <div class="mixer-grid">
      <div class="mixer-pill"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#ffcc80" opacity="0.6"/></svg>Tonic</div>
      <div class="mixer-pill"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#fff9c4" opacity="0.6"/></svg>Soda</div>
      <div class="mixer-pill"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#ffab91" opacity="0.6"/></svg>Ginger Beer</div>
      <div class="mixer-pill"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#ff8a65" opacity="0.6"/></svg>OJ</div>
      <div class="mixer-pill"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#c5e1a5" opacity="0.6"/></svg>Lime Juice</div>
      <div class="mixer-pill"><svg width="10" height="10" viewBox="0 0 10 10"><circle cx="5" cy="5" r="4" fill="#b71c1c" opacity="0.5"/></svg>Cranberry</div>
    </div>

    <div style="font-family: 'Cinzel', serif; font-weight: 600; font-size: 0.72em; color: var(--accent); letter-spacing: 0.12em; text-transform: uppercase; margin: 14px 0 8px;">Garnish</div>
    <div class="garnish-row">
      <div class="garnish-item"><svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="5" fill="#8bc34a" opacity="0.5"/><line x1="3" y1="6" x2="9" y2="6" stroke="#fff" stroke-width="0.5"/></svg>Lime Wheel</div>
      <div class="garnish-item"><svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="4" fill="#4caf50" opacity="0.5"/></svg>Olive</div>
      <div class="garnish-item"><svg width="12" height="12" viewBox="0 0 12 12"><rect x="2" y="4" width="8" height="5" rx="1" fill="#ff8a65" opacity="0.5"/></svg>Orange Peel</div>
      <div class="garnish-item"><svg width="12" height="12" viewBox="0 0 12 12"><path d="M3,9 Q6,2 9,9" fill="#4caf50" opacity="0.5" stroke="#2e7d32" stroke-width="0.5"/></svg>Mint Sprig</div>
      <div class="garnish-item"><svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="4" fill="#c62828" opacity="0.5"/></svg>Cherry</div>
    </div>
  </div>

  <div style="flex: 0.7; display: flex; flex-direction: column; align-items: center; justify-content: center;">
    <div style="font-family: 'Cinzel', serif; font-weight: 600; font-size: 0.72em; color: var(--accent); letter-spacing: 0.12em; text-transform: uppercase; margin-bottom: 14px;">Flavor Profile</div>
    <!-- Radar chart SVG -->
    <svg width="200" height="200" viewBox="0 0 200 200">
      <!-- Background grid -->
      <polygon points="100,30 157,65 157,135 100,170 43,135 43,65" fill="none" stroke="#2a2a2a" stroke-width="0.8"/>
      <polygon points="100,50 143,75 143,125 100,150 57,125 57,75" fill="none" stroke="#2a2a2a" stroke-width="0.5"/>
      <polygon points="100,70 128,85 128,115 100,130 72,115 72,85" fill="none" stroke="#2a2a2a" stroke-width="0.5"/>
      <!-- Axis lines -->
      <line x1="100" y1="30" x2="100" y2="170" stroke="#2a2a2a" stroke-width="0.5"/>
      <line x1="43" y1="65" x2="157" y2="135" stroke="#2a2a2a" stroke-width="0.5"/>
      <line x1="43" y1="135" x2="157" y2="65" stroke="#2a2a2a" stroke-width="0.5"/>
      <!-- Data shape -->
      <polygon points="100,40 150,72 140,140 100,155 55,120 60,68" fill="#d4af37" opacity="0.15" stroke="#d4af37" stroke-width="1.5"/>
      <!-- Data points -->
      <circle cx="100" cy="40" r="4" fill="#d4af37"/>
      <circle cx="150" cy="72" r="4" fill="#d4af37"/>
      <circle cx="140" cy="140" r="4" fill="#d4af37"/>
      <circle cx="100" cy="155" r="4" fill="#d4af37"/>
      <circle cx="55" cy="120" r="4" fill="#d4af37"/>
      <circle cx="60" cy="68" r="4" fill="#d4af37"/>
      <!-- Labels -->
      <text x="100" y="22" text-anchor="middle" fill="#f0e6d2" font-family="Cinzel" font-size="10" font-weight="600">SWEET</text>
      <text x="168" y="65" text-anchor="start" fill="#f0e6d2" font-family="Cinzel" font-size="10" font-weight="600">SOUR</text>
      <text x="168" y="140" text-anchor="start" fill="#f0e6d2" font-family="Cinzel" font-size="10" font-weight="600">BITTER</text>
      <text x="100" y="188" text-anchor="middle" fill="#f0e6d2" font-family="Cinzel" font-size="10" font-weight="600">STRONG</text>
      <text x="30" y="140" text-anchor="end" fill="#f0e6d2" font-family="Cinzel" font-size="10" font-weight="600">SPICY</text>
      <text x="30" y="65" text-anchor="end" fill="#f0e6d2" font-family="Cinzel" font-size="10" font-weight="600">FLORAL</text>
    </svg>
  </div>
</div>
