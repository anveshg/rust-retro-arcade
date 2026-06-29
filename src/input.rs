//! Input snapshot for one game frame, built from macroquad's keyboard API.
//!
//! Rust concepts demonstrated: `derive` macros (`Default`, `Clone`, `Copy`),
//! `Option<T>` for "value that may be absent", tuple return types, associated
//! functions vs methods, struct update syntax, and in-file unit tests.

use macroquad::prelude::*;

/// A plain-data snapshot of which keys are active this frame.
///
/// `#[derive(Default, Clone, Copy)]` asks the compiler to generate three trait
/// implementations automatically — no hand-written boilerplate needed:
///
/// * `Default` — lets you write `Input::default()` to get all-`false`/`None`
///   fields without listing every one. Rust requires every field to have a
///   known default; `bool` defaults to `false`, `Option<_>` to `None`.
/// * `Clone` — provides `.clone()` for an explicit deep copy.
/// * `Copy` — a marker that tells the compiler this type is cheap to duplicate
///   by simply memcpy-ing its bytes. Once a type is `Copy`, assigning it to
///   a new variable does NOT move it — it silently copies instead. `Clone`
///   is the explicit/manual version; `Copy` makes it automatic. A type can
///   only be `Copy` if ALL its fields are `Copy` too (`bool`, `u8`, and
///   `Option<u8>` all qualify here).
///
/// All fields are `pub` so callers in other modules can read them directly
/// without getters — appropriate for a plain-data "value object".
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
    /// `Option<u8>` means "either `Some(digit)` or `None`" — Rust's type-safe
    /// alternative to a nullable value or a sentinel like `-1`. The compiler
    /// forces callers to handle both cases before using the inner value.
    pub digit: Option<u8>,
    pub any_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
}

impl Input {
    /// Reads the keyboard once and returns a fresh `Input` snapshot.
    ///
    /// This is an *associated function* (sometimes called a static method in
    /// C++/Python): it has no `self` parameter and is called as `Input::poll()`
    /// rather than `instance.poll()`. It acts like a constructor or factory.
    ///
    /// The return type `Self` is an alias for the type being implemented
    /// (`Input`), so `-> Self` here means `-> Input`.
    pub fn poll() -> Self {
        // Build an Option<u8> with a chain of if/else if/else expressions.
        // In Rust, `if/else` is an *expression* — every branch must produce
        // the same type, and the last value in a block is returned without
        // a semicolon.  `None` is the "no digit pressed" branch.
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
        // Struct literal: every field is named explicitly.  `digit` on its own
        // is shorthand for `digit: digit` — allowed when the local variable
        // name matches the field name exactly ("field init shorthand").
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
            up_pressed: is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W),
            down_pressed: is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S),
        }
    }

    /// Grid direction from arrows OR WASD. Vertical wins when both axes are held.
    ///
    /// Takes `&self` (a shared/immutable reference) because reading fields
    /// never needs to mutate the struct. The return type `(i32, i32)` is a
    /// *tuple* — a fixed-length, anonymous composite type. Callers destructure
    /// it with `let (dx, dy) = input.dir4();`.
    pub fn dir4(&self) -> (i32, i32) {
        // `||` short-circuits: the right-hand side is only evaluated if the
        // left is false — identical to C++/JS/Python behaviour.
        let up = self.up || self.w;
        let down = self.down || self.s;
        let left = self.left || self.a;
        let right = self.right || self.d;
        // Early `return` exits the function immediately, just like C++/Python.
        // Vertical checks run first so they win over any simultaneous horizontal.
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
        // The final expression (no semicolon) is the implicit return value
        // when no key is pressed or opposing keys cancel each other out.
        (0, 0)
    }
}

/// Unit tests for `Input`. `#[cfg(test)]` tells the compiler to include this
/// module only when running `cargo test` — it is stripped from release builds.
/// `use super::*;` imports everything from the parent module so tests can
/// reference `Input` directly.
#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms that the `up` field maps to the negative-Y grid direction.
    ///
    /// `..Default::default()` is *struct update syntax*: it fills every field
    /// not listed above it with that field's default value. Here every `bool`
    /// becomes `false` and `digit` becomes `None`, so only `up` is `true`.
    #[test]
    fn up_arrow_maps_to_negative_y() {
        let i = Input {
            up: true,
            ..Default::default()
        };
        assert_eq!(i.dir4(), (0, -1));
    }

    /// Confirms that the `d` (WASD right) field maps to positive-X.
    #[test]
    fn wasd_d_maps_to_positive_x() {
        let i = Input {
            d: true,
            ..Default::default()
        };
        assert_eq!(i.dir4(), (1, 0));
    }

    /// Confirms that holding both `left` and `right` simultaneously cancels to (0, 0).
    #[test]
    fn opposite_horizontals_cancel() {
        let i = Input {
            left: true,
            right: true,
            ..Default::default()
        };
        assert_eq!(i.dir4(), (0, 0));
    }

    /// Confirms vertical direction wins when a vertical and horizontal key are both held.
    #[test]
    fn vertical_has_priority_over_horizontal() {
        let i = Input {
            up: true,
            right: true,
            ..Default::default()
        };
        assert_eq!(i.dir4(), (0, -1));
    }
}
