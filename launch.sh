#!/usr/bin/env bash
# ZenDesign desktop app launcher

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVER_BIN="$DIR/server/target/release/zen-design-server"

# Check if server is running on port 7456
if ! curl -s --connect-timeout 1 http://127.0.0.1:7456/api/health >/dev/null 2>&1; then
    echo "Starting ZenDesign backend daemon..."
    if [ -x "$SERVER_BIN" ]; then
        cd "$DIR/server" && ./target/release/zen-design-server >/dev/null 2>&1 &
        sleep 1.2
    fi
fi

# Launch app in standalone window
if command -v google-chrome >/dev/null 2>&1; then
    exec google-chrome --app="http://127.0.0.1:7456" "$@"
elif command -v chromium >/dev/null 2>&1; then
    exec chromium --app="http://127.0.0.1:7456" "$@"
else
    xdg-open "http://127.0.0.1:7456"
fi
