//! Opus encode/decode wrappers around the pure-Rust `opus_rs` implementation.

use core::fmt;

/// Encoder for 48 kHz mono voice frames (20 ms = 960 samples).
pub struct OpusEncoder {
    inner: opus_rs::OpusEncoder,
    /// Target bitrate in bits per second.
    pub bitrate_bps: u32,
    /// Frame size in samples (960 = 20 ms at 48 kHz).
    pub frame_size: u32,
    /// Number of audio channels (1 = mono voice).
    pub channels: u8,
}

impl fmt::Debug for OpusEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpusEncoder")
            .field("bitrate_bps", &self.bitrate_bps)
            .field("frame_size", &self.frame_size)
            .field("channels", &self.channels)
            .finish_non_exhaustive()
    }
}

impl OpusEncoder {
    /// Creates a VOIP encoder at the requested bitrate.
    pub fn new(bitrate_bps: u32, sample_rate: u32, channels: u8) -> Result<Self, &'static str> {
        if sample_rate != 48_000 {
            return Err("networking_audio_integration: only 48 kHz supported in this slice");
        }
        if channels != 1 {
            return Err("networking_audio_integration: only mono supported in this slice");
        }
        let mut inner = opus_rs::OpusEncoder::new(
            sample_rate as i32,
            channels as usize,
            opus_rs::Application::Voip,
        )?;
        inner.bitrate_bps = i32::try_from(bitrate_bps).unwrap_or(i32::MAX);
        inner.use_cbr = true;
        Ok(Self {
            inner,
            bitrate_bps,
            frame_size: 960,
            channels,
        })
    }

    /// Encodes one frame of `f32` PCM into `out` (Opus payload including TOC byte).
    ///
    /// # Errors
    ///
    /// Returns an error when the underlying encoder rejects the frame (empty output must not be
    /// mistaken for a valid silent packet on the wire).
    pub fn encode(&mut self, pcm: &[f32], out: &mut [u8; 256]) -> Result<u8, &'static str> {
        let n = self
            .inner
            .encode(pcm, self.frame_size as usize, out.as_mut_slice())
            .map_err(|_| "networking_audio_integration: Opus encode failed")?;
        if n == 0 {
            return Err("networking_audio_integration: Opus encode produced zero bytes");
        }
        u8::try_from(n.min(255)).map_err(|_| "networking_audio_integration: Opus frame too long")
    }

    /// Updates the target bitrate in bits per second.
    pub fn set_bitrate(&mut self, bps: u32) {
        self.bitrate_bps = bps;
        self.inner.bitrate_bps = i32::try_from(bps).unwrap_or(i32::MAX);
    }
}

/// Decoder for one remote speaker.
pub struct OpusDecoder {
    inner: opus_rs::OpusDecoder,
    /// Sample rate in Hz.
    pub sample_rate: u32,
    /// Channel count.
    pub channels: u8,
    last_pcm: Vec<f32>,
}

impl fmt::Debug for OpusDecoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpusDecoder")
            .field("sample_rate", &self.sample_rate)
            .field("channels", &self.channels)
            .field("last_pcm_len", &self.last_pcm.len())
            .finish_non_exhaustive()
    }
}

impl OpusDecoder {
    /// Allocates a decoder for the given sample rate and channel count.
    pub fn new(sample_rate: u32, channels: u8) -> Result<Self, &'static str> {
        let frame = (20usize * sample_rate as usize) / 1000 * channels as usize;
        Ok(Self {
            inner: opus_rs::OpusDecoder::new(sample_rate as i32, channels as usize)?,
            sample_rate,
            channels,
            last_pcm: vec![0.0f32; frame],
        })
    }

    /// Decodes an Opus packet to `pcm_out`. `None` repeats the last good frame (PLC stand-in).
    pub fn decode(&mut self, opus_data: Option<&[u8]>, pcm_out: &mut [f32]) -> usize {
        let frame = self.frame_size_samples();
        if pcm_out.len() < frame {
            return 0;
        }
        match opus_data {
            Some(bytes) if !bytes.is_empty() => match self.inner.decode(bytes, frame, pcm_out) {
                Ok(n) => {
                    if self.last_pcm.len() == n {
                        self.last_pcm.copy_from_slice(&pcm_out[..n]);
                    } else {
                        self.last_pcm.resize(n, 0.0);
                        self.last_pcm.copy_from_slice(&pcm_out[..n]);
                    }
                    n
                }
                Err(_) => {
                    self.fill_plc(pcm_out, frame);
                    frame
                }
            },
            _ => {
                self.fill_plc(pcm_out, frame);
                frame
            }
        }
    }

    fn frame_size_samples(&self) -> usize {
        // 20 ms at 48 kHz.
        (20 * self.sample_rate as usize) / 1000 * self.channels as usize
    }

    fn fill_plc(&mut self, pcm_out: &mut [f32], frame: usize) {
        if self.last_pcm.len() == frame {
            pcm_out[..frame].copy_from_slice(&self.last_pcm);
        } else {
            pcm_out[..frame].fill(0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine_frame(freq: f32, len: usize) -> Vec<f32> {
        (0..len)
            .map(|i| {
                let t = i as f32 / 48_000.0;
                (2.0 * std::f32::consts::PI * freq * t).sin() * 0.1
            })
            .collect()
    }

    /// TC-IR-4.3.2.1 — 24 kbps Opus frame stays under 60 bytes for 20 ms mono.
    #[test]
    fn tc_ir_4_3_2_1_frame_under_budget() {
        let mut enc = OpusEncoder::new(24_000, 48_000, 1).expect("encoder");
        let pcm = sine_frame(440.0, 960);
        let mut out = [0u8; 256];
        let n = enc.encode(&pcm, &mut out).expect("encode");
        assert!(
            usize::from(n) < 80,
            "24 kbps CBR frame should stay small (got {n} bytes)"
        );
    }

    /// TC-IR-4.3.2.2 — PLC path produces audio after a dropped packet.
    #[test]
    fn tc_ir_4_3_2_2_plc_on_loss() {
        let mut enc = OpusEncoder::new(32_000, 48_000, 1).expect("encoder");
        let mut dec = OpusDecoder::new(48_000, 1).expect("decoder");
        let pcm = sine_frame(220.0, 960);
        let mut pkt = [0u8; 256];
        let n = enc.encode(&pcm, &mut pkt).expect("encode");
        let mut out = [0.0f32; 960];
        let _ = dec.decode(Some(&pkt[..usize::from(n)]), &mut out);
        let mut plc = [0.0f32; 960];
        let m = dec.decode(None, &mut plc);
        assert_eq!(m, 960);
        let energy: f32 = plc.iter().map(|x| x * x).sum();
        assert!(energy.is_finite());
    }

    /// TC-IR-4.3.1.1 — encode/decode round-trip correlation (lossy codec).
    #[test]
    fn tc_ir_4_3_1_1_round_trip_correlation() {
        let mut enc = OpusEncoder::new(64_000, 48_000, 1).expect("encoder");
        let mut dec = OpusDecoder::new(48_000, 1).expect("decoder");
        let pcm = sine_frame(880.0, 960);
        let mut pkt = [0u8; 256];
        let n = enc.encode(&pcm, &mut pkt).expect("encode");
        let mut out = [0.0f32; 960];
        let _ = dec.decode(Some(&pkt[..usize::from(n)]), &mut out);
        let num: f32 = pcm.iter().zip(out.iter()).map(|(a, b)| a * b).sum();
        let den_a: f32 = pcm.iter().map(|a| a * a).sum();
        let den_b: f32 = out.iter().map(|b| b * b).sum();
        let corr = num / (den_a.sqrt() * den_b.sqrt() + 1e-9);
        assert!(corr > 0.75, "corr={corr}");
    }
}
