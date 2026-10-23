# Install: Logo Sting & Outro

A punchy brand reveal or a clean end card.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/logo-sting?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download logo-sting.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `logo-sting.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `logo-sting (1).zip`, and Safari unzips downloads on its own into a `logo-sting` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `logo-sting`:

- Claude Code: `~/.claude/skills/logo-sting/` (or `.claude/skills/logo-sting/` inside a project)
- Codex: `~/.agents/skills/logo-sting/`
- Cursor: `.cursor/skills/logo-sting/` inside the project
- Gemini CLI: `~/.gemini/skills/logo-sting/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/logo-sting*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/logo-sting/SKILL.md ]; then cp -R ~/Downloads/logo-sting ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/logo-sting/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the logo-sting skill with our logo at assets/logo.svg, name "Helix", tagline "Science, accelerated." Make a 16:9 sting and a 9:16 outro with the CTA "helix.bio"."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
