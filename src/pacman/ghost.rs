//! Ghost AI — one instance per ghost, each stepping deterministically toward
//! a target tile each game tick using greedy squared-distance minimisation.

use crate::pacman::maze::Maze;

/// An enemy ghost: its current grid position and last-used movement direction.
///
/// All fields are `pub` so the game loop in `mod.rs` can read positions for
/// collision detection; the AI that *writes* them is encapsulated in `step`.
pub struct Ghost {
    /// Current column in grid coordinates.
    pub col: i32,
    /// Current row in grid coordinates.
    pub row: i32,
    /// The direction used on the previous step — prevents immediate reversal.
    pub dir: (i32, i32),
}

impl Ghost {
    /// Construct a ghost placed at `(col, row)` with no initial direction.
    pub fn new(col: i32, row: i32) -> Self {
        Ghost {
            col,
            row,
            dir: (0, 0),
        }
    }

    /// Greedy single-step pathfinding toward `target`.
    ///
    /// **Rust feature – `Option<T>`:** `best` starts as `None` (no candidate yet)
    /// and becomes `Some((direction, squared_distance))` when a valid neighbour is
    /// found.  `is_none_or(|val| cond)` returns `true` when the `Option` holds no
    /// value *or* when the closure evaluates to `true` — a compact running-minimum
    /// update that avoids unwrapping or pattern-matching manually.
    ///
    /// **Rust feature – `if let Some((d, _)) = best`:** pattern-matching against
    /// an `Option` with tuple destructuring.  `d` captures the winning direction
    /// and `_` discards the distance.  This is idiomatic Rust for "unwrap if
    /// present, skip if `None`" — a safe alternative to C-style null checks.
    ///
    /// The AI is fully deterministic: the same maze + position + target always
    /// produce the same move.  No randomness is needed because each ghost already
    /// receives a different target tile (see the game loop in `mod.rs`).
    /// Step one tile toward `target`, choosing the open non-reverse neighbour that
    /// minimizes squared distance. Reverses only if no other option exists.
    pub fn step(&mut self, maze: &Maze, target: (i32, i32)) {
        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let reverse = (-self.dir.0, -self.dir.1);
        let mut best: Option<((i32, i32), i32)> = None;
        for d in dirs {
            if d == reverse {
                continue;
            }
            let nc = self.col + d.0;
            let nr = self.row + d.1;
            if maze.is_wall(nc, nr) {
                continue;
            }
            let dist = (nc - target.0).pow(2) + (nr - target.1).pow(2);
            // Update `best` when no candidate exists yet or this neighbour is closer.
            if best.is_none_or(|(_, bd)| dist < bd) {
                best = Some((d, dist));
            }
        }
        if best.is_none() {
            let nc = self.col + reverse.0;
            let nr = self.row + reverse.1;
            if !maze.is_wall(nc, nr) {
                best = Some((reverse, 0));
            }
        }
        if let Some((d, _)) = best {
            self.dir = d;
            self.col += d.0;
            self.row += d.1;
        }
    }

    /// Returns `true` when this ghost occupies the same tile as `(col, row)`.
    ///
    /// Called by the game loop each frame for player-ghost collision detection.
    pub fn touches(&self, col: i32, row: i32) -> bool {
        self.col == col && self.row == row
    }
}

/// Unit tests for ghost movement.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_toward_target_along_a_corridor() {
        let m = Maze::from_ascii(&["#######", "#.....#", "#######"]);
        let mut g = Ghost::new(1, 1);
        g.step(&m, (5, 1));
        assert_eq!((g.col, g.row), (2, 1));
    }

    #[test]
    fn does_not_reverse_when_forward_is_open() {
        let m = Maze::from_ascii(&["#######", "#.....#", "#######"]);
        let mut g = Ghost::new(3, 1);
        g.dir = (1, 0); // moving right
                        // Target is behind it, but it should keep moving right (no reverse).
        g.step(&m, (0, 1));
        assert_eq!((g.col, g.row), (4, 1));
    }

    #[test]
    fn touches_detects_same_cell() {
        let g = Ghost::new(3, 4);
        assert!(g.touches(3, 4));
        assert!(!g.touches(3, 5));
    }
}
