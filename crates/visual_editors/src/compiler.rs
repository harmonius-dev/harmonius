//! ECS compilation bridge for logic graphs.

use crate::graph_ir::{GraphId, LogicGraph};
use crate::inference::StaticTypeTable;
use crate::validate::NodeRegistry;

/// Compiled ECS-facing artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledGraph {
    pub graph_id: GraphId,
    pub systems: Vec<String>,
    pub suspend_states: usize,
}

/// Compile-time failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileError(pub String);

/// Transforms a validated graph into ECS systems (stub strings).
pub struct EcsCompiler;

impl EcsCompiler {
    /// Compiles a graph when validation passes.
    pub fn compile(
        graph: &LogicGraph,
        _type_table: &'static StaticTypeTable,
        registry: &NodeRegistry,
    ) -> Result<CompiledGraph, CompileError> {
        let v = crate::validate::GraphValidator::validate(graph, registry);
        if !v.errors.is_empty() {
            return Err(CompileError(v.errors.join(";")));
        }
        Ok(CompiledGraph {
            graph_id: graph.graph_id,
            systems: vec!["compiled_system_stub".into()],
            suspend_states: 1,
        })
    }
}

/// Render graph compile output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderGraphPasses {
    pub passes: Vec<String>,
}

pub fn compile_render_graph_config() -> RenderGraphPasses {
    RenderGraphPasses {
        passes: vec!["gbuffer".into(), "lighting".into()],
    }
}

/// Animation logic graph lowering marker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimEcsSystems {
    pub names: Vec<String>,
}

pub fn lower_animation_logic_graph() -> AnimEcsSystems {
    AnimEcsSystems {
        names: vec!["anim_graph_system".into()],
    }
}

/// Audio mixer event list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MixerEvents {
    pub events: Vec<String>,
}

pub fn lower_audio_logic_graph() -> MixerEvents {
    MixerEvents {
        events: vec!["route_bus".into()],
    }
}

/// Custom tool registration entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolAction {
    pub name: String,
}

pub fn compile_custom_tool_graph() -> Vec<ToolAction> {
    vec![ToolAction {
        name: "custom_tool_action".into(),
    }]
}
