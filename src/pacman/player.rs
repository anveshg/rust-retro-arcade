//! Player state and movement for the Pac-Man game.
//!
//! A `Player` stores its grid position as two `i32` fields and tracks its
//! current and queued direction as `(i32, i32)` tuples.  A single call to
//! `advance` per game tick moves the character and reports the outcome.

use crate::pacman::maze::Maze;

/// The player character's position and buffered movement direction.
///
/// **Rust feature – `(i32, i32)` tuples as direction vectors:** Rust's built-in
/// tuple type groups values without naming them.  A direction like "move right"
/// is `(1, 0)` — `(Δcol, Δrow)`.  Tuples are stack-allocated value types;
/// `(0, 0)` serves as the conventional "stopped" sentinel throughout this crate.
pub struct Player {
    /// Current column (x-axis) in grid coordinates.
    pub col: i32,
    /// Current row (y-axis) in grid coordinates.
    pub row: i32,
    /// The direction the player moved last tick — `(Δcol, Δrow)`.
    pub dir: (i32, i32),
    /// The direction the player *wants* to turn into on the next tick.
    pub next_dir: (i32, i32),
}

impl Player {
    /// Construct a player placed at `(col, row)` with no initial direction.
    pub fn new(col: i32, row: i32) -> Self {
        Player {
            col,
            row,
            dir: (0, 0),
            next_dir: (0, 0),
        }
    }

    /// Try the queued direction first, then fall back to the current direction.
    ///
    /// **Rust feature – `for d in [expr, expr]`:** iterating a *temporary array
    /// literal* `[T; N]` is allocation-free.  The two candidates `next_dir` and
    /// `dir` are tried in priority order; the first one leading to an open tile
    /// wins.  This is idiomatic Rust — no heap allocation, no pointer chasing.
    ///
    /// **Rust feature – returning a tuple `(bool, bool)`:** Rust functions can
    /// return multiple values packed as a tuple.  Callers destructure with
    /// `let (moved, ate) = player.advance(...)` — no out-parameters needed.
    /// Advance one tile. Prefers `want` (if non-zero, it is queued), falling back to
    /// the current direction. Eats a pellet on arrival. Returns (moved, ate_pellet).
    pub fn advance(&mut self, maze: &mut Maze, want: (i32, i32)) -> (bool, bool) {
        if want != (0, 0) {
            self.next_dir = want;
        }
        for d in [self.next_dir, self.dir] {
            if d == (0, 0) {
                continue;
            }
            let nc = self.col + d.0;
            let nr = self.row + d.1;
            if !maze.is_wall(nc, nr) {
                self.col = nc;
                self.row = nr;
                self.dir = d;
                let ate = maze.eat(nc, nr);
                return (true, ate);
            }
        }
        (false, false)
    }
}

/// Unit tests for `Player` movement.
#[cfg(test)]
mod tests {
    use super::*;

    fn tiny() -> Maze {
        // P at (1,1), pellets to the right, wall at the ends.
        Maze::from_ascii(&["#####", "#P..#", "#####"])
    }

    #[test]
    fn moves_into_open_tile_and_eats() {
        let mut m = tiny();
        let (pc, pr) = m.player_start;
        let mut p = Player::new(pc, pr);
        let (moved, ate) = p.advance(&mut m, (1, 0));
        assert!(moved);
        assert!(ate);
        assert_eq!((p.col, p.row), (2, 1));
    }

    #[test]
    fn blocked_by_wall_does_not_move() {
        let mut m = tiny();
        let mut p = Player::new(1, 1);
        // Move right to (2,1), (3,1); next right is wall at (4,1).
        p.advance(&mut m, (1, 0));
        p.advance(&mut m, (1, 0));
        let (moved, _) = p.advance(&mut m, (1, 0));
        assert!(!moved);
        assert_eq!((p.col, p.row), (3, 1));
    }

    #[test]
    fn continues_in_current_dir_when_want_is_blocked() {
        let mut m = tiny();
        let mut p = Player::new(1, 1);
        // Establish rightward motion.
        p.advance(&mut m, (1, 0));
        // Want up (wall), should keep going right.
        let (moved, _) = p.advance(&mut m, (0, -1));
        assert!(moved);
        assert_eq!((p.col, p.row), (3, 1));
    }
}
