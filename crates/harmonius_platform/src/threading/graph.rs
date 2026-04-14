//! DAG-shaped [`TaskGraph`] compilation and execution.

use std::collections::VecDeque;

use smallvec::SmallVec;

use super::pool::ThreadPool;

/// Stable identifier for a node in [`TaskGraphBuilder`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TaskNodeId(pub(crate) u32);

/// Scheduling hint stored on the builder (execution ignores it for now).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskPriority {
    /// Frame-critical work.
    High,
    /// Default priority.
    Normal,
    /// Background work.
    Low,
}

/// Executable graph node.
pub(crate) enum GraphNode {
    /// Leaf CPU task.
    Task(Box<dyn FnOnce() + Send>),
    /// Nested graph executed before dependents continue.
    Subgraph(TaskGraph),
}

/// Immutable task graph ready for [`ThreadPool::execute_graph`].
pub struct TaskGraph {
    pub(crate) nodes: Vec<Option<GraphNode>>,
    pub(crate) order: Vec<usize>,
}

/// Builder for [`TaskGraph`].
pub struct TaskGraphBuilder {
    nodes: Vec<BuilderNode>,
    edges: Vec<(TaskNodeId, TaskNodeId)>,
    priorities: Vec<(TaskNodeId, TaskPriority)>,
}

#[allow(dead_code)]
enum BuilderNode {
    Task {
        name: &'static str,
        task: Box<dyn FnOnce() + Send>,
    },
    Subgraph {
        name: &'static str,
        graph: TaskGraph,
    },
}

/// Validation errors returned by [`TaskGraphBuilder::build`].
#[derive(Debug, Eq, PartialEq)]
pub enum TaskGraphError {
    /// The graph contains a directed cycle.
    CycleDetected {
        /// Nodes participating in the cycle (best-effort trace; may be empty).
        cycle: Vec<TaskNodeId>,
    },
    /// An edge referenced an unknown node id.
    InvalidDependency {
        /// Dependency side of the edge.
        from: TaskNodeId,
        /// Dependent side of the edge.
        to: TaskNodeId,
    },
    /// No nodes were added before calling [`TaskGraphBuilder::build`].
    EmptyGraph,
}

impl TaskGraphBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            priorities: Vec::new(),
        }
    }

    /// Adds a leaf task.
    pub fn add_task<F>(&mut self, name: &'static str, f: F) -> TaskNodeId
    where
        F: FnOnce() + Send + 'static,
    {
        let id = TaskNodeId(self.nodes.len() as u32);
        self.nodes.push(BuilderNode::Task {
            name,
            task: Box::new(f),
        });
        id
    }

    /// Records that `dependent` waits on `dependency`.
    pub fn add_dependency(&mut self, dependency: TaskNodeId, dependent: TaskNodeId) {
        self.edges.push((dependency, dependent));
    }

    /// Embeds a compiled subgraph as a single node.
    pub fn add_subgraph(&mut self, name: &'static str, subgraph: TaskGraph) -> TaskNodeId {
        let id = TaskNodeId(self.nodes.len() as u32);
        self.nodes.push(BuilderNode::Subgraph {
            name,
            graph: subgraph,
        });
        id
    }

    /// Stores a priority hint for profiling hooks (does not change execution order yet).
    pub fn set_node_priority(&mut self, node: TaskNodeId, priority: TaskPriority) {
        self.priorities.push((node, priority));
    }

    /// Validates the DAG and freezes the graph.
    pub fn build(self) -> Result<TaskGraph, TaskGraphError> {
        let n = self.nodes.len();
        if n == 0 {
            return Err(TaskGraphError::EmptyGraph);
        }
        for &(from, to) in &self.edges {
            if from.0 as usize >= n || to.0 as usize >= n {
                return Err(TaskGraphError::InvalidDependency { from, to });
            }
            if from == to {
                return Err(TaskGraphError::InvalidDependency { from, to });
            }
        }
        let _ = self.priorities;

        let mut adj: Vec<SmallVec<[usize; 4]>> = vec![SmallVec::new(); n];
        let mut indeg = vec![0u32; n];
        for &(TaskNodeId(a), TaskNodeId(b)) in &self.edges {
            adj[a as usize].push(b as usize);
            indeg[b as usize] += 1;
        }

        let mut queue: VecDeque<usize> = indeg
            .iter()
            .enumerate()
            .filter_map(|(idx, &d)| if d == 0 { Some(idx) } else { None })
            .collect();
        let mut order = Vec::with_capacity(n);
        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &adj[u] {
                indeg[v] -= 1;
                if indeg[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        if order.len() != n {
            return Err(TaskGraphError::CycleDetected { cycle: Vec::new() });
        }

        let mut nodes_out: Vec<Option<GraphNode>> = Vec::with_capacity(n);
        for node in self.nodes {
            match node {
                BuilderNode::Task { task, .. } => {
                    nodes_out.push(Some(GraphNode::Task(task)));
                }
                BuilderNode::Subgraph { graph, .. } => {
                    nodes_out.push(Some(GraphNode::Subgraph(graph)));
                }
            }
        }

        Ok(TaskGraph {
            nodes: nodes_out,
            order,
        })
    }
}

impl Default for TaskGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskGraph {
    #[allow(clippy::only_used_in_recursion)]
    pub(crate) fn execute_on(self, pool: &ThreadPool) {
        let TaskGraph { mut nodes, order } = self;
        for idx in order {
            let node = nodes[idx]
                .take()
                .expect("topological order visits each node once");
            match node {
                GraphNode::Task(f) => f(),
                GraphNode::Subgraph(g) => g.execute_on(pool),
            }
        }
    }
}
