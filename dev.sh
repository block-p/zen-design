#!/usr/bin/env bash
# Build the Rust/WASM studio if needed, then start the ZenDesign daemon.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

if ! command -v trunk >/dev/null 2>&1; then
  echo "error: trunk is required to build the Rust/WASM studio." >&2
  echo "       cargo install trunk --locked" >&2
  exit 1
fi

if ! rustup target list --installed 2>/dev/null | grep -qx "wasm32-unknown-unknown"; then
  echo "error: the wasm32 target is not installed." >&2
  echo "       rustup target add wasm32-unknown-unknown" >&2
  exit 1
fi

STUDIO_INDEX="$ROOT/studio/dist/index.html"
STALE="$(find \
  "$ROOT/studio/src" \
  "$ROOT/studio/assets" \
  "$ROOT/studio/Cargo.toml" \
  "$ROOT/studio/Cargo.lock" \
  "$ROOT/studio/Trunk.toml" \
  "$ROOT/studio/index.html" \
  "$ROOT/studio/style.css" \
  -newer "$STUDIO_INDEX" -print -quit 2>/dev/null || true)"

if [[ ! -f "$STUDIO_INDEX" || -n "$STALE" ]]; then
  echo "→ Building the Rust/WASM studio (release)…"
  (cd "$ROOT/studio" && trunk build --release)
fi

echo
echo "  ZenDesign studio   http://127.0.0.1:7456"
echo "  Artifact preview   http://127.0.0.1:7456/preview/index.html"
echo "  Press Ctrl+C to stop."
echo

cd "$ROOT/server"
exec cargo run
