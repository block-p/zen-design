# Install: Manus Keynote Launch

A chaptered launch: a black intro, a ticker of what's new, chapters of the product at work over landscapes, One More Thing, mascots.

- Tier: Premium (bought on its own for $4.99, or unlocked by a subscription or lifetime pass)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/manus-keynote-launch?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download manus-keynote-launch.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `manus-keynote-launch.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `manus-keynote-launch (1).zip`, and Safari unzips downloads on its own into a `manus-keynote-launch` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".
If the page says the skill is locked, the user can buy it on that page for $4.99, or get a plan: https://claude-motion.com/pricing. Once they've paid, ask them to tell you, then run this step again.

## 2. Install

Unzip it so the folder is named `manus-keynote-launch`:

- Claude Code: `~/.claude/skills/manus-keynote-launch/` (or `.claude/skills/manus-keynote-launch/` inside a project)
- Codex: `~/.agents/skills/manus-keynote-launch/`
- Cursor: `.cursor/skills/manus-keynote-launch/` inside the project
- Gemini CLI: `~/.gemini/skills/manus-keynote-launch/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/manus-keynote-launch*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/manus-keynote-launch/SKILL.md ]; then cp -R ~/Downloads/manus-keynote-launch ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/manus-keynote-launch/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use manus-keynote-launch to make a launch film for our 2.0 release at https://your-product.com: one chapter per new capability."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
