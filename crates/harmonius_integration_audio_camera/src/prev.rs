//! Previous-frame listener positions for finite-difference velocity.

use glam::Vec3;

/// Maximum local players for split-screen; fixed listener slot count.
pub const MAX_LOCAL_PLAYERS: usize = 4;

/// Per-slot prior world positions for backward-difference velocity.
///
/// Indexed by `player_index`. `None` means no prior sample (first frame or rejoin).
#[derive(Debug)]
pub struct ListenerPrevPositions {
    slots: [Option<Vec3>; MAX_LOCAL_PLAYERS],
}

impl ListenerPrevPositions {
    /// Initializes all slots to empty.
    pub fn new() -> Self {
        Self {
            slots: [None; MAX_LOCAL_PLAYERS],
        }
    }

    /// Returns the last stored position for `index`, if any.
    pub fn get(&self, index: u8) -> Option<Vec3> {
        self.slots.get(index as usize).copied().flatten()
    }

    /// Stores `pos` for `index` when `index` is in range; otherwise no-op.
    pub fn set(&mut self, index: u8, pos: Vec3) {
        if let Some(slot) = self.slots.get_mut(index as usize) {
            *slot = Some(pos);
        }
    }

    /// Clears history for `index` when in range (player left / despawn).
    pub fn clear(&mut self, index: u8) {
        if let Some(slot) = self.slots.get_mut(index as usize) {
            *slot = None;
        }
    }
}

impl Default for ListenerPrevPositions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_1_7_1_u1_prev_pos_slot_roundtrip() {
        let mut p = ListenerPrevPositions::new();
        let pos = Vec3::new(1.0, 2.0, 3.0);
        p.set(1, pos);
        assert_eq!(p.get(1), Some(pos));
    }

    #[test]
    fn tc_ir_1_7_1_u2_prev_pos_empty_slot() {
        let p = ListenerPrevPositions::new();
        assert_eq!(p.get(2), None);
    }

    #[test]
    fn tc_ir_1_7_1_u3_prev_pos_clear() {
        let mut p = ListenerPrevPositions::new();
        let pos = Vec3::splat(4.0);
        p.set(0, pos);
        p.clear(0);
        assert_eq!(p.get(0), None);
    }

    #[test]
    fn tc_ir_1_7_4_u1_slot_idx_out_of_range_no_panic() {
        let mut p = ListenerPrevPositions::new();
        p.set(10, Vec3::ONE);
        assert_eq!(p.get(10), None);
    }

    #[test]
    fn tc_ir_1_7_4_n1_idx_above_max_no_crash() {
        let mut p = ListenerPrevPositions::new();
        p.set(9, Vec3::ONE);
        assert_eq!(p.get(9), None);
    }
}
