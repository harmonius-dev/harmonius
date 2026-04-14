//! Replay recording, playback, seek, and kill-cam buffer (TC-8.6.*).

use std::collections::BTreeMap;

/// Records keyframes every `keyframe_interval` ticks and deltas otherwise.
#[derive(Clone, Debug)]
pub struct ReplayRecorder {
    pub keyframe_interval: u64,
    keyframes: Vec<u64>,
    deltas: u64,
    last_tick: u64,
}

impl ReplayRecorder {
    pub fn new(keyframe_interval: u64) -> Self {
        Self {
            keyframe_interval,
            keyframes: Vec::new(),
            deltas: 0,
            last_tick: 0,
        }
    }

    pub fn on_tick(&mut self, tick: u64) {
        self.last_tick = tick;
        if tick.is_multiple_of(self.keyframe_interval) {
            self.keyframes.push(tick);
        } else {
            self.deltas += 1;
        }
    }

    pub fn keyframe_count(&self) -> usize {
        self.keyframes.len()
    }

    pub fn delta_count(&self) -> u64 {
        self.deltas
    }
}

#[derive(Clone, Debug, Default)]
pub struct ReplayFile {
    pub checksums: BTreeMap<u64, u64>,
    pub keyframes: BTreeMap<u64, u64>,
}

impl ReplayFile {
    pub fn record_tick(&mut self, tick: u64, checksum: u64, is_keyframe: bool) {
        self.checksums.insert(tick, checksum);
        if is_keyframe {
            self.keyframes.insert(tick, checksum);
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReplayPlayer {
    file: ReplayFile,
}

impl ReplayPlayer {
    pub fn new(file: ReplayFile) -> Self {
        Self { file }
    }

    pub fn playback_checksums(&self) -> Vec<u64> {
        self.file.checksums.values().copied().collect()
    }

    pub fn seek_checksum(&self, target_tick: u64) -> Option<u64> {
        self.file.checksums.get(&target_tick).copied()
    }
}

#[derive(Clone, Debug)]
pub struct KillCamBuffer {
    pub window_ticks: usize,
    ring: Vec<u64>,
}

impl KillCamBuffer {
    pub fn new(window_secs: u32, tick_hz: u32) -> Self {
        let window_ticks = (window_secs as usize).saturating_mul(tick_hz as usize);
        Self {
            window_ticks,
            ring: Vec::new(),
        }
    }

    pub fn push_tick(&mut self, tick_hash: u64) {
        self.ring.push(tick_hash);
        if self.ring.len() > self.window_ticks {
            self.ring.remove(0);
        }
    }

    pub fn extract_clip(&self) -> &[u64] {
        &self.ring
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.6.1.1
    #[test]
    fn test_replay_keyframe_interval() {
        let mut rec = ReplayRecorder::new(30);
        for t in 0..90 {
            rec.on_tick(t);
        }
        assert_eq!(rec.keyframe_count(), 3);
        assert_eq!(rec.delta_count(), 87);
    }

    /// TC-8.6.2.1
    #[test]
    fn test_replay_deterministic_playback() {
        let mut f = ReplayFile::default();
        for t in (0..=1800).step_by(30) {
            let cs = (t as u64).wrapping_mul(1_001);
            f.record_tick(t, cs, t % 300 == 0);
        }
        let p1 = ReplayPlayer::new(f.clone());
        let p2 = ReplayPlayer::new(f);
        assert_eq!(p1.playback_checksums(), p2.playback_checksums());
    }

    /// TC-8.6.3.1
    #[test]
    fn test_replay_seek_to_midpoint() {
        let mut f = ReplayFile::default();
        for t in 0..100 {
            let kf = t % 30 == 0;
            f.record_tick(t, 1000 + t, kf);
        }
        let p = ReplayPlayer::new(f);
        let at = 45_u64;
        assert_eq!(p.seek_checksum(at), Some(1000 + at));
    }

    /// TC-8.6.5.1
    #[test]
    fn test_kill_cam_rolling_buffer() {
        let hz = 30;
        let mut kc = KillCamBuffer::new(10, hz);
        for t in 0..60 * hz {
            kc.push_tick(t as u64);
        }
        let clip = kc.extract_clip();
        assert_eq!(clip.len(), 10 * hz as usize);
        assert_eq!(clip[0], (60 * hz - 10 * hz) as u64);
    }
}
