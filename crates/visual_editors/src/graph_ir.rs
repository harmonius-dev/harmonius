//! Graph intermediate representation shared by visual editors.

use glam::Vec2;
use smallvec::SmallVec;
use uuid::Uuid;

/// Stable identifier for a logic graph asset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GraphId(pub Uuid);

/// Opaque node identifier within a graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub u32);

/// Pin endpoint within a node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PinId(pub u32);

/// Graph-level variable identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VariableId(pub u32);

/// Generic parameter slot used during inference.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GenericParamId(pub u32);

/// Closed-world type tag for editor graphs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeId(pub u32);

/// High-level authoring domain for a graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GraphDomain {
    Gameplay,
    Shader,
    Material,
    Animation,
    Audio,
    RenderPipeline,
    Tool,
}

/// Minimal node taxonomy for deterministic tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Stub {
        name: String,
    },
    PureAdd,
    ConstantI32(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PinDirection {
    In,
    Out,
}

/// Pin typing surface used by inference and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PinType {
    Execution,
    Data(TypeId),
    Generic(GenericParamId),
    Wildcard,
}

#[derive(Debug, Clone)]
pub struct Pin {
    pub pin_id: PinId,
    pub direction: PinDirection,
    pub pin_type: PinType,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub source_node: NodeId,
    pub source_pin: PinId,
    pub target_node: NodeId,
    pub target_pin: PinId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableScope {
    Graph,
    Local,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub var_id: VariableId,
    pub name: String,
    pub type_id: TypeId,
    pub scope: VariableScope,
}

#[derive(Debug, Clone)]
pub struct SubgraphRef {
    pub graph_id: GraphId,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_id: NodeId,
    pub kind: NodeKind,
    pub pins: SmallVec<[Pin; 8]>,
    pub position: Vec2,
}

#[derive(Debug, Clone)]
pub struct LogicGraph {
    pub graph_id: GraphId,
    pub domain: GraphDomain,
    pub name: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub variables: Vec<Variable>,
    pub subgraph_refs: Vec<SubgraphRef>,
}

impl LogicGraph {
    /// Creates an empty graph with a fresh [`GraphId`].
    pub fn new(domain: GraphDomain, name: &str) -> Self {
        Self {
            graph_id: GraphId(Uuid::new_v4()),
            domain,
            name: name.to_string(),
            nodes: Vec::new(),
            edges: Vec::new(),
            variables: Vec::new(),
            subgraph_refs: Vec::new(),
        }
    }

    /// Appends a node to the graph.
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    /// Removes a node and any incident edges.
    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.retain(|n| n.node_id != id);
        self
            .edges
            .retain(|e| e.source_node != id && e.target_node != id);
    }

    /// Records a directed edge between pins.
    pub fn connect(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    /// Adds a graph variable.
    pub fn add_variable(&mut self, variable: Variable) {
        self.variables.push(variable);
    }

    /// Deletes a variable by id.
    pub fn remove_variable(&mut self, id: VariableId) {
        self.variables.retain(|v| v.var_id != id);
    }

    /// Renames an existing variable in place.
    pub fn rename_variable(&mut self, id: VariableId, new_name: &str) {
        if let Some(v) = self.variables.iter_mut().find(|v| v.var_id == id) {
            v.name.clear();
            v.name.push_str(new_name);
        }
    }
}
