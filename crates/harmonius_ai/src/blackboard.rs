//! Typed blackboard storage with self and group scopes.

use std::collections::BTreeMap;

/// Dense key id for blackboard variables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlackboardKey(pub u32);

/// Supported blackboard payload types.
#[derive(Clone, Debug, PartialEq)]
pub enum BlackboardValue {
    /// Boolean fact.
    Bool(bool),
    /// 32-bit signed integer.
    Int(i32),
    /// Single-precision float.
    Float(f32),
}

/// Declared type for schema validation (subset used by tests).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlackboardValueType {
    /// Boolean column.
    Bool,
    /// Integer column.
    Int,
    /// Float column.
    Float,
}

/// Identifier for a shared squad / group blackboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroupId(pub u32);

/// Per-agent self-scoped key/value store.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScopedBlackboard {
    vars: BTreeMap<u32, BlackboardValue>,
}

impl ScopedBlackboard {
    /// Empty self-scope store.
    pub fn new() -> Self {
        Self {
            vars: BTreeMap::new(),
        }
    }

    /// Read a self-scoped value.
    pub fn get(&self, key: BlackboardKey) -> Option<&BlackboardValue> {
        self.vars.get(&key.0)
    }

    /// Insert or replace a self-scoped value.
    pub fn set(&mut self, key: BlackboardKey, value: BlackboardValue) {
        self.vars.insert(key.0, value);
    }

    /// Removes a key if present.
    pub fn remove(&mut self, key: BlackboardKey) {
        self.vars.remove(&key.0);
    }
}

/// Group-scoped blackboards keyed by [`GroupId`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GroupBlackboardStore {
    groups: BTreeMap<u32, BTreeMap<u32, BlackboardValue>>,
}

impl GroupBlackboardStore {
    /// Empty group store.
    pub fn new() -> Self {
        Self {
            groups: BTreeMap::new(),
        }
    }

    /// Read from a group's map.
    pub fn get(&self, group: GroupId, key: BlackboardKey) -> Option<&BlackboardValue> {
        self.groups.get(&group.0).and_then(|m| m.get(&key.0))
    }

    /// Writes into a group map, creating the group if needed.
    pub fn set(&mut self, group: GroupId, key: BlackboardKey, value: BlackboardValue) {
        self.groups.entry(group.0).or_default().insert(key.0, value);
    }
}

/// Resolves a key for tests: self scope only (no group fallback).
pub fn get_self(agent: &ScopedBlackboard, key: BlackboardKey) -> Option<&BlackboardValue> {
    agent.get(key)
}

/// Resolves with optional group membership.
pub fn resolve_with_group(
    agent: &ScopedBlackboard,
    group: Option<GroupId>,
    groups: &GroupBlackboardStore,
    key: BlackboardKey,
) -> Option<BlackboardValue> {
    if let Some(v) = agent.get(key) {
        return Some(v.clone());
    }
    if let Some(g) = group {
        return groups.get(g, key).cloned();
    }
    None
}

/// Observer helper: counts notifications when a stored value actually changes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObserverRegistry {
    fire_count: u32,
}

impl ObserverRegistry {
    /// Zero notifications.
    pub fn new() -> Self {
        Self { fire_count: 0 }
    }

    /// Number of observer callbacks invoked.
    pub fn fire_count(&self) -> u32 {
        self.fire_count
    }

    /// Sets `key` on `board`, incrementing [`Self::fire_count`] when the stored value changes
    /// (including absent → present).
    pub fn set_and_notify(
        &mut self,
        board: &mut ScopedBlackboard,
        key: BlackboardKey,
        value: BlackboardValue,
    ) {
        let changed = match board.get(key) {
            None => true,
            Some(prev) => prev != &value,
        };
        if changed {
            self.fire_count += 1;
        }
        board.set(key, value);
    }
}
