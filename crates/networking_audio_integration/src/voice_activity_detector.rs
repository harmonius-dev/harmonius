//! Lightweight RMS-based voice activity detection with hangover.

/// Gates transmission when a frame's RMS energy stays below threshold.
#[derive(Debug)]
pub struct VoiceActivityDetector {
    /// RMS energy threshold for speech detection.
    pub threshold: f32,
    /// Hangover frames after last speech detection.
    pub hangover_frames: u16,
    /// Remaining hangover counter.
    pub hangover_remaining: u16,
}

impl VoiceActivityDetector {
    /// Builds a detector with the given RMS threshold.
    ///
    /// Default `hangover_frames` is `5` (≈100 ms at 20 ms frames, 48 kHz / 960-sample frames), a
    /// practical stand-in for WebRTC VAD mode-1 style tail; override the field after construction
    /// for other tunings.
    #[must_use]
    pub fn new(threshold: f32) -> Self {
        Self {
            threshold,
            hangover_frames: 5,
            hangover_remaining: 0,
        }
    }

    /// Returns `true` when the frame should be transmitted as speech.
    pub fn detect(&mut self, pcm: &[f32]) -> bool {
        if pcm.is_empty() {
            return self.consume_hangover();
        }
        let mean_sq: f32 = pcm.iter().map(|x| x * x).sum::<f32>() / pcm.len() as f32;
        let rms = mean_sq.sqrt();
        if rms > self.threshold {
            self.hangover_remaining = self.hangover_frames;
            return true;
        }
        self.consume_hangover()
    }

    fn consume_hangover(&mut self) -> bool {
        if self.hangover_remaining > 0 {
            self.hangover_remaining -= 1;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-4.3.5.1 — silence stays below threshold with no hangover left.
    #[test]
    fn tc_ir_4_3_5_1_vad_silence() {
        let mut vad = VoiceActivityDetector::new(0.05);
        vad.hangover_remaining = 0;
        let silence = vec![0.0f32; 960];
        assert!(!vad.detect(&silence));
    }

    /// TC-IR-4.3.5.2 — energetic frame triggers transmission.
    #[test]
    fn tc_ir_4_3_5_2_vad_speech() {
        let mut vad = VoiceActivityDetector::new(0.05);
        let speech = vec![0.3f32; 960];
        assert!(vad.detect(&speech));
    }
}
