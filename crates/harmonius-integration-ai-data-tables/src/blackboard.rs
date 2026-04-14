//! Minimal blackboard used by the integration systems.

use std::collections::BTreeMap;

use rkyv_derive::{Archive, Deserialize, Serialize};

/// Opaque blackboard key (`u32` namespace).
#[derive(
    Archive, Serialize, Deserialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[repr(transparent)]
pub struct BlackboardKey(pub u32);

/// Value stored in a blackboard.
#[derive(Clone, Debug, PartialEq)]
pub enum BlackboardValue {
    /// Floating scalar.
    Float(f32),
    /// Signed integer.
    Int(i32),
    /// Boolean.
    Bool(bool),
    /// UTF-8 string.
    String(String),
    /// Sentinel / missing binding.
    None,
}

/// Sorted-map blackboard (deterministic iteration, no hash randomization).
#[derive(Clone, Debug, Default)]
pub struct Blackboard {
    entries: BTreeMap<BlackboardKey, BlackboardValue>,
}

impl Blackboard {
    /// Read a key.
    pub fn get(&self, key: BlackboardKey) -> BlackboardValue {
        self.entries
            .get(&key)
            .cloned()
            .unwrap_or(BlackboardValue::None)
    }

    /// Write a key.
    pub fn set(&mut self, key: BlackboardKey, value: BlackboardValue) {
        self.entries.insert(key, value);
    }
}
