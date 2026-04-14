//! Identifier and supporting types for event logs.

use rkyv::{Archive, Deserialize, Serialize};

/// Stable entity identifier used by event log entries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub struct Entity(pub u64);

/// Unique identifier for a single log entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub struct EntryId(pub u32);

/// Unique identifier for an event log instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub struct EventLogId(pub u32);

/// Compact gameplay tag token (intern table lives outside this crate).
pub type GameplayTag = u32;

/// Opaque asset identifier for threshold actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub struct AssetId(pub u32);

/// Predicate identifier resolved by the editor codegen pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub struct PredicateId(pub u32);

/// Event type discriminator for queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub struct EventTypeId(pub u32);

/// Typed world position for an event entry.
#[derive(Clone, Copy, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub enum SpatialDim {
    /// Two-dimensional sample.
    D2([f32; 2]),
    /// Three-dimensional sample.
    D3([f32; 3]),
}

/// Tag filter list used by propagation rules.
#[derive(Clone, Debug, Default, PartialEq, Eq, Archive, Serialize, Deserialize)]
pub struct TagSet(pub Vec<GameplayTag>);

impl TagSet {
    /// Returns an empty tag set.
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Returns true when `tag` is listed in this set.
    pub fn contains(&self, tag: GameplayTag) -> bool {
        self.0.contains(&tag)
    }

    /// Returns true when any entry tag matches this set (or the set is empty = match all).
    pub fn matches_entry_tags(&self, entry_tags: &[GameplayTag]) -> bool {
        if self.0.is_empty() {
            return true;
        }
        entry_tags.iter().any(|&t| self.contains(t))
    }

    /// Builds a filter from explicit tag ids (tests and tools).
    pub fn from_ids(ids: &[GameplayTag]) -> Self {
        Self(ids.to_vec())
    }
}
