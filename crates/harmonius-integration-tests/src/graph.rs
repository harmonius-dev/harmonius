//! Graph programs, instances, and execution results.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;

use crate::access::GraphAccessDescriptor;
use crate::arena::ThreadArena;
use crate::commands::{CommandSegment, ParallelCommandWriter};
use crate::context::ExecutionContext;
use crate::events::{DebugBridge, EventWriter};
use crate::world::{Entity, World};

/// Stable asset handle for `GraphProgram` data.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AssetId(pub u64);

/// ECS schedule phase binding (fixed vs variable timestep class).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphSchedulePhase {
    /// Fixed timestep simulation slice.
    SimulationFixed,
    /// Variable timestep animation slice.
    AnimVariable,
}

/// Where a graph suspended last frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResumeVariant {
    /// First entry.
    Entry,
    /// Resume after yield marker.
    AfterYield(u32),
    /// Terminal.
    Done,
}

/// Mutable locals for one graph instance.
#[derive(Clone, Debug)]
pub struct GraphInstanceState {
    /// Hot-reload / save layout fingerprint.
    pub layout_hash: u64,
    /// Opaque scratch for tests.
    pub scratch: i32,
    /// Suspend point.
    pub suspend: ResumeVariant,
}

impl Default for GraphInstanceState {
    fn default() -> Self {
        Self {
            layout_hash: 0,
            scratch: 0,
            suspend: ResumeVariant::Entry,
        }
    }
}

/// Outcome of one graph step.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StepResult {
    /// Completed successfully.
    Success,
    /// Cooperative yield.
    Yield(ResumeVariant),
    /// Hard stop.
    Error(GraphError),
}

/// Graph runtime errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphError {
    /// Command segment capacity exhausted.
    CommandBufferFull {
        /// Engine frame.
        frame: u64,
        /// First op slot that failed.
        op_slot: u64,
    },
    /// Instruction budget exceeded.
    BudgetExhausted,
}

/// Registration-time scheduler errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphRegistrationError {
    /// Program phase disagrees with registration target.
    PhaseMismatch {
        /// Declared on asset.
        program_phase: GraphSchedulePhase,
        /// Registration target.
        target_phase: GraphSchedulePhase,
    },
}

/// Hot-reload failure modes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HotReloadError {
    /// Layout hash mismatch.
    LayoutMismatch {
        /// Expected hash.
        expected: u64,
        /// Offered hash.
        offered: u64,
    },
}

/// Function pointer table for one program.
#[derive(Clone)]
pub struct FnPtrTable {
    /// Entry function.
    pub entry: GraphFn,
}

/// Codegen'd graph entry signature.
pub type GraphFn =
    fn(&mut GraphInstanceState, &mut ExecutionContext<'_>) -> StepResult;

/// Loaded graph asset metadata.
#[derive(Clone)]
pub struct GraphProgram {
    /// Asset id.
    pub id: AssetId,
    /// Schedule phase class.
    pub phase: GraphSchedulePhase,
    /// Access metadata.
    pub access: GraphAccessDescriptor,
    /// Function pointers.
    pub fn_table: FnPtrTable,
    /// Expected `GraphInstanceState::layout_hash`.
    pub state_layout_hash: u64,
}

/// Per-entity graph component data.
#[derive(Clone)]
pub struct GraphInstance {
    /// Program asset reference.
    pub program_id: AssetId,
    /// Mutable graph state.
    pub state: GraphInstanceState,
    /// Revision counter.
    pub version: u64,
}

/// Engine-level graph execution knobs.
#[derive(Clone)]
pub struct GraphExecutionConfig {
    /// Max back-edges per step.
    pub instruction_budget: u32,
    /// Worker threads for `par_iter` slice.
    pub parallel_workers: usize,
    /// Current frame id.
    pub frame: u64,
    /// Delta time seconds.
    pub delta_time: f32,
    /// Optional diagnostic sink.
    pub diagnostics: Option<Arc<Mutex<Vec<DiagnosticEvent>>>>,
}

/// Structured diagnostics for tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiagnosticEvent {
    /// Command segment overflow.
    CommandBufferFull {
        /// Program id.
        program: AssetId,
        /// Worker index.
        worker: usize,
    },
}

/// ECS system façade coordinating parallel graph execution.
pub struct GraphExecutionSystem;

static LAST_MAX_CONCURRENCY: AtomicUsize = AtomicUsize::new(0);

/// Max concurrent graph bodies observed in the last `run` call.
pub fn max_concurrency_this_frame() -> usize {
    LAST_MAX_CONCURRENCY.load(Ordering::SeqCst)
}

/// Count of parallel scheduling buckets (integration diagnostics).
pub fn graph_instance_bucket_count(
    instances: &[(Entity, GraphInstance)],
    programs: &HashMap<AssetId, GraphProgram>,
    phase: GraphSchedulePhase,
) -> usize {
    partition_instances(instances, programs, phase).len()
}

impl GraphExecutionSystem {
    /// Run all `instances` for `phase`, writing commands into `parallel`.
    #[allow(clippy::too_many_arguments)]
    pub fn run(
        config: &GraphExecutionConfig,
        world: &World,
        programs: &HashMap<AssetId, GraphProgram>,
        instances: &mut [(Entity, GraphInstance)],
        phase: GraphSchedulePhase,
        parallel: &mut ParallelCommandWriter,
        events: &EventWriter,
        arena: &ThreadArena,
        debug: Option<&DebugBridge>,
    ) -> Vec<(Entity, StepResult)> {
        LAST_MAX_CONCURRENCY.store(0, Ordering::SeqCst);
        let workers = parallel.workers().max(1);
        let active = Arc::new(AtomicUsize::new(0));
        let max_active = Arc::new(AtomicUsize::new(0));

        let locks: Arc<Vec<Mutex<GraphInstance>>> = Arc::new(
            instances
                .iter()
                .map(|(_, g)| Mutex::new(g.clone()))
                .collect(),
        );

        let buckets = partition_instances(instances, programs, phase);
        let results: Arc<Mutex<Vec<(Entity, StepResult)>>> = Arc::new(Mutex::new(Vec::new()));

        for batch in buckets.chunks(workers) {
            let (mi, mt) = parallel.writer(0).limits();
            let mut locals: Vec<CommandSegment> = (0..batch.len())
                .map(|_| CommandSegment::with_limits(mi, mt))
                .collect();
            let barrier = Arc::new(Barrier::new(batch.len()));
            thread::scope(|s| {
                for (bucket, local_seg) in batch.iter().zip(locals.iter_mut()) {
                    let barrier = Arc::clone(&barrier);
                    let locks = Arc::clone(&locks);
                    let results = Arc::clone(&results);
                    let active = Arc::clone(&active);
                    let max_active = Arc::clone(&max_active);
                    s.spawn(move || {
                        barrier.wait();
                        bump_active(&active, &max_active);
                        for &(entity, inst_idx) in bucket {
                            let mut inst = locks[inst_idx].lock().expect("poison");
                            let prog = programs
                                .get(&inst.program_id)
                                .expect("program missing for instance");
                            let mut ctx = ExecutionContext {
                                arena,
                                commands: local_seg,
                                debug,
                                delta_time: config.delta_time,
                                entity,
                                events,
                                frame: config.frame,
                                instruction_budget: config.instruction_budget,
                                world,
                            };
                            let step = (prog.fn_table.entry)(&mut inst.state, &mut ctx);
                            results.lock().expect("poison").push((entity, step));
                        }
                        dec_active(&active);
                    });
                }
            });
            for (slot, local) in locals.into_iter().enumerate() {
                parallel.absorb_worker_segment(slot, local);
            }
        }

        for (i, (_, gi)) in instances.iter_mut().enumerate() {
            *gi = locks[i].lock().expect("poison").clone();
        }

        let _ = max_active;
        let mut out = Arc::try_unwrap(results)
            .ok()
            .map(|m| m.into_inner().expect("poison"))
            .unwrap_or_default();
        out.sort_by_key(|(e, _)| e.0);
        out
    }
}

fn bump_active(active: &Arc<AtomicUsize>, max_active: &Arc<AtomicUsize>) {
    let v = active.fetch_add(1, Ordering::SeqCst) + 1;
    loop {
        let cur = max_active.load(Ordering::SeqCst);
        if v <= cur {
            break;
        }
        if max_active
            .compare_exchange_weak(cur, v, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            LAST_MAX_CONCURRENCY.store(v, Ordering::SeqCst);
            break;
        }
    }
}

fn dec_active(active: &Arc<AtomicUsize>) {
    active.fetch_sub(1, Ordering::SeqCst);
}

/// Partition instance indices into serial buckets by intersecting `writes` access.
fn partition_instances(
    instances: &[(Entity, GraphInstance)],
    programs: &HashMap<AssetId, GraphProgram>,
    phase: GraphSchedulePhase,
) -> Vec<Vec<(Entity, usize)>> {
    let refs: Vec<(Entity, usize)> = instances
        .iter()
        .enumerate()
        .filter_map(|(idx, (e, gi))| {
            programs
                .get(&gi.program_id)
                .filter(|p| p.phase == phase)
                .map(|_| (*e, idx))
        })
        .collect();

    let n = refs.len();
    if n == 0 {
        return Vec::new();
    }

    let mut parent: Vec<usize> = (0..n).collect();
    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    fn union(parent: &mut [usize], a: usize, b: usize) {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra != rb {
            parent[rb] = ra;
        }
    }

    for a in 0..n {
        for b in a + 1..n {
            let ia = refs[a].1;
            let ib = refs[b].1;
            let pa = programs[&instances[ia].1.program_id].access.writes;
            let pb = programs[&instances[ib].1.program_id].access.writes;
            if pa.intersects(pb) {
                union(&mut parent, a, b);
            }
        }
    }

    let mut groups: HashMap<usize, Vec<(Entity, usize)>> = HashMap::new();
    for (i, &(entity, idx)) in refs.iter().enumerate() {
        let root = find(&mut parent, i);
        groups.entry(root).or_default().push((entity, idx));
    }

    groups.into_values().collect()
}

/// Report command-buffer overflow to optional diagnostics.
pub fn report_command_buffer_full(
    config: &GraphExecutionConfig,
    program: AssetId,
    worker: usize,
) {
    if let Some(sink) = &config.diagnostics {
        if let Ok(mut g) = sink.lock() {
            g.push(DiagnosticEvent::CommandBufferFull { program, worker });
        }
    }
}
