//! `scores` — high-score persistence for the arcade.
//!
//! Two `persist` implementations are compiled in mutually exclusively: native
//! builds use the filesystem; Wasm builds call JavaScript through `extern "C"`
//! and `unsafe`.  The public `load`/`save` API is identical on both targets.
//! Concepts: `#[derive]`, `&mut self`, `Result`/`Option` chains, `#[cfg(...)]`.

/// Stores lifetime-best scores across every game in the arcade.
///
/// `#[derive(...)]` auto-generates trait implementations the compiler would
/// otherwise require you to write by hand:
/// - `Clone` — `h.clone()` makes a copy (trivial here since the type is `Copy`).
/// - `Copy` — values are bitwise-copied on assignment instead of *moved* (zero-cost, no heap).
/// - `Default` — `HighScores::default()` gives the zero-value (`0u32` per field).
/// - `PartialEq` — enables `==` / `!=` between two `HighScores` values.
/// - `Eq` — strengthens `PartialEq` (`a == a` always holds), required by some generic bounds.
/// - `Debug` — enables `{:?}` / `{:#?}` formatting; invaluable in tests.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct HighScores {
    /// Best Pac-Man score ever recorded on this machine / in this browser.
    pub best_pacman: u32,
    /// Total Pong wins accumulated across all sessions.
    pub pong_wins: u32,
}

impl HighScores {
    /// Records a Pac-Man score. Returns true if it beat the previous best.
    ///
    /// `&mut self` gives this method an *exclusive* mutable reference to the
    /// receiver.  Rust's borrow checker guarantees no other reference to this
    /// `HighScores` exists while this call runs — a compile-time lock with zero
    /// runtime cost.  Callers must hold a `mut` binding:
    /// `let mut h = HighScores::default(); h.record_pacman(42);`
    pub fn record_pacman(&mut self, score: u32) -> bool {
        if score > self.best_pacman {
            self.best_pacman = score;
            true
        } else {
            false
        }
    }

    /// Increments the Pong win counter by one.
    ///
    /// Also takes `&mut self`: only one mutable reference may exist at a time.
    /// If you try to call this while another `&mut HighScores` is live, the
    /// compiler rejects the program — no runtime check needed.
    pub fn record_pong_win(&mut self) {
        self.pong_wins += 1;
    }
}

/// Load persisted high scores (or defaults). Never panics.
///
/// Delegates to whichever `persist` module the compiler selected; the caller
/// never needs to know which platform is running.
pub fn load() -> HighScores {
    persist::load()
}

/// Persist high scores. Best-effort: failures degrade silently.
///
/// Takes `&HighScores` — a shared, immutable borrow.  The caller keeps full
/// ownership; `save` only reads the data and never modifies it.
pub fn save(h: &HighScores) {
    persist::save(h);
}

// Native: a small text file in the working directory.
/// Native (non-Wasm) persistence layer: reads and writes a plain text file.
///
/// `#[cfg(not(target_arch = "wasm32"))]` is *conditional compilation*: this
/// entire module is included **only** when the build target is not WebAssembly.
/// Think of it as a C `#ifdef`, but the excluded branch is fully type-checked
/// and then discarded — it cannot compile-error silently or bloat the binary.
#[cfg(not(target_arch = "wasm32"))]
mod persist {
    use super::HighScores;
    use std::fs;

    /// Path of the score file, relative to the process working directory.
    const FILE: &str = "highscores.txt";

    /// Reads the score file and parses two whitespace-separated `u32`s.
    ///
    /// `fs::read_to_string` returns `Result<String, io::Error>`.
    /// `if let Ok(contents)` is a *pattern-match destructure*: it unwraps the
    /// success variant and silently ignores `Err` — fine here because a missing
    /// file simply means no prior scores exist yet.
    ///
    /// `.next().and_then(|x| x.parse().ok())` is `Option` chaining:
    /// `Iterator::next()` → `Option<&str>`, then `and_then` threads it through
    /// `str::parse::<u32>()` (which returns `Result`) converted to `Option` by
    /// `.ok()`, collapsing all error paths to `None` without an explicit `match`.
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

    /// Writes both scores as space-separated integers to the text file.
    ///
    /// `let _ = fs::write(...)` *intentionally discards* the returned `Result`.
    /// Rust warns when a `Result` is silently dropped, so `let _ =` is the
    /// idiomatic way to say "I know this can fail; I choose not to handle it."
    pub fn save(h: &HighScores) {
        let _ = fs::write(FILE, format!("{} {}", h.best_pacman, h.pong_wins));
    }
}

// Web: browser localStorage via a tiny integer-only JS plugin (see index.html).
// Passing only u32s avoids the fragile string FFI / sapp_jsutils version coupling.
/// Wasm persistence layer: calls JavaScript functions that wrap `localStorage`.
///
/// `#[cfg(target_arch = "wasm32")]` is the mirror of the native attribute above —
/// this module is compiled in **only** for WebAssembly targets.  Because both
/// `persist` modules expose identical `load`/`save` signatures, the rest of the
/// crate is completely unaware of which one was selected at build time.
#[cfg(target_arch = "wasm32")]
mod persist {
    use super::HighScores;

    // Declares three JavaScript functions the Wasm runtime will import.
    // (A doc comment can't attach to an `extern` block itself, only to the
    // items inside it — so this orientation note uses plain `//`.)
    //
    // `extern "C"` tells Rust to expect these symbols using the C calling
    // convention — the ABI that WebAssembly uses for cross-language calls.
    // The real implementations live in the inline JS plugin in `index.html`;
    // Rust only declares the types so the compiler knows the call signatures.
    extern "C" {
        /// Returns the best Pac-Man score stored in `localStorage`.
        fn hs_load_pacman() -> u32;
        /// Returns the total Pong wins stored in `localStorage`.
        fn hs_load_pong() -> u32;
        /// Writes both scores to `localStorage` from the JavaScript side.
        fn hs_save(pacman: u32, pong: u32);
    }

    /// Calls the JS imports and constructs a `HighScores` from their return values.
    ///
    /// Every call into foreign code requires an `unsafe { }` block.  Rust cannot
    /// verify that a C or JS function upholds memory-safety invariants, so the
    /// programmer asserts correctness via a `SAFETY` comment.  Here the invariant
    /// is: the JS plugin is always registered before any Rust code executes, so
    /// the imported function pointers are guaranteed to be valid.
    pub fn load() -> HighScores {
        // SAFETY: provided by the `highscores` JS plugin registered in index.html.
        unsafe {
            HighScores {
                best_pacman: hs_load_pacman(),
                pong_wins: hs_load_pong(),
            }
        }
    }

    /// Calls the JS `hs_save` import to persist both counters.
    pub fn save(h: &HighScores) {
        // SAFETY: see `load`.
        unsafe { hs_save(h.best_pacman, h.pong_wins) }
    }
}

/// Unit tests for the in-memory score logic.
///
/// `#[cfg(test)]` means this module is compiled **only** when running
/// `cargo test` — it is stripped from every release binary.  `use super::*`
/// imports everything from the parent module so tests can access items without
/// artificially widening the production API surface.
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
