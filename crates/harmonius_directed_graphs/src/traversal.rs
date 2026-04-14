use crate::id::NodeId;

/// Stable asset handle for graph instances.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct AssetId(pub u64);

/// Dense generational map placeholder carried on traversal state.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HandleMap<T> {
    _marker: core::marker::PhantomData<T>,
}

impl<T> HandleMap<T> {
    /// Empty map.
    pub fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

/// Minimal per-node lifecycle tag for integration tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeStatus {
    /// Node is ready to run.
    Available,
}

/// ECS-facing traversal snapshot for one logic graph instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphTraversalState {
    /// Owning graph asset.
    pub graph_id: AssetId,
    /// Per-node bookkeeping (placeholder in this bootstrap).
    pub node_states: HandleMap<NodeStatus>,
    /// Active node, if any.
    pub current_node: Option<NodeId>,
    /// Tick when traversal began.
    pub started_at: u64,
}

impl Default for GraphTraversalState {
    fn default() -> Self {
        Self {
            graph_id: AssetId(0),
            node_states: HandleMap::new(),
            current_node: None,
            started_at: 0,
        }
    }
}
