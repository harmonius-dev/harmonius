//! Single log entry representation.

use rkyv::{Archive, Deserialize, Serialize};

use crate::types::{Entity, EntryId, GameplayTag, SpatialDim};

/// One timestamped, decaying record inside an [`EventLog`](crate::EventLog).
#[derive(Clone, Debug, Archive, Serialize, Deserialize)]
pub struct DecayingEntry<T>
where
    T: Clone + rkyv::Archive,
{
    /// Unique entry identifier within this log.
    pub id: EntryId,
    /// Typed payload.
    pub data: T,
    /// Game tick when this entry was recorded.
    pub timestamp: u64,
    /// Confidence in this memory; `1.0` is fresh.
    pub accuracy: f32,
    /// Entity that caused this event, if any.
    pub source: Option<Entity>,
    /// World position sample, if known.
    pub position: Option<SpatialDim>,
    /// Propagation hop count (`0` is first-hand).
    pub hop_count: u8,
    /// Gameplay tags attached to this entry.
    pub tags: Vec<GameplayTag>,
}
