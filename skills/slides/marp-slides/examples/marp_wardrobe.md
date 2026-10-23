---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Cormorant:wght@400;500;600;700&family=Karla:wght@300;400;500;600;700&display=swap');

  :root {
    --accent: #c9a96e;
    --accent-hover: #dfc28e;
    --dark: #ffffff;
    --card: #f7f5f0;
    --border: #e5e0d5;
    --muted: #8a8578;
    --light: #1a1a1a;
  }

  section {
    background: var(--dark);
    color: var(--light);
    font-family: 'Karla', sans-serif;
    font-weight: 400;
    padding: 50px 70px;
    line-height: 1.6;
  }

  h1 {
    font-family: 'Cormorant', serif;
    font-weight: 700;
    font-size: 3.8em;
    color: var(--light);
    letter-spacing: -0.01em;
    line-height: 1.05;
    margin-bottom: 0.1em;
  }

  h2 {
    font-family: 'Cormorant', serif;
    font-weight: 600;
    font-size: 2em;
    color: var(--light);
    margin-bottom: 0.3em;
  }

  h3 {
    font-family: 'Karla', sans-serif;
    font-weight: 600;
    font-size: 0.7em;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.2em;
    margin-bottom: 0.1em;
  }

  strong { color: var(--accent); font-weight: 700; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: linear-gradient(180deg, #ffffff 0%, #f9f6f0 100%);
  }
  section.lead h1 { font-size: 4.8em; }
  section.lead p { color: var(--muted); font-size: 1em; font-family: 'Karla', sans-serif; }

  .divider {
    width: 50px; height: 2px;
    background: var(--accent);
    margin: 14px auto 18px;
  }

  .pill-row { display: flex; gap: 10px; justify-content: center; margin-top: 14px; }
  .pill {
    display: inline-flex; align-items: center; gap: 6px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 24px;
    padding: 6px 16px;
    font-size: 0.65em;
    color: var(--muted);
    letter-spacing: 0.06em;
    font-weight: 500;
  }

  .swatch {
    display: inline-block;
    width: 12px; height: 12px;
    border-radius: 50%;
    border: 1px solid var(--border);
    vertical-align: middle;
  }

  .item-card {
    background: var(--dark);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 10px 8px;
    text-align: center;
    min-width: 80px;
    flex: 1;
  }
  .item-card .item-name {
    font-size: 0.6em; font-weight: 600; color: var(--light);
    margin-top: 6px; line-height: 1.3;
  }

  .category-label {
    font-family: 'Karla', sans-serif;
    font-size: 0.6em;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.15em;
    color: var(--accent);
    writing-mode: vertical-rl;
    text-orientation: mixed;
    transform: rotate(180deg);
    padding-right: 10px;
  }

  .outfit-col {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }
  .outfit-piece {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 8px 12px;
    text-align: center;
    width: 130px;
    font-size: 0.65em;
    font-weight: 500;
    color: var(--light);
    display: flex;
    align-items: center;
    gap: 6px;
    justify-content: center;
  }
  .outfit-label {
    font-family: 'Cormorant', serif;
    font-size: 0.75em;
    font-weight: 600;
    color: var(--accent);
    margin-top: 8px;
    text-transform: uppercase;
    letter-spacing: 0.1em;
  }
  .connector-line {
    width: 1px; height: 10px;
    background: var(--border);
  }

  .palette-row {
    display: flex; gap: 6px; justify-content: center; margin-top: 12px;
  }
  .palette-dot {
    width: 20px; height: 20px; border-radius: 50%;
    border: 2px solid var(--border);
  }

footer: ''
---

<!-- _class: lead -->

<svg width="80" height="80" viewBox="0 0 80 80" fill="none">
  <path d="M25 20 C25 12 55 12 55 20" stroke="var(--accent)" stroke-width="2.5" fill="none" stroke-linecap="round"/>
  <line x1="40" y1="20" x2="40" y2="8" stroke="var(--accent)" stroke-width="2.5" stroke-linecap="round"/>
  <circle cx="40" cy="6" r="3" fill="none" stroke="var(--accent)" stroke-width="2"/>
  <path d="M25 20 L22 65" stroke="var(--accent)" stroke-width="2" stroke-linecap="round"/>
  <path d="M55 20 L58 65" stroke="var(--accent)" stroke-width="2" stroke-linecap="round"/>
  <path d="M30 35 L50 35" stroke="var(--accent)" stroke-width="1.5" stroke-dasharray="3 2"/>
  <path d="M28 50 L52 50" stroke="var(--accent)" stroke-width="1.5" stroke-dasharray="3 2"/>
</svg>

# Capsule Wardrobe

<div class="divider"></div>

A curated collection of 15 versatile pieces that mix and match endlessly

<div class="pill-row">
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
    15 Pieces
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/></svg>
    Spring / Autumn
  </span>
  <span class="pill">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
    50+ Outfits
  </span>
</div>

<div class="palette-row">
  <div class="palette-dot" style="background: #1a1a1a;"></div>
  <div class="palette-dot" style="background: #f5f0e8;"></div>
  <div class="palette-dot" style="background: #5c534a;"></div>
  <div class="palette-dot" style="background: #c9a96e;"></div>
  <div class="palette-dot" style="background: #8b6f4e;"></div>
  <div class="palette-dot" style="background: #d4cec3;"></div>
  <div class="palette-dot" style="background: #2c3e50;"></div>
</div>

---

### The Collection

## 15 Pieces, 3 Categories

<div style="display: flex; gap: 16px; margin-top: 18px;">

  <!-- TOPS -->
  <div style="flex: 1;">
    <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 10px;">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M20.38 3.46L16 2 12 5 8 2 3.62 3.46A2 2 0 0 0 2 5.4V7a2 2 0 0 0 2 2h1v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V9h1a2 2 0 0 0 2-2V5.4a2 2 0 0 0-1.62-1.94z"/></svg>
      <span style="font-size: 0.7em; font-weight: 700; text-transform: uppercase; letter-spacing: 0.12em; color: var(--accent);">Tops</span>
    </div>
    <div style="display: flex; flex-direction: column; gap: 6px;">
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><path d="M20 8L12 4 4 8v2l8 4 8-4V8z"/><line x1="8" y1="12" x2="8" y2="20"/><line x1="16" y1="12" x2="16" y2="20"/><line x1="8" y1="20" x2="16" y2="20"/></svg><div class="item-name">White Tee <span class="swatch" style="background: #f5f0e8;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><path d="M20 8L12 4 4 8v2l8 4 8-4V8z"/><line x1="8" y1="12" x2="8" y2="20"/><line x1="16" y1="12" x2="16" y2="20"/><line x1="8" y1="20" x2="16" y2="20"/></svg><div class="item-name">Black Tee <span class="swatch" style="background: #1a1a1a;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><path d="M3 7c0 0 2-3 9-3s9 3 9 3v2c0 0-2 2-9 2s-9-2-9-2V7z"/><line x1="6" y1="11" x2="6" y2="21"/><line x1="18" y1="11" x2="18" y2="21"/><path d="M6 21h12"/></svg><div class="item-name">Oxford Shirt <span class="swatch" style="background: #d4cec3;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><path d="M3 7c0 0 2-3 9-3s9 3 9 3v2c0 0-2 2-9 2s-9-2-9-2V7z"/><line x1="6" y1="11" x2="6" y2="21"/><line x1="18" y1="11" x2="18" y2="21"/><path d="M6 21h12"/></svg><div class="item-name">Navy Shirt <span class="swatch" style="background: #2c3e50;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><path d="M12 3L4 7v4c0 0 0 6 8 10 8-4 8-10 8-10V7l-8-4z"/></svg><div class="item-name">Breton Stripe <span class="swatch" style="background: #1a1a1a; width: 6px;"></span><span class="swatch" style="background: #f5f0e8; width: 6px;"></span></div></div>
    </div>
  </div>

  <!-- BOTTOMS -->
  <div style="flex: 1;">
    <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 10px;">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><rect x="4" y="2" width="16" height="5" rx="1"/><path d="M4 7l2 15h4l2-10 2 10h4l2-15"/></svg>
      <span style="font-size: 0.7em; font-weight: 700; text-transform: uppercase; letter-spacing: 0.12em; color: var(--accent);">Bottoms</span>
    </div>
    <div style="display: flex; flex-direction: column; gap: 6px;">
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><rect x="5" y="3" width="14" height="4" rx="1"/><path d="M5 7l2 14h3l2-8 2 8h3l2-14"/></svg><div class="item-name">Dark Denim <span class="swatch" style="background: #2c3e50;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><rect x="5" y="3" width="14" height="4" rx="1"/><path d="M5 7l1 14h12l1-14"/></svg><div class="item-name">Tan Chinos <span class="swatch" style="background: #c9a96e;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><rect x="5" y="3" width="14" height="4" rx="1"/><path d="M5 7l1 14h12l1-14"/></svg><div class="item-name">Black Trousers <span class="swatch" style="background: #1a1a1a;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><rect x="5" y="3" width="14" height="4" rx="1"/><path d="M5 7l2 8h10l2-8"/></svg><div class="item-name">Olive Shorts <span class="swatch" style="background: #5c534a;"></span></div></div>
    </div>
  </div>

  <!-- LAYERS + SHOES -->
  <div style="flex: 1;">
    <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 10px;">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
      <span style="font-size: 0.7em; font-weight: 700; text-transform: uppercase; letter-spacing: 0.12em; color: var(--accent);">Layers + Shoes</span>
    </div>
    <div style="display: flex; flex-direction: column; gap: 6px;">
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><path d="M12 3C8 3 4 5 4 8v12h16V8c0-3-4-5-8-5z"/><path d="M8 20v-6M16 20v-6"/></svg><div class="item-name">Camel Overcoat <span class="swatch" style="background: #c9a96e;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><path d="M4 6c0 0 2-2 8-2s8 2 8 2v14H4V6z"/><line x1="12" y1="4" x2="12" y2="20"/></svg><div class="item-name">Navy Blazer <span class="swatch" style="background: #2c3e50;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><path d="M4 8c0-2 3-4 8-4s8 2 8 4v10c0 2-3 4-8 4s-8-2-8-4V8z"/></svg><div class="item-name">Grey Knit <span class="swatch" style="background: #d4cec3;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><path d="M3 18h18v2H3v-2z"/><path d="M5 18c0-4 2-6 7-6s7 2 7 6"/><path d="M8 12V8M16 12V8"/></svg><div class="item-name">White Sneakers <span class="swatch" style="background: #f5f0e8;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><path d="M3 18h18v2H3v-2z"/><path d="M5 18c0-3 3-8 7-8s7 5 7 8"/></svg><div class="item-name">Brown Boots <span class="swatch" style="background: #8b6f4e;"></span></div></div>
      <div class="item-card"><svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="var(--light)" stroke-width="1.2"><path d="M3 18h18v2H3v-2z"/><path d="M6 18c0-4 2-7 6-7s6 3 6 7"/></svg><div class="item-name">Black Loafers <span class="swatch" style="background: #1a1a1a;"></span></div></div>
    </div>
  </div>

</div>

---

### Mix & Match

## 4 Ready-Made Outfits

<div style="display: flex; gap: 28px; margin-top: 18px; justify-content: center;">

  <!-- Outfit 1 -->
  <div class="outfit-col">
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M20 8L12 4 4 8v2l8 4 8-4V8z"/></svg>
      White Tee
    </div>
    <div class="connector-line"></div>
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><rect x="5" y="3" width="14" height="4" rx="1"/><path d="M5 7l2 14h3l2-8 2 8h3l2-14"/></svg>
      Dark Denim
    </div>
    <div class="connector-line"></div>
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M3 18h18v2H3v-2z"/><path d="M5 18c0-4 2-6 7-6s7 2 7 6"/></svg>
      White Sneakers
    </div>
    <div class="outfit-label">Weekend</div>
  </div>

  <!-- Outfit 2 -->
  <div class="outfit-col">
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M3 7c0 0 2-3 9-3s9 3 9 3v2c0 0-2 2-9 2s-9-2-9-2V7z"/></svg>
      Oxford Shirt
    </div>
    <div class="connector-line"></div>
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><rect x="5" y="3" width="14" height="4" rx="1"/><path d="M5 7l1 14h12l1-14"/></svg>
      Tan Chinos
    </div>
    <div class="connector-line"></div>
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M4 6c0 0 2-2 8-2s8 2 8 2v14H4V6z"/></svg>
      Navy Blazer
    </div>
    <div class="connector-line"></div>
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M3 18h18v2H3v-2z"/><path d="M6 18c0-4 2-7 6-7s6 3 6 7"/></svg>
      Brown Boots
    </div>
    <div class="outfit-label">Smart Casual</div>
  </div>

  <!-- Outfit 3 -->
  <div class="outfit-col">
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M3 7c0 0 2-3 9-3s9 3 9 3v2c0 0-2 2-9 2s-9-2-9-2V7z"/></svg>
      Navy Shirt
    </div>
    <div class="connector-line"></div>
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><rect x="5" y="3" width="14" height="4" rx="1"/><path d="M5 7l1 14h12l1-14"/></svg>
      Black Trousers
    </div>
    <div class="connector-line"></div>
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M12 3C8 3 4 5 4 8v12h16V8c0-3-4-5-8-5z"/></svg>
      Camel Overcoat
    </div>
    <div class="connector-line"></div>
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M3 18h18v2H3v-2z"/><path d="M6 18c0-4 2-7 6-7s6 3 6 7"/></svg>
      Black Loafers
    </div>
    <div class="outfit-label">Date Night</div>
  </div>

  <!-- Outfit 4 -->
  <div class="outfit-col">
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M12 3L4 7v4c0 0 0 6 8 10 8-4 8-10 8-10V7l-8-4z"/></svg>
      Breton Stripe
    </div>
    <div class="connector-line"></div>
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><rect x="5" y="3" width="14" height="4" rx="1"/><path d="M5 7l2 8h10l2-8"/></svg>
      Olive Shorts
    </div>
    <div class="connector-line"></div>
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M4 8c0-2 3-4 8-4s8 2 8 4v10c0 2-3 4-8 4s-8-2-8-4V8z"/></svg>
      Grey Knit
    </div>
    <div class="connector-line"></div>
    <div class="outfit-piece">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M3 18h18v2H3v-2z"/><path d="M5 18c0-4 2-6 7-6s7 2 7 6"/></svg>
      White Sneakers
    </div>
    <div class="outfit-label">Brunch</div>
  </div>

</div>

<div style="display: flex; justify-content: center; margin-top: 22px;">
  <div style="background: var(--card); border: 1px solid var(--border); border-radius: 12px; padding: 12px 28px; display: flex; align-items: center; gap: 10px;">
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
    <span style="font-size: 0.7em; color: var(--muted);">Every piece pairs with at least <strong>4 others</strong> — no dead items</span>
  </div>
</div>
