# Install: Lower Thirds

Name and title bars in four broadcast styles.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/lower-thirds?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download lower-thirds.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `lower-thirds.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `lower-thirds (1).zip`, and Safari unzips downloads on its own into a `lower-thirds` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `lower-thirds`:

- Claude Code: `~/.claude/skills/lower-thirds/` (or `.claude/skills/lower-thirds/` inside a project)
- Codex: `~/.agents/skills/lower-thirds/`
- Cursor: `.cursor/skills/lower-thirds/` inside the project
- Gemini CLI: `~/.gemini/skills/lower-thirds/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/lower-thirds*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/lower-thirds/SKILL.md ]; then cp -R ~/Downloads/lower-thirds ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/lower-thirds/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the lower-thirds skill to make a transparent overlay for our interview: name "Dr. Lena Ortiz", role "Chief Scientist, Helix", pill style, bottom left. Render it as a MOV with transparency."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
