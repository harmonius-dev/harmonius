//! Unified GPU light buffer packing for forward and deferred paths (R-2.3.1, TC-2.3.1.1).

/// Discriminant stored with each packed light for shader branching.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum GpuLightKind {
    /// Omnidirectional point source.
    Point = 0,
    /// Cone-shaped spot source.
    Spot = 1,
    /// Orthogonal directional sun/moon.
    Directional = 2,
}

/// Minimal CPU-side record before SSBO upload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GpuLightRecord {
    /// Light category.
    pub kind: GpuLightKind,
    /// Stable slot index in the unified buffer.
    pub slot: u32,
}

/// Packs lights in submission order. One record per input light.
#[must_use]
pub fn pack_unified_light_buffer(kinds: &[GpuLightKind]) -> Vec<GpuLightRecord> {
    kinds
        .iter()
        .enumerate()
        .map(|(i, &kind)| GpuLightRecord {
            kind,
            slot: i as u32,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{pack_unified_light_buffer, GpuLightKind, GpuLightRecord};

    /// TC-2.3.1.1 — point, spot, and directional lights land in one buffer with correct kinds.
    #[test]
    fn test_unified_light_buffer_write() {
        let kinds = [
            GpuLightKind::Point,
            GpuLightKind::Spot,
            GpuLightKind::Directional,
        ];
        let buffer = pack_unified_light_buffer(&kinds);
        assert_eq!(buffer.len(), 3);
        assert_eq!(
            buffer,
            vec![
                GpuLightRecord {
                    kind: GpuLightKind::Point,
                    slot: 0,
                },
                GpuLightRecord {
                    kind: GpuLightKind::Spot,
                    slot: 1,
                },
                GpuLightRecord {
                    kind: GpuLightKind::Directional,
                    slot: 2,
                },
            ]
        );
        let kinds_in_buffer: Vec<GpuLightKind> = buffer.iter().map(|r| r.kind).collect();
        assert!(kinds_in_buffer.contains(&GpuLightKind::Point));
        assert!(kinds_in_buffer.contains(&GpuLightKind::Spot));
        assert!(kinds_in_buffer.contains(&GpuLightKind::Directional));
    }
}
