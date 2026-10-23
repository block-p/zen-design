# Install: Changelog Video

Release notes as a crisp, shareable clip.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/changelog-video?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download changelog-video.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `changelog-video.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `changelog-video (1).zip`, and Safari unzips downloads on its own into a `changelog-video` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `changelog-video`:

- Claude Code: `~/.claude/skills/changelog-video/` (or `.claude/skills/changelog-video/` inside a project)
- Codex: `~/.agents/skills/changelog-video/`
- Cursor: `.cursor/skills/changelog-video/` inside the project
- Gemini CLI: `~/.gemini/skills/changelog-video/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/changelog-video*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/changelog-video/SKILL.md ]; then cp -R ~/Downloads/changelog-video ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/changelog-video/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the changelog-video skill for release 3.1 of Orbit: "New: Calendar sync | Improved: 40% faster load | Fixed: Export on Safari". Make a 1:1 version for social."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
