//! `PipelineStateTable` — generational arena + sorted variant index (IR-5.2.6).

use crate::contracts::{GpuTextureFormat, VertexLayout};

/// Stable key for a shader variant (content or structural hash in full engine).
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VariantKey(pub u64);

/// Describes a validated GPU pipeline.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PipelineStateDesc {
    pub shader_hash: [u8; 32],
    pub vertex_layout: VertexLayout,
    pub blend_state: crate::contracts::BlendState,
    pub depth_stencil_state: crate::contracts::DepthStencilState,
    pub rasterizer_state: crate::contracts::RasterizerState,
    pub render_target_formats: Vec<GpuTextureFormat>,
}

/// Generational handle into `PipelineStateTable`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PipelineStateHandle {
    pub index: u32,
    pub generation: u32,
}

#[derive(Debug)]
enum Slot {
    Vacant {
        next_generation: u32,
    },
    Occupied {
        generation: u32,
        desc: PipelineStateDesc,
    },
}

/// Typed arena of pipeline state descriptors with sorted variant lookup.
#[derive(Debug, Default)]
pub struct PipelineStateTable {
    slots: Vec<Slot>,
    variants: Vec<(VariantKey, PipelineStateHandle)>,
}

impl PipelineStateTable {
    /// Creates an empty table.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Publishes a descriptor for `key`, replacing any prior mapping for that key.
    pub fn publish_variant(
        &mut self,
        key: VariantKey,
        desc: PipelineStateDesc,
    ) -> PipelineStateHandle {
        if let Ok(i) = self.variants.binary_search_by(|e| e.0.cmp(&key)) {
            let old = self.variants[i].1;
            self.free(old);
        }
        let handle = self.publish(desc);
        match self.variants.binary_search_by(|e| e.0.cmp(&key)) {
            Ok(i) => self.variants[i] = (key, handle),
            Err(i) => self.variants.insert(i, (key, handle)),
        }
        handle
    }

    /// Publishes a descriptor into the next free slot.
    pub fn publish(&mut self, desc: PipelineStateDesc) -> PipelineStateHandle {
        let (index, generation) = self.allocate_slot(desc);
        PipelineStateHandle { index, generation }
    }

    /// Frees a slot; any existing handle with the same `(index, generation)` becomes stale.
    pub fn free(&mut self, handle: PipelineStateHandle) {
        let idx = handle.index as usize;
        if idx >= self.slots.len() {
            return;
        }
        if let Slot::Occupied { generation, .. } = &self.slots[idx] {
            if *generation != handle.generation {
                return;
            }
        } else {
            return;
        }
        let next_generation = handle.generation.saturating_add(1);
        self.slots[idx] = Slot::Vacant { next_generation };
        self.variants.retain(|(_, h)| h.index != handle.index);
    }

    /// Resolves a live handle to its descriptor.
    #[must_use]
    pub fn resolve(&self, handle: PipelineStateHandle) -> Option<&PipelineStateDesc> {
        let idx = handle.index as usize;
        match self.slots.get(idx)? {
            Slot::Occupied { generation, desc } if *generation == handle.generation => Some(desc),
            _ => None,
        }
    }

    /// Looks up the newest published handle for `key`.
    #[must_use]
    pub fn lookup_by_variant(&self, key: VariantKey) -> Option<PipelineStateHandle> {
        self.variants
            .binary_search_by(|e| e.0.cmp(&key))
            .ok()
            .map(|i| self.variants[i].1)
    }

    fn allocate_slot(&mut self, desc: PipelineStateDesc) -> (u32, u32) {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if let Slot::Vacant { next_generation } = slot {
                let generation = *next_generation;
                *slot = Slot::Occupied { generation, desc };
                return (i as u32, generation);
            }
        }
        let generation = 0;
        let index = self.slots.len() as u32;
        self.slots.push(Slot::Occupied { generation, desc });
        (index, generation)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;
    use crate::contracts::{BlendState, DepthStencilState, RasterizerState};

    fn sample_desc() -> PipelineStateDesc {
        PipelineStateDesc {
            shader_hash: [7_u8; 32],
            vertex_layout: VertexLayout,
            blend_state: BlendState,
            depth_stencil_state: DepthStencilState,
            rasterizer_state: RasterizerState,
            render_target_formats: vec![GpuTextureFormat::Bc7Unorm],
        }
    }

    #[test]
    fn tc_ir_5_2_6_u1_generation_survives_free_and_republish() {
        let mut table = PipelineStateTable::new();
        let h0 = table.publish(sample_desc());
        assert!(table.resolve(h0).is_some());
        table.free(h0);
        assert!(table.resolve(h0).is_none());
        let h1 = table.publish(sample_desc());
        assert_eq!(h0.index, h1.index);
        assert_ne!(h0.generation, h1.generation);
        assert!(table.resolve(h0).is_none());
        assert!(table.resolve(h1).is_some());
    }

    #[test]
    fn tc_ir_5_2_6_u2_variant_lookup_binary_search_matches_linear_scan() {
        let mut table = PipelineStateTable::new();
        let mut keys: Vec<u64> = (0_u64..100).collect();
        keys.reverse();
        for k in &keys {
            let mut d = sample_desc();
            d.shader_hash[0] = (*k) as u8;
            table.publish_variant(VariantKey(*k), d);
        }
        for i in 1..table.variants.len() {
            assert_eq!(
                table.variants[i - 1].0.cmp(&table.variants[i].0),
                Ordering::Less
            );
        }
        for want in 0_u64..100 {
            let key = VariantKey(want);
            let handle = table.lookup_by_variant(key).expect("handle");
            let desc = table.resolve(handle).expect("desc");
            assert_eq!(desc.shader_hash[0], want as u8);
            let bs = table
                .variants
                .binary_search_by(|e| e.0.cmp(&key))
                .expect("binary search hit");
            assert_eq!(table.variants[bs].0, key);
            assert_eq!(table.variants[bs].1, handle);
        }
    }
}
