# Rust Arcade — Design Doc

**Date:** 2026-06-29
**Status:** Approved (design); ready for implementation planning

## 1. Summary

Refactor a legacy DOS Turbo/Borland C++ game collection (Pac-Man + Pong, with a
menu, demo, instructions, and credits screens) into a single, modern Rust
application that runs **in the browser (WebAssembly) first** and natively as a
bonus. We are **modernizing freely** — using the originals as inspiration rather
than reproducing their exact behavior.

The original sources used `graphics.h` (BGI), `conio.h`, `dos.h`, direct
keyboard-port reads (`inp(0x60)`), hardware interrupt handlers, and PC-speaker
`sound()`. None of that is portable; all of it is being replaced.

## 2. Goals

- A web-first Rust arcade with a main menu launching **Pac-Man** and **Pong**,
  plus **Instructions** and **Credits** screens (modern equivalent of the
  original `COVER.CPP`).
- Clean, idiomatic Rust with game **logic separated from rendering** so the logic
  is unit-testable.
- One codebase that targets both `wasm32-unknown-unknown` (primary) and native
  desktop (`cargo run`, for fast iteration).
- In-scope extras: **sound effects**, **high-score persistence**, **Pac-Man
  speed/difficulty levels**, and a **2-player Pong** mode.

## 3. Non-Goals (v1 — YAGNI)

- Pac-Man power-pellets / frightened-ghost mode, fruit bonuses, multiple maze
  layouts.
- The original "level code" cheat screen.
- The auto-playing demo (`DEMO.CPP`) — superseded by playable games.
- Bug-for-bug fidelity with the original collision math or monster AI.
- Networked/online multiplayer, mobile touch controls, level editor.

## 4. Technology Choice

**Engine: `macroquad`.** A small immediate-mode 2D library whose single codebase
compiles to both native and WASM with no code changes, with built-in 2D drawing,
keyboard input, and audio. Chosen over Bevy (too heavyweight for two simple
arcade games — larger WASM bundles, slower builds) and raw `wasm-bindgen` +
Canvas2D (no native path, more manual plumbing).

Supporting crates:
- `quad-storage` — high-score persistence: browser `localStorage` on web, a
  local file on native, behind one API.
- (Audio uses macroquad's built-in audio playing in-memory generated WAV bytes —
  no external asset files.)

## 5. Architecture

### 5.1 Screen state machine

The modern replacement for `COVER.CPP`'s menu dispatch. One trait, each screen
isolated and independently testable:

```rust
enum Transition { Goto(ScreenId), Quit }

trait Screen {
    fn update(&mut self, input: &Input, dt: f32) -> Option<Transition>;
    fn draw(&self);
}
```

A top-level `App` owns the current `Box<dyn Screen>` and swaps it when a screen
returns a `Transition`. Screens: `Menu`, `Instructions`, `Credits`, `Pacman`,
`Pong`, `GameOver`. The `Credits` screen attributes the project solely to the
author (Anvesh); prior collaborators' names from the original are removed, as
this is now a sole-authored rewrite.

### 5.2 Virtual resolution

All screens render to a fixed **640×480** retro canvas, scaled (letterboxed) to
fit the window or browser viewport. Keeps the arcade feel and makes coordinates
match across native and web.

### 5.3 Logic / render separation (core principle)

Game logic (ball/paddle/ghost/pellet math, scoring, transitions) lives in plain
structs with pure `update(dt, input)` methods that take no dependency on
macroquad. The drawing/input layer is thin and only *reads* state. This is the
key departure from the originals, where logic and `graphics.h` calls were
tangled.

### 5.4 Proposed file layout

```
Cargo.toml
index.html                 web shell
build-web.sh               assembles web/ bundle
src/
  main.rs                  macroquad entry + app loop
  app.rs                   screen state machine / transitions
  input.rs                 input snapshot abstraction
  theme.rs                 palette, fonts, canvas constants
  audio.rs                 sound-effect manager (generated WAV beeps)
  scores.rs                high-score persistence (quad-storage)
  screens/
    menu.rs  instructions.rs  credits.rs  gameover.rs
  pacman/
    mod.rs  maze.rs  ghost.rs  player.rs     (render-free logic + a draw fn)
  pong/
    mod.rs  ball.rs  paddle.rs               (render-free logic + a draw fn)
legacy-cpp/                original .CPP files, moved here for reference
docs/superpowers/specs/    this design doc and the implementation plan
```

The six original `.CPP` files are **moved** (not deleted) into `legacy-cpp/`.

## 6. Pac-Man

A proper **tile-based** Pac-Man (the natural modernization of the original's
arena-with-edge-dots layout).

- **Maze:** one hand-designed grid of walls + pellets on the 640×480 canvas for
  v1 (well-tested, rather than many layouts).
- **Player:** grid-aligned movement via arrow keys / WASD, smooth motion between
  tiles. Wall collision is a single "is the next tile a wall?" check (replacing
  the original's giant hand-written boolean conditionals).
- **Pellets:** eating all pellets clears the level. Each pellet scores points and
  plays the chomp sound.
- **Ghosts:** 2 ghosts (echoing `mons`/`mons2`) with simple, readable chase AI —
  step toward the player at junctions with slight randomness so they're not
  perfect. Touching a ghost costs a life.
- **Lives & difficulty:** start with 3 lives. Keep the original's selectable
  **speed levels (1–4)** as difficulty — higher level = faster ghosts + score
  bonus. Clearing the maze advances to a faster level.
- **HUD:** score, lives, level, and a timer; finishing faster grants a
  time-based score bonus (preserving the original's time-reward idea).

## 7. Pong

Classic two-paddle Pong on the 640×480 canvas.

- **Physics:** ball bounces off top/bottom walls and paddles; **bounce angle
  depends on hit position** on the paddle (center = straight, edges = sharp) —
  a clean recreation of the original's `d1` angle states.
- **Modes:** vs-AI (single player) and **2-player** (left = W/S, right = ↑/↓),
  chosen from a small pre-game prompt.
- **AI:** right paddle tracks the ball with a capped speed + slight reaction
  delay so it is beatable; difficulty tied to a speed setting.
- **Win condition:** first to N points (default 7), replacing the original's
  idiosyncratic lives/charge-bar mechanic.

## 8. Audio

Generate simple **square-wave beeps as in-memory WAV** at startup (chomp,
paddle-hit, death, win), played through macroquad's audio. Keeps the beepy
PC-speaker spirit of the original `sound()` and ships **zero asset files**. Note:
browsers require a first user gesture before audio can start — handled by
starting audio after the first key/click on the menu.

## 9. High Scores

`quad-storage` persists: the **best Pac-Man score** and a **cumulative Pong
wins count** (since Pong is win/lose rather than score-based). Writes to browser
`localStorage` on web and a local file on native, behind one API. Displayed on
the menu and game-over screens. Storage failure degrades gracefully (no
persistence) rather than crashing.

## 10. Build & Run

- **Native dev:** `cargo run`.
- **Web:** `build-web.sh` runs
  `cargo build --release --target wasm32-unknown-unknown`, then assembles
  `index.html` + the `.wasm` + macroquad's JS bundle into a `web/` folder.
  Serve with any static server (e.g. `basic-http-server web/` or
  `python3 -m http.server` from `web/`). Documented in the README.

## 11. Testing

`cargo test` over the render-free logic:
- Pong ball/paddle bounce math and scoring.
- Ghost step-choice at junctions.
- Pellet collision, level-clear, and win detection.
- Score and time-bonus calculation.
- Screen transitions.

Rendering and input are the thin, untested layer.

## 12. Error Handling

Game logic is infallible. Storage and audio initialization return `Result` and
degrade gracefully on failure (no high-score persistence / silent audio) instead
of crashing.

## 13. Open Questions / Future Work

- Add power-pellets + frightened ghosts, fruit, and more maze layouts to Pac-Man.
- Deploy the `web/` bundle to a static host (e.g. GitHub Pages) for a shareable
  link.
- Optional gamepad and touch controls.
