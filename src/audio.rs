use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};

const SAMPLE_RATE: u32 = 44_100;

/// Generate a mono 16-bit PCM WAV containing a decaying square wave.
pub fn square_wave_wav(freq_hz: f32, ms: u32, volume: f32) -> Vec<u8> {
    let num_samples = (SAMPLE_RATE as u64 * ms as u64 / 1000) as u32;
    let data_len = num_samples * 2; // 16-bit mono => 2 bytes/sample
    let mut out = Vec::with_capacity(44 + data_len as usize);

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

    let amp = volume.clamp(0.0, 1.0) * i16::MAX as f32;
    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let phase = (freq_hz * t).fract();
        let square = if phase < 0.5 { 1.0 } else { -1.0 };
        let env = 1.0 - (i as f32 / num_samples.max(1) as f32); // linear decay
        let sample = (square * amp * env) as i16;
        out.extend_from_slice(&sample.to_le_bytes());
    }
    out
}

#[derive(Clone, Copy)]
pub enum Sfx {
    Chomp,
    Bounce,
    Score,
    Death,
    Win,
    Select,
}

#[derive(Default)]
pub struct Audio {
    chomp: Option<Sound>,
    bounce: Option<Sound>,
    score: Option<Sound>,
    death: Option<Sound>,
    win: Option<Sound>,
    select: Option<Sound>,
}

impl Audio {
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
