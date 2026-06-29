use crate::pacman::maze::Maze;

pub struct Ghost {
    pub col: i32,
    pub row: i32,
    pub dir: (i32, i32),
}

impl Ghost {
    pub fn new(col: i32, row: i32) -> Self {
        Ghost {
            col,
            row,
            dir: (0, 0),
        }
    }

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

    pub fn touches(&self, col: i32, row: i32) -> bool {
        self.col == col && self.row == row
    }
}

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
