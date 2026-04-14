//! Trigger volume components for reverb and ambient loops.

use crate::ids::{AssetHandle, AudioClip, VoiceId};

/// Stable identifier for a reverb zone preset.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ReverbZoneId(pub u32);

/// Reverb parameters carried into [`crate::commands::AudioCommand::ActivateReverb`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ReverbParams {
    /// Wet/dry mix in `[0, 1]`.
    pub wet: f32,
}

/// Reverb zone marker on a trigger entity.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ReverbZone {
    /// Zone id for activate/deactivate pairing.
    pub id: ReverbZoneId,
    /// Tunable preset parameters.
    pub params: ReverbParams,
}

/// Ambient loop source on a trigger entity.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmbientLoop {
    /// Clip to loop.
    pub clip: AssetHandle<AudioClip>,
    /// Linear gain for the ambient bed.
    pub gain: f32,
    /// Runtime voice handle once started.
    pub active_voice: Option<VoiceId>,
}

/// Snapshot of zone components for a trigger entity used by [`crate::bridge::handle_trigger_zone`].
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TriggerZoneSnapshot {
    /// Optional reverb zone.
    pub reverb: Option<ReverbZone>,
    /// Optional ambient loop definition.
    pub ambient: Option<AmbientLoop>,
}
