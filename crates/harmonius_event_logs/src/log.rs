//! Bounded ring-buffer event log.

use std::collections::VecDeque;

use rkyv::{Archive, Deserialize, Serialize};
use smallvec::SmallVec;

use crate::decay::accuracy_at_tick;
use crate::entry::DecayingEntry;
use crate::types::{Entity, EntryId, EventLogId, GameplayTag, SpatialDim};
use crate::DecayCurve;

/// Bounded ring buffer of typed entries with timestamps.
#[derive(Clone, Debug, Archive, Serialize, Deserialize)]
pub struct EventLog<T>
where
    T: Clone + rkyv::Archive,
{
    /// Unique log identifier.
    pub id: EventLogId,
    /// Maximum number of entries.
    pub capacity: u32,
    /// How entries decay over time.
    pub decay_curve: DecayCurve,
    /// Tick cadence metadata for consumers.
    pub tick_rate: u32,
    /// Stored entries in oldest-first order (FIFO eviction at capacity).
    pub(crate) entries: VecDeque<DecayingEntry<T>>,
    pub(crate) next_entry_id: u32,
}

impl<T: Clone + rkyv::Archive> EventLog<T> {
    /// Creates a new empty log.
    pub fn new(id: EventLogId, capacity: u32, decay_curve: DecayCurve, tick_rate: u32) -> Self {
        Self {
            id,
            capacity,
            decay_curve,
            tick_rate,
            entries: VecDeque::new(),
            next_entry_id: 0,
        }
    }

    /// Pushes a new entry with empty gameplay tags.
    pub fn push(
        &mut self,
        data: T,
        tick: u64,
        source: Option<Entity>,
        position: Option<SpatialDim>,
    ) {
        self.push_with_tags(data, tick, source, position, Vec::new());
    }

    /// Pushes a new entry carrying gameplay tags for propagation filters.
    pub fn push_with_tags(
        &mut self,
        data: T,
        tick: u64,
        source: Option<Entity>,
        position: Option<SpatialDim>,
        tags: Vec<GameplayTag>,
    ) {
        let cap = self.capacity as usize;
        if cap == 0 {
            return;
        }
        let id = EntryId(self.next_entry_id);
        self.next_entry_id = self.next_entry_id.wrapping_add(1);
        let entry = DecayingEntry {
            id,
            data,
            timestamp: tick,
            accuracy: 1.0,
            source,
            position,
            hop_count: 0,
            tags,
        };
        if self.entries.len() == cap {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    /// Borrows all valid entries in oldest-first order.
    pub fn entries(&self) -> std::collections::vec_deque::Iter<'_, DecayingEntry<T>> {
        self.entries.iter()
    }

    /// Entries whose accuracy is strictly above `threshold`.
    pub fn entries_above_accuracy(&self, threshold: f32) -> SmallVec<[&DecayingEntry<T>; 8]> {
        self.entries
            .iter()
            .filter(|e| e.accuracy > threshold)
            .collect()
    }

    /// Entries whose timestamps fall in `from_tick..=to_tick` inclusive.
    pub fn entries_in_window(
        &self,
        from_tick: u64,
        to_tick: u64,
    ) -> SmallVec<[&DecayingEntry<T>; 8]> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= from_tick && e.timestamp <= to_tick)
            .collect()
    }

    /// Most recently added entry, if any.
    pub fn most_recent(&self) -> Option<&DecayingEntry<T>> {
        self.entries.back()
    }

    /// Applies decay based on elapsed ticks since each entry's timestamp.
    ///
    /// When `tick_rate` is greater than `1`, decay is only recomputed on ticks where
    /// `current_tick % tick_rate == 0`, so accuracy changes at the configured cadence.
    pub fn decay_tick(&mut self, current_tick: u64) {
        let rate = self.tick_rate.max(1) as u64;
        if !current_tick.is_multiple_of(rate) {
            return;
        }
        for entry in &mut self.entries {
            let elapsed = current_tick.saturating_sub(entry.timestamp);
            entry.accuracy = accuracy_at_tick(&self.decay_curve, elapsed);
        }
    }

    /// Removes entries whose accuracy is strictly below `min_accuracy`.
    pub fn prune_below(&mut self, min_accuracy: f32) {
        self.entries.retain(|e| e.accuracy >= min_accuracy);
    }

    /// Number of valid entries.
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// True when the log holds `capacity` entries.
    pub fn is_full(&self) -> bool {
        self.entries.len() == self.capacity as usize
    }

    /// Clears all entries without resetting identifiers.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
impl<T: Clone + rkyv::Archive> EventLog<T> {
    /// Sets `accuracy` on the entry at `index` in oldest-first storage order.
    pub(crate) fn test_set_entry_accuracy(&mut self, index: usize, accuracy: f32) {
        if let Some(entry) = self.entries.get_mut(index) {
            entry.accuracy = accuracy;
        }
    }
}
