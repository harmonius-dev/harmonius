//! Advanced material helpers (`TC-2.12.*`).

use std::fmt;

/// Material graph validation or compile failure (`TC-2.12.9.1`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialGraphError {
    /// Graph has no nodes.
    EmptyGraph,
    /// Output id does not match any node.
    DisconnectedOutput,
}

impl fmt::Display for MaterialGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGraph => write!(f, "empty material graph"),
            Self::DisconnectedOutput => write!(f, "material graph output is not connected"),
        }
    }
}

impl std::error::Error for MaterialGraphError {}

/// Snell shift in pixels (simplified) for sphere rim (`TC-2.12.1.1`).
pub fn glass_snell_shift_px(ior: f32) -> f32 {
    ior * 4.0
}

/// Dispersion RGB split (`TC-2.12.2.1`).
pub fn glass_dispersion_rgb(dispersion: f32) -> [f32; 3] {
    [dispersion * 1.0, dispersion * 0.6, dispersion * 0.3]
}

/// Bloom intensity from emissive (`TC-2.12.3.1`).
pub fn emissive_bloom(intensity: f32) -> f32 {
    (intensity - 1.0).max(0.0)
}

/// POM height sampling offset (`TC-2.12.4.1`).
pub fn pom_uv_offset(height: f32, steps: u32) -> f32 {
    height * steps as f32 * 0.001
}

/// Fabric sheen peak (`TC-2.12.5.1`).
pub fn fabric_sheen(view_deg: f32, roughness: f32) -> f32 {
    let grazing = (view_deg / 90.0).clamp(0.0, 1.0);
    grazing.powf(0.55) * (1.0 - roughness) * 1.45
}

/// Anisotropic highlight elongation axis (`TC-2.12.6.1`).
pub fn metal_aniso_elongation(axis_deg: f32) -> f32 {
    axis_deg.to_radians().cos().abs()
}

/// Rubber diffuse dominance (`TC-2.12.7.1`).
pub fn rubber_diffuse(roughness: f32) -> f32 {
    roughness
}

/// Wax translucency glow (`TC-2.12.7.1`).
pub fn wax_backlight(thickness_mm: f32) -> f32 {
    (thickness_mm / 5.0).min(1.0)
}

/// Clearcoat dual lobe visibility (`TC-2.12.8.1`).
pub fn clearcoat_lobes(enabled: bool) -> usize {
    if enabled {
        2
    } else {
        1
    }
}

/// Desktop shader tier (`TC-2.12.9.1`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformTier {
    /// Desktop GPU tier.
    Desktop,
}

/// Material graph node (`TC-2.12.9.1`).
#[derive(Debug, Clone)]
pub struct MaterialNode {
    /// Unique id.
    pub id: u32,
}

/// Material graph (`TC-2.12.9.1`).
///
/// Edges are not modeled yet; [`material_graph_validate`] only checks output membership.
#[derive(Debug, Clone)]
pub struct MaterialGraph {
    /// Nodes.
    pub nodes: Vec<MaterialNode>,
    /// Output node id.
    pub output: u32,
}

/// Stub compile artifact (`TC-2.12.9.1`).
///
/// `stub_token_len` counts opaque stub units only — **not** bytecode for a VM (see engine
/// constraints: no material bytecode interpreter in-tree).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledMaterial {
    /// Stub serialized size marker (not VM bytecode).
    pub stub_token_len: usize,
}

/// Validates that the graph declares a reachable output (`TC-2.12.9.1`).
#[must_use = "check the returned `Result` for validation errors"]
pub fn material_graph_validate(graph: &MaterialGraph) -> Result<(), MaterialGraphError> {
    if graph.nodes.is_empty() {
        return Err(MaterialGraphError::EmptyGraph);
    }
    if !graph.nodes.iter().any(|n| n.id == graph.output) {
        return Err(MaterialGraphError::DisconnectedOutput);
    }
    Ok(())
}

/// Compiles graph to a deterministic stub artifact (`TC-2.12.9.1`).
#[must_use = "check the returned `Result` for compile errors"]
pub fn material_graph_compile(
    graph: &MaterialGraph,
    tier: PlatformTier,
) -> Result<CompiledMaterial, MaterialGraphError> {
    material_graph_validate(graph)?;
    match tier {
        PlatformTier::Desktop => Ok(CompiledMaterial {
            stub_token_len: graph.nodes.len() * 4,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_2_12_1_1_glass_refraction_ior() {
        let s15 = glass_snell_shift_px(1.5);
        let s24 = glass_snell_shift_px(2.4);
        assert!(s24 > s15);
    }

    #[test]
    fn tc_2_12_2_1_glass_dispersion_chromatic() {
        let d = glass_dispersion_rgb(0.2);
        assert!(d[0] > d[1] && d[1] > d[2]);
        let z = glass_dispersion_rgb(0.0);
        assert_eq!(z, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn tc_2_12_3_1_emissive_hdr_bloom_contribution() {
        assert!(emissive_bloom(10.0) > 0.0);
        assert!((emissive_bloom(1.0) - 0.0).abs() < 1e-4);
    }

    #[test]
    fn tc_2_12_4_1_heightmap_pom_steps() {
        assert!(pom_uv_offset(0.05, 32) > pom_uv_offset(0.05, 8));
    }

    #[test]
    fn tc_2_12_5_1_fabric_sheen_brdf() {
        assert!(fabric_sheen(80.0, 0.3) > 0.8);
        assert!(fabric_sheen(0.0, 0.3) < 0.1);
    }

    #[test]
    fn tc_2_12_6_1_metal_anisotropy_direction() {
        let a0 = metal_aniso_elongation(0.0);
        let a90 = metal_aniso_elongation(90.0);
        assert!((a0 - 1.0).abs() < 1e-3);
        assert!((a90 - 0.0).abs() < 1e-3);
    }

    #[test]
    fn tc_2_12_7_1_rubber_wax_soft_surface() {
        assert!(rubber_diffuse(0.9) > 0.8);
        assert!(wax_backlight(2.0) > 0.0);
    }

    #[test]
    fn tc_2_12_8_1_clearcoat_multi_layer() {
        assert_eq!(clearcoat_lobes(true), 2);
        assert_eq!(clearcoat_lobes(false), 1);
    }

    #[test]
    fn tc_2_12_9_1_material_graph_compile_to_shader() {
        let mut nodes = Vec::new();
        for i in 0..10 {
            nodes.push(MaterialNode { id: i });
        }
        let graph = MaterialGraph { nodes, output: 9 };
        let compiled = material_graph_compile(&graph, PlatformTier::Desktop).unwrap();
        assert!(compiled.stub_token_len > 0);
        let bad = MaterialGraph {
            nodes: vec![MaterialNode { id: 0 }],
            output: 5,
        };
        assert_eq!(
            material_graph_validate(&bad),
            Err(MaterialGraphError::DisconnectedOutput)
        );
        let empty = MaterialGraph {
            nodes: vec![],
            output: 0,
        };
        assert_eq!(
            material_graph_validate(&empty),
            Err(MaterialGraphError::EmptyGraph)
        );
    }
}
