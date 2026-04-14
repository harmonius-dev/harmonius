//! Boundary overlap ghosts (`R-8.7.17`).

use crate::mmo::zone::Transform;

/// Tracks authoritative vs ghost copies across an overlap band (`TC-8.7.17.1`).
#[derive(Debug)]
pub struct OverlapSimulator {
    authority_transform: Transform,
    ghost_transform: Transform,
    dirty: bool,
}

impl OverlapSimulator {
    /// Starts with ghost mirroring authority.
    pub fn new(initial: Transform) -> Self {
        Self {
            authority_transform: initial.clone(),
            ghost_transform: initial,
            dirty: false,
        }
    }

    /// Mutates authoritative transform (zone 1 ownership).
    pub fn set_authority_transform(&mut self, transform: Transform) {
        self.authority_transform = transform;
        self.dirty = true;
    }

    /// Applies a single sync interval (<=100 ms in design defaults).
    pub fn sync_step(&mut self) {
        if self.dirty {
            self.ghost_transform = self.authority_transform.clone();
            self.dirty = false;
        }
    }

    /// Reads ghost transform after synchronization.
    pub fn ghost_transform(&self) -> &Transform {
        &self.ghost_transform
    }

    /// Reads authoritative transform.
    pub fn authority_transform(&self) -> &Transform {
        &self.authority_transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-8.7.17.1` `test_overlap_co_simulation`
    #[test]
    fn test_overlap_co_simulation() {
        let mut sim = OverlapSimulator::new(Transform {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        sim.set_authority_transform(Transform {
            x: 9.0,
            y: 2.0,
            z: 3.0,
        });
        sim.sync_step();
        assert_eq!(sim.ghost_transform(), sim.authority_transform());
    }
}
