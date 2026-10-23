# Install: Social Post Card

A post that types itself out and racks up likes.

- Tier: Free (sign in with Google to download)
- Engine: hyperframes
- Requirements: Node.js 22+, FFmpeg, and the HyperFrames CLI (`npx hyperframes --version` installs it on first use). The first render downloads the renderer and its Chrome once (about 600 MB, a few minutes); keep about 1.5 GB of disk free.

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/social-post-card?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download social-post-card.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `social-post-card.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `social-post-card (1).zip`, and Safari unzips downloads on its own into a `social-post-card` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".

## 2. Install

Unzip it so the folder is named `social-post-card`:

- Claude Code: `~/.claude/skills/social-post-card/` (or `.claude/skills/social-post-card/` inside a project)
- Codex: `~/.agents/skills/social-post-card/`
- Cursor: `.cursor/skills/social-post-card/` inside the project
- Gemini CLI: `~/.gemini/skills/social-post-card/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/social-post-card*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/social-post-card/SKILL.md ]; then cp -R ~/Downloads/social-post-card ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/social-post-card/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use the social-post-card skill with this customer post: name "Rina K.", handle "rinabuilds", text "Switched our docs to Folio and support tickets dropped 30% in a week." Likes 3.4k. 9:16."

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
