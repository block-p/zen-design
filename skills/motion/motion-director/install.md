# Install: Motion Director

Describe a video; your agent directs it shot by shot and renders the MP4.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/motion-director?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download motion-director.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `motion-director.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `motion-director (1).zip`, and Safari unzips downloads on its own into a `motion-director` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `motion-director`:

- Claude Code: `~/.claude/skills/motion-director/` (or `.claude/skills/motion-director/` inside a project)
- Codex: `~/.agents/skills/motion-director/`
- Cursor: `.cursor/skills/motion-director/` inside the project
- Gemini CLI: `~/.gemini/skills/motion-director/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/motion-director*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/motion-director/SKILL.md ]; then cp -R ~/Downloads/motion-director ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/motion-director/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the motion-director skill: a 20-second launch video for our 2.0 release, 16:9, from the notes in CHANGELOG.md."
- If they have nothing ready, or just want to see it work: copy the sample `motion-director/examples/CHANGELOG.md` from the installed skill folder into the working folder and run the skill on it (a 20-second launch video for the sample app's 2.0 release, 16:9). Tell them it's made-up sample content they can swap for their own.

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
