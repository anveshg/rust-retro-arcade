#![allow(dead_code)]

/// 19 columns x 21 rows. '#' wall, '.' pellet, ' ' empty, 'P' player, 'G' ghost.
pub const MAZE: &[&str] = &[
    "###################",
    "#........#........#",
    "#.###.###.###.###.#",
    "#.................#",
    "#.###.#.###.#.###.#",
    "#.....#..#..#.....#",
    "#####.##.#.##.#####",
    "#   #.#  G  #.#   #",
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
}

pub struct Maze {
    pub cols: i32,
    pub rows: i32,
    tiles: Vec<Tile>,
    pellets: Vec<bool>,
    pub player_start: (i32, i32),
    pub ghost_starts: Vec<(i32, i32)>,
    pub pellet_total: usize,
}

impl Maze {
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

    fn in_bounds(&self, c: i32, r: i32) -> bool {
        c >= 0 && r >= 0 && c < self.cols && r < self.rows
    }

    fn index(&self, c: i32, r: i32) -> usize {
        (r * self.cols + c) as usize
    }

    pub fn is_wall(&self, c: i32, r: i32) -> bool {
        if !self.in_bounds(c, r) {
            return true;
        }
        self.tiles[self.index(c, r)] == Tile::Wall
    }

    pub fn pellet_at(&self, c: i32, r: i32) -> bool {
        self.in_bounds(c, r) && self.pellets[self.index(c, r)]
    }

    pub fn eat(&mut self, c: i32, r: i32) -> bool {
        if self.pellet_at(c, r) {
            let i = self.index(c, r);
            self.pellets[i] = false;
            true
        } else {
            false
        }
    }

    pub fn pellets_remaining(&self) -> usize {
        self.pellets.iter().filter(|&&p| p).count()
    }

    pub fn cleared(&self) -> bool {
        self.pellets_remaining() == 0
    }
}

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
