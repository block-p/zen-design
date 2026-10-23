# Install: Line Trend

A line draws with a running value, and one moment gets annotated.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/line-trend?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download line-trend.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `line-trend.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `line-trend (1).zip`, and Safari unzips downloads on its own into a `line-trend` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `line-trend`:

- Claude Code: `~/.claude/skills/line-trend/` (or `.claude/skills/line-trend/` inside a project)
- Codex: `~/.agents/skills/line-trend/`
- Cursor: `.cursor/skills/line-trend/` inside the project
- Gemini CLI: `~/.gemini/skills/line-trend/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/line-trend*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/line-trend/SKILL.md ]; then cp -R ~/Downloads/line-trend ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/line-trend/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the line-trend skill: title "MRR", data "Q1: 12 | Q2: 15 | Q3: 22 | Q4: 41", prefix "$", suffix "k", mark "Q3", note "Launched video ads". Paper theme, 1:1."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
