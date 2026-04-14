//! Bounded one-way queue modeling `CH-22` from the integration design.
//!
//! This is a deterministic, single-threaded `VecDeque` stand-in for the threaded MPSC handoff in
//! the full engine. Backpressure (`FM-1`) is modeled without draining into `GameWorld` outside
//! `HeadlessHarness::run_frame` pre-phase drains (see `harness` module).

use std::collections::VecDeque;

use crate::mutation::EditorMutation;
use crate::world::GameWorld;

/// Capacity for editor → runtime mutation staging (`CH-22` in the design doc).
pub const CH_22_CAPACITY: usize = 256;

/// One-way MPSC-style bridge from editor staging into the runtime world.
#[derive(Debug, Default)]
pub struct EventBridge {
    queue: VecDeque<EditorMutation>,
    /// Count of observed `FM-1` backpressure events (enqueue while full).
    pub fm1_backpressure_events: u64,
    /// Count of observed `FM-2` mutation id collisions (deduped while enqueueing).
    pub fm2_id_collision_events: u64,
}

impl EventBridge {
    /// Creates an empty bridge with zeroed telemetry counters.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Attempts to enqueue a mutation; returns the mutation on `CH-22` backpressure.
    pub fn try_enqueue(&mut self, mutation: EditorMutation) -> Result<(), EditorMutation> {
        if self.queue.len() >= CH_22_CAPACITY {
            return Err(mutation);
        }
        let had_dup = self
            .queue
            .iter()
            .any(|existing| existing.mutation_id == mutation.mutation_id);
        if had_dup {
            self.queue
                .retain(|existing| existing.mutation_id != mutation.mutation_id);
            self.fm2_id_collision_events = self.fm2_id_collision_events.saturating_add(1);
        }
        self.queue.push_back(mutation);
        Ok(())
    }

    /// Drains every queued mutation into `game` in FIFO order.
    pub fn drain_into_game(&mut self, game: &mut GameWorld) -> usize {
        let drained = self.queue.len();
        while let Some(m) = self.queue.pop_front() {
            game.apply_editor_mutation(&m);
        }
        drained
    }

    /// Returns queued mutations without applying them (used in benchmarks/tests).
    #[must_use]
    pub fn queued_len(&self) -> usize {
        self.queue.len()
    }
}
