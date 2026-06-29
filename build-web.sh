#!/usr/bin/env bash
set -euo pipefail

# Build the wasm binary and assemble a self-contained web/ bundle.
# All JS is vendored under js/ (no CDN dependency), so the bundle works offline.
cargo build --release --target wasm32-unknown-unknown

rm -rf web
mkdir -p web/js
cp target/wasm32-unknown-unknown/release/rust-retro-arcade.wasm web/rust-retro-arcade.wasm
cp index.html web/index.html
cp js/mq_js_bundle.js web/js/

echo "Built self-contained web/ bundle (no CDN). Serve it, e.g.:"
echo "  (cd web && python3 -m http.server 8080)"
echo "then open http://localhost:8080"
