//! Audio propagation integration between spatial indexing and the audio thread.
//!
//! Implements the data contracts described in `docs/design/integration/audio-spatial-awareness.md`:
//! occlusion rays against a shared spatial index, acoustic material lookup, bounded MPSC bridging,
//! and deterministic amortization helpers.

#![deny(clippy::all)]
#![allow(clippy::module_name_repetitions)]

pub mod amortization;
pub mod channel;
pub mod entity;
pub mod material;
pub mod propagation;
pub mod snapshot;
pub mod spatial_audio;
pub mod spatial_index;
pub mod store;
pub mod system;
pub mod voice;

pub use amortization::{
    amortized_trace_count, should_retrace_audio_propagation, should_retrace_source,
    SourceTraceState,
};
pub use channel::{
    propagation_channel_pair, AudioPropagationReceiver, AudioPropagationSender, MPSC_CAPACITY,
};
pub use entity::Entity;
pub use material::{AcousticMaterial, AcousticMaterialTable};
pub use propagation::{
    compute_propagation, compute_propagation_with_dirs, db_to_linear_loss,
    occlusion_ray_directions, select_top_reflections, PropagationTraceInput,
};
pub use snapshot::PropagationSnapshot;
pub use spatial_audio::SpatialAudio;
pub use spatial_index::{AxisAlignedSurface, SharedSpatialIndex, SurfaceHit};
pub use store::PropagationResultStore;
pub use system::{run_audio_propagation_tick, AudioPropagationEnvironment, PropagationSourceView};
pub use voice::{apply_propagation_to_voice, VoiceFilterState, MAX_VOICE_LPF_HZ, MIN_VOICE_LPF_HZ};

use glam::Vec3;

/// Single early-reflection tap delivered to the audio thread.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReflectionTap {
    /// Delay in milliseconds along the reflected path.
    pub delay_ms: f32,
    /// Linear energy of the tap.
    pub gain: f32,
    /// Outgoing reflection direction (unit vector when possible).
    pub direction: Vec3,
}

/// Result of propagation tracing for one source.
#[derive(Debug, Clone, PartialEq)]
pub struct PropagationResult {
    /// Source entity this result belongs to.
    pub source: Entity,
    /// `0.0` = fully occluded, `1.0` = line of sight.
    pub occlusion: f32,
    /// Per-band transmission loss as linear attenuation in \[0, 1\] (higher is more lossy).
    pub band_loss: [f32; 3],
    /// Early reflection taps (delay + gain pairs).
    pub reflections: [ReflectionTap; 8],
    /// Number of valid entries in `reflections`.
    pub reflection_count: u8,
    /// Reverb send level derived from geometry.
    pub reverb_send: f32,
    /// Frame number when last updated.
    pub last_updated_frame: u64,
}

impl PropagationResult {
    /// Line-of-sight defaults for a new or untraced source.
    #[must_use]
    pub fn line_of_sight_default(source: Entity) -> Self {
        Self {
            source,
            occlusion: 1.0,
            band_loss: [0.0, 0.0, 0.0],
            reflections: [ReflectionTap {
                delay_ms: 0.0,
                gain: 0.0,
                direction: Vec3::ZERO,
            }; 8],
            reflection_count: 0,
            reverb_send: 0.0,
            last_updated_frame: 0,
        }
    }
}
