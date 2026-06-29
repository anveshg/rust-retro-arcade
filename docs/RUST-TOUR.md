# A Guided Tour of Rust — using this arcade as the textbook

This document teaches **Rust** to someone who already programs in another language
(C++, Python, JavaScript, Java…). Instead of toy snippets, it uses a real, working
program — the `rust-retro-arcade` (a Pac‑Man + Pong arcade built with
[macroquad](https://github.com/not-fl3/macroquad)) — as the running example.

Every source file is also **heavily commented**: a `//!` block at the top of each
file says what it does and which Rust ideas it shows, and `///` / `//` comments
explain language features at the exact line they appear. This tour gives you the
**reading order** and ties the concepts together.

> How to use it: read this page once for the map, then read the files in the order
> below with the file open in your editor. Each file is small (40–280 lines).

---

## 0. Prerequisites — run it first

Seeing it run makes the code concrete.

```sh
# Install Rust (if needed): https://rustup.rs
cargo run            # opens the native game window
cargo test           # runs the unit tests (fast, no window)
```

Three things to know about the toolchain:

- **`cargo`** is Rust's build tool + package manager (like `npm`/`pip` + `make`).
  `cargo run` compiles and runs; `cargo test` runs tests; `cargo build` just compiles.
- **`Cargo.toml`** is the manifest (dependencies, package name, edition). `Cargo.lock`
  pins exact dependency versions.
- Dependencies (called **crates**) come from [crates.io](https://crates.io). This
  project uses exactly one: `macroquad`.

---

## 1. The big picture

The program is a small **state machine of screens**. One screen is active at a time
(Menu, Pong, Pac‑Man, Instructions, Credits, Game Over). Each screen, every frame,
gets a chance to **update** (react to input/time) and **draw** itself, and may ask to
**transition** to another screen.

```
            ┌─────────────────────────── main.rs ───────────────────────────┐
            │  game loop:  poll input → app.update(dt) → app.draw()          │
            │  shared state: Rc<RefCell<Ctx>>  (audio, high scores, result)  │
            └───────────────┬───────────────────────────────────────────────┘
                            │ owns the current
                            ▼
                    Box<dyn Screen>            ← a trait object (app.rs)
            ┌───────────────┼────────────────────────────────────┐
         Menu   Instructions   Credits   GameOver   PongGame   PacmanGame
        (screens/*)                                  (pong/*)   (pacman/*)
```

The two interesting Rust ideas in that diagram:

- **`trait Screen`** (in `app.rs`) is an *interface*: a set of methods
  (`update`, `draw`, `id`) that every screen implements. `Box<dyn Screen>` is a
  *trait object* — a value whose concrete type is only known at runtime, so the
  game loop can hold "whatever the current screen is" and call the trait methods on it.
- **`Rc<RefCell<Ctx>>`** is how several screens *share* the same audio player and
  high‑score data. Rust normally forbids two owners of one value; this is the
  standard escape hatch (explained in §5 and §9).

The render‑free game logic (ball physics, ghost AI, the maze, scoring) lives in
plain structs and functions with **unit tests**; the macroquad drawing/input code is
a thin layer on top. That separation is why `cargo test` can verify the game without
opening a window.

---

## 2. Reading order

Read these in order — each builds on the previous one. The parenthetical is the
**main Rust concept** the file teaches.

| # | File | Teaches |
|---|------|---------|
| 1 | `src/theme.rs` | `const`, struct literals, `pub`, types |
| 2 | `src/input.rs` | structs, `#[derive(...)]`, `Default`, `Option`, tuples, methods vs associated fns |
| 3 | `src/app.rs` | **traits**, **trait objects** (`Box<dyn>`), **generics**, data‑carrying `enum`s, `match` |
| 4 | `src/scores.rs` | `#[cfg(...)]` per‑platform code, modules, `Result` vs `Option`, `unsafe`/`extern` FFI |
| 5 | `src/audio.rs` | `Vec<u8>`, bytes & casts (`as`), `async`/`.await`, graceful `Option` |
| 6 | `src/screens/*` | implementing a trait for many types, `pub use` re‑exports, borrowing shared state |
| 7 | `src/pong/*` | `&mut self`, `Vec2` math, small state‑machine `enum`s, `RefCell` borrows |
| 8 | `src/pacman/*` | `Vec<T>` + indexing, iterators (`map`/`enumerate`/`collect`), tuples as coords, AI |
| 9 | `src/main.rs` | **ties it together**: `Rc<RefCell>`, closures, `async` main, the game loop |

If you only read three files, read **`app.rs`** (the trait + generics), **`scores.rs`**
(systems‑Rust: `cfg`, `Result`, `unsafe`), and **`main.rs`** (ownership across the app).

---

## 3. The concepts, and where to see them

A "go look at the real line" index. Each entry points to a file where the concept is
used and commented.

### Ownership & borrowing (Rust's defining feature)
- **`&self` vs `&mut self`** — read‑only vs mutating methods: `pong/ball.rs`
  (`step` mutates, `bounce_offset` reads).
- **Borrowing rules in practice** — why `record_pacman(&mut self)` needs `&mut`:
  `scores.rs`.
- **Shared ownership** — `Rc<RefCell<Ctx>>` so Menu/Pong/Pac‑Man share one audio +
  scores object: `main.rs`, used in every screen via `.borrow()` / `.borrow_mut()`.
- **`.clone()` of an `Rc` is cheap** (bumps a counter; doesn't copy the data):
  `main.rs`.

### Types & data
- **`struct` with `pub` fields** — `pong/ball.rs`, `input.rs`.
- **`enum` that carries data** (unlike C enums) — `Transition::Goto(ScreenId)` in
  `app.rs`; `Tile` in `pacman/maze.rs`.
- **`Option<T>`** ("maybe a value", no `null`) — `input.rs` (`Option<u8>`),
  `audio.rs` (`Option<Sound>`), `screens/gameover.rs` (`Option<GameResult>`).
- **`Result<T, E>`** ("ok or error") — `scores.rs` (`fs::read_to_string`), turned into
  an `Option` with `.ok()` in `audio.rs`.
- **Tuples** as lightweight grouped values — directions `(i32, i32)` in
  `pacman/player.rs` & `ghost.rs`; return `(bool, bool)` in `player.rs`.
- **Fixed‑size arrays** `[&str; 5]` — `screens/menu.rs`; slices `&[&str]` — `pacman/maze.rs`.

### Polymorphism
- **`trait`** (interface) — `Screen` in `app.rs`.
- **`impl Trait for Type`** — every screen in `screens/*`, `pong/mod.rs`, `pacman/mod.rs`.
- **Trait objects** `Box<dyn Screen>` (dynamic dispatch) — `app.rs`, `main.rs`.
- **Generics with a trait bound** `App<F: Fn(...) -> ...>` — `app.rs`.
- **Closures** captured by `move` and passed as a value — `main.rs`.

### Pattern matching
- **`match`** (exhaustive) — `app.rs` (`Transition`), `screens/menu.rs` (`menu_target`),
  `pacman/mod.rs` (`move_interval`).
- **`if let Some(x) = ...`** — `audio.rs` (`play`), `screens/gameover.rs`, `scores.rs`.

### Compile‑time configuration & FFI
- **`#[cfg(target_arch = "wasm32")]`** — two implementations of `persist`, one for
  native (a file) and one for the browser (localStorage): `scores.rs`.
- **`extern "C"` + `unsafe`** — calling JavaScript functions from the wasm build:
  `scores.rs`.

### Iterators & collections
- **`Vec<T>`** and building it — `pacman/maze.rs` (`Vec<Tile>`), `pacman/mod.rs`
  (`Vec<Ghost>` via `.iter().map(...).collect()`).
- **`.iter_mut().enumerate()`** to mutate items with their index — `pacman/mod.rs`
  (give each ghost a different chase target).
- **`.chars().enumerate()`, `.filter().count()`** — `pacman/maze.rs`.

### Async
- **`async fn` / `.await`** — `audio.rs` (loading sounds), `main.rs` (the
  `#[macroquad::main]` loop runs in an async context).

### Modules & visibility
- **`mod foo;`** maps to `foo.rs` / `foo/mod.rs` — `main.rs`.
- **`pub mod` + `pub use`** to re‑export and flatten paths — `screens/mod.rs`.
- **`pub`** controls what's visible outside the module — everywhere.

### Tests
- **`#[cfg(test)] mod tests`** (tests live beside the code) — `input.rs`, `scores.rs`,
  `audio.rs`, `pong/*`, `pacman/*`. Note floats are compared with an epsilon, not `==`
  (`pong/ball.rs`).

---

## 4. Three patterns worth internalizing

These are the "aha" moments this codebase is built around.

**1. The trait‑object state machine.** `App` holds one `Box<dyn Screen>` and a factory
closure. When a screen's `update` returns `Some(Transition::Goto(id))`, `App` calls the
factory to build the next screen and swaps it in. This is how you get polymorphism +
runtime‑chosen behavior in a language with no inheritance. See `app.rs` then `main.rs`.

**2. `Rc<RefCell<T>>` for shared, mutable state.** The borrow checker forbids two
owners of mutable data at compile time. When you genuinely need shared mutable state
(here: one audio player + one high‑score record used by several screens), you opt into
**runtime** borrow checking: `Rc` allows multiple owners, `RefCell` allows mutation
through a shared reference and panics if you break the rules at runtime. Each use is a
short `self.ctx.borrow()` / `.borrow_mut()` that drops immediately. See `main.rs` and
any screen's `update`.

**3. `#[cfg]` for "same API, different implementation per platform."** `scores.rs`
defines the `persist` module twice — once `#[cfg(not(target_arch = "wasm32"))]` (writes
a file) and once `#[cfg(target_arch = "wasm32")]` (calls JavaScript via `extern "C"`).
The rest of the program calls `scores::load()` / `save()` and never knows which one is
compiled in.

---

## 5. Things that will surprise you coming from C++/Python/JS

- **No `null`.** "Maybe a value" is `Option<T>`; you must handle the `None` case, which
  is why bugs like null‑pointer dereferences mostly vanish.
- **No exceptions.** Recoverable failure is a returned `Result<T, E>`; you handle it
  (or propagate with `?`). `panic!` is for unrecoverable bugs.
- **Casts are explicit.** `x as i64`, `n as f32` — no silent numeric coercion. See the
  byte math in `audio.rs` and `level_bonus` in `pacman/mod.rs`.
- **The compiler enforces ownership.** A value has one owner; passing it can *move* it.
  Borrowing (`&`, `&mut`) lets you use it without taking ownership, under rules the
  compiler checks. When that's too strict for genuinely shared state, you reach for
  `Rc`/`RefCell` (and accept runtime checks).
- **`match` must be exhaustive.** You can't forget an `enum` case; the compiler refuses.
- **Tests live next to code** in `#[cfg(test)] mod tests` and are compiled only for tests.

---

## 6. Explore further

- **Generate API docs:** `cargo doc --open` renders all the `///` comments as browsable
  HTML — a nice way to see the documented surface.
- **Run one test:** `cargo test pong::ball` runs just the ball‑physics tests.
- **Experiments to try** (great for learning — the compiler is your tutor):
  - In `pong/mod.rs`, change `TARGET` from `7` to `3` and re‑run. (Find where the win
    condition uses it.)
  - In `pacman/maze.rs`, edit the `MAZE` ascii art (keep every row 19 chars) and watch
    the maze change.
  - Try removing a `match` arm in `app.rs`'s `make_screen` — the compiler will refuse to
    build and tell you exactly which case you dropped. That refusal *is* the lesson.
  - Add a new `Sfx` variant in `audio.rs` without handling it in `play`'s `match` — again
    the compiler stops you.

When the compiler complains, read the error slowly: Rust's error messages are unusually
good and usually tell you the fix. Fighting the borrow checker for a week and then
"getting it" is the normal path.

---

*Architecture & design rationale live in
[`docs/superpowers/specs/`](superpowers/specs/) and
[`docs/superpowers/plans/`](superpowers/plans/). The original DOS C++ this was rewritten
from is under [`legacy-cpp/`](../legacy-cpp/).*
