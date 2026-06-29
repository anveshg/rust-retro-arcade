# rust-retro-arcade Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a web-first Rust arcade (macroquad) with a menu launching modern Pac-Man and Pong, plus instructions and credits, with sound, high scores, and a WebAssembly build.

**Architecture:** A single `macroquad` binary. A `Screen` trait + `App` state machine routes between Menu/Instructions/Credits/Pong/Pacman/GameOver. Game *logic* (ball/paddle/ghost/pellet/score math) lives in render-free modules unit-tested with `cargo test`; the macroquad draw/input layer is thin and only reads state. Shared services (audio, high scores, last result) live in an `Rc<RefCell<Ctx>>` passed to each screen.

**Tech Stack:** Rust (edition 2021), `macroquad` 0.4 (graphics/input/audio, native + WASM), `quad-storage` 0.1 (localStorage on web / file on native).

## Global Constraints

- Rust edition **2021**; crate name **`rust-retro-arcade`**.
- Dependencies limited to **`macroquad = "0.4"`** and **`quad-storage = "0.1"`**. No other crates without updating this plan.
- Virtual canvas is **640×480**; all drawing uses these coordinates. Window is fixed 640×480 native; the browser canvas scales via CSS preserving aspect.
- Sole-authored: the **Credits screen shows only "ANVESH"**. No other names anywhere.
- Audio uses **in-memory generated WAV** (no asset files).
- Every game-logic module is **render-free** (no `macroquad::prelude` drawing/input calls) so it is unit-testable.
- Run `cargo fmt` and `cargo clippy` clean before each commit; commit messages end with `Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>`.
- Original sources are reference-only under `legacy-cpp/` and are never compiled.

---

## File Structure

```
Cargo.toml
index.html                 web shell (created in Task 15)
build-web.sh               web build script (Task 15)
src/
  main.rs                  macroquad entry, Ctx, screen factory, game loop
  theme.rs                 palette constants + center_x helper
  input.rs                 Input snapshot + dir4 helper
  app.rs                   Screen trait, ScreenId, Transition, App<F>
  scores.rs                HighScores logic + quad-storage load/save
  audio.rs                 square_wave_wav generator + Audio player
  screens/
    mod.rs                 re-exports Menu/Instructions/Credits/GameOver
    menu.rs
    instructions.rs
    credits.rs
    gameover.rs
  pong/
    mod.rs                 PongGame (Screen)
    ball.rs                Ball + reflect()
    paddle.rs              Paddle
  pacman/
    mod.rs                 PacmanGame (Screen)
    maze.rs                Maze (tiles, pellets)
    player.rs              Player (grid movement)
    ghost.rs               Ghost (chase AI)
legacy-cpp/                untouched reference sources
```

---

## Task 1: Project scaffold and window

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: a runnable macroquad binary with a 640×480 window titled `rust-retro-arcade`.

- [ ] **Step 1: Create `Cargo.toml`**

```toml
[package]
name = "rust-retro-arcade"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"
quad-storage = "0.1"
```

- [ ] **Step 2: Create `src/main.rs` with a minimal window**

```rust
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "rust-retro-arcade".to_owned(),
        window_width: 640,
        window_height: 480,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(Color::new(0.04, 0.04, 0.08, 1.0));
        draw_text("rust-retro-arcade", 180.0, 240.0, 36.0, WHITE);
        next_frame().await;
    }
}
```

- [ ] **Step 3: Verify it builds**

Run: `cargo build`
Expected: compiles successfully (first build downloads macroquad; that's fine).

- [ ] **Step 4: Verify it runs**

Run: `cargo run`
Expected: a 640×480 window opens showing "rust-retro-arcade" on a dark background. Close the window to exit.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock src/main.rs
git commit -m "feat: scaffold macroquad project with 640x480 window

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 2: theme.rs — palette and centering helper

**Files:**
- Create: `src/theme.rs`
- Modify: `src/main.rs` (add `mod theme;`)

**Interfaces:**
- Consumes: nothing.
- Produces: `theme::VIRTUAL_W: f32`, `theme::VIRTUAL_H: f32`, color consts (`BG, WALL, PELLET, PACMAN, GHOST_A, GHOST_B, TEXT, ACCENT`), and `theme::center_x(text_width: f32, area_width: f32) -> f32`.

- [ ] **Step 1: Write the failing test**

Create `src/theme.rs`:

```rust
use macroquad::prelude::Color;

pub const VIRTUAL_W: f32 = 640.0;
pub const VIRTUAL_H: f32 = 480.0;

pub const BG: Color = Color { r: 0.04, g: 0.04, b: 0.08, a: 1.0 };
pub const WALL: Color = Color { r: 0.16, g: 0.20, b: 0.85, a: 1.0 };
pub const PELLET: Color = Color { r: 1.0, g: 0.85, b: 0.40, a: 1.0 };
pub const PACMAN: Color = Color { r: 1.0, g: 0.92, b: 0.0, a: 1.0 };
pub const GHOST_A: Color = Color { r: 1.0, g: 0.25, b: 0.25, a: 1.0 };
pub const GHOST_B: Color = Color { r: 0.30, g: 0.90, b: 1.0, a: 1.0 };
pub const TEXT: Color = Color { r: 0.90, g: 0.95, b: 1.0, a: 1.0 };
pub const ACCENT: Color = Color { r: 0.20, g: 0.90, b: 0.50, a: 1.0 };

/// X coordinate to horizontally center an element of `text_width` in `area_width`.
pub fn center_x(text_width: f32, area_width: f32) -> f32 {
    (area_width - text_width) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centers_within_area() {
        assert_eq!(center_x(100.0, 640.0), 270.0);
    }

    #[test]
    fn centers_full_width_at_zero() {
        assert_eq!(center_x(640.0, 640.0), 0.0);
    }
}
```

- [ ] **Step 2: Register the module**

In `src/main.rs`, add below the `use` line:

```rust
mod theme;
```

- [ ] **Step 3: Run the tests to verify they pass**

Run: `cargo test theme`
Expected: PASS (2 tests).

- [ ] **Step 4: Commit**

```bash
git add src/theme.rs src/main.rs
git commit -m "feat: add theme palette and center_x helper

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 3: input.rs — input snapshot and dir4

**Files:**
- Create: `src/input.rs`
- Modify: `src/main.rs` (add `mod input;`)

**Interfaces:**
- Consumes: nothing.
- Produces:
  - `struct Input` with public bool fields `up, down, left, right, w, a, s, d, enter, space, escape`, `digit: Option<u8>`, `any_pressed: bool`; derives `Default, Clone, Copy`.
  - `Input::poll() -> Input` (reads macroquad).
  - `Input::dir4(&self) -> (i32, i32)` returning a grid direction in {-1,0,1}², vertical prioritized.

- [ ] **Step 1: Write the failing test**

Create `src/input.rs`:

```rust
use macroquad::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub enter: bool,
    pub space: bool,
    pub escape: bool,
    pub digit: Option<u8>,
    pub any_pressed: bool,
}

impl Input {
    pub fn poll() -> Self {
        let digit = if is_key_pressed(KeyCode::Key1) {
            Some(1)
        } else if is_key_pressed(KeyCode::Key2) {
            Some(2)
        } else if is_key_pressed(KeyCode::Key3) {
            Some(3)
        } else if is_key_pressed(KeyCode::Key4) {
            Some(4)
        } else {
            None
        };
        Input {
            up: is_key_down(KeyCode::Up),
            down: is_key_down(KeyCode::Down),
            left: is_key_down(KeyCode::Left),
            right: is_key_down(KeyCode::Right),
            w: is_key_down(KeyCode::W),
            a: is_key_down(KeyCode::A),
            s: is_key_down(KeyCode::S),
            d: is_key_down(KeyCode::D),
            enter: is_key_pressed(KeyCode::Enter),
            space: is_key_pressed(KeyCode::Space),
            escape: is_key_pressed(KeyCode::Escape),
            digit,
            any_pressed: get_last_key_pressed().is_some(),
        }
    }

    /// Grid direction from arrows OR WASD. Vertical wins when both axes are held.
    pub fn dir4(&self) -> (i32, i32) {
        let up = self.up || self.w;
        let down = self.down || self.s;
        let left = self.left || self.a;
        let right = self.right || self.d;
        if up && !down {
            return (0, -1);
        }
        if down && !up {
            return (0, 1);
        }
        if left && !right {
            return (-1, 0);
        }
        if right && !left {
            return (1, 0);
        }
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn up_arrow_maps_to_negative_y() {
        let i = Input { up: true, ..Default::default() };
        assert_eq!(i.dir4(), (0, -1));
    }

    #[test]
    fn wasd_d_maps_to_positive_x() {
        let i = Input { d: true, ..Default::default() };
        assert_eq!(i.dir4(), (1, 0));
    }

    #[test]
    fn opposite_horizontals_cancel() {
        let i = Input { left: true, right: true, ..Default::default() };
        assert_eq!(i.dir4(), (0, 0));
    }

    #[test]
    fn vertical_has_priority_over_horizontal() {
        let i = Input { up: true, right: true, ..Default::default() };
        assert_eq!(i.dir4(), (0, -1));
    }
}
```

- [ ] **Step 2: Register the module**

In `src/main.rs` add: `mod input;`

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test input`
Expected: PASS (4 tests).

- [ ] **Step 4: Commit**

```bash
git add src/input.rs src/main.rs
git commit -m "feat: add Input snapshot and dir4 grid-direction helper

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 4: app.rs — Screen trait and App state machine

**Files:**
- Create: `src/app.rs`
- Modify: `src/main.rs` (add `mod app;`)

**Interfaces:**
- Consumes: `crate::input::Input`.
- Produces:
  - `enum ScreenId { Menu, Instructions, Credits, Pong, Pacman, GameOver }` (derives `Clone, Copy, PartialEq, Eq, Debug`).
  - `enum Transition { Goto(ScreenId), Quit }` (derives `Clone, Copy, PartialEq, Eq, Debug`).
  - `trait Screen { fn update(&mut self, input: &Input, dt: f32) -> Option<Transition>; fn draw(&self); fn id(&self) -> ScreenId; }`
  - `struct App<F: Fn(ScreenId) -> Box<dyn Screen>>` with `new(make: F, start: ScreenId)`, `current_id(&self) -> ScreenId`, `update(&mut self, &Input, f32)`, `draw(&self)`, public field `quit: bool`.

- [ ] **Step 1: Write the failing test**

Create `src/app.rs`:

```rust
use crate::input::Input;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScreenId {
    Menu,
    Instructions,
    Credits,
    Pong,
    Pacman,
    GameOver,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Transition {
    Goto(ScreenId),
    Quit,
}

pub trait Screen {
    fn update(&mut self, input: &Input, dt: f32) -> Option<Transition>;
    fn draw(&self);
    fn id(&self) -> ScreenId;
}

pub struct App<F: Fn(ScreenId) -> Box<dyn Screen>> {
    make: F,
    current: Box<dyn Screen>,
    pub quit: bool,
}

impl<F: Fn(ScreenId) -> Box<dyn Screen>> App<F> {
    pub fn new(make: F, start: ScreenId) -> Self {
        let current = make(start);
        App { make, current, quit: false }
    }

    pub fn current_id(&self) -> ScreenId {
        self.current.id()
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        if let Some(t) = self.current.update(input, dt) {
            match t {
                Transition::Goto(id) => self.current = (self.make)(id),
                Transition::Quit => self.quit = true,
            }
        }
    }

    pub fn draw(&self) {
        self.current.draw();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fake {
        id: ScreenId,
        emit: Option<Transition>,
    }
    impl Screen for Fake {
        fn update(&mut self, _i: &Input, _dt: f32) -> Option<Transition> {
            self.emit.take()
        }
        fn draw(&self) {}
        fn id(&self) -> ScreenId {
            self.id
        }
    }

    fn make(id: ScreenId) -> Box<dyn Screen> {
        // Menu emits a Goto(Pong); every other screen is inert.
        let emit = match id {
            ScreenId::Menu => Some(Transition::Goto(ScreenId::Pong)),
            _ => None,
        };
        Box::new(Fake { id, emit })
    }

    #[test]
    fn starts_on_requested_screen() {
        let app = App::new(make, ScreenId::Menu);
        assert_eq!(app.current_id(), ScreenId::Menu);
    }

    #[test]
    fn goto_transition_swaps_screen() {
        let mut app = App::new(make, ScreenId::Menu);
        app.update(&Input::default(), 0.0);
        assert_eq!(app.current_id(), ScreenId::Pong);
        assert!(!app.quit);
    }

    #[test]
    fn quit_transition_sets_flag() {
        fn make_quit(id: ScreenId) -> Box<dyn Screen> {
            Box::new(Fake { id, emit: Some(Transition::Quit) })
        }
        let mut app = App::new(make_quit, ScreenId::Menu);
        app.update(&Input::default(), 0.0);
        assert!(app.quit);
    }
}
```

- [ ] **Step 2: Register the module**

In `src/main.rs` add: `mod app;`

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test app`
Expected: PASS (3 tests).

- [ ] **Step 4: Commit**

```bash
git add src/app.rs src/main.rs
git commit -m "feat: add Screen trait and App state machine

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 5: scores.rs — high-score logic and persistence

**Files:**
- Create: `src/scores.rs`
- Modify: `src/main.rs` (add `mod scores;`)

**Interfaces:**
- Consumes: nothing (logic); `quad_storage` (persistence).
- Produces:
  - `struct HighScores { pub best_pacman: u32, pub pong_wins: u32 }` (derives `Clone, Copy, Default, PartialEq, Eq, Debug`).
  - `HighScores::record_pacman(&mut self, score: u32) -> bool` (true if new best).
  - `HighScores::record_pong_win(&mut self)`.
  - `scores::load() -> HighScores`, `scores::save(h: &HighScores)`.

- [ ] **Step 1: Write the failing test**

Create `src/scores.rs`:

```rust
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct HighScores {
    pub best_pacman: u32,
    pub pong_wins: u32,
}

impl HighScores {
    /// Records a Pac-Man score. Returns true if it beat the previous best.
    pub fn record_pacman(&mut self, score: u32) -> bool {
        if score > self.best_pacman {
            self.best_pacman = score;
            true
        } else {
            false
        }
    }

    pub fn record_pong_win(&mut self) {
        self.pong_wins += 1;
    }
}

pub fn load() -> HighScores {
    let storage = quad_storage::STORAGE.lock().unwrap();
    let best_pacman = storage
        .get("best_pacman")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let pong_wins = storage
        .get("pong_wins")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    HighScores { best_pacman, pong_wins }
}

pub fn save(h: &HighScores) {
    let mut storage = quad_storage::STORAGE.lock().unwrap();
    storage.set("best_pacman", &h.best_pacman.to_string());
    storage.set("pong_wins", &h.pong_wins.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_score_is_a_new_best() {
        let mut h = HighScores::default();
        assert!(h.record_pacman(120));
        assert_eq!(h.best_pacman, 120);
    }

    #[test]
    fn lower_score_does_not_replace_best() {
        let mut h = HighScores { best_pacman: 200, pong_wins: 0 };
        assert!(!h.record_pacman(150));
        assert_eq!(h.best_pacman, 200);
    }

    #[test]
    fn pong_wins_accumulate() {
        let mut h = HighScores::default();
        h.record_pong_win();
        h.record_pong_win();
        assert_eq!(h.pong_wins, 2);
    }
}
```

- [ ] **Step 2: Register the module**

In `src/main.rs` add: `mod scores;`

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test scores`
Expected: PASS (3 tests). (`load`/`save` are not unit-tested — they require the macroquad runtime; they are exercised manually in Task 15's in-browser check.)

- [ ] **Step 4: Commit**

```bash
git add src/scores.rs src/main.rs
git commit -m "feat: add HighScores logic and quad-storage persistence

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 6: audio.rs — generated WAV beeps and player

**Files:**
- Create: `src/audio.rs`
- Modify: `src/main.rs` (add `mod audio;`)

**Interfaces:**
- Consumes: nothing (generation); `macroquad::audio` (playback).
- Produces:
  - `audio::square_wave_wav(freq_hz: f32, ms: u32, volume: f32) -> Vec<u8>` (valid 16-bit mono 44.1 kHz WAV).
  - `enum Sfx { Chomp, Bounce, Score, Death, Win, Select }` (derives `Clone, Copy`).
  - `struct Audio` with `async fn load() -> Audio` and `fn play(&self, sfx: Sfx)`.

- [ ] **Step 1: Write the failing test (WAV generator)**

Create `src/audio.rs`:

```rust
use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};

const SAMPLE_RATE: u32 = 44_100;

/// Generate a mono 16-bit PCM WAV containing a decaying square wave.
pub fn square_wave_wav(freq_hz: f32, ms: u32, volume: f32) -> Vec<u8> {
    let num_samples = (SAMPLE_RATE as u64 * ms as u64 / 1000) as u32;
    let data_len = num_samples * 2; // 16-bit mono => 2 bytes/sample
    let mut out = Vec::with_capacity(44 + data_len as usize);

    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes()); // fmt chunk size
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&1u16.to_le_bytes()); // mono
    out.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    out.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes()); // byte rate
    out.extend_from_slice(&2u16.to_le_bytes()); // block align
    out.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());

    let amp = volume.clamp(0.0, 1.0) * i16::MAX as f32;
    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let phase = (freq_hz * t).fract();
        let square = if phase < 0.5 { 1.0 } else { -1.0 };
        let env = 1.0 - (i as f32 / num_samples.max(1) as f32); // linear decay
        let sample = (square * amp * env) as i16;
        out.extend_from_slice(&sample.to_le_bytes());
    }
    out
}

#[derive(Clone, Copy)]
pub enum Sfx {
    Chomp,
    Bounce,
    Score,
    Death,
    Win,
    Select,
}

pub struct Audio {
    chomp: Sound,
    bounce: Sound,
    score: Sound,
    death: Sound,
    win: Sound,
    select: Sound,
}

impl Audio {
    pub async fn load() -> Self {
        async fn beep(freq: f32, ms: u32) -> Sound {
            load_sound_from_bytes(&square_wave_wav(freq, ms, 0.3))
                .await
                .expect("generated WAV must decode")
        }
        Audio {
            chomp: beep(660.0, 40).await,
            bounce: beep(440.0, 60).await,
            score: beep(880.0, 120).await,
            death: beep(140.0, 400).await,
            win: beep(990.0, 300).await,
            select: beep(550.0, 50).await,
        }
    }

    pub fn play(&self, sfx: Sfx) {
        let snd = match sfx {
            Sfx::Chomp => &self.chomp,
            Sfx::Bounce => &self.bounce,
            Sfx::Score => &self.score,
            Sfx::Death => &self.death,
            Sfx::Win => &self.win,
            Sfx::Select => &self.select,
        };
        play_sound_once(snd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wav_has_riff_and_wave_magic() {
        let w = square_wave_wav(440.0, 100, 0.3);
        assert_eq!(&w[0..4], b"RIFF");
        assert_eq!(&w[8..12], b"WAVE");
    }

    #[test]
    fn wav_length_matches_samples() {
        let ms = 100;
        let num_samples = SAMPLE_RATE * ms / 1000;
        let w = square_wave_wav(440.0, ms, 0.3);
        assert_eq!(w.len(), 44 + (num_samples * 2) as usize);
    }

    #[test]
    fn wav_data_chunk_size_is_correct() {
        let ms = 50;
        let num_samples = SAMPLE_RATE * ms / 1000;
        let w = square_wave_wav(440.0, ms, 0.3);
        let data_len = u32::from_le_bytes([w[40], w[41], w[42], w[43]]);
        assert_eq!(data_len, num_samples * 2);
    }
}
```

- [ ] **Step 2: Register the module**

In `src/main.rs` add: `mod audio;`

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test audio`
Expected: PASS (3 tests).

- [ ] **Step 4: Commit**

```bash
git add src/audio.rs src/main.rs
git commit -m "feat: add generated-WAV sound effects and Audio player

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 7: Shared Ctx, screens (menu/instructions/credits/gameover), and main wiring

**Files:**
- Create: `src/screens/mod.rs`
- Create: `src/screens/menu.rs`
- Create: `src/screens/instructions.rs`
- Create: `src/screens/credits.rs`
- Create: `src/screens/gameover.rs`
- Modify: `src/main.rs` (Ctx, factory, game loop, module decls)

**Interfaces:**
- Consumes: `crate::app::{Screen, ScreenId, Transition}`, `crate::input::Input`, `crate::theme`, `crate::audio::Audio`, `crate::scores::HighScores`.
- Produces (in `main.rs`):
  - `struct GameResult { pub title: String, pub score: u32, pub subtitle: String }`
  - `struct Ctx { pub audio: Audio, pub scores: HighScores, pub last_result: Option<GameResult> }`
  - `type SharedCtx = std::rc::Rc<std::cell::RefCell<Ctx>>`
  - `fn make_screen(id: ScreenId, ctx: SharedCtx) -> Box<dyn Screen>`
- Produces (screens): `Menu::new(ctx: SharedCtx)`, `Instructions::new()`, `Credits::new()`, `GameOver::new(ctx: SharedCtx)`, all `impl Screen`.
- Produces (menu logic): `screens::menu::menu_target(index: usize) -> Transition`.

- [ ] **Step 1: Write the failing test for menu mapping**

Create `src/screens/menu.rs`:

```rust
use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use crate::SharedCtx;
use macroquad::prelude::*;

const ITEMS: [&str; 5] = ["Pac-Man", "Pong", "Instructions", "Credits", "Quit"];

/// Maps a 0-based menu index to its transition.
pub fn menu_target(index: usize) -> Transition {
    match index {
        0 => Transition::Goto(ScreenId::Pacman),
        1 => Transition::Goto(ScreenId::Pong),
        2 => Transition::Goto(ScreenId::Instructions),
        3 => Transition::Goto(ScreenId::Credits),
        _ => Transition::Quit,
    }
}

pub struct Menu {
    ctx: SharedCtx,
    selected: usize,
}

impl Menu {
    pub fn new(ctx: SharedCtx) -> Self {
        Menu { ctx, selected: 0 }
    }
}

impl Screen for Menu {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.up || input.w {
            self.selected = (self.selected + ITEMS.len() - 1) % ITEMS.len();
        }
        if input.down || input.s {
            self.selected = (self.selected + 1) % ITEMS.len();
        }
        if let Some(d) = input.digit {
            let idx = (d as usize).saturating_sub(1);
            if idx < ITEMS.len() {
                return Some(menu_target(idx));
            }
        }
        if input.escape {
            return Some(Transition::Quit);
        }
        if input.enter || input.space {
            self.ctx.borrow().audio.play(crate::audio::Sfx::Select);
            return Some(menu_target(self.selected));
        }
        None
    }

    fn draw(&self) {
        draw_text("RUST RETRO ARCADE", 110.0, 80.0, 44.0, theme::ACCENT);
        for (i, label) in ITEMS.iter().enumerate() {
            let y = 170.0 + i as f32 * 46.0;
            let color = if i == self.selected { theme::PACMAN } else { theme::TEXT };
            let prefix = if i == self.selected { "> " } else { "  " };
            draw_text(&format!("{}{}. {}", prefix, i + 1, label), 220.0, y, 32.0, color);
        }
        let s = self.ctx.borrow().scores;
        draw_text(
            &format!("Best Pac-Man: {}    Pong wins: {}", s.best_pacman, s.pong_wins),
            120.0,
            440.0,
            22.0,
            theme::TEXT,
        );
    }

    fn id(&self) -> ScreenId {
        ScreenId::Menu
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_zero_starts_pacman() {
        assert_eq!(menu_target(0), Transition::Goto(ScreenId::Pacman));
    }

    #[test]
    fn index_one_starts_pong() {
        assert_eq!(menu_target(1), Transition::Goto(ScreenId::Pong));
    }

    #[test]
    fn last_index_quits() {
        assert_eq!(menu_target(4), Transition::Quit);
    }
}
```

> Note: "Quit" (the 5th item) is reached by selecting it and pressing Enter, or by pressing Escape. There is no digit-5 handling because `Input::digit` only ranges 1–4.

- [ ] **Step 2: Create the static screens**

Create `src/screens/instructions.rs`:

```rust
use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use macroquad::prelude::*;

#[derive(Default)]
pub struct Instructions;

impl Instructions {
    pub fn new() -> Self {
        Instructions
    }
}

impl Screen for Instructions {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.any_pressed || input.escape || input.enter {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }

    fn draw(&self) {
        draw_text("INSTRUCTIONS", 200.0, 70.0, 40.0, theme::ACCENT);
        let lines = [
            "Pac-Man: arrow keys or WASD to move.",
            "Eat every pellet to clear the maze.",
            "Avoid the ghosts. Finishing faster scores more.",
            "Choose speed 1-4 at the start.",
            "",
            "Pong: left paddle W/S, right paddle Up/Down.",
            "1 = vs CPU, 2 = two players. First to 7 wins.",
            "",
            "Press any key to return to the menu.",
        ];
        for (i, l) in lines.iter().enumerate() {
            draw_text(l, 60.0, 140.0 + i as f32 * 32.0, 24.0, theme::TEXT);
        }
    }

    fn id(&self) -> ScreenId {
        ScreenId::Instructions
    }
}
```

Create `src/screens/credits.rs`:

```rust
use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use macroquad::prelude::*;

#[derive(Default)]
pub struct Credits;

impl Credits {
    pub fn new() -> Self {
        Credits
    }
}

impl Screen for Credits {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.any_pressed || input.escape || input.enter {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }

    fn draw(&self) {
        draw_text("CREDITS", 220.0, 120.0, 48.0, theme::ACCENT);
        draw_text("ANVESH", 240.0, 230.0, 56.0, theme::PACMAN);
        draw_text("Rust rewrite of an old DOS C++ project", 110.0, 320.0, 24.0, theme::TEXT);
        draw_text("Press any key to return", 200.0, 420.0, 22.0, theme::TEXT);
    }

    fn id(&self) -> ScreenId {
        ScreenId::Credits
    }
}
```

Create `src/screens/gameover.rs`:

```rust
use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use crate::SharedCtx;
use macroquad::prelude::*;

pub struct GameOver {
    ctx: SharedCtx,
}

impl GameOver {
    pub fn new(ctx: SharedCtx) -> Self {
        GameOver { ctx }
    }
}

impl Screen for GameOver {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.any_pressed || input.escape || input.enter {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }

    fn draw(&self) {
        let ctx = self.ctx.borrow();
        let (title, score, subtitle) = match &ctx.last_result {
            Some(r) => (r.title.as_str(), r.score, r.subtitle.as_str()),
            None => ("GAME OVER", 0, ""),
        };
        draw_text(title, 180.0, 180.0, 48.0, theme::ACCENT);
        draw_text(&format!("Score: {}", score), 230.0, 260.0, 36.0, theme::PACMAN);
        draw_text(subtitle, 150.0, 320.0, 26.0, theme::TEXT);
        draw_text("Press any key for the menu", 180.0, 420.0, 22.0, theme::TEXT);
    }

    fn id(&self) -> ScreenId {
        ScreenId::GameOver
    }
}
```

- [ ] **Step 3: Create `src/screens/mod.rs`**

```rust
pub mod credits;
pub mod gameover;
pub mod instructions;
pub mod menu;

pub use credits::Credits;
pub use gameover::GameOver;
pub use instructions::Instructions;
pub use menu::Menu;
```

- [ ] **Step 4: Rewrite `src/main.rs` to wire everything**

Replace the entire contents of `src/main.rs` with:

```rust
use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod app;
mod audio;
mod input;
mod pacman;
mod pong;
mod scores;
mod screens;
mod theme;

use app::{App, Screen, ScreenId};
use input::Input;

pub struct GameResult {
    pub title: String,
    pub score: u32,
    pub subtitle: String,
}

pub struct Ctx {
    pub audio: audio::Audio,
    pub scores: scores::HighScores,
    pub last_result: Option<GameResult>,
}

pub type SharedCtx = Rc<RefCell<Ctx>>;

fn make_screen(id: ScreenId, ctx: SharedCtx) -> Box<dyn Screen> {
    match id {
        ScreenId::Menu => Box::new(screens::Menu::new(ctx)),
        ScreenId::Instructions => Box::new(screens::Instructions::new()),
        ScreenId::Credits => Box::new(screens::Credits::new()),
        ScreenId::Pong => Box::new(pong::PongGame::new(ctx)),
        ScreenId::Pacman => Box::new(pacman::PacmanGame::new(ctx)),
        ScreenId::GameOver => Box::new(screens::GameOver::new(ctx)),
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "rust-retro-arcade".to_owned(),
        window_width: 640,
        window_height: 480,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let audio = audio::Audio::load().await;
    let scores = scores::load();
    let ctx: SharedCtx = Rc::new(RefCell::new(Ctx {
        audio,
        scores,
        last_result: None,
    }));

    let factory_ctx = ctx.clone();
    let mut app = App::new(move |id| make_screen(id, factory_ctx.clone()), ScreenId::Menu);

    loop {
        let input = Input::poll();
        let dt = get_frame_time();
        app.update(&input, dt);
        if app.quit {
            break;
        }
        clear_background(theme::BG);
        app.draw();
        next_frame().await;
    }

    scores::save(&ctx.borrow().scores);
}
```

> This references `pong::PongGame` and `pacman::PacmanGame`, created in later tasks. To keep this task compiling and runnable on its own, create temporary stub modules in this step:

Create `src/pong/mod.rs` (temporary stub — replaced in Task 10):

```rust
use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use crate::SharedCtx;
use macroquad::prelude::*;

pub struct PongGame {
    _ctx: SharedCtx,
}
impl PongGame {
    pub fn new(ctx: SharedCtx) -> Self {
        PongGame { _ctx: ctx }
    }
}
impl Screen for PongGame {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.escape {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }
    fn draw(&self) {
        draw_text("PONG (coming soon)", 170.0, 240.0, 36.0, theme::TEXT);
        draw_text("Esc = menu", 250.0, 300.0, 24.0, theme::TEXT);
    }
    fn id(&self) -> ScreenId {
        ScreenId::Pong
    }
}
```

Create `src/pacman/mod.rs` (temporary stub — replaced in Task 14):

```rust
use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use crate::SharedCtx;
use macroquad::prelude::*;

pub struct PacmanGame {
    _ctx: SharedCtx,
}
impl PacmanGame {
    pub fn new(ctx: SharedCtx) -> Self {
        PacmanGame { _ctx: ctx }
    }
}
impl Screen for PacmanGame {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.escape {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }
    fn draw(&self) {
        draw_text("PAC-MAN (coming soon)", 150.0, 240.0, 36.0, theme::TEXT);
        draw_text("Esc = menu", 250.0, 300.0, 24.0, theme::TEXT);
    }
    fn id(&self) -> ScreenId {
        ScreenId::Pacman
    }
}
```

- [ ] **Step 5: Run tests**

Run: `cargo test`
Expected: PASS (all prior tests + 3 menu tests).

- [ ] **Step 6: Verify it runs**

Run: `cargo run`
Expected: menu appears with 5 items and a high-score line; ↑/↓ (or W/S) moves the `>` cursor; Enter or number keys open Instructions/Credits (and the Pong/Pac-Man "coming soon" stubs); Escape from a screen returns to the menu; Escape on the menu quits.

- [ ] **Step 7: Commit**

```bash
git add src/screens src/pong/mod.rs src/pacman/mod.rs src/main.rs
git commit -m "feat: menu/instructions/credits/gameover screens with shared Ctx wiring

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 8: pong/ball.rs — ball physics and reflect

**Files:**
- Create: `src/pong/ball.rs`
- Modify: `src/pong/mod.rs` (add `pub mod ball;` at the top of the stub)

**Interfaces:**
- Consumes: `macroquad::math::Vec2`.
- Produces:
  - `struct Ball { pub pos: Vec2, pub vel: Vec2, pub radius: f32 }`
  - `Ball::new(cx: f32, cy: f32, vx: f32, vy: f32) -> Ball` (radius 8.0).
  - `Ball::step(&mut self, dt: f32)`.
  - `Ball::bounce_walls(&mut self, top: f32, bottom: f32) -> bool`.
  - `pong::ball::reflect(speed: f32, offset: f32, dir: f32) -> Vec2` (offset in [-1,1], dir ±1).

- [ ] **Step 1: Write the failing test**

Create `src/pong/ball.rs`:

```rust
use macroquad::math::Vec2;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
}

impl Ball {
    pub fn new(cx: f32, cy: f32, vx: f32, vy: f32) -> Self {
        Ball {
            pos: Vec2::new(cx, cy),
            vel: Vec2::new(vx, vy),
            radius: 8.0,
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }

    /// Reflect off the top/bottom walls. Returns true if a wall was hit.
    pub fn bounce_walls(&mut self, top: f32, bottom: f32) -> bool {
        if self.pos.y - self.radius <= top && self.vel.y < 0.0 {
            self.pos.y = top + self.radius;
            self.vel.y = -self.vel.y;
            return true;
        }
        if self.pos.y + self.radius >= bottom && self.vel.y > 0.0 {
            self.pos.y = bottom - self.radius;
            self.vel.y = -self.vel.y;
            return true;
        }
        false
    }
}

/// New velocity after a paddle hit.
/// `offset` in [-1,1] (paddle-relative hit position), `dir` is +1 (rightward) or -1 (leftward).
pub fn reflect(speed: f32, offset: f32, dir: f32) -> Vec2 {
    let max_angle = std::f32::consts::FRAC_PI_3; // 60 degrees
    let angle = offset.clamp(-1.0, 1.0) * max_angle;
    Vec2::new(dir * speed * angle.cos(), speed * angle.sin())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_advances_by_velocity_times_dt() {
        let mut b = Ball::new(100.0, 100.0, 50.0, 0.0);
        b.step(1.0);
        assert_eq!(b.pos.x, 150.0);
        assert_eq!(b.pos.y, 100.0);
    }

    #[test]
    fn bounces_off_top_wall() {
        let mut b = Ball::new(100.0, 2.0, 0.0, -30.0);
        let hit = b.bounce_walls(0.0, 480.0);
        assert!(hit);
        assert!(b.vel.y > 0.0);
        assert_eq!(b.pos.y, b.radius);
    }

    #[test]
    fn no_bounce_when_moving_away_from_wall() {
        let mut b = Ball::new(100.0, 2.0, 0.0, 30.0);
        assert!(!b.bounce_walls(0.0, 480.0));
    }

    #[test]
    fn reflect_center_goes_straight() {
        let v = reflect(300.0, 0.0, 1.0);
        assert!((v.x - 300.0).abs() < 0.001);
        assert!(v.y.abs() < 0.001);
    }

    #[test]
    fn reflect_direction_sign_is_respected() {
        let v = reflect(300.0, 0.0, -1.0);
        assert!(v.x < 0.0);
    }

    #[test]
    fn reflect_edge_hit_adds_vertical_speed() {
        let v = reflect(300.0, 1.0, 1.0);
        assert!(v.y > 0.0);
        assert!(v.x > 0.0);
    }
}
```

- [ ] **Step 2: Register the submodule**

At the top of `src/pong/mod.rs` (the stub), add:

```rust
pub mod ball;
```

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test pong::ball`
Expected: PASS (6 tests).

- [ ] **Step 4: Commit**

```bash
git add src/pong/ball.rs src/pong/mod.rs
git commit -m "feat: pong ball physics and paddle reflect

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 9: pong/paddle.rs — paddle movement, collision, AI

**Files:**
- Create: `src/pong/paddle.rs`
- Modify: `src/pong/mod.rs` (add `pub mod paddle;`)

**Interfaces:**
- Consumes: `crate::pong::ball::Ball`, `macroquad::math::Vec2`.
- Produces:
  - `struct Paddle { pub x: f32, pub y: f32, pub w: f32, pub h: f32, pub speed: f32 }`
  - `Paddle::new(x: f32, y: f32) -> Paddle` (w 12.0, h 80.0, speed 320.0).
  - `Paddle::center_y(&self) -> f32`.
  - `Paddle::move_by(&mut self, dy: f32, top: f32, bottom: f32)`.
  - `Paddle::track(&mut self, target_y: f32, dt: f32, top: f32, bottom: f32)` (AI; capped by speed*dt).
  - `Paddle::hits(&self, ball: &Ball) -> bool`.
  - `Paddle::bounce_offset(&self, ball: &Ball) -> f32` (in [-1,1]).

- [ ] **Step 1: Write the failing test**

Create `src/pong/paddle.rs`:

```rust
use crate::pong::ball::Ball;

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub speed: f32,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        Paddle {
            x,
            y,
            w: 12.0,
            h: 80.0,
            speed: 320.0,
        }
    }

    pub fn center_y(&self) -> f32 {
        self.y + self.h / 2.0
    }

    pub fn move_by(&mut self, dy: f32, top: f32, bottom: f32) {
        self.y = (self.y + dy).clamp(top, bottom - self.h);
    }

    /// Move toward `target_y`, capped by `speed * dt`, clamped to the field.
    pub fn track(&mut self, target_y: f32, dt: f32, top: f32, bottom: f32) {
        let delta = target_y - self.center_y();
        let max = self.speed * dt;
        let step = delta.clamp(-max, max);
        self.y = (self.y + step).clamp(top, bottom - self.h);
    }

    pub fn hits(&self, ball: &Ball) -> bool {
        ball.pos.x - ball.radius <= self.x + self.w
            && ball.pos.x + ball.radius >= self.x
            && ball.pos.y >= self.y
            && ball.pos.y <= self.y + self.h
    }

    /// Hit position relative to paddle center, normalized to [-1, 1].
    pub fn bounce_offset(&self, ball: &Ball) -> f32 {
        ((ball.pos.y - self.center_y()) / (self.h / 2.0)).clamp(-1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_by_clamps_to_field() {
        let mut p = Paddle::new(10.0, 10.0);
        p.move_by(-100.0, 0.0, 480.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn track_is_capped_by_speed() {
        let mut p = Paddle::new(10.0, 200.0);
        // target far above; dt small so movement is capped to speed*dt = 32.
        p.track(0.0, 0.1, 0.0, 480.0);
        assert_eq!(p.y, 200.0 - 32.0);
    }

    #[test]
    fn bounce_offset_is_zero_at_center() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(20.0, p.center_y(), 0.0, 0.0);
        assert!(p.bounce_offset(&ball).abs() < 0.001);
    }

    #[test]
    fn bounce_offset_is_negative_above_center() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(20.0, p.y, 0.0, 0.0); // top edge
        assert!(p.bounce_offset(&ball) < 0.0);
    }

    #[test]
    fn hits_detects_overlap() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(18.0, 140.0, 0.0, 0.0);
        assert!(p.hits(&ball));
    }

    #[test]
    fn misses_when_ball_is_past_paddle_vertically() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(18.0, 300.0, 0.0, 0.0);
        assert!(!p.hits(&ball));
    }
}
```

- [ ] **Step 2: Register the submodule**

In `src/pong/mod.rs` add: `pub mod paddle;`

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test pong::paddle`
Expected: PASS (6 tests).

- [ ] **Step 4: Commit**

```bash
git add src/pong/paddle.rs src/pong/mod.rs
git commit -m "feat: pong paddle movement, collision, and AI tracking

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 10: pong/mod.rs — PongGame screen

**Files:**
- Modify: `src/pong/mod.rs` (replace the stub body, keep `pub mod ball;` / `pub mod paddle;`)

**Interfaces:**
- Consumes: `crate::pong::ball::{Ball, reflect}`, `crate::pong::paddle::Paddle`, `crate::app::{Screen, ScreenId, Transition}`, `crate::input::Input`, `crate::audio::Sfx`, `crate::theme`, `crate::{SharedCtx, GameResult}`.
- Produces: `PongGame::new(ctx: SharedCtx) -> PongGame` (impl `Screen`); `pong::serve_velocity(dir: f32, speed: f32) -> macroquad::math::Vec2` (testable serve helper).

- [ ] **Step 1: Write the failing test for the serve helper**

Replace the entire contents of `src/pong/mod.rs` with:

```rust
pub mod ball;
pub mod paddle;

use crate::app::{Screen, ScreenId, Transition};
use crate::audio::Sfx;
use crate::input::Input;
use crate::theme;
use crate::{GameResult, SharedCtx};
use ball::{reflect, Ball};
use macroquad::math::Vec2;
use macroquad::prelude::*;
use paddle::Paddle;

const TOP: f32 = 30.0;
const BOTTOM: f32 = 480.0;
const TARGET: u32 = 7;
const BALL_SPEED: f32 = 300.0;

/// Initial ball velocity for a serve toward `dir` (+1 right, -1 left).
pub fn serve_velocity(dir: f32, speed: f32) -> Vec2 {
    Vec2::new(dir * speed, 0.0)
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    ModeSelect,
    Playing,
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    VsCpu,
    TwoPlayer,
}

pub struct PongGame {
    ctx: SharedCtx,
    phase: Phase,
    mode: Mode,
    ball: Ball,
    left: Paddle,
    right: Paddle,
    score_l: u32,
    score_r: u32,
    serve_dir: f32,
}

impl PongGame {
    pub fn new(ctx: SharedCtx) -> Self {
        PongGame {
            ctx,
            phase: Phase::ModeSelect,
            mode: Mode::VsCpu,
            ball: Ball::new(320.0, 240.0, BALL_SPEED, 0.0),
            left: Paddle::new(20.0, 200.0),
            right: Paddle::new(608.0, 200.0),
            score_l: 0,
            score_r: 0,
            serve_dir: 1.0,
        }
    }

    fn reset_ball(&mut self, dir: f32) {
        let v = serve_velocity(dir, BALL_SPEED);
        self.ball = Ball::new(320.0, 240.0, v.x, v.y);
    }
}

impl Screen for PongGame {
    fn update(&mut self, input: &Input, dt: f32) -> Option<Transition> {
        if input.escape {
            return Some(Transition::Goto(ScreenId::Menu));
        }

        if self.phase == Phase::ModeSelect {
            if input.digit == Some(1) {
                self.mode = Mode::VsCpu;
                self.phase = Phase::Playing;
            } else if input.digit == Some(2) {
                self.mode = Mode::TwoPlayer;
                self.phase = Phase::Playing;
            }
            return None;
        }

        // Left paddle: W/S.
        if input.w {
            self.left.move_by(-self.left.speed * dt, TOP, BOTTOM);
        }
        if input.s {
            self.left.move_by(self.left.speed * dt, TOP, BOTTOM);
        }

        // Right paddle: human (Up/Down) or AI.
        match self.mode {
            Mode::TwoPlayer => {
                if input.up {
                    self.right.move_by(-self.right.speed * dt, TOP, BOTTOM);
                }
                if input.down {
                    self.right.move_by(self.right.speed * dt, TOP, BOTTOM);
                }
            }
            Mode::VsCpu => {
                self.right.track(self.ball.pos.y, dt, TOP, BOTTOM);
            }
        }

        self.ball.step(dt);
        if self.ball.bounce_walls(TOP, BOTTOM) {
            self.ctx.borrow().audio.play(Sfx::Bounce);
        }

        // Paddle collisions.
        if self.ball.vel.x < 0.0 && self.left.hits(&self.ball) {
            let off = self.left.bounce_offset(&self.ball);
            self.ball.vel = reflect(BALL_SPEED, off, 1.0);
            self.ball.pos.x = self.left.x + self.left.w + self.ball.radius;
            self.ctx.borrow().audio.play(Sfx::Bounce);
        }
        if self.ball.vel.x > 0.0 && self.right.hits(&self.ball) {
            let off = self.right.bounce_offset(&self.ball);
            self.ball.vel = reflect(BALL_SPEED, off, -1.0);
            self.ball.pos.x = self.right.x - self.ball.radius;
            self.ctx.borrow().audio.play(Sfx::Bounce);
        }

        // Scoring.
        if self.ball.pos.x < 0.0 {
            self.score_r += 1;
            self.ctx.borrow().audio.play(Sfx::Score);
            self.serve_dir = -1.0;
            self.reset_ball(self.serve_dir);
        } else if self.ball.pos.x > 640.0 {
            self.score_l += 1;
            self.ctx.borrow().audio.play(Sfx::Score);
            self.serve_dir = 1.0;
            self.reset_ball(self.serve_dir);
        }

        // Win check.
        if self.score_l >= TARGET || self.score_r >= TARGET {
            let player_won = self.score_l >= TARGET;
            self.ctx.borrow().audio.play(if player_won { Sfx::Win } else { Sfx::Death });
            let title = if self.mode == Mode::TwoPlayer {
                if player_won { "LEFT PLAYER WINS" } else { "RIGHT PLAYER WINS" }
            } else if player_won {
                "YOU WIN"
            } else {
                "CPU WINS"
            };
            if self.mode == Mode::VsCpu && player_won {
                self.ctx.borrow_mut().scores.record_pong_win();
            }
            self.ctx.borrow_mut().last_result = Some(GameResult {
                title: title.to_string(),
                score: self.score_l.max(self.score_r),
                subtitle: format!("{} - {}", self.score_l, self.score_r),
            });
            return Some(Transition::Goto(ScreenId::GameOver));
        }

        None
    }

    fn draw(&self) {
        if self.phase == Phase::ModeSelect {
            draw_text("PONG", 270.0, 150.0, 56.0, theme::ACCENT);
            draw_text("Press 1 for vs CPU", 210.0, 250.0, 30.0, theme::TEXT);
            draw_text("Press 2 for two players", 195.0, 300.0, 30.0, theme::TEXT);
            draw_text("Esc = menu", 260.0, 380.0, 22.0, theme::TEXT);
            return;
        }
        // Net.
        let mut y = TOP;
        while y < BOTTOM {
            draw_rectangle(318.0, y, 4.0, 16.0, theme::TEXT);
            y += 28.0;
        }
        draw_rectangle(self.left.x, self.left.y, self.left.w, self.left.h, theme::PACMAN);
        draw_rectangle(self.right.x, self.right.y, self.right.w, self.right.h, theme::GHOST_B);
        draw_circle(self.ball.pos.x, self.ball.pos.y, self.ball.radius, theme::TEXT);
        draw_text(&format!("{}", self.score_l), 270.0, 24.0, 30.0, theme::PACMAN);
        draw_text(&format!("{}", self.score_r), 360.0, 24.0, 30.0, theme::GHOST_B);
    }

    fn id(&self) -> ScreenId {
        ScreenId::Pong
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serve_goes_right_for_positive_dir() {
        let v = serve_velocity(1.0, 300.0);
        assert_eq!(v.x, 300.0);
        assert_eq!(v.y, 0.0);
    }

    #[test]
    fn serve_goes_left_for_negative_dir() {
        let v = serve_velocity(-1.0, 300.0);
        assert!(v.x < 0.0);
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test pong`
Expected: PASS (ball + paddle + 2 serve tests).

- [ ] **Step 3: Verify it runs**

Run: `cargo run`
Expected: from the menu choose Pong (key 2 or select+Enter). A mode prompt appears; press 1 for CPU. Ball bounces, paddles work (W/S left; CPU tracks right), scoring increments, first to 7 ends to the Game Over screen showing the result; Esc returns to menu.

- [ ] **Step 4: Commit**

```bash
git add src/pong/mod.rs
git commit -m "feat: playable Pong with CPU and 2-player modes

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 11: pacman/maze.rs — tile maze and pellets

**Files:**
- Create: `src/pacman/maze.rs`
- Modify: `src/pacman/mod.rs` (add `pub mod maze;` to the stub)

**Interfaces:**
- Consumes: nothing.
- Produces:
  - `pacman::maze::MAZE: &[&str]` (the layout; `#`=wall, `.`=pellet, `P`=player start, `G`=ghost start, space=empty).
  - `struct Maze { pub cols: i32, pub rows: i32, pub player_start: (i32, i32), pub ghost_starts: Vec<(i32, i32)>, pub pellet_total: usize }` plus private tiles/pellets.
  - `Maze::from_ascii(rows: &[&str]) -> Maze`.
  - `Maze::is_wall(&self, c: i32, r: i32) -> bool` (out-of-bounds counts as wall).
  - `Maze::pellet_at(&self, c: i32, r: i32) -> bool`.
  - `Maze::eat(&mut self, c: i32, r: i32) -> bool` (true if a pellet was there).
  - `Maze::pellets_remaining(&self) -> usize`.
  - `Maze::cleared(&self) -> bool`.

- [ ] **Step 1: Write the failing test**

Create `src/pacman/maze.rs`:

```rust
/// 19 columns x 21 rows. '#' wall, '.' pellet, ' ' empty, 'P' player, 'G' ghost.
pub const MAZE: &[&str] = &[
    "###################",
    "#........#........#",
    "#.###.###.###.###.#",
    "#.................#",
    "#.###.#.###.#.###.#",
    "#.....#..#..#.....#",
    "#####.##.#.##.#####",
    "#   #.#  G  #.#   #",
    "#####.# ### #.#####",
    "#........#........#",
    "#.###.###.###.###.#",
    "#...#....P....#...#",
    "###.#.#.###.#.#.###",
    "#.....#..#..#.....#",
    "#.#######.#######.#",
    "#.................#",
    "#.###.###.###.###.#",
    "#...#.........#...#",
    "#.#.#.#######.#.#.#",
    "#.................#",
    "###################",
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
}

pub struct Maze {
    pub cols: i32,
    pub rows: i32,
    tiles: Vec<Tile>,
    pellets: Vec<bool>,
    pub player_start: (i32, i32),
    pub ghost_starts: Vec<(i32, i32)>,
    pub pellet_total: usize,
}

impl Maze {
    pub fn from_ascii(rows: &[&str]) -> Self {
        let height = rows.len() as i32;
        let width = rows.iter().map(|r| r.len()).max().unwrap_or(0) as i32;
        let mut tiles = vec![Tile::Open; (width * height) as usize];
        let mut pellets = vec![false; (width * height) as usize];
        let mut player_start = (1, 1);
        let mut ghost_starts = Vec::new();
        let mut pellet_total = 0;

        for (r, line) in rows.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                let idx = r * width as usize + c;
                match ch {
                    '#' => tiles[idx] = Tile::Wall,
                    '.' => {
                        pellets[idx] = true;
                        pellet_total += 1;
                    }
                    'P' => player_start = (c as i32, r as i32),
                    'G' => ghost_starts.push((c as i32, r as i32)),
                    _ => {}
                }
            }
        }

        Maze {
            cols: width,
            rows: height,
            tiles,
            pellets,
            player_start,
            ghost_starts,
            pellet_total,
        }
    }

    fn in_bounds(&self, c: i32, r: i32) -> bool {
        c >= 0 && r >= 0 && c < self.cols && r < self.rows
    }

    fn index(&self, c: i32, r: i32) -> usize {
        (r * self.cols + c) as usize
    }

    pub fn is_wall(&self, c: i32, r: i32) -> bool {
        if !self.in_bounds(c, r) {
            return true;
        }
        self.tiles[self.index(c, r)] == Tile::Wall
    }

    pub fn pellet_at(&self, c: i32, r: i32) -> bool {
        self.in_bounds(c, r) && self.pellets[self.index(c, r)]
    }

    pub fn eat(&mut self, c: i32, r: i32) -> bool {
        if self.pellet_at(c, r) {
            let i = self.index(c, r);
            self.pellets[i] = false;
            true
        } else {
            false
        }
    }

    pub fn pellets_remaining(&self) -> usize {
        self.pellets.iter().filter(|&&p| p).count()
    }

    pub fn cleared(&self) -> bool {
        self.pellets_remaining() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_dimensions() {
        let m = Maze::from_ascii(MAZE);
        assert_eq!(m.rows, 21);
        assert_eq!(m.cols, 19);
    }

    #[test]
    fn border_is_wall_and_start_is_open() {
        let m = Maze::from_ascii(MAZE);
        assert!(m.is_wall(0, 0));
        let (pc, pr) = m.player_start;
        assert!(!m.is_wall(pc, pr));
    }

    #[test]
    fn out_of_bounds_counts_as_wall() {
        let m = Maze::from_ascii(MAZE);
        assert!(m.is_wall(-1, 5));
        assert!(m.is_wall(1000, 5));
    }

    #[test]
    fn has_pellets_and_at_least_one_ghost() {
        let m = Maze::from_ascii(MAZE);
        assert!(m.pellet_total > 0);
        assert!(!m.ghost_starts.is_empty());
    }

    #[test]
    fn eating_a_pellet_removes_it() {
        let mut m = Maze::from_ascii(MAZE);
        // Find any pellet cell.
        let mut found = None;
        for r in 0..m.rows {
            for c in 0..m.cols {
                if m.pellet_at(c, r) {
                    found = Some((c, r));
                }
            }
        }
        let (c, r) = found.expect("maze should have a pellet");
        let before = m.pellets_remaining();
        assert!(m.eat(c, r));
        assert!(!m.eat(c, r)); // already eaten
        assert_eq!(m.pellets_remaining(), before - 1);
    }
}
```

- [ ] **Step 2: Register the submodule**

At the top of `src/pacman/mod.rs` (the stub) add:

```rust
pub mod maze;
```

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test pacman::maze`
Expected: PASS (5 tests).

- [ ] **Step 4: Commit**

```bash
git add src/pacman/maze.rs src/pacman/mod.rs
git commit -m "feat: pac-man tile maze with pellets

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 12: pacman/player.rs — grid movement

**Files:**
- Create: `src/pacman/player.rs`
- Modify: `src/pacman/mod.rs` (add `pub mod player;`)

**Interfaces:**
- Consumes: `crate::pacman::maze::Maze`.
- Produces:
  - `struct Player { pub col: i32, pub row: i32, pub dir: (i32, i32), pub next_dir: (i32, i32) }`
  - `Player::new(col: i32, row: i32) -> Player`.
  - `Player::advance(&mut self, maze: &mut Maze, want: (i32, i32)) -> (bool, bool)` — tries `want` then current `dir`; moves one tile if open, eats pellet. Returns `(moved, ate_pellet)`.

- [ ] **Step 1: Write the failing test**

Create `src/pacman/player.rs`:

```rust
use crate::pacman::maze::Maze;

pub struct Player {
    pub col: i32,
    pub row: i32,
    pub dir: (i32, i32),
    pub next_dir: (i32, i32),
}

impl Player {
    pub fn new(col: i32, row: i32) -> Self {
        Player {
            col,
            row,
            dir: (0, 0),
            next_dir: (0, 0),
        }
    }

    /// Advance one tile. Prefers `want` (if non-zero, it is queued), falling back to
    /// the current direction. Eats a pellet on arrival. Returns (moved, ate_pellet).
    pub fn advance(&mut self, maze: &mut Maze, want: (i32, i32)) -> (bool, bool) {
        if want != (0, 0) {
            self.next_dir = want;
        }
        for d in [self.next_dir, self.dir] {
            if d == (0, 0) {
                continue;
            }
            let nc = self.col + d.0;
            let nr = self.row + d.1;
            if !maze.is_wall(nc, nr) {
                self.col = nc;
                self.row = nr;
                self.dir = d;
                let ate = maze.eat(nc, nr);
                return (true, ate);
            }
        }
        (false, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiny() -> Maze {
        // P at (1,1), pellets to the right, wall at the ends.
        Maze::from_ascii(&["#####", "#P..#", "#####"])
    }

    #[test]
    fn moves_into_open_tile_and_eats() {
        let mut m = tiny();
        let (pc, pr) = m.player_start;
        let mut p = Player::new(pc, pr);
        let (moved, ate) = p.advance(&mut m, (1, 0));
        assert!(moved);
        assert!(ate);
        assert_eq!((p.col, p.row), (2, 1));
    }

    #[test]
    fn blocked_by_wall_does_not_move() {
        let mut m = tiny();
        let mut p = Player::new(1, 1);
        // Move right to (2,1), (3,1); next right is wall at (4,1).
        p.advance(&mut m, (1, 0));
        p.advance(&mut m, (1, 0));
        let (moved, _) = p.advance(&mut m, (1, 0));
        assert!(!moved);
        assert_eq!((p.col, p.row), (3, 1));
    }

    #[test]
    fn continues_in_current_dir_when_want_is_blocked() {
        let mut m = tiny();
        let mut p = Player::new(1, 1);
        // Establish rightward motion.
        p.advance(&mut m, (1, 0));
        // Want up (wall), should keep going right.
        let (moved, _) = p.advance(&mut m, (0, -1));
        assert!(moved);
        assert_eq!((p.col, p.row), (3, 1));
    }
}
```

- [ ] **Step 2: Register the submodule**

In `src/pacman/mod.rs` add: `pub mod player;`

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test pacman::player`
Expected: PASS (3 tests).

- [ ] **Step 4: Commit**

```bash
git add src/pacman/player.rs src/pacman/mod.rs
git commit -m "feat: pac-man grid-based player movement

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 13: pacman/ghost.rs — chase AI

**Files:**
- Create: `src/pacman/ghost.rs`
- Modify: `src/pacman/mod.rs` (add `pub mod ghost;`)

**Interfaces:**
- Consumes: `crate::pacman::maze::Maze`.
- Produces:
  - `struct Ghost { pub col: i32, pub row: i32, pub dir: (i32, i32) }`
  - `Ghost::new(col: i32, row: i32) -> Ghost`.
  - `Ghost::step(&mut self, maze: &Maze, target: (i32, i32))` — moves one tile toward target, no reversing unless forced (deterministic).
  - `Ghost::touches(&self, col: i32, row: i32) -> bool`.

- [ ] **Step 1: Write the failing test**

Create `src/pacman/ghost.rs`:

```rust
use crate::pacman::maze::Maze;

pub struct Ghost {
    pub col: i32,
    pub row: i32,
    pub dir: (i32, i32),
}

impl Ghost {
    pub fn new(col: i32, row: i32) -> Self {
        Ghost {
            col,
            row,
            dir: (0, 0),
        }
    }

    /// Step one tile toward `target`, choosing the open non-reverse neighbour that
    /// minimizes squared distance. Reverses only if no other option exists.
    pub fn step(&mut self, maze: &Maze, target: (i32, i32)) {
        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let reverse = (-self.dir.0, -self.dir.1);
        let mut best: Option<((i32, i32), i32)> = None;
        for d in dirs {
            if d == reverse {
                continue;
            }
            let nc = self.col + d.0;
            let nr = self.row + d.1;
            if maze.is_wall(nc, nr) {
                continue;
            }
            let dist = (nc - target.0).pow(2) + (nr - target.1).pow(2);
            if best.map_or(true, |(_, bd)| dist < bd) {
                best = Some((d, dist));
            }
        }
        if best.is_none() {
            let nc = self.col + reverse.0;
            let nr = self.row + reverse.1;
            if !maze.is_wall(nc, nr) {
                best = Some((reverse, 0));
            }
        }
        if let Some((d, _)) = best {
            self.dir = d;
            self.col += d.0;
            self.row += d.1;
        }
    }

    pub fn touches(&self, col: i32, row: i32) -> bool {
        self.col == col && self.row == row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_toward_target_along_a_corridor() {
        let m = Maze::from_ascii(&["#######", "#.....#", "#######"]);
        let mut g = Ghost::new(1, 1);
        g.step(&m, (5, 1));
        assert_eq!((g.col, g.row), (2, 1));
    }

    #[test]
    fn does_not_reverse_when_forward_is_open() {
        let m = Maze::from_ascii(&["#######", "#.....#", "#######"]);
        let mut g = Ghost::new(3, 1);
        g.dir = (1, 0); // moving right
        // Target is behind it, but it should keep moving right (no reverse).
        g.step(&m, (0, 1));
        assert_eq!((g.col, g.row), (4, 1));
    }

    #[test]
    fn touches_detects_same_cell() {
        let g = Ghost::new(3, 4);
        assert!(g.touches(3, 4));
        assert!(!g.touches(3, 5));
    }
}
```

- [ ] **Step 2: Register the submodule**

In `src/pacman/mod.rs` add: `pub mod ghost;`

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test pacman::ghost`
Expected: PASS (3 tests).

- [ ] **Step 4: Commit**

```bash
git add src/pacman/ghost.rs src/pacman/mod.rs
git commit -m "feat: pac-man ghost chase AI

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 14: pacman/mod.rs — PacmanGame screen

**Files:**
- Modify: `src/pacman/mod.rs` (replace the stub body; keep the `pub mod maze/player/ghost;` lines)

**Interfaces:**
- Consumes: `crate::pacman::maze::{Maze, MAZE}`, `crate::pacman::player::Player`, `crate::pacman::ghost::Ghost`, `crate::app::{Screen, ScreenId, Transition}`, `crate::input::Input`, `crate::audio::Sfx`, `crate::theme`, `crate::{SharedCtx, GameResult}`.
- Produces: `PacmanGame::new(ctx: SharedCtx)` (impl `Screen`); `pacman::level_bonus(seconds: f32) -> u32` (testable time-bonus helper).

- [ ] **Step 1: Write the failing test for the bonus helper**

Replace the entire contents of `src/pacman/mod.rs` with:

```rust
pub mod ghost;
pub mod maze;
pub mod player;

use crate::app::{Screen, ScreenId, Transition};
use crate::audio::Sfx;
use crate::input::Input;
use crate::theme;
use crate::{GameResult, SharedCtx};
use ghost::Ghost;
use macroquad::prelude::*;
use maze::{Maze, MAZE};
use player::Player;

const PELLET_SCORE: u32 = 10;
const ORIGIN_X: f32 = 130.0;
const ORIGIN_Y: f32 = 40.0;
const TILE: f32 = 20.0;

/// Time bonus: 200 points minus 2 per elapsed second, floored at 0.
pub fn level_bonus(seconds: f32) -> u32 {
    let penalty = (seconds * 2.0) as i64;
    (200i64 - penalty).max(0) as u32
}

fn move_interval(level: u32) -> f32 {
    match level {
        1 => 0.18,
        2 => 0.13,
        3 => 0.09,
        _ => 0.06,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    SpeedSelect,
    Playing,
}

pub struct PacmanGame {
    ctx: SharedCtx,
    phase: Phase,
    level: u32,
    maze: Maze,
    player: Player,
    ghosts: Vec<Ghost>,
    score: u32,
    lives: u32,
    elapsed: f32,
    move_timer: f32,
    ghost_timer: f32,
}

impl PacmanGame {
    pub fn new(ctx: SharedCtx) -> Self {
        let maze = Maze::from_ascii(MAZE);
        let (pc, pr) = maze.player_start;
        let ghosts = maze.ghost_starts.iter().map(|&(c, r)| Ghost::new(c, r)).collect();
        PacmanGame {
            ctx,
            phase: Phase::SpeedSelect,
            level: 1,
            player: Player::new(pc, pr),
            ghosts,
            maze,
            score: 0,
            lives: 3,
            elapsed: 0.0,
            move_timer: 0.0,
            ghost_timer: 0.0,
        }
    }

    fn reset_positions(&mut self) {
        let (pc, pr) = self.maze.player_start;
        self.player = Player::new(pc, pr);
        for (g, &(c, r)) in self.ghosts.iter_mut().zip(self.maze.ghost_starts.iter()) {
            g.col = c;
            g.row = r;
            g.dir = (0, 0);
        }
    }

    fn tile_center(&self, col: i32, row: i32) -> (f32, f32) {
        (
            ORIGIN_X + col as f32 * TILE + TILE / 2.0,
            ORIGIN_Y + row as f32 * TILE + TILE / 2.0,
        )
    }

    fn lose_life_or_end(&mut self) -> Option<Transition> {
        self.lives -= 1;
        self.ctx.borrow().audio.play(Sfx::Death);
        if self.lives == 0 {
            let is_best = self.ctx.borrow_mut().scores.record_pacman(self.score);
            self.ctx.borrow_mut().last_result = Some(GameResult {
                title: "GAME OVER".to_string(),
                score: self.score,
                subtitle: if is_best { "New best score!".to_string() } else { String::new() },
            });
            return Some(Transition::Goto(ScreenId::GameOver));
        }
        self.reset_positions();
        None
    }
}

impl Screen for PacmanGame {
    fn update(&mut self, input: &Input, dt: f32) -> Option<Transition> {
        if input.escape {
            return Some(Transition::Goto(ScreenId::Menu));
        }

        if self.phase == Phase::SpeedSelect {
            if let Some(d) = input.digit {
                self.level = d as u32;
                self.phase = Phase::Playing;
            }
            return None;
        }

        self.elapsed += dt;
        self.move_timer += dt;
        self.ghost_timer += dt;
        let interval = move_interval(self.level);

        // Player steps on its own cadence.
        if self.move_timer >= interval {
            self.move_timer -= interval;
            let (_, ate) = self.player.advance(&mut self.maze, input.dir4());
            if ate {
                self.score += PELLET_SCORE;
                self.ctx.borrow().audio.play(Sfx::Chomp);
            }
        }

        // Ghosts step slightly slower than the player.
        if self.ghost_timer >= interval * 1.15 {
            self.ghost_timer -= interval * 1.15;
            let target = (self.player.col, self.player.row);
            for g in &mut self.ghosts {
                g.step(&self.maze, target);
            }
        }

        // Collision (check after movement).
        if self.ghosts.iter().any(|g| g.touches(self.player.col, self.player.row)) {
            if let Some(t) = self.lose_life_or_end() {
                return Some(t);
            }
        }

        // Win: maze cleared.
        if self.maze.cleared() {
            let bonus = level_bonus(self.elapsed);
            self.score += bonus;
            self.ctx.borrow().audio.play(Sfx::Win);
            let is_best = self.ctx.borrow_mut().scores.record_pacman(self.score);
            self.ctx.borrow_mut().last_result = Some(GameResult {
                title: "MAZE CLEARED".to_string(),
                score: self.score,
                subtitle: if is_best { "New best score!".to_string() } else { format!("Time bonus {}", bonus) },
            });
            return Some(Transition::Goto(ScreenId::GameOver));
        }

        None
    }

    fn draw(&self) {
        if self.phase == Phase::SpeedSelect {
            draw_text("PAC-MAN", 230.0, 150.0, 52.0, theme::ACCENT);
            draw_text("Choose speed: 1  2  3  4", 180.0, 250.0, 30.0, theme::TEXT);
            draw_text("(1 = slow, 4 = fast)   Esc = menu", 150.0, 300.0, 24.0, theme::TEXT);
            return;
        }

        // Walls and pellets.
        for r in 0..self.maze.rows {
            for c in 0..self.maze.cols {
                let x = ORIGIN_X + c as f32 * TILE;
                let y = ORIGIN_Y + r as f32 * TILE;
                if self.maze.is_wall(c, r) {
                    draw_rectangle(x, y, TILE, TILE, theme::WALL);
                } else if self.maze.pellet_at(c, r) {
                    draw_circle(x + TILE / 2.0, y + TILE / 2.0, 2.5, theme::PELLET);
                }
            }
        }

        // Player.
        let (px, py) = self.tile_center(self.player.col, self.player.row);
        draw_circle(px, py, TILE / 2.0 - 2.0, theme::PACMAN);

        // Ghosts.
        for (i, g) in self.ghosts.iter().enumerate() {
            let (gx, gy) = self.tile_center(g.col, g.row);
            let color = if i % 2 == 0 { theme::GHOST_A } else { theme::GHOST_B };
            draw_circle(gx, gy, TILE / 2.0 - 2.0, color);
        }

        // HUD.
        draw_text(&format!("Score {}", self.score), 10.0, 20.0, 24.0, theme::TEXT);
        draw_text(&format!("Lives {}", self.lives), 250.0, 20.0, 24.0, theme::TEXT);
        draw_text(&format!("Lv {}", self.level), 420.0, 20.0, 24.0, theme::TEXT);
        draw_text(&format!("Time {:.0}", self.elapsed), 520.0, 20.0, 24.0, theme::TEXT);
    }

    fn id(&self) -> ScreenId {
        ScreenId::Pacman
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus_is_full_at_zero_seconds() {
        assert_eq!(level_bonus(0.0), 200);
    }

    #[test]
    fn bonus_decreases_over_time() {
        assert_eq!(level_bonus(10.0), 180);
    }

    #[test]
    fn bonus_floors_at_zero() {
        assert_eq!(level_bonus(1000.0), 0);
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test`
Expected: PASS (all suites including 3 new bonus tests).

- [ ] **Step 3: Verify it runs**

Run: `cargo run`
Expected: from the menu choose Pac-Man. Speed prompt appears; press 1–4. The maze renders; the pac dot moves on arrows/WASD and eats pellets (chomp sound, score rises); ghosts chase and cost a life on contact; clearing all pellets or losing all lives goes to Game Over with the score (and a time bonus on a clear). Esc returns to menu.

- [ ] **Step 4: Commit**

```bash
git add src/pacman/mod.rs
git commit -m "feat: playable Pac-Man with ghosts, lives, speed levels, time bonus

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

---

## Task 15: Web (WASM) build, README, and final polish

**Files:**
- Create: `index.html`
- Create: `build-web.sh`
- Modify: `README.md`
- Modify: `.gitignore` (already ignores `/web/*.wasm`; confirm)

**Interfaces:**
- Consumes: the finished binary.
- Produces: a `web/` bundle playable in a browser.

- [ ] **Step 1: Add the wasm target**

Run: `rustup target add wasm32-unknown-unknown`
Expected: target installed (or "up to date").

- [ ] **Step 2: Create `index.html`**

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>rust-retro-arcade</title>
  <style>
    html, body { margin: 0; height: 100%; background: #07070f; }
    body { display: flex; align-items: center; justify-content: center; }
    canvas {
      /* Scale the 640x480 canvas to fit the viewport, preserving aspect. */
      width: min(100vw, 133.333vh);
      height: auto;
      image-rendering: pixelated;
      outline: none;
    }
  </style>
</head>
<body>
  <canvas id="glcanvas" width="640" height="480" tabindex="1"></canvas>
  <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
  <script src="https://not-fl3.github.io/miniquad-samples/quad-storage.js"></script>
  <script>
    load("rust-retro-arcade.wasm");
  </script>
</body>
</html>
```

- [ ] **Step 3: Create `build-web.sh`**

```bash
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
```

- [ ] **Step 4: Make it executable and build the web bundle**

Run:
```bash
chmod +x build-web.sh
./build-web.sh
```
Expected: `web/rust-retro-arcade.wasm` and `web/index.html` are produced; the release wasm build compiles cleanly.

- [ ] **Step 5: Verify in a browser**

Run:
```bash
(cd web && python3 -m http.server 8080)
```
Then open `http://localhost:8080`.
Expected: the arcade loads in the browser. Verify, with the browser console open (check for errors):
- Menu navigates; both games are playable with the keyboard.
- Sound plays after the first key press (browsers gate audio on a user gesture).
- **High-score persistence:** set a Pac-Man/Pong high score, then reload the page — the menu still shows it. If the score resets, check the console: the `quad-storage.js` script tag must load without error; fix the `src` URL if needed.

> If a console error shows `quad-storage` is unavailable, vendor the JS locally: download `mq_js_bundle.js` and `quad-storage.js` into `web/js/` and change the two `<script src=...>` tags (and the `cp` lines in `build-web.sh`) to point at `js/…`. Re-test.

- [ ] **Step 6: Update `README.md`**

Replace the "Status", "Build & run (planned)" sections so they reflect reality:

```markdown
## Status

Playable. Pac-Man and Pong run natively and in the browser (WASM).

## Build & run

```sh
# Native
cargo run

# Web (WASM)
rustup target add wasm32-unknown-unknown   # once
./build-web.sh
(cd web && python3 -m http.server 8080)    # then open http://localhost:8080
```
```

- [ ] **Step 7: Final quality gate**

Run:
```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```
Expected: fmt makes no further changes, clippy is clean, all tests pass.

- [ ] **Step 8: Commit**

```bash
git add index.html build-web.sh README.md .gitignore
git commit -m "feat: web (wasm) build, README, and final polish

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

- [ ] **Step 9: Push**

```bash
git push
```
Expected: branch pushed to `origin/main` on GitHub.

---

## Self-Review Notes (for the planner)

- **Spec coverage:** menu/instructions/credits (Task 7), Pac-Man tile maze + ghosts + lives + speed levels + time bonus (Tasks 11–14), Pong vs-AI + 2-player + angle bounce + first-to-7 (Tasks 8–10), sound effects (Task 6, used in 10 & 14), high scores web+native (Task 5, surfaced in 7/10/14), virtual 640×480 + CSS scaling (Tasks 1 & 15), logic/render separation (every game module is render-free and tested), web build (Task 15). Credits show only ANVESH (Task 7). All spec sections map to tasks.
- **Known integration risk:** web high-score persistence depends on the `quad-storage` JS include; Task 15 Step 5 explicitly verifies it in-browser and gives a vendoring fallback.
- **Out of scope (per spec):** power pellets, fruit, multiple mazes, level-code cheat, the auto-demo.
```
