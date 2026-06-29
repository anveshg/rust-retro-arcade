//! The `Instructions` screen — shows game controls and returns to the menu
//! on any key press.
//!
//! Teaches: unit structs, `#[derive(Default)]`, and why a `new()` constructor
//! exists alongside `Default`.  The `Screen` trait and its three methods are
//! documented fully in `menu.rs`; only brief references appear here.

use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use macroquad::prelude::*;

/// A *unit struct* — a struct with no fields at all.
///
/// Unit structs occupy zero bytes at runtime.  They are useful when a type
/// needs its own identity (so it can implement traits and be named distinctly)
/// but carries no data.  The single semicolon after the name is the entire
/// struct body.
///
/// `#[derive(Default)]` asks the compiler to auto-generate a `Default` impl.
/// For a unit struct, `Default::default()` simply returns `Instructions` —
/// the only value the type has.  This satisfies any `Default` bound and
/// lets the type be used in generic contexts that require it.
#[derive(Default)]
pub struct Instructions;

impl Instructions {
    /// Constructs an `Instructions` value.
    ///
    /// For a unit struct this is equivalent to writing the literal `Instructions`,
    /// but providing a `new()` function is idiomatic: calling code can stay
    /// uniform across all screens, including those (like `Menu`) that do real
    /// initialisation work inside `new`.
    pub fn new() -> Self {
        Instructions
    }
}

/// Implements `Screen` for `Instructions`.
///
/// See `menu.rs` for a full explanation of what `impl Screen for T` means,
/// why `update` returns `Option<Transition>`, and what `&mut self` vs `&self`
/// signals about mutability.
impl Screen for Instructions {
    /// Returns `Some(Goto(Menu))` on any key press; `None` to stay on screen.
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.any_pressed || input.escape || input.enter {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }

    /// Draws the instructions text.  `&self` suffices — no state changes here.
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

    /// Returns `ScreenId::Instructions` so the router knows which screen is active.
    fn id(&self) -> ScreenId {
        ScreenId::Instructions
    }
}
