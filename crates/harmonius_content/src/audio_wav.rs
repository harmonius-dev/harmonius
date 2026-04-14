//! Minimal RIFF WAVE parser for fmt + optional `cue` chunk.

/// Extracted WAV metadata for tests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WavMeta {
    /// Sample rate in Hz.
    pub sample_rate: u32,
    /// Channel count.
    pub channels: u16,
    /// Bits per sample (e.g. 16).
    pub bit_depth: u16,
    /// Cue sample positions.
    pub cue_markers: Vec<u32>,
}

/// Parse `bytes` as WAV with `fmt` and optional `cue` chunk.
pub fn parse_wav_with_cue(bytes: &[u8]) -> Result<WavMeta, &'static str> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("not wav");
    }
    let mut i = 12usize;
    let mut sample_rate = 0u32;
    let mut channels = 0u16;
    let mut bit_depth = 0u16;
    let mut cue_markers = Vec::new();
    while i + 8 <= bytes.len() {
        let id = &bytes[i..i + 4];
        let len = u32::from_le_bytes(bytes[i + 4..i + 8].try_into().unwrap()) as usize;
        i += 8;
        let end = i + len;
        if end > bytes.len() {
            return Err("truncated chunk");
        }
        let payload = &bytes[i..end];
        if id == b"fmt " && len >= 16 {
            channels = u16::from_le_bytes(payload[2..4].try_into().unwrap());
            sample_rate = u32::from_le_bytes(payload[4..8].try_into().unwrap());
            bit_depth = u16::from_le_bytes(payload[14..16].try_into().unwrap());
        } else if id == b"cue " && len >= 4 {
            let n = u32::from_le_bytes(payload[0..4].try_into().unwrap()) as usize;
            let mut p = 4usize;
            for _ in 0..n {
                if p + 24 > payload.len() {
                    break;
                }
                // Cue point: id u32, pos u32, ...
                let pos = u32::from_le_bytes(payload[p + 20..p + 24].try_into().unwrap());
                cue_markers.push(pos);
                p += 24;
            }
        }
        i = end + (len % 2);
    }
    Ok(WavMeta {
        sample_rate,
        channels,
        bit_depth,
        cue_markers,
    })
}
