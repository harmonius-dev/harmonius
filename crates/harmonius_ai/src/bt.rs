//! Behavior tree asset, runtime state, and deterministic ticking.

use crate::debug_trace::TraceLog;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Unique index into [`BehaviorTreeAsset::nodes`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u16);

/// Registered leaf type id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LeafId(pub u32);

/// Result of ticking a node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    /// Not visited this session.
    Idle,
    /// In progress across frames.
    Running,
    /// Terminal success.
    Success,
    /// Terminal failure.
    Failure,
}

/// Abort semantics on composites.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbortMode {
    /// No conditional abort.
    None,
    /// Re-evaluate higher-priority children (index 0) while lower runs.
    SelfAbort,
    /// Higher-priority siblings may preempt lower branches.
    LowerPriority,
    /// Both modes active.
    Both,
}

/// Parallel completion policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParallelPolicy {
    /// All children must match.
    RequireAll,
    /// Any child may match.
    RequireOne,
}

/// Decorator variants.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DecoratorKind {
    /// Invert terminal status; [`NodeStatus::Running`] passes through.
    Inverter,
    /// Repeat child `count` successes; 0 = infinite until failure.
    Repeater {
        /// Success repetitions required.
        count: u32,
    },
    /// Always [`NodeStatus::Success`].
    Succeeder,
    /// Limit child ticks to `hz` frequency using [`BtTickContext::time_secs`].
    RateLimiter {
        /// Ticks per second budget for the child.
        hz: f32,
    },
    /// Block child until `duration_secs` elapse after a successful child tick.
    Cooldown {
        /// Seconds the child stays blocked after success.
        duration_secs: f32,
    },
}

/// Sequence / selector shared payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompositeData {
    /// Child node ids in priority order (lower index = higher priority).
    pub children: Vec<NodeId>,
    /// Abort configuration.
    pub abort_mode: AbortMode,
}

/// Parallel node payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParallelData {
    /// Children evaluated each tick.
    pub children: Vec<NodeId>,
    /// Success aggregation.
    pub success_policy: ParallelPolicy,
    /// Failure aggregation.
    pub failure_policy: ParallelPolicy,
}

/// Decorator payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecoratorData {
    /// Wrapped child.
    pub child: NodeId,
    /// Decorator kind + parameters.
    pub kind: DecoratorKind,
}

/// Subtree asset reference (opaque id for serialization tests).
pub type SubtreeAssetId = u32;

/// One BT node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BtNode {
    /// Ordered AND.
    Sequence(CompositeData),
    /// Ordered OR.
    Selector(CompositeData),
    /// Concurrent children.
    Parallel(ParallelData),
    /// Single-child wrapper.
    Decorator(DecoratorData),
    /// Leaf invocation.
    Leaf(LeafId),
    /// Reference to another serialized asset.
    SubtreeRef(SubtreeAssetId),
}

/// Declared blackboard keys (placeholder for load-time validation).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BlackboardSchema {}

/// Serialized tree asset.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BehaviorTreeAsset {
    /// Entry node.
    pub root: NodeId,
    /// Flat node table indexed by [`NodeId::0`].
    pub nodes: Vec<BtNode>,
    /// Parallel human-readable labels for [`TraceLog`] rows.
    pub labels: Vec<String>,
    /// Schema hook.
    pub blackboard_schema: BlackboardSchema,
}

impl BehaviorTreeAsset {
    /// Ensures `labels.len() == nodes.len()` by padding empty strings.
    pub fn normalize_labels(&mut self) {
        while self.labels.len() < self.nodes.len() {
            self.labels.push(String::new());
        }
    }

    fn label(&self, id: NodeId) -> String {
        self.labels.get(id.0 as usize).cloned().unwrap_or_default()
    }
}

/// Runtime data for decorators that need timers or counters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecoratorRuntime {
    /// Cooldown: child may run only when `time >= ready_at`.
    Cooldown {
        /// Next eligible time (seconds).
        ready_at: f64,
    },
    /// Rate limiter: accumulated simulation seconds since last child tick.
    RateLimiter {
        /// Integrator for leftover time.
        acc: f32,
    },
}

/// Persistent tick state for one agent + asset pair.
#[derive(Clone, Debug, PartialEq)]
pub struct BtTickState {
    /// Active child per composite/parallel node (`None` when idle at composite root).
    pub active_child: Vec<Option<u8>>,
    /// Last published status per node (for abort reset visibility).
    pub node_status: Vec<NodeStatus>,
    /// Cooldown / rate state keyed by decorator [`NodeId`].
    pub decorator_runtime: BTreeMap<u16, DecoratorRuntime>,
}

impl BtTickState {
    /// Allocates state vectors sized to `node_count`.
    pub fn new(node_count: usize) -> Self {
        Self {
            active_child: vec![None; node_count],
            node_status: vec![NodeStatus::Idle; node_count],
            decorator_runtime: BTreeMap::new(),
        }
    }

    /// Clears active indices and statuses for every node in the subtree rooted at `root`.
    pub fn reset_subtree(&mut self, asset: &BehaviorTreeAsset, root: NodeId) {
        let mut stack = vec![root.0 as usize];
        while let Some(i) = stack.pop() {
            if i >= asset.nodes.len() {
                continue;
            }
            if let Some(ac) = self.active_child.get_mut(i) {
                *ac = None;
            }
            if let Some(ns) = self.node_status.get_mut(i) {
                *ns = NodeStatus::Idle;
            }
            match &asset.nodes[i] {
                BtNode::Sequence(c) | BtNode::Selector(c) => {
                    for ch in &c.children {
                        stack.push(ch.0 as usize);
                    }
                }
                BtNode::Parallel(p) => {
                    for ch in &p.children {
                        stack.push(ch.0 as usize);
                    }
                }
                BtNode::Decorator(d) => stack.push(d.child.0 as usize),
                BtNode::Leaf(_) | BtNode::SubtreeRef(_) => {}
            }
        }
    }
}

/// Host callbacks for a single tick.
pub struct BtTickContext<'a> {
    /// Global time in seconds (monotonic in tests).
    pub time_secs: f64,
    /// Simulation step length in seconds (used by [`DecoratorKind::RateLimiter`]).
    pub dt_secs: f32,
    /// Leaf resolver.
    pub leaf: &'a mut dyn FnMut(LeafId) -> NodeStatus,
    /// Optional trace sink.
    pub trace: Option<&'a mut TraceLog>,
    /// Invoked after an abort-driven reset (self or lower priority).
    pub on_abort: Option<&'a mut dyn FnMut()>,
}

impl<'a> BtTickContext<'a> {
    /// Builds a tick context without tracing or abort hooks.
    pub fn new(time_secs: f64, leaf: &'a mut dyn FnMut(LeafId) -> NodeStatus) -> Self {
        Self {
            time_secs,
            dt_secs: 0.1,
            leaf,
            trace: None,
            on_abort: None,
        }
    }
}

fn record_trace(
    ctx: &mut BtTickContext<'_>,
    asset: &BehaviorTreeAsset,
    id: NodeId,
    st: NodeStatus,
) {
    if let Some(t) = ctx.trace.as_mut() {
        t.push(asset.label(id), st);
    }
}

fn record_trace_front(
    ctx: &mut BtTickContext<'_>,
    asset: &BehaviorTreeAsset,
    id: NodeId,
    st: NodeStatus,
) {
    if let Some(t) = ctx.trace.as_mut() {
        t.push_front(asset.label(id), st);
    }
}

fn set_status(st: &mut BtTickState, id: NodeId, s: NodeStatus) {
    if let Some(ns) = st.node_status.get_mut(id.0 as usize) {
        *ns = s;
    }
}

/// Ticks `asset` once, mutating `state`.
pub fn tick_tree(
    asset: &BehaviorTreeAsset,
    state: &mut BtTickState,
    ctx: &mut BtTickContext<'_>,
) -> NodeStatus {
    tick_node(asset, state, ctx, asset.root)
}

fn tick_node(
    asset: &BehaviorTreeAsset,
    state: &mut BtTickState,
    ctx: &mut BtTickContext<'_>,
    id: NodeId,
) -> NodeStatus {
    let idx = id.0 as usize;
    if idx >= asset.nodes.len() {
        return NodeStatus::Failure;
    }
    let out = match &asset.nodes[idx] {
        BtNode::Sequence(c) => tick_sequence(asset, state, ctx, id, c),
        BtNode::Selector(c) => tick_selector(asset, state, ctx, id, c),
        BtNode::Parallel(p) => tick_parallel(asset, state, ctx, id, p),
        BtNode::Decorator(d) => tick_decorator(asset, state, ctx, id, d),
        BtNode::Leaf(lid) => {
            let s = (ctx.leaf)(*lid);
            set_status(state, id, s);
            record_trace(ctx, asset, id, s);
            s
        }
        BtNode::SubtreeRef(_) => {
            set_status(state, id, NodeStatus::Success);
            NodeStatus::Success
        }
    };
    set_status(state, id, out);
    out
}

fn tick_sequence(
    asset: &BehaviorTreeAsset,
    state: &mut BtTickState,
    ctx: &mut BtTickContext<'_>,
    id: NodeId,
    c: &CompositeData,
) -> NodeStatus {
    let idx = id.0 as usize;
    let self_abort = matches!(c.abort_mode, AbortMode::SelfAbort | AbortMode::Both);
    let mut i = state.active_child[idx].unwrap_or(0) as usize;

    if self_abort && i > 0 {
        let guard = c.children[0];
        let g = tick_node(asset, state, ctx, guard);
        if g == NodeStatus::Failure {
            state.reset_subtree(asset, c.children[i]);
            state.active_child[idx] = None;
            if let Some(cb) = ctx.on_abort.as_mut() {
                (**cb)();
            }
            let out = NodeStatus::Failure;
            record_trace_front(ctx, asset, id, out);
            return out;
        }
    }

    while i < c.children.len() {
        let ch = c.children[i];
        let s = tick_node(asset, state, ctx, ch);
        match s {
            NodeStatus::Failure => {
                state.active_child[idx] = None;
                let out = NodeStatus::Failure;
                record_trace_front(ctx, asset, id, out);
                return out;
            }
            NodeStatus::Running => {
                state.active_child[idx] = Some(i as u8);
                let out = NodeStatus::Running;
                record_trace_front(ctx, asset, id, out);
                return out;
            }
            NodeStatus::Success => {
                i += 1;
            }
            NodeStatus::Idle => {
                i += 1;
            }
        }
    }
    state.active_child[idx] = None;
    let out = NodeStatus::Success;
    record_trace_front(ctx, asset, id, out);
    out
}

fn tick_selector(
    asset: &BehaviorTreeAsset,
    state: &mut BtTickState,
    ctx: &mut BtTickContext<'_>,
    id: NodeId,
    c: &CompositeData,
) -> NodeStatus {
    let idx = id.0 as usize;
    let lower = matches!(c.abort_mode, AbortMode::LowerPriority | AbortMode::Both);
    let active = state.active_child[idx];

    if lower {
        if let Some(run) = active {
            let run = run as usize;
            if run > 0 {
                for j in 0..run {
                    let ch = c.children[j];
                    let s = tick_node(asset, state, ctx, ch);
                    if s == NodeStatus::Success {
                        state.reset_subtree(asset, c.children[run]);
                        state.active_child[idx] = None;
                        if let Some(cb) = ctx.on_abort.as_mut() {
                            (**cb)();
                        }
                        let out = NodeStatus::Success;
                        record_trace_front(ctx, asset, id, out);
                        return out;
                    }
                }
            }
        }
    }

    let mut start = active.map(|v| v as usize).unwrap_or(0);
    if start >= c.children.len() {
        start = 0;
    }

    for off in 0..c.children.len() {
        let i = (start + off) % c.children.len();
        let ch = c.children[i];
        let s = tick_node(asset, state, ctx, ch);
        match s {
            NodeStatus::Success => {
                state.active_child[idx] = None;
                let out = NodeStatus::Success;
                record_trace_front(ctx, asset, id, out);
                return out;
            }
            NodeStatus::Running => {
                state.active_child[idx] = Some(i as u8);
                let out = NodeStatus::Running;
                record_trace_front(ctx, asset, id, out);
                return out;
            }
            NodeStatus::Failure | NodeStatus::Idle => {}
        }
    }
    state.active_child[idx] = None;
    let out = NodeStatus::Failure;
    record_trace_front(ctx, asset, id, out);
    out
}

fn tick_parallel(
    asset: &BehaviorTreeAsset,
    state: &mut BtTickState,
    ctx: &mut BtTickContext<'_>,
    id: NodeId,
    p: &ParallelData,
) -> NodeStatus {
    let mut statuses = Vec::new();
    for ch in &p.children {
        statuses.push(tick_node(asset, state, ctx, *ch));
    }

    let out = match p.success_policy {
        ParallelPolicy::RequireAll => {
            if statuses.contains(&NodeStatus::Failure) {
                NodeStatus::Failure
            } else if statuses.iter().all(|s| *s == NodeStatus::Success) {
                NodeStatus::Success
            } else {
                NodeStatus::Running
            }
        }
        ParallelPolicy::RequireOne => {
            if statuses.contains(&NodeStatus::Success) {
                NodeStatus::Success
            } else if statuses.iter().all(|s| *s == NodeStatus::Failure) {
                NodeStatus::Failure
            } else {
                NodeStatus::Running
            }
        }
    };
    let _ = p.failure_policy;
    record_trace_front(ctx, asset, id, out);
    out
}

fn tick_decorator(
    asset: &BehaviorTreeAsset,
    state: &mut BtTickState,
    ctx: &mut BtTickContext<'_>,
    id: NodeId,
    d: &DecoratorData,
) -> NodeStatus {
    let key = id.0;
    match d.kind {
        DecoratorKind::Inverter => {
            let s = tick_node(asset, state, ctx, d.child);
            match s {
                NodeStatus::Success => NodeStatus::Failure,
                NodeStatus::Failure => NodeStatus::Success,
                other => other,
            }
        }
        DecoratorKind::Repeater { count } => {
            let mut done = 0u32;
            loop {
                let s = tick_node(asset, state, ctx, d.child);
                if s == NodeStatus::Failure {
                    return NodeStatus::Failure;
                }
                if s == NodeStatus::Running {
                    return NodeStatus::Running;
                }
                if s == NodeStatus::Success {
                    done += 1;
                    if count == 0 {
                        return NodeStatus::Running;
                    }
                    if done >= count {
                        return NodeStatus::Success;
                    }
                } else {
                    break;
                }
            }
            NodeStatus::Success
        }
        DecoratorKind::Succeeder => {
            let _ = tick_node(asset, state, ctx, d.child);
            NodeStatus::Success
        }
        DecoratorKind::RateLimiter { hz } => {
            let rt = state
                .decorator_runtime
                .entry(key)
                .or_insert(DecoratorRuntime::RateLimiter { acc: 0.0 });
            if let DecoratorRuntime::RateLimiter { ref mut acc } = rt {
                *acc += ctx.dt_secs;
                let period = 1.0 / hz.max(1e-6);
                let mut ticks = 0u32;
                while *acc >= period - 1e-9 {
                    *acc -= period;
                    ticks += 1;
                }
                for _ in 0..ticks {
                    let _ = tick_node(asset, state, ctx, d.child);
                }
            }
            NodeStatus::Success
        }
        DecoratorKind::Cooldown { duration_secs } => {
            let ready_at = match state.decorator_runtime.get(&key) {
                Some(DecoratorRuntime::Cooldown { ready_at }) => *ready_at,
                _ => 0.0,
            };
            if ctx.time_secs < ready_at {
                return NodeStatus::Failure;
            }
            let s = tick_node(asset, state, ctx, d.child);
            if s == NodeStatus::Success {
                state.decorator_runtime.insert(
                    key,
                    DecoratorRuntime::Cooldown {
                        ready_at: ctx.time_secs + f64::from(duration_secs),
                    },
                );
            }
            s
        }
    }
}
