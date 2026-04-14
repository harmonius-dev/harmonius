//! DAG-shaped [`TaskGraph`] compilation and execution.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use smallvec::SmallVec;

use super::pool::{PoolShared, ThreadPool};

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

/// Shared execution context for [`TaskGraph`] runs on a [`ThreadPool`].
struct GraphExec {
    nodes: Arc<Vec<Mutex<Option<GraphNode>>>>,
    dependents: Arc<Vec<Vec<usize>>>,
    rem: Arc<Vec<AtomicU32>>,
    left: Arc<AtomicUsize>,
    done: crossbeam_channel::Sender<()>,
    shared: Arc<PoolShared>,
}

fn run_node(ctx: Arc<GraphExec>, idx: usize) {
    let node = {
        let mut slot = ctx.nodes[idx].lock().expect("graph node mutex poisoned");
        slot.take()
    };
    let Some(node) = node else {
        return;
    };
    match node {
        GraphNode::Task(f) => f(),
        GraphNode::Subgraph(g) => {
            // Nested graphs run sequentially on the calling thread so a single worker cannot
            // deadlock waiting for pool capacity while holding a worker slot.
            g.execute_sequential();
        }
    }
    for &v in ctx.dependents[idx].iter() {
        if ctx.rem[v].fetch_sub(1, Ordering::AcqRel) == 1 {
            let ctx2 = Arc::clone(&ctx);
            ctx.shared.submit_job(Box::new(move || run_node(ctx2, v)));
        }
    }
    if ctx.left.fetch_sub(1, Ordering::AcqRel) == 1 {
        let _ = ctx.done.send(());
    }
}

fn execute_parallel_graph(shared: Arc<PoolShared>, graph: TaskGraph) {
    let n = graph.nodes.len();
    let (done_tx, done_rx) = crossbeam_channel::bounded(1);
    let nodes = Arc::new(graph.nodes.into_iter().map(Mutex::new).collect::<Vec<_>>());
    let dependents = Arc::new(graph.dependents);
    let rem: Arc<Vec<AtomicU32>> = Arc::new(
        graph
            .initial_indegree
            .iter()
            .map(|&d| AtomicU32::new(d))
            .collect(),
    );
    let left = Arc::new(AtomicUsize::new(n));
    let ctx = Arc::new(GraphExec {
        nodes,
        dependents,
        rem,
        left,
        done: done_tx,
        shared: Arc::clone(&shared),
    });
    for i in 0..n {
        if ctx.rem[i].load(Ordering::Acquire) == 0 {
            let ctx2 = Arc::clone(&ctx);
            shared.submit_job(Box::new(move || run_node(ctx2, i)));
        }
    }
    let _ = done_rx.recv();
}

/// Immutable task graph ready for [`ThreadPool::execute_graph`].
pub struct TaskGraph {
    pub(crate) nodes: Vec<Option<GraphNode>>,
    /// Topological visit order for sequential subgraph execution.
    pub(crate) order: Vec<usize>,
    /// For each node `u`, dependents `v` with edge `(u, v)`.
    pub(crate) dependents: Vec<Vec<usize>>,
    /// Indegree snapshot after validation (before Kahn drains counts).
    pub(crate) initial_indegree: Vec<u32>,
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
        let initial_indegree = indeg.clone();

        let mut dependents = vec![Vec::new(); n];
        for &(TaskNodeId(a), TaskNodeId(b)) in &self.edges {
            dependents[a as usize].push(b as usize);
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
            dependents,
            initial_indegree,
        })
    }
}

impl Default for TaskGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskGraph {
    /// Runs this graph on the calling thread without nested parallel scheduling (avoids pool
    /// deadlocks when worker count is low).
    fn execute_sequential(self) {
        let TaskGraph {
            mut nodes, order, ..
        } = self;
        for idx in order {
            let node = nodes[idx]
                .take()
                .expect("topological order visits each node once");
            match node {
                GraphNode::Task(f) => f(),
                GraphNode::Subgraph(g) => g.execute_sequential(),
            }
        }
    }

    pub(crate) fn execute_on(self, pool: &ThreadPool) {
        execute_parallel_graph(pool.shared(), self);
    }
}
