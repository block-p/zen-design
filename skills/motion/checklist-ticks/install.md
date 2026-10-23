# Install: Checklist Ticks

A checklist where every box ticks itself, one by one.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/checklist-ticks?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download checklist-ticks.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `checklist-ticks.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `checklist-ticks (1).zip`, and Safari unzips downloads on its own into a `checklist-ticks` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `checklist-ticks`:

- Claude Code: `~/.claude/skills/checklist-ticks/` (or `.claude/skills/checklist-ticks/` inside a project)
- Codex: `~/.agents/skills/checklist-ticks/`
- Cursor: `.cursor/skills/checklist-ticks/` inside the project
- Gemini CLI: `~/.gemini/skills/checklist-ticks/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/checklist-ticks*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/checklist-ticks/SKILL.md ]; then cp -R ~/Downloads/checklist-ticks ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/checklist-ticks/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the checklist-ticks skill: title "Launch day", items "Docs live | Pricing page | Changelog | Tweet thread | Demo video", done badge "All set". Electric theme, 9:16."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
