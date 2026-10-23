# Install: Data → Video

Turn a spreadsheet or CSV into a short data video with the story found for you.

- Tier: Premium (bought on its own for $4.99, or unlocked by a subscription or lifetime pass)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/data-to-video?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download data-to-video.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `data-to-video.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `data-to-video (1).zip`, and Safari unzips downloads on its own into a `data-to-video` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".
If the page says the skill is locked, the user can buy it on that page for $4.99, or get a plan: https://claude-motion.com/pricing. Once they've paid, ask them to tell you, then run this step again.

## 2. Install

Unzip it so the folder is named `data-to-video`:

- Claude Code: `~/.claude/skills/data-to-video/` (or `.claude/skills/data-to-video/` inside a project)
- Codex: `~/.agents/skills/data-to-video/`
- Cursor: `.cursor/skills/data-to-video/` inside the project
- Gemini CLI: `~/.gemini/skills/data-to-video/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/data-to-video*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/data-to-video/SKILL.md ]; then cp -R ~/Downloads/data-to-video ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/data-to-video/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the data-to-video skill on revenue.csv: a 25-second results video for the team, 16:9."
- If they have nothing ready, or just want to see it work: copy the sample `data-to-video/examples/revenue.csv` from the installed skill folder into the working folder and run the skill on it (a 20-second results video, 16:9). Tell them it's made-up sample content they can swap for their own.

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
