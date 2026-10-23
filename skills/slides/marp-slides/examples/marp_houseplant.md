---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Quicksand:wght@400;500;600;700&family=Lato:wght@300;400;700&display=swap');

  :root {
    --accent: #4CAF50;
    --accent-hover: #66BB6A;
    --dark: #2d5a2d;
    --card: #ffffff;
    --border: #d4e8d4;
    --muted: #7a9a7a;
    --light: #f0f7f0;
    --sun-full: #f9a825;
    --sun-empty: #e0e0d8;
    --water: #42a5f5;
    --spring: #66BB6A;
    --summer: #FDD835;
    --fall: #FF8F00;
    --winter: #90CAF9;
  }

  section {
    background: var(--light);
    color: var(--dark);
    font-family: 'Lato', sans-serif;
    font-weight: 400;
    padding: 48px 64px;
    line-height: 1.5;
  }

  section::after { font-family: 'Lato'; font-size: 0.55em; color: #d4e8d4; }

  h1 {
    font-family: 'Quicksand', sans-serif;
    font-weight: 700;
    font-size: 2.8em;
    color: var(--dark);
    letter-spacing: -0.01em;
    line-height: 1.05;
    margin: 0 0 4px;
  }

  h2 {
    font-family: 'Quicksand', sans-serif;
    font-weight: 500;
    font-size: 1.2em;
    color: var(--muted);
    margin: 0 0 16px;
  }

  h3 {
    font-family: 'Lato';
    font-weight: 700;
    font-size: 0.6em;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.18em;
    margin: 0 0 4px;
  }

  strong { color: var(--accent); font-weight: 700; }

  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    background: linear-gradient(180deg, #f0f7f0 0%, #e0f0e0 50%, #d0e8d0 100%);
  }
  section.lead h1 { font-size: 3em; }
footer: ''
---

<!-- _class: lead -->

<svg width="140" height="160" viewBox="0 0 140 160" fill="none">
  <!-- Pot -->
  <path d="M40 110 L48 150 L92 150 L100 110Z" fill="#c0845a" stroke="#a0704a" stroke-width="1.5"/>
  <rect x="35" y="105" width="70" height="10" rx="3" fill="#d49668" stroke="#a0704a" stroke-width="1.5"/>
  <!-- Soil -->
  <ellipse cx="70" cy="110" rx="30" ry="4" fill="#6d4c2a"/>
  <!-- Main stem -->
  <path d="M70 110 C70 90 68 70 70 50" stroke="#4CAF50" stroke-width="3" fill="none" stroke-linecap="round"/>
  <!-- Left large leaf -->
  <path d="M70 70 C55 55 35 50 30 58 C25 66 40 72 55 70 C62 69 68 70 70 70" fill="#66BB6A" stroke="#4CAF50" stroke-width="1"/>
  <path d="M70 70 C55 64 38 60 30 58" fill="none" stroke="#4CAF50" stroke-width="0.8" opacity="0.5"/>
  <!-- Right large leaf -->
  <path d="M70 60 C85 45 105 42 110 50 C115 58 100 62 85 60 C78 59 72 60 70 60" fill="#81C784" stroke="#4CAF50" stroke-width="1"/>
  <path d="M70 60 C85 54 102 50 110 50" fill="none" stroke="#4CAF50" stroke-width="0.8" opacity="0.5"/>
  <!-- Top leaf -->
  <path d="M70 50 C62 35 55 20 60 14 C65 8 75 8 80 14 C85 20 78 35 70 50" fill="#A5D6A7" stroke="#4CAF50" stroke-width="1"/>
  <path d="M70 50 C70 35 68 22 70 14" fill="none" stroke="#4CAF50" stroke-width="0.8" opacity="0.5"/>
  <!-- Small left leaf -->
  <path d="M68 85 C58 78 45 76 42 80 C39 84 48 88 58 86 C63 85 67 85 68 85" fill="#81C784" stroke="#4CAF50" stroke-width="0.8"/>
  <!-- Small right leaf -->
  <path d="M72 78 C80 72 90 71 92 75 C94 79 87 82 80 80 C76 79 73 78 72 78" fill="#A5D6A7" stroke="#4CAF50" stroke-width="0.8"/>
</svg>

# Indoor Plant Care

## Your quick-reference guide to happy houseplants

<div style="display: flex; gap: 10px; justify-content: center; margin-top: 14px;">
  <div style="display: flex; align-items: center; gap: 5px; background: #fff; border: 1px solid #d4e8d4; border-radius: 20px; padding: 5px 14px;">
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#4CAF50" stroke-width="2"><circle cx="12" cy="12" r="4"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>
    <span style="font-size: 0.65em; color: #7a9a7a;">Light Guide</span>
  </div>
  <div style="display: flex; align-items: center; gap: 5px; background: #fff; border: 1px solid #d4e8d4; border-radius: 20px; padding: 5px 14px;">
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#42a5f5" stroke-width="2"><path d="M12 2C6.5 9 4 13 4 16a8 8 0 0 0 16 0c0-3-2.5-7-8-14z"/></svg>
    <span style="font-size: 0.65em; color: #7a9a7a;">Watering</span>
  </div>
  <div style="display: flex; align-items: center; gap: 5px; background: #fff; border: 1px solid #d4e8d4; border-radius: 20px; padding: 5px 14px;">
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#FF8F00" stroke-width="2"><rect x="3" y="4" width="18" height="18" rx="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg>
    <span style="font-size: 0.65em; color: #7a9a7a;">Seasonal Care</span>
  </div>
</div>

---

### Plant Profiles

<div style="display: flex; gap: 18px; margin-top: 6px;">

  <div style="flex: 1; background: #fff; border: 1px solid #d4e8d4; border-radius: 14px; padding: 16px 14px;">
    <div style="text-align: center; margin-bottom: 8px;">
      <svg width="40" height="50" viewBox="0 0 40 50" fill="none">
        <path d="M20 48 L20 25" stroke="#4CAF50" stroke-width="2"/>
        <path d="M20 30 C10 20 2 12 8 6 C14 0 22 10 20 20" fill="#81C784" stroke="#4CAF50" stroke-width="0.8"/>
        <path d="M20 25 C30 15 38 8 32 2 C26 -4 18 8 20 18" fill="#A5D6A7" stroke="#4CAF50" stroke-width="0.8"/>
        <path d="M20 35 C12 30 5 28 5 33 C5 38 14 38 20 35" fill="#66BB6A" stroke="#4CAF50" stroke-width="0.8"/>
      </svg>
      <div style="font-family: 'Quicksand'; font-weight: 700; font-size: 0.9em; color: #2d5a2d;">Monstera</div>
      <div style="font-size: 0.55em; color: #7a9a7a;">Swiss Cheese Plant</div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="font-size: 0.6em; color: #7a9a7a; margin-bottom: 3px; font-weight: 700;">LIGHT</div>
      <div style="display: flex; gap: 4px;">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="#f9a825" stroke="#f9a825" stroke-width="1"><circle cx="12" cy="12" r="4"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="#f9a825" stroke="#f9a825" stroke-width="1"><circle cx="12" cy="12" r="4"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#e0e0d8" stroke-width="1"><circle cx="12" cy="12" r="4"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>
      </div>
      <div style="font-size: 0.55em; color: #7a9a7a; margin-top: 2px;">Bright indirect</div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="font-size: 0.6em; color: #7a9a7a; margin-bottom: 3px; font-weight: 700;">WATER</div>
      <div style="background: #e8f5e9; border-radius: 6px; height: 8px; overflow: hidden;">
        <div style="background: linear-gradient(90deg, #42a5f5, #90CAF9); width: 55%; height: 100%; border-radius: 6px;"></div>
      </div>
      <div style="font-size: 0.55em; color: #7a9a7a; margin-top: 2px;">Every 7-10 days</div>
    </div>
    <div>
      <div style="font-size: 0.6em; color: #7a9a7a; margin-bottom: 3px; font-weight: 700;">DIFFICULTY</div>
      <div style="display: flex; gap: 3px;">
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #4CAF50;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #4CAF50;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #d4e8d4;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #d4e8d4;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #d4e8d4;"></div>
      </div>
      <div style="font-size: 0.55em; color: #7a9a7a; margin-top: 2px;">Easy</div>
    </div>
    <div style="display: flex; gap: 4px; margin-top: 8px; justify-content: center;">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#4CAF50" stroke-width="1.5" title="Pet warning"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
      <span style="font-size: 0.5em; color: #7a9a7a;">Toxic to pets</span>
    </div>
  </div>

  <div style="flex: 1; background: #fff; border: 1px solid #d4e8d4; border-radius: 14px; padding: 16px 14px;">
    <div style="text-align: center; margin-bottom: 8px;">
      <svg width="40" height="50" viewBox="0 0 40 50" fill="none">
        <path d="M20 48 L20 20" stroke="#4CAF50" stroke-width="2"/>
        <path d="M20 20 C20 10 10 2 8 8 C6 14 16 18 20 20" fill="#66BB6A" stroke="#4CAF50" stroke-width="0.8"/>
        <path d="M20 20 C20 10 30 2 32 8 C34 14 24 18 20 20" fill="#81C784" stroke="#4CAF50" stroke-width="0.8"/>
        <path d="M20 28 C14 22 6 20 6 26 C6 32 16 30 20 28" fill="#A5D6A7" stroke="#4CAF50" stroke-width="0.8"/>
        <path d="M20 28 C26 22 34 20 34 26 C34 32 24 30 20 28" fill="#66BB6A" stroke="#4CAF50" stroke-width="0.8"/>
        <path d="M20 36 C14 32 8 31 8 35 C8 39 16 38 20 36" fill="#81C784" stroke="#4CAF50" stroke-width="0.8"/>
      </svg>
      <div style="font-family: 'Quicksand'; font-weight: 700; font-size: 0.9em; color: #2d5a2d;">Pothos</div>
      <div style="font-size: 0.55em; color: #7a9a7a;">Devil's Ivy</div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="font-size: 0.6em; color: #7a9a7a; margin-bottom: 3px; font-weight: 700;">LIGHT</div>
      <div style="display: flex; gap: 4px;">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="#f9a825" stroke="#f9a825" stroke-width="1"><circle cx="12" cy="12" r="4"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#e0e0d8" stroke-width="1"><circle cx="12" cy="12" r="4"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#e0e0d8" stroke-width="1"><circle cx="12" cy="12" r="4"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>
      </div>
      <div style="font-size: 0.55em; color: #7a9a7a; margin-top: 2px;">Low to medium</div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="font-size: 0.6em; color: #7a9a7a; margin-bottom: 3px; font-weight: 700;">WATER</div>
      <div style="background: #e8f5e9; border-radius: 6px; height: 8px; overflow: hidden;">
        <div style="background: linear-gradient(90deg, #42a5f5, #90CAF9); width: 40%; height: 100%; border-radius: 6px;"></div>
      </div>
      <div style="font-size: 0.55em; color: #7a9a7a; margin-top: 2px;">Every 10-14 days</div>
    </div>
    <div>
      <div style="font-size: 0.6em; color: #7a9a7a; margin-bottom: 3px; font-weight: 700;">DIFFICULTY</div>
      <div style="display: flex; gap: 3px;">
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #4CAF50;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #d4e8d4;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #d4e8d4;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #d4e8d4;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #d4e8d4;"></div>
      </div>
      <div style="font-size: 0.55em; color: #7a9a7a; margin-top: 2px;">Beginner</div>
    </div>
    <div style="display: flex; gap: 4px; margin-top: 8px; justify-content: center;">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#42a5f5" stroke-width="1.5"><path d="M12 2C6.5 9 4 13 4 16a8 8 0 0 0 16 0c0-3-2.5-7-8-14z"/></svg>
      <span style="font-size: 0.5em; color: #7a9a7a;">Drought tolerant</span>
    </div>
  </div>

  <div style="flex: 1; background: #fff; border: 1px solid #d4e8d4; border-radius: 14px; padding: 16px 14px;">
    <div style="text-align: center; margin-bottom: 8px;">
      <svg width="40" height="50" viewBox="0 0 40 50" fill="none">
        <path d="M20 48 L20 30" stroke="#4CAF50" stroke-width="2"/>
        <path d="M20 30 C12 25 4 15 10 8 C16 1 20 12 20 20" fill="#66BB6A" stroke="#4CAF50" stroke-width="0.8"/>
        <path d="M20 30 C28 25 36 15 30 8 C24 1 20 12 20 20" fill="#A5D6A7" stroke="#4CAF50" stroke-width="0.8"/>
        <path d="M20 34 C16 28 10 22 8 26 C6 30 14 34 20 34" fill="#81C784" stroke="#4CAF50" stroke-width="0.8"/>
        <path d="M20 34 C24 28 30 22 32 26 C34 30 26 34 20 34" fill="#66BB6A" stroke="#4CAF50" stroke-width="0.8"/>
        <path d="M20 40 C14 38 8 36 10 40 C12 44 18 42 20 40" fill="#A5D6A7" stroke="#4CAF50" stroke-width="0.8"/>
      </svg>
      <div style="font-family: 'Quicksand'; font-weight: 700; font-size: 0.9em; color: #2d5a2d;">Fiddle Leaf Fig</div>
      <div style="font-size: 0.55em; color: #7a9a7a;">Ficus lyrata</div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="font-size: 0.6em; color: #7a9a7a; margin-bottom: 3px; font-weight: 700;">LIGHT</div>
      <div style="display: flex; gap: 4px;">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="#f9a825" stroke="#f9a825" stroke-width="1"><circle cx="12" cy="12" r="4"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="#f9a825" stroke="#f9a825" stroke-width="1"><circle cx="12" cy="12" r="4"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="#f9a825" stroke="#f9a825" stroke-width="1"><circle cx="12" cy="12" r="4"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>
      </div>
      <div style="font-size: 0.55em; color: #7a9a7a; margin-top: 2px;">Bright direct</div>
    </div>
    <div style="margin-bottom: 8px;">
      <div style="font-size: 0.6em; color: #7a9a7a; margin-bottom: 3px; font-weight: 700;">WATER</div>
      <div style="background: #e8f5e9; border-radius: 6px; height: 8px; overflow: hidden;">
        <div style="background: linear-gradient(90deg, #42a5f5, #90CAF9); width: 70%; height: 100%; border-radius: 6px;"></div>
      </div>
      <div style="font-size: 0.55em; color: #7a9a7a; margin-top: 2px;">Every 5-7 days</div>
    </div>
    <div>
      <div style="font-size: 0.6em; color: #7a9a7a; margin-bottom: 3px; font-weight: 700;">DIFFICULTY</div>
      <div style="display: flex; gap: 3px;">
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #4CAF50;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #4CAF50;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #4CAF50;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #4CAF50;"></div>
        <div style="width: 8px; height: 8px; border-radius: 50%; background: #d4e8d4;"></div>
      </div>
      <div style="font-size: 0.55em; color: #7a9a7a; margin-top: 2px;">Advanced</div>
    </div>
    <div style="display: flex; gap: 4px; margin-top: 8px; justify-content: center;">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#FF8F00" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
      <span style="font-size: 0.5em; color: #7a9a7a;">Sensitive to drafts</span>
    </div>
  </div>

</div>

---

### Seasonal Calendar

<div style="display: flex; gap: 12px; margin-top: 8px;">

  <div style="flex: 1; background: #fff; border-top: 4px solid #66BB6A; border-radius: 0 0 12px 12px; padding: 14px 12px; border-left: 1px solid #d4e8d4; border-right: 1px solid #d4e8d4; border-bottom: 1px solid #d4e8d4;">
    <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 10px;">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#66BB6A" stroke-width="1.5"><path d="M12 22c4 0 8-2 8-6 0-3-2-5-4-6 1-2 1-4 0-6-2 2-4 3-4 3s-2-1-4-3c-1 2-1 4 0 6-2 1-4 3-4 6 0 4 4 6 8 6z"/></svg>
      <div style="font-family: 'Quicksand'; font-weight: 700; font-size: 0.85em; color: #66BB6A;">Spring</div>
    </div>
    <div style="font-size: 0.62em; color: #2d5a2d; line-height: 1.8;">
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#42a5f5" stroke-width="2"><path d="M12 2C6.5 9 4 13 4 16a8 8 0 0 0 16 0c0-3-2.5-7-8-14z"/></svg>
        Increase watering
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#4CAF50" stroke-width="2"><path d="M12 22c4 0 8-2 8-6 0-3-2-5-4-6 1-2 1-4 0-6-2 2-4 3-4 3s-2-1-4-3c-1 2-1 4 0 6-2 1-4 3-4 6 0 4 4 6 8 6z"/></svg>
        Start fertilizing
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#FF8F00" stroke-width="2"><rect x="3" y="8" width="18" height="12" rx="2"/><path d="M7 8V6a5 5 0 0 1 10 0v2"/></svg>
        Repot if root-bound
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#9b59b6" stroke-width="2"><path d="M6 3v12"/><path d="M18 9a3 3 0 0 1 0 6h-6"/><path d="M6 8h8a3 3 0 0 1 0 6"/></svg>
        Prune dead leaves
      </div>
    </div>
    <div style="margin-top: 8px;">
      <div style="font-size: 0.5em; color: #7a9a7a; margin-bottom: 2px;">WATERING</div>
      <div style="display: flex; gap: 2px;">
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #42a5f5;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #42a5f5;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #42a5f5;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #e0e0d8;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #e0e0d8;"></div>
      </div>
    </div>
  </div>

  <div style="flex: 1; background: #fff; border-top: 4px solid #FDD835; border-radius: 0 0 12px 12px; padding: 14px 12px; border-left: 1px solid #d4e8d4; border-right: 1px solid #d4e8d4; border-bottom: 1px solid #d4e8d4;">
    <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 10px;">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#FDD835" stroke-width="1.5"><circle cx="12" cy="12" r="4"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>
      <div style="font-family: 'Quicksand'; font-weight: 700; font-size: 0.85em; color: #F9A825;">Summer</div>
    </div>
    <div style="font-size: 0.62em; color: #2d5a2d; line-height: 1.8;">
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#42a5f5" stroke-width="2"><path d="M12 2C6.5 9 4 13 4 16a8 8 0 0 0 16 0c0-3-2.5-7-8-14z"/></svg>
        Water frequently
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#4CAF50" stroke-width="2"><path d="M12 22c4 0 8-2 8-6 0-3-2-5-4-6 1-2 1-4 0-6-2 2-4 3-4 3s-2-1-4-3c-1 2-1 4 0 6-2 1-4 3-4 6 0 4 4 6 8 6z"/></svg>
        Monthly fertilizer
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#FF8F00" stroke-width="2"><path d="M12 22s-8-4.5-8-11.8A8.1 8.1 0 0 1 12 2a8.1 8.1 0 0 1 8 8.2c0 7.3-8 11.8-8 11.8z"/></svg>
        Shield from direct heat
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#9b59b6" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
        Check for pests
      </div>
    </div>
    <div style="margin-top: 8px;">
      <div style="font-size: 0.5em; color: #7a9a7a; margin-bottom: 2px;">WATERING</div>
      <div style="display: flex; gap: 2px;">
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #42a5f5;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #42a5f5;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #42a5f5;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #42a5f5;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #e0e0d8;"></div>
      </div>
    </div>
  </div>

  <div style="flex: 1; background: #fff; border-top: 4px solid #FF8F00; border-radius: 0 0 12px 12px; padding: 14px 12px; border-left: 1px solid #d4e8d4; border-right: 1px solid #d4e8d4; border-bottom: 1px solid #d4e8d4;">
    <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 10px;">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#FF8F00" stroke-width="1.5"><path d="M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5"/><path d="M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5"/><path d="M8 2l3 5-3 5"/><path d="M16 2l-3 5 3 5"/></svg>
      <div style="font-family: 'Quicksand'; font-weight: 700; font-size: 0.85em; color: #FF8F00;">Fall</div>
    </div>
    <div style="font-size: 0.62em; color: #2d5a2d; line-height: 1.8;">
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#42a5f5" stroke-width="2"><path d="M12 2C6.5 9 4 13 4 16a8 8 0 0 0 16 0c0-3-2.5-7-8-14z"/></svg>
        Reduce watering
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#4CAF50" stroke-width="2"><path d="M18 6L6 18M6 6l12 12"/></svg>
        Stop fertilizing
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#FF8F00" stroke-width="2"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/></svg>
        Move away from cold windows
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#9b59b6" stroke-width="2"><path d="M6 3v12"/><path d="M18 9a3 3 0 0 1 0 6h-6"/><path d="M6 8h8a3 3 0 0 1 0 6"/></svg>
        Last chance to propagate
      </div>
    </div>
    <div style="margin-top: 8px;">
      <div style="font-size: 0.5em; color: #7a9a7a; margin-bottom: 2px;">WATERING</div>
      <div style="display: flex; gap: 2px;">
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #42a5f5;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #42a5f5;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #e0e0d8;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #e0e0d8;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #e0e0d8;"></div>
      </div>
    </div>
  </div>

  <div style="flex: 1; background: #fff; border-top: 4px solid #90CAF9; border-radius: 0 0 12px 12px; padding: 14px 12px; border-left: 1px solid #d4e8d4; border-right: 1px solid #d4e8d4; border-bottom: 1px solid #d4e8d4;">
    <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 10px;">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#90CAF9" stroke-width="1.5"><path d="M2 12h4M18 12h4M12 2v4M12 18v4M6.34 6.34l1.42 1.42M16.24 16.24l1.42 1.42M6.34 17.66l1.42-1.42M16.24 7.76l1.42-1.42"/><circle cx="12" cy="12" r="3" fill="none"/></svg>
      <div style="font-family: 'Quicksand'; font-weight: 700; font-size: 0.85em; color: #64B5F6;">Winter</div>
    </div>
    <div style="font-size: 0.62em; color: #2d5a2d; line-height: 1.8;">
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#42a5f5" stroke-width="2"><path d="M12 2C6.5 9 4 13 4 16a8 8 0 0 0 16 0c0-3-2.5-7-8-14z"/></svg>
        Minimal watering
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#4CAF50" stroke-width="2"><path d="M18 6L6 18M6 6l12 12"/></svg>
        No fertilizer
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#FF8F00" stroke-width="2"><path d="M12 22s-8-4.5-8-11.8A8.1 8.1 0 0 1 12 2a8.1 8.1 0 0 1 8 8.2c0 7.3-8 11.8-8 11.8z"/></svg>
        Keep away from heaters
      </div>
      <div style="display: flex; align-items: center; gap: 5px;">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#9b59b6" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
        Monitor humidity
      </div>
    </div>
    <div style="margin-top: 8px;">
      <div style="font-size: 0.5em; color: #7a9a7a; margin-bottom: 2px;">WATERING</div>
      <div style="display: flex; gap: 2px;">
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #42a5f5;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #e0e0d8;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #e0e0d8;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #e0e0d8;"></div>
        <div style="flex: 1; height: 5px; border-radius: 2px; background: #e0e0d8;"></div>
      </div>
    </div>
  </div>

</div>
