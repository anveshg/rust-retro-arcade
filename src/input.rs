#![allow(dead_code)]

use macroquad::prelude::*;

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
    pub digit: Option<u8>,
    pub any_pressed: bool,
}

impl Input {
    pub fn poll() -> Self {
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
        }
    }

    /// Grid direction from arrows OR WASD. Vertical wins when both axes are held.
    pub fn dir4(&self) -> (i32, i32) {
        let up = self.up || self.w;
        let down = self.down || self.s;
        let left = self.left || self.a;
        let right = self.right || self.d;
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
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn up_arrow_maps_to_negative_y() {
        let i = Input {
            up: true,
            ..Default::default()
        };
        assert_eq!(i.dir4(), (0, -1));
    }

    #[test]
    fn wasd_d_maps_to_positive_x() {
        let i = Input {
            d: true,
            ..Default::default()
        };
        assert_eq!(i.dir4(), (1, 0));
    }

    #[test]
    fn opposite_horizontals_cancel() {
        let i = Input {
            left: true,
            right: true,
            ..Default::default()
        };
        assert_eq!(i.dir4(), (0, 0));
    }

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
