pub mod ball;
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

const TOP: f32 = 30.0;
const BOTTOM: f32 = 480.0;
const TARGET: u32 = 7;
const BALL_SPEED: f32 = 300.0;

/// Initial ball velocity for a serve toward `dir` (+1 right, -1 left).
pub fn serve_velocity(dir: f32, speed: f32) -> Vec2 {
    Vec2::new(dir * speed, 0.0)
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    ModeSelect,
    Playing,
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    VsCpu,
    TwoPlayer,
}

pub struct PongGame {
    ctx: SharedCtx,
    phase: Phase,
    mode: Mode,
    ball: Ball,
    left: Paddle,
    right: Paddle,
    score_l: u32,
    score_r: u32,
    serve_dir: f32,
}

impl PongGame {
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

    fn reset_ball(&mut self, dir: f32) {
        let v = serve_velocity(dir, BALL_SPEED);
        self.ball = Ball::new(320.0, 240.0, v.x, v.y);
    }
}

impl Screen for PongGame {
    fn update(&mut self, input: &Input, dt: f32) -> Option<Transition> {
        if input.escape {
            return Some(Transition::Goto(ScreenId::Menu));
        }

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
                self.ctx.borrow_mut().scores.record_pong_win();
            }
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

    fn id(&self) -> ScreenId {
        ScreenId::Pong
    }
}

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
