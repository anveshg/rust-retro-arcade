//! The main `Menu` screen — the hub of the arcade.
//!
//! Demonstrates: a struct holding shared state, implementing a trait,
//! fixed-size const arrays, wrap-around modular arithmetic, `Option` returns,
//! and reading from a `RefCell`-wrapped shared context via `.borrow()`.

use crate::app::{Screen, ScreenId, Transition};
use crate::input::Input;
use crate::theme;
use crate::SharedCtx;
use macroquad::prelude::*;

/// The ordered list of menu entries displayed on screen.
///
/// `const` in Rust is evaluated at compile time (like `constexpr` in C++).
/// `[&str; 5]` is a *fixed-size array* type — the length is baked into the
/// type itself, so the compiler rejects an array literal of a different length.
/// `&str` is a string *slice* (a fat pointer into static UTF-8 bytes); it
/// borrows the string data rather than owning it, making it free to store here.
const ITEMS: [&str; 5] = ["Pac-Man", "Pong", "Instructions", "Credits", "Quit"];

/// Maps a 0-based menu index to the [`Transition`] that index should trigger.
///
/// `usize` is Rust's platform-sized unsigned integer used for all indices and
/// lengths.  Returning `Transition` (not `Option<Transition>`) keeps callers
/// simple — the `_` wildcard arm handles any out-of-range index as a Quit so
/// the `match` stays exhaustive without a fallible return type.
/// The `#[cfg(test)]` module below tests this function in isolation.
pub fn menu_target(index: usize) -> Transition {
    match index {
        0 => Transition::Goto(ScreenId::Pacman),
        1 => Transition::Goto(ScreenId::Pong),
        2 => Transition::Goto(ScreenId::Instructions),
        3 => Transition::Goto(ScreenId::Credits),
        _ => Transition::Quit,
    }
}

/// The main menu screen, holding the shared application context and the
/// currently highlighted item index.
///
/// `SharedCtx` is a `Rc<RefCell<AppCtx>>` — a reference-counted,
/// interior-mutable handle.  Storing it here gives `Menu` read access to
/// persistent scores without taking sole ownership of the data.
pub struct Menu {
    ctx: SharedCtx,
    selected: usize,
}

/// Inherent (non-trait) methods on `Menu` — functions specific to this type,
/// not required by any interface.
impl Menu {
    /// Constructs a new `Menu` with the first item highlighted.
    ///
    /// By Rust convention, constructors are free functions named `new` that
    /// return `Self` (a compiler alias for the type being `impl`-ed).  There
    /// is no language-level `new` keyword — it is purely a naming convention.
    pub fn new(ctx: SharedCtx) -> Self {
        Menu { ctx, selected: 0 }
    }
}

/// Implements the `Screen` trait for `Menu`.
///
/// A *trait* in Rust is similar to a pure-virtual interface in C++ or a
/// Protocol in Swift.  Writing `impl Screen for Menu` promises the compiler
/// that `Menu` provides every method declared in `Screen`.  The trait defines
/// three methods:
/// - `update` — called each frame; may signal a screen change via `Transition`.
/// - `draw`   — renders the screen; takes an immutable `&self`.
/// - `id`     — returns the `ScreenId` variant that names this screen.
///
/// After this impl block, the compiler treats `&mut Menu` as `&mut dyn Screen`
/// wherever a trait object is expected.
impl Screen for Menu {
    /// Handles one frame of input and returns an optional screen transition.
    ///
    /// `&mut self` means the method has an *exclusive* borrow of `Menu` —
    /// no other code can read or write the struct while this call is live.
    ///
    /// The return type `Option<Transition>` is idiomatic Rust: `None` means
    /// "stay on this screen"; `Some(t)` hands the router a value to act on.
    /// This replaces a sentinel return value or output parameter from C++.
    ///
    /// The up-arrow expression `(self.selected + ITEMS.len() - 1) % ITEMS.len()`
    /// avoids unsigned underflow: subtracting 1 from a `usize` of 0 would
    /// panic in debug or silently wrap in release, so adding `ITEMS.len()`
    /// first guarantees the value stays positive before the modulus is taken.
    fn update(&mut self, input: &Input, _dt: f32) -> Option<Transition> {
        if input.up_pressed {
            self.selected = (self.selected + ITEMS.len() - 1) % ITEMS.len();
        }
        if input.down_pressed {
            self.selected = (self.selected + 1) % ITEMS.len();
        }
        if let Some(d) = input.digit {
            // `if let` unwraps an `Option` in one step — a null-check and
            // binding in C++ written as two lines becomes one here.  `d` is
            // only bound inside this block.
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

    /// Renders the menu to the framebuffer.
    ///
    /// `&self` is a *shared* (immutable) borrow — the compiler statically
    /// guarantees no field is mutated during the call.
    ///
    /// `self.ctx.borrow()` acquires a `Ref<AppCtx>` guard from the `RefCell`.
    /// The guard enforces the borrow rules at runtime (panics if a mutable
    /// borrow is active simultaneously).  Crucially, the guard is *dropped at
    /// the end of the statement that created it* — not at the end of the block
    /// — so each of the two `.borrow()` calls here releases the lock before the
    /// next line runs, keeping them independent and safe.
    fn draw(&self) {
        draw_text("RUST RETRO ARCADE", 110.0, 80.0, 44.0, theme::ACCENT);
        for (i, label) in ITEMS.iter().enumerate() {
            let y = 170.0 + i as f32 * 46.0;
            let color = if i == self.selected {
                theme::PACMAN
            } else {
                theme::TEXT
            };
            let prefix = if i == self.selected { "> " } else { "  " };
            let item_text = format!("{}{}. {}", prefix, i + 1, label);
            draw_text(&item_text, 220.0, y, 32.0, color);
        }
        let s = self.ctx.borrow().scores;
        let scores_text = format!(
            "Best Pac-Man: {}    Pong wins: {}",
            s.best_pacman, s.pong_wins
        );
        draw_text(&scores_text, 120.0, 440.0, 22.0, theme::TEXT);
    }

    /// Returns the `ScreenId` that uniquely identifies this screen to the
    /// router, which uses it to skip redundant transitions.
    fn id(&self) -> ScreenId {
        ScreenId::Menu
    }
}

/// Unit tests for `menu_target`, compiled only during `cargo test`.
///
/// `#[cfg(test)]` is a *conditional compilation attribute*: the entire module
/// is stripped from release builds, so test helpers never ship.
/// `use super::*` imports everything from the parent module (this file) so the
/// tests can call `menu_target` and reference `Transition`/`ScreenId` directly.
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
