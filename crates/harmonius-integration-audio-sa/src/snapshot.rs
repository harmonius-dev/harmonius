//! Audio-thread snapshot built by draining the propagation MPSC receiver.

use std::collections::HashMap;

use crossbeam_channel::Receiver;

use crate::{Entity, PropagationResult};

/// Snapshot of propagation results keyed by source entity.
#[derive(Debug, Clone, PartialEq)]
pub struct PropagationSnapshot {
    map: HashMap<Entity, PropagationResult>,
}

impl PropagationSnapshot {
    /// Creates an empty snapshot.
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Drains every pending message from `rx`, inserting valid entities and skipping orphans.
    pub fn drain_from(&mut self, rx: &Receiver<PropagationResult>, alive: impl Fn(Entity) -> bool) {
        while let Ok(result) = rx.try_recv() {
            if alive(result.source) {
                self.map.insert(result.source, result);
            }
        }
    }

    /// Fetches a stored result.
    #[must_use]
    pub fn get(&self, source: Entity) -> Option<&PropagationResult> {
        self.map.get(&source)
    }

    /// Number of stored propagation results.
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` when no propagation results are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Default for PropagationSnapshot {
    fn default() -> Self {
        Self::new()
    }
}
