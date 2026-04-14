//! Rumble patterns, adaptive triggers, and audio-driven haptics helpers.

use crate::device::DeviceCapabilities;

/// Low / high frequency motor targets 0..1.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DualMotorState {
    /// Low-frequency motor amplitude.
    pub low_freq: f32,
    /// High-frequency motor amplitude.
    pub high_freq: f32,
}

/// Attack / sustain / decay envelope for rumble patterns.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RumbleEnvelope {
    /// Attack rise time (ms).
    pub attack: f32,
    /// Sustain plateau (ms).
    pub sustain: f32,
    /// Decay tail (ms).
    pub decay: f32,
}

/// Pattern definition combining envelope and peak.
#[derive(Clone, Debug, PartialEq)]
pub struct RumblePattern {
    /// Envelope shape.
    pub envelope: RumbleEnvelope,
    /// Peak amplitude 0..1.
    pub peak: f32,
}

/// Plays one-shot rumble envelopes for tests and gameplay.
#[derive(Clone, Debug)]
pub struct PatternPlayer {
    elapsed_ms: f32,
    active: bool,
    pattern: Option<RumblePattern>,
    intensity: f32,
}

impl Default for PatternPlayer {
    fn default() -> Self {
        Self {
            elapsed_ms: 0.0,
            active: false,
            pattern: None,
            intensity: 1.0,
        }
    }
}

impl PatternPlayer {
    /// Arm a pattern at a normalized intensity.
    pub fn play(&mut self, pattern: &RumblePattern, intensity: f32) {
        self.pattern = Some(pattern.clone());
        self.intensity = intensity.clamp(0.0, 1.0);
        self.elapsed_ms = 0.0;
        self.active = true;
    }

    /// Advance the player and return the instantaneous motor mix.
    pub fn tick(&mut self, dt: f32) -> DualMotorState {
        if !self.active {
            return DualMotorState::default();
        }
        let Some(p) = &self.pattern else {
            return DualMotorState::default();
        };
        self.elapsed_ms += dt * 1000.0;
        let e = &p.envelope;
        let t = self.elapsed_ms;
        let amp = if t <= e.attack {
            (t / e.attack.max(1e-3)) * p.peak
        } else if t <= e.attack + e.sustain {
            p.peak
        } else if t <= e.attack + e.sustain + e.decay {
            let u = (t - e.attack - e.sustain) / e.decay.max(1e-3);
            p.peak * (1.0 - u)
        } else {
            self.active = false;
            0.0
        };
        let a = amp * self.intensity;
        DualMotorState {
            low_freq: a,
            high_freq: a * 0.25,
        }
    }

    /// Stop playback immediately.
    pub fn stop_all(&mut self) {
        self.active = false;
        self.pattern = None;
        self.elapsed_ms = 0.0;
    }
}

/// DualSense-style adaptive trigger effect (subset for tests).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdaptiveTriggerEffect {
    /// Constant resistance curve.
    Feedback,
    /// Weapon pull / per-game semantics.
    Weapon,
    /// Oscillating vibration on the trigger.
    Vibration,
}

/// Simplified trigger effect payload for drivers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TriggerEffect {
    /// Normalized strength 0..1.
    pub strength: f32,
}

/// Stub driver that respects capability bits.
#[derive(Clone, Copy, Debug, Default)]
pub struct AdaptiveTriggerDriver;

impl AdaptiveTriggerDriver {
    /// Apply an effect; clears `hid_out` when adaptive triggers are unavailable.
    pub fn apply(
        &self,
        caps: &DeviceCapabilities,
        _effect: &TriggerEffect,
        hid_out: &mut Vec<u8>,
    ) -> Result<(), HapticError> {
        hid_out.clear();
        if caps.has_adaptive_triggers {
            hid_out.extend_from_slice(&[0x01, (255.0 * _effect.strength) as u8]);
        }
        Ok(())
    }
}

/// Driver errors (reserved for future I/O failures).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HapticError {
    /// Device removed mid-transaction.
    DeviceLost,
}

/// Known backend families for intensity normalization tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HapticBackendKind {
    /// Xbox 0..255 motors.
    Xbox,
    /// DualSense 0..1 normalized voice-coil.
    DualSense,
    /// Switch HD rumble linearized 0..1.
    Switch,
    /// Steam Input abstract 0..1.
    SteamInput,
}

/// Abstract intensity command 0..1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HapticCommand {
    /// Normalized intensity.
    pub intensity: f32,
}

/// Map abstract intensity to backend-native units.
pub fn normalize_intensity(cmd: &HapticCommand, backend: HapticBackendKind) -> f32 {
    let x = cmd.intensity.clamp(0.0, 1.0);
    match backend {
        HapticBackendKind::Xbox => (255.0 * x).round(),
        HapticBackendKind::DualSense => x,
        HapticBackendKind::Switch => x,
        HapticBackendKind::SteamInput => x,
    }
}

/// Extremely small Goertzel-style probe for audio→haptic tests.
#[derive(Clone, Debug, Default)]
pub struct AudioHapticGenerator {
    /// Sample rate (Hz).
    pub sample_rate: f32,
}

/// Encode dual-motor amplitudes into two independent HID bytes (tests only).
pub fn encode_dual_motor_hid(low: f32, high: f32) -> [u8; 2] {
    [
        (low.clamp(0.0, 1.0) * 255.0).round() as u8,
        (high.clamp(0.0, 1.0) * 255.0).round() as u8,
    ]
}

/// Parse editor-authored adaptive trigger profile keywords.
pub fn parse_adaptive_profile_line(line: &str) -> Option<AdaptiveTriggerEffect> {
    match line.trim().to_ascii_lowercase().as_str() {
        "feedback" => Some(AdaptiveTriggerEffect::Feedback),
        "weapon" => Some(AdaptiveTriggerEffect::Weapon),
        "vibration" => Some(AdaptiveTriggerEffect::Vibration),
        _ => None,
    }
}

impl AudioHapticGenerator {
    /// Measure narrow-band energy near `target_hz` over `samples`.
    pub fn band_energy(&self, samples: &[f32], target_hz: f32) -> f32 {
        let k = (2.0 * std::f32::consts::PI * target_hz / self.sample_rate).max(1e-6);
        let mut s = 0.0f32;
        let mut c = 0.0f32;
        for (n, &x) in samples.iter().enumerate() {
            let phase = k * n as f32;
            s += x * phase.sin();
            c += x * phase.cos();
        }
        (s * s + c * c).sqrt() / samples.len().max(1) as f32
    }
}
