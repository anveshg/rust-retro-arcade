# rust-retro-arcade

A web-first retro arcade written in Rust — a modern, sole-authored rewrite of an
old DOS Turbo C++ project (Pac-Man + Pong with a menu, instructions, and credits
screens).

The original used BGI graphics, `conio.h`, DOS interrupt handlers, and the PC
speaker; this rewrite uses [`macroquad`](https://github.com/not-fl3/macroquad) and
compiles to **WebAssembly** (primary target) and native desktop.

## Status

🚧 In development. The design is complete; implementation is starting.

- Design spec: [`docs/superpowers/specs/2026-06-29-rust-arcade-design.md`](docs/superpowers/specs/2026-06-29-rust-arcade-design.md)
- Original DOS C++ sources (kept for reference): [`legacy-cpp/`](legacy-cpp/)

## Planned

- **Pac-Man** — tile-based maze, pellets, two chasing ghosts, lives, speed levels.
- **Pong** — vs-AI and 2-player, angle-based paddle bounce, first-to-N scoring.
- Sound effects, high-score persistence (browser localStorage / native file),
  and a main menu tying it together.

## Build & run (planned)

```sh
# Native dev
cargo run

# Web (WASM)
./build-web.sh        # builds + assembles the web/ bundle
# then serve web/ with any static server
```

## License

© Anvesh. All rights reserved (license TBD).
