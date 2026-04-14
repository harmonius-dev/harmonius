//! Nameplate screen projection buffer (IR-3.6.7).

/// Stable entity identifier placeholder until core ECS types land in-crate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Entity(pub u64);

/// Nameplate screen projection result (frame-transient, design `NameplateScreenPos`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NameplateScreenPos {
    /// Widget / anchor entity.
    pub entity: Entity,
    /// Screen-space position in pixels.
    pub screen_xy: (f32, f32),
    /// Depth after projection (used for culling).
    pub depth: f32,
    /// False when culled (behind camera plane, etc.).
    pub visible: bool,
}

/// Default desktop nameplate buffer capacity (design `NAMEPLATE_CAPACITY`).
pub const NAMEPLATE_CAPACITY: usize = 256;

/// Arena-style nameplate ring without heap growth after construction (IR-3.6.7).
#[derive(Debug)]
pub struct NameplateBuffer<'a> {
    entries: &'a mut [NameplateScreenPos],
    len: usize,
}

impl<'a> NameplateBuffer<'a> {
    /// Wraps a caller-provided slice (bump arena or stack buffer).
    #[must_use]
    pub fn new(entries: &'a mut [NameplateScreenPos]) -> Self {
        Self { entries, len: 0 }
    }

    /// Clears all entries without deallocating backing storage.
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Pushes one projection; returns `false` when the buffer is full (TC-IR-3.6.7.N2).
    pub fn push(&mut self, pos: NameplateScreenPos) -> bool {
        if self.len >= self.entries.len() {
            return false;
        }
        self.entries[self.len] = pos;
        self.len += 1;
        true
    }

    /// Borrow the filled prefix.
    #[must_use]
    pub fn as_slice(&self) -> &[NameplateScreenPos] {
        &self.entries[..self.len]
    }
}

#[cfg(test)]
mod tests {
    use super::{Entity, NameplateBuffer, NameplateScreenPos};

    /// TC-IR-3.6.7.N2 — excess nameplates beyond backing capacity are dropped.
    #[test]
    fn tc_ir_3_6_7_n2_overflow_drops_excess() {
        let mut storage = [NameplateScreenPos {
            entity: Entity(0),
            screen_xy: (0.0, 0.0),
            depth: 0.0,
            visible: false,
        }; 4];
        let mut buf = NameplateBuffer::new(&mut storage);
        for i in 0..10 {
            let ok = buf.push(NameplateScreenPos {
                entity: Entity(i),
                screen_xy: (1.0, 2.0),
                depth: 0.5,
                visible: true,
            });
            assert_eq!(ok, i < 4, "push {i}");
        }
        assert_eq!(buf.as_slice().len(), 4);
    }

    /// TC-IR-3.6.7.N1 — behind-camera samples marked not visible at buffer level.
    #[test]
    fn tc_ir_3_6_7_n1_respects_visible_flag() {
        let mut storage = [NameplateScreenPos {
            entity: Entity(0),
            screen_xy: (0.0, 0.0),
            depth: 0.0,
            visible: false,
        }; 2];
        let mut buf = NameplateBuffer::new(&mut storage);
        assert!(buf.push(NameplateScreenPos {
            entity: Entity(1),
            screen_xy: (0.0, 0.0),
            depth: -1.0,
            visible: false,
        }));
        assert!(!buf.as_slice()[0].visible);
    }
}
