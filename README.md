# rust-retro-arcade

**▶ Play it in your browser: https://anveshg.github.io/rust-retro-arcade/**

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

## The games

**Menu** — `↑`/`↓` (or `W`/`S`) to move the cursor, `Enter`/`Space` to select, or
press a number key directly. `Esc` quits.

**Pac-Man**
- **Goal:** eat every pellet to clear the maze.
- **Move:** arrow keys or `WASD`.
- **Start:** choose a speed, `1`–`4` (1 = slow, 4 = fast).
- Avoid the two ghosts (one chases you directly, the other ambushes ahead) — each
  touch costs one of your 3 lives. Finishing faster earns a time bonus. `Esc` →
  menu.

**Pong**
- **Modes:** press `1` for vs-CPU, `2` for two players.
- **Move:** left paddle `W`/`S`, right paddle `↑`/`↓` (the CPU drives the right
  paddle in 1-player). The bounce angle depends on where the ball hits the paddle.
- First to **7** points wins. `Esc` → menu.

High scores (best Pac-Man score, total Pong wins) persist between sessions — in a
local `highscores.txt` on desktop, and in the browser's `localStorage` on the web.

## Learning Rust from this code

This project doubles as a **Rust teaching example** for programmers coming from
another language. Every source file carries explanatory doc comments.

- **Guided tour:** [`docs/RUST-TOUR.md`](docs/RUST-TOUR.md) — a reading order and a
  concept index (each Rust feature → where to see it live).
- **Extend it:** [`docs/ADDING-A-SCREEN.md`](docs/ADDING-A-SCREEN.md) — a hands-on
  walkthrough that teaches the trait-based architecture by adding a new screen.
- **Browse the API docs:** `cargo doc --open`.

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
