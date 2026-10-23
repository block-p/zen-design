# Install: Data Chart Story

An animated bar or line chart that lands one takeaway.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/data-chart-story?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download data-chart-story.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `data-chart-story.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `data-chart-story (1).zip`, and Safari unzips downloads on its own into a `data-chart-story` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `data-chart-story`:

- Claude Code: `~/.claude/skills/data-chart-story/` (or `.claude/skills/data-chart-story/` inside a project)
- Codex: `~/.agents/skills/data-chart-story/`
- Cursor: `.cursor/skills/data-chart-story/` inside the project
- Gemini CLI: `~/.gemini/skills/data-chart-story/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/data-chart-story*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/data-chart-story/SKILL.md ]; then cp -R ~/Downloads/data-chart-story ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/data-chart-story/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the data-chart-story skill with our signups: "W1: 320 | W2: 410 | W3: 640 | W4: 980", title "Weekly signups", line chart, insight "3x in a month"."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
