---
marp: true
theme: default
paginate: true
style: |
  @import url('https://fonts.googleapis.com/css2?family=Sora:wght@200;400;600;700&display=swap');
  section { background: #09090b; color: #fafafa; font-family: 'Sora', sans-serif; font-weight: 200; padding: 56px 72px; line-height: 1.5; }
  h1 { font-family: 'Sora'; font-weight: 700; font-size: 2.6em; color: #fafafa; letter-spacing: -0.03em; line-height: 1.05; margin: 0 0 4px; }
  h2 { font-family: 'Sora'; font-weight: 200; font-size: 1.2em; color: #555; margin: 0 0 20px; }
  h3 { font-family: 'Sora'; font-weight: 600; font-size: 0.6em; color: #444; text-transform: uppercase; letter-spacing: 0.2em; margin: 0 0 4px; }
  section::after { font-family: 'Sora'; font-size: 0.55em; color: #151515; }
footer: ''
---

### Head to Head

# Claude Code vs Cursor

## Which AI coding tool wins?

<div style="display: flex; justify-content: center; gap: 80px; margin-top: 40px;">
  <div style="text-align: center;">
    <div style="width: 80px; height: 80px; border-radius: 20px; background: #18181b; border: 1px solid #27272a; display: flex; align-items: center; justify-content: center; margin: 0 auto;">
      <svg width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="#f97316" stroke-width="1.2"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z"/><path d="M8 12l3 3 5-5"/></svg>
    </div>
    <div style="font-weight: 600; font-size: 0.95em; margin-top: 12px;">Claude Code</div>
    <div style="font-weight: 200; font-size: 0.65em; color: #555; margin-top: 2px;">Anthropic · CLI-first</div>
  </div>
  <div style="display: flex; align-items: center;">
    <div style="font-weight: 700; font-size: 1.4em; color: #27272a;">vs</div>
  </div>
  <div style="text-align: center;">
    <div style="width: 80px; height: 80px; border-radius: 20px; background: #18181b; border: 1px solid #27272a; display: flex; align-items: center; justify-content: center; margin: 0 auto;">
      <svg width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="#8b5cf6" stroke-width="1.2"><rect x="2" y="3" width="20" height="14" rx="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/></svg>
    </div>
    <div style="font-weight: 600; font-size: 0.95em; margin-top: 12px;">Cursor</div>
    <div style="font-weight: 200; font-size: 0.65em; color: #555; margin-top: 2px;">Anysphere · IDE-first</div>
  </div>
</div>

---

### Feature Comparison

<div style="margin-top: 12px;">
  <div style="display: flex; padding: 10px 0; border-bottom: 1px solid #18181b; font-size: 0.72em;">
    <div style="flex: 2; color: #555; font-weight: 400;">Feature</div>
    <div style="flex: 1; text-align: center; color: #f97316; font-weight: 600;">Claude Code</div>
    <div style="flex: 1; text-align: center; color: #8b5cf6; font-weight: 600;">Cursor</div>
  </div>
  <div style="display: flex; align-items: center; padding: 12px 0; border-bottom: 1px solid #111; font-size: 0.75em;">
    <div style="flex: 2; color: #888; font-weight: 200;">Runs shell commands</div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#ef4444" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg></div>
  </div>
  <div style="display: flex; align-items: center; padding: 12px 0; border-bottom: 1px solid #111; font-size: 0.75em;">
    <div style="flex: 2; color: #888; font-weight: 200;">Multi-file editing</div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></div>
  </div>
  <div style="display: flex; align-items: center; padding: 12px 0; border-bottom: 1px solid #111; font-size: 0.75em;">
    <div style="flex: 2; color: #888; font-weight: 200;">Tab completion</div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#ef4444" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg></div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></div>
  </div>
  <div style="display: flex; align-items: center; padding: 12px 0; border-bottom: 1px solid #111; font-size: 0.75em;">
    <div style="flex: 2; color: #888; font-weight: 200;">Autonomous agents</div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#f59e0b" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="8" y1="12" x2="16" y2="12"/></svg></div>
  </div>
  <div style="display: flex; align-items: center; padding: 12px 0; border-bottom: 1px solid #111; font-size: 0.75em;">
    <div style="flex: 2; color: #888; font-weight: 200;">MCP tool support</div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></div>
  </div>
  <div style="display: flex; align-items: center; padding: 12px 0; border-bottom: 1px solid #111; font-size: 0.75em;">
    <div style="flex: 2; color: #888; font-weight: 200;">Custom skills/prompts</div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></div>
  </div>
  <div style="display: flex; align-items: center; padding: 12px 0; font-size: 0.75em;">
    <div style="flex: 2; color: #888; font-weight: 200;">Git integration</div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></div>
    <div style="flex: 1; text-align: center;"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></div>
  </div>
</div>

---

### Verdict

<div style="display: flex; gap: 2px; margin-top: 16px;">
  <div style="flex: 1; background: #18181b; border-radius: 12px 0 0 12px; padding: 28px; border-top: 3px solid #f97316;">
    <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 16px;">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#f97316" stroke-width="1.5"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z"/><path d="M8 12l3 3 5-5"/></svg>
      <span style="font-weight: 600; font-size: 0.95em; color: #f97316;">Claude Code</span>
    </div>
    <div style="font-weight: 200; font-size: 0.78em; color: #888; line-height: 1.8;">
      Best for autonomous workflows<br>
      Terminal-native power users<br>
      Multi-agent orchestration<br>
      Full system access<br>
      <span style="font-weight: 400; color: #f97316; font-size: 0.9em; margin-top: 8px; display: inline-block;">$200/mo Max plan</span>
    </div>
  </div>
  <div style="flex: 1; background: #18181b; border-radius: 0 12px 12px 0; padding: 28px; border-top: 3px solid #8b5cf6;">
    <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 16px;">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#8b5cf6" stroke-width="1.5"><rect x="2" y="3" width="20" height="14" rx="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/></svg>
      <span style="font-weight: 600; font-size: 0.95em; color: #8b5cf6;">Cursor</span>
    </div>
    <div style="font-weight: 200; font-size: 0.78em; color: #888; line-height: 1.8;">
      Best for inline editing<br>
      Visual diff previews<br>
      Tab completion speed<br>
      Familiar IDE feel<br>
      <span style="font-weight: 400; color: #8b5cf6; font-size: 0.9em; margin-top: 8px; display: inline-block;">$20/mo Pro plan</span>
    </div>
  </div>
</div>

<div style="text-align: center; margin-top: 20px; font-weight: 200; font-size: 0.75em; color: #333;">
  Different tools for different workflows. Most power users run both.
</div>
