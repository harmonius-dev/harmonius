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

impl CooldownSlot {
    const EMPTY: Self = Self {
        entity_a: Entity(0),
        entity_b: Entity(0),
        remaining_sec: 0.0,
    };
}

/// Fixed 256-slot cooldown tracker with FIFO eviction (integration design).
#[derive(Clone, Debug, PartialEq)]
pub struct ImpactCooldownTracker {
    entries: [CooldownSlot; 256],
    len: usize,
}

impl Default for ImpactCooldownTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ImpactCooldownTracker {
    /// Constructs an empty tracker.
    pub fn new() -> Self {
        Self {
            entries: [CooldownSlot::EMPTY; 256],
            len: 0,
        }
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
        self.entries[..self.len]
            .iter()
            .any(|s| s.entity_a == ea && s.entity_b == eb && s.remaining_sec > 0.0)
    }

    /// Starts or refreshes cooldown for a pair, evicting FIFO when at capacity.
    pub fn start(&mut self, a: Entity, b: Entity, duration: f32) {
        let (ea, eb) = Self::normalize_pair(a, b);
        for i in 0..self.len {
            let slot = &mut self.entries[i];
            if slot.entity_a == ea && slot.entity_b == eb {
                slot.remaining_sec = duration;
                return;
            }
        }
        if self.len == 256 {
            for i in 1..256 {
                self.entries[i - 1] = self.entries[i];
            }
            self.len = 255;
        }
        self.entries[self.len] = CooldownSlot {
            entity_a: ea,
            entity_b: eb,
            remaining_sec: duration,
        };
        self.len += 1;
    }

    /// Advances cooldown timers and compacts expired slots.
    pub fn tick(&mut self, dt: f32) {
        let mut w = 0usize;
        for r in 0..self.len {
            let mut slot = self.entries[r];
            slot.remaining_sec -= dt;
            if slot.remaining_sec > 0.0 {
                self.entries[w] = slot;
                w += 1;
            }
        }
        self.len = w;
    }

    /// Returns the number of live cooldown slots (for diagnostics tests).
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` when no cooldown slots are active.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
