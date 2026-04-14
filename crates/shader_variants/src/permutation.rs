//! Permutation key dimensions and stable content hashing.

use blake3::Hasher;

/// Stable 32-byte content identity for a [`PermutationKey`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ContentHash(pub [u8; 32]);

impl core::fmt::Debug for ContentHash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ContentHash(")?;
        for b in &self.0 {
            write!(f, "{:02x}", b)?;
        }
        write!(f, ")")
    }
}

/// High-level shading model for a compiled shader variant.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShadingModel {
    /// Unlit base pass.
    Unlit = 0,
    /// Standard lit BRDF.
    Lit = 1,
    /// Subsurface scattering profile.
    Subsurface = 2,
    /// Cloth-specific shading.
    Cloth = 3,
    /// Hair strand shading.
    Hair = 4,
    /// Eye shading.
    Eye = 5,
    /// Water surface shading.
    Water = 6,
    /// Custom graph hook.
    Custom = 7,
}

impl ShadingModel {
    /// All shading models in stable discriminant order.
    pub const VARIANTS: [Self; 8] = [
        Self::Unlit,
        Self::Lit,
        Self::Subsurface,
        Self::Cloth,
        Self::Hair,
        Self::Eye,
        Self::Water,
        Self::Custom,
    ];
}

/// Fixed render-path dimension for static dispatch.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RenderPath {
    /// Forward lighting path.
    Forward = 0,
    /// Deferred lighting path.
    Deferred = 1,
    /// G-buffer only pass.
    GBufferOnly = 2,
    /// Depth-only pass.
    DepthOnly = 3,
    /// Shadow map pass.
    ShadowOnly = 4,
}

impl RenderPath {
    /// All render paths in stable discriminant order.
    pub const VARIANTS: [Self; 5] = [
        Self::Forward,
        Self::Deferred,
        Self::GBufferOnly,
        Self::DepthOnly,
        Self::ShadowOnly,
    ];
}

/// Discrete LOD band for shader selection.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LodLevel {
    /// Highest quality LOD.
    High = 0,
    /// Medium quality LOD.
    Medium = 1,
    /// Lowest quality LOD.
    Low = 2,
}

impl LodLevel {
    /// All LOD levels in stable discriminant order.
    pub const VARIANTS: [Self; 3] = [Self::High, Self::Medium, Self::Low];
}

/// Feature bit set gating `#ifdef` blocks in HLSL (bits 0-15 named; 16-31 reserved).
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderFeatures {
    /// Raw feature bitmask.
    pub bits: u32,
}

impl ShaderFeatures {
    /// Number of engine-defined feature bits (0..16); high half is reserved.
    pub const NAMED_BIT_COUNT: u32 = 16;

    /// Bit 0: GPU skinning.
    pub const SKINNING: Self = Self { bits: 1 << 0 };
    /// Bit 1: morph targets.
    pub const MORPH_TARGETS: Self = Self { bits: 1 << 1 };
    /// Bit 2: vertex color modulation.
    pub const VERTEX_COLOR: Self = Self { bits: 1 << 2 };
    /// Bit 3: tangent-space normal mapping.
    pub const NORMAL_MAP: Self = Self { bits: 1 << 3 };
    /// Bit 4: alpha cutout / clip.
    pub const ALPHA_CUTOUT: Self = Self { bits: 1 << 4 };
    /// Bit 5: alpha blending.
    pub const ALPHA_BLEND: Self = Self { bits: 1 << 5 };
    /// Bit 6: disable backface culling.
    pub const TWO_SIDED: Self = Self { bits: 1 << 6 };
    /// Bit 7: parallax / POM.
    pub const PARALLAX: Self = Self { bits: 1 << 7 };
    /// Bit 8: decal projection.
    pub const DECAL: Self = Self { bits: 1 << 8 };
    /// Bit 9: emissive channel.
    pub const EMISSIVE: Self = Self { bits: 1 << 9 };
    /// Bit 10: clear coat layer.
    pub const CLEAR_COAT: Self = Self { bits: 1 << 10 };
    /// Bit 11: anisotropic highlights.
    pub const ANISOTROPIC: Self = Self { bits: 1 << 11 };
    /// Bit 12: refraction / transmission.
    pub const REFRACTION: Self = Self { bits: 1 << 12 };
    /// Bit 13: motion vectors.
    pub const MOTION_VECTORS: Self = Self { bits: 1 << 13 };
    /// Bit 14: per-object velocity export.
    pub const VELOCITY: Self = Self { bits: 1 << 14 };
    /// Bit 15: stereo / multiview.
    pub const STEREO: Self = Self { bits: 1 << 15 };

    /// Returns true when every set bit in `other` is also set in `self`.
    #[must_use]
    pub fn contains(self, other: Self) -> bool {
        self.bits & other.bits == other.bits
    }

    /// Bitwise OR of two feature masks.
    #[must_use]
    pub fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }
}

/// Tuple key over the four finite shader variant dimensions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PermutationKey {
    /// Shading model dimension.
    pub shading_model: ShadingModel,
    /// Material feature bits.
    pub features: ShaderFeatures,
    /// Active render path.
    pub render_path: RenderPath,
    /// LOD band.
    pub lod: LodLevel,
}

impl PermutationKey {
    /// Stable Blake3 digest over the serialized tuple (LE feature bits, `u8` enums).
    #[must_use]
    pub fn content_hash(&self) -> ContentHash {
        let mut hasher = Hasher::new();
        hasher.update(&[self.shading_model as u8]);
        hasher.update(&self.features.bits.to_le_bytes());
        hasher.update(&[self.render_path as u8]);
        hasher.update(&[self.lod as u8]);
        ContentHash(*hasher.finalize().as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_key_hash_deterministic() {
        let k = PermutationKey {
            shading_model: ShadingModel::Lit,
            features: ShaderFeatures::NORMAL_MAP,
            render_path: RenderPath::Forward,
            lod: LodLevel::High,
        };
        let a = k.content_hash();
        let b = k.content_hash();
        assert_eq!(a, b);
    }

    #[test]
    fn test_permutation_key_distinct_features() {
        let base = PermutationKey {
            shading_model: ShadingModel::Lit,
            features: ShaderFeatures { bits: 0 },
            render_path: RenderPath::Forward,
            lod: LodLevel::High,
        };
        let with_nm = PermutationKey {
            features: ShaderFeatures::NORMAL_MAP,
            ..base
        };
        assert_ne!(base.content_hash(), with_nm.content_hash());
    }

    #[test]
    fn test_permutation_key_ordering() {
        let a = PermutationKey {
            shading_model: ShadingModel::Unlit,
            features: ShaderFeatures { bits: 0 },
            render_path: RenderPath::Forward,
            lod: LodLevel::High,
        };
        let b = PermutationKey {
            shading_model: ShadingModel::Lit,
            ..a
        };
        assert!(a < b);
        let mut v = vec![b, a];
        v.sort();
        assert_eq!(v, vec![a, b]);
    }

    #[test]
    fn test_dimension_shading_model_count() {
        assert_eq!(ShadingModel::VARIANTS.len(), 8);
    }

    #[test]
    fn test_dimension_features_16_bits_used() {
        assert_eq!(ShaderFeatures::NAMED_BIT_COUNT, 16);
        let _named = [
            ShaderFeatures::SKINNING,
            ShaderFeatures::MORPH_TARGETS,
            ShaderFeatures::VERTEX_COLOR,
            ShaderFeatures::NORMAL_MAP,
            ShaderFeatures::ALPHA_CUTOUT,
            ShaderFeatures::ALPHA_BLEND,
            ShaderFeatures::TWO_SIDED,
            ShaderFeatures::PARALLAX,
            ShaderFeatures::DECAL,
            ShaderFeatures::EMISSIVE,
            ShaderFeatures::CLEAR_COAT,
            ShaderFeatures::ANISOTROPIC,
            ShaderFeatures::REFRACTION,
            ShaderFeatures::MOTION_VECTORS,
            ShaderFeatures::VELOCITY,
            ShaderFeatures::STEREO,
        ];
        let reserved = ShaderFeatures { bits: 1 << 31 };
        assert!(!ShaderFeatures { bits: 0 }.contains(reserved));
    }

    #[test]
    fn test_dimension_render_path_5_variants() {
        assert_eq!(RenderPath::VARIANTS.len(), 5);
    }

    #[test]
    fn test_dimension_lod_3_levels() {
        assert_eq!(LodLevel::VARIANTS.len(), 3);
    }

    #[test]
    fn test_features_contains() {
        let a = ShaderFeatures::SKINNING.union(ShaderFeatures::NORMAL_MAP);
        assert!(a.contains(ShaderFeatures::SKINNING));
        assert!(!ShaderFeatures::SKINNING.contains(a));
    }

    #[test]
    fn test_features_union() {
        let u = ShaderFeatures::EMISSIVE.union(ShaderFeatures::DECAL);
        assert_eq!(
            u.bits,
            ShaderFeatures::EMISSIVE.bits | ShaderFeatures::DECAL.bits
        );
    }
}
