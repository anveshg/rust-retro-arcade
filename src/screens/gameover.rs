//! The `GameOver` screen — displays the result of the last game session.
//!
//! Teaches: exhaustive `match` on `Option` with `Some(r)` / `None` arms,
//! borrowing inside a match arm, and `.as_str()` to convert an owned `String`
//! to a `&str` slice.  For the `borrow()` guard lifetime note, see `menu.rs`.

use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use crate::SharedCtx;
use macroquad::prelude::*;

/// Holds a reference to the shared context so it can read `last_result` when
/// drawing.
///
/// Unlike `Instructions` and `Credits`, this screen needs runtime data, so it
/// is a *named-field struct* rather than a unit struct.  The `ctx` field is a
/// `SharedCtx` (`Rc<RefCell<AppCtx>>`), giving read access without ownership.
pub struct GameOver {
    ctx: SharedCtx,
}

impl GameOver {
    /// Constructs a `GameOver` screen bound to the given shared context.
    ///
    /// Rust's *field-init shorthand* lets us write `GameOver { ctx }` when the
    /// local variable name exactly matches the struct field name — equivalent
    /// to `GameOver { ctx: ctx }` but without the repetition.
    pub fn new(ctx: SharedCtx) -> Self {
        GameOver { ctx }
    }
}

/// Implements `Screen` for `GameOver`.  See `menu.rs` for the full trait
/// explanation.
impl Screen for GameOver {
    /// Returns to the menu on any key press.
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.any_pressed || input.escape || input.enter {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }

    /// Draws the end-of-game summary by reading `last_result` from shared state.
    ///
    /// The `match` on `&ctx.last_result` is the key Rust pattern here.
    /// `last_result` is an `Option<GameResult>`.  Rust's `match` is
    /// *exhaustive*: every variant must be covered or the code will not
    /// compile — there is no accidental fall-through.
    ///
    /// We match on `&ctx.last_result` (a reference) rather than moving the
    /// value out of the borrowed context.  This keeps `ctx` intact and lets the
    /// `Ref` guard drop cleanly.  Inside the `Some(r)` arm, `r` is a
    /// `&GameResult`; calling `r.title.as_str()` converts the owned `String`
    /// field to a `&str` slice valid for the lifetime of `r` — no allocation.
    fn draw(&self) {
        let ctx = self.ctx.borrow();
        // Match on a reference so the Option's contents are borrowed, not moved.
        let (title, score, subtitle) = match &ctx.last_result {
            Some(r) => (r.title.as_str(), r.score, r.subtitle.as_str()),
            None => ("GAME OVER", 0, ""),
        };
        draw_text(title, 180.0, 180.0, 48.0, theme::ACCENT);
        let score_text = format!("Score: {score}");
        draw_text(&score_text, 230.0, 260.0, 36.0, theme::PACMAN);
        draw_text(subtitle, 150.0, 320.0, 26.0, theme::TEXT);
        draw_text(
            "Press any key for the menu",
            180.0,
            420.0,
            22.0,
            theme::TEXT,
        );
    }

    /// Returns `ScreenId::GameOver` to identify this screen to the router.
    fn id(&self) -> ScreenId {
        ScreenId::GameOver
    }
}
