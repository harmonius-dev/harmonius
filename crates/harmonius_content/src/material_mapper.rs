//! glTF PBR → engine material descriptor mapping.

/// Source material format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialSource {
    /// glTF 2.0 metallic-roughness.
    GltfPbr,
}

/// Engine texture slots.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HarTextureSlot {
    /// Base color.
    BaseColor,
    /// Metallic-roughness combined.
    MetallicRoughness,
    /// Emissive.
    Emissive,
}

/// Imported glTF PBR material (minimal).
#[derive(Clone, Debug, Default)]
pub struct ImportedMaterial {
    /// Has base color texture.
    pub has_base_color: bool,
    /// Has MR texture.
    pub has_metallic_roughness: bool,
    /// Has emissive texture.
    pub has_emissive: bool,
}

/// Engine material description.
#[derive(Clone, Debug, PartialEq)]
pub struct HarMaterialDesc {
    /// Base color factor or map presence.
    pub base_color: bool,
    /// MR map presence.
    pub mr: bool,
    /// Emissive RGBA (fallback when no texture).
    pub emissive: [f32; 4],
}

/// Non-fatal mapping notes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaterialWarning {
    /// Message.
    pub message: String,
}

/// Translate glTF PBR material with configured emissive fallback.
pub fn translate_gltf_pbr_material(
    source: &ImportedMaterial,
    emissive_fallback: [f32; 4],
) -> (HarMaterialDesc, Vec<MaterialWarning>) {
    let w = Vec::new();
    let desc = HarMaterialDesc {
        base_color: source.has_base_color,
        mr: source.has_metallic_roughness,
        emissive: if source.has_emissive {
            [1.0, 1.0, 1.0, 1.0]
        } else {
            emissive_fallback
        },
    };
    (desc, w)
}
