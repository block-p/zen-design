# Install: Paper Lecture

Ruled-paper lecture notes with quizzes, drag exercises and a narrated walk-through.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+ and a browser. Optional: narration needs Python 3.10+ and espeak-ng (`bash deck/setup-voice.sh` sets it up once); MP4 export needs FFmpeg and Chrome (the HyperFrames CLI installs itself through npx).

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/paper-lecture?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download paper-lecture.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `paper-lecture.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `paper-lecture (1).zip`, and Safari unzips downloads on its own into a `paper-lecture` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `paper-lecture`:

- Claude Code: `~/.claude/skills/paper-lecture/` (or `.claude/skills/paper-lecture/` inside a project)
- Codex: `~/.agents/skills/paper-lecture/`
- Cursor: `.cursor/skills/paper-lecture/` inside the project
- Gemini CLI: `~/.gemini/skills/paper-lecture/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/paper-lecture*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/paper-lecture/SKILL.md ]; then cp -R ~/Downloads/paper-lecture ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/paper-lecture/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the paper-lecture skill to turn my lecture notes in week3.md into a narrated deck with two quizzes."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
