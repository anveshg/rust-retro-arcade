//! The `Credits` screen — a static splash that names the author.
//!
//! Structurally identical to `instructions.rs`; see that file for the
//! commentary on unit structs and `#[derive(Default)]`.

use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use macroquad::prelude::*;

/// Unit struct for the credits screen.
///
/// Zero-sized — all displayed text is hard-coded in `draw`.
/// `#[derive(Default)]` is included so `Credits` remains consistent with the
/// other simple screens and can satisfy a `Default` bound if needed.
/// See `instructions.rs` for the full explanation of unit structs and
/// `#[derive(Default)]`.
#[derive(Default)]
pub struct Credits;

impl Credits {
    /// Constructs a `Credits` value.
    ///
    /// Like `Instructions::new`, this exists for API uniformity across all
    /// screens rather than for any real initialisation work.
    pub fn new() -> Self {
        Credits
    }
}

/// Implements `Screen` for `Credits`.  Refer to `menu.rs` for the full
/// breakdown of the trait, its methods, and the receiver types.
impl Screen for Credits {
    /// Returns to the menu on any key press.
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.any_pressed || input.escape || input.enter {
            return Some(Transition::Goto(ScreenId::Menu));
        }
        None
    }

    /// Draws the credits.  No mutable state is needed, so the receiver is `&self`.
    fn draw(&self) {
        draw_text("CREDITS", 220.0, 120.0, 48.0, theme::ACCENT);
        draw_text("ANVESH", 240.0, 230.0, 56.0, theme::PACMAN);
        draw_text(
            "Rust rewrite of an old DOS C++ project",
            110.0,
            320.0,
            24.0,
            theme::TEXT,
        );
        draw_text("Press any key to return", 200.0, 420.0, 22.0, theme::TEXT);
    }

    /// Identifies this screen as `ScreenId::Credits` to the router.
    fn id(&self) -> ScreenId {
        ScreenId::Credits
    }
}
