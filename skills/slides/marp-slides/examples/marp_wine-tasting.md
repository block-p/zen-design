---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Cormorant+Garamond:wght@300;400;500;600;700&family=Inter:wght@300;400;500&display=swap');

  :root {
    --accent: #c9a84c;
    --accent-hover: #dfc06a;
    --dark: #2d0a0a;
    --card: #3a1212;
    --border: #4d1c1c;
    --muted: #a08070;
    --light: #f5ede4;
    --wine-red: #8b1a2b;
    --wine-white: #f0e6a0;
    --wine-rose: #e8a0b0;
  }

  section {
    background: var(--dark);
    color: var(--light);
    font-family: 'Inter', sans-serif;
    font-weight: 300;
    padding: 56px 72px;
    line-height: 1.5;
  }

  section::after { font-family: 'Inter'; font-size: 0.55em; color: #3a1212; }

  h1 {
    font-family: 'Cormorant Garamond', serif;
    font-weight: 700;
    font-size: 3.4em;
    color: var(--accent);
    letter-spacing: -0.01em;
    line-height: 1.05;
    margin: 0 0 4px;
  }

  h2 {
    font-family: 'Cormorant Garamond', serif;
    font-weight: 400;
    font-size: 1.4em;
    color: var(--muted);
    margin: 0 0 16px;
  }

  h3 {
    font-family: 'Inter';
    font-weight: 500;
    font-size: 0.6em;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.2em;
    margin: 0 0 4px;
  }

  strong { color: var(--accent); font-weight: 500; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: radial-gradient(ellipse at 50% 80%, #4a1515 0%, var(--dark) 70%);
  }
  section.lead h1 { font-size: 3.8em; }
  section.lead p { color: var(--muted); font-size: 1em; }

  .divider {
    width: 60px; height: 2px;
    background: var(--accent);
    border-radius: 2px;
    margin: 14px auto 16px;
  }
footer: ''
---

<!-- _class: lead -->

<svg width="80" height="120" viewBox="0 0 80 120" fill="none">
  <ellipse cx="40" cy="16" rx="4" ry="4" fill="none" stroke="#c9a84c" stroke-width="1.5"/>
  <line x1="40" y1="20" x2="40" y2="55" stroke="#c9a84c" stroke-width="1.5"/>
  <path d="M20 55 Q20 30 40 28 Q60 30 60 55 Q60 75 40 80 Q20 75 20 55Z" fill="rgba(139,26,43,0.3)" stroke="#c9a84c" stroke-width="1.5"/>
  <path d="M24 55 Q24 38 40 36 Q56 38 56 55 Q56 70 40 74 Q24 70 24 55Z" fill="rgba(139,26,43,0.15)"/>
  <line x1="40" y1="80" x2="40" y2="100" stroke="#c9a84c" stroke-width="2"/>
  <ellipse cx="40" cy="104" rx="22" ry="4" fill="none" stroke="#c9a84c" stroke-width="1.5"/>
  <path d="M28 52 Q34 48 40 52 Q46 56 52 52" fill="none" stroke="rgba(201,168,76,0.3)" stroke-width="1"/>
</svg>

# Evening Tasting

<div class="divider"></div>

## Three Wines, Three Stories

<div style="display: flex; gap: 10px; justify-content: center; margin-top: 16px;">
  <span style="display: inline-block; background: var(--card); border: 1px solid var(--border); border-radius: 20px; padding: 5px 16px; font-size: 0.6em; color: var(--muted); letter-spacing: 0.06em; text-transform: uppercase;">April 2026</span>
  <span style="display: inline-block; background: var(--card); border: 1px solid var(--border); border-radius: 20px; padding: 5px 16px; font-size: 0.6em; color: var(--muted); letter-spacing: 0.06em; text-transform: uppercase;">Barossa Valley</span>
  <span style="display: inline-block; background: var(--card); border: 1px solid var(--border); border-radius: 20px; padding: 5px 16px; font-size: 0.6em; color: var(--muted); letter-spacing: 0.06em; text-transform: uppercase;">3 Varietals</span>
</div>

---

### The Wines

<div style="display: flex; gap: 20px; margin-top: 8px;">

  <div style="flex: 1; background: var(--card); border: 1px solid var(--border); border-radius: 14px; padding: 20px 16px; text-align: center;">
    <div style="width: 40px; height: 40px; border-radius: 50%; background: #8b1a2b; margin: 0 auto 8px; border: 2px solid #a02030;"></div>
    <div style="font-family: 'Cormorant Garamond'; font-size: 1.1em; font-weight: 600; color: var(--light);">Shiraz</div>
    <div style="font-size: 0.6em; color: var(--muted); margin-bottom: 10px;">2021 Barossa Old Vine</div>
    <svg width="110" height="110" viewBox="0 0 110 110" style="margin: 0 auto; display: block;">
      <circle cx="55" cy="55" r="48" fill="none" stroke="#4d1c1c" stroke-width="1"/>
      <circle cx="55" cy="55" r="38" fill="none" stroke="#4d1c1c" stroke-width="1"/>
      <circle cx="55" cy="55" r="28" fill="none" stroke="#4d1c1c" stroke-width="1"/>
      <text x="55" y="14" text-anchor="middle" fill="#c9a84c" font-family="Inter" font-size="6" font-weight="500">PEPPER</text>
      <text x="98" y="38" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">PLUM</text>
      <text x="100" y="75" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">SMOKE</text>
      <text x="55" y="100" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">LEATHER</text>
      <text x="10" y="75" text-anchor="middle" fill="#c9a84c" font-family="Inter" font-size="6">DARK CHERRY</text>
      <text x="10" y="38" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">VANILLA</text>
      <line x1="55" y1="18" x2="55" y2="30" stroke="#c9a84c" stroke-width="1.5"/>
      <line x1="90" y1="34" x2="80" y2="40" stroke="#a08070" stroke-width="1"/>
      <line x1="92" y1="72" x2="82" y2="66" stroke="#a08070" stroke-width="1"/>
      <line x1="55" y1="92" x2="55" y2="82" stroke="#a08070" stroke-width="1"/>
      <line x1="18" y1="72" x2="28" y2="66" stroke="#c9a84c" stroke-width="1.5"/>
      <line x1="18" y1="34" x2="28" y2="40" stroke="#a08070" stroke-width="1"/>
    </svg>
    <div style="display: flex; justify-content: center; gap: 3px; margin-top: 8px;">
      <div style="width: 8px; height: 8px; border-radius: 50%; background: var(--accent);"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: var(--accent);"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: var(--accent);"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: var(--accent);"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: #4d1c1c;"></div>
    </div>
    <div style="font-size: 0.55em; color: var(--muted); margin-top: 4px;">4 / 5</div>
  </div>

  <div style="flex: 1; background: var(--card); border: 1px solid var(--border); border-radius: 14px; padding: 20px 16px; text-align: center;">
    <div style="width: 40px; height: 40px; border-radius: 50%; background: #f0e6a0; margin: 0 auto 8px; border: 2px solid #d4ca80;"></div>
    <div style="font-family: 'Cormorant Garamond'; font-size: 1.1em; font-weight: 600; color: var(--light);">Chardonnay</div>
    <div style="font-size: 0.6em; color: var(--muted); margin-bottom: 10px;">2023 Adelaide Hills</div>
    <svg width="110" height="110" viewBox="0 0 110 110" style="margin: 0 auto; display: block;">
      <circle cx="55" cy="55" r="48" fill="none" stroke="#4d1c1c" stroke-width="1"/>
      <circle cx="55" cy="55" r="38" fill="none" stroke="#4d1c1c" stroke-width="1"/>
      <circle cx="55" cy="55" r="28" fill="none" stroke="#4d1c1c" stroke-width="1"/>
      <text x="55" y="14" text-anchor="middle" fill="#c9a84c" font-family="Inter" font-size="6" font-weight="500">CITRUS</text>
      <text x="98" y="38" text-anchor="middle" fill="#c9a84c" font-family="Inter" font-size="6">BUTTER</text>
      <text x="100" y="75" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">OAK</text>
      <text x="55" y="100" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">HONEY</text>
      <text x="10" y="75" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">PEACH</text>
      <text x="12" y="38" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">HAZELNUT</text>
      <line x1="55" y1="18" x2="55" y2="30" stroke="#c9a84c" stroke-width="1.5"/>
      <line x1="90" y1="34" x2="80" y2="40" stroke="#c9a84c" stroke-width="1.5"/>
      <line x1="92" y1="72" x2="82" y2="66" stroke="#a08070" stroke-width="1"/>
      <line x1="55" y1="92" x2="55" y2="82" stroke="#a08070" stroke-width="1"/>
      <line x1="18" y1="72" x2="28" y2="66" stroke="#a08070" stroke-width="1"/>
      <line x1="18" y1="34" x2="28" y2="40" stroke="#a08070" stroke-width="1"/>
    </svg>
    <div style="display: flex; justify-content: center; gap: 3px; margin-top: 8px;">
      <div style="width: 8px; height: 8px; border-radius: 50%; background: var(--accent);"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: var(--accent);"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: var(--accent);"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: #4d1c1c;"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: #4d1c1c;"></div>
    </div>
    <div style="font-size: 0.55em; color: var(--muted); margin-top: 4px;">3 / 5</div>
  </div>

  <div style="flex: 1; background: var(--card); border: 1px solid var(--border); border-radius: 14px; padding: 20px 16px; text-align: center;">
    <div style="width: 40px; height: 40px; border-radius: 50%; background: #e8a0b0; margin: 0 auto 8px; border: 2px solid #d08898;"></div>
    <div style="font-family: 'Cormorant Garamond'; font-size: 1.1em; font-weight: 600; color: var(--light);">Rose</div>
    <div style="font-size: 0.6em; color: var(--muted); margin-bottom: 10px;">2024 Grenache Dry</div>
    <svg width="110" height="110" viewBox="0 0 110 110" style="margin: 0 auto; display: block;">
      <circle cx="55" cy="55" r="48" fill="none" stroke="#4d1c1c" stroke-width="1"/>
      <circle cx="55" cy="55" r="38" fill="none" stroke="#4d1c1c" stroke-width="1"/>
      <circle cx="55" cy="55" r="28" fill="none" stroke="#4d1c1c" stroke-width="1"/>
      <text x="55" y="14" text-anchor="middle" fill="#c9a84c" font-family="Inter" font-size="6" font-weight="500">STRAWBERRY</text>
      <text x="98" y="38" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">MELON</text>
      <text x="100" y="75" text-anchor="middle" fill="#c9a84c" font-family="Inter" font-size="6">ROSE PETAL</text>
      <text x="55" y="100" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">MINERAL</text>
      <text x="10" y="75" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">GRAPEFRUIT</text>
      <text x="10" y="38" text-anchor="middle" fill="#a08070" font-family="Inter" font-size="6">HERBS</text>
      <line x1="55" y1="18" x2="55" y2="30" stroke="#c9a84c" stroke-width="1.5"/>
      <line x1="90" y1="34" x2="80" y2="40" stroke="#a08070" stroke-width="1"/>
      <line x1="92" y1="72" x2="82" y2="66" stroke="#c9a84c" stroke-width="1.5"/>
      <line x1="55" y1="92" x2="55" y2="82" stroke="#a08070" stroke-width="1"/>
      <line x1="18" y1="72" x2="28" y2="66" stroke="#a08070" stroke-width="1"/>
      <line x1="18" y1="34" x2="28" y2="40" stroke="#a08070" stroke-width="1"/>
    </svg>
    <div style="display: flex; justify-content: center; gap: 3px; margin-top: 8px;">
      <div style="width: 8px; height: 8px; border-radius: 50%; background: var(--accent);"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: var(--accent);"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: var(--accent);"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: var(--accent);"></div>
      <div style="width: 8px; height: 8px; border-radius: 50%; background: #4d1c1c;"></div>
    </div>
    <div style="font-size: 0.55em; color: var(--muted); margin-top: 4px;">4 / 5</div>
  </div>

</div>

---

### Food Pairings

<div style="display: flex; gap: 20px; margin-top: 12px;">

  <div style="flex: 1; text-align: center;">
    <div style="background: var(--card); border: 1px solid var(--border); border-radius: 14px; padding: 20px 16px;">
      <div style="width: 40px; height: 40px; border-radius: 50%; background: #8b1a2b; margin: 0 auto 6px; border: 2px solid #a02030;"></div>
      <div style="font-family: 'Cormorant Garamond'; font-size: 0.95em; font-weight: 600; color: var(--light); margin-bottom: 12px;">Shiraz</div>
      <div style="display: flex; flex-direction: column; gap: 10px;">
        <div style="display: flex; align-items: center; gap: 10px; text-align: left;">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.2" stroke-linecap="round"><path d="M15 11h.01M11 11h.01M7 11h.01M3 5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v6a8 8 0 0 1-8 8H9a6 6 0 0 1-6-6V5z"/><path d="M20 8h1a2 2 0 0 1 0 4h-1"/></svg>
          <div>
            <div style="font-size: 0.78em; color: var(--light); font-weight: 500;">Lamb Rack</div>
            <div style="font-size: 0.6em; color: var(--muted);">Herb-crusted, rosemary jus</div>
          </div>
        </div>
        <div style="display: flex; align-items: center; gap: 10px; text-align: left;">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.2" stroke-linecap="round"><circle cx="12" cy="12" r="10"/><path d="M8 12s1.5 2 4 2 4-2 4-2"/><path d="M9 8h.01M15 8h.01"/></svg>
          <div>
            <div style="font-size: 0.78em; color: var(--light); font-weight: 500;">Aged Cheddar</div>
            <div style="font-size: 0.6em; color: var(--muted);">24-month, sharp & crumbly</div>
          </div>
        </div>
        <div style="display: flex; align-items: center; gap: 10px; text-align: left;">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.2" stroke-linecap="round"><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.9 0 1.8-.1 2.6-.4"/><path d="M18 14a4 4 0 1 0 0 8 4 4 0 0 0 0-8z"/><path d="M15 3l2 6 6 2-6 2"/></svg>
          <div>
            <div style="font-size: 0.78em; color: var(--light); font-weight: 500;">Dark Chocolate</div>
            <div style="font-size: 0.6em; color: var(--muted);">72% cacao, sea salt</div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div style="flex: 1; text-align: center;">
    <div style="background: var(--card); border: 1px solid var(--border); border-radius: 14px; padding: 20px 16px;">
      <div style="width: 40px; height: 40px; border-radius: 50%; background: #f0e6a0; margin: 0 auto 6px; border: 2px solid #d4ca80;"></div>
      <div style="font-family: 'Cormorant Garamond'; font-size: 0.95em; font-weight: 600; color: var(--light); margin-bottom: 12px;">Chardonnay</div>
      <div style="display: flex; flex-direction: column; gap: 10px;">
        <div style="display: flex; align-items: center; gap: 10px; text-align: left;">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.2" stroke-linecap="round"><path d="M6.5 6.5C8 4 11 3 12 3s4 1 5.5 3.5C19 9 19 12 18 14c-1 2-3 4-6 7-3-3-5-5-6-7-1-2-1-5 .5-7.5z"/></svg>
          <div>
            <div style="font-size: 0.78em; color: var(--light); font-weight: 500;">Lobster Tail</div>
            <div style="font-size: 0.6em; color: var(--muted);">Butter-poached, lemon</div>
          </div>
        </div>
        <div style="display: flex; align-items: center; gap: 10px; text-align: left;">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.2" stroke-linecap="round"><rect x="3" y="8" width="18" height="12" rx="2"/><path d="M7 8V6a5 5 0 0 1 10 0v2"/></svg>
          <div>
            <div style="font-size: 0.78em; color: var(--light); font-weight: 500;">Triple Cream Brie</div>
            <div style="font-size: 0.6em; color: var(--muted);">Soft, buttery, mild</div>
          </div>
        </div>
        <div style="display: flex; align-items: center; gap: 10px; text-align: left;">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.2" stroke-linecap="round"><path d="M12 3v18M3 12h18M5.6 5.6l12.8 12.8M18.4 5.6L5.6 18.4"/></svg>
          <div>
            <div style="font-size: 0.78em; color: var(--light); font-weight: 500;">Roast Chicken</div>
            <div style="font-size: 0.6em; color: var(--muted);">Tarragon, cream sauce</div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div style="flex: 1; text-align: center;">
    <div style="background: var(--card); border: 1px solid var(--border); border-radius: 14px; padding: 20px 16px;">
      <div style="width: 40px; height: 40px; border-radius: 50%; background: #e8a0b0; margin: 0 auto 6px; border: 2px solid #d08898;"></div>
      <div style="font-family: 'Cormorant Garamond'; font-size: 0.95em; font-weight: 600; color: var(--light); margin-bottom: 12px;">Rose</div>
      <div style="display: flex; flex-direction: column; gap: 10px;">
        <div style="display: flex; align-items: center; gap: 10px; text-align: left;">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.2" stroke-linecap="round"><path d="M12 22s-8-4.5-8-11.8A8.1 8.1 0 0 1 12 2a8.1 8.1 0 0 1 8 8.2c0 7.3-8 11.8-8 11.8z"/><circle cx="12" cy="10" r="3"/></svg>
          <div>
            <div style="font-size: 0.78em; color: var(--light); font-weight: 500;">Grilled Prawns</div>
            <div style="font-size: 0.6em; color: var(--muted);">Garlic, chilli, lime</div>
          </div>
        </div>
        <div style="display: flex; align-items: center; gap: 10px; text-align: left;">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.2" stroke-linecap="round"><path d="M12 3C7 3 3 7 3 12l4.5 7.5h9L21 12c0-5-4-9-9-9z"/><path d="M12 3v9l4.5 7.5"/><path d="M12 12H3"/></svg>
          <div>
            <div style="font-size: 0.78em; color: var(--light); font-weight: 500;">Goat Cheese Salad</div>
            <div style="font-size: 0.6em; color: var(--muted);">Fig, walnut, arugula</div>
          </div>
        </div>
        <div style="display: flex; align-items: center; gap: 10px; text-align: left;">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.2" stroke-linecap="round"><path d="M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z"/><line x1="4" y1="22" x2="4" y2="15"/></svg>
          <div>
            <div style="font-size: 0.78em; color: var(--light); font-weight: 500;">Charcuterie Board</div>
            <div style="font-size: 0.6em; color: var(--muted);">Prosciutto, olives, nuts</div>
          </div>
        </div>
      </div>
    </div>
  </div>

</div>
