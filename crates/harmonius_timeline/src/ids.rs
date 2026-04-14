//! Stable identifiers used by timeline assets and playback.

/// Opaque asset handle for a baked timeline.
#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize,
)]
pub struct AssetId(pub u64);

/// ECS entity placeholder until engine wiring lands in this crate.
#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize,
)]
pub struct Entity(pub u32);

/// Stable id for a keyframe within a track.
#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize,
)]
pub struct KeyframeId(pub u32);

/// Stable id for a track inside a multi-track timeline.
#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize,
)]
pub struct TrackId(pub u16);
