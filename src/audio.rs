//! Audio module: generates WAV beeps entirely in memory and loads them
//! as macroquad `Sound` objects for in-game sound effects.
//!
//! Key Rust concepts illustrated: `Vec<u8>` byte construction with
//! `extend_from_slice`; `to_le_bytes()` for little-endian layout; explicit
//! `as` casts; `async fn` / `.await`; `Option<T>` graceful-degradation;
//! `enum` tags; `#[derive(Clone, Copy, Default)]`; `match`; `#[cfg(test)]`.
use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};

const SAMPLE_RATE: u32 = 44_100;

/// Build a complete WAV file in memory as a `Vec<u8>` and return it.
///
/// **`Vec<u8>` and byte building** — `Vec::with_capacity` pre-allocates the
/// exact number of bytes needed; `extend_from_slice` appends a `&[u8]` slice
/// in one call, which is faster than pushing bytes one at a time.
///
/// **Little-endian layout** — WAV files require little-endian integers.
/// `u32::to_le_bytes()` and `i16::to_le_bytes()` each return a fixed-size
/// `[u8; N]` array; passing `&array` to `extend_from_slice` coerces it to
/// `&[u8]` automatically.
///
/// **Explicit `as` casts** — Rust never coerces numeric types silently.
/// Every widening or narrowing conversion (`SAMPLE_RATE as u64`, `i as f32`,
/// `sample as i16`, etc.) must be written out. `.clamp(0.0, 1.0)` bounds
/// the volume; `.fract()` extracts the fractional part of a float for phase.
pub fn square_wave_wav(freq_hz: f32, ms: u32, volume: f32) -> Vec<u8> {
    let num_samples = (SAMPLE_RATE as u64 * ms as u64 / 1000) as u32;
    let data_len = num_samples * 2; // 16-bit mono => 2 bytes/sample
    let mut out = Vec::with_capacity(44 + data_len as usize);

    // --- 44-byte RIFF/WAV header (field order required by the spec) ---
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes()); // fmt chunk size
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&1u16.to_le_bytes()); // mono
    out.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    out.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes()); // byte rate
    out.extend_from_slice(&2u16.to_le_bytes()); // block align
    out.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());

    // `i16::MAX` is 32 767 — the largest positive value of a signed 16-bit int.
    // `volume.clamp(0.0, 1.0)` guards the range; `i16::MAX as f32` is an
    // explicit widening cast. Rust requires all numeric conversions to be
    // spelled out; there is no implicit numeric promotion as in C/C++.
    let amp = volume.clamp(0.0, 1.0) * i16::MAX as f32;
    // Emit one signed 16-bit PCM sample per iteration of the sample loop.
    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        // `(freq_hz * t).fract()` keeps only the 0.0..1.0 fractional part,
        // producing a sawtooth that resets every cycle — this is the phase.
        let phase = (freq_hz * t).fract();
        let square = if phase < 0.5 { 1.0 } else { -1.0 };
        let env = 1.0 - (i as f32 / num_samples.max(1) as f32); // linear decay
                                                                // Narrowing cast: `as i16` truncates the f32 to a signed 16-bit int.
                                                                // The value is already bounded to [-i16::MAX, i16::MAX], so no overflow.
        let sample = (square * amp * env) as i16;
        out.extend_from_slice(&sample.to_le_bytes());
    }
    out
}

/// A closed set of named sound-effect variants.
///
/// **`enum` as a tag type** — even with no data payload, a Rust `enum` is a
/// distinct named type. These variants act like a closed set of constants but
/// are checked exhaustively by the compiler in every `match` expression.
///
/// **`#[derive(Clone, Copy)]`** — `Copy` tells the compiler it can copy an
/// `Sfx` value bitwise (like an integer) instead of moving ownership. This is
/// ergonomic for small discriminant types. `Clone` is a supertrait of `Copy`
/// and must also be derived.
#[derive(Clone, Copy)]
pub enum Sfx {
    Chomp,
    Bounce,
    Score,
    Death,
    Win,
    Select,
}

/// Owns all loaded sound effects for a game session.
///
/// **`#[derive(Default)]`** — auto-generates `Audio::default()`, which sets
/// every field to its own `Default` value. `Option<T>` defaults to `None`, so
/// a freshly-defaulted `Audio` has no sounds loaded — harmless and correct.
///
/// **`Option<Sound>` fields** — each field is `Option<Sound>` rather than
/// `Sound` so that a load failure leaves the rest of the game intact. A
/// missing sound becomes `None` and is silently skipped at playback time.
#[derive(Default)]
pub struct Audio {
    /// Chomp effect (short 660 Hz burst); `None` if loading failed.
    chomp: Option<Sound>,
    /// Bounce thud (440 Hz); `None` if loading failed.
    bounce: Option<Sound>,
    /// Score ping (880 Hz); `None` if loading failed.
    score: Option<Sound>,
    /// Death rumble (140 Hz); `None` if loading failed.
    death: Option<Sound>,
    /// Victory fanfare (990 Hz); `None` if loading failed.
    win: Option<Sound>,
    /// Menu selection click (550 Hz); `None` if loading failed.
    select: Option<Sound>,
}

impl Audio {
    /// Asynchronously generate and load all six sound effects.
    ///
    /// **`async fn` / `.await`** — an `async fn` returns a `Future` that does
    /// nothing until polled. Macroquad's executor polls it; `.await` suspends
    /// the current task until the inner future (e.g. `load_sound_from_bytes`)
    /// resolves. `load` must be `async` because its body contains `.await`.
    ///
    /// **Nested `async fn`** — `beep` is declared inside `load`. Rust allows
    /// `async fn` anywhere a regular `fn` is permitted, including nested inside
    /// another function body, which keeps the helper private to `load`.
    ///
    /// **`.await.ok()` — `Result` → `Option`** — `load_sound_from_bytes`
    /// returns `Result<Sound, _>`. `.ok()` maps `Ok(v)` to `Some(v)` and any
    /// `Err(_)` to `None`, implementing the graceful-degradation pattern.
    pub async fn load() -> Self {
        // Decode the generated WAVs; on any failure that effect is simply silent
        // (the game still runs). The WAV is self-generated so this is belt-and-braces.
        async fn beep(freq: f32, ms: u32) -> Option<Sound> {
            load_sound_from_bytes(&square_wave_wav(freq, ms, 0.3))
                .await
                .ok()
        }
        Audio {
            chomp: beep(660.0, 40).await,
            bounce: beep(440.0, 60).await,
            score: beep(880.0, 120).await,
            death: beep(140.0, 400).await,
            win: beep(990.0, 300).await,
            select: beep(550.0, 50).await,
        }
    }

    /// Play the sound effect identified by `sfx`.
    ///
    /// **`match` dispatch** — the `match` expression maps each `Sfx` variant
    /// to the corresponding `Option<Sound>` field. The compiler enforces
    /// exhaustiveness: adding a variant without updating `play` is a compile
    /// error, not a silent runtime bug.
    ///
    /// **`if let Some(snd)`** — this is the graceful-degradation payoff: when
    /// a field is `None` (sound failed to load), `if let` simply does not match
    /// and `play_sound_once` is never called. No panic, no error path needed.
    pub fn play(&self, sfx: Sfx) {
        let snd = match sfx {
            Sfx::Chomp => &self.chomp,
            Sfx::Bounce => &self.bounce,
            Sfx::Score => &self.score,
            Sfx::Death => &self.death,
            Sfx::Win => &self.win,
            Sfx::Select => &self.select,
        };
        if let Some(snd) = snd {
            play_sound_once(snd);
        }
    }
}

/// Unit tests for `square_wave_wav`.
///
/// **`#[cfg(test)]`** — this attribute gates the module so it is compiled only
/// during `cargo test`, keeping test helpers out of the release binary.
///
/// **Slice indexing** — `&w[0..4]` borrows a `&[u8]` slice from the `Vec<u8>`
/// using a half-open range (`0` inclusive, `4` exclusive). The tests confirm
/// that WAV header fields land at the correct byte offsets.
///
/// **Byte-string literals** — `b"RIFF"` has type `&[u8; 4]`. `assert_eq!`
/// comparing it against a `&[u8]` slice verifies both byte content and length.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wav_has_riff_and_wave_magic() {
        let w = square_wave_wav(440.0, 100, 0.3);
        assert_eq!(&w[0..4], b"RIFF");
        assert_eq!(&w[8..12], b"WAVE");
    }

    #[test]
    fn wav_length_matches_samples() {
        let ms = 100;
        let num_samples = SAMPLE_RATE * ms / 1000;
        let w = square_wave_wav(440.0, ms, 0.3);
        assert_eq!(w.len(), 44 + (num_samples * 2) as usize);
    }

    #[test]
    fn wav_data_chunk_size_is_correct() {
        let ms = 50;
        let num_samples = SAMPLE_RATE * ms / 1000;
        let w = square_wave_wav(440.0, ms, 0.3);
        let data_len = u32::from_le_bytes([w[40], w[41], w[42], w[43]]);
        assert_eq!(data_len, num_samples * 2);
    }
}
