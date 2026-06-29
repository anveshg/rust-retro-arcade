pub mod ghost;
pub mod maze;
pub mod player;

use crate::app::{Screen, ScreenId, Transition};
use crate::audio::Sfx;
use crate::input::Input;
use crate::theme;
use crate::{GameResult, SharedCtx};
use ghost::Ghost;
use macroquad::prelude::*;
use maze::{Maze, MAZE};
use player::Player;

const PELLET_SCORE: u32 = 10;
const ORIGIN_X: f32 = 130.0;
const ORIGIN_Y: f32 = 40.0;
const TILE: f32 = 20.0;

/// Time bonus: 200 points minus 2 per elapsed second, floored at 0.
pub fn level_bonus(seconds: f32) -> u32 {
    let penalty = (seconds * 2.0) as i64;
    (200i64 - penalty).max(0) as u32
}

fn move_interval(level: u32) -> f32 {
    match level {
        1 => 0.18,
        2 => 0.13,
        3 => 0.09,
        _ => 0.06,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    SpeedSelect,
    Playing,
}

pub struct PacmanGame {
    ctx: SharedCtx,
    phase: Phase,
    level: u32,
    maze: Maze,
    player: Player,
    ghosts: Vec<Ghost>,
    score: u32,
    lives: u32,
    elapsed: f32,
    move_timer: f32,
    ghost_timer: f32,
}

impl PacmanGame {
    pub fn new(ctx: SharedCtx) -> Self {
        let maze = Maze::from_ascii(MAZE);
        let (pc, pr) = maze.player_start;
        let ghosts = maze
            .ghost_starts
            .iter()
            .map(|&(c, r)| Ghost::new(c, r))
            .collect();
        PacmanGame {
            ctx,
            phase: Phase::SpeedSelect,
            level: 1,
            player: Player::new(pc, pr),
            ghosts,
            maze,
            score: 0,
            lives: 3,
            elapsed: 0.0,
            move_timer: 0.0,
            ghost_timer: 0.0,
        }
    }

    fn reset_positions(&mut self) {
        let (pc, pr) = self.maze.player_start;
        self.player = Player::new(pc, pr);
        for (g, &(c, r)) in self.ghosts.iter_mut().zip(self.maze.ghost_starts.iter()) {
            g.col = c;
            g.row = r;
            g.dir = (0, 0);
        }
    }

    fn tile_center(&self, col: i32, row: i32) -> (f32, f32) {
        (
            ORIGIN_X + col as f32 * TILE + TILE / 2.0,
            ORIGIN_Y + row as f32 * TILE + TILE / 2.0,
        )
    }

    fn lose_life_or_end(&mut self) -> Option<Transition> {
        self.lives -= 1;
        self.ctx.borrow().audio.play(Sfx::Death);
        if self.lives == 0 {
            let is_best = self.ctx.borrow_mut().scores.record_pacman(self.score);
            self.ctx.borrow_mut().last_result = Some(GameResult {
                title: "GAME OVER".to_string(),
                score: self.score,
                subtitle: if is_best {
                    "New best score!".to_string()
                } else {
                    String::new()
                },
            });
            return Some(Transition::Goto(ScreenId::GameOver));
        }
        self.reset_positions();
        None
    }
}

impl Screen for PacmanGame {
    fn update(&mut self, input: &Input, dt: f32) -> Option<Transition> {
        if input.escape {
            return Some(Transition::Goto(ScreenId::Menu));
        }

        if self.phase == Phase::SpeedSelect {
            if let Some(d) = input.digit {
                self.level = d as u32;
                self.phase = Phase::Playing;
            }
            return None;
        }

        self.elapsed += dt;
        self.move_timer += dt;
        self.ghost_timer += dt;
        let interval = move_interval(self.level);

        // Player steps on its own cadence.
        if self.move_timer >= interval {
            self.move_timer -= interval;
            let (_, ate) = self.player.advance(&mut self.maze, input.dir4());
            if ate {
                self.score += PELLET_SCORE;
                self.ctx.borrow().audio.play(Sfx::Chomp);
            }
        }

        // Ghosts step slightly slower than the player.
        if self.ghost_timer >= interval * 1.15 {
            self.ghost_timer -= interval * 1.15;
            let target = (self.player.col, self.player.row);
            for g in &mut self.ghosts {
                g.step(&self.maze, target);
            }
        }

        // Collision (check after movement).
        if self
            .ghosts
            .iter()
            .any(|g| g.touches(self.player.col, self.player.row))
        {
            if let Some(t) = self.lose_life_or_end() {
                return Some(t);
            }
        }

        // Win: maze cleared.
        if self.maze.cleared() {
            let bonus = level_bonus(self.elapsed);
            self.score += bonus;
            self.ctx.borrow().audio.play(Sfx::Win);
            let is_best = self.ctx.borrow_mut().scores.record_pacman(self.score);
            self.ctx.borrow_mut().last_result = Some(GameResult {
                title: "MAZE CLEARED".to_string(),
                score: self.score,
                subtitle: if is_best {
                    "New best score!".to_string()
                } else {
                    format!("Time bonus {}", bonus)
                },
            });
            return Some(Transition::Goto(ScreenId::GameOver));
        }

        None
    }

    fn draw(&self) {
        if self.phase == Phase::SpeedSelect {
            draw_text("PAC-MAN", 230.0, 150.0, 52.0, theme::ACCENT);
            draw_text("Choose speed: 1  2  3  4", 180.0, 250.0, 30.0, theme::TEXT);
            draw_text(
                "(1 = slow, 4 = fast)   Esc = menu",
                150.0,
                300.0,
                24.0,
                theme::TEXT,
            );
            return;
        }

        // Walls and pellets.
        for r in 0..self.maze.rows {
            for c in 0..self.maze.cols {
                let x = ORIGIN_X + c as f32 * TILE;
                let y = ORIGIN_Y + r as f32 * TILE;
                if self.maze.is_wall(c, r) {
                    draw_rectangle(x, y, TILE, TILE, theme::WALL);
                } else if self.maze.pellet_at(c, r) {
                    draw_circle(x + TILE / 2.0, y + TILE / 2.0, 2.5, theme::PELLET);
                }
            }
        }

        // Player.
        let (px, py) = self.tile_center(self.player.col, self.player.row);
        draw_circle(px, py, TILE / 2.0 - 2.0, theme::PACMAN);

        // Ghosts.
        for (i, g) in self.ghosts.iter().enumerate() {
            let (gx, gy) = self.tile_center(g.col, g.row);
            let color = if i % 2 == 0 {
                theme::GHOST_A
            } else {
                theme::GHOST_B
            };
            draw_circle(gx, gy, TILE / 2.0 - 2.0, color);
        }

        // HUD.
        let score_text = format!("Score {}", self.score);
        draw_text(&score_text, 10.0, 20.0, 24.0, theme::TEXT);
        let lives_text = format!("Lives {}", self.lives);
        draw_text(&lives_text, 250.0, 20.0, 24.0, theme::TEXT);
        let lv_text = format!("Lv {}", self.level);
        draw_text(&lv_text, 420.0, 20.0, 24.0, theme::TEXT);
        let time_text = format!("Time {:.0}", self.elapsed);
        draw_text(&time_text, 520.0, 20.0, 24.0, theme::TEXT);
    }

    fn id(&self) -> ScreenId {
        ScreenId::Pacman
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus_is_full_at_zero_seconds() {
        assert_eq!(level_bonus(0.0), 200);
    }

    #[test]
    fn bonus_decreases_over_time() {
        assert_eq!(level_bonus(10.0), 180);
    }

    #[test]
    fn bonus_floors_at_zero() {
        assert_eq!(level_bonus(1000.0), 0);
    }
}
