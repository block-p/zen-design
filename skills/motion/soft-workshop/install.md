# Install: Soft Workshop

A gentle pastel workshop deck with practice built in.

- Tier: Premium (bought on its own for $4.99, or unlocked by a subscription or lifetime pass)
- Engine: hyperframes
- Requirements: Node.js 22+ and a browser. Optional: narration needs Python 3.10+ and espeak-ng (`bash deck/setup-voice.sh` sets it up once); MP4 export needs FFmpeg and Chrome (the HyperFrames CLI installs itself through npx).

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/soft-workshop?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download soft-workshop.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `soft-workshop.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `soft-workshop (1).zip`, and Safari unzips downloads on its own into a `soft-workshop` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".
If the page says the skill is locked, the user can buy it on that page for $4.99, or get a plan: https://claude-motion.com/pricing. Once they've paid, ask them to tell you, then run this step again.

## 2. Install

Unzip it so the folder is named `soft-workshop`:

- Claude Code: `~/.claude/skills/soft-workshop/` (or `.claude/skills/soft-workshop/` inside a project)
- Codex: `~/.agents/skills/soft-workshop/`
- Cursor: `.cursor/skills/soft-workshop/` inside the project
- Gemini CLI: `~/.gemini/skills/soft-workshop/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/soft-workshop*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/soft-workshop/SKILL.md ]; then cp -R ~/Downloads/soft-workshop ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/soft-workshop/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use soft-workshop to make a 45-minute onboarding session on our code review guidelines."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
