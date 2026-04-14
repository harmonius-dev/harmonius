//! Minimal shader graph IR for deterministic HLSL emission tests (IR-3.5.1).

use crate::shader_types::{RenderPath, ShaderFeatures, ShadingModel};

/// Stable node id used in topological ordering (IR-3.5.1).
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId(pub u32);

/// PBR-related node kinds for fixture graphs (IR-3.5.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphNodeKind {
    /// Base color input.
    Albedo,
    /// Metallic scalar.
    Metallic,
    /// Perceptual roughness.
    Roughness,
    /// Surface closure combining inputs.
    Surface,
}

/// Authoring-time graph snapshot passed to [`crate::codegen_hlsl`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShaderGraphIr {
    /// Shading model selected in the graph root.
    pub shading_model: ShadingModel,
    /// Forward vs deferred path.
    pub render_path: RenderPath,
    /// Enabled optional features for this material.
    pub features: ShaderFeatures,
    /// Node ids with kinds (ids must be unique).
    pub nodes: Vec<(NodeId, GraphNodeKind)>,
    /// `(producer, consumer)` dependency: `consumer` is emitted after `producer`.
    pub edges: Vec<(NodeId, NodeId)>,
}

impl ShaderGraphIr {
    /// Deterministic three-node PBR graph feeding one surface node (TC-IR-3.5.1.U3).
    #[must_use]
    pub fn pbr_three_node_graph() -> Self {
        Self {
            shading_model: ShadingModel::DefaultLit,
            render_path: RenderPath::ForwardPlus,
            features: ShaderFeatures::NORMAL_MAP,
            nodes: vec![
                (NodeId(0), GraphNodeKind::Albedo),
                (NodeId(1), GraphNodeKind::Metallic),
                (NodeId(2), GraphNodeKind::Roughness),
                (NodeId(3), GraphNodeKind::Surface),
            ],
            edges: vec![
                (NodeId(0), NodeId(3)),
                (NodeId(1), NodeId(3)),
                (NodeId(2), NodeId(3)),
            ],
        }
    }
}
