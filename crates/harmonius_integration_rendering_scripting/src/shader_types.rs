//! Shader models, render paths, feature flags, and permutation keys.

use core::cmp::Ordering;

use rkyv_derive::{Archive, Deserialize, Serialize};

/// Codegen target for the graph compiler (IR-3.5.1).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum CompileTarget {
    /// Emit HLSL source for a shader graph.
    Hlsl = 0,
    /// Emit Rust source for a logic graph compiled into a middleman dylib.
    NativeDylib = 1,
}

/// Surface shading model (IR-3.5.1).
#[derive(
    Archive, Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[rkyv(
    bytecheck(),
    derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)
)]
#[repr(u8)]
pub enum ShadingModel {
    /// Default PBR lit surface.
    DefaultLit = 0,
    /// Subsurface scattering profile.
    Subsurface = 1,
    /// Clear coat layered specular.
    ClearCoat = 2,
    /// Woven cloth response.
    Cloth = 3,
    /// Anisotropic hair strands.
    Hair = 4,
    /// Refractive eye shading.
    Eye = 5,
    /// Thin translucent sheet.
    ThinTranslucent = 6,
    /// Two-sided foliage.
    Foliage = 7,
    /// Unlit emissive-only.
    Unlit = 8,
}

/// Forward-plus or deferred path (IR-3.5.3).
#[derive(
    Archive, Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[rkyv(
    bytecheck(),
    derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)
)]
#[repr(u8)]
pub enum RenderPath {
    /// Forward+ clustered lighting.
    ForwardPlus = 0,
    /// G-buffer deferred shading.
    Deferred = 1,
}

/// Optional shader features as a `u32` bitset (IR-3.5.3).
#[derive(
    Archive, Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[rkyv(
    bytecheck(),
    derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)
)]
#[repr(transparent)]
pub struct ShaderFeatures(pub u32);

impl ShaderFeatures {
    /// No optional features enabled.
    pub const NONE: Self = Self(0);
    /// Tangent-space normal mapping.
    pub const NORMAL_MAP: Self = Self(1 << 0);
    /// Emissive channel.
    pub const EMISSIVE: Self = Self(1 << 1);
    /// Alpha cutout / clip.
    pub const ALPHA_CLIP: Self = Self(1 << 2);
    /// Per-vertex color modulation.
    pub const VERTEX_COLOR: Self = Self(1 << 3);
    /// Secondary detail map.
    pub const DETAIL_MAP: Self = Self(1 << 4);
    /// Height / parallax mapping.
    pub const PARALLAX: Self = Self(1 << 5);
    /// Second normal for clear coat layer.
    pub const CLEARCOAT_NORMAL: Self = Self(1 << 6);
    /// Subsurface scattering feature block.
    pub const SUBSURFACE_SCATTER: Self = Self(1 << 7);
    /// Reserved forward-compat bit (test TC-IR-3.5.3.U2 nine-bit layout).
    pub const RESERVED_FORWARD: Self = Self(1 << 8);
    /// Union of every named feature bit used in IR-3.5.3 tests.
    pub const ALL_NAMED_BITS: Self = Self(0x1FF);
}

/// Unique shader permutation (IR-3.5.3); compared via sorted `Vec` lookup, not `HashMap`.
#[derive(
    Archive, Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[rkyv(
    bytecheck(),
    derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)
)]
pub struct PermutationKey {
    /// Active shading model.
    pub shading_model: ShadingModel,
    /// Optional feature bits.
    pub features: ShaderFeatures,
    /// Forward or deferred selection.
    pub render_path: RenderPath,
}

impl PermutationKey {
    /// Stable 64-bit fingerprint for diagnostics (not `std::hash::Hash` / `RandomState`).
    #[must_use]
    pub fn stable_fingerprint(self) -> u64 {
        let mut x = 0x9E37_79B9_7F4A_7C15u64;
        x ^= u64::from(self.shading_model as u8);
        x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
        x ^= u64::from(self.features.0);
        x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
        x ^= u64::from(self.render_path as u8);
        x ^ (x >> 32)
    }
}

/// Lexicographic ordering for sorted permutation tables (IR-3.5.3).
#[must_use]
pub fn cmp_permutation_key(a: &PermutationKey, b: &PermutationKey) -> Ordering {
    a.shading_model
        .cmp(&b.shading_model)
        .then_with(|| a.features.0.cmp(&b.features.0))
        .then_with(|| a.render_path.cmp(&b.render_path))
}

/// Span of [`PermutationKey`] rows stored in [`super::MaterialShaderCache`](crate::MaterialShaderCache).
#[derive(Archive, Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(bytecheck(), derive(Copy, Clone, Debug, Eq, PartialEq))]
pub struct PermutationSpan {
    /// Start index in the permutation arena.
    pub offset: u32,
    /// Number of consecutive keys.
    pub length: u32,
}

/// `dxc` shader profile (IR-3.5.2).
#[derive(
    Archive, Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[rkyv(
    bytecheck(),
    derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)
)]
#[repr(u8)]
pub enum ShaderProfile {
    /// VS 6.6.
    Vertex6_6 = 0,
    /// PS 6.6.
    Pixel6_6 = 1,
    /// CS 6.6.
    Compute6_6 = 2,
    /// Mesh shader 6.6.
    Mesh6_6 = 3,
    /// Amplification shader 6.6.
    Amplification6_6 = 4,
}
