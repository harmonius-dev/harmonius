//! Quad batching keys and contiguous batch descriptors (IR-3.6.2).

/// Blend modes packed into batch descriptors. All variants are defined so batch merging is
/// deterministic (design: `BlendState` in rendering–UI integration).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum BlendState {
    /// Opaque compositing.
    Opaque = 0,
    /// Standard alpha blending.
    AlphaBlend = 1,
    /// Additive blending.
    Additive = 2,
    /// Multiply blending.
    Multiply = 3,
    /// Premultiplied alpha.
    PremultipliedAlpha = 4,
}

/// One draw batch sharing atlas page and blend state (IR-3.6.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BatchDescriptor {
    /// Atlas page index shared by all instances in the batch.
    pub atlas_page: u32,
    /// Blend state for the batch.
    pub blend_state: BlendState,
    /// Offset into the instance buffer for this batch.
    pub instance_offset: u32,
    /// Number of instances in this batch.
    pub instance_count: u32,
}

/// Minimal per-quad routing data used only for batch merging tests (IR-3.6.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuadDrawKey {
    /// Atlas page for the quad’s texture.
    pub atlas_page: u32,
    /// Blend mode for the quad.
    pub blend_state: BlendState,
}

/// Builds batch descriptors for a slice of quads in submission order. A new batch starts whenever
/// `(atlas_page, blend_state)` changes between consecutive quads (IR-3.6.2 indirect args).
#[must_use]
pub fn build_batch_descriptors(keys: &[QuadDrawKey]) -> Vec<BatchDescriptor> {
    if keys.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut offset: u32 = 0;
    let mut i = 0;
    while i < keys.len() {
        let first = keys[i];
        let mut count: u32 = 1;
        i += 1;
        while i < keys.len() && keys[i] == first {
            count = count.saturating_add(1);
            i += 1;
        }
        out.push(BatchDescriptor {
            atlas_page: first.atlas_page,
            blend_state: first.blend_state,
            instance_offset: offset,
            instance_count: count,
        });
        offset = offset.saturating_add(count);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{build_batch_descriptors, BlendState, QuadDrawKey};

    /// TC-IR-3.6.2.1 — same atlas and blend across many quads yields one batch.
    #[test]
    fn tc_ir_3_6_2_1_single_atlas_one_batch() {
        let keys: Vec<_> = (0..100)
            .map(|_| QuadDrawKey {
                atlas_page: 0,
                blend_state: BlendState::AlphaBlend,
            })
            .collect();
        let batches = build_batch_descriptors(&keys);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].instance_count, 100);
        assert_eq!(batches[0].atlas_page, 0);
    }

    /// TC-IR-3.6.2.2 — two atlas pages in contiguous runs yield two batches.
    #[test]
    fn tc_ir_3_6_2_2_two_atlases_two_batches() {
        let mut keys = Vec::new();
        keys.extend((0..50).map(|_| QuadDrawKey {
            atlas_page: 0,
            blend_state: BlendState::Opaque,
        }));
        keys.extend((0..50).map(|_| QuadDrawKey {
            atlas_page: 1,
            blend_state: BlendState::Opaque,
        }));
        let batches = build_batch_descriptors(&keys);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0].instance_count, 50);
        assert_eq!(batches[1].instance_count, 50);
    }

    /// TC-IR-3.6.2.3 — blend change splits batches even on the same atlas.
    #[test]
    fn tc_ir_3_6_2_3_blend_split_two_batches() {
        let mut keys = Vec::new();
        keys.extend((0..50).map(|_| QuadDrawKey {
            atlas_page: 0,
            blend_state: BlendState::Opaque,
        }));
        keys.extend((0..50).map(|_| QuadDrawKey {
            atlas_page: 0,
            blend_state: BlendState::Additive,
        }));
        let batches = build_batch_descriptors(&keys);
        assert_eq!(batches.len(), 2);
    }
}
