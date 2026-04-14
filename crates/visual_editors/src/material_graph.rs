//! Material graph authoring surface.

use glam::Vec2;

use crate::graph_ir::NodeId;
use crate::shader::HlslSource;

/// Asset handle placeholder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AssetId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaterialPinId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaterialPinType {
    Float,
    Vec3,
    Texture2d,
}

#[derive(Debug, Clone)]
pub struct MaterialPin {
    pub id: MaterialPinId,
    pub direction: crate::graph_ir::PinDirection,
    pub pin_type: MaterialPinType,
}

#[derive(Debug, Clone)]
pub struct MaterialNode {
    pub id: NodeId,
    pub kind: MaterialNodeType,
    pub pins: Vec<MaterialPin>,
    pub position: Vec2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaterialNodeType {
    PbrOutput,
    SampleTexture,
    FunctionInline { body: String },
    Parameter { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialEdge {
    pub from_node: NodeId,
    pub from_pin: MaterialPinId,
    pub to_node: NodeId,
    pub to_pin: MaterialPinId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PinTypeError {
    Incompatible,
}

#[derive(Debug, Clone)]
pub struct MaterialGraph {
    pub id: AssetId,
    pub nodes: Vec<MaterialNode>,
    pub edges: Vec<MaterialEdge>,
    pub parameter_values: Vec<(String, f32)>,
    pub last_preview_edit_generation: u64,
}

impl MaterialGraph {
    /// Empty material graph.
    pub fn new(id: AssetId) -> Self {
        Self {
            id,
            nodes: Vec::new(),
            edges: Vec::new(),
            parameter_values: Vec::new(),
            last_preview_edit_generation: 0,
        }
    }

    /// Adds a material node and returns its id.
    pub fn add_node(
        &mut self,
        node_type: MaterialNodeType,
        position: Vec2,
    ) -> NodeId {
        let id = NodeId(
            self.nodes
                .iter()
                .map(|n| n.id.0)
                .max()
                .unwrap_or(0)
                + 1,
        );
        let pins = default_pins(&node_type);
        self.nodes.push(MaterialNode {
            id,
            kind: node_type,
            pins,
            position,
        });
        id
    }

    /// Connects two pins with type checking.
    pub fn connect(
        &mut self,
        from_node: NodeId,
        from_pin: MaterialPinId,
        to_node: NodeId,
        to_pin: MaterialPinId,
    ) -> Result<(), PinTypeError> {
        let ft = pin_type_of(self, from_node, from_pin).ok_or(PinTypeError::Incompatible)?;
        let tt = pin_type_of(self, to_node, to_pin).ok_or(PinTypeError::Incompatible)?;
        if ft != tt {
            return Err(PinTypeError::Incompatible);
        }
        self.edges.push(MaterialEdge {
            from_node,
            from_pin,
            to_node,
            to_pin,
        });
        Ok(())
    }

    /// Emits minimal HLSL for validation pipelines.
    pub fn compile(&self) -> Result<HlslSource, String> {
        if self.nodes.iter().any(|n| matches!(n.kind, MaterialNodeType::PbrOutput)) {
            Ok(HlslSource("// material\nfloat4 main() : SV_Position { return 0; }\n".into()))
        } else {
            Err("missing PBR output".into())
        }
    }

    /// Updates a scalar parameter without changing the compile signature.
    pub fn tweak_parameter(&mut self, name: &str, value: f32) {
        if let Some(i) = self.parameter_values.iter().position(|(k, _)| k == name) {
            self.parameter_values[i].1 = value;
        } else {
            self.parameter_values.push((name.to_string(), value));
            self.parameter_values.sort_by(|a, b| a.0.cmp(&b.0));
        }
        self.touch_preview_after_param_edit();
    }

    /// Marks preview as needing refresh after a parameter tweak.
    pub fn touch_preview_after_param_edit(&mut self) {
        self.last_preview_edit_generation = self.last_preview_edit_generation.saturating_add(1);
    }

    /// Returns true when split-view preview should refresh.
    pub fn split_view_dirty(&self) -> bool {
        self.last_preview_edit_generation > 0
    }
}

fn default_pins(kind: &MaterialNodeType) -> Vec<MaterialPin> {
    use crate::graph_ir::PinDirection::{In, Out};
    match kind {
        MaterialNodeType::PbrOutput => vec![MaterialPin {
            id: MaterialPinId(0),
            direction: In,
            pin_type: MaterialPinType::Vec3,
        }],
        MaterialNodeType::SampleTexture => vec![
            MaterialPin {
                id: MaterialPinId(0),
                direction: In,
                pin_type: MaterialPinType::Texture2d,
            },
            MaterialPin {
                id: MaterialPinId(1),
                direction: Out,
                pin_type: MaterialPinType::Vec3,
            },
        ],
        MaterialNodeType::FunctionInline { .. } => vec![],
        MaterialNodeType::Parameter { .. } => vec![MaterialPin {
            id: MaterialPinId(0),
            direction: Out,
            pin_type: MaterialPinType::Float,
        }],
    }
}

fn pin_type_of(g: &MaterialGraph, node: NodeId, pin: MaterialPinId) -> Option<MaterialPinType> {
    let n = g.nodes.iter().find(|n| n.id == node)?;
    let p = n.pins.iter().find(|p| p.id == pin)?;
    Some(p.pin_type.clone())
}

/// Inlines a material function body by copying its string onto the graph.
pub fn inline_material_function(graph: &mut MaterialGraph, body: &str) {
    let id = graph.add_node(
        MaterialNodeType::FunctionInline {
            body: body.to_string(),
        },
        Vec2::ZERO,
    );
    let _ = id;
}
