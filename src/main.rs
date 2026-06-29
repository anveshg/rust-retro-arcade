//! Entry point for `rust-retro-arcade`, a macroquad arcade launcher that
//! hosts multiple retro games (Pong, Pac-Man) behind a shared menu.
//!
//! **Architecture**: every "page" of the app is a type that implements the
//! `Screen` trait (`app` module).  `App` drives whichever screen is active
//! and swaps to the next via a *factory closure* supplied here.
//!
//! This file owns the game loop (`async fn main`) and the shared mutable
//! context (`Rc<RefCell<Ctx>>`) through which audio and high-score data flow.
use macroquad::prelude::*;
// `use macroquad::prelude::*` is a *glob import*: it pulls every public name
// from the prelude module into scope at once.  Handy for game crates; avoid
// it in library code because it obscures where names originate.
use std::cell::RefCell;
use std::rc::Rc;

// Each `mod foo;` tells the compiler to find `src/foo.rs` (or
// `src/foo/mod.rs`) and compile it as a child module.  Rust's module tree
// is *explicit* — a file is never compiled unless declared with `mod`.
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

/// Data carried from a finished game back to the `GameOver` screen.
///
/// A plain Rust struct with `pub` fields; no getter methods are needed
/// because Rust's visibility rules let callers access fields directly.
pub struct GameResult {
    pub title: String,
    pub score: u32,
    pub subtitle: String,
}

/// Shared application context: audio engine, persistent high scores, and
/// the result of the most recently completed game.
///
/// `Ctx` itself has no special ownership semantics — the `Rc<RefCell<…>>`
/// wrapper (see `SharedCtx`) is what enables shared, mutable access from
/// multiple screens without transferring ownership between them.
pub struct Ctx {
    /// The audio engine, loaded once at startup.  Screens borrow it via the
    /// `RefCell` to play sounds without needing to own the engine themselves.
    pub audio: audio::Audio,
    /// Persistent high scores, loaded from disk at startup and saved on exit.
    pub scores: scores::HighScores,
    /// `Option<T>` is Rust's null-safe optional: `None` means "no result yet";
    /// `Some(result)` wraps the actual value.  No null-pointer bugs possible.
    pub last_result: Option<GameResult>,
}

/// Type alias (`type` keyword) for the shared-ownership wrapper around `Ctx`.
///
/// `Rc<T>` (reference-counted pointer) allows *multiple owners* of one heap
/// allocation.  `.clone()` on an `Rc` increments a counter and returns a new
/// handle to the **same** data — O(1), no deep copy of `Ctx` occurs.
///
/// `RefCell<T>` adds *interior mutability*: the value appears immutable from
/// the outside but can hand out `&mut` borrows at run time, enforcing Rust's
/// "one writer OR many readers" rule with a panic rather than a compile error.
///
/// Together, `Rc<RefCell<T>>` is the idiomatic single-threaded shared-state
/// pattern.  (The multi-threaded equivalent is `Arc<Mutex<T>>`.)
pub type SharedCtx = Rc<RefCell<Ctx>>;

/// Factory that converts a `ScreenId` variant into a heap-allocated screen.
///
/// The return type `Box<dyn Screen>` is a *trait object*: each `match` arm
/// produces a different concrete type (`screens::Menu`, `pong::PongGame`, …),
/// so the concrete type is erased behind a pointer to the `Screen` trait with
/// vtable dispatch.  `Box` owns the heap allocation.
///
/// The `match` is exhaustive — the compiler forces every `ScreenId` variant
/// to be handled, preventing silent fall-throughs the way C `switch` cannot.
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

/// Window configuration passed to the `#[macroquad::main]` attribute macro.
///
/// `..Default::default()` is *struct update syntax*: every field not listed
/// above receives its default value — a concise alternative to spelling out
/// every field of `Conf` explicitly.
fn window_conf() -> Conf {
    Conf {
        window_title: "rust-retro-arcade".to_owned(),
        window_width: 640,
        window_height: 480,
        high_dpi: false,
        ..Default::default()
    }
}

/// The game loop — entry point driven by the `#[macroquad::main]` macro.
///
/// `#[macroquad::main(window_conf)]` is an *attribute macro*: it rewrites
/// this `async fn` at compile time, generating a real `fn main()` that boots
/// the macroquad runtime and then drives this coroutine frame by frame.
///
/// `async fn` returns a `Future`.  Macroquad resumes it once per display
/// frame; every `next_frame().await` suspends execution here, yields to the
/// runtime (and to the browser event loop on web), then resumes when the
/// next frame is ready.  That single `.await` is the heartbeat of the loop.
#[macroquad::main(window_conf)]
async fn main() {
    let audio = audio::Audio::load().await;
    let scores = scores::load();

    // Wrap `Ctx` in `Rc<RefCell<…>>` so multiple screens can share it.
    // `Rc::new(RefCell::new(value))` is the standard construction idiom —
    // the two wrappers are applied inside-out around the initial value.
    let ctx: SharedCtx = Rc::new(RefCell::new(Ctx {
        audio,
        scores,
        last_result: None,
    }));

    // `ctx.clone()` increments the reference count only — both `ctx` and
    // `factory_ctx` point to the **same** `RefCell<Ctx>` on the heap.
    // No data is copied; only the smart-pointer handle is duplicated.
    let factory_ctx = ctx.clone();

    // A *closure* that captures `factory_ctx` by `move` (takes ownership of
    // it into the closure's environment).  Its inferred type is
    // `impl Fn(ScreenId) -> Box<dyn Screen>`, satisfying the generic `Fn`
    // bound expected by `App::new`.  Each call clones `factory_ctx` again,
    // handing a fresh `Rc` handle to each newly created screen.
    let mut app = App::new(
        move |id| make_screen(id, factory_ctx.clone()),
        ScreenId::Menu,
    );

    loop {
        let input = Input::poll();
        let dt = get_frame_time();
        app.update(&input, dt);
        if app.quit {
            break;
        }

        // The game always draws in a fixed 640x480 space. A Camera2D maps that
        // space into a centered, aspect-correct viewport of the real window
        // (which may be any size, especially on web), letterboxing the rest.
        // This works on WebGL1 (no render target / WebGL2 needed).
        clear_background(BLACK);
        let scale = (screen_width() / theme::VIRTUAL_W).min(screen_height() / theme::VIRTUAL_H);
        let vw = theme::VIRTUAL_W * scale;
        let vh = theme::VIRTUAL_H * scale;
        let vx = (screen_width() - vw) / 2.0;
        let vy = (screen_height() - vh) / 2.0;

        // `Camera2D::from_display_rect` returns a value; we bind it `mut` so
        // individual fields can be patched before passing to `set_camera`.
        let mut cam =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, theme::VIRTUAL_W, theme::VIRTUAL_H));
        // from_display_rect flips Y (intended for render targets); un-flip for
        // direct-to-screen so our top-left-origin draw code renders upright.
        cam.zoom.y = -cam.zoom.y;
        // `Some((…))` wraps a pixel-rectangle tuple in `Option`, telling
        // macroquad which sub-rectangle of the window to render into.
        cam.viewport = Some((vx as i32, vy as i32, vw as i32, vh as i32));
        set_camera(&cam);

        draw_rectangle(0.0, 0.0, theme::VIRTUAL_W, theme::VIRTUAL_H, theme::BG);
        app.draw();

        set_default_camera();
        next_frame().await;
    }

    // Loop exited because `app.quit` was set.  `.borrow()` asks `RefCell` for
    // a shared `&Ctx` reference, checked at run time (panics if a `&mut` is
    // still live), then reads `scores` and persists them to disk.
    scores::save(&ctx.borrow().scores);
}
