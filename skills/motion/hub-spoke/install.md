# Install: Hub & Spoke

A central hub radiates out to everything it connects.

- Tier: Premium (bought on its own for $4.99, or unlocked by a subscription or lifetime pass)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/hub-spoke?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download hub-spoke.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `hub-spoke.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `hub-spoke (1).zip`, and Safari unzips downloads on its own into a `hub-spoke` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".
If the page says the skill is locked, the user can buy it on that page for $4.99, or get a plan: https://claude-motion.com/pricing. Once they've paid, ask them to tell you, then run this step again.

## 2. Install

Unzip it so the folder is named `hub-spoke`:

- Claude Code: `~/.claude/skills/hub-spoke/` (or `.claude/skills/hub-spoke/` inside a project)
- Codex: `~/.agents/skills/hub-spoke/`
- Cursor: `.cursor/skills/hub-spoke/` inside the project
- Gemini CLI: `~/.gemini/skills/hub-spoke/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/hub-spoke*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/hub-spoke/SKILL.md ]; then cp -R ~/Downloads/hub-spoke ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/hub-spoke/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the hub-spoke skill: hub "Brand kit", spokes "Website | Decks | Social | Video | Email", title "One source of truth". Paper theme, 1:1."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
