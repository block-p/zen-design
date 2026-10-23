# Install: Doc → Narrated Deck

Hand your agent a document; get a narrated, interactive deck and an MP4.

- Tier: Premium (bought on its own for $4.99, or unlocked by a subscription or lifetime pass)
- Engine: hyperframes
- Requirements: Node.js 22+ and a browser. Optional: narration needs Python 3.10+ and espeak-ng (`bash deck/setup-voice.sh` sets it up once); MP4 export needs FFmpeg and Chrome (the HyperFrames CLI installs itself through npx).

## 1. Download

Ask the user to open this link in their browser:

https://claude-motion.com/api/download/doc-to-narrated-deck?src=agent

If they're signed in, the download starts at once. If not, it asks them to sign in with Google, then the skill page opens with an orange **Download doc-to-narrated-deck.zip** button to press. Ask them to tell you when the file is downloaded.

It downloads `doc-to-narrated-deck.zip` (usually to ~/Downloads). Look before asking for it again: a repeat download is saved as `doc-to-narrated-deck (1).zip`, and Safari unzips downloads on its own into a `doc-to-narrated-deck` folder and removes the .zip. The script below handles all three; ask the user to download again only if it prints "not found".
If the page says the skill is locked, the user can buy it on that page for $4.99, or get a plan: https://claude-motion.com/pricing. Once they've paid, ask them to tell you, then run this step again.

## 2. Install

Unzip it so the folder is named `doc-to-narrated-deck`:

- Claude Code: `~/.claude/skills/doc-to-narrated-deck/` (or `.claude/skills/doc-to-narrated-deck/` inside a project)
- Codex: `~/.agents/skills/doc-to-narrated-deck/`
- Cursor: `.cursor/skills/doc-to-narrated-deck/` inside the project
- Gemini CLI: `~/.gemini/skills/doc-to-narrated-deck/`

Example (Claude Code):

```bash
mkdir -p ~/.claude/skills
zip=$(ls -t ~/Downloads/doc-to-narrated-deck*.zip 2>/dev/null | head -1)
if [ -n "$zip" ]; then unzip -oq "$zip" -d ~/.claude/skills/
elif [ -f ~/Downloads/doc-to-narrated-deck/SKILL.md ]; then cp -R ~/Downloads/doc-to-narrated-deck ~/.claude/skills/
else echo "not found in ~/Downloads"; fi
test -f ~/.claude/skills/doc-to-narrated-deck/SKILL.md && echo installed
```

## 3. Make a first video

Tell the user the skill is installed, then make a first video with it (read its SKILL.md first):

- If they have material ready (a file, a folder, this project, a link or a topic), ask for it and for what the video is for, then follow SKILL.md. A typical request: "Use doc-to-narrated-deck on onboarding-guide.pdf: make a narrated training deck with a quiz every few slides, and export an MP4."
- If they have nothing ready, or just want to see it work: copy the sample `doc-to-narrated-deck/examples/onboarding-guide.md` from the installed skill folder into the working folder and run the skill on it (a narrated training deck of about 8 slides with one quiz, and an MP4 (narration only if the voice is already set up)). Tell them it's made-up sample content they can swap for their own.

Before the first render, tell the user it downloads the renderer once (about 600 MB) and takes a few minutes.
