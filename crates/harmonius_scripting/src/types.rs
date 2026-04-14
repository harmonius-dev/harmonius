//! Logic graph payloads shared with directed-graph assets.

use harmonius_directed_graphs::DirectedGraph;
use rkyv_derive::{Archive, Deserialize, Serialize};

/// Codegen'd node operation identifier from the visual editor palette.
#[derive(
    Archive, Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize,
)]
#[rkyv(compare(PartialEq))]
pub struct NodeOpId(pub u32);

/// Codegen'd type identifier for pins and edges.
#[derive(
    Archive, Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize,
)]
#[rkyv(compare(PartialEq))]
pub struct ScriptTypeId(pub u32);

impl ScriptTypeId {
    /// Reserved unknown-type sentinel (`u32::MAX`).
    pub const UNKNOWN: ScriptTypeId = ScriptTypeId(u32::MAX);
    /// Builtin `f32` channel type for integration tests.
    pub const F32: ScriptTypeId = ScriptTypeId(1);
    /// Builtin `i32` channel type for integration tests.
    pub const I32: ScriptTypeId = ScriptTypeId(2);
}

/// Node operation type stored in each graph node.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodePayload {
    /// Operation identifier from the palette.
    pub op: NodeOpId,
    /// Input pin types for compile-time checking.
    pub input_types: Vec<ScriptTypeId>,
    /// Output pin types for compile-time checking.
    pub output_types: Vec<ScriptTypeId>,
}

/// Edge metadata for data-flow type checking.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EdgePayload {
    /// Source pin index on the from-node.
    pub source_pin: u16,
    /// Target pin index on the to-node.
    pub target_pin: u16,
    /// Data type flowing along this edge.
    pub data_type: ScriptTypeId,
}

/// Canonical logic graph representation for scripting compilation.
pub type LogicGraph = DirectedGraph<NodePayload, EdgePayload>;
