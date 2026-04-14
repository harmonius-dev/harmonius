//! Audio commands produced by the bridge.

use crate::ids::{AssetHandle, AudioClip, VoiceId};
use crate::math::Vec3;
use crate::zones::{ReverbParams, ReverbZoneId};

/// Dispatch timing for an audio command.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AudioTimestamp {
    /// Play at start of next buffer callback.
    Immediate,
    /// Sample-accurate offset from the current audio clock.
    SampleOffset(u64),
}

/// Logical audio bus for routing and mixing.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum BusId {
    /// Master bus.
    Master = 0,
    /// Sound effects bus.
    Sfx = 1,
    /// Ambient bed bus.
    Ambient = 2,
    /// Music bus.
    Music = 3,
    /// User-interface bus.
    Ui = 4,
    /// Voice/dialog bus.
    Voice = 5,
}

/// Voice priority for steal and overflow ordering.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum VoicePriority {
    /// Lowest priority; dropped first on queue pressure.
    Low = 0,
    /// Medium priority.
    Medium = 1,
    /// High priority; survives longest under pressure.
    High = 2,
}

/// Runtime voice parameter targets for [`AudioCommand::SetParam`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum VoiceParam {
    /// Linear gain.
    Gain = 0,
    /// Playback pitch multiplier.
    Pitch = 1,
    /// Stereo pan.
    Pan = 2,
    /// Low-pass cutoff frequency.
    LowPassCutoff = 3,
    /// High-pass cutoff frequency.
    HighPassCutoff = 4,
}

impl VoiceParam {
    /// Decodes a forward-compatible wire tag; unknown tags return `None`.
    pub fn from_wire(tag: u8) -> Option<Self> {
        match tag {
            0 => Some(Self::Gain),
            1 => Some(Self::Pitch),
            2 => Some(Self::Pan),
            3 => Some(Self::LowPassCutoff),
            4 => Some(Self::HighPassCutoff),
            _ => None,
        }
    }
}

/// Commands enqueued for the real-time audio thread.
#[derive(Clone, Debug, PartialEq)]
pub enum AudioCommand {
    /// Start or continue one-shot / looping playback.
    Play {
        /// Voice slot.
        voice_id: VoiceId,
        /// Clip asset.
        clip: AssetHandle<AudioClip>,
        /// Output bus.
        bus: BusId,
        /// Steal / overflow priority.
        priority: VoicePriority,
        /// World-space position when spatialized.
        position: Option<Vec3>,
        /// Scheduling hint.
        timestamp: AudioTimestamp,
        /// Linear gain.
        gain: f32,
        /// Pitch multiplier.
        pitch: f32,
    },
    /// Stop playback with optional fade.
    Stop {
        /// Voice slot.
        voice_id: VoiceId,
        /// Fade length in samples.
        fade_samples: u32,
        /// Scheduling hint.
        timestamp: AudioTimestamp,
    },
    /// Update a live voice parameter.
    SetParam {
        /// Voice slot.
        voice_id: VoiceId,
        /// Target parameter.
        param: VoiceParam,
        /// Parameter value interpretation is parameter-specific.
        value: f32,
        /// Scheduling hint.
        timestamp: AudioTimestamp,
    },
    /// Activates a reverb zone preset.
    ActivateReverb {
        /// Zone identifier.
        zone_id: ReverbZoneId,
        /// Zone parameters snapshot.
        params: ReverbParams,
    },
    /// Deactivates a reverb zone preset.
    DeactivateReverb {
        /// Zone identifier.
        zone_id: ReverbZoneId,
    },
}
