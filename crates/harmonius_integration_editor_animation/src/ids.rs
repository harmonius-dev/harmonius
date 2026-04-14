//! Stable identifiers shared across editor and animation domains.

/// Opaque entity identifier in the ECS world.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Entity(pub u64);

/// Index into the engine-wide float parameter table (`u16` per state-machine design).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ParameterId(pub u16);

/// Interned string token for bone names in pose snapshots.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StringId(pub u32);

/// Index of a bone track inside an animation clip.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BoneTrackIndex(pub u32);

/// Identifier for a state node inside a [`crate::StateGraphDef`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StateId(pub u32);
