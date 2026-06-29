//! # `paddle` — Paddle state and collision helpers
//!
//! Defines the [`Paddle`] struct used for both the human-controlled and
//! AI-controlled sides. Demonstrates `&self` read-only borrows, `&mut self`
//! mutation, and passing a reference to another struct (`&Ball`) as an
//! argument — all with no runtime overhead.

use crate::pong::ball::Ball;

/// A Pong paddle: a rectangle described by position, dimensions, and speed.
///
/// All fields are `pub` so the game loop in `mod.rs` can read them directly.
/// Mutation goes through `&mut self` methods to keep physics logic in one
/// place rather than scattered across the game loop.
pub struct Paddle {
    /// Left edge x-coordinate in screen pixels.
    pub x: f32,
    /// Top edge y-coordinate in screen pixels.
    pub y: f32,
    /// Width in pixels.
    pub w: f32,
    /// Height in pixels.
    pub h: f32,
    /// Maximum movement speed in pixels/second.
    pub speed: f32,
}

impl Paddle {
    /// Create a paddle at `(x, y)` with default size (12 × 80 px) and
    /// speed 320 px/s. Width, height, and speed are fixed constants here;
    /// only position varies between the two sides.
    pub fn new(x: f32, y: f32) -> Self {
        Paddle {
            x,
            y,
            w: 12.0,
            h: 80.0,
            speed: 320.0,
        }
    }

    /// Y-coordinate of the paddle's centre.
    ///
    /// Takes `&self` (read-only borrow) — computing a midpoint does not need
    /// to write any field. A `&self` method can be called even when only a
    /// shared reference to the paddle is available.
    pub fn center_y(&self) -> f32 {
        self.y + self.h / 2.0
    }

    /// Shift the paddle by `dy` pixels, clamped so it stays in
    /// `[top, bottom - self.h]`.
    ///
    /// Uses `&mut self` because it writes `self.y`. `.clamp()` enforces the
    /// boundary in a single expression — no `if`/`else` branches needed.
    pub fn move_by(&mut self, dy: f32, top: f32, bottom: f32) {
        self.y = (self.y + dy).clamp(top, bottom - self.h);
    }

    /// Move toward `target_y`, capped by `speed * dt`, clamped to the field.
    pub fn track(&mut self, target_y: f32, dt: f32, top: f32, bottom: f32) {
        let delta = target_y - self.center_y();
        let max = self.speed * dt;
        let step = delta.clamp(-max, max);
        self.y = (self.y + step).clamp(top, bottom - self.h);
    }

    /// Return `true` if the ball overlaps the paddle rectangle.
    ///
    /// **Borrowing another struct:** `ball: &Ball` is a shared borrow of the
    /// ball. Both `self` (`&Paddle`) and `ball` (`&Ball`) are live shared
    /// borrows at the same time, which Rust allows because neither is `&mut`
    /// — multiple read-only borrows of *different* values are always legal.
    pub fn hits(&self, ball: &Ball) -> bool {
        ball.pos.x - ball.radius <= self.x + self.w
            && ball.pos.x + ball.radius >= self.x
            && ball.pos.y >= self.y
            && ball.pos.y <= self.y + self.h
    }

    /// Hit position relative to paddle center, normalized to [-1, 1].
    pub fn bounce_offset(&self, ball: &Ball) -> f32 {
        ((ball.pos.y - self.center_y()) / (self.h / 2.0)).clamp(-1.0, 1.0)
    }
}

/// Unit tests for paddle movement and collision detection.
///
/// `Ball::new` is available inside this block via `use super::*;`, which
/// re-exports the `Ball` import at the top of this file. Cross-module
/// fixtures work with no extra ceremony in Rust's test harness.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_by_clamps_to_field() {
        let mut p = Paddle::new(10.0, 10.0);
        p.move_by(-100.0, 0.0, 480.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn track_is_capped_by_speed() {
        let mut p = Paddle::new(10.0, 200.0);
        // target far above; dt small so movement is capped to speed*dt = 32.
        p.track(0.0, 0.1, 0.0, 480.0);
        assert_eq!(p.y, 200.0 - 32.0);
    }

    #[test]
    fn bounce_offset_is_zero_at_center() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(20.0, p.center_y(), 0.0, 0.0);
        assert!(p.bounce_offset(&ball).abs() < 0.001);
    }

    #[test]
    fn bounce_offset_is_negative_above_center() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(20.0, p.y, 0.0, 0.0); // top edge
        assert!(p.bounce_offset(&ball) < 0.0);
    }

    #[test]
    fn hits_detects_overlap() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(18.0, 140.0, 0.0, 0.0);
        assert!(p.hits(&ball));
    }

    #[test]
    fn misses_when_ball_is_past_paddle_vertically() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(18.0, 300.0, 0.0, 0.0);
        assert!(!p.hits(&ball));
    }
}
