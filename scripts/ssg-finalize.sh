#!/usr/bin/env bash
# Finalize the SSG output after `dx build --fullstack --ssg`.
#
# The dx SSG phase runs in parallel with the client build, so what the
# incremental renderer used as its page shell races with the client build's
# asset copy of index.html — and on a fresh CI checkout there is no
# index.html at all at SSG time, so every prerendered route can come out
# with the renderer's default shell (no title, no meta, no stylesheets, no
# wasm bootstrap script). This step makes the result deterministic:
#
# 1. Take the post-build public/index.html (the client-emitted template
#    with dx's wasm bootstrap tags, or an already-correct prerender) as the
#    shell, and construct the bootstrap tags if they are missing.
# 2. Delete every prerendered route output — the renderer serves cached
#    files when they exist, so broken copies must be invalidated.
# 3. Re-render all routes through the built server.
# 4. Collapse the prerendered 404 page into the `404.html` SPA fallback
#    GitHub Pages serves for unknown paths.
#
# Usage: scripts/ssg-finalize.sh [public_dir] [server_binary]
set -euo pipefail

cd "$(dirname "$0")/.."
PUBLIC_DIR="${1:-target/dx/companion-landing-page/release/web/public}"
SERVER_BIN="${2:-target/dx/companion-landing-page/release/web/server}"
PORT="${SSG_PORT:-9944}"

if [ ! -x "$SERVER_BIN" ]; then
    echo "server binary not found at $SERVER_BIN — run 'dx build --fullstack --ssg --release' first" >&2
    exit 1
fi
if [ ! -f "$PUBLIC_DIR/index.html" ]; then
    echo "no index.html in $PUBLIC_DIR — run 'dx build --fullstack --ssg --release' first" >&2
    exit 1
fi

# ── 1. Build a shell that boots the app ─────────────────────────────────
cp "$PUBLIC_DIR/index.html" "$PUBLIC_DIR/.shell.html"
if ! grep -q 'type="module"' "$PUBLIC_DIR/.shell.html"; then
    GLUE_URL="/./assets/$(ls -t "$PUBLIC_DIR"/assets/companion-landing-page-dxh*.js | head -1 | xargs basename)"
    python3 - "$PUBLIC_DIR/.shell.html" "$GLUE_URL" <<'PYEOF'
import sys
path, glue = sys.argv[1], sys.argv[2]
s = open(path).read()
s = s.replace('</head>', f'<link rel="preload" as="script" href="{glue}" crossorigin></head>', 1)
s = s.replace('</body>', f'<script type="module" async src="{glue}"></script></body>', 1)
open(path, 'w').write(s)
PYEOF
fi
# The renderer reads its page shell from public/index.html when the server
# starts — put the shell there (GET "/" re-renders over it afterwards).
mv "$PUBLIC_DIR/.shell.html" "$PUBLIC_DIR/index.html"

# ── 2. Invalidate every cached route output ─────────────────────────────
rm -rf "$PUBLIC_DIR/404.html" \
       "$PUBLIC_DIR/docs" "$PUBLIC_DIR/plugins" "$PUBLIC_DIR/legal"

# ── 3. Re-render all routes through the built server ────────────────────
IP=127.0.0.1 PORT="$PORT" "$SERVER_BIN" >/dev/null 2>&1 &
SERVER_PID=$!
trap 'kill "$SERVER_PID" 2>/dev/null || true' EXIT

for _ in $(seq 1 50); do
    if curl -sf -o /dev/null "http://127.0.0.1:$PORT/api/detect_os"; then
        break
    fi
    sleep 0.2
done

routes=$(curl -sf -X POST -d '{}' "http://127.0.0.1:$PORT/api/static_routes")
for route in $(echo "$routes" | python3 -c 'import json,sys; print("\n".join(json.load(sys.stdin)))'); do
    curl -sf -H "Accept: text/html" -o /dev/null "http://127.0.0.1:$PORT$route"
    echo "prerendered $route"
done

kill "$SERVER_PID" 2>/dev/null || true
wait "$SERVER_PID" 2>/dev/null || true

# ── 4. Collapse the 404 page into the GitHub Pages SPA fallback ─────────
if [ -f "$PUBLIC_DIR/404.html/index.html" ]; then
    mv "$PUBLIC_DIR/404.html/index.html" "$PUBLIC_DIR/404.html.part"
    rmdir "$PUBLIC_DIR/404.html"
    mv "$PUBLIC_DIR/404.html.part" "$PUBLIC_DIR/404.html"
    echo "collapsed 404.html/index.html -> 404.html"
fi
rm -f "$PUBLIC_DIR/index.html.bak"

# ── Sanity: pages must be prerendered AND boot the wasm ─────────────────
for f in index.html docs/index.html 404.html; do
    if ! grep -q 'data-node-hydration' "$PUBLIC_DIR/$f"; then
        echo "WARNING: $f has no hydration markers (not prerendered?)" >&2
        exit 1
    fi
    if ! grep -q 'type="module"' "$PUBLIC_DIR/$f"; then
        echo "WARNING: $f has no wasm bootstrap script (page would never hydrate)" >&2
        exit 1
    fi
done
echo "SSG finalize complete"
