//! Adaptive reordering buffer for unreliable voice datagrams.

use crate::voice_packet::VoicePacket;

/// Reorders voice packets by `sequence` using a fixed 16-slot ring.
#[derive(Debug)]
pub struct JitterBuffer {
    buffer: [Option<VoicePacket>; 16],
    /// Current adaptive depth in milliseconds.
    pub depth_ms: f32,
    /// Target depth range (min, max).
    pub depth_range: (f32, f32),
    /// Next expected sequence number.
    pub next_sequence: u32,
    /// Sustained starvation duration in milliseconds.
    pub starvation_ms: f32,
    /// Set after sustained starvation exceeds the pause threshold.
    pub voice_paused: bool,
    /// Counts starvation pause events (metric hook).
    pub voice_starvation_events: u32,
}

impl JitterBuffer {
    const PAUSE_MS: f32 = 500.0;

    /// Builds a buffer with the given adaptive depth range in milliseconds.
    ///
    /// `next_sequence` starts at `1`, matching the design’s first expected datagram sequence after
    /// bootstrap; call [`JitterBuffer::push`](Self::push) after starvation reset to realign if the
    /// wire path begins at `0`.
    #[must_use]
    pub fn new(min_depth_ms: f32, max_depth_ms: f32) -> Self {
        Self {
            buffer: std::array::from_fn(|_| None),
            depth_ms: min_depth_ms,
            depth_range: (min_depth_ms, max_depth_ms),
            next_sequence: 1,
            starvation_ms: 0.0,
            voice_paused: false,
            voice_starvation_events: 0,
        }
    }

    fn clear_buffer(&mut self) {
        for slot in &mut self.buffer {
            *slot = None;
        }
    }

    /// Inserts a received packet; overwrites any colliding `sequence % 16` slot.
    pub fn push(&mut self, packet: VoicePacket) {
        if self.voice_paused {
            self.clear_buffer();
            self.next_sequence = packet.sequence;
            self.voice_paused = false;
            self.starvation_ms = 0.0;
        }
        let idx = packet.sequence as usize % 16;
        self.buffer[idx] = Some(packet);
    }

    /// Dequeues the next in-sequence packet, if available.
    pub fn pop(&mut self) -> Option<VoicePacket> {
        let idx = self.next_sequence as usize % 16;
        let took = match self.buffer[idx].as_ref() {
            Some(p) if p.sequence == self.next_sequence => self.buffer[idx].take(),
            _ => None,
        };
        if let Some(p) = took {
            self.starvation_ms = 0.0;
            self.next_sequence = self.next_sequence.wrapping_add(1);
            return Some(p);
        }
        None
    }

    /// Accumulates wall time after a failed `pop` (no packet ready).
    pub fn record_idle(&mut self, dt_ms: f32) {
        self.starvation_ms += dt_ms;
        if self.starvation_ms > Self::PAUSE_MS && !self.voice_paused {
            self.voice_paused = true;
            self.voice_starvation_events = self.voice_starvation_events.saturating_add(1);
        }
    }

    /// Updates adaptive depth from measured network jitter (milliseconds).
    pub fn adapt(&mut self, jitter_ms: f32) {
        self.depth_ms = (0.875 * f64::from(self.depth_ms) + 0.125 * f64::from(jitter_ms)) as f32;
        self.depth_ms = self.depth_ms.clamp(self.depth_range.0, self.depth_range.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::ConnectionId;
    use crate::voice_packet::VoiceChannelId;

    fn pkt(seq: u32) -> VoicePacket {
        VoicePacket {
            sequence: seq,
            sender: ConnectionId(1),
            channel: VoiceChannelId::Proximity,
            auth_tag: [0u8; 8],
            opus_data: [0u8; 256],
            opus_len: 0,
        }
    }

    /// TC-IR-4.3.3.1 — packets 3,1,2 arrive out of order; dequeue 1,2,3.
    #[test]
    fn tc_ir_4_3_3_1_reorder_packets() {
        let mut jb = JitterBuffer::new(20.0, 60.0);
        jb.next_sequence = 1;
        jb.push(pkt(3));
        jb.push(pkt(1));
        jb.push(pkt(2));
        assert_eq!(jb.pop(), Some(pkt(1)));
        assert_eq!(jb.pop(), Some(pkt(2)));
        assert_eq!(jb.pop(), Some(pkt(3)));
    }

    /// TC-IR-4.3.3.2 — jitter spike grows adaptive depth toward the clamped max.
    #[test]
    fn tc_ir_4_3_3_2_adapt_clamps() {
        let mut jb = JitterBuffer::new(20.0, 60.0);
        jb.depth_ms = 20.0;
        jb.adapt(40.0);
        assert!(jb.depth_ms > 20.0);
        for _ in 0..32 {
            jb.adapt(200.0);
        }
        assert!((jb.depth_ms - 60.0).abs() < 1.0);
    }

    /// TC-IR-4.3.3.3 — sustained idle past 500 ms pauses voice.
    #[test]
    fn tc_ir_4_3_3_3_starvation_pause() {
        let mut jb = JitterBuffer::new(20.0, 60.0);
        jb.next_sequence = 1;
        for _ in 0..12 {
            jb.record_idle(50.0);
        }
        assert!(jb.voice_paused);
        assert!(jb.voice_starvation_events >= 1);
    }

    /// TC-IR-4.3.3.4 — after pause, a new sequence resumes without panic.
    #[test]
    fn tc_ir_4_3_3_4_starvation_recovery() {
        let mut jb = JitterBuffer::new(20.0, 60.0);
        jb.next_sequence = 1;
        for _ in 0..12 {
            jb.record_idle(50.0);
        }
        assert!(jb.voice_paused);
        jb.push(pkt(42));
        assert!(!jb.voice_paused);
        assert_eq!(jb.pop(), Some(pkt(42)));
    }
}
