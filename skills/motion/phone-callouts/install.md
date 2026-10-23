# Install: Phone Callouts

A phone rises in, and labels pop out with leader lines to its screen.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/phone-callouts?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download phone-callouts.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `phone-callouts.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `phone-callouts (1).zip`, and Safari unzips downloads on its own into a `phone-callouts` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `phone-callouts`:

- Claude Code: `~/.claude/skills/phone-callouts/` (or `.claude/skills/phone-callouts/` inside a project)
- Codex: `~/.agents/skills/phone-callouts/`
- Cursor: `.cursor/skills/phone-callouts/` inside the project
- Gemini CLI: `~/.gemini/skills/phone-callouts/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/phone-callouts*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/phone-callouts/SKILL.md ]; then cp -R ~/Downloads/phone-callouts ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/phone-callouts/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the phone-callouts skill with assets/app-home.png and callouts "0.5, 0.2: Your balance, live | 0.3, 0.7: Send in two taps | 0.7, 0.7: Split with friends". Paper theme, 1:1."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
