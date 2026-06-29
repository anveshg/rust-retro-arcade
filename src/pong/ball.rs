//! # `ball` — Ball physics for Pong
//!
//! Defines the [`Ball`] struct and its movement/collision methods, plus the
//! free function [`reflect`] that computes a post-paddle velocity. This module
//! introduces `pub struct` with `pub` fields, `&self`/`&mut self` borrows,
//! `Vec2` arithmetic, and `#[cfg(test)]` unit tests.

use macroquad::math::Vec2;

/// The Pong ball: position, velocity, and radius.
///
/// **Rust concept — `pub struct` with `pub` fields:** In C++ you'd write a
/// class with public members; here `pub struct` declares the type visible
/// outside this module, and each `pub` field is individually accessible.
/// Fields without `pub` would be private to this module.
///
/// **`Vec2`** is macroquad's two-component float vector (`x`, `y`). It
/// supports `+`, `-`, scalar `*`, and `+=` in-place — making physics
/// arithmetic concise without any extra boilerplate.
pub struct Ball {
    /// Centre position in screen pixels.
    pub pos: Vec2,
    /// Velocity in pixels/second. Multiplied by `dt` each frame in `step`.
    pub vel: Vec2,
    /// Collision radius in pixels.
    pub radius: f32,
}

impl Ball {
    /// Create a ball centred at `(cx, cy)` with velocity `(vx, vy)`.
    ///
    /// **Rust note:** There is no `new` keyword; by convention a static
    /// constructor is a plain function called `new`. `Self` inside an `impl`
    /// block is an alias for the enclosing type (`Ball` here).
    pub fn new(cx: f32, cy: f32, vx: f32, vy: f32) -> Self {
        Ball {
            pos: Vec2::new(cx, cy),
            vel: Vec2::new(vx, vy),
            radius: 8.0,
        }
    }

    /// Advance the ball by one frame.
    ///
    /// **`&mut self` vs `&self`:** `&self` is a shared, read-only borrow —
    /// like `const T*` in C++. `&mut self` is an exclusive, writable borrow —
    /// like `T*`. The compiler statically guarantees nothing else holds `self`
    /// while this method runs, so data races are impossible without any locks.
    ///
    /// `self.pos += self.vel * dt` is `Vec2` operator overloading: scalar-
    /// multiply the velocity by `dt` (seconds elapsed this frame), then add
    /// the result into `pos` in-place — one-line Euler integration.
    pub fn step(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }

    /// **Returns `bool`:** Many Rust methods return `bool` or `Option<T>` to
    /// signal what happened, replacing C++-style out-parameters. The return
    /// value can be used directly in an `if` expression (as in `mod.rs`).
    ///
    /// Reflect off the top/bottom walls. Returns true if a wall was hit.
    pub fn bounce_walls(&mut self, top: f32, bottom: f32) -> bool {
        if self.pos.y - self.radius <= top && self.vel.y < 0.0 {
            self.pos.y = top + self.radius;
            self.vel.y = -self.vel.y;
            return true;
        }
        if self.pos.y + self.radius >= bottom && self.vel.y > 0.0 {
            self.pos.y = bottom - self.radius;
            self.vel.y = -self.vel.y;
            return true;
        }
        false
    }
}

/// **Free function (not a method):** `reflect` lives outside any `impl` block
/// so it has no `self`. Rust encourages standalone functions when the logic
/// does not belong to one specific type — the reflection formula is neither
/// `Ball` behaviour nor `Paddle` behaviour, so a free function is clearest.
///
/// **`.clamp(lo, hi)`** pins a value to `[lo, hi]`. It is a standard method
/// on all Rust numeric types — no external crate or helper needed.
///
/// New velocity after a paddle hit.
/// `offset` in [-1,1] (paddle-relative hit position), `dir` is +1 (rightward) or -1 (leftward).
pub fn reflect(speed: f32, offset: f32, dir: f32) -> Vec2 {
    let max_angle = std::f32::consts::FRAC_PI_3; // 60 degrees
    let angle = offset.clamp(-1.0, 1.0) * max_angle;
    Vec2::new(dir * speed * angle.cos(), speed * angle.sin())
}

/// Unit tests for ball physics.
///
/// **`#[cfg(test)]`** is a *conditional compilation attribute*: the `mod
/// tests` block is compiled only when running `cargo test`, never in a
/// release build. `use super::*;` imports everything from the parent module
/// into the test scope (similar to `using namespace` but scoped to this mod).
///
/// **Why epsilon comparisons instead of `assert_eq!` for floats?**
/// Floating-point arithmetic is not bit-for-bit reproducible across different
/// expression orderings, so `(a - b).abs() < 0.001` is the idiomatic guard.
/// The only safe use of `==` on `f32`/`f64` is when the value was stored
/// directly from an exact integer source with no arithmetic applied.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_advances_by_velocity_times_dt() {
        let mut b = Ball::new(100.0, 100.0, 50.0, 0.0);
        b.step(1.0);
        assert_eq!(b.pos.x, 150.0);
        assert_eq!(b.pos.y, 100.0);
    }

    #[test]
    fn bounces_off_top_wall() {
        let mut b = Ball::new(100.0, 2.0, 0.0, -30.0);
        let hit = b.bounce_walls(0.0, 480.0);
        assert!(hit);
        assert!(b.vel.y > 0.0);
        assert_eq!(b.pos.y, b.radius);
    }

    #[test]
    fn no_bounce_when_moving_away_from_wall() {
        let mut b = Ball::new(100.0, 2.0, 0.0, 30.0);
        assert!(!b.bounce_walls(0.0, 480.0));
    }

    #[test]
    fn reflect_center_goes_straight() {
        let v = reflect(300.0, 0.0, 1.0);
        assert!((v.x - 300.0).abs() < 0.001);
        assert!(v.y.abs() < 0.001);
    }

    #[test]
    fn reflect_direction_sign_is_respected() {
        let v = reflect(300.0, 0.0, -1.0);
        assert!(v.x < 0.0);
    }

    #[test]
    fn reflect_edge_hit_adds_vertical_speed() {
        let v = reflect(300.0, 1.0, 1.0);
        assert!(v.y > 0.0);
        assert!(v.x > 0.0);
    }
}
