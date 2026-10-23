# Install: Before / After

A wipe comparison with a sweeping divider.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/before-after?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download before-after.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `before-after.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `before-after (1).zip`, and Safari unzips downloads on its own into a `before-after` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `before-after`:

- Claude Code: `~/.claude/skills/before-after/` (or `.claude/skills/before-after/` inside a project)
- Codex: `~/.agents/skills/before-after/`
- Cursor: `.cursor/skills/before-after/` inside the project
- Gemini CLI: `~/.gemini/skills/before-after/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/before-after*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/before-after/SKILL.md ]; then cp -R ~/Downloads/before-after ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/before-after/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the before-after skill with assets/old.png and assets/new.png, title "Checkout, rebuilt", labels "v1" and "v2". 16:9."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
