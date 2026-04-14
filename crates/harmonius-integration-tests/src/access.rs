//! Access sets and graph access metadata (`GraphAccessDescriptor`).

use rkyv_derive::{Archive, Deserialize, Serialize};

/// Bitset of ECS component kinds (integration slice uses up to 128 slots).
#[derive(
    Archive, Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize,
)]
pub struct AccessSet {
    bits: u128,
}

impl AccessSet {
    /// Empty set.
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Single slot index `0..127`.
    pub fn singleton(slot: u8) -> Self {
        debug_assert!(slot < 128);
        Self {
            bits: 1u128 << slot as u128,
        }
    }

    /// Raw bits (for tests).
    pub const fn from_bits(bits: u128) -> Self {
        Self { bits }
    }

    /// Union of two sets.
    pub fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    /// Whether any slot overlaps.
    pub fn intersects(self, other: Self) -> bool {
        (self.bits & other.bits) != 0
    }

    /// In-place insert slot.
    pub fn insert(&mut self, slot: u8) {
        debug_assert!(slot < 128);
        self.bits |= 1u128 << slot as u128;
    }
}

/// Compiler-emitted access metadata for a `GraphProgram`.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GraphAccessDescriptor {
    /// Components read by this graph program.
    pub reads: AccessSet,
    /// Components written by this graph program.
    pub writes: AccessSet,
    /// Whether the graph uses command buffers.
    pub has_commands: bool,
}

impl GraphAccessDescriptor {
    /// New descriptor.
    pub fn new(reads: AccessSet, writes: AccessSet, has_commands: bool) -> Self {
        Self {
            has_commands,
            reads,
            writes,
        }
    }
}
