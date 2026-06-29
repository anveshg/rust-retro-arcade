//! Visual constants shared across every screen in the arcade.
//!
//! Rust concepts showcased: `pub const` (evaluated at compile time, inlined
//! at every use site — no runtime stack slot); struct literals with named
//! `pub` fields (`Color { r, g, b, a }`); and `pub` as explicit opt-in
//! visibility (items are private by default in Rust).
use macroquad::prelude::Color;

/// `const` items are evaluated at compile time and inlined at every use site;
/// no runtime stack slot is allocated. `pub` makes this constant visible to
/// any module that imports `theme` — items are private by default in Rust.
///
/// Fixed virtual canvas the whole game draws into; scaled to the real window.
pub const VIRTUAL_W: f32 = 640.0;
/// Height of the virtual canvas in logical pixels.
pub const VIRTUAL_H: f32 = 480.0;

/// Background fill color for all screens.
///
/// `Color { r, g, b, a }` is a **struct literal**: macroquad's `Color` exposes
/// all four `f32` fields as `pub`, so we set them directly by name. Values are
/// in `[0.0, 1.0]`. Using a struct literal in a `const` is valid because every
/// sub-expression is also a compile-time constant.
pub const BG: Color = Color {
    r: 0.04,
    g: 0.04,
    b: 0.08,
    a: 1.0,
};
/// Wall tile color used in the Pac-Man maze.
pub const WALL: Color = Color {
    r: 0.16,
    g: 0.20,
    b: 0.85,
    a: 1.0,
};
/// Dot / pellet color in the Pac-Man level.
pub const PELLET: Color = Color {
    r: 1.0,
    g: 0.85,
    b: 0.40,
    a: 1.0,
};
/// Pac-Man character color.
pub const PACMAN: Color = Color {
    r: 1.0,
    g: 0.92,
    b: 0.0,
    a: 1.0,
};
/// Primary ghost color (Blinky, the red ghost).
pub const GHOST_A: Color = Color {
    r: 1.0,
    g: 0.25,
    b: 0.25,
    a: 1.0,
};
/// Secondary ghost color (Inky, the cyan ghost).
pub const GHOST_B: Color = Color {
    r: 0.30,
    g: 0.90,
    b: 1.0,
    a: 1.0,
};
/// Default UI text color.
pub const TEXT: Color = Color {
    r: 0.90,
    g: 0.95,
    b: 1.0,
    a: 1.0,
};
/// Accent / highlight color for menus and UI elements.
pub const ACCENT: Color = Color {
    r: 0.20,
    g: 0.90,
    b: 0.50,
    a: 1.0,
};
