//! Shared identifiers and the modifier aggregation pipeline.
use smallvec::SmallVec;
use std::marker::PhantomData;

/// Placeholder ECS world reference for condition evaluation.
#[derive(Debug, Default)]
pub struct World;

/// Stable entity identifier used for modifier provenance.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Entity(pub u64);

/// Asset identifier for effect definitions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AssetId(pub u32);

/// Opaque gameplay tag handle.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GameplayTag(pub u32);

/// Minimal gameplay tag container carried on definitions.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GameplayTagSet;

/// Row handle placeholder for data table integration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RowRef;

/// Typed asset handle placeholder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Handle<T> {
    pub id: u32,
    marker: PhantomData<T>,
}

impl<T> Handle<T> {
    /// Constructs a typed asset handle.
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }
}

/// Modifier operation applied in deterministic phases.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModOp {
    /// Adds a flat offset before percentage phases.
    Flat,
    /// Addsitive percentage bucket: `base * (1 + sum)`.
    PercentAdd,
    /// Multiplicative percentage factors: multiply by `Π(1 + value)`.
    PercentMul,
    /// Overrides the post-percent value; highest `priority` wins.
    Override,
}

/// Single modifier entry with explicit sort key.
#[derive(Clone, Debug, PartialEq)]
pub struct StatModifier {
    /// Modifier operation.
    pub op: ModOp,
    /// Operation magnitude (meaning depends on `op`).
    pub value: f32,
    /// Source entity for bulk removal.
    pub source: Entity,
    /// Sort key within an operation group (larger wins for overrides).
    pub priority: i32,
}

/// Small inline modifier stack with deterministic evaluation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatAggregator {
    modifiers: SmallVec<[StatModifier; 8]>,
}

impl StatAggregator {
    /// Creates an empty stack.
    pub fn new() -> Self {
        Self {
            modifiers: SmallVec::new(),
        }
    }

    /// Adds a modifier to the stack.
    pub fn add(&mut self, modifier: StatModifier) {
        self.modifiers.push(modifier);
    }

    /// Removes modifiers attributed to `source`.
    pub fn remove_by_source(&mut self, source: Entity) {
        self.modifiers.retain(|modifier| modifier.source != source);
    }

    /// Clears all modifiers.
    pub fn clear(&mut self) {
        self.modifiers.clear();
    }

    /// Returns the number of modifiers currently stored.
    pub fn len(&self) -> usize {
        self.modifiers.len()
    }

    /// Returns whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    /// Evaluates the layered pipeline: flat → additive percent → multiplicative percent → override.
    #[must_use]
    pub fn evaluate(&self, base: f32, min_value: f32, max_value: f32) -> f32 {
        self.evaluate_inner(base, min_value, max_value, None)
    }

    /// Like [`Self::evaluate`], but increments `audit` when work is performed.
    #[must_use]
    pub fn evaluate_tracked(
        &self,
        base: f32,
        min_value: f32,
        max_value: f32,
        audit: Option<&mut u32>,
    ) -> f32 {
        self.evaluate_inner(base, min_value, max_value, audit)
    }

    fn evaluate_inner(
        &self,
        base: f32,
        min_value: f32,
        max_value: f32,
        audit: Option<&mut u32>,
    ) -> f32 {
        if let Some(counter) = audit {
            *counter = (*counter).saturating_add(1);
        }

        let mut flat_total = 0.0f32;
        let mut pct_add_total = 0.0f32;
        let mut pct_mul_product = 1.0f32;
        let mut overrides: SmallVec<[(i32, f32, usize); 4]> = SmallVec::new();

        let mut flat_entries: SmallVec<[(i32, usize, f32); 8]> = SmallVec::new();
        let mut pct_add_entries: SmallVec<[(i32, usize, f32); 8]> = SmallVec::new();
        let mut pct_mul_entries: SmallVec<[(i32, usize, f32); 8]> = SmallVec::new();

        for (index, modifier) in self.modifiers.iter().enumerate() {
            match modifier.op {
                ModOp::Flat => {
                    flat_entries.push((modifier.priority, index, modifier.value));
                }
                ModOp::PercentAdd => {
                    pct_add_entries.push((modifier.priority, index, modifier.value));
                }
                ModOp::PercentMul => {
                    pct_mul_entries.push((modifier.priority, index, modifier.value));
                }
                ModOp::Override => {
                    overrides.push((modifier.priority, modifier.value, index));
                }
            }
        }

        flat_entries.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
        for (_, _, value) in flat_entries {
            flat_total += value;
        }

        pct_add_entries
            .sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
        for (_, _, value) in pct_add_entries {
            pct_add_total += value;
        }

        pct_mul_entries
            .sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
        for (_, _, value) in pct_mul_entries {
            pct_mul_product *= 1.0 + value;
        }

        let after_flat = base + flat_total;
        let after_pct_add = after_flat * (1.0 + pct_add_total);
        let mut value = after_pct_add * pct_mul_product;

        overrides.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.2.cmp(&right.2)));
        if let Some((_, override_value, _)) = overrides.last() {
            value = *override_value;
        }

        clamp_f32(value, min_value, max_value)
    }
}

fn clamp_f32(value: f32, min_value: f32, max_value: f32) -> f32 {
    value.clamp(min_value, max_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_flat_additive() {
        let mut stack = StatAggregator::new();
        stack.add(StatModifier {
            op: ModOp::Flat,
            value: 5.0,
            source: Entity(1),
            priority: 0,
        });
        assert_eq!(stack.evaluate(10.0, f32::NEG_INFINITY, f32::INFINITY), 15.0);
    }

    #[test]
    fn test_modifier_percent_additive() {
        let mut stack = StatAggregator::new();
        stack.add(StatModifier {
            op: ModOp::PercentAdd,
            value: 0.5,
            source: Entity(1),
            priority: 0,
        });
        stack.add(StatModifier {
            op: ModOp::PercentAdd,
            value: 0.25,
            source: Entity(1),
            priority: 0,
        });
        assert_eq!(stack.evaluate(10.0, f32::NEG_INFINITY, f32::INFINITY), 17.5);
    }

    #[test]
    fn test_modifier_percent_mul() {
        let mut stack = StatAggregator::new();
        stack.add(StatModifier {
            op: ModOp::PercentMul,
            value: 0.2,
            source: Entity(1),
            priority: 0,
        });
        stack.add(StatModifier {
            op: ModOp::PercentMul,
            value: 0.1,
            source: Entity(1),
            priority: 0,
        });
        let result = stack.evaluate(10.0, f32::NEG_INFINITY, f32::INFINITY);
        assert!((result - 13.2).abs() < f32::EPSILON * 16.0);
    }

    #[test]
    fn test_modifier_override_priority() {
        let mut stack = StatAggregator::new();
        stack.add(StatModifier {
            op: ModOp::Override,
            value: 50.0,
            source: Entity(1),
            priority: 1,
        });
        stack.add(StatModifier {
            op: ModOp::Override,
            value: 99.0,
            source: Entity(2),
            priority: 2,
        });
        assert_eq!(stack.evaluate(0.0, f32::NEG_INFINITY, f32::INFINITY), 99.0);
    }

    #[test]
    fn test_modifier_full_pipeline() {
        let mut stack = StatAggregator::new();
        stack.add(StatModifier {
            op: ModOp::Flat,
            value: 10.0,
            source: Entity(1),
            priority: 0,
        });
        stack.add(StatModifier {
            op: ModOp::PercentAdd,
            value: 0.2,
            source: Entity(1),
            priority: 0,
        });
        stack.add(StatModifier {
            op: ModOp::PercentMul,
            value: 0.1,
            source: Entity(1),
            priority: 0,
        });
        let result = stack.evaluate(100.0, f32::NEG_INFINITY, f32::INFINITY);
        assert!((result - 145.2).abs() < f32::EPSILON * 16.0);
    }

    #[test]
    fn test_buff_equipment_mod_stack() {
        let mut stack = StatAggregator::new();
        stack.add(StatModifier {
            op: ModOp::Flat,
            value: 5.0,
            source: Entity(1),
            priority: 0,
        });
        stack.add(StatModifier {
            op: ModOp::PercentMul,
            value: 0.1,
            source: Entity(2),
            priority: 0,
        });
        let result = stack.evaluate(10.0, f32::NEG_INFINITY, f32::INFINITY);
        assert!((result - 16.5).abs() < f32::EPSILON * 16.0);
    }
}
