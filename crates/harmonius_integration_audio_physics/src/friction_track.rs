//! Active friction voices keyed by entity pair.

use crate::ids::{Entity, VoiceId};

/// One active friction slot.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FrictionSlot {
    /// First body in the pair.
    pub entity_a: Entity,
    /// Second body in the pair.
    pub entity_b: Entity,
    /// Playing voice id.
    pub voice: VoiceId,
}

/// Fixed-size map from collision pair to friction voice (design: 128 entries).
#[derive(Clone, Debug, PartialEq)]
pub struct ActiveFrictionSounds {
    /// Pair entries.
    pub entries: [FrictionSlot; 128],
    /// Occupied entry count.
    pub len: usize,
}

impl Default for ActiveFrictionSounds {
    fn default() -> Self {
        Self::new()
    }
}

impl ActiveFrictionSounds {
    /// Constructs an empty tracker.
    pub fn new() -> Self {
        Self {
            entries: [FrictionSlot {
                entity_a: Entity(0),
                entity_b: Entity(0),
                voice: VoiceId(0),
            }; 128],
            len: 0,
        }
    }

    fn norm(a: Entity, b: Entity) -> (Entity, Entity) {
        if a <= b {
            (a, b)
        } else {
            (b, a)
        }
    }

    /// Returns the active voice for a pair when present.
    pub fn get(&self, a: Entity, b: Entity) -> Option<VoiceId> {
        let (ea, eb) = Self::norm(a, b);
        for i in 0..self.len {
            let s = self.entries[i];
            if s.entity_a == ea && s.entity_b == eb {
                return Some(s.voice);
            }
        }
        None
    }

    /// Inserts a new friction voice, replacing on duplicate pair.
    ///
    /// When full, evicts the oldest entry (index `0`, FIFO) then appends the new pair.
    pub fn insert(&mut self, a: Entity, b: Entity, voice: VoiceId) {
        let (ea, eb) = Self::norm(a, b);
        if let Some(i) = (0..self.len)
            .find(|&i| self.entries[i].entity_a == ea && self.entries[i].entity_b == eb)
        {
            self.entries[i].voice = voice;
            return;
        }
        if self.len >= 128 {
            for i in 1..self.len {
                self.entries[i - 1] = self.entries[i];
            }
            self.len -= 1;
        }
        self.entries[self.len] = FrictionSlot {
            entity_a: ea,
            entity_b: eb,
            voice,
        };
        self.len += 1;
    }

    /// Removes and returns the voice for a pair when present.
    pub fn remove(&mut self, a: Entity, b: Entity) -> Option<VoiceId> {
        let (ea, eb) = Self::norm(a, b);
        if let Some(pos) = (0..self.len)
            .position(|i| self.entries[i].entity_a == ea && self.entries[i].entity_b == eb)
        {
            let voice = self.entries[pos].voice;
            self.len -= 1;
            if pos < self.len {
                self.entries.swap(pos, self.len);
            }
            return Some(voice);
        }
        None
    }
}
