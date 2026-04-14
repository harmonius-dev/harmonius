//! Per-entity-pair impact cooldown ring.

use crate::ids::Entity;

/// One cooldown slot in [`ImpactCooldownTracker`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CooldownSlot {
    /// Lower [`Entity`] id in the pair.
    pub entity_a: Entity,
    /// Higher [`Entity`] id in the pair.
    pub entity_b: Entity,
    /// Remaining cooldown time in seconds.
    pub remaining_sec: f32,
}

/// Fixed-capacity cooldown list with FIFO eviction at 256 entries (integration design).
#[derive(Clone, Debug, PartialEq)]
pub struct ImpactCooldownTracker {
    slots: Vec<CooldownSlot>,
}

impl Default for ImpactCooldownTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ImpactCooldownTracker {
    /// Constructs an empty tracker.
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    fn normalize_pair(a: Entity, b: Entity) -> (Entity, Entity) {
        if a <= b {
            (a, b)
        } else {
            (b, a)
        }
    }

    /// Returns `true` when the pair is actively cooling.
    pub fn is_cooling(&self, a: Entity, b: Entity) -> bool {
        let (ea, eb) = Self::normalize_pair(a, b);
        self.slots.iter().any(|s| {
            s.entity_a == ea && s.entity_b == eb && s.remaining_sec > 0.0
        })
    }

    /// Starts or refreshes cooldown for a pair, evicting FIFO when at capacity.
    pub fn start(&mut self, a: Entity, b: Entity, duration: f32) {
        let (ea, eb) = Self::normalize_pair(a, b);
        for slot in &mut self.slots {
            if slot.entity_a == ea && slot.entity_b == eb {
                slot.remaining_sec = duration;
                return;
            }
        }
        if self.slots.len() == 256 {
            self.slots.remove(0);
        }
        self.slots.push(CooldownSlot {
            entity_a: ea,
            entity_b: eb,
            remaining_sec: duration,
        });
    }

    /// Advances cooldown timers and drops expired slots.
    pub fn tick(&mut self, dt: f32) {
        for slot in &mut self.slots {
            slot.remaining_sec -= dt;
        }
        self.slots.retain(|s| s.remaining_sec > 0.0);
    }

    /// Returns the number of live cooldown slots (for diagnostics tests).
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Returns `true` when no cooldown slots are active.
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }
}
