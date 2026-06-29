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

/// Load persisted high scores (or defaults). Never panics.
pub fn load() -> HighScores {
    persist::load()
}

/// Persist high scores. Best-effort: failures degrade silently.
pub fn save(h: &HighScores) {
    persist::save(h);
}

// Native: a small text file in the working directory.
#[cfg(not(target_arch = "wasm32"))]
mod persist {
    use super::HighScores;
    use std::fs;

    const FILE: &str = "highscores.txt";

    pub fn load() -> HighScores {
        let mut h = HighScores::default();
        if let Ok(contents) = fs::read_to_string(FILE) {
            let mut nums = contents.split_whitespace();
            if let Some(v) = nums.next().and_then(|x| x.parse().ok()) {
                h.best_pacman = v;
            }
            if let Some(v) = nums.next().and_then(|x| x.parse().ok()) {
                h.pong_wins = v;
            }
        }
        h
    }

    pub fn save(h: &HighScores) {
        let _ = fs::write(FILE, format!("{} {}", h.best_pacman, h.pong_wins));
    }
}

// Web: browser localStorage via a tiny integer-only JS plugin (see index.html).
// Passing only u32s avoids the fragile string FFI / sapp_jsutils version coupling.
#[cfg(target_arch = "wasm32")]
mod persist {
    use super::HighScores;

    extern "C" {
        fn hs_load_pacman() -> u32;
        fn hs_load_pong() -> u32;
        fn hs_save(pacman: u32, pong: u32);
    }

    pub fn load() -> HighScores {
        // SAFETY: provided by the `highscores` JS plugin registered in index.html.
        unsafe {
            HighScores {
                best_pacman: hs_load_pacman(),
                pong_wins: hs_load_pong(),
            }
        }
    }

    pub fn save(h: &HighScores) {
        // SAFETY: see `load`.
        unsafe { hs_save(h.best_pacman, h.pong_wins) }
    }
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
