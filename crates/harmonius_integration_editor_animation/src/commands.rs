//! Undoable editor commands for animation authoring (in-memory only).

use crate::animation_data::{AnimEventMarker, BlendSample2D, Keyframe, TangentPair, TrackKey};
use crate::ids::{BoneTrackIndex, Entity};
use crate::world::World;

/// Recoverable editor command failure (no panics in library paths).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandError {
    /// Authoring invariant was violated (duplicate apply, missing slot, etc.).
    InvariantViolated(&'static str),
}

/// Harmonius editor undo stack entry point for tools code.
pub trait EditorCommand: core::fmt::Debug {
    /// Short human-readable label for UI menus.
    fn description(&self) -> &'static str;

    /// Applies the command forward to `world`.
    fn execute(&self, world: &mut World) -> Result<(), CommandError>;

    /// Reverts [`Self::execute`].
    fn undo(&self, world: &mut World) -> Result<(), CommandError>;

    /// Approximate heap footprint for memory telemetry.
    fn size_bytes(&self) -> usize {
        core::mem::size_of_val(self)
    }
}

/// Keyframe insert / delete / move.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyframeEditCommand {
    /// Target entity.
    pub entity: Entity,
    /// Affected bone track.
    pub track: BoneTrackIndex,
    /// Previous keyframe state before this edit.
    pub old_keyframe: Option<Keyframe>,
    /// New keyframe state after this edit.
    pub new_keyframe: Option<Keyframe>,
}

impl EditorCommand for KeyframeEditCommand {
    fn description(&self) -> &'static str {
        "edit keyframe"
    }

    fn execute(&self, world: &mut World) -> Result<(), CommandError> {
        let key = TrackKey {
            entity: self.entity,
            track: self.track,
        };
        world.set_keyframe(key, self.new_keyframe);
        Ok(())
    }

    fn undo(&self, world: &mut World) -> Result<(), CommandError> {
        let key = TrackKey {
            entity: self.entity,
            track: self.track,
        };
        world.set_keyframe(key, self.old_keyframe);
        Ok(())
    }
}

/// Curve tangent manipulation.
#[derive(Clone, Debug, PartialEq)]
pub struct TangentEditCommand {
    /// Target entity.
    pub entity: Entity,
    /// Affected bone track.
    pub track: BoneTrackIndex,
    /// Keyframe row index inside the clip.
    pub keyframe_index: u32,
    /// Tangents before the edit.
    pub old_tangent: TangentPair,
    /// Tangents after the edit.
    pub new_tangent: TangentPair,
}

impl EditorCommand for TangentEditCommand {
    fn description(&self) -> &'static str {
        "edit tangent"
    }

    fn execute(&self, world: &mut World) -> Result<(), CommandError> {
        world.set_tangent(
            self.entity,
            self.track,
            self.keyframe_index,
            self.new_tangent,
        );
        Ok(())
    }

    fn undo(&self, world: &mut World) -> Result<(), CommandError> {
        world.set_tangent(
            self.entity,
            self.track,
            self.keyframe_index,
            self.old_tangent,
        );
        Ok(())
    }
}

/// Animation event marker add / remove / move.
#[derive(Clone, Debug, PartialEq)]
pub struct EventMarkerEditCommand {
    /// Target entity.
    pub entity: Entity,
    /// Marker before the edit.
    pub old_marker: Option<AnimEventMarker>,
    /// Marker after the edit.
    pub new_marker: Option<AnimEventMarker>,
}

impl EditorCommand for EventMarkerEditCommand {
    fn description(&self) -> &'static str {
        "edit event marker"
    }

    fn execute(&self, world: &mut World) -> Result<(), CommandError> {
        world.set_marker(self.entity, self.new_marker);
        Ok(())
    }

    fn undo(&self, world: &mut World) -> Result<(), CommandError> {
        world.set_marker(self.entity, self.old_marker);
        Ok(())
    }
}

/// Blend space sample add / remove / reposition.
#[derive(Clone, Debug, PartialEq)]
pub struct BlendSampleEditCommand {
    /// Target entity owning the blend space asset.
    pub entity: Entity,
    /// Sample before the edit.
    pub old_sample: Option<BlendSample2D>,
    /// Sample after the edit.
    pub new_sample: Option<BlendSample2D>,
}

impl EditorCommand for BlendSampleEditCommand {
    fn description(&self) -> &'static str {
        "edit blend sample"
    }

    fn execute(&self, world: &mut World) -> Result<(), CommandError> {
        world.set_blend_sample(self.entity, self.new_sample);
        Ok(())
    }

    fn undo(&self, world: &mut World) -> Result<(), CommandError> {
        world.set_blend_sample(self.entity, self.old_sample);
        Ok(())
    }
}
