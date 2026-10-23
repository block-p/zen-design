---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Nunito:wght@400;600;700;800&family=Source+Sans+3:wght@300;400;500&display=swap');

  :root {
    --accent: #e67e22;
    --accent-hover: #f39c12;
    --dark: #4a3728;
    --card: #ffffff;
    --border: #e8e0d8;
    --muted: #8a7e72;
    --light: #faf8f5;
    --energy: #e74c3c;
    --groom: #9b59b6;
    --friendly: #e67e22;
    --train: #27ae60;
  }

  section {
    background: var(--light);
    color: var(--dark);
    font-family: 'Source Sans 3', sans-serif;
    font-weight: 400;
    padding: 48px 64px;
    line-height: 1.5;
  }

  section::after { font-family: 'Source Sans 3'; font-size: 0.55em; color: #e8e0d8; }

  h1 {
    font-family: 'Nunito', sans-serif;
    font-weight: 800;
    font-size: 2.8em;
    color: var(--dark);
    letter-spacing: -0.02em;
    line-height: 1.05;
    margin: 0 0 4px;
  }

  h2 {
    font-family: 'Nunito', sans-serif;
    font-weight: 600;
    font-size: 1.2em;
    color: var(--muted);
    margin: 0 0 16px;
  }

  h3 {
    font-family: 'Source Sans 3';
    font-weight: 500;
    font-size: 0.6em;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.18em;
    margin: 0 0 4px;
  }

  strong { color: var(--accent); font-weight: 600; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: linear-gradient(180deg, #faf8f5 0%, #f5efe8 100%);
  }
  section.lead h1 { font-size: 3.2em; }
footer: ''
---

<!-- _class: lead -->

<svg width="160" height="130" viewBox="0 0 160 130" fill="none">
  <!-- Dog silhouette - sitting golden retriever style -->
  <path d="M80 120 C60 120 45 115 40 100 C35 85 38 75 42 68 C46 61 45 55 42 48 C39 41 35 32 40 25 C45 18 55 15 60 18 C65 21 68 28 70 32 C72 36 75 35 78 32 C81 29 82 22 88 18 C94 14 102 16 108 22 C114 28 114 38 110 48 C106 58 108 65 112 72 C116 79 120 88 118 100 C116 112 102 120 80 120Z" fill="#4a3728" opacity="0.12"/>
  <path d="M80 120 C60 120 45 115 40 100 C35 85 38 75 42 68 C46 61 45 55 42 48 C39 41 35 32 40 25 C45 18 55 15 60 18 C65 21 68 28 70 32 C72 36 75 35 78 32 C81 29 82 22 88 18 C94 14 102 16 108 22 C114 28 114 38 110 48 C106 58 108 65 112 72 C116 79 120 88 118 100 C116 112 102 120 80 120Z" fill="none" stroke="#4a3728" stroke-width="2"/>
  <!-- Eyes -->
  <circle cx="62" cy="40" r="3" fill="#4a3728"/>
  <circle cx="96" cy="40" r="3" fill="#4a3728"/>
  <!-- Nose -->
  <ellipse cx="79" cy="52" rx="5" ry="3" fill="#4a3728"/>
  <!-- Tail -->
  <path d="M115 80 C125 65 130 50 125 40" fill="none" stroke="#4a3728" stroke-width="2.5" stroke-linecap="round"/>
  <!-- Collar -->
  <path d="M55 62 Q80 68 105 62" fill="none" stroke="#e67e22" stroke-width="3" stroke-linecap="round"/>
  <circle cx="80" cy="66" r="4" fill="#e67e22"/>
</svg>

# Find Your Perfect Match

## Answer three questions. Meet your new best friend.

<div style="display: flex; gap: 10px; justify-content: center; margin-top: 16px;">
  <div style="display: flex; align-items: center; gap: 6px; background: #fff; border: 1px solid #e8e0d8; border-radius: 20px; padding: 6px 14px;">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#e67e22" stroke-width="2"><path d="M12 21C12 21 4 15 4 8.5C4 5.42 6.42 3 9.5 3C11.24 3 12 4 12 4S12.76 3 14.5 3C17.58 3 20 5.42 20 8.5C20 15 12 21 12 21Z"/></svg>
    <span style="font-size: 0.7em; color: #8a7e72;">3 Breeds</span>
  </div>
  <div style="display: flex; align-items: center; gap: 6px; background: #fff; border: 1px solid #e8e0d8; border-radius: 20px; padding: 6px 14px;">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#e67e22" stroke-width="2"><path d="M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5"/><path d="M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5"/><circle cx="12" cy="12" r="3"/></svg>
    <span style="font-size: 0.7em; color: #8a7e72;">4 Traits</span>
  </div>
  <div style="display: flex; align-items: center; gap: 6px; background: #fff; border: 1px solid #e8e0d8; border-radius: 20px; padding: 6px 14px;">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#e67e22" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
    <span style="font-size: 0.7em; color: #8a7e72;">1 Match</span>
  </div>
</div>

---

### Breed Comparison

<div style="display: flex; gap: 24px; margin-top: 6px;">

  <div style="flex: 1;">
    <div style="text-align: center; margin-bottom: 12px;">
      <svg width="48" height="48" viewBox="0 0 48 48" fill="none">
        <circle cx="24" cy="24" r="22" fill="#fff" stroke="#e8e0d8" stroke-width="1.5"/>
        <path d="M18 18c0-2 2-4 6-4s6 2 6 4M15 28c0 4 4 7 9 7s9-3 9-7" fill="none" stroke="#4a3728" stroke-width="1.5" stroke-linecap="round"/>
        <circle cx="19" cy="22" r="2" fill="#4a3728"/><circle cx="29" cy="22" r="2" fill="#4a3728"/>
        <ellipse cx="24" cy="26" rx="3" ry="2" fill="#4a3728"/>
      </svg>
      <div style="font-family: 'Nunito'; font-weight: 700; font-size: 0.95em; color: #4a3728; margin-top: 4px;">Golden Retriever</div>
      <div style="font-size: 0.6em; color: #8a7e72;">Large  /  55-75 lbs</div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#e74c3c" stroke-width="2"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg> Energy</span>
        <span style="color: #8a7e72;">85%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #e74c3c, #ff6b6b); width: 85%; height: 100%; border-radius: 6px;"></div></div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#9b59b6" stroke-width="2"><path d="M12 22s-8-4.5-8-11.8A8.1 8.1 0 0 1 12 2a8.1 8.1 0 0 1 8 8.2c0 7.3-8 11.8-8 11.8z"/></svg> Grooming</span>
        <span style="color: #8a7e72;">75%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #9b59b6, #c39bd3); width: 75%; height: 100%; border-radius: 6px;"></div></div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#e67e22" stroke-width="2"><path d="M12 21C12 21 4 15 4 8.5C4 5.42 6.42 3 9.5 3C11.24 3 12 4 12 4S12.76 3 14.5 3C17.58 3 20 5.42 20 8.5C20 15 12 21 12 21Z"/></svg> Friendliness</span>
        <span style="color: #8a7e72;">95%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #e67e22, #f5b041); width: 95%; height: 100%; border-radius: 6px;"></div></div>
    </div>
    <div>
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/></svg> Trainability</span>
        <span style="color: #8a7e72;">90%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #27ae60, #52be80); width: 90%; height: 100%; border-radius: 6px;"></div></div>
    </div>
  </div>

  <div style="width: 1px; background: #e8e0d8;"></div>

  <div style="flex: 1;">
    <div style="text-align: center; margin-bottom: 12px;">
      <svg width="48" height="48" viewBox="0 0 48 48" fill="none">
        <circle cx="24" cy="24" r="22" fill="#fff" stroke="#e8e0d8" stroke-width="1.5"/>
        <path d="M14 16l4 6M34 16l-4 6" fill="none" stroke="#4a3728" stroke-width="1.5" stroke-linecap="round"/>
        <circle cx="19" cy="24" r="2" fill="#4a3728"/><circle cx="29" cy="24" r="2" fill="#4a3728"/>
        <path d="M20 30q4 3 8 0" fill="none" stroke="#4a3728" stroke-width="1.5" stroke-linecap="round"/>
        <ellipse cx="24" cy="28" rx="3" ry="2" fill="#4a3728"/>
      </svg>
      <div style="font-family: 'Nunito'; font-weight: 700; font-size: 0.95em; color: #4a3728; margin-top: 4px;">French Bulldog</div>
      <div style="font-size: 0.6em; color: #8a7e72;">Small  /  16-28 lbs</div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#e74c3c" stroke-width="2"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg> Energy</span>
        <span style="color: #8a7e72;">40%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #e74c3c, #ff6b6b); width: 40%; height: 100%; border-radius: 6px;"></div></div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#9b59b6" stroke-width="2"><path d="M12 22s-8-4.5-8-11.8A8.1 8.1 0 0 1 12 2a8.1 8.1 0 0 1 8 8.2c0 7.3-8 11.8-8 11.8z"/></svg> Grooming</span>
        <span style="color: #8a7e72;">25%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #9b59b6, #c39bd3); width: 25%; height: 100%; border-radius: 6px;"></div></div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#e67e22" stroke-width="2"><path d="M12 21C12 21 4 15 4 8.5C4 5.42 6.42 3 9.5 3C11.24 3 12 4 12 4S12.76 3 14.5 3C17.58 3 20 5.42 20 8.5C20 15 12 21 12 21Z"/></svg> Friendliness</span>
        <span style="color: #8a7e72;">80%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #e67e22, #f5b041); width: 80%; height: 100%; border-radius: 6px;"></div></div>
    </div>
    <div>
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/></svg> Trainability</span>
        <span style="color: #8a7e72;">50%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #27ae60, #52be80); width: 50%; height: 100%; border-radius: 6px;"></div></div>
    </div>
  </div>

  <div style="width: 1px; background: #e8e0d8;"></div>

  <div style="flex: 1;">
    <div style="text-align: center; margin-bottom: 12px;">
      <svg width="48" height="48" viewBox="0 0 48 48" fill="none">
        <circle cx="24" cy="24" r="22" fill="#fff" stroke="#e8e0d8" stroke-width="1.5"/>
        <path d="M16 14l2 8M32 14l-2 8" fill="none" stroke="#4a3728" stroke-width="1.5" stroke-linecap="round"/>
        <circle cx="19" cy="24" r="2" fill="#4a3728"/><circle cx="29" cy="24" r="2" fill="#4a3728"/>
        <ellipse cx="24" cy="28" rx="2.5" ry="1.8" fill="#4a3728"/>
        <path d="M18 32q6 4 12 0" fill="none" stroke="#4a3728" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
      <div style="font-family: 'Nunito'; font-weight: 700; font-size: 0.95em; color: #4a3728; margin-top: 4px;">Border Collie</div>
      <div style="font-size: 0.6em; color: #8a7e72;">Medium  /  30-55 lbs</div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#e74c3c" stroke-width="2"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg> Energy</span>
        <span style="color: #8a7e72;">98%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #e74c3c, #ff6b6b); width: 98%; height: 100%; border-radius: 6px;"></div></div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#9b59b6" stroke-width="2"><path d="M12 22s-8-4.5-8-11.8A8.1 8.1 0 0 1 12 2a8.1 8.1 0 0 1 8 8.2c0 7.3-8 11.8-8 11.8z"/></svg> Grooming</span>
        <span style="color: #8a7e72;">60%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #9b59b6, #c39bd3); width: 60%; height: 100%; border-radius: 6px;"></div></div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#e67e22" stroke-width="2"><path d="M12 21C12 21 4 15 4 8.5C4 5.42 6.42 3 9.5 3C11.24 3 12 4 12 4S12.76 3 14.5 3C17.58 3 20 5.42 20 8.5C20 15 12 21 12 21Z"/></svg> Friendliness</span>
        <span style="color: #8a7e72;">70%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #e67e22, #f5b041); width: 70%; height: 100%; border-radius: 6px;"></div></div>
    </div>
    <div>
      <div style="display: flex; justify-content: space-between; font-size: 0.65em; margin-bottom: 3px;">
        <span style="display: flex; align-items: center; gap: 4px;"><svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/></svg> Trainability</span>
        <span style="color: #8a7e72;">97%</span>
      </div>
      <div style="background: #f0ebe5; border-radius: 6px; height: 10px; overflow: hidden;"><div style="background: linear-gradient(90deg, #27ae60, #52be80); width: 97%; height: 100%; border-radius: 6px;"></div></div>
    </div>
  </div>

</div>

---

### Your Match

<div style="display: flex; gap: 28px; margin-top: 8px; align-items: flex-start;">

  <div style="text-align: center; width: 200px;">
    <svg width="150" height="150" viewBox="0 0 150 150">
      <circle cx="75" cy="75" r="65" fill="none" stroke="#f0ebe5" stroke-width="10"/>
      <circle cx="75" cy="75" r="65" fill="none" stroke="#e67e22" stroke-width="10"
              stroke-dasharray="408" stroke-dashoffset="37"
              stroke-linecap="round" transform="rotate(-90 75 75)"/>
      <text x="75" y="68" text-anchor="middle" fill="#4a3728" font-family="Nunito"
            font-size="36" font-weight="800">91</text>
      <text x="75" y="86" text-anchor="middle" fill="#8a7e72" font-family="Source Sans 3"
            font-size="11" letter-spacing="1.5">% MATCH</text>
    </svg>
    <div style="font-family: 'Nunito'; font-weight: 800; font-size: 1.4em; color: #4a3728; margin-top: 8px;">Golden Retriever</div>
    <div style="font-size: 0.72em; color: #8a7e72;">The family favourite. Loyal, gentle, endlessly patient.</div>
  </div>

  <div style="flex: 1;">
    <div style="font-family: 'Nunito'; font-weight: 700; font-size: 0.85em; color: #4a3728; margin-bottom: 10px;">Why This Breed</div>
    <div style="display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 16px;">
      <div style="background: rgba(230,126,34,0.1); border: 1px solid rgba(230,126,34,0.25); border-radius: 10px; padding: 8px 14px; display: flex; align-items: center; gap: 6px;">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#e67e22" stroke-width="2"><path d="M12 21C12 21 4 15 4 8.5C4 5.42 6.42 3 9.5 3C11.24 3 12 4 12 4S12.76 3 14.5 3C17.58 3 20 5.42 20 8.5C20 15 12 21 12 21Z"/></svg>
        <span style="font-size: 0.72em; color: #e67e22; font-weight: 500;">Kid-Friendly</span>
      </div>
      <div style="background: rgba(39,174,96,0.1); border: 1px solid rgba(39,174,96,0.25); border-radius: 10px; padding: 8px 14px; display: flex; align-items: center; gap: 6px;">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/></svg>
        <span style="font-size: 0.72em; color: #27ae60; font-weight: 500;">Easy to Train</span>
      </div>
      <div style="background: rgba(231,76,60,0.1); border: 1px solid rgba(231,76,60,0.25); border-radius: 10px; padding: 8px 14px; display: flex; align-items: center; gap: 6px;">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#e74c3c" stroke-width="2"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
        <span style="font-size: 0.72em; color: #e74c3c; font-weight: 500;">Active Lifestyle</span>
      </div>
      <div style="background: rgba(155,89,182,0.1); border: 1px solid rgba(155,89,182,0.25); border-radius: 10px; padding: 8px 14px; display: flex; align-items: center; gap: 6px;">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9b59b6" stroke-width="2"><path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"/><path d="M12 6v6l4 2"/></svg>
        <span style="font-size: 0.72em; color: #9b59b6; font-weight: 500;">Long Lifespan</span>
      </div>
    </div>

    <div style="font-family: 'Nunito'; font-weight: 700; font-size: 0.85em; color: #4a3728; margin-bottom: 8px;">Care Checklist</div>
    <div style="display: flex; flex-direction: column; gap: 6px;">
      <div style="display: flex; align-items: center; gap: 8px;">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
        <span style="font-size: 0.75em; color: #4a3728;">Daily walk (60 min minimum)</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
        <span style="font-size: 0.75em; color: #4a3728;">Brush coat 2-3 times per week</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#27ae60" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
        <span style="font-size: 0.75em; color: #4a3728;">Vet check every 6 months (hip/elbow screening)</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#e67e22" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
        <span style="font-size: 0.75em; color: #4a3728;">Watch for weight gain - loves treats</span>
      </div>
    </div>
  </div>

</div>
