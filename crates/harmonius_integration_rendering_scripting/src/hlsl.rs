//! HLSL arena cache and [`MaterialShaderOutput`] (IR-3.5.1).

use rkyv_derive::{Archive, Deserialize, Serialize};

use crate::shader_types::{PermutationSpan, ShaderFeatures, ShadingModel};

/// Generational handle into [`MaterialShaderCache`] HLSL bytes (IR-3.5.1).
#[derive(
    Archive, Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[rkyv(
    bytecheck(),
    derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)
)]
pub struct HlslSourceHandle {
    /// Slot index in the arena.
    pub index: u32,
    /// Bumped when a slot is recycled.
    pub generation: u32,
}

/// Material graph compilation output persisted through rkyv (IR-3.5.1).
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(bytecheck())]
#[repr(C, align(16))]
pub struct MaterialShaderOutput {
    /// Handle into the HLSL arena.
    pub hlsl: HlslSourceHandle,
    /// Shading model compiled into the graph.
    pub shading_model: ShadingModel,
    /// Feature bits compiled into the graph.
    pub features: ShaderFeatures,
    /// Span of permutation keys in the cache arena.
    pub permutations: PermutationSpan,
    /// Fingerprint of emitted HLSL + metadata.
    pub content_hash: u64,
}

#[derive(Debug)]
struct HlslSlot {
    generation: u32,
    bytes: Vec<u8>,
}

/// Worker-thread HLSL byte arena plus permutation key table (IR-3.5.1).
#[derive(Debug, Default)]
pub struct MaterialShaderCache {
    hlsl_slots: Vec<HlslSlot>,
    hlsl_free: Vec<u32>,
    permutations: Vec<crate::shader_types::PermutationKey>,
}

impl MaterialShaderCache {
    /// Creates an empty cache.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts `src` bytes and returns a generational handle.
    pub fn insert_hlsl(&mut self, src: &[u8]) -> HlslSourceHandle {
        if let Some(index) = self.hlsl_free.pop() {
            let slot = &mut self.hlsl_slots[index as usize];
            slot.bytes.clear();
            slot.bytes.extend_from_slice(src);
            return HlslSourceHandle {
                index,
                generation: slot.generation,
            };
        }
        let index = u32::try_from(self.hlsl_slots.len()).expect("fits u32");
        self.hlsl_slots.push(HlslSlot {
            generation: 0,
            bytes: src.to_vec(),
        });
        HlslSourceHandle {
            index,
            generation: 0,
        }
    }

    /// Returns HLSL bytes for a live handle, or `None` if stale.
    #[must_use]
    pub fn resolve_hlsl(&self, handle: HlslSourceHandle) -> Option<&[u8]> {
        let slot = self.hlsl_slots.get(handle.index as usize)?;
        if slot.generation != handle.generation {
            return None;
        }
        Some(slot.bytes.as_slice())
    }

    /// Appends permutation keys and returns their span coordinates.
    pub fn append_permutation_keys(
        &mut self,
        keys: &[crate::shader_types::PermutationKey],
    ) -> PermutationSpan {
        let offset = u32::try_from(self.permutations.len()).expect("offset fits u32");
        self.permutations.extend_from_slice(keys);
        PermutationSpan {
            offset,
            length: u32::try_from(keys.len()).expect("length fits u32"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_5_1_u2_hlsl_handle_roundtrip() {
        let mut cache = MaterialShaderCache::new();
        let src = b"float3 albedo = 1;\n";
        let h = cache.insert_hlsl(src);
        assert_eq!(cache.resolve_hlsl(h), Some(src.as_slice()));
    }

    #[test]
    fn material_shader_output_aligns_for_rkyv() {
        let out = MaterialShaderOutput {
            hlsl: HlslSourceHandle {
                index: 0,
                generation: 0,
            },
            shading_model: ShadingModel::Unlit,
            features: ShaderFeatures::NONE,
            permutations: PermutationSpan {
                offset: 0,
                length: 0,
            },
            content_hash: 1,
        };
        let p = core::mem::align_of_val(&out);
        assert_eq!(p % 16, 0);
        assert!(core::mem::size_of_val(&out) >= 16);
    }
}
