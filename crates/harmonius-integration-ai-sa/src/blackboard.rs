//! Blackboard keys and backing store (`BTreeMap`) per integration design IR-1.10.3.

use std::collections::BTreeMap;

/// ECS blackboard key for serialized awareness level (`i32` ordinal).
pub const AWARENESS_LEVEL_KEY: &str = "awareness_level";
/// Current highest-threat target entity.
pub const THREAT_TARGET_KEY: &str = "threat_target";
/// Last known world position of the threat target.
pub const THREAT_POSITION_KEY: &str = "threat_position";

/// Typed values stored in a blackboard.
#[derive(Clone, Debug, PartialEq)]
pub enum BlackboardValue {
    /// 32-bit signed integer slot.
    Int(i32),
    /// Entity identifier slot.
    Entity(crate::types::Entity),
    /// 3D vector slot.
    Vec3(crate::types::Vec3),
}

/// Sorted-key blackboard (hot-path friendly per design).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Blackboard {
    entries: BTreeMap<&'static str, BlackboardValue>,
}

impl Blackboard {
    /// Returns an empty blackboard.
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Inserts or replaces `key`.
    pub fn set(&mut self, key: &'static str, value: BlackboardValue) {
        self.entries.insert(key, value);
    }

    /// Removes `key` if present.
    pub fn remove(&mut self, key: &'static str) {
        self.entries.remove(key);
    }

    /// Borrows a value for `key`.
    pub fn get(&self, key: &'static str) -> Option<&BlackboardValue> {
        self.entries.get(key)
    }
}
