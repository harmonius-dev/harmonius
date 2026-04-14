//! `GameStateSnapshot` and a tiny triple-buffer handoff for editor reads.

use crate::world::{EntityId, GameTime, WorldState};

/// Read-only runtime view delivered to the editor after the snapshot phase.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameStateSnapshot {
    /// Monotonic frame index; tests align this with `GameTime.tick`.
    pub frame_index: u64,
    /// Entity ids present in the snapshot (minimal stand-in for `EntitySnapshot` list).
    pub entities: Vec<EntityId>,
    /// Placeholder render payload for the design's `RenderFrame` slot.
    pub render_token: u32,
}

impl GameStateSnapshot {
    /// Builds a snapshot from the current runtime world and tick counter.
    #[must_use]
    pub fn from_world(time: &GameTime, world: &WorldState) -> Self {
        Self {
            frame_index: time.tick,
            entities: world.entities.iter().copied().collect(),
            render_token: 0,
        }
    }
}

/// Triple-buffered publisher/reader pair for `FM-3` stale-read tests.
#[derive(Debug, Default)]
pub struct TripleSnapshotBuffer {
    slots: [Option<GameStateSnapshot>; 3],
    write_index: usize,
    read_index: usize,
    /// Increments when a reader had to reuse the previous snapshot (`FM-3`).
    pub fm3_stale_read_events: u64,
    /// When true, publish is deferred to simulate a blocked game thread.
    pub block_next_publish: bool,
}

impl TripleSnapshotBuffer {
    /// Creates an empty buffer.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Publishes a snapshot from the game thread unless blocked for `FM-3` simulation.
    pub fn publish(&mut self, snapshot: GameStateSnapshot) {
        if self.block_next_publish {
            self.block_next_publish = false;
            return;
        }
        self.slots[self.write_index] = Some(snapshot);
        self.read_index = self.write_index;
        self.write_index = (self.write_index + 1) % 3;
    }

    /// Reads the latest published snapshot without mutating counters.
    #[must_use]
    pub fn read_latest(&self) -> Option<GameStateSnapshot> {
        self.slots[self.read_index].clone()
    }

    /// Reads the latest snapshot, counting `FM-3` when nothing was published yet.
    pub fn read_latest_or(&mut self, fallback: GameStateSnapshot) -> GameStateSnapshot {
        match self.read_latest() {
            Some(s) => s,
            None => {
                self.fm3_stale_read_events = self.fm3_stale_read_events.saturating_add(1);
                fallback
            }
        }
    }

    /// Reads the latest snapshot and increments `FM-3` when `frame_index` differs from
    /// `expected_tick` (blocked publish or first-frame lag).
    pub fn read_expecting_tick(&mut self, expected_tick: u64) -> GameStateSnapshot {
        let cur = self.read_latest().unwrap_or(GameStateSnapshot {
            frame_index: 0,
            entities: Vec::new(),
            render_token: 0,
        });
        if cur.frame_index != expected_tick {
            self.fm3_stale_read_events = self.fm3_stale_read_events.saturating_add(1);
        }
        cur
    }
}
