# Install: Lyric Video

Timed lyrics with karaoke fill, kinetic words or stacking lines.

- Tier: Premium (bought on its own for $4.99, or unlocked by a subscription or lifetime pass)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/lyric-video?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download lyric-video.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `lyric-video.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `lyric-video (1).zip`, and Safari unzips downloads on its own into a `lyric-video` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".
If the page says the skill is locked, the user can buy it on that page for $4.99, or get a plan: https://claude-motion.com/pricing. Once they've paid, ask them to tell you, then run this step again.

## 2. Install

Unzip it so the folder is named `lyric-video`:

- Claude Code: `~/.claude/skills/lyric-video/` (or `.claude/skills/lyric-video/` inside a project)
- Codex: `~/.agents/skills/lyric-video/`
- Cursor: `.cursor/skills/lyric-video/` inside the project
- Gemini CLI: `~/.gemini/skills/lyric-video/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/lyric-video*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/lyric-video/SKILL.md ]; then cp -R ~/Downloads/lyric-video ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/lyric-video/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the lyric-video skill with lyrics.lrc for our 2:40 song at 96 BPM, kinetic style, 9:16. Mux song.mp3 at the end."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
