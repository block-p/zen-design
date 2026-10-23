---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Noto+Sans+JP:wght@300;400;500;600;700;900&family=Outfit:wght@300;400;500;600;700;800&family=JetBrains+Mono:wght@300;400;500;600&display=swap');

  :root {
    --accent: #e94560;
    --accent-hover: #f06070;
    --secondary: #0f3460;
    --dark: #1a1a2e;
    --card: #16213e;
    --border: #1f3050;
    --muted: #7a8ba8;
    --light: #eee;
  }

  section {
    background: var(--dark);
    color: var(--light);
    font-family: 'Outfit', sans-serif;
    font-weight: 400;
    padding: 50px 70px;
    line-height: 1.6;
  }

  h1 {
    font-family: 'Outfit', sans-serif;
    font-weight: 800;
    font-size: 3.4em;
    color: var(--accent);
    letter-spacing: -0.02em;
    line-height: 1.1;
    margin-bottom: 0.1em;
  }

  h2 {
    font-family: 'Outfit', sans-serif;
    font-weight: 700;
    font-size: 1.8em;
    color: var(--light);
    margin-bottom: 0.3em;
  }

  h3 {
    font-family: 'Outfit', sans-serif;
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
    background: radial-gradient(ellipse at 50% 80%, #1a1040 0%, var(--dark) 70%);
  }
  section.lead h1 { font-size: 4em; }

  .divider {
    width: 60px; height: 3px;
    background: var(--accent);
    border-radius: 2px;
    margin: 14px auto 18px;
  }

  .level-badge {
    display: inline-flex; align-items: center; gap: 8px;
    background: var(--card);
    border: 1px solid var(--accent);
    border-radius: 20px;
    padding: 6px 18px;
    font-size: 0.7em;
    color: var(--accent);
    letter-spacing: 0.08em;
    font-weight: 600;
  }

  .phrase-grid {
    display: grid; grid-template-columns: repeat(3, 1fr); gap: 14px; margin-top: 14px;
  }
  .phrase-card {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 14px;
    padding: 16px 14px;
    position: relative;
    overflow: hidden;
  }
  .phrase-card::before {
    content: '';
    position: absolute; top: 0; left: 0; width: 4px; height: 100%;
    background: var(--accent);
    border-radius: 4px 0 0 4px;
  }
  .phrase-card .jp {
    font-family: 'Noto Sans JP', sans-serif;
    font-weight: 700; font-size: 1.2em; color: var(--light);
    margin-bottom: 2px;
  }
  .phrase-card .roma {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.65em; color: var(--accent); font-weight: 400;
    margin-bottom: 4px; letter-spacing: 0.02em;
  }
  .phrase-card .en {
    font-size: 0.72em; color: var(--muted); margin-bottom: 6px;
  }
  .phrase-card .context-tag {
    display: inline-block;
    font-size: 0.55em; padding: 2px 8px; border-radius: 10px;
    letter-spacing: 0.06em; text-transform: uppercase; font-weight: 600;
  }
  .tag-formal { background: rgba(15,52,96,0.6); color: #5b9bd5; border: 1px solid #2a4a6e; }
  .tag-casual { background: rgba(233,69,96,0.15); color: #e94560; border: 1px solid rgba(233,69,96,0.3); }

  .phrase-card .speaker-icon {
    position: absolute; top: 12px; right: 12px; opacity: 0.4;
  }

  .conj-section { display: flex; gap: 20px; margin-top: 10px; }
  .conj-table-wrap { flex: 1.5; }
  .difficulty-ladder { flex: 0.5; }

  table {
    width: 100%;
    border-collapse: separate;
    border-spacing: 0 4px;
    font-size: 0.72em;
  }
  table th {
    background: var(--secondary);
    color: var(--light);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    font-size: 0.8em;
    padding: 8px 12px;
    border: none;
    font-family: 'Outfit', sans-serif;
  }
  table th:first-child { border-radius: 6px 0 0 6px; }
  table th:last-child { border-radius: 0 6px 6px 0; }
  table td {
    background: var(--card);
    padding: 8px 12px;
    border: none;
    font-family: 'Noto Sans JP', 'JetBrains Mono', sans-serif;
    font-size: 0.95em;
  }
  table td:first-child {
    border-radius: 6px 0 0 6px;
    font-weight: 600; color: var(--light);
  }
  table td:last-child { border-radius: 0 6px 6px 0; }

  .form-dict { color: var(--light); }
  .form-polite { color: #5b9bd5; }
  .form-neg { color: var(--accent); }
  .form-past { color: #a78bfa; }

  .ladder-step {
    display: flex; align-items: center; gap: 10px;
    padding: 8px 12px; margin-bottom: 6px;
    background: var(--card); border: 1px solid var(--border);
    border-radius: 10px; position: relative;
  }
  .ladder-step.active {
    border-color: var(--accent);
    box-shadow: 0 0 10px rgba(233,69,96,0.2);
  }
  .ladder-connector {
    width: 2px; height: 6px; background: var(--border); margin-left: 18px;
  }

footer: ''
---

<!-- _class: lead -->
<!-- _paginate: false -->

<!-- Large kanji SVG for "learn" -->
<svg width="110" height="110" viewBox="0 0 110 110">
  <rect x="5" y="5" width="100" height="100" rx="16" fill="var(--card)" stroke="var(--accent)" stroke-width="2"/>
  <text x="55" y="72" text-anchor="middle" fill="var(--light)" font-family="Noto Sans JP" font-size="58" font-weight="900">学</text>
  <text x="55" y="96" text-anchor="middle" fill="var(--accent)" font-family="JetBrains Mono" font-size="12" font-weight="400">manabu</text>
</svg>

# Japanese Essentials

<div class="divider"></div>

<div style="color: #7a8ba8; font-size: 0.85em; margin-bottom: 14px;">Your first steps into the language</div>

<div style="display: flex; gap: 12px; justify-content: center;">
  <span class="level-badge">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#e94560" stroke-width="2"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/></svg>
    JLPT N5
  </span>
  <span class="level-badge" style="border-color: var(--border); color: var(--muted);">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#7a8ba8" stroke-width="2"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>
    24 Phrases
  </span>
</div>

---

### Essential Phrases

## Everyday Japanese

<div class="phrase-grid">
  <div class="phrase-card">
    <svg class="speaker-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#7a8ba8" stroke-width="1.5"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14"/></svg>
    <div class="jp">おはようございます</div>
    <div class="roma">ohayou gozaimasu</div>
    <div class="en">Good morning</div>
    <span class="context-tag tag-formal">formal</span>
  </div>

  <div class="phrase-card">
    <svg class="speaker-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#7a8ba8" stroke-width="1.5"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14"/></svg>
    <div class="jp">すみません</div>
    <div class="roma">sumimasen</div>
    <div class="en">Excuse me / Sorry</div>
    <span class="context-tag tag-formal">formal</span>
  </div>

  <div class="phrase-card">
    <svg class="speaker-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#7a8ba8" stroke-width="1.5"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14"/></svg>
    <div class="jp">ありがとう</div>
    <div class="roma">arigatou</div>
    <div class="en">Thanks</div>
    <span class="context-tag tag-casual">casual</span>
  </div>

  <div class="phrase-card">
    <svg class="speaker-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#7a8ba8" stroke-width="1.5"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14"/></svg>
    <div class="jp">お願いします</div>
    <div class="roma">onegai shimasu</div>
    <div class="en">Please</div>
    <span class="context-tag tag-formal">formal</span>
  </div>

  <div class="phrase-card">
    <svg class="speaker-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#7a8ba8" stroke-width="1.5"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14"/></svg>
    <div class="jp">大丈夫</div>
    <div class="roma">daijoubu</div>
    <div class="en">It's okay / I'm fine</div>
    <span class="context-tag tag-casual">casual</span>
  </div>

  <div class="phrase-card">
    <svg class="speaker-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#7a8ba8" stroke-width="1.5"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14"/></svg>
    <div class="jp">いくらですか</div>
    <div class="roma">ikura desu ka</div>
    <div class="en">How much is it?</div>
    <span class="context-tag tag-formal">formal</span>
  </div>
</div>

---

### Verb Conjugation

## Core Verb Forms

<div class="conj-section">
  <div class="conj-table-wrap">

| Verb | <span class="form-dict">Dictionary</span> | <span class="form-polite">Polite</span> | <span class="form-neg">Negative</span> | <span class="form-past">Past</span> |
|---|---|---|---|---|
| **to eat** | <span class="form-dict">食べる taberu</span> | <span class="form-polite">食べます tabemasu</span> | <span class="form-neg">食べない tabenai</span> | <span class="form-past">食べた tabeta</span> |
| **to go** | <span class="form-dict">行く iku</span> | <span class="form-polite">行きます ikimasu</span> | <span class="form-neg">行かない ikanai</span> | <span class="form-past">行った itta</span> |
| **to do** | <span class="form-dict">する suru</span> | <span class="form-polite">します shimasu</span> | <span class="form-neg">しない shinai</span> | <span class="form-past">した shita</span> |

  </div>

  <div class="difficulty-ladder">
    <div style="font-family: 'Outfit', sans-serif; font-weight: 600; font-size: 0.72em; color: var(--accent); letter-spacing: 0.12em; text-transform: uppercase; margin-bottom: 10px; text-align: center;">JLPT Levels</div>

    <div class="ladder-step" style="opacity: 0.4;">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#a78bfa" stroke-width="1.5"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
      <div>
        <div style="font-weight: 700; font-size: 0.8em; color: #a78bfa;">N1</div>
        <div style="font-size: 0.55em; color: var(--muted);">Fluent</div>
      </div>
    </div>
    <div class="ladder-connector"></div>

    <div class="ladder-step" style="opacity: 0.5;">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#7c3aed" stroke-width="1.5"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
      <div>
        <div style="font-weight: 700; font-size: 0.8em; color: #7c3aed;">N2</div>
        <div style="font-size: 0.55em; color: var(--muted);">Advanced</div>
      </div>
    </div>
    <div class="ladder-connector"></div>

    <div class="ladder-step" style="opacity: 0.6;">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#5b9bd5" stroke-width="1.5"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
      <div>
        <div style="font-weight: 700; font-size: 0.8em; color: #5b9bd5;">N3</div>
        <div style="font-size: 0.55em; color: var(--muted);">Intermediate</div>
      </div>
    </div>
    <div class="ladder-connector"></div>

    <div class="ladder-step" style="opacity: 0.8;">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#f59e0b" stroke-width="1.5"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
      <div>
        <div style="font-weight: 700; font-size: 0.8em; color: #f59e0b;">N4</div>
        <div style="font-size: 0.55em; color: var(--muted);">Elementary</div>
      </div>
    </div>
    <div class="ladder-connector"></div>

    <div class="ladder-step active">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#e94560" stroke-width="2"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
      <div>
        <div style="font-weight: 700; font-size: 0.8em; color: var(--accent);">N5</div>
        <div style="font-size: 0.55em; color: var(--muted);">Beginner</div>
      </div>
      <svg width="10" height="10" viewBox="0 0 10 10" style="position: absolute; right: 10px;"><circle cx="5" cy="5" r="4" fill="var(--accent)"><animate attributeName="opacity" values="1;0.3;1" dur="2s" repeatCount="indefinite"/></circle></svg>
    </div>
  </div>
</div>
