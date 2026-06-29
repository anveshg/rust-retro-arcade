//! Top-level Pac-Man game screen: orchestrates the maze, player, and ghosts.
//!
//! `PacmanGame` implements the `Screen` trait so the application loop can call
//! `update` each frame and `draw` each render pass.  Two timer accumulators
//! drive the player and ghosts at different rates, independently of frame rate.

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

/// Score awarded for eating one pellet.
const PELLET_SCORE: u32 = 10;
/// Left edge of the maze in screen pixels.
const ORIGIN_X: f32 = 130.0;
/// Top edge of the maze in screen pixels.
const ORIGIN_Y: f32 = 40.0;
/// Side length of one grid tile in screen pixels.
const TILE: f32 = 20.0;

/// Calculate the time-based level-completion bonus.
///
/// **Rust feature – numeric casts and `.max()`:** `as i64` and `as u32` are
/// explicit primitive casts — Rust has no implicit numeric coercions.  The
/// intermediate `i64` lets the subtraction go negative before `.max(0)` clamps
/// it; casting back to `u32` is then safe.  This mirrors C-style casts but
/// forces the programmer to be deliberate about every conversion.
/// Time bonus: 200 points minus 2 per elapsed second, floored at 0.
pub fn level_bonus(seconds: f32) -> u32 {
    let penalty = (seconds * 2.0) as i64;
    (200i64 - penalty).max(0) as u32
}

/// Return the seconds between movement steps for the given difficulty level.
///
/// **Rust feature – `match` as an expression:** Unlike C's `switch`, Rust's
/// `match` is an *expression* that evaluates to a value which can be returned
/// directly.  The wildcard arm `_` covers every level above 3, making the match
/// exhaustive — the compiler rejects any non-exhaustive match on a non-boolean
/// integer type.
fn move_interval(level: u32) -> f32 {
    match level {
        1 => 0.18,
        2 => 0.13,
        3 => 0.09,
        _ => 0.06,
    }
}

/// The two high-level states of the game screen.
///
/// `#[derive(Clone, Copy, PartialEq)]` auto-generates cheap bitwise copying and
/// `==` comparison so the game loop can branch on phase without boilerplate.
#[derive(Clone, Copy, PartialEq)]
enum Phase {
    /// Waiting for the player to press a digit key (1–4) to choose difficulty.
    SpeedSelect,
    /// The maze is active; timers and movement are running.
    Playing,
}

/// All runtime state for one Pac-Man game session.
///
/// **Rust feature – `Vec<Ghost>`:** an owned, growable list of ghosts allocated
/// once in `new` via `.iter().map(…).collect()`.  The count is determined by
/// the ASCII maze at runtime, not by a hard-coded constant.
pub struct PacmanGame {
    /// Shared application context (audio, scores).  `SharedCtx` wraps an
    /// `Rc<RefCell<…>>` so multiple screens can hold a handle; runtime borrow
    /// checking replaces the compile-time checker for this shared mutable state.
    ctx: SharedCtx,
    /// Whether we are waiting for a speed selection or actively playing.
    phase: Phase,
    /// Current difficulty level (1–4), chosen by the player at startup.
    level: u32,
    /// The parsed, live maze — tracks which pellets still remain.
    maze: Maze,
    /// The player character.
    player: Player,
    /// All active ghosts.
    ghosts: Vec<Ghost>,
    /// Cumulative score for this session.
    score: u32,
    /// Remaining lives; game ends when this reaches zero.
    lives: u32,
    /// Total seconds elapsed while `Playing` (used for the time bonus).
    elapsed: f32,
    /// Accumulator for player movement — fires when it exceeds `move_interval`.
    move_timer: f32,
    /// Accumulator for ghost movement — fires at a slightly slower cadence.
    ghost_timer: f32,
}

impl PacmanGame {
    /// Construct a fresh game session tied to the shared application context.
    ///
    /// **Rust feature – `.iter().map(…).collect()`:** this chain lazily applies a
    /// closure to each ghost-start coordinate from the maze, then `collect()`
    /// drives the iterator to completion and allocates the resulting `Vec<Ghost>`.
    /// The output type is inferred from the `ghosts` field of `PacmanGame`.
    pub fn new(ctx: SharedCtx) -> Self {
        let maze = Maze::from_ascii(MAZE);
        let (pc, pr) = maze.player_start;
        // Build the ghost list from maze-defined spawn points.
        // `|&(c, r)|` pattern-destructures the `&(i32, i32)` reference from `.iter()`.
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

    /// Restore the player and every ghost to their spawn positions after a death.
    ///
    /// **Rust feature – `iter_mut().zip()`:** `iter_mut()` yields `&mut Ghost`
    /// references so field writes happen in place without cloning.  `.zip()` pairs
    /// each mutable ghost with its corresponding start coordinate; iteration stops
    /// at the shorter sequence — a safe, implicitly-bounded parallel walk.
    fn reset_positions(&mut self) {
        let (pc, pr) = self.maze.player_start;
        self.player = Player::new(pc, pr);
        for (g, &(c, r)) in self.ghosts.iter_mut().zip(self.maze.ghost_starts.iter()) {
            g.col = c;
            g.row = r;
            g.dir = (0, 0);
        }
    }

    /// Convert a grid tile `(col, row)` to the screen-pixel centre of that tile.
    fn tile_center(&self, col: i32, row: i32) -> (f32, f32) {
        (
            ORIGIN_X + col as f32 * TILE + TILE / 2.0,
            ORIGIN_Y + row as f32 * TILE + TILE / 2.0,
        )
    }

    /// Deduct a life; if none remain, record the score and transition to Game Over.
    ///
    /// **Rust feature – separate borrow statements for `RefCell`:** `self.ctx` is
    /// an `Rc<RefCell<…>>`.  Calling `.borrow()` or `.borrow_mut()` on a `RefCell`
    /// panics at runtime if the same cell is already mutably borrowed.  Splitting
    /// each access into its own statement ensures the temporary borrow is released
    /// before the next one begins — satisfying the runtime borrow checker safely.
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
            let snapshot = self.ctx.borrow().scores;
            crate::scores::save(&snapshot);
            return Some(Transition::Goto(ScreenId::GameOver));
        }
        self.reset_positions();
        None
    }
}

/// Wire `PacmanGame` into the application's screen-switching framework.
///
/// **Rust feature – `impl Trait for Type`:** Rust has no inheritance.  Shared
/// behaviour is expressed through *traits* (similar to Java interfaces or C++
/// pure-virtual classes).  Implementing `Screen` here lets the engine call
/// `update`, `draw`, and `id` through a trait object without knowing the
/// concrete type at compile time.
impl Screen for PacmanGame {
    /// Process one frame of input and advance simulation timers.
    ///
    /// Returns `Some(Transition)` to switch screens, or `None` to stay here.
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
        // `iter_mut().enumerate()` yields (&mut Ghost, index) so we can assign
        // a different target per ghost without a separate index variable.
        if self.ghost_timer >= interval * 1.15 {
            self.ghost_timer -= interval * 1.15;
            let player_tile = (self.player.col, self.player.row);
            for (i, g) in self.ghosts.iter_mut().enumerate() {
                // Ghost 0 chases the player tile directly; ghost 1 leads 2 ahead.
                let target = if i == 0 {
                    player_tile
                } else {
                    (
                        player_tile.0 + 2 * self.player.dir.0,
                        player_tile.1 + 2 * self.player.dir.1,
                    )
                };
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
            let snapshot = self.ctx.borrow().scores;
            crate::scores::save(&snapshot);
            return Some(Transition::Goto(ScreenId::GameOver));
        }

        None
    }

    /// Render the current frame: speed-select prompt, maze, player, ghosts, HUD.
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
        let eaten = self.maze.pellet_total - self.maze.pellets_remaining();
        let pellet_text = format!("Dot {}/{}", eaten, self.maze.pellet_total);
        draw_text(&pellet_text, 10.0, 44.0, 20.0, theme::TEXT);
    }

    /// Report which screen this is so the router can manage transitions.
    fn id(&self) -> ScreenId {
        ScreenId::Pacman
    }
}

/// Unit tests for `level_bonus`.
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
