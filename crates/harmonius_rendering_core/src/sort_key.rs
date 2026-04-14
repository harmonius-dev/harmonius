//! Packed 64-bit draw sort keys (`SortKey` in the rendering-core design, TC-2.10.4.2).
//!
//! # Bit layout (MSB → LSB)
//!
//! | Field         | Width | Notes                                      |
//! |---------------|------:|--------------------------------------------|
//! | `translucent` | 1     | `0` = opaque path, `1` = translucent path |
//! | `phase`       | 8     | `RenderPhase` discriminant (`0..=255`)     |
//! | `pipeline`    | 11    | Pipeline id (`0..2048`)                    |
//! | `material`    | 12    | `MaterialId` payload (`0..4096`)           |
//! | `depth`       | 32    | `f32::to_bits` view-space depth key        |
//!
//! Total: 64 bits. `encode` / `decode` are inverse functions for valid field ranges.

/// Raw sort key carried on draw commands in the render queue.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SortKey(pub u64);

/// Decoded sort-key fields (TC-2.10.4.2 round-trip payload).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SortKeyFields {
    /// `false` for opaque draw order, `true` for translucent / back-to-front path.
    pub translucent: bool,
    /// Compact render phase id (`0..256`).
    pub phase: u8,
    /// Pipeline id (`0..2048`).
    pub pipeline: u16,
    /// Material id payload (`0..4096`).
    pub material: u16,
    /// Depth ordering key, stored as `f32` bit pattern.
    pub depth: f32,
}

const TRANSLUCENT_SHIFT: u32 = 63;
const PHASE_SHIFT: u32 = 55;
const PIPELINE_SHIFT: u32 = 44;
const MATERIAL_SHIFT: u32 = 32;

const PHASE_MASK: u64 = 0xFF;
const PIPELINE_MASK: u64 = 0x7FF;
const MATERIAL_MASK: u64 = 0xFFF;

impl SortKey {
    /// Packs `fields` into a single `u64` key.
    #[must_use]
    pub fn encode(fields: SortKeyFields) -> Self {
        debug_assert!(fields.pipeline < 2048);
        debug_assert!(fields.material < 4096);
        let t = u64::from(fields.translucent) << TRANSLUCENT_SHIFT;
        let ph = u64::from(fields.phase) << PHASE_SHIFT;
        let pl = u64::from(fields.pipeline & (PIPELINE_MASK as u16)) << PIPELINE_SHIFT;
        let m = u64::from(fields.material & (MATERIAL_MASK as u16)) << MATERIAL_SHIFT;
        let d = u64::from(fields.depth.to_bits());
        Self(t | ph | pl | m | d)
    }

    /// Unpacks every field from `self`.
    #[must_use]
    pub fn decode(self) -> SortKeyFields {
        let bits = self.0;
        let translucent = ((bits >> TRANSLUCENT_SHIFT) & 1) != 0;
        let phase = ((bits >> PHASE_SHIFT) & PHASE_MASK) as u8;
        let pipeline = ((bits >> PIPELINE_SHIFT) & PIPELINE_MASK) as u16;
        let material = ((bits >> MATERIAL_SHIFT) & MATERIAL_MASK) as u16;
        let depth = f32::from_bits(bits as u32);
        SortKeyFields {
            translucent,
            phase,
            pipeline,
            material,
            depth,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SortKey, SortKeyFields};

    /// TC-2.10.4.2 — 64-bit layout round-trips all logical fields.
    #[test]
    fn test_sort_key_64bit_layout() {
        let fields = SortKeyFields {
            translucent: true,
            phase: 91,
            pipeline: 2001,
            material: 900,
            depth: 0.03125,
        };
        let key = SortKey::encode(fields);
        assert_eq!(key.decode(), fields);
    }
}
