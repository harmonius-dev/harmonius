//! Per-entity blackboard with deterministic `BTreeMap` storage (IR-2.2.5, SC hot path).

use std::collections::BTreeMap;

/// Typed key into a blackboard scope.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlackboardKey(pub u32);

/// Minimal value model sufficient for integration tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlackboardValue {
    /// Integer counter (e.g., threat count).
    I32(i32),
    /// Normalized scalar (e.g., utility score).
    F32(f32),
}

/// Sorted key/value store — `HashMap` is intentionally not used here (TC-IR-2.2.N3).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BlackboardScope {
    /// Deterministic iteration order.
    pub entries: BTreeMap<BlackboardKey, BlackboardValue>,
}

impl BlackboardScope {
    /// Returns an empty scope.
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Reads a key.
    pub fn get(&self, key: BlackboardKey) -> Option<BlackboardValue> {
        self.entries.get(&key).copied()
    }

    /// Inserts or overwrites a key.
    pub fn set(&mut self, key: BlackboardKey, value: BlackboardValue) {
        self.entries.insert(key, value);
    }
}

/// Agent-local blackboard container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Blackboard {
    /// Owned scope (no `Arc` — mutable hot path).
    pub scope: BlackboardScope,
}

impl Blackboard {
    /// Returns an empty blackboard.
    pub fn new() -> Self {
        Self {
            scope: BlackboardScope::new(),
        }
    }

    /// Reads through the scope.
    pub fn get(&self, key: BlackboardKey) -> Option<BlackboardValue> {
        self.scope.get(key)
    }

    /// Writes through the scope.
    pub fn set(&mut self, key: BlackboardKey, value: BlackboardValue) {
        self.scope.set(key, value);
    }
}
