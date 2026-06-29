//! # `pong` — The Pong game screen
//!
//! Orchestrates a full game of Pong: declares the `ball` and `paddle`
//! sub-modules, defines two private state-machine `enum`s, holds all game
//! state in `PongGame`, and implements the `Screen` trait so the app shell
//! can drive the game frame-by-frame. Good place to study `enum`/`match`,
//! trait implementations, and Rust's runtime interior-mutability pattern.

/// Re-export the `ball` sub-module.
///
/// **`pub mod name;`** does two things at once: (1) it tells the compiler to
/// load `ball.rs` (or `ball/mod.rs`) from disk, and (2) it makes the module
/// visible as `pong::ball` outside this crate. Omitting `pub` would still
/// compile the module but keep it private to `pong`.
pub mod ball;
/// Re-export the `paddle` sub-module (see `ball` above for the `pub mod`
/// explanation).
pub mod paddle;

use crate::app::{Screen, ScreenId, Transition};
use crate::audio::Sfx;
use crate::input::Input;
use crate::theme;
use crate::{GameResult, SharedCtx};
use ball::{reflect, Ball};
use macroquad::math::Vec2;
use macroquad::prelude::*;
use paddle::Paddle;

/// Top boundary of the play field (pixels from the top of the screen).
const TOP: f32 = 30.0;
/// Bottom boundary of the play field in pixels.
const BOTTOM: f32 = 480.0;
/// Score needed to win a match.
const TARGET: u32 = 7;
/// Starting speed of the ball in pixels/second.
const BALL_SPEED: f32 = 300.0;

/// **Testable free function:** placing this logic outside `PongGame::update`
/// makes it unit-testable without constructing a full game struct — a useful
/// Rust pattern for keeping game-loop code lean and independently verifiable.
///
/// Initial ball velocity for a serve toward `dir` (+1 right, -1 left).
pub fn serve_velocity(dir: f32, speed: f32) -> Vec2 {
    Vec2::new(dir * speed, 0.0)
}

/// Which stage of the Pong game loop we are in.
///
/// **Private `enum` as a state machine:** `Phase` is not `pub`, so only code
/// inside this module can construct or match on it. `#[derive(Clone, Copy,
/// PartialEq)]` lets Rust auto-generate cheap copy semantics and `==`
/// comparison — ideal for small tag types that carry no heap data.
#[derive(Clone, Copy, PartialEq)]
enum Phase {
    /// Showing the mode-selection menu before a match starts.
    ModeSelect,
    /// A match is actively in progress.
    Playing,
}

/// Whether the right paddle is driven by a human or the CPU.
///
/// Another private state-machine enum. `match` on `Mode` in `update` replaces
/// an if-ladder, and the compiler errors if any variant is unhandled —
/// exhaustiveness is enforced at compile time, not runtime.
#[derive(Clone, Copy, PartialEq)]
enum Mode {
    /// Right paddle is driven by the AI tracker.
    VsCpu,
    /// Right paddle is driven by player-two arrow-key input.
    TwoPlayer,
}

/// All state owned by one game of Pong.
///
/// `ctx` is a `SharedCtx` — a `Rc<RefCell<...>>` handle shared with the rest
/// of the app. `RefCell` enables *interior mutability*: even through a shared
/// `&self` reference we can borrow the inner value mutably at runtime, with
/// the borrow rule checked dynamically instead of at compile time.
pub struct PongGame {
    /// Shared application context (audio, persistent scores, result slot).
    ctx: SharedCtx,
    /// Current stage: mode-select menu or active play.
    phase: Phase,
    /// Opponent type chosen by the player.
    mode: Mode,
    /// The ball (position + velocity).
    ball: Ball,
    /// Left (player-one) paddle.
    left: Paddle,
    /// Right (player-two or CPU) paddle.
    right: Paddle,
    /// Goals scored by the left player.
    score_l: u32,
    /// Goals scored by the right player.
    score_r: u32,
    /// Direction of the next serve: `+1.0` toward right, `-1.0` toward left.
    serve_dir: f32,
}

impl PongGame {
    /// Construct a fresh `PongGame` starting at the mode-select screen.
    pub fn new(ctx: SharedCtx) -> Self {
        PongGame {
            ctx,
            phase: Phase::ModeSelect,
            mode: Mode::VsCpu,
            ball: Ball::new(320.0, 240.0, BALL_SPEED, 0.0),
            left: Paddle::new(20.0, 200.0),
            right: Paddle::new(608.0, 200.0),
            score_l: 0,
            score_r: 0,
            serve_dir: 1.0,
        }
    }

    /// Respawn the ball at centre moving toward `dir`.
    /// Private (`fn` without `pub`) — only callable within this module.
    fn reset_ball(&mut self, dir: f32) {
        let v = serve_velocity(dir, BALL_SPEED);
        self.ball = Ball::new(320.0, 240.0, v.x, v.y);
    }
}

/// **Trait implementation:** `impl Screen for PongGame` wires this struct into
/// the app shell's polymorphic screen stack. In C++ terms, `Screen` is the
/// abstract base class; `impl Trait for Type` provides the vtable entries.
/// The shell calls `update` and `draw` each frame without knowing the concrete
/// screen type — classic dynamic dispatch via trait objects.
impl Screen for PongGame {
    /// Process one frame of game logic; return a screen transition if needed.
    ///
    /// **`Option<Transition>`** is Rust's null-safe return type: `None` means
    /// "stay on this screen"; `Some(Transition::Goto(id))` tells the shell to
    /// switch screens. No null pointer, no sentinel value needed.
    fn update(&mut self, input: &Input, dt: f32) -> Option<Transition> {
        if input.escape {
            return Some(Transition::Goto(ScreenId::Menu));
        }

        // Phase::ModeSelect: wait for a key press to choose a game mode.
        // Comparing enum variants with == works because Phase derives PartialEq.
        if self.phase == Phase::ModeSelect {
            if input.digit == Some(1) {
                self.mode = Mode::VsCpu;
                self.phase = Phase::Playing;
            } else if input.digit == Some(2) {
                self.mode = Mode::TwoPlayer;
                self.phase = Phase::Playing;
            }
            return None;
        }

        // Left paddle: W/S.
        if input.w {
            self.left.move_by(-self.left.speed * dt, TOP, BOTTOM);
        }
        if input.s {
            self.left.move_by(self.left.speed * dt, TOP, BOTTOM);
        }

        // Right paddle: human (Up/Down) or AI.
        // match is Rust's exhaustive switch: the compiler errors if any Mode
        // variant is unhandled, so adding a new variant forces a code update.
        match self.mode {
            Mode::TwoPlayer => {
                if input.up {
                    self.right.move_by(-self.right.speed * dt, TOP, BOTTOM);
                }
                if input.down {
                    self.right.move_by(self.right.speed * dt, TOP, BOTTOM);
                }
            }
            Mode::VsCpu => {
                self.right.track(self.ball.pos.y, dt, TOP, BOTTOM);
            }
        }

        self.ball.step(dt);
        if self.ball.bounce_walls(TOP, BOTTOM) {
            // borrow() returns a temporary Ref<Ctx> guard that is released at
            // the semicolon — the borrow lasts exactly one statement. Holding
            // a borrow() and a borrow_mut() alive at the same time would panic
            // at runtime, so every ctx access is its own separate statement.
            self.ctx.borrow().audio.play(Sfx::Bounce);
        }

        // Paddle collisions.
        if self.ball.vel.x < 0.0 && self.left.hits(&self.ball) {
            let off = self.left.bounce_offset(&self.ball);
            self.ball.vel = reflect(BALL_SPEED, off, 1.0);
            self.ball.pos.x = self.left.x + self.left.w + self.ball.radius;
            self.ctx.borrow().audio.play(Sfx::Bounce);
        }
        if self.ball.vel.x > 0.0 && self.right.hits(&self.ball) {
            let off = self.right.bounce_offset(&self.ball);
            self.ball.vel = reflect(BALL_SPEED, off, -1.0);
            self.ball.pos.x = self.right.x - self.ball.radius;
            self.ctx.borrow().audio.play(Sfx::Bounce);
        }

        // Scoring.
        if self.ball.pos.x < 0.0 {
            self.score_r += 1;
            self.ctx.borrow().audio.play(Sfx::Score);
            self.serve_dir = -1.0;
            self.reset_ball(self.serve_dir);
        } else if self.ball.pos.x > 640.0 {
            self.score_l += 1;
            self.ctx.borrow().audio.play(Sfx::Score);
            self.serve_dir = 1.0;
            self.reset_ball(self.serve_dir);
        }

        // Win check.
        if self.score_l >= TARGET || self.score_r >= TARGET {
            let player_won = self.score_l >= TARGET;
            self.ctx
                .borrow()
                .audio
                .play(if player_won { Sfx::Win } else { Sfx::Death });
            let title = if self.mode == Mode::TwoPlayer {
                if player_won {
                    "LEFT PLAYER WINS"
                } else {
                    "RIGHT PLAYER WINS"
                }
            } else if player_won {
                "YOU WIN"
            } else {
                "CPU WINS"
            };
            if self.mode == Mode::VsCpu && player_won {
                // borrow_mut() gives exclusive write access to the inner Ctx.
                // This is a separate statement from the borrow() calls above,
                // so no two borrows overlap — the RefCell runtime check passes.
                self.ctx.borrow_mut().scores.record_pong_win();
            }
            // Build a GameResult and store it via borrow_mut(); the GameOver
            // screen reads it after the transition. Some(Transition::Goto(...))
            // returned below hands control back to the shell — analogous to
            // posting an event or invoking a callback.
            self.ctx.borrow_mut().last_result = Some(GameResult {
                title: title.to_string(),
                score: self.score_l.max(self.score_r),
                subtitle: format!("{} - {}", self.score_l, self.score_r),
            });
            let snapshot = self.ctx.borrow().scores;
            crate::scores::save(&snapshot);
            return Some(Transition::Goto(ScreenId::GameOver));
        }

        None
    }

    /// Render the current frame.
    ///
    /// Takes `&self` (read-only) — drawing never mutates game state. The
    /// update/draw split is intentional: keeping mutation out of `draw` means
    /// the two can be called in any order without introducing subtle bugs.
    fn draw(&self) {
        if self.phase == Phase::ModeSelect {
            draw_text("PONG", 270.0, 150.0, 56.0, theme::ACCENT);
            draw_text("Press 1 for vs CPU", 210.0, 250.0, 30.0, theme::TEXT);
            draw_text("Press 2 for two players", 195.0, 300.0, 30.0, theme::TEXT);
            draw_text("Esc = menu", 260.0, 380.0, 22.0, theme::TEXT);
            return;
        }
        // Net.
        let mut y = TOP;
        while y < BOTTOM {
            draw_rectangle(318.0, y, 4.0, 16.0, theme::TEXT);
            y += 28.0;
        }
        draw_rectangle(
            self.left.x,
            self.left.y,
            self.left.w,
            self.left.h,
            theme::PACMAN,
        );
        draw_rectangle(
            self.right.x,
            self.right.y,
            self.right.w,
            self.right.h,
            theme::GHOST_B,
        );
        draw_circle(
            self.ball.pos.x,
            self.ball.pos.y,
            self.ball.radius,
            theme::TEXT,
        );
        draw_text(
            &self.score_l.to_string() as &str,
            270.0,
            24.0,
            30.0,
            theme::PACMAN,
        );
        draw_text(
            &self.score_r.to_string() as &str,
            360.0,
            24.0,
            30.0,
            theme::GHOST_B,
        );
    }

    /// Identify this screen to the app shell.
    fn id(&self) -> ScreenId {
        ScreenId::Pong
    }
}

/// Tests for the pure free functions in this module.
///
/// `PongGame` itself is not tested here because it requires a `SharedCtx`
/// and a running macroquad window. Extracting testable logic into free
/// functions (like `serve_velocity`) is the idiomatic Rust pattern for making
/// game-loop code unit-testable without mocking an entire framework.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serve_goes_right_for_positive_dir() {
        let v = serve_velocity(1.0, 300.0);
        assert_eq!(v.x, 300.0);
        assert_eq!(v.y, 0.0);
    }

    #[test]
    fn serve_goes_left_for_negative_dir() {
        let v = serve_velocity(-1.0, 300.0);
        assert!(v.x < 0.0);
    }
}
