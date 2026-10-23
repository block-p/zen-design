# Install: Kinetic Type Title

An editorial headline reveal in three motion styles.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/kinetic-type-title?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download kinetic-type-title.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `kinetic-type-title.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `kinetic-type-title (1).zip`, and Safari unzips downloads on its own into a `kinetic-type-title` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `kinetic-type-title`:

- Claude Code: `~/.claude/skills/kinetic-type-title/` (or `.claude/skills/kinetic-type-title/` inside a project)
- Codex: `~/.agents/skills/kinetic-type-title/`
- Cursor: `.cursor/skills/kinetic-type-title/` inside the project
- Gemini CLI: `~/.gemini/skills/kinetic-type-title/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/kinetic-type-title*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/kinetic-type-title/SKILL.md ]; then cp -R ~/Downloads/kinetic-type-title ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/kinetic-type-title/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the kinetic-type-title skill to make a 9:16 title card for our launch: kicker "New", headline "Ship faster | sleep better", subline "Acme CI is now 3x faster." Use the paper theme with our accent #2b59ff."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
