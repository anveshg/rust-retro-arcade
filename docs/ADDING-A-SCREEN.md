# Hands-on: add a new screen

This walkthrough teaches the codebase's architecture by **extending it**. We'll add a
simple new screen called **About**, wiring it into the menu. Along the way you'll see
how Rust's `trait` + `enum` + `match` design makes the compiler *guide* you through
every place that needs updating — a very Rust experience.

> This is a teaching exercise, not a shipped feature. Follow it on a scratch branch
> and throw it away (`git checkout -- .`) when you're done, or keep it if you like it.

**Prerequisites:** read [`RUST-TOUR.md`](RUST-TOUR.md) first (especially `app.rs` and
`main.rs`). Recall the core idea: the app holds one `Box<dyn Screen>`, and each screen
implements the `Screen` trait (`update`, `draw`, `id`).

---

## The mental model

A screen is any type that implements `Screen`:

```rust
pub trait Screen {
    fn update(&mut self, input: &Input, dt: f32) -> Option<Transition>;
    fn draw(&self);
    fn id(&self) -> ScreenId;
}
```

To add one, you touch exactly **five** places. The compiler will refuse to build until
all five agree — so you can't half-finish it by accident. That's the whole lesson:
**make an illegal state un-compilable.**

---

## Step 1 — write the screen (`src/screens/about.rs`)

A minimal static screen that returns to the menu on any key. Compare it with
`instructions.rs` / `credits.rs` — same shape.

```rust
//! A tiny static "About" screen (worked example for docs/ADDING-A-SCREEN.md).

use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use macroquad::prelude::*;

/// A read-only screen that goes back to the menu on any key press.
#[derive(Default)]
pub struct About;

impl About {
    /// Constructor, matching the `::new()` convention the other screens use.
    pub fn new() -> Self {
        About
    }
}

impl Screen for About {
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        // `_dt` is unused here; the leading underscore tells the compiler that's intended.
        if input.any_pressed || input.escape || input.enter {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None // stay on this screen
    }

    fn draw(&self) {
        // Coordinates are in the fixed 640x480 virtual space; main.rs's camera
        // scales this to the real window for us.
        draw_text("ABOUT", 250.0, 120.0, 48.0, theme::ACCENT);
        draw_text("A Rust rewrite of a DOS arcade.", 110.0, 240.0, 26.0, theme::TEXT);
        draw_text("Press any key to return", 200.0, 420.0, 22.0, theme::TEXT);
    }

    fn id(&self) -> ScreenId {
        ScreenId::About
    }
}
```

Two Rust notes:
- `#[derive(Default)]` on a unit struct + a `new()` keeps clippy happy (it dislikes a
  `new()` with no matching `Default`) and matches the other screens.
- `theme` and `draw_text` are the same drawing tools every screen uses; the screen
  never deals with scaling or the window — that's `main.rs`'s job.

## Step 2 — register the module (`src/screens/mod.rs`)

Add the submodule and re-export the type so callers can write `screens::About`:

```rust
pub mod about;        // add this
pub use about::About; // and this
```

## Step 3 — add the `ScreenId` variant (`src/app.rs`)

```rust
pub enum ScreenId {
    Menu,
    Instructions,
    Credits,
    Pong,
    Pacman,
    GameOver,
    About,   // add this
}
```

**The moment you do this, the project stops compiling** — and that's good. The `match`
in `make_screen` is now non-exhaustive (it doesn't handle `About`), and Rust refuses to
build a `match` that could fall through. The error points you straight at Step 4. The
compiler is your checklist.

## Step 4 — build it in the factory (`src/main.rs`)

`make_screen` maps each `ScreenId` to a boxed screen. Add the new arm:

```rust
fn make_screen(id: ScreenId, ctx: SharedCtx) -> Box<dyn Screen> {
    match id {
        ScreenId::Menu => Box::new(screens::Menu::new(ctx)),
        ScreenId::Instructions => Box::new(screens::Instructions::new()),
        ScreenId::Credits => Box::new(screens::Credits::new()),
        ScreenId::Pong => Box::new(pong::PongGame::new(ctx)),
        ScreenId::Pacman => Box::new(pacman::PacmanGame::new(ctx)),
        ScreenId::GameOver => Box::new(screens::GameOver::new(ctx)),
        ScreenId::About => Box::new(screens::About::new()), // add this
    }
}
```

`About::new()` takes no `ctx` because the About screen doesn't use shared state. Screens
that *do* (like `Menu`, which reads high scores) take the `SharedCtx` handle instead.
The return type is `Box<dyn Screen>` — a trait object — so all the arms can return
different concrete types through one uniform interface.

## Step 5 — make it reachable from the menu (`src/screens/menu.rs`)

A screen nothing can navigate to is dead code. Add a menu entry:

```rust
// was: const ITEMS: [&str; 5] = ["Pac-Man", "Pong", "Instructions", "Credits", "Quit"];
const ITEMS: [&str; 6] = ["Pac-Man", "Pong", "Instructions", "Credits", "About", "Quit"];

pub fn menu_target(index: usize) -> Transition {
    match index {
        0 => Transition::Goto(ScreenId::Pacman),
        1 => Transition::Goto(ScreenId::Pong),
        2 => Transition::Goto(ScreenId::Instructions),
        3 => Transition::Goto(ScreenId::Credits),
        4 => Transition::Goto(ScreenId::About), // add this
        _ => Transition::Quit,                  // index 5 (Quit) and anything else
    }
}
```

Note the array length changed from `5` to `6`: Rust arrays carry their length in the
type (`[&str; 6]`), so the count must be exact — the compiler checks it.

One honest caveat: number-key shortcuts only cover `1`–`4` (see `Input::poll`), so the
5th and 6th menu items ("About", "Quit") are reached with the arrow keys + `Enter`, not
a digit. Extending `Input` to read more digits would be a nice follow-up exercise.

## Step 6 — verify

```sh
cargo test     # logic still green (menu_target has a unit test you can extend)
cargo run      # arrow down to "About", press Enter
```

Add a test for the new mapping while you're here:

```rust
#[test]
fn index_four_opens_about() {
    assert_eq!(menu_target(4), Transition::Goto(ScreenId::About));
}
```

---

## What you just learned

- **Trait objects** let the app treat every screen uniformly (`Box<dyn Screen>`),
  while each screen is its own type with its own state.
- **Exhaustive `match`** turns "I forgot to handle the new case" from a runtime bug
  into a compile error. Adding the `ScreenId` variant *forced* you to update the
  factory.
- **Arrays know their length at the type level** (`[&str; 6]`), so the menu can't
  silently get out of sync with its item count.
- The **render-free vs render split**: your screen just draws in 640×480 and returns a
  `Transition`; it never touches the window, the camera, or the game loop.

### Going further: a screen with state

The About screen is stateless. A screen that *does* something keeps fields and mutates
them in `update(&mut self, ...)`. Look at `pong/mod.rs` and `pacman/mod.rs`: they hold
game state (ball, paddles, maze, score), take `SharedCtx` for audio + high scores, and
on game-over set `ctx.last_result` and return `Some(Transition::Goto(ScreenId::GameOver))`.
The wiring (Steps 2–5) is identical — only Step 1 grows.
