#!/usr/bin/env bash
set -euo pipefail

# Build the wasm binary and assemble the web/ bundle.
cargo build --release --target wasm32-unknown-unknown

mkdir -p web
cp target/wasm32-unknown-unknown/release/rust-retro-arcade.wasm web/rust-retro-arcade.wasm
cp index.html web/index.html

echo "Built web/ bundle. Serve it, e.g.:"
echo "  (cd web && python3 -m http.server 8080)"
echo "then open http://localhost:8080"
