//! [`GraphBuilder`] collects passes and resources before [`RenderGraph::compile`].

use crate::compiler::compile;
use crate::compiler::{CompilationConfig, DeviceCapabilities, ExecutionPlan, RenderGraphError};
use crate::types::{
    Capability, PassHandle, PassPriority, QueueAffinity, ResourceId, ResourceLifetime,
};

/// Pending streaming resource state for placeholder substitution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamingState {
    /// Asset not resident; compiler binds a placeholder.
    Pending,
    /// Resident; use declared resource.
    Ready,
}

#[derive(Clone, Debug)]
#[allow(dead_code)] // `streaming` is consumed by upcoming compile phases (placeholder binding).
pub(crate) struct PassDraft {
    pub(crate) name: String,
    pub(crate) reads: Vec<ResourceId>,
    pub(crate) writes: Vec<ResourceId>,
    pub(crate) requires: Vec<Capability>,
    pub(crate) fallback: Option<PassHandle>,
    pub(crate) queue: QueueAffinity,
    pub(crate) cost_ms: f32,
    pub(crate) priority: PassPriority,
    pub(crate) streaming: Option<(ResourceId, StreamingState)>,
}

/// Declarative render graph under construction.
#[derive(Debug)]
pub struct GraphBuilder {
    passes: Vec<PassDraft>,
    resources: Vec<ResourceLifetime>,
    next_res: u32,
    multi_view_cull: Option<PassHandle>,
    multi_view_draws: Vec<PassHandle>,
}

impl GraphBuilder {
    /// Empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            resources: Vec::new(),
            next_res: 0,
            multi_view_cull: None,
            multi_view_draws: Vec::new(),
        }
    }

    /// Declares a transient resource with lifetime bounds and byte size (for aliasing tests).
    pub fn create_transient(&mut self, first: u32, last: u32, size: u64) -> ResourceId {
        let id = ResourceId(self.next_res);
        self.next_res = self.next_res.saturating_add(1);
        self.resources.push(ResourceLifetime::Transient {
            first_pass: first,
            last_pass: last,
            size,
        });
        id
    }

    /// Declares a transient resource without explicit lifetime (filled later by tests via
    /// `create_transient` overload). Simplified: size-only, lifetime derived on compile from pass
    /// touches — for tests we use explicit overload below.
    pub fn add_transient_sized(&mut self, size: u64) -> ResourceId {
        let id = ResourceId(self.next_res);
        self.next_res = self.next_res.saturating_add(1);
        self.resources.push(ResourceLifetime::Transient {
            first_pass: u32::MAX,
            last_pass: 0,
            size,
        });
        id
    }

    /// Starts building a named pass.
    pub fn pass<'a>(&'a mut self, name: &str) -> PassBuilder<'a> {
        PassBuilder {
            gb: self,
            name: name.to_string(),
            reads: Vec::new(),
            writes: Vec::new(),
            requires: Vec::new(),
            fallback: None,
            queue: QueueAffinity::Graphics,
            cost_ms: 0.0,
            priority: PassPriority::Normal,
            streaming: None,
        }
    }

    /// Records a shared cull pass feeding multiple per-view draws (TC-2.2.7.1).
    pub fn set_multi_view_pattern(&mut self, cull: PassHandle, draws: &[PassHandle]) {
        self.multi_view_cull = Some(cull);
        self.multi_view_draws = draws.to_vec();
    }

    /// Seals the graph for compilation.
    pub fn build(self) -> Result<RenderGraph, RenderGraphError> {
        Ok(RenderGraph {
            passes: self.passes,
            resources: self.resources,
            multi_view_cull: self.multi_view_cull,
            multi_view_draws: self.multi_view_draws,
        })
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Fluent pass declaration.
pub struct PassBuilder<'a> {
    gb: &'a mut GraphBuilder,
    name: String,
    reads: Vec<ResourceId>,
    writes: Vec<ResourceId>,
    requires: Vec<Capability>,
    fallback: Option<PassHandle>,
    queue: QueueAffinity,
    cost_ms: f32,
    priority: PassPriority,
    streaming: Option<(ResourceId, StreamingState)>,
}

impl PassBuilder<'_> {
    /// Reads a resource in this pass.
    pub fn read(mut self, r: ResourceId) -> Self {
        self.reads.push(r);
        self
    }

    /// Writes a resource in this pass.
    pub fn write(mut self, r: ResourceId) -> Self {
        self.writes.push(r);
        self
    }

    /// Requires a capability (may prune or substitute fallback).
    pub fn require(mut self, c: Capability) -> Self {
        self.requires.push(c);
        self
    }

    /// Fallback pass when this pass is pruned.
    pub fn fallback(mut self, p: PassHandle) -> Self {
        self.fallback = Some(p);
        self
    }

    /// Preferred queue affinity.
    pub fn queue(mut self, q: QueueAffinity) -> Self {
        self.queue = q;
        self
    }

    /// Estimated GPU cost in milliseconds (budget culling).
    pub fn cost_ms(mut self, ms: f32) -> Self {
        self.cost_ms = ms;
        self
    }

    /// Priority for budget culling.
    pub fn priority(mut self, p: PassPriority) -> Self {
        self.priority = p;
        self
    }

    /// Streaming state for a sampled resource.
    pub fn streaming(mut self, r: ResourceId, st: StreamingState) -> Self {
        self.streaming = Some((r, st));
        self
    }

    /// Appends the pass and returns its handle.
    pub fn finish(self) -> PassHandle {
        let idx = self.gb.passes.len() as u32;
        self.gb.passes.push(PassDraft {
            name: self.name,
            reads: self.reads,
            writes: self.writes,
            requires: self.requires,
            fallback: self.fallback,
            queue: self.queue,
            cost_ms: self.cost_ms,
            priority: self.priority,
            streaming: self.streaming,
        });
        PassHandle(idx)
    }
}

/// Immutable render graph description ready for [`RenderGraph::compile`].
#[derive(Debug)]
pub struct RenderGraph {
    pub(crate) passes: Vec<PassDraft>,
    pub(crate) resources: Vec<ResourceLifetime>,
    pub(crate) multi_view_cull: Option<PassHandle>,
    pub(crate) multi_view_draws: Vec<PassHandle>,
}

impl RenderGraph {
    /// Compiles into an [`ExecutionPlan`].
    pub fn compile(
        &self,
        caps: &DeviceCapabilities,
        cfg: &CompilationConfig,
    ) -> Result<ExecutionPlan, RenderGraphError> {
        compile(self, caps, cfg)
    }
}
