//! Deferred blackboard writes from gossip propagation (IR-2.2.5).

use std::collections::BTreeMap;

use crate::blackboard::{Blackboard, BlackboardKey, BlackboardValue};
use crate::ids::Entity;

/// Buffers `(entity, key, value)` triples and applies them deterministically.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropagationBuffer {
    pending: Vec<(Entity, BlackboardKey, BlackboardValue)>,
}

impl PropagationBuffer {
    /// Queues a blackboard update.
    pub fn push(&mut self, target: Entity, key: BlackboardKey, value: BlackboardValue) {
        self.pending.push((target, key, value));
    }

    /// Flushes pending writes with stable sort on `(Entity, BlackboardKey)`; last writer wins.
    pub fn flush(&mut self, blackboards: &mut BTreeMap<Entity, Blackboard>) {
        self.pending
            .sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        let mut merged: BTreeMap<(Entity, BlackboardKey), BlackboardValue> = BTreeMap::new();
        for (e, k, v) in self.pending.drain(..) {
            merged.insert((e, k), v);
        }
        for ((e, k), v) in merged {
            blackboards.entry(e).or_default().set(k, v);
        }
    }
}
