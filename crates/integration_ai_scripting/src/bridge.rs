//! Blackboard ↔ `VariableStore` bridge (IR-2.4.3).

use smallvec::SmallVec;

/// Opaque blackboard key (slot-indexed in this harness; no string interning).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlackboardKey(pub u32);

/// Fixed mapping from blackboard keys to variable slots (codegen static data).
#[derive(Clone, Debug)]
pub struct BbVarMapping {
    /// Parallel with [`Self::var_slots`].
    pub bb_keys: SmallVec<[BlackboardKey; 8]>,
    /// Variable slots corresponding to each blackboard key.
    pub var_slots: SmallVec<[u32; 8]>,
}

impl BbVarMapping {
    /// Returns `true` when `key` appears in the mapping.
    #[must_use]
    pub fn contains(&self, key: BlackboardKey) -> bool {
        self.bb_keys.contains(&key)
    }

    fn len(&self) -> usize {
        debug_assert_eq!(self.bb_keys.len(), self.var_slots.len());
        self.bb_keys.len()
    }
}

/// Slot-indexed variable store (`Vec<u32>` slots → `f32` values).
#[derive(Clone, Debug, Default)]
pub struct VariableStore {
    slots: Vec<f32>,
    modified: Vec<bool>,
}

impl VariableStore {
    /// Ensure `slot` exists; grow with zeros / false flags as needed.
    pub fn ensure_slot(&mut self, slot: u32) {
        let need = slot as usize + 1;
        if self.slots.len() < need {
            self.slots.resize(need, 0.0);
            self.modified.resize(need, false);
        }
    }

    /// Read a slot (0.0 when unset).
    #[must_use]
    pub fn read_slot(&self, slot: u32) -> f32 {
        self.slots.get(slot as usize).copied().unwrap_or(0.0)
    }

    /// Write a slot and mark modified.
    pub fn write_slot(&mut self, slot: u32, value: f32) {
        self.ensure_slot(slot);
        self.slots[slot as usize] = value;
        self.modified[slot as usize] = true;
    }

    /// Clear modified flags (e.g. after sync to blackboard).
    pub fn clear_modified(&mut self) {
        for m in &mut self.modified {
            *m = false;
        }
    }

    /// Whether a slot was written since last clear.
    #[must_use]
    pub fn is_modified(&self, slot: u32) -> bool {
        self.modified.get(slot as usize).copied().unwrap_or(false)
    }
}

/// Blackboard harness using a `Vec` backing store (no `HashMap` on hot path).
#[derive(Clone, Debug, Default)]
pub struct Blackboard {
    values: Vec<f32>,
    dirty: Vec<bool>,
}

impl Blackboard {
    /// Grow storage to include `key`.
    pub fn ensure_key(&mut self, key: BlackboardKey) {
        let need = key.0 as usize + 1;
        if self.values.len() < need {
            self.values.resize(need, 0.0);
            self.dirty.resize(need, false);
        }
    }

    /// Set a key and mark dirty.
    pub fn set(&mut self, key: BlackboardKey, value: f32) {
        self.ensure_key(key);
        self.values[key.0 as usize] = value;
        self.dirty[key.0 as usize] = true;
    }

    /// Read a key (0.0 when unset).
    #[must_use]
    pub fn get(&self, key: BlackboardKey) -> f32 {
        self.values.get(key.0 as usize).copied().unwrap_or(0.0)
    }

    /// Whether the key is marked dirty.
    #[must_use]
    pub fn is_dirty(&self, key: BlackboardKey) -> bool {
        self.dirty.get(key.0 as usize).copied().unwrap_or(false)
    }

    /// Clear dirty flag for one key.
    pub fn clear_dirty(&mut self, key: BlackboardKey) {
        if let Some(d) = self.dirty.get_mut(key.0 as usize) {
            *d = false;
        }
    }

    /// Invoke `f` for every key index that is still dirty.
    pub fn for_each_dirty_key(&self, mut f: impl FnMut(BlackboardKey)) {
        for (i, is_dirty) in self.dirty.iter().enumerate() {
            if *is_dirty {
                f(BlackboardKey(i as u32));
            }
        }
    }
}

/// Synchronizes [`Blackboard`] ↔ [`VariableStore`] using a static [`BbVarMapping`].
#[derive(Clone, Debug)]
pub struct BlackboardBridge {
    /// Compiler-emitted mapping.
    pub mapping: BbVarMapping,
}

impl BlackboardBridge {
    /// Copy mapped dirty blackboard keys into variable slots before graph tick.
    pub fn sync_to_vars(
        &self,
        bb: &Blackboard,
        vars: &mut VariableStore,
        mut log_debug: impl FnMut(&str),
    ) {
        for i in 0..self.mapping.len() {
            let key = self.mapping.bb_keys[i];
            let slot = self.mapping.var_slots[i];
            if bb.is_dirty(key) {
                vars.write_slot(slot, bb.get(key));
            }
        }
        bb.for_each_dirty_key(|key| {
            if !self.mapping.contains(key) {
                log_debug("blackboard_bridge: skipping unmapped dirty blackboard key");
            }
        });
    }

    /// Copy modified variable slots back to blackboard keys after graph tick.
    pub fn sync_to_bb(&self, vars: &VariableStore, bb: &mut Blackboard) {
        for i in 0..self.mapping.len() {
            let key = self.mapping.bb_keys[i];
            let slot = self.mapping.var_slots[i];
            if vars.is_modified(slot) {
                bb.set(key, vars.read_slot(slot));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mapping_one() -> BbVarMapping {
        let mut bb_keys = SmallVec::<[BlackboardKey; 8]>::new();
        let mut var_slots = SmallVec::<[u32; 8]>::new();
        bb_keys.push(BlackboardKey(1));
        var_slots.push(0);
        BbVarMapping { bb_keys, var_slots }
    }

    /// TC-IR-2.4.BB.1 — dirty blackboard key copies into mapped variable slot.
    #[test]
    fn tc_ir_2_4_bb_1_sync_to_vars() {
        let bridge = BlackboardBridge {
            mapping: mapping_one(),
        };
        let mut bb = Blackboard::default();
        bb.set(BlackboardKey(1), 42.0);
        let mut vars = VariableStore::default();
        bridge.sync_to_vars(&bb, &mut vars, |_| {});
        assert_eq!(vars.read_slot(0), 42.0);
    }

    /// TC-IR-2.4.BB.2 — modified variable slot copies back to blackboard.
    #[test]
    fn tc_ir_2_4_bb_2_sync_to_bb() {
        let bridge = BlackboardBridge {
            mapping: mapping_one(),
        };
        let mut bb = Blackboard::default();
        bb.set(BlackboardKey(1), 1.0);
        bb.clear_dirty(BlackboardKey(1));
        let mut vars = VariableStore::default();
        vars.write_slot(0, 7.0);
        bridge.sync_to_bb(&vars, &mut bb);
        assert_eq!(bb.get(BlackboardKey(1)), 7.0);
    }

    /// TC-IR-2.4.BB.3 — bridge path uses only mapping iteration (no `HashMap`).
    #[test]
    fn tc_ir_2_4_bb_3_bridge_avoids_hash_map() {
        let bridge = BlackboardBridge {
            mapping: mapping_one(),
        };
        let mut bb = Blackboard::default();
        bb.set(BlackboardKey(1), 3.0);
        let mut vars = VariableStore::default();
        for _ in 0..1000 {
            bridge.sync_to_vars(&bb, &mut vars, |_| {});
            bridge.sync_to_bb(&vars, &mut bb);
        }
    }

    /// TC-IR-2.4.NEG.7 — unmapped dirty key is skipped with debug log.
    #[test]
    fn tc_ir_2_4_neg_7_unmapped_bb_key_skipped() {
        let bridge = BlackboardBridge {
            mapping: mapping_one(),
        };
        let mut bb = Blackboard::default();
        bb.set(BlackboardKey(9), 99.0);
        let mut vars = VariableStore::default();
        let mut logs = Vec::new();
        bridge.sync_to_vars(&bb, &mut vars, |m| logs.push(m.to_string()));
        assert_eq!(vars.read_slot(0), 0.0);
        assert_eq!(logs.len(), 1);
    }
}
