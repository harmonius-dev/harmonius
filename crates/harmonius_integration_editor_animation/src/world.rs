//! Minimal editor world surface for undo/redo command unit tests.

use std::collections::HashMap;

use crate::animation_data::{AnimEventMarker, BlendSample2D, Keyframe, TangentPair, TrackKey};
use crate::ids::{BoneTrackIndex, Entity};

/// Authoring-time scratch state mutated by [`crate::EditorCommand`] implementations.
#[derive(Debug, Default)]
pub struct World {
    keyframes: HashMap<TrackKey, Keyframe>,
    tangents: HashMap<(Entity, BoneTrackIndex, u32), TangentPair>,
    markers: HashMap<Entity, AnimEventMarker>,
    blend_samples: HashMap<Entity, BlendSample2D>,
}

impl World {
    /// Reads the current keyframe slot for a track (authoring convenience).
    #[must_use]
    pub fn keyframe(&self, key: TrackKey) -> Option<Keyframe> {
        self.keyframes.get(&key).copied()
    }

    /// Reads the tangent pair for a keyframe index on a track.
    #[must_use]
    pub fn tangent(
        &self,
        entity: Entity,
        track: BoneTrackIndex,
        keyframe_index: u32,
    ) -> Option<TangentPair> {
        self.tangents.get(&(entity, track, keyframe_index)).copied()
    }

    /// Reads the active event marker for an entity (single-slot authoring stub).
    #[must_use]
    pub fn event_marker(&self, entity: Entity) -> Option<AnimEventMarker> {
        self.markers.get(&entity).copied()
    }

    /// Reads the active blend sample for an entity (single-slot authoring stub).
    #[must_use]
    pub fn blend_sample(&self, entity: Entity) -> Option<BlendSample2D> {
        self.blend_samples.get(&entity).copied()
    }

    pub(crate) fn set_keyframe(&mut self, key: TrackKey, value: Option<Keyframe>) {
        match value {
            Some(k) => {
                self.keyframes.insert(key, k);
            }
            None => {
                self.keyframes.remove(&key);
            }
        }
    }

    pub(crate) fn set_tangent(
        &mut self,
        entity: Entity,
        track: BoneTrackIndex,
        keyframe_index: u32,
        value: TangentPair,
    ) {
        self.tangents.insert((entity, track, keyframe_index), value);
    }

    pub(crate) fn set_marker(&mut self, entity: Entity, value: Option<AnimEventMarker>) {
        match value {
            Some(m) => {
                self.markers.insert(entity, m);
            }
            None => {
                self.markers.remove(&entity);
            }
        }
    }

    pub(crate) fn set_blend_sample(&mut self, entity: Entity, value: Option<BlendSample2D>) {
        match value {
            Some(s) => {
                self.blend_samples.insert(entity, s);
            }
            None => {
                self.blend_samples.remove(&entity);
            }
        }
    }
}
