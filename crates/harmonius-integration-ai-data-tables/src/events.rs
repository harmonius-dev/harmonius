//! `TableReloaded` entity events.

use crate::ids::TableId;

/// Emitted when a `DataTable` hot-reloads to a new version.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableReloaded {
    /// Affected table.
    pub table: TableId,
    /// Previous version.
    pub old_version: u64,
    /// New version after swap.
    pub new_version: u64,
}

/// Minimal FIFO queue mirroring the ECS entity-event buffer.
#[derive(Clone, Debug)]
pub struct EntityEventQueue<E> {
    pending: Vec<E>,
}

impl<E> Default for EntityEventQueue<E> {
    fn default() -> Self {
        Self {
            pending: Vec::new(),
        }
    }
}

impl<E> EntityEventQueue<E> {
    /// Enqueues an event at `FrameEnd`.
    pub fn push(&mut self, event: E) {
        self.pending.push(event);
    }

    /// Drains all pending events (Phase 1 PreUpdate).
    pub fn drain(&mut self) -> Vec<E> {
        std::mem::take(&mut self.pending)
    }
}
