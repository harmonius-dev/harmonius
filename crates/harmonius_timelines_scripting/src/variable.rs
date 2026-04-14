//! Typed variable slots for scripting (`VariableStore`).

use std::collections::HashMap;

use crate::ids::SlotId;
use crate::timeline::TrackValue;

/// Script-side type tag for slot compatibility checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScriptTypeId {
    F32,
    Bool,
    Vec3,
}

/// Typed slot payload stored in a `VariableStore`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypedSlot {
    F32(f32),
    Bool(bool),
    Vec3([f32; 3]),
}

/// Bind-time validation failure (FM-4).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BindError {
    pub track: crate::ids::TrackId,
    pub slot: SlotId,
    pub expected: ScriptTypeId,
    pub actual: ScriptTypeId,
}

/// Variable storage keyed by `SlotId`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VariableStore {
    slots: HashMap<SlotId, TypedSlot>,
    types: HashMap<SlotId, ScriptTypeId>,
}

impl VariableStore {
    /// Inserts a typed slot with its compile-time type tag.
    pub fn insert_typed(&mut self, slot: SlotId, kind: ScriptTypeId, value: TypedSlot) {
        self.types.insert(slot, kind);
        self.slots.insert(slot, value);
    }

    /// Reads a slot value.
    pub fn get(&self, slot: SlotId) -> Option<TypedSlot> {
        self.slots.get(&slot).copied()
    }

    /// Writes a slot if it exists.
    pub fn set(&mut self, slot: SlotId, value: TypedSlot) -> bool {
        use std::collections::hash_map::Entry;
        match self.slots.entry(slot) {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                true
            }
            Entry::Vacant(_) => false,
        }
    }

    /// Declared type for a slot, if any.
    pub fn slot_type(&self, slot: SlotId) -> Option<ScriptTypeId> {
        self.types.get(&slot).copied()
    }

    /// Lists allocated slot ids (FM-7 diagnostics).
    pub fn slot_ids(&self) -> Vec<SlotId> {
        self.slots.keys().copied().collect()
    }

    /// Drains entries whose slot ids are not in `keep`.
    pub fn retain_slots(&mut self, keep: &[SlotId]) {
        let keep_set: std::collections::HashSet<SlotId> = keep.iter().copied().collect();
        self.slots.retain(|k, _| keep_set.contains(k));
        self.types.retain(|k, _| keep_set.contains(k));
    }
}

impl TrackValue {
    /// Maps a sampled track value to the expected script slot type.
    pub fn expected_script_type(&self) -> ScriptTypeId {
        match self {
            TrackValue::F32(_) => ScriptTypeId::F32,
            TrackValue::Bool(_) | TrackValue::Entity(_) => ScriptTypeId::Bool,
            TrackValue::Vec3(_) => ScriptTypeId::Vec3,
        }
    }

    /// Converts a sampled value to a typed slot (no coercion).
    pub fn to_typed_slot(&self) -> TypedSlot {
        match *self {
            TrackValue::F32(v) => TypedSlot::F32(v),
            TrackValue::Bool(v) => TypedSlot::Bool(v),
            TrackValue::Vec3(v) => TypedSlot::Vec3(v),
            TrackValue::Entity(e) => TypedSlot::Bool(e.0 != 0),
        }
    }
}

/// Validates a track→slot binding before sampling (FM-4).
pub fn validate_binding(
    track: crate::ids::TrackId,
    track_value: &TrackValue,
    store: &VariableStore,
    slot: SlotId,
) -> Result<(), BindError> {
    let track_ty = track_value.expected_script_type();
    let Some(expected) = store.slot_type(slot) else {
        return Err(BindError {
            track,
            slot,
            expected: track_ty,
            actual: track_ty,
        });
    };
    if expected != track_ty {
        return Err(BindError {
            track,
            slot,
            expected,
            actual: track_ty,
        });
    }
    Ok(())
}
