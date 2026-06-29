//! Maze definition and grid-state management for the Pac-Man game.
//!
//! The raw layout lives in `MAZE`, a `const` slice of string slices (`&[&str]`).
//! [`Maze::from_ascii`] parses it once at startup into flat `Vec` buffers for
//! fast indexed access.  All coordinates follow `(col, row)` order, origin
//! at the top-left.

/// The raw ASCII layout used by [`Maze::from_ascii`] to build the game grid.
///
/// **Rust feature – `&[&str]` (slice of string slices):** `&[T]` is a *borrowed
/// slice* — a fat pointer (data pointer + length) to a contiguous sequence of
/// `T` values.  Here `T = &str`, itself a borrowed string slice.  Both layers
/// point into the binary's read-only segment (they are `'static` string literals),
/// so the constant costs zero heap allocation.  The outer `&` prevents anyone
/// from replacing the pointer stored in `MAZE`.
///
/// 19 columns x 21 rows. '#' wall, '.' pellet, ' ' empty, 'P' player, 'G' ghost.
pub const MAZE: &[&str] = &[
    "###################",
    "#........#........#",
    "#.###.###.###.###.#",
    "#.................#",
    "#.###.#.###.#.###.#",
    "#.....#..#..#.....#",
    "#####.##.#.##.#####",
    "#   #.#G G  #.#   #",
    "#####.# ### #.#####",
    "#........#........#",
    "#.###.###.###.###.#",
    "#...#....P....#...#",
    "###.#.#.###.#.#.###",
    "#.....#..#..#.....#",
    "#.#######.#######.#",
    "#.................#",
    "#.###.###.###.###.#",
    "#...#.........#...#",
    "#.#.#.#######.#.#.#",
    "#.................#",
    "###################",
];

/// Classifies each grid cell as impassable or traversable.
///
/// **Rust feature – private `enum`:** Omitting `pub` keeps `Tile` module-private.
/// Callers use the higher-level `is_wall` / `pellet_at` methods instead.
/// The `#[derive]` attribute auto-generates `Clone`, `Copy` (cheap bitwise copy),
/// `PartialEq`, and `Eq` so that `tile == Tile::Wall` compiles without any
/// hand-written trait implementations.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
}

/// Runtime grid state, built from an ASCII layout by [`Maze::from_ascii`].
pub struct Maze {
    /// Number of tile columns (x-axis width).
    pub cols: i32,
    /// Number of tile rows (y-axis height).
    pub rows: i32,
    /// Flat tile buffer — cell `(c, r)` lives at index `r * cols + c`.
    ///
    /// **Rust feature – `Vec<T>`:** an owned, heap-allocated, growable array.
    /// Unlike a C array, Rust's borrow checker statically prevents dangling
    /// references into a `Vec` after it has been reallocated or moved.
    tiles: Vec<Tile>,
    /// Parallel pellet presence flags; `true` means the pellet has not been eaten.
    pellets: Vec<bool>,
    /// Grid position `(col, row)` where the player spawns.
    pub player_start: (i32, i32),
    /// Grid positions `(col, row)` where each ghost spawns.
    pub ghost_starts: Vec<(i32, i32)>,
    /// Total pellets present in the original layout — constant after construction.
    pub pellet_total: usize,
}

impl Maze {
    /// Parse an ASCII grid and produce a fully initialised `Maze`.
    ///
    /// **Rust feature – `for (r, line) in rows.iter().enumerate()`:** `.enumerate()`
    /// wraps any iterator so each item arrives paired with its 0-based index as a
    /// destructured tuple `(index, value)`.  The inner loop does the same for
    /// characters: `for (c, ch) in line.chars().enumerate()`.  This is equivalent
    /// to Python's `enumerate` but compiles to the same code as a hand-written
    /// index loop — zero overhead.
    ///
    /// The flat index `r * width + c` maps a 2-D coordinate to a 1-D `Vec` slot,
    /// a cache-friendly layout used throughout this module.
    pub fn from_ascii(rows: &[&str]) -> Self {
        let height = rows.len() as i32;
        let width = rows.iter().map(|r| r.len()).max().unwrap_or(0) as i32;
        let mut tiles = vec![Tile::Open; (width * height) as usize];
        let mut pellets = vec![false; (width * height) as usize];
        let mut player_start = (1, 1);
        let mut ghost_starts = Vec::new();
        let mut pellet_total = 0;

        for (r, line) in rows.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                // Flat 1-D index: row-major order, same formula used in `Self::index`.
                let idx = r * width as usize + c;
                match ch {
                    '#' => tiles[idx] = Tile::Wall,
                    '.' => {
                        pellets[idx] = true;
                        pellet_total += 1;
                    }
                    'P' => player_start = (c as i32, r as i32),
                    'G' => ghost_starts.push((c as i32, r as i32)),
                    _ => {}
                }
            }
        }

        Maze {
            cols: width,
            rows: height,
            tiles,
            pellets,
            player_start,
            ghost_starts,
            pellet_total,
        }
    }

    /// Returns `true` when `(c, r)` is inside the grid.
    ///
    /// Kept private so external code always goes through the safe `is_wall` /
    /// `pellet_at` guards, which return a defined default for out-of-bounds queries.
    fn in_bounds(&self, c: i32, r: i32) -> bool {
        c >= 0 && r >= 0 && c < self.cols && r < self.rows
    }

    /// Convert a 2-D grid coordinate to a flat 1-D buffer index.
    ///
    /// **Rust feature – `usize` for indexing:** Rust requires `Vec` / slice indices
    /// to be `usize`.  The `as usize` cast is the idiomatic conversion from `i32`
    /// after bounds have already been verified by `in_bounds`.
    fn index(&self, c: i32, r: i32) -> usize {
        (r * self.cols + c) as usize
    }

    /// Returns `true` when the cell is a wall *or* lies outside the grid.
    ///
    /// Treating out-of-bounds as wall simplifies movement code: no special-casing
    /// needed at the grid edge.
    pub fn is_wall(&self, c: i32, r: i32) -> bool {
        if !self.in_bounds(c, r) {
            return true;
        }
        self.tiles[self.index(c, r)] == Tile::Wall
    }

    /// Returns `true` when a pellet is still present at `(c, r)`.
    pub fn pellet_at(&self, c: i32, r: i32) -> bool {
        self.in_bounds(c, r) && self.pellets[self.index(c, r)]
    }

    /// Remove the pellet at `(c, r)` and return `true` if one was there.
    ///
    /// `&mut self` signals to Rust that this call may modify the `Maze`.  The
    /// compiler ensures no other live reference to `self` coexists — the borrow
    /// checker's core exclusive-mutation guarantee.
    pub fn eat(&mut self, c: i32, r: i32) -> bool {
        if self.pellet_at(c, r) {
            let i = self.index(c, r);
            self.pellets[i] = false;
            true
        } else {
            false
        }
    }

    /// Count pellets still on the grid.
    ///
    /// **Rust feature – `.iter().filter(…).count()`:** `.iter()` borrows each
    /// element as a reference; `.filter(|&&p| p)` keeps only `true` entries
    /// (the double `&&` auto-derefs through the iterator reference and the `bool`
    /// reference); `.count()` consumes the iterator and tallies the result.
    /// No explicit loop or mutable counter is needed.
    pub fn pellets_remaining(&self) -> usize {
        self.pellets.iter().filter(|&&p| p).count()
    }

    /// Returns `true` when every pellet has been eaten.
    pub fn cleared(&self) -> bool {
        self.pellets_remaining() == 0
    }
}

/// Unit tests compiled only during `cargo test`.
///
/// **Rust feature – `#[cfg(test)] mod tests`:** the `cfg(test)` attribute gates
/// compilation of this entire module to test builds only; release binaries never
/// include it.  `use super::*;` imports all items from the parent module —
/// including private ones like `Tile` — which is the idiomatic white-box test
/// pattern in Rust.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_dimensions() {
        let m = Maze::from_ascii(MAZE);
        assert_eq!(m.rows, 21);
        assert_eq!(m.cols, 19);
    }

    #[test]
    fn border_is_wall_and_start_is_open() {
        let m = Maze::from_ascii(MAZE);
        assert!(m.is_wall(0, 0));
        let (pc, pr) = m.player_start;
        assert!(!m.is_wall(pc, pr));
    }

    #[test]
    fn out_of_bounds_counts_as_wall() {
        let m = Maze::from_ascii(MAZE);
        assert!(m.is_wall(-1, 5));
        assert!(m.is_wall(1000, 5));
    }

    #[test]
    fn has_pellets_and_at_least_one_ghost() {
        let m = Maze::from_ascii(MAZE);
        assert!(m.pellet_total > 0);
        assert!(!m.ghost_starts.is_empty());
    }

    #[test]
    fn eating_a_pellet_removes_it() {
        let mut m = Maze::from_ascii(MAZE);
        // Find any pellet cell.
        let mut found = None;
        for r in 0..m.rows {
            for c in 0..m.cols {
                if m.pellet_at(c, r) {
                    found = Some((c, r));
                }
            }
        }
        let (c, r) = found.expect("maze should have a pellet");
        let before = m.pellets_remaining();
        assert!(m.eat(c, r));
        assert!(!m.eat(c, r)); // already eaten
        assert_eq!(m.pellets_remaining(), before - 1);
    }
}
