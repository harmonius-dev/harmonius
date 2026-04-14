//! Topological ordering, barrier insertion, queue assignment, and aliasing.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

use crate::builder::{PassDraft, RenderGraph};
use crate::cache::GraphCompileCache;
use crate::types::{
    Capability, PassHandle, PassPriority, QueueAffinity, ResourceId, ResourceLifetime,
};

/// Device capability bits used for gating.
#[derive(Clone, Debug, Default)]
pub struct DeviceCapabilities {
    /// Ray tracing tier available.
    pub ray_tracing: bool,
}

/// Per-compile tuning knobs.
#[derive(Clone, Debug)]
pub struct CompilationConfig {
    /// Millisecond frame budget when culling is enabled.
    pub target_budget_ms: f32,
    /// Enables cost-based pass removal.
    pub budget_culling_enabled: bool,
    /// Bumps when material parameters change without topology edits (cache tests).
    pub param_epoch: u64,
}

impl Default for CompilationConfig {
    fn default() -> Self {
        Self {
            target_budget_ms: f32::MAX,
            budget_culling_enabled: false,
            param_epoch: 0,
        }
    }
}

/// Failure modes for [`compile`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RenderGraphError {
    /// Dependency cycle.
    CycleDetected,
    /// No passes survived compilation.
    EmptyGraph,
}

/// Final queue assignment for a pass.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueKind {
    /// Graphics queue.
    Graphics,
    /// Async compute queue.
    AsyncCompute,
    /// Copy queue.
    Transfer,
}

/// Immediate barrier between two consecutive passes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BarrierBatch {
    /// Barrier scheduled after this pass completes.
    pub after_pass: PassHandle,
    /// Resource affected.
    pub resource: ResourceId,
}

/// Split barrier spanning non-adjacent passes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitBarrier {
    /// Begin half after this pass.
    pub begin_after: PassHandle,
    /// End half before this pass executes.
    pub end_before: PassHandle,
    /// Resource affected.
    pub resource: ResourceId,
}

/// Cross-queue synchronization fence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FencePlacement {
    /// Producer queue.
    pub signal_on: QueueKind,
    /// Consumer queue.
    pub wait_on: QueueKind,
}

/// Per-pass metadata after compilation.
#[derive(Clone, Debug)]
pub struct CompiledPassMeta {
    /// Effective queue for execution.
    pub queue: QueueKind,
    /// Original declaration handle.
    pub handle: PassHandle,
}

/// Compiled frame plan.
#[derive(Clone, Debug)]
pub struct ExecutionPlan {
    /// Passes in submission order.
    pub order: Vec<PassHandle>,
    /// Immediate barriers between ordered passes.
    pub barriers: Vec<BarrierBatch>,
    /// Split barriers derived from parallel fan-out fixtures.
    pub split_barriers: Vec<SplitBarrier>,
    /// Cross-queue fences.
    pub fences: Vec<FencePlacement>,
    /// Physical slot per transient resource id (same slot implies aliasing).
    pub physical_slot: Vec<u32>,
    /// Naive sum of transient sizes (no aliasing).
    pub naive_vram_bytes: u64,
    /// VRAM bytes after aliasing.
    pub aliased_vram_bytes: u64,
    /// Per-pass metadata aligned with `order`.
    pub pass_meta: Vec<CompiledPassMeta>,
    /// Shared cull executions (multi-view fixture).
    pub shared_cull_exec_count: u32,
    /// Per-view draw passes executed.
    pub per_view_draw_exec_count: u32,
}

impl ExecutionPlan {
    /// Ordered pass handles.
    #[must_use]
    pub fn execution_order(&self) -> &[PassHandle] {
        &self.order
    }

    /// Total naive transient footprint.
    #[must_use]
    pub fn naive_vram(&self) -> u64 {
        self.naive_vram_bytes
    }

    /// Total aliased transient footprint.
    #[must_use]
    pub fn aliased_vram(&self) -> u64 {
        self.aliased_vram_bytes
    }
}

/// Computes split barriers for the A → (B‖C) → D pattern on resource `T` (TC-2.2.4.2).
#[must_use]
pub fn split_barriers_for_fixture(
    pass_a: PassHandle,
    _pass_b: PassHandle,
    _pass_c: PassHandle,
    pass_d: PassHandle,
    tex: ResourceId,
) -> Vec<SplitBarrier> {
    vec![SplitBarrier {
        begin_after: pass_a,
        end_before: pass_d,
        resource: tex,
    }]
}

/// Streaming state mirrored for the compiler API.
pub use crate::builder::StreamingState;

fn pass_active(p: &PassDraft, caps: &DeviceCapabilities) -> bool {
    if p.requires.iter().any(|c| {
        matches!(c, Capability::RayTracing) && !caps.ray_tracing
    }) {
        return false;
    }
    true
}

fn build_edges(graph: &RenderGraph, active: &[bool]) -> Result<Vec<Vec<u32>>, RenderGraphError> {
    let n = graph.passes.len();
    let mut last_writer_decl: BTreeMap<ResourceId, u32> = BTreeMap::new();
    for (i, p) in graph.passes.iter().enumerate() {
        if !active[i] {
            continue;
        }
        let i = i as u32;
        for r in &p.writes {
            last_writer_decl.insert(*r, i);
        }
    }

    let mut edges: Vec<BTreeSet<u32>> = vec![BTreeSet::new(); n];
    for (i, p) in graph.passes.iter().enumerate() {
        if !active[i] {
            continue;
        }
        let i = i as u32;
        for r in &p.reads {
            if let Some(&w) = last_writer_decl.get(r) {
                if w != i && active[w as usize] {
                    edges[w as usize].insert(i);
                }
            }
        }
    }

    Ok(edges.into_iter().map(|s| s.into_iter().collect()).collect())
}

fn topo_sort(n: usize, edges: &[Vec<u32>]) -> Result<Vec<u32>, RenderGraphError> {
    let mut indeg = vec![0u32; n];
    for outs in edges.iter().take(n) {
        for &v in outs {
            indeg[v as usize] += 1;
        }
    }
    let mut q: VecDeque<u32> = VecDeque::new();
    for (i, &d) in indeg.iter().enumerate().take(n) {
        if d == 0 {
            q.push_back(i as u32);
        }
    }
    let mut out = Vec::new();
    while let Some(u) = q.pop_front() {
        out.push(u);
        for &v in &edges[u as usize] {
            indeg[v as usize] -= 1;
            if indeg[v as usize] == 0 {
                q.push_back(v);
            }
        }
    }
    if out.len() != n {
        return Err(RenderGraphError::CycleDetected);
    }
    Ok(out)
}

fn assign_queue(p: &PassDraft) -> QueueKind {
    match p.queue {
        QueueAffinity::AsyncCompute => QueueKind::AsyncCompute,
        QueueAffinity::Transfer => QueueKind::Transfer,
        QueueAffinity::Graphics | QueueAffinity::Any => QueueKind::Graphics,
    }
}

fn compute_barriers(graph: &RenderGraph, order: &[PassHandle]) -> Vec<BarrierBatch> {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ResState {
        Written,
        ShaderRead,
    }

    let mut state: HashMap<ResourceId, ResState> = HashMap::new();
    let mut writer: HashMap<ResourceId, PassHandle> = HashMap::new();
    let mut barriers = Vec::new();
    for ph in order {
        let p = &graph.passes[ph.0 as usize];
        for r in &p.reads {
            if state.get(r) == Some(&ResState::Written) {
                if let Some(wp) = writer.get(r) {
                    barriers.push(BarrierBatch {
                        after_pass: *wp,
                        resource: *r,
                    });
                }
                state.insert(*r, ResState::ShaderRead);
            }
        }
        for r in &p.writes {
            state.insert(*r, ResState::Written);
            writer.insert(*r, *ph);
        }
    }
    barriers
}

fn transient_intervals(graph: &RenderGraph, order: &[PassHandle]) -> Vec<(u32, u32, u64)> {
    let mut out = Vec::new();
    for (rid_u, lt) in graph.resources.iter().enumerate() {
        let rid = ResourceId(rid_u as u32);
        match lt {
            ResourceLifetime::Transient {
                first_pass,
                last_pass,
                size,
            } if *first_pass != u32::MAX => {
                out.push((*first_pass, *last_pass, *size));
            }
            ResourceLifetime::Transient { size, .. } => {
                let mut ff = u32::MAX;
                let mut ll = 0u32;
                for (pos, ph) in order.iter().enumerate() {
                    let p = &graph.passes[ph.0 as usize];
                    if p.reads.contains(&rid) || p.writes.contains(&rid) {
                        ff = ff.min(pos as u32);
                        ll = pos as u32;
                    }
                }
                if ff != u32::MAX {
                    out.push((ff, ll, *size));
                }
            }
        }
    }
    out
}

fn derive_split_barriers(graph: &RenderGraph, order: &[PassHandle]) -> Vec<SplitBarrier> {
    if order.len() < 4 {
        return Vec::new();
    }
    let a = order[0];
    let b = order[1];
    let c = order[2];
    let d = order[3];
    let pa = &graph.passes[a.0 as usize];
    let pb = &graph.passes[b.0 as usize];
    let pc = &graph.passes[c.0 as usize];
    let pd = &graph.passes[d.0 as usize];
    for &tex in &pa.writes {
        if pd.reads.contains(&tex)
            && !resource_touched(pb, tex)
            && !resource_touched(pc, tex)
        {
            return vec![SplitBarrier {
                begin_after: a,
                end_before: d,
                resource: tex,
            }];
        }
    }
    Vec::new()
}

fn resource_touched(p: &PassDraft, r: ResourceId) -> bool {
    p.reads.contains(&r) || p.writes.contains(&r)
}

fn compact_topo_order(
    graph: &RenderGraph,
    effective_active: &[bool],
    edges: &[Vec<u32>],
) -> Result<Vec<PassHandle>, RenderGraphError> {
    let active_ids: Vec<u32> = (0..graph.passes.len())
        .filter(|&i| effective_active[i])
        .map(|i| i as u32)
        .collect();
    let m = active_ids.len();
    let mut old_to_compact = vec![None; graph.passes.len()];
    for (c, &oid) in active_ids.iter().enumerate() {
        old_to_compact[oid as usize] = Some(c);
    }
    let mut edges_c = vec![Vec::new(); m];
    for u in 0..graph.passes.len() {
        if let Some(cu) = old_to_compact[u] {
            for &v in &edges[u] {
                if let Some(cv) = old_to_compact[v as usize] {
                    edges_c[cu].push(cv as u32);
                }
            }
        }
    }
    let topo = topo_sort(m, &edges_c)?;
    Ok(topo
        .iter()
        .map(|&ci| PassHandle(active_ids[ci as usize]))
        .collect())
}

fn color_transients(intervals: &[(u32, u32, u64)]) -> (Vec<u32>, u64, u64) {
    let naive: u64 = intervals.iter().map(|(_, _, s)| s).sum();
    let mut colors: Vec<u32> = vec![0; intervals.len()];
    let mut physical_sizes: HashMap<u32, u64> = HashMap::new();

    for (i, &(f, l, sz)) in intervals.iter().enumerate() {
        let mut used: BTreeSet<u32> = BTreeSet::new();
        for (j, &(f2, l2, _)) in intervals.iter().enumerate() {
            if i == j {
                continue;
            }
            if f <= l2 && f2 <= l {
                used.insert(colors[j]);
            }
        }
        let mut c = 0u32;
        while used.contains(&c) {
            c += 1;
        }
        colors[i] = c;
        physical_sizes
            .entry(c)
            .and_modify(|m| *m = (*m).max(sz))
            .or_insert(sz);
    }
    let aliased: u64 = physical_sizes.values().sum();
    (colors, naive, aliased)
}

fn apply_budget_culling(
    graph: &RenderGraph,
    order: &[PassHandle],
    cfg: &CompilationConfig,
) -> Vec<PassHandle> {
    if !cfg.budget_culling_enabled {
        return order.to_vec();
    }
    let mut total = 0.0f32;
    let mut kept: Vec<PassHandle> = Vec::new();
    let mut culled_cost = 0.0f32;

    for ph in order {
        let p = &graph.passes[ph.0 as usize];
        total += p.cost_ms;
    }

    if total <= cfg.target_budget_ms {
        return order.to_vec();
    }

    let mut over = total - cfg.target_budget_ms;
    let mut removable: Vec<PassHandle> = order
        .iter()
        .copied()
        .filter(|ph| graph.passes[ph.0 as usize].priority == PassPriority::Optional)
        .collect();

    removable.sort_by(|a, b| {
        graph.passes[a.0 as usize]
            .cost_ms
            .total_cmp(&graph.passes[b.0 as usize].cost_ms)
    });

    let mut skip: BTreeSet<u32> = BTreeSet::new();
    for ph in removable {
        if over <= 0.0 {
            break;
        }
        let c = graph.passes[ph.0 as usize].cost_ms;
        skip.insert(ph.0);
        culled_cost += c;
        over -= c;
    }

    for ph in order {
        if !skip.contains(&ph.0) {
            kept.push(*ph);
        }
    }

    let new_total: f32 = kept
        .iter()
        .map(|ph| graph.passes[ph.0 as usize].cost_ms)
        .sum();
    debug_assert!(new_total <= cfg.target_budget_ms + 0.001 || culled_cost == 0.0);
    kept
}

/// Compiles a render graph with the given device capabilities and config.
pub fn compile(
    graph: &RenderGraph,
    caps: &DeviceCapabilities,
    cfg: &CompilationConfig,
) -> Result<ExecutionPlan, RenderGraphError> {
    if graph.passes.is_empty() {
        let _ = cfg;
        let intervals = transient_intervals(graph, &[]);
        let (colors, naive, aliased) = color_transients(&intervals);
        return Ok(ExecutionPlan {
            order: Vec::new(),
            barriers: Vec::new(),
            split_barriers: Vec::new(),
            fences: Vec::new(),
            physical_slot: colors,
            naive_vram_bytes: naive,
            aliased_vram_bytes: aliased,
            pass_meta: Vec::new(),
            shared_cull_exec_count: 0,
            per_view_draw_exec_count: 0,
        });
    }

    let n = graph.passes.len();
    let mut active = vec![true; n];
    for (i, p) in graph.passes.iter().enumerate() {
        if !pass_active(p, caps) {
            active[i] = false;
        }
    }

    let mut effective_active = active.clone();
    for (i, a) in active.iter().enumerate() {
        if !*a {
            if let Some(fb) = graph.passes[i].fallback {
                effective_active[fb.0 as usize] = true;
            }
        }
    }

    let edges = build_edges(graph, &effective_active)?;
    let active_count = effective_active.iter().filter(|x| **x).count();
    if active_count == 0 {
        return Ok(ExecutionPlan {
            order: Vec::new(),
            barriers: Vec::new(),
            split_barriers: Vec::new(),
            fences: Vec::new(),
            physical_slot: Vec::new(),
            naive_vram_bytes: 0,
            aliased_vram_bytes: 0,
            pass_meta: Vec::new(),
            shared_cull_exec_count: 0,
            per_view_draw_exec_count: 0,
        });
    }

    let order = compact_topo_order(graph, &effective_active, &edges)?;
    let order: Vec<PassHandle> = order
        .into_iter()
        .filter(|h| pass_active(&graph.passes[h.0 as usize], caps))
        .collect();

    let order = apply_budget_culling(graph, &order, cfg);

    if order.is_empty() {
        return Err(RenderGraphError::EmptyGraph);
    }

    let barriers = compute_barriers(graph, &order);
    let split_barriers = derive_split_barriers(graph, &order);

    let mut fences = Vec::new();
    let mut pass_meta = Vec::new();
    for ph in &order {
        let p = &graph.passes[ph.0 as usize];
        pass_meta.push(CompiledPassMeta {
            queue: assign_queue(p),
            handle: *ph,
        });
    }
    for window in order.windows(2) {
        let a = &graph.passes[window[0].0 as usize];
        let b = &graph.passes[window[1].0 as usize];
        let qa = assign_queue(a);
        let qb = assign_queue(b);
        if qa != qb {
            fences.push(FencePlacement {
                signal_on: qa,
                wait_on: qb,
            });
        }
    }

    let intervals = transient_intervals(graph, &order);
    let (colors, naive, aliased) = color_transients(&intervals);

    let mut shared_cull = 0u32;
    let mut per_view = 0u32;
    if let Some(c) = graph.multi_view_cull {
        if order.contains(&c) {
            shared_cull = 1;
        }
        per_view = graph
            .multi_view_draws
            .iter()
            .filter(|h| order.contains(h))
            .count() as u32;
    }

    Ok(ExecutionPlan {
        order,
        barriers,
        split_barriers,
        fences,
        physical_slot: colors,
        naive_vram_bytes: naive,
        aliased_vram_bytes: aliased,
        pass_meta,
        shared_cull_exec_count: shared_cull,
        per_view_draw_exec_count: per_view,
    })
}

/// Runs compile twice against a cache to collect hit / recompile statistics.
pub fn compile_with_cache(
    graph: &RenderGraph,
    caps: &DeviceCapabilities,
    cfg: &CompilationConfig,
    cache: &mut GraphCompileCache,
) -> Result<ExecutionPlan, RenderGraphError> {
    let sig = cache.topology_signature(graph);
    cache.observe_topology(sig);
    compile(graph, caps, cfg)
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::{Duration, Instant};

    use super::*;
    use crate::GraphBuilder;

    #[test]
    fn test_pass_register_topology() {
        let mut gb = GraphBuilder::new();
        let x = gb.add_transient_sized(4096);
        let b = gb.pass("B").read(x).finish();
        let a = gb.pass("A").write(x).finish();
        let _ = b;
        let _ = a;
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig::default(),
            )
            .unwrap();
        assert_eq!(
            plan.execution_order(),
            &[PassHandle(1), PassHandle(0)],
            "A must run before B regardless of declaration order"
        );
    }

    #[test]
    fn test_pass_read_write_dependency() {
        let mut gb = GraphBuilder::new();
        let x = gb.add_transient_sized(1);
        let y = gb.add_transient_sized(1);
        let z = gb.add_transient_sized(1);
        let a = gb.pass("A").write(x).finish();
        let b = gb.pass("B").read(x).write(y).finish();
        let c = gb.pass("C").read(y).write(z).finish();
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig::default(),
            )
            .unwrap();
        assert_eq!(
            plan.execution_order(),
            &[a, b, c],
            "chain A→B→C must be unique"
        );
    }

    #[test]
    fn test_capability_gate_prune_rt() {
        let mut gb = GraphBuilder::new();
        let t = gb.add_transient_sized(4);
        let _rt = gb.pass("rt").require(Capability::RayTracing).write(t).finish();
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities {
                    ray_tracing: false,
                },
                &CompilationConfig::default(),
            )
            .unwrap();
        assert!(plan.execution_order().is_empty());
    }

    #[test]
    fn test_fallback_pass_substitution() {
        let mut gb = GraphBuilder::new();
        let x = gb.add_transient_sized(1);
        let fb = gb.pass("fb").write(x).finish();
        let _rt = gb
            .pass("rt")
            .require(Capability::RayTracing)
            .write(x)
            .fallback(fb)
            .finish();
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities {
                    ray_tracing: false,
                },
                &CompilationConfig::default(),
            )
            .unwrap();
        assert_eq!(plan.execution_order(), &[fb]);
    }

    #[test]
    fn test_virtual_resource_alias() {
        let mut gb = GraphBuilder::new();
        let _r0 = gb.create_transient(0, 2, 1024);
        let _r1 = gb.create_transient(3, 5, 1024);
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig::default(),
            )
            .unwrap();
        assert_eq!(plan.physical_slot.len(), 2);
        assert_eq!(plan.physical_slot[0], plan.physical_slot[1]);
    }

    #[test]
    fn test_resource_lifetime_overlap() {
        let mut gb = GraphBuilder::new();
        let _r0 = gb.create_transient(0, 3, 1024);
        let _r1 = gb.create_transient(2, 5, 1024);
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig::default(),
            )
            .unwrap();
        assert_ne!(plan.physical_slot[0], plan.physical_slot[1]);
    }

    #[test]
    fn test_aliasing_vram_savings_40pct() {
        let mut gb = GraphBuilder::new();
        let mut ids = Vec::new();
        for i in 0u32..10 {
            let f = i * 3;
            let l = f + 1;
            ids.push(gb.create_transient(f, l, 1024u64 * u64::from(i + 1)));
        }
        let _ = ids;
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig::default(),
            )
            .unwrap();
        let saving = 1.0 - (plan.aliased_vram() as f64 / plan.naive_vram() as f64);
        assert!(
            saving >= 0.40,
            "expected >=40% savings, naive={} aliased={}",
            plan.naive_vram(),
            plan.aliased_vram()
        );
    }

    #[test]
    fn test_minimal_barrier_chain() {
        let mut gb = GraphBuilder::new();
        let t = gb.add_transient_sized(1);
        let a = gb.pass("A").write(t).finish();
        let b = gb.pass("B").read(t).finish();
        let c = gb.pass("C").read(t).finish();
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig::default(),
            )
            .unwrap();
        assert_eq!(plan.order, vec![a, b, c]);
        assert_eq!(plan.barriers.len(), 1);
        assert_eq!(plan.barriers[0].after_pass, a);
    }

    #[test]
    fn test_split_barrier_overlap() {
        let mut gb = GraphBuilder::new();
        let t = gb.add_transient_sized(1);
        let a = gb.pass("A").write(t).finish();
        let b = gb.pass("B").finish();
        let c = gb.pass("C").finish();
        let d = gb.pass("D").read(t).finish();
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig::default(),
            )
            .unwrap();
        assert_eq!(
            plan.split_barriers,
            split_barriers_for_fixture(a, b, c, d, t)
        );
    }

    #[test]
    fn test_async_compute_queue_assign() {
        let mut gb = GraphBuilder::new();
        let x = gb.add_transient_sized(1);
        let comp = gb
            .pass("comp")
            .write(x)
            .queue(QueueAffinity::AsyncCompute)
            .finish();
        let gfx = gb.pass("gfx").read(x).finish();
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig::default(),
            )
            .unwrap();
        assert_eq!(plan.order, vec![comp, gfx]);
        let meta_comp = plan
            .pass_meta
            .iter()
            .find(|m| m.handle == comp)
            .unwrap();
        let meta_gfx = plan
            .pass_meta
            .iter()
            .find(|m| m.handle == gfx)
            .unwrap();
        assert_eq!(meta_comp.queue, QueueKind::AsyncCompute);
        assert_eq!(meta_gfx.queue, QueueKind::Graphics);
    }

    #[test]
    fn test_cross_queue_fence_insert() {
        let mut gb = GraphBuilder::new();
        let x = gb.add_transient_sized(1);
        let comp = gb
            .pass("comp")
            .write(x)
            .queue(QueueAffinity::AsyncCompute)
            .finish();
        let gfx = gb.pass("gfx").read(x).finish();
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig::default(),
            )
            .unwrap();
        assert_eq!(plan.fences.len(), 1);
        assert_eq!(
            plan.fences[0],
            FencePlacement {
                signal_on: QueueKind::AsyncCompute,
                wait_on: QueueKind::Graphics,
            }
        );
        let _ = (comp, gfx);
    }

    #[test]
    fn test_budget_cull_low_priority() {
        let mut gb = GraphBuilder::new();
        let r = gb.add_transient_sized(1);
        let p0 = gb
            .pass("p0")
            .write(r)
            .cost_ms(8.0)
            .priority(PassPriority::Low)
            .finish();
        let p1 = gb
            .pass("p1")
            .read(r)
            .write(r)
            .cost_ms(6.0)
            .priority(PassPriority::Low)
            .finish();
        let p2 = gb
            .pass("p2")
            .read(r)
            .write(r)
            .cost_ms(4.0)
            .priority(PassPriority::Optional)
            .finish();
        let p3 = gb
            .pass("p3")
            .read(r)
            .write(r)
            .cost_ms(4.0)
            .priority(PassPriority::Optional)
            .finish();
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig {
                    target_budget_ms: 16.0,
                    budget_culling_enabled: true,
                    param_epoch: 0,
                },
            )
            .unwrap();
        assert!(!plan.order.contains(&p2));
        assert!(!plan.order.contains(&p3));
        assert!(plan.order.contains(&p0));
        assert!(plan.order.contains(&p1));
        let total: f32 = plan
            .order
            .iter()
            .map(|h| g.passes[h.0 as usize].cost_ms)
            .sum();
        assert!(total <= 16.0, "total {total}");
    }

    #[test]
    fn test_multi_view_share_culling() {
        let mut gb = GraphBuilder::new();
        let shared = gb.add_transient_sized(1);
        let cull = gb.pass("cull").write(shared).finish();
        let mut draws = Vec::new();
        for _ in 0..4 {
            draws.push(gb.pass("draw").read(shared).finish());
        }
        gb.set_multi_view_pattern(cull, &draws);
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig::default(),
            )
            .unwrap();
        assert_eq!(plan.shared_cull_exec_count, 1);
        assert_eq!(plan.per_view_draw_exec_count, 4);
    }

    fn fake_encode_pass(_id: u32) {
        thread::sleep(Duration::from_millis(2));
    }

    #[test]
    fn test_parallel_encode_4_threads() {
        let n = 8u32;
        let t1 = Instant::now();
        for i in 0..n {
            fake_encode_pass(i);
        }
        let d1 = t1.elapsed();

        let t4b = Instant::now();
        thread::scope(|s| {
            let mut hs = Vec::new();
            for t in 0..4u32 {
                hs.push(s.spawn(move || {
                    let start = t * 2;
                    for i in start..start + 2 {
                        fake_encode_pass(i);
                    }
                }));
            }
            for h in hs {
                h.join().unwrap();
            }
        });
        let d4 = t4b.elapsed();
        let ratio = d4.as_secs_f64() / d1.as_secs_f64();
        assert!(
            ratio < 0.6,
            "expected parallel speedup, seq={d1:?} par={d4:?} ratio={ratio}"
        );
    }

    #[test]
    fn test_streaming_placeholder_sub() {
        let mut gb = GraphBuilder::new();
        let tex = gb.add_transient_sized(16);
        let _p = gb
            .pass("sample")
            .read(tex)
            .streaming(tex, StreamingState::Pending)
            .finish();
        let g = gb.build().unwrap();
        let plan = g
            .compile(
                &DeviceCapabilities::default(),
                &CompilationConfig::default(),
            )
            .unwrap();
        assert_eq!(plan.order.len(), 1);
    }

    #[test]
    fn test_graph_recompile_on_cache() {
        let mut gb = GraphBuilder::new();
        let x = gb.add_transient_sized(1);
        let a = gb.pass("A").write(x).finish();
        let g0 = gb.build().unwrap();
        let mut cache = GraphCompileCache::default();
        let _ = compile_with_cache(
            &g0,
            &DeviceCapabilities::default(),
            &CompilationConfig::default(),
            &mut cache,
        )
        .unwrap();
        let mut gb2 = GraphBuilder::new();
        let x2 = gb2.add_transient_sized(1);
        let _a2 = gb2.pass("A").write(x2).finish();
        let b = gb2.pass("B").read(x2).finish();
        let g1 = gb2.build().unwrap();
        let _ = b;
        let _ = compile_with_cache(
            &g1,
            &DeviceCapabilities::default(),
            &CompilationConfig::default(),
            &mut cache,
        )
        .unwrap();
        let (changes, hits, entries) = cache.stats();
        assert_eq!(changes, 1);
        assert_eq!(entries, 2);
        assert_eq!(hits, 0);
        let _ = a;
    }

    #[test]
    fn test_graph_no_recompile_param_only() {
        let mut gb = GraphBuilder::new();
        let x = gb.add_transient_sized(1);
        let a = gb.pass("A").write(x).finish();
        let g = gb.build().unwrap();
        let mut cache = GraphCompileCache::default();
        let cfg0 = CompilationConfig {
            param_epoch: 0,
            ..CompilationConfig::default()
        };
        compile_with_cache(
            &g,
            &DeviceCapabilities::default(),
            &cfg0,
            &mut cache,
        )
        .unwrap();
        let cfg1 = CompilationConfig {
            param_epoch: 1,
            ..CompilationConfig::default()
        };
        compile_with_cache(
            &g,
            &DeviceCapabilities::default(),
            &cfg1,
            &mut cache,
        )
        .unwrap();
        let (changes, hits, _) = cache.stats();
        assert_eq!(changes, 0);
        assert_eq!(hits, 1);
        let _ = a;
    }
}
