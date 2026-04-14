//! `ActiveEffects` stack with deterministic eviction (`TC-IR-2.6.6.N1`).

use smallvec::SmallVec;

/// One running effect instance tracked for integration tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveEffectEntry {
    /// Higher means more important when evicting the lowest priority slot.
    pub priority: i32,
    /// Resolved effect definition id.
    pub effect_id: u32,
}

/// Fixed-capacity stack (`SmallVec<[T;16]>`) mirroring the attributes-effects design.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ActiveEffects {
    effects: SmallVec<[ActiveEffectEntry; 16]>,
}

impl ActiveEffects {
    /// Empty stack.
    #[must_use]
    pub fn new() -> Self {
        Self {
            effects: SmallVec::new(),
        }
    }

    /// Applies an effect, evicting the lowest priority entry when full (`TC-IR-2.6.6.N1`).
    pub fn apply(&mut self, priority: i32, effect_id: u32) -> Option<ActiveEffectEntry> {
        let entry = ActiveEffectEntry {
            priority,
            effect_id,
        };
        if self.effects.len() < 16 {
            self.effects.push(entry);
            return None;
        }
        let mut min_idx = 0usize;
        let mut min_pri = self.effects[0].priority;
        for (i, e) in self.effects.iter().enumerate().skip(1) {
            if e.priority < min_pri {
                min_pri = e.priority;
                min_idx = i;
            }
        }
        let evicted = self.effects[min_idx].clone();
        self.effects[min_idx] = entry;
        Some(evicted)
    }

    /// Returns whether an effect id is present.
    #[must_use]
    pub fn contains_effect(&self, effect_id: u32) -> bool {
        self.effects.iter().any(|e| e.effect_id == effect_id)
    }

    /// Removes every entry with `effect_id`, returning count removed.
    pub fn remove_effect(&mut self, effect_id: u32) -> usize {
        let before = self.effects.len();
        self.effects.retain(|e| e.effect_id != effect_id);
        before - self.effects.len()
    }

    /// Current stack depth.
    #[must_use]
    pub fn len(&self) -> usize {
        self.effects.len()
    }

    /// `true` when no effects are active.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    /// Iterator over entries (sorted by insertion order).
    pub fn entries(&self) -> impl Iterator<Item = &ActiveEffectEntry> {
        self.effects.iter()
    }
}
