#![allow(dead_code)]

use macroquad::prelude::Color;

pub const VIRTUAL_W: f32 = 640.0;
pub const VIRTUAL_H: f32 = 480.0;

pub const BG: Color = Color {
    r: 0.04,
    g: 0.04,
    b: 0.08,
    a: 1.0,
};
pub const WALL: Color = Color {
    r: 0.16,
    g: 0.20,
    b: 0.85,
    a: 1.0,
};
pub const PELLET: Color = Color {
    r: 1.0,
    g: 0.85,
    b: 0.40,
    a: 1.0,
};
pub const PACMAN: Color = Color {
    r: 1.0,
    g: 0.92,
    b: 0.0,
    a: 1.0,
};
pub const GHOST_A: Color = Color {
    r: 1.0,
    g: 0.25,
    b: 0.25,
    a: 1.0,
};
pub const GHOST_B: Color = Color {
    r: 0.30,
    g: 0.90,
    b: 1.0,
    a: 1.0,
};
pub const TEXT: Color = Color {
    r: 0.90,
    g: 0.95,
    b: 1.0,
    a: 1.0,
};
pub const ACCENT: Color = Color {
    r: 0.20,
    g: 0.90,
    b: 0.50,
    a: 1.0,
};

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
