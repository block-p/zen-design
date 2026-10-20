# ZenDesign

A local-first AI design agent workspace built as a fast, resilient alternative to
OpenDesign. A native Rust daemon streams agent progress over a zero-timeout SSE
connection and serves the generated artifact. The studio itself is a
**Rust/WASM Leptos application** compiled with Trunk and served by the same
daemon — no React, Vite, or Node process anywhere in the build or runtime.

```
zen-design/
├── server/          Rust daemon — Axum + Tokio + notify
│   └── src/
│       ├── main.rs      Routes, SSE heartbeats, studio + preview serving
│       ├── agent.rs     Orchestrator, prompts, CSP hardening, offline template
│       ├── providers.rs Model adapters: OpenAI-compatible, Anthropic, OpenCode CLI
│       ├── settings.rs  Providers, model choice, 0600 config persistence
│       └── watcher.rs   300ms debounced workspace watcher
├── studio/          Rust studio — Leptos 0.8 + WASM + bilingual English/Persian UI
│   ├── src/
│   │   ├── app.rs           Root shell, reactive signals, html lang/dir sync
│   │   ├── i18n.rs          Bilingual dictionary, LTR/RTL toggle, Persian digits
│   │   ├── settings.rs      Provider metadata + settings/models API types
│   │   ├── stream.rs        EventSource + heartbeat watchdog + token progress
│   │   └── components/{header,agent_panel,activity_log,preview_panel,settings_panel}.rs
│   ├── style.css         FarsiUI light design system (handcrafted CSS, no Node toolchain)
│   └── assets/           Inter Variable & Vazirmatn font files
└── workspace/       The live folder the agent writes
    └── index.html   Initial artifact, so the preview is never empty
```

## Visual language & Persian-first design

ZenDesign's studio interface is inspired by **FarsiUI**:
- **Light neutral canvas**: Crisp slate/gray surfaces (`#f8fafc`, `#ffffff`), subtle borders (`#e2e8f0`), rounded cards, and primary blue interactive accents (`#2563eb`).
- **Bilingual English & Persian**: Persistent language toggle in the header (`EN` / `فا`), stored in `localStorage` under `zen-language`.
- **RTL & typography**: Sets `<html dir="rtl" lang="fa">` dynamically with zero-flash boot script. Uses **Vazirmatn** for Persian text and **Inter Variable** for English, with localized Persian numerals and logical CSS layout properties.
- **Brand integrity**: Latin design system brand names (*Stripe Clean*, *Linear Dark*, *Cyberpunk Neon*, *Swiss International*) remain in Latin across both languages.


## Requirements

- Rust **1.88+** with the `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev/) **0.21+**
- No Node.js, npm, React, or Vite

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
```

Optional, for MP4 export only: Google Chrome (or Chromium/Brave), `ffmpeg`, and
`Xvfb` (`sudo apt install xvfb ffmpeg`). The rest of ZenDesign runs without them.

## Run it

From the project root:

```bash
./dev.sh
```

The script builds the Rust/WASM studio in release mode when its sources are
newer than `studio/dist/`, then starts the daemon. Open **http://127.0.0.1:7456**.

Or do it manually:

```bash
cd studio
trunk build --release

cd ../server
cargo run
```

Open **http://127.0.0.1:7456** — Axum serves the studio at `/`, the API at
`/api/*`, and the live artifact at `/preview/*` on one origin. The bind address
can be overridden, for example `ZEN_BIND_ADDR=127.0.0.1:7457 ./dev.sh`.

For frontend iteration, run `trunk watch --release` from `studio/` in one
terminal and the daemon in another. Trunk rewrites `studio/dist/` and the daemon
serves the new bundle on refresh; no second web server is involved.

The daemon can also be used without the studio:

```bash
curl http://127.0.0.1:7456/api/health
curl -N http://127.0.0.1:7456/api/events
# then open http://127.0.0.1:7456/preview/index.html in a browser
```

## API

| Method | Route                            | Purpose                                                                             |
| ------ | -------------------------------- | ----------------------------------------------------------------------------------- |
| `GET`  | `/`                              | Rust/WASM studio bundle                                                             |
| `GET`  | `/api/health`                    | `{ "status": "ok" }`                                                                |
| `GET`  | `/api/events`                    | SSE: `heartbeat` every 2s, plus `status`, `token`, `writing`, `done`, `agent_error`, `reload`, `export_status`, `export_done`, `export_error` |
| `POST` | `/api/prompt`                    | Accepts `{ prompt, design_system, mode, deep? }`, returns `202` immediately        |
| `POST` | `/api/agent/stop`                | Cancels the active run (aborts the provider call, kills an OpenCode child)          |
| `POST` | `/api/export`                    | Records the preview to MP4, `{ "seconds": 8 }` (3–20); `202` immediately, progress over SSE |
| `GET`  | `/api/export/latest`             | Downloads the most recent MP4 (`Content-Disposition: attachment`)                   |
| `GET`  | `/api/settings`                  | Public provider settings; API keys are reported only as `api_key_set`               |
| `PUT`  | `/api/settings`                  | Select provider and update base URL, model, or API key                              |
| `POST` | `/api/settings/import-opencode`  | Import the OpenAI-compatible/9Router provider already configured in OpenCode        |
| `GET`  | `/api/models?provider=…`         | Live model list (`openai`, `anthropic`, `opencode`); model fields also accept free text |
| `GET`  | `/preview/*`                     | Static files from `workspace/` (index served at `/preview/`)                        |

```bash
curl -X POST http://127.0.0.1:7456/api/prompt \
  -H 'Content-Type: application/json' \
  -d '{
    "prompt": "A calm analytics dashboard for a climate startup",
    "design_system": "linear",
    "mode": "prototype"
  }'
```

`design_system` is one of `stripe`, `linear`, `cyberpunk`, `swiss`; `mode` is one
of `prototype`, `slides`, `motion`. `deep` is optional and defaults to `false`;
set it to `true` to run the render → critique → rewrite loop described below.

## Rendering model

Leptos compiles the studio's components and reactive state to WebAssembly; the
browser's DOM is built from Rust view macros, not from a JavaScript framework.
Axum serves the compiled `.wasm`, JS loader, CSS, and fonts. The artifact itself
is intentionally plain HTML generated and served by Rust, so it renders inside a
sandboxed same-origin iframe without any client framework at all.

Tailwind's compiled stylesheet is checked in as `studio/style.css`. It is a
static asset: rebuilding or running ZenDesign never invokes Node or a CSS
toolchain.

The offline template embeds its own compiled Tailwind CSS
(`server/assets/preview.css`), so its preview makes zero external requests: no
CDN, no fonts, nothing. Model-backed artifacts are held to the same bar by the
system prompt and the injected CSP. Regenerate the embedded stylesheet with the
standalone CLI if needed:

```sh
tailwindcss -i server/assets/preview.input.css -o server/assets/preview.css --minify
```

## MP4 export

The download button beside **Reload preview** records the artifact to an MP4
(1440×900, 30 fps, H.264, 8 seconds). The headline, hero animation and any CSS/JS
motion are captured exactly as the browser paints them.

The daemon does not bundle a browser. It launches the Chrome/Chromium/Brave you
already have on a private `Xvfb` display, records that display with `ffmpeg
x11grab`, and serves the result from `/api/export/latest`. Video work happens
entirely outside `workspace/`, so exporting never triggers a preview reload and
never touches the artifact.

Progress arrives as `export_status` frames; `export_done` triggers the studio's
download. Recordings land in `~/.cache/zen-design/exports/preview.mp4`. Binary
locations and the destination can be overridden with `ZEN_CHROME_BIN`,
`ZEN_FFMPEG_BIN`, `ZEN_XVFB_BIN`, and `ZEN_EXPORT_DIR`. If Chrome is already
recording, `POST /api/export` answers `409`.

The recording starts on the artifact's first frame: a black holding page is
loaded first, ffmpeg starts, and only then is the artifact mounted (`/api/export/stage`
polls `/api/export/armed`). Expect roughly 12 seconds for an 8-second clip — the
browser needs a few seconds to open and the encoder needs a moment to finalize.

## Deep review

**Deep review** in the composer turns a one-shot generation into a bounded
render → critique → rewrite loop. It is off by default because it is slower and
costs more tokens.

1. Generate the first cut with the selected provider.
2. Render it on the same private Xvfb display used by MP4 export and capture
   four stills (roughly 1s, 3.5s, 6.5s and 9.5s into the film).
3. Send those stills plus the current document, the original brief, and a luma
   report back to the model. It names up to three material problems and
   rewrites only if one is real.
4. Render the revision and check it before accepting it. A revision is rejected
   if it drops quoted copy from the brief, invents numbers or more than one new
   word, references external files, or collapses the frame contrast. Rejections
   get one correction attempt with the reason attached; a second failure keeps
   the previous cut.
5. Repeat for up to three accepted passes, or until the 45-minute budget runs
   out. Only the final accepted document is written to `workspace/index.html`,
   so the preview reloads once at the end.

The loop is cancellable from the same **Stop** button. It needs an
OpenAI-compatible or Anthropic API provider and the same Chrome/ffmpeg/Xvfb
tools as MP4 export; the OpenCode provider runs one-shot because it has no
attached-frame path. Vision review also needs a model that accepts multiple
images: some Gemini gateways accept one image but return an empty response for
four, in which case the loop keeps the one-shot cut. If the initial cut cannot
be rendered, Deep review is skipped and the one-shot artifact is kept.

```bash
curl -X POST http://127.0.0.1:7456/api/prompt \
  -H 'Content-Type: application/json' \
  -d '{
    "prompt": "A 10-second kinetic title film for NOVA",
    "design_system": "linear",
    "mode": "motion",
    "deep": true
  }'
```

## Connecting models

Open the gear button in the studio header. Everything is local: there is no
ZenDesign account, no OAuth flow, and no cloud service in the middle. A provider
is simply an endpoint or a local process that you already control.

| Provider               | What it does                                                                | Typical use                                                    |
| ---------------------- | --------------------------------------------------------------------------- | -------------------------------------------------------------- |
| **9Router / OpenAI**   | Streams from `{base_url}/chat/completions` (OpenAI-compatible)              | 9Router, OpenRouter, Ollama, LM Studio, vLLM, OpenAI            |
| **Anthropic**          | Native Messages API with SSE                                                | Claude Opus / Sonnet / Haiku directly                           |
| **OpenCode agent**     | Runs `opencode run --format json --auto` inside `workspace/`                 | Your local agent, its tools, and every model OpenCode knows     |
| **Offline template**   | Deterministic Rust generator, no model call                                  | Zero-network demos and a guaranteed fallback                    |

**9Router / Antigravity.** Choose *9Router / OpenAI* and press **Import 9Router**.
The daemon reads the OpenAI-compatible provider already configured in
`~/.config/opencode/opencode.json`, fills in the base URL and key, and asks the
endpoint for its live model list. Models prefixed `ag/*` are Antigravity-backed
(Claude, Gemini, GPT-OSS); `kr/*` route Claude, DeepSeek, GLM, Qwen, and others.
The import picks `ag/claude-opus-4-6-thinking` when it is available; any model ID
can also be typed by hand.

**Keys.** Keys are written to `~/.config/zen-design/config.toml` with `0600`
permissions (override the location with `ZEN_CONFIG_PATH`). The API never returns
a key: the studio only learns `api_key_set` and whether it came from the config
or the environment. The key field shows a masked placeholder and leaves the
stored value untouched unless you type a replacement or press *Clear key*.
Environment fallbacks are `OPENAI_API_KEY` and `ANTHROPIC_API_KEY`.

**Plain HTTP.** If an imported endpoint is a non-loopback `http://` URL, the
dialog warns: prompts and the bearer key cross the network unencrypted. Prefer
HTTPS, a VPN/tunnel, or a provider running on localhost.

**OpenCode.** The bridge needs `opencode` on `PATH` (`ZEN_OPENCODE_BIN`
overrides the binary). The agent inherits your OpenCode models, including
9Router. It may either return HTML for ZenDesign to write, or write
`workspace/index.html` itself; the daemon detects both. Saved settings apply to
the next prompt immediately. Restart the daemon only when upgrading from a build
that predates the provider routes.

## Prompt contract

`server/src/agent.rs` owns the prompts and the artifact pipeline. For every
model-backed run it sends the selected design tokens, the mode rules, and a
strict contract: one self-contained HTML document, inline CSS/JS only, no
external requests. Motion mode produces a different artifact: an 8–12 second,
self-playing film on a fixed 16:9 artboard with 3–5 beats, kinetic typography,
one graphic event, a seamless loop, and `prefers-reduced-motion` support. It is
explicitly not a website — no nav, sections, cards, pricing, FAQ or footer.
Slides mode likewise builds a keyboard-navigable 16:9 deck, not a scrolling page.

The prompt also carries concrete craft standards: one committed art direction, a
12-column grid, a fixed spacing scale, a modular type scale, contrast floors,
real copy, accessible interaction states, and an explicit anti-pattern list
(generic feature-card rows, gradient blobs, glassmorphism everywhere, emoji as
icons). "Make it beautiful" produces templates; measurable constraints produce
designed pages.

Replies are streamed as `token` events (batched to a few frames per second) and
the studio shows a single live "Writing artifact · N KB" entry. The returned
HTML is extracted, capped at 512 KB, given an inline CSP with
`connect-src 'none'`, and written to `workspace/index.html`. Agent-written
artifacts get the same CSP when they are self-contained; deliberately multi-file
artifacts are left alone rather than being broken. The offline template is used
only when the provider is set to *Offline template*.

## Why it stays stable

- **No reasoning timeout.** `/api/events` has no deadline. A named `heartbeat`
  frame is emitted every 2 seconds, so the EventSource stays alive while an LLM
  is thinking. The browser's native reconnection is reflected as
  `Reconnecting` / `Heartbeat lost` in the header; a 6.5s watchdog catches
  half-open streams where the socket stays alive but pings stop.
- **No reload storms.** `notify` feeds a 300ms quiet-window debouncer. Six rapid
  writes produce exactly one `reload` event. Each event bumps a cache-busting
  preview URL inside a fault-tolerant iframe.
- **No mid-run preview flicker.** While an agent is writing (`OpenCode` rewrites
  `index.html` on every step), reload events are buffered and applied once when
  the run finishes. If a reload lands on a mid-write empty document, the preview
  retries it transparently before ever showing a fallback panel.
- **No runaway runs.** A run can be cancelled from the studio; the daemon aborts
  the provider future and kills the OpenCode child process.
- **No half-baked exports.** MP4 recording is disabled while an agent is writing
  and while another export is running. The daemon verifies Chrome mapped a window
  on the virtual display before capturing, so a browser that opens elsewhere
  fails loudly instead of producing a black video.
- **No blank artifacts.** The preview has a 12s watchdog and an empty-document
  check. If the daemon is restarting, the studio shows a retry panel instead of a
  permanent white screen.
- **No request/agent collision.** `POST /api/prompt` returns immediately and the
  orchestrator runs in a serialized background task; progress arrives over SSE.
- **One process, one origin.** The daemon serves the WASM studio, API, and
  artifact preview together. There is no Vite proxy, no CORS hop, and no Node
  runtime. The workspace path can be overridden with `ZEN_WORKSPACE_DIR`.

## Manual verification

With the daemon running:

1. **Health**

   ```bash
   curl http://127.0.0.1:7456/api/health
   # {"status":"ok"}
   ```

2. **Rust studio**

   ```bash
   curl -sI http://127.0.0.1:7456/ | head -1
   # HTTP/1.1 200 OK
   curl -s http://127.0.0.1:7456/ | grep -o 'zen-design-studio[^"]*\.wasm'
   ```

3. **Heartbeat** (one frame immediately, then every 2s)

   ```bash
   timeout 5 curl -N http://127.0.0.1:7456/api/events
   # event: heartbeat
   # data: {"interval_ms":2000,...}
   ```

4. **Debounced reload** — in another terminal:

   ```bash
   for i in 1 2 3 4 5 6; do echo "edit $i" >> workspace/notes.txt; sleep 0.05; done
   ```

   Watch the SSE terminal (or the studio footer): exactly **one** `reload`
   event appears ~300ms after the last write, and the iframe refreshes once.

5. **Agent loop** — type a brief in the studio and press **Generate** (or
   `Ctrl/Cmd+Enter`). The activity log shows Thinking → Writing *N* KB →
   Finished → Reloaded, and the preview updates without a manual refresh.

6. **Model settings** — press the gear button, pick a provider, and save. The
   same round trip over HTTP:

   ```bash
   curl -s http://127.0.0.1:7456/api/settings | jq '{provider, openai: .openai.model}'
   curl -s -X POST http://127.0.0.1:7456/api/settings/import-opencode | jq '.models | length'
   curl -s 'http://127.0.0.1:7456/api/models?provider=openai' | jq '.models[:5]'
   ```

   The settings response must never contain a raw key — only
   `api_key_set: true/false`.

## Production build

```bash
cd studio
trunk build --release

cd ../server
cargo build --release
```

The optimized WASM bundle is typically ~380 KB. `cargo build --release` produces
`server/target/release/zen-design-server`; run it from a directory containing
`studio/dist/` and `workspace/`, or use the manifest-relative path resolution
built into the daemon. Set `ZEN_WORKSPACE_DIR` to put the generated artifact
somewhere else.
