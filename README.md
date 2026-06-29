# rust-retro-arcade

A web-first retro arcade written in Rust — a modern, sole-authored rewrite of an
old DOS Turbo C++ project (Pac-Man + Pong with a menu, instructions, and credits
screens).

The original used BGI graphics, `conio.h`, DOS interrupt handlers, and the PC
speaker; this rewrite uses [`macroquad`](https://github.com/not-fl3/macroquad) and
compiles to **WebAssembly** (primary target) and native desktop.

## Status

Playable. Pac-Man and Pong run natively and in the browser (WASM).

- Design spec: [`docs/superpowers/specs/2026-06-29-rust-arcade-design.md`](docs/superpowers/specs/2026-06-29-rust-arcade-design.md)
- Original DOS C++ sources (kept for reference): [`legacy-cpp/`](legacy-cpp/)

## Build & run

```sh
# Native
cargo run

# Web (WASM)
rustup target add wasm32-unknown-unknown   # once
./build-web.sh
(cd web && python3 -m http.server 8080)    # then open http://localhost:8080
```

## License

© Anvesh. All rights reserved (license TBD).
