//! Library-side propagation tick matching the `audio_propagation_system` loop from the integration
//! design. Host ECS code should call [`run_audio_propagation_tick`] once Bevy wiring exists.

use crossbeam_channel::Sender;
use glam::Vec3;

use crate::material::AcousticMaterialTable;
use crate::propagation::{compute_propagation, PropagationTraceInput};
use crate::spatial_audio::SpatialAudio;
use crate::spatial_index::SharedSpatialIndex;
use crate::store::PropagationResultStore;
use crate::{Entity, PropagationResult};

/// Shared inputs for [`run_audio_propagation_tick`] (maps ECS `Res` / frame data).
pub struct AudioPropagationEnvironment<'a> {
    /// World-space listener position.
    pub listener_position: Vec3,
    /// Shared spatial index for occlusion.
    pub index: &'a SharedSpatialIndex,
    /// Acoustic materials keyed by hit entity.
    pub materials: &'a AcousticMaterialTable,
    /// Worker-writable propagation slot store.
    pub store: &'a PropagationResultStore,
    /// Bounded channel to the audio thread.
    pub sender: &'a Sender<PropagationResult>,
    /// Current simulation frame.
    pub frame: u64,
}

/// Source inputs for one propagation trace (ECS `Query` row projection).
#[derive(Debug, Clone, Copy)]
pub struct PropagationSourceView {
    /// Source entity id.
    pub entity: Entity,
    /// World-space source position.
    pub position: Vec3,
    /// Spatial audio configuration for this source.
    pub spatial: SpatialAudio,
}

/// Runs one propagation tick: trace, write store slot, then `try_send` when `alive` returns true.
///
/// `entity_to_slot` maps each [`Entity`] to a disjoint store index (dense ECS table contract).
pub fn run_audio_propagation_tick(
    env: &AudioPropagationEnvironment<'_>,
    sources: &[PropagationSourceView],
    entity_to_slot: impl Fn(Entity) -> usize,
    alive: impl Fn(Entity) -> bool,
) {
    for src in sources {
        let input = PropagationTraceInput {
            source: src.entity,
            source_pos: src.position,
            listener_pos: env.listener_position,
            spatial: src.spatial,
        };
        let result = compute_propagation(&input, env.index, env.materials, env.frame);
        let slot = entity_to_slot(src.entity);
        env.store.write_slot(slot, result.clone());
        if alive(src.entity) {
            let _ = env.sender.try_send(result);
        }
    }
}
