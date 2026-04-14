//! Domain-specific editor hooks (VFX, quests, loot, hierarchy).

use crate::graph_ir::{GraphId, LogicGraph, Node, NodeId, NodeKind};
use glam::Vec2;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VfxNodeKind {
    Emitter,
    Initializer,
}

#[derive(Debug, Clone)]
pub struct VfxNode {
    pub id: NodeId,
    pub kind: VfxNodeKind,
}

#[derive(Debug, Clone)]
pub struct VfxGraph {
    pub nodes: Vec<VfxNode>,
    pub edges: Vec<(NodeId, NodeId)>,
}

impl VfxGraph {
    /// Connects emitter output to initializer input when both exist.
    pub fn connect_emitter_to_initializer(&mut self) -> bool {
        let em = self.nodes.iter().find(|n| n.kind == VfxNodeKind::Emitter);
        let init = self.nodes.iter().find(|n| n.kind == VfxNodeKind::Initializer);
        if let (Some(e), Some(i)) = (em, init) {
            self.edges.push((e.id, i.id));
            true
        } else {
            false
        }
    }
}

/// Quest objective chain lowered to graph nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestIr {
    pub node_count: usize,
}

pub fn compile_quest_objectives(count: usize) -> QuestIr {
    QuestIr { node_count: count }
}

/// Global node type registry for palettes.
#[derive(Debug, Default, Clone)]
pub struct NodeLibrary {
    pub types: Vec<String>,
}

impl NodeLibrary {
    /// Registers a node type name idempotently.
    pub fn register(&mut self, name: &str) {
        if !self.types.iter().any(|t| t == name) {
            self.types.push(name.to_string());
        }
        self.types.sort();
    }

    /// Enumerates all registered node types deterministically.
    pub fn enumerate(&self) -> &[String] {
        &self.types
    }
}

/// Equipment comparison view model.
#[derive(Debug, Clone)]
pub struct EquipmentTable {
    pub rows: Vec<Vec<i32>>,
    pub comparison_mode: bool,
}

impl EquipmentTable {
    /// Sorts rows by column and enables comparison mode.
    pub fn sort_and_compare(&mut self, col: usize) {
        self.rows.sort_by_key(|r| r.get(col).copied().unwrap_or(0));
        self.comparison_mode = true;
    }
}

/// Price ledger with deterministic inflation step.
#[derive(Debug, Clone)]
pub struct PriceLedger {
    pub prices: Vec<f32>,
    pub inflation_rate: f32,
}

impl PriceLedger {
    /// Applies one inflation simulation tick.
    pub fn inflation_step(&mut self) {
        for p in &mut self.prices {
            *p *= 1.0 + self.inflation_rate;
        }
    }
}

/// Entity row in a hierarchy panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyEntity {
    pub name: String,
    pub template: Option<String>,
    pub selected: bool,
}

/// Hierarchy panel snapshot.
#[derive(Debug, Clone)]
pub struct EntityHierarchyPanel {
    pub entities: Vec<HierarchyEntity>,
}

impl EntityHierarchyPanel {
    /// Returns selected entity names with template markers.
    pub fn selection_summary(&self) -> Vec<String> {
        self.entities
            .iter()
            .filter(|e| e.selected)
            .map(|e| {
                if let Some(t) = &e.template {
                    format!("{} [{}]", e.name, t)
                } else {
                    e.name.clone()
                }
            })
            .collect()
    }
}

/// Inspector field with optional template override badge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectorField {
    pub name: String,
    pub overridden_from_template: bool,
}

#[derive(Debug, Clone)]
pub struct InspectorPanel {
    pub fields: Vec<InspectorField>,
}

impl InspectorPanel {
    /// True when any field shows an override badge.
    pub fn template_override_badge_visible(&self) -> bool {
        self.fields.iter().any(|f| f.overridden_from_template)
    }
}

/// Builds a minimal logic graph for quest compilation tests.
pub fn logic_graph_stub_objectives(n: usize) -> LogicGraph {
    let mut g = LogicGraph::new(crate::graph_ir::GraphDomain::Gameplay, "quest");
    for i in 0..n {
        g.add_node(Node {
            node_id: NodeId(i as u32 + 1),
            kind: NodeKind::Stub {
                name: format!("obj{i}"),
            },
            pins: SmallVec::new(),
            position: Vec2::ZERO,
        });
    }
    g
}

/// Stub graph id for lowering helpers.
pub fn dummy_graph_id() -> GraphId {
    LogicGraph::new(crate::graph_ir::GraphDomain::Tool, "dummy").graph_id
}
