#![allow(dead_code)]

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct HighScores {
    pub best_pacman: u32,
    pub pong_wins: u32,
}

impl HighScores {
    /// Records a Pac-Man score. Returns true if it beat the previous best.
    pub fn record_pacman(&mut self, score: u32) -> bool {
        if score > self.best_pacman {
            self.best_pacman = score;
            true
        } else {
            false
        }
    }

    pub fn record_pong_win(&mut self) {
        self.pong_wins += 1;
    }
}

pub fn load() -> HighScores {
    let storage = quad_storage::STORAGE.lock().unwrap();
    let best_pacman = storage
        .get("best_pacman")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let pong_wins = storage
        .get("pong_wins")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    HighScores {
        best_pacman,
        pong_wins,
    }
}

pub fn save(h: &HighScores) {
    let mut storage = quad_storage::STORAGE.lock().unwrap();
    storage.set("best_pacman", &h.best_pacman.to_string());
    storage.set("pong_wins", &h.pong_wins.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_score_is_a_new_best() {
        let mut h = HighScores::default();
        assert!(h.record_pacman(120));
        assert_eq!(h.best_pacman, 120);
    }

    #[test]
    fn lower_score_does_not_replace_best() {
        let mut h = HighScores {
            best_pacman: 200,
            pong_wins: 0,
        };
        assert!(!h.record_pacman(150));
        assert_eq!(h.best_pacman, 200);
    }

    #[test]
    fn pong_wins_accumulate() {
        let mut h = HighScores::default();
        h.record_pong_win();
        h.record_pong_win();
        assert_eq!(h.pong_wins, 2);
    }
}
