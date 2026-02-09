//! Graph compilation pipeline.
//!
//! Transforms a declarative render graph into an optimized execution plan
//! with automatic barrier insertion, resource aliasing, and cross-queue
//! synchronization.
//!
//! ## Resource index convention
//!
//! `ResourceAccess::resource_index` uses separate index spaces for images
//! and buffers (image layout distinguishes the two: `UNDEFINED` ⇒ buffer).
//! Output structs (`ResourceLifetime`, `AliasingGroup`) use a **unified**
//! index scheme: `0..num_images` for images, `num_images..num_images+num_buffers`
//! for buffers.

use std::collections::{HashMap, HashSet};

use ash::vk;
use smallvec::SmallVec;

use crate::graph::barrier::{
    deduce_barrier, deduce_queue_transfer, merge_barriers, PassBarriers,
};
use crate::graph::pass::{
    AttachmentInfo, PassCondition, PassNode, QueueType, RecordedCommand, ResourceAccess,
};
use crate::graph::resource::{
    BufferInfo, BufferHandle, FlagInfo, GraphError, ImageInfo, ResourceKind,
};

// ---------------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------------

/// A submission batch: a group of passes on the same queue.
#[derive(Debug)]
pub(crate) struct SubmissionBatch {
    pub queue: QueueType,
    /// Positions in `CompiledGraph::pass_order`.
    pub passes: Vec<usize>,
}

/// Timeline semaphore operation for cross-queue sync.
#[derive(Debug)]
pub(crate) struct SemaphoreOp {
    pub signal_value: u64,
    pub signal_queue: QueueType,
    /// Position in `pass_order` of the signalling pass.
    pub signal_after_pass: usize,
    pub wait_queue: QueueType,
    /// Position in `pass_order` of the waiting pass.
    pub wait_before_pass: usize,
}

/// Resource lifetime for aliasing analysis.
///
/// `resource_index` uses the unified scheme (images first, then buffers).
#[derive(Debug)]
pub(crate) struct ResourceLifetime {
    pub resource_index: u16,
    /// Position in `pass_order` of first use.
    pub first_use: usize,
    /// Position in `pass_order` of last use.
    pub last_use: usize,
}

/// Aliasing assignment: resources that may share physical memory.
///
/// Resource indices use the unified scheme.
#[derive(Debug)]
pub(crate) struct AliasingGroup {
    pub resources: Vec<u16>,
}

/// The result of compiling a `RenderGraph`.
#[derive(Debug)]
pub struct CompiledGraph {
    /// Passes in topologically sorted execution order (original pass indices).
    pub(crate) pass_order: Vec<usize>,
    /// Barriers to insert before each pass (indexed by `pass_order` position).
    pub(crate) barriers: Vec<PassBarriers>,
    /// Submission batches grouping passes by queue.
    pub(crate) batches: Vec<SubmissionBatch>,
    /// Semaphore operations for cross-queue synchronization.
    pub(crate) semaphore_ops: Vec<SemaphoreOp>,
    /// Resource lifetimes for aliasing.
    pub(crate) lifetimes: Vec<ResourceLifetime>,
    /// Aliasing groups (resources sharing memory).
    pub(crate) aliasing_groups: Vec<AliasingGroup>,
    /// The final command lists per pass (with draw slots spliced in).
    pub(crate) pass_commands: Vec<Vec<RecordedCommand>>,
    /// Color attachment info per pass.
    pub(crate) pass_color_attachments: Vec<SmallVec<[AttachmentInfo; 4]>>,
    /// Depth attachment info per pass.
    pub(crate) pass_depth_attachments: Vec<Option<AttachmentInfo>>,
}

// ---------------------------------------------------------------------------
// Internal type aliases
// ---------------------------------------------------------------------------

/// (resource_index, is_image, version) → writer pass index.
type WritersMap = HashMap<(u16, bool, u16), usize>;

// ---------------------------------------------------------------------------
// Compile entry point
// ---------------------------------------------------------------------------

pub(crate) fn compile(
    passes: &[PassNode],
    images: &[ImageInfo],
    buffers: &[BufferInfo],
    flags: &[FlagInfo],
    draw_slot_commands: &[Vec<RecordedCommand>],
    _generation: u32,
) -> Result<CompiledGraph, GraphError> {
    let num_passes = passes.len();

    // Warn on duplicate pass names.
    check_duplicate_names(passes);

    // -- Step 0: Evaluate condition flags ------------------------------------

    let (mut active, pass_accesses) = evaluate_conditions(passes, flags);

    // Build writer lookup from *original* accesses (includes disabled passes
    // so transitive-disable can detect missing producers).
    let all_writers = build_all_writers_map(passes);

    // Step 0.3: transitive disabling.
    transitive_disable(&mut active, &pass_accesses, &all_writers);

    // -- Step 1: Splice draw slot commands -----------------------------------

    let mut all_commands = splice_draw_slots(passes, &active, draw_slot_commands)?;

    // -- Deferred validation (command-resource, queue, usage, duplicate writes)

    validate_command_resources(passes, &active, &all_commands, &pass_accesses, buffers)?;
    validate_command_queue(passes, &active, &all_commands)?;
    validate_usage_flags(&pass_accesses, passes, &active, images, buffers)?;
    validate_duplicate_writes(&pass_accesses, passes, &active, images, buffers)?;

    // -- Step 2: Build DAG edges ---------------------------------------------

    let (adjacency, reverse_adj) =
        build_dag_edges(num_passes, &active, &pass_accesses, &all_writers);

    // -- Step 3: Dead-pass elimination ---------------------------------------

    eliminate_dead_passes(
        &mut active,
        &pass_accesses,
        &reverse_adj,
        images,
        buffers,
    );

    // Recompute in-degree for the surviving passes.
    let in_degree = compute_in_degree(&active, &adjacency);

    // -- Step 4: Topological sort (Kahn's algorithm) -------------------------

    let pass_order = topological_sort(passes, &active, &adjacency, in_degree)?;

    // -- Step 5: Resource lifetimes ------------------------------------------

    let lifetimes = compute_lifetimes(
        &pass_order,
        &pass_accesses,
        images,
        buffers,
    );

    // -- Step 6: Transient resource aliasing ---------------------------------

    let aliasing_groups = compute_aliasing(&lifetimes, images, buffers);

    // -- Step 7: Barrier deduction -------------------------------------------

    let barriers = deduce_all_barriers(
        &pass_order,
        &pass_accesses,
        passes,
        images,
    );

    // -- Step 8: Timeline semaphore insertion ---------------------------------

    let semaphore_ops = insert_semaphores(&pass_order, passes, &adjacency, &active);

    // -- Step 9: Build submission batches ------------------------------------

    let batches = build_batches(&pass_order, passes);

    // -- Collect per-pass outputs (indexed by position in pass_order) --------

    let mut out_commands = Vec::with_capacity(pass_order.len());
    let mut out_colors = Vec::with_capacity(pass_order.len());
    let mut out_depths = Vec::with_capacity(pass_order.len());
    for &pidx in &pass_order {
        out_commands.push(std::mem::take(&mut all_commands[pidx]));
        out_colors.push(passes[pidx].color_attachments.clone());
        out_depths.push(passes[pidx].depth_attachment);
    }

    Ok(CompiledGraph {
        pass_order,
        barriers,
        batches,
        semaphore_ops,
        lifetimes,
        aliasing_groups,
        pass_commands: out_commands,
        pass_color_attachments: out_colors,
        pass_depth_attachments: out_depths,
    })
}

// ---------------------------------------------------------------------------
// Step 0: Condition flag evaluation
// ---------------------------------------------------------------------------

/// Evaluate pass conditions (step 0.1) and strip conditional accesses whose
/// flags are false (step 0.2).  Returns the active-pass bitmap and the
/// filtered per-pass accesses.
fn evaluate_conditions(
    passes: &[PassNode],
    flags: &[FlagInfo],
) -> (Vec<bool>, Vec<SmallVec<[ResourceAccess; 8]>>) {
    let n = passes.len();
    let mut active = vec![true; n];
    let mut pass_accesses = Vec::with_capacity(n);

    // 0.1: Mark passes whose condition is not met.
    for (i, pass) in passes.iter().enumerate() {
        match pass.condition {
            PassCondition::Always => {}
            PassCondition::WhenFlag(f) => {
                if !flags[f.index as usize].value {
                    active[i] = false;
                }
            }
            PassCondition::WhenNotFlag(f) => {
                if flags[f.index as usize].value {
                    active[i] = false;
                }
            }
        }
    }

    // 0.2: For each active pass, drop conditional accesses whose flag is false.
    for (i, pass) in passes.iter().enumerate() {
        if !active[i] {
            pass_accesses.push(SmallVec::new());
            continue;
        }
        let filtered: SmallVec<[ResourceAccess; 8]> = pass
            .accesses
            .iter()
            .filter(|a| {
                a.condition
                    .map_or(true, |f| flags[f.index as usize].value)
            })
            .copied()
            .collect();
        pass_accesses.push(filtered);
    }

    (active, pass_accesses)
}

/// Step 0.3 — iterate until fixed-point: if an enabled pass has a
/// **required** (non-conditional) read whose producer is disabled, disable
/// this pass too.
fn transitive_disable(
    active: &mut [bool],
    pass_accesses: &[SmallVec<[ResourceAccess; 8]>],
    all_writers: &WritersMap,
) {
    loop {
        let mut changed = false;
        for i in 0..active.len() {
            if !active[i] {
                continue;
            }
            let mut should_disable = false;
            for access in &pass_accesses[i] {
                // Only required (non-conditional) reads matter.
                if access.condition.is_some() {
                    continue;
                }
                if let Some(ver) = access.version_read {
                    let key = (access.resource_index, access_is_image(access), ver);
                    if let Some(&writer) = all_writers.get(&key) {
                        if !active[writer] {
                            should_disable = true;
                            break;
                        }
                    }
                }
            }
            if should_disable {
                active[i] = false;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

// ---------------------------------------------------------------------------
// Step 1: Draw-slot splicing
// ---------------------------------------------------------------------------

fn splice_draw_slots(
    passes: &[PassNode],
    active: &[bool],
    draw_slot_commands: &[Vec<RecordedCommand>],
) -> Result<Vec<Vec<RecordedCommand>>, GraphError> {
    let mut result = Vec::with_capacity(passes.len());

    for (i, pass) in passes.iter().enumerate() {
        if !active[i] {
            result.push(Vec::new());
            continue;
        }
        let mut commands = Vec::with_capacity(pass.commands.len());
        for cmd in &pass.commands {
            if let RecordedCommand::DrawSlotPlaceholder(slot) = cmd {
                let idx = slot.slot_index as usize;
                match draw_slot_commands.get(idx) {
                    Some(slot_cmds) if !slot_cmds.is_empty() => {
                        commands.extend_from_slice(slot_cmds);
                    }
                    _ => {
                        return Err(GraphError::UnfilledDrawSlot {
                            pass: pass.name,
                            slot_index: slot.slot_index,
                        });
                    }
                }
            } else {
                commands.push(cmd.clone());
            }
        }
        result.push(commands);
    }

    Ok(result)
}

// ---------------------------------------------------------------------------
// Step 2: DAG construction
// ---------------------------------------------------------------------------

/// Build a writers map from *all* passes (including disabled ones).
/// Needed for transitive-disable and later DAG edge queries.
fn build_all_writers_map(passes: &[PassNode]) -> WritersMap {
    let mut writers = HashMap::new();
    for (i, pass) in passes.iter().enumerate() {
        for access in &pass.accesses {
            if let Some(v) = access.version_write {
                writers.insert(
                    (access.resource_index, access_is_image(access), v),
                    i,
                );
            }
        }
    }
    writers
}

/// Build forward and reverse adjacency lists from the filtered accesses of
/// active passes.
fn build_dag_edges(
    num_passes: usize,
    active: &[bool],
    pass_accesses: &[SmallVec<[ResourceAccess; 8]>],
    writers: &WritersMap,
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut adjacency = vec![Vec::new(); num_passes];
    let mut reverse_adj = vec![Vec::new(); num_passes];
    let mut edge_set: HashSet<(usize, usize)> = HashSet::new();

    for (reader, accesses) in pass_accesses.iter().enumerate() {
        if !active[reader] {
            continue;
        }
        for access in accesses {
            if let Some(ver) = access.version_read {
                let key = (access.resource_index, access_is_image(access), ver);
                if let Some(&writer) = writers.get(&key) {
                    if writer != reader
                        && active[writer]
                        && edge_set.insert((writer, reader))
                    {
                        adjacency[writer].push(reader);
                        reverse_adj[reader].push(writer);
                    }
                }
            }
        }
    }

    (adjacency, reverse_adj)
}

/// Recompute in-degree counts for active passes only (used after dead-pass
/// elimination invalidates the original counts).
fn compute_in_degree(active: &[bool], adjacency: &[Vec<usize>]) -> Vec<usize> {
    let n = active.len();
    let mut in_deg = vec![0usize; n];
    for (i, neighbors) in adjacency.iter().enumerate() {
        if !active[i] {
            continue;
        }
        for &j in neighbors {
            if active[j] {
                in_deg[j] += 1;
            }
        }
    }
    in_deg
}

// ---------------------------------------------------------------------------
// Step 3: Dead-pass elimination
// ---------------------------------------------------------------------------

/// Walk backward from output passes (those writing imported resources) and
/// cull any active pass that does not contribute to an output.
fn eliminate_dead_passes(
    active: &mut [bool],
    pass_accesses: &[SmallVec<[ResourceAccess; 8]>],
    reverse_adj: &[Vec<usize>],
    images: &[ImageInfo],
    buffers: &[BufferInfo],
) {
    let n = active.len();
    let mut reachable = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();

    // Seed: passes that write to imported (externally-owned) resources.
    for (i, accesses) in pass_accesses.iter().enumerate() {
        if !active[i] {
            continue;
        }
        let is_output = accesses.iter().any(|a| {
            if a.version_write.is_none() {
                return false;
            }
            let is_img = access_is_image(a);
            let info = if is_img {
                images.get(a.resource_index as usize).map(|x| &x.resource)
            } else {
                buffers.get(a.resource_index as usize).map(|x| &x.resource)
            };
            info.is_some_and(|r| matches!(r.kind, ResourceKind::Imported))
        });
        if is_output {
            reachable[i] = true;
            stack.push(i);
        }
    }

    // Walk backward through the DAG.
    while let Some(idx) = stack.pop() {
        for &dep in &reverse_adj[idx] {
            if active[dep] && !reachable[dep] {
                reachable[dep] = true;
                stack.push(dep);
            }
        }
    }

    // Cull unreachable.
    for i in 0..n {
        if active[i] && !reachable[i] {
            active[i] = false;
        }
    }
}

// ---------------------------------------------------------------------------
// Step 4: Topological sort
// ---------------------------------------------------------------------------

fn topological_sort(
    passes: &[PassNode],
    active: &[bool],
    adjacency: &[Vec<usize>],
    mut in_degree: Vec<usize>,
) -> Result<Vec<usize>, GraphError> {
    // Seed ready-set with zero-in-degree active passes.
    let mut ready: Vec<usize> = (0..passes.len())
        .filter(|&i| active[i] && in_degree[i] == 0)
        .collect();

    let mut sorted = Vec::new();
    let mut last_queue: Option<QueueType> = None;

    while !ready.is_empty() {
        let pick = pick_next_pass(&ready, passes, last_queue);
        let chosen = ready.swap_remove(pick);

        last_queue = Some(passes[chosen].queue);
        sorted.push(chosen);

        for &next in &adjacency[chosen] {
            if !active[next] {
                continue;
            }
            in_degree[next] -= 1;
            if in_degree[next] == 0 {
                ready.push(next);
            }
        }
    }

    // Cycle detection.
    let active_count = active.iter().filter(|&&a| a).count();
    if sorted.len() != active_count {
        let cycle: Vec<&'static str> = (0..passes.len())
            .filter(|&i| active[i] && in_degree[i] > 0)
            .map(|i| passes[i].name)
            .collect();
        return Err(GraphError::DependencyCycle { cycle });
    }

    Ok(sorted)
}

/// Heuristic for choosing the next pass to schedule from the ready set.
///
/// Primary: prefer the same queue as the previous pass (fewer queue
/// switches).  Secondary: prefer `AsyncCompute` (schedule async work early).
fn pick_next_pass(
    ready: &[usize],
    passes: &[PassNode],
    last_queue: Option<QueueType>,
) -> usize {
    if let Some(lq) = last_queue {
        if let Some(idx) = ready.iter().position(|&i| passes[i].queue == lq) {
            return idx;
        }
    }
    if let Some(idx) = ready
        .iter()
        .position(|&i| passes[i].queue == QueueType::AsyncCompute)
    {
        return idx;
    }
    0
}

// ---------------------------------------------------------------------------
// Step 5: Resource lifetimes
// ---------------------------------------------------------------------------

fn compute_lifetimes(
    pass_order: &[usize],
    pass_accesses: &[SmallVec<[ResourceAccess; 8]>],
    images: &[ImageInfo],
    buffers: &[BufferInfo],
) -> Vec<ResourceLifetime> {
    let num_images = images.len() as u16;

    // (unified_id) → (first_position, last_position)
    let mut usage: HashMap<u16, (usize, usize)> = HashMap::new();

    for (pos, &pass_idx) in pass_order.iter().enumerate() {
        for access in &pass_accesses[pass_idx] {
            let uid =
                unified_resource_id(access.resource_index, access_is_image(access), num_images);
            let entry = usage.entry(uid).or_insert((pos, pos));
            entry.0 = entry.0.min(pos);
            entry.1 = entry.1.max(pos);
        }
    }

    let mut lifetimes = Vec::new();
    for (&uid, &(first, last)) in &usage {
        let (ridx, is_img) = decode_unified_id(uid, num_images);
        let is_transient = if is_img {
            images
                .get(ridx as usize)
                .is_some_and(|i| matches!(i.resource.kind, ResourceKind::Transient))
        } else {
            buffers
                .get(ridx as usize)
                .is_some_and(|b| matches!(b.resource.kind, ResourceKind::Transient))
        };
        if is_transient {
            lifetimes.push(ResourceLifetime {
                resource_index: uid,
                first_use: first,
                last_use: last,
            });
        }
    }

    lifetimes
}

// ---------------------------------------------------------------------------
// Step 6: Transient resource aliasing
// ---------------------------------------------------------------------------

fn compute_aliasing(
    lifetimes: &[ResourceLifetime],
    images: &[ImageInfo],
    buffers: &[BufferInfo],
) -> Vec<AliasingGroup> {
    let num_images = images.len() as u16;

    // Pair each lifetime with an estimated size, then sort descending.
    let mut sized: Vec<(usize, u64)> = lifetimes
        .iter()
        .enumerate()
        .map(|(i, lt)| {
            let (ridx, is_img) = decode_unified_id(lt.resource_index, num_images);
            let size = if is_img {
                estimate_image_size(&images[ridx as usize])
            } else {
                buffers[ridx as usize].desc.size
            };
            (i, size)
        })
        .collect();
    sized.sort_by(|a, b| b.1.cmp(&a.1));

    // Greedy interval-based allocator.
    let mut groups: Vec<AliasingGroup> = Vec::new();
    let mut group_intervals: Vec<Vec<(usize, usize)>> = Vec::new();

    for (lt_idx, _) in sized {
        let lt = &lifetimes[lt_idx];
        let mut assigned = false;

        for (g, intervals) in group_intervals.iter_mut().enumerate() {
            let overlaps = intervals
                .iter()
                .any(|&(f, l)| !(lt.last_use < f || lt.first_use > l));
            if !overlaps {
                groups[g].resources.push(lt.resource_index);
                intervals.push((lt.first_use, lt.last_use));
                assigned = true;
                break;
            }
        }

        if !assigned {
            groups.push(AliasingGroup {
                resources: vec![lt.resource_index],
            });
            group_intervals.push(vec![(lt.first_use, lt.last_use)]);
        }
    }

    // Single-resource groups don't alias.
    groups.retain(|g| g.resources.len() >= 2);
    groups
}

/// Rough size estimate for an image (used only for aliasing sort order).
fn estimate_image_size(info: &ImageInfo) -> u64 {
    let d = &info.desc;
    let bpp = format_bytes_per_pixel(d.format) as u64;
    (d.extent.width as u64)
        * (d.extent.height as u64)
        * (d.extent.depth as u64)
        * bpp
        * (d.array_layers as u64)
}

fn format_bytes_per_pixel(format: vk::Format) -> u32 {
    #[allow(unreachable_patterns)]
    match format {
        vk::Format::R8_UNORM | vk::Format::R8_SNORM | vk::Format::R8_UINT
        | vk::Format::R8_SINT => 1,

        vk::Format::R8G8_UNORM | vk::Format::R8G8_SNORM | vk::Format::R16_SFLOAT
        | vk::Format::D16_UNORM => 2,

        vk::Format::R8G8B8A8_UNORM
        | vk::Format::R8G8B8A8_SRGB
        | vk::Format::B8G8R8A8_UNORM
        | vk::Format::B8G8R8A8_SRGB
        | vk::Format::R32_SFLOAT
        | vk::Format::R32_UINT
        | vk::Format::R32_SINT
        | vk::Format::R16G16_SFLOAT
        | vk::Format::D32_SFLOAT
        | vk::Format::D24_UNORM_S8_UINT => 4,

        vk::Format::R16G16B16A16_SFLOAT
        | vk::Format::R16G16B16A16_UNORM
        | vk::Format::R32G32_SFLOAT
        | vk::Format::R32G32_UINT => 8,

        vk::Format::R32G32B32A32_SFLOAT | vk::Format::R32G32B32A32_UINT => 16,

        // Conservative default.
        _ => 4,
    }
}

// ---------------------------------------------------------------------------
// Step 7: Barrier deduction
// ---------------------------------------------------------------------------

fn deduce_all_barriers(
    pass_order: &[usize],
    pass_accesses: &[SmallVec<[ResourceAccess; 8]>],
    passes: &[PassNode],
    images: &[ImageInfo],
) -> Vec<PassBarriers> {
    let mut result = vec![PassBarriers::default(); pass_order.len()];

    // Track last access per resource: (access, queue_type).
    // Key = (resource_index, is_image).
    let mut last_access: HashMap<(u16, bool), (ResourceAccess, QueueType)> = HashMap::new();

    for (pos, &pass_idx) in pass_order.iter().enumerate() {
        let queue = passes[pass_idx].queue;

        // Deduplicate: only create a barrier for the *first* access to each
        // resource within this pass (subsequent accesses are same-pass and
        // don't need inter-pass barriers).
        let mut first_seen: HashSet<(u16, bool)> = HashSet::new();
        // Track the last access per resource within this pass so the next
        // pass sees the correct source.
        let mut pass_last: HashMap<(u16, bool), ResourceAccess> = HashMap::new();

        for access in &pass_accesses[pass_idx] {
            let is_img = access_is_image(access);
            let rkey = (access.resource_index, is_img);

            if first_seen.insert(rkey) {
                let subresource = if is_img {
                    images
                        .get(access.resource_index as usize)
                        .and_then(|i| i.resource.subresource)
                } else {
                    None
                };

                if let Some((prev, prev_queue)) = last_access.get(&rkey) {
                    if *prev_queue != queue {
                        // Cross-queue ownership transfer.
                        let src_qf = queue_family_index(*prev_queue);
                        let dst_qf = queue_family_index(queue);
                        let transfer = deduce_queue_transfer(
                            prev, access, is_img, subresource, src_qf, dst_qf,
                        );
                        result[pos].transfers.push(transfer);
                    } else if let Some(barrier) =
                        deduce_barrier(prev, access, is_img, subresource)
                    {
                        result[pos].barriers.push(barrier);
                    }
                }
            }

            pass_last.insert(rkey, *access);
        }

        // Publish this pass's last access per resource for subsequent passes.
        for (rkey, acc) in pass_last {
            last_access.insert(rkey, (acc, queue));
        }

        merge_barriers(&mut result[pos].barriers);
    }

    result
}

/// Placeholder queue-family indices (real values come from the device at
/// execution time).
fn queue_family_index(qt: QueueType) -> u32 {
    match qt {
        QueueType::Graphics => 0,
        QueueType::AsyncCompute => 1,
        QueueType::Transfer => 2,
    }
}

// ---------------------------------------------------------------------------
// Step 8: Timeline semaphore insertion
// ---------------------------------------------------------------------------

fn insert_semaphores(
    pass_order: &[usize],
    passes: &[PassNode],
    adjacency: &[Vec<usize>],
    active: &[bool],
) -> Vec<SemaphoreOp> {
    // Map pass_idx → position in pass_order.
    let mut position_of = vec![0usize; passes.len()];
    for (pos, &pidx) in pass_order.iter().enumerate() {
        position_of[pidx] = pos;
    }

    let mut ops = Vec::new();
    let mut signal_counter = 0u64;

    for &pass_idx in pass_order {
        for &next in &adjacency[pass_idx] {
            if !active[next] {
                continue;
            }
            if passes[pass_idx].queue != passes[next].queue {
                signal_counter += 1;
                ops.push(SemaphoreOp {
                    signal_value: signal_counter,
                    signal_queue: passes[pass_idx].queue,
                    signal_after_pass: position_of[pass_idx],
                    wait_queue: passes[next].queue,
                    wait_before_pass: position_of[next],
                });
            }
        }
    }

    ops
}

// ---------------------------------------------------------------------------
// Step 9: Submission batches
// ---------------------------------------------------------------------------

fn build_batches(pass_order: &[usize], passes: &[PassNode]) -> Vec<SubmissionBatch> {
    if pass_order.is_empty() {
        return Vec::new();
    }

    let mut batches = Vec::new();
    let mut cur_queue = passes[pass_order[0]].queue;
    let mut cur_passes = vec![0usize];

    for (pos, &pidx) in pass_order.iter().enumerate().skip(1) {
        if passes[pidx].queue == cur_queue {
            cur_passes.push(pos);
        } else {
            batches.push(SubmissionBatch {
                queue: cur_queue,
                passes: std::mem::take(&mut cur_passes),
            });
            cur_queue = passes[pidx].queue;
            cur_passes.push(pos);
        }
    }

    batches.push(SubmissionBatch {
        queue: cur_queue,
        passes: cur_passes,
    });

    batches
}

// ---------------------------------------------------------------------------
// Utility helpers
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Deferred validation (GraphError)
// ---------------------------------------------------------------------------

/// Required access kind for a buffer referenced by a command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BufferAccessKind {
    Read,
    ReadIndirect,
    Write,
}

/// Collect all buffer handles and their required access from a command list.
fn buffers_used_by_commands(commands: &[RecordedCommand]) -> Vec<(BufferHandle, BufferAccessKind)> {
    let mut out = Vec::new();
    for cmd in commands {
        match cmd {
            RecordedCommand::DrawIndirect { buffer, .. }
            | RecordedCommand::DrawIndexedIndirect { buffer, .. } => {
                out.push((*buffer, BufferAccessKind::ReadIndirect));
            }
            RecordedCommand::DrawIndirectCount {
                command_buffer,
                count_buffer,
                ..
            } => {
                out.push((*command_buffer, BufferAccessKind::ReadIndirect));
                out.push((*count_buffer, BufferAccessKind::ReadIndirect));
            }
            RecordedCommand::DrawIndexedIndirectCount {
                command_buffer,
                count_buffer,
                ..
            } => {
                out.push((*command_buffer, BufferAccessKind::ReadIndirect));
                out.push((*count_buffer, BufferAccessKind::ReadIndirect));
            }
            RecordedCommand::DispatchIndirect { buffer, .. } => {
                out.push((*buffer, BufferAccessKind::Read));
            }
            RecordedCommand::FillBuffer { buffer, .. } => {
                out.push((*buffer, BufferAccessKind::Write));
            }
            RecordedCommand::CopyBuffer { src, dst, .. } => {
                out.push((*src, BufferAccessKind::Read));
                out.push((*dst, BufferAccessKind::Write));
            }
            _ => {}
        }
    }
    out
}

/// Command-resource consistency: every buffer used in a pass's commands must
/// be declared in the pass's resource accesses with the correct read/write.
fn validate_command_resources(
    passes: &[PassNode],
    active: &[bool],
    all_commands: &[Vec<RecordedCommand>],
    pass_accesses: &[SmallVec<[ResourceAccess; 8]>],
    buffers: &[BufferInfo],
) -> Result<(), GraphError> {
    let num_buffers = buffers.len() as u16;
    for (i, pass) in passes.iter().enumerate() {
        if !active[i] {
            continue;
        }
        let commands = &all_commands[i];
        let accesses = &pass_accesses[i];
        for (handle, required) in buffers_used_by_commands(commands) {
            if handle.index >= num_buffers {
                return Err(GraphError::UndeclaredCommandResource {
                    pass: pass.name,
                    resource: format!("buffer#{}", handle.index),
                });
            }
            let declared_write = accesses
                .iter()
                .any(|a| !access_is_image(a) && a.resource_index == handle.index && a.version_write.is_some());
            let declared_read = accesses
                .iter()
                .any(|a| !access_is_image(a) && a.resource_index == handle.index && a.version_read.is_some());
            let ok = match required {
                BufferAccessKind::Write => declared_write,
                BufferAccessKind::Read | BufferAccessKind::ReadIndirect => declared_read || declared_write,
            };
            if !ok {
                return Err(GraphError::UndeclaredCommandResource {
                    pass: pass.name,
                    resource: buffers[handle.index as usize].resource.name.to_string(),
                });
            }
        }
    }
    Ok(())
}

/// Classify a command as graphics, compute, transfer, or state-only.
fn command_queue_class(cmd: &RecordedCommand) -> Option<QueueType> {
    use RecordedCommand::*;
    match cmd {
        Draw { .. } | DrawIndexed { .. }
        | DrawIndirect { .. } | DrawIndexedIndirect { .. }
        | DrawIndirectCount { .. } | DrawIndexedIndirectCount { .. }
        | BindGraphicsPipeline(_) | SetViewport { .. } | SetScissor { .. } => Some(QueueType::Graphics),
        Dispatch { .. } | DispatchIndirect { .. } | BindComputePipeline(_) => Some(QueueType::AsyncCompute),
        FillBuffer { .. } | CopyBuffer { .. } => Some(QueueType::Transfer),
        BindDescriptorSets { .. } | BindVertexBuffers { .. } | BindIndexBuffer { .. }
        | PushConstants { .. } | DrawSlotPlaceholder(_) => None, // state_bind
    }
}

/// Command-queue compatibility: transfer pass only transfer+state_bind;
/// compute only compute+transfer+state_bind; graphics all.
fn validate_command_queue(
    passes: &[PassNode],
    active: &[bool],
    all_commands: &[Vec<RecordedCommand>],
) -> Result<(), GraphError> {
    for (i, pass) in passes.iter().enumerate() {
        if !active[i] {
            continue;
        }
        let commands = &all_commands[i];
        for cmd in commands {
            let Some(cmd_queue) = command_queue_class(cmd) else { continue };
            let allowed = match pass.queue {
                QueueType::Transfer => cmd_queue == QueueType::Transfer,
                QueueType::AsyncCompute => {
                    cmd_queue == QueueType::AsyncCompute || cmd_queue == QueueType::Transfer
                }
                QueueType::Graphics => true,
            };
            if !allowed {
                return Err(GraphError::CommandQueueMismatch {
                    pass: pass.name,
                    queue: format!("{:?}", pass.queue),
                    command: cmd_command_name(cmd),
                });
            }
        }
    }
    Ok(())
}

fn cmd_command_name(cmd: &RecordedCommand) -> &'static str {
    match cmd {
        RecordedCommand::Draw { .. } => "Draw",
        RecordedCommand::DrawIndexed { .. } => "DrawIndexed",
        RecordedCommand::DrawIndirect { .. } => "DrawIndirect",
        RecordedCommand::DrawIndexedIndirect { .. } => "DrawIndexedIndirect",
        RecordedCommand::DrawIndirectCount { .. } => "DrawIndirectCount",
        RecordedCommand::DrawIndexedIndirectCount { .. } => "DrawIndexedIndirectCount",
        RecordedCommand::Dispatch { .. } => "Dispatch",
        RecordedCommand::DispatchIndirect { .. } => "DispatchIndirect",
        RecordedCommand::FillBuffer { .. } => "FillBuffer",
        RecordedCommand::CopyBuffer { .. } => "CopyBuffer",
        RecordedCommand::BindGraphicsPipeline(_) => "BindGraphicsPipeline",
        RecordedCommand::BindComputePipeline(_) => "BindComputePipeline",
        _ => "command",
    }
}

/// Usage flag consistency: each resource access must have the descriptor
/// usage flags required for that access type.
fn validate_usage_flags(
    pass_accesses: &[SmallVec<[ResourceAccess; 8]>],
    _passes: &[PassNode],
    active: &[bool],
    images: &[ImageInfo],
    buffers: &[BufferInfo],
) -> Result<(), GraphError> {
    use ash::vk::{ImageUsageFlags, BufferUsageFlags};
    for (i, accesses) in pass_accesses.iter().enumerate() {
        if !active[i] {
            continue;
        }
        for access in accesses.iter() {
            if access_is_image(access) {
                let idx = access.resource_index as usize;
                let Some(info) = images.get(idx) else { continue };
                let usage = info.desc.usage;
                if access.layout == vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
                    && !usage.contains(ImageUsageFlags::COLOR_ATTACHMENT)
                {
                    return Err(GraphError::MissingUsageFlags {
                        resource: info.resource.name.to_string(),
                        usage: "color_attachment",
                    });
                }
                if access.layout == vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL
                    && !usage.contains(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
                {
                    return Err(GraphError::MissingUsageFlags {
                        resource: info.resource.name.to_string(),
                        usage: "depth_attachment",
                    });
                }
                if access.layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
                    && !usage.contains(ImageUsageFlags::SAMPLED)
                {
                    return Err(GraphError::MissingUsageFlags {
                        resource: info.resource.name.to_string(),
                        usage: "sample_image",
                    });
                }
                if access.layout == vk::ImageLayout::GENERAL
                    && access.access.contains(vk::AccessFlags2::SHADER_WRITE)
                    && !usage.contains(ImageUsageFlags::STORAGE)
                {
                    return Err(GraphError::MissingUsageFlags {
                        resource: info.resource.name.to_string(),
                        usage: "write_storage_image",
                    });
                }
            } else {
                let idx = access.resource_index as usize;
                let Some(info) = buffers.get(idx) else { continue };
                let usage = info.desc.usage;
                if access.access.contains(vk::AccessFlags2::INDIRECT_COMMAND_READ)
                    && !usage.contains(BufferUsageFlags::INDIRECT_BUFFER)
                {
                    return Err(GraphError::MissingUsageFlags {
                        resource: info.resource.name.to_string(),
                        usage: "read_indirect_buffer",
                    });
                }
                if access.access.contains(vk::AccessFlags2::SHADER_WRITE)
                    && !usage.contains(BufferUsageFlags::STORAGE_BUFFER)
                {
                    return Err(GraphError::MissingUsageFlags {
                        resource: info.resource.name.to_string(),
                        usage: "write_storage_buffer",
                    });
                }
                if (access.access.contains(vk::AccessFlags2::SHADER_READ)
                    || access.access.contains(vk::AccessFlags2::UNIFORM_READ))
                    && !usage.contains(BufferUsageFlags::STORAGE_BUFFER)
                    && !usage.contains(BufferUsageFlags::UNIFORM_BUFFER)
                {
                    return Err(GraphError::MissingUsageFlags {
                        resource: info.resource.name.to_string(),
                        usage: "read_buffer",
                    });
                }
            }
        }
    }
    Ok(())
}

/// Duplicate writes: a pass must not declare two writes to the same
/// (resource_index, version) (same resource version).
fn validate_duplicate_writes(
    pass_accesses: &[SmallVec<[ResourceAccess; 8]>],
    passes: &[PassNode],
    active: &[bool],
    images: &[ImageInfo],
    buffers: &[BufferInfo],
) -> Result<(), GraphError> {
    for (i, accesses) in pass_accesses.iter().enumerate() {
        if !active[i] {
            continue;
        }
        let pass = &passes[i];
        let mut written: HashMap<(u16, bool, u16), ()> = HashMap::new();
        for access in accesses.iter() {
            let Some(ver) = access.version_write else { continue };
            let is_img = access_is_image(access);
            let key = (access.resource_index, is_img, ver);
            if written.insert(key, ()).is_some() {
                let name = if is_img {
                    images
                        .get(access.resource_index as usize)
                        .map(|x| x.resource.name)
                        .unwrap_or("?")
                } else {
                    buffers
                        .get(access.resource_index as usize)
                        .map(|x| x.resource.name)
                        .unwrap_or("?")
                };
                return Err(GraphError::DuplicateWrite {
                    pass: pass.name,
                    resource: name.to_string(),
                });
            }
        }
    }
    Ok(())
}

fn check_duplicate_names(passes: &[PassNode]) {
    let mut seen = HashSet::new();
    for pass in passes {
        if !seen.insert(pass.name) {
            log::warn!("duplicate pass name: `{}`", pass.name);
        }
    }
}

/// Determine whether a [`ResourceAccess`] targets an image or a buffer.
///
/// Convention: buffer accesses always use `ImageLayout::UNDEFINED`; image
/// accesses always specify a concrete layout.
fn access_is_image(access: &ResourceAccess) -> bool {
    access.layout != vk::ImageLayout::UNDEFINED
}

/// Encode `(resource_index, is_image)` into a single `u16` for lifetime and
/// aliasing tracking.
///
/// Images keep their original index; buffers are offset by `num_images`.
fn unified_resource_id(index: u16, is_image: bool, num_images: u16) -> u16 {
    if is_image {
        index
    } else {
        num_images + index
    }
}

/// Decode a unified resource id back to `(original_index, is_image)`.
fn decode_unified_id(uid: u16, num_images: u16) -> (u16, bool) {
    if uid < num_images {
        (uid, true)
    } else {
        (uid - num_images, false)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::resource::*;

    // -- Test helpers -------------------------------------------------------

    fn make_pass(name: &'static str) -> PassNode {
        PassNode {
            name,
            queue: QueueType::Graphics,
            condition: PassCondition::Always,
            accesses: SmallVec::new(),
            color_attachments: SmallVec::new(),
            depth_attachment: None,
            commands: Vec::new(),
            draw_slots: SmallVec::new(),
        }
    }

    fn pass_with(name: &'static str, accesses: &[ResourceAccess]) -> PassNode {
        PassNode {
            name,
            queue: QueueType::Graphics,
            condition: PassCondition::Always,
            accesses: SmallVec::from_slice(accesses),
            color_attachments: SmallVec::new(),
            depth_attachment: None,
            commands: Vec::new(),
            draw_slots: SmallVec::new(),
        }
    }

    fn img_write(index: u16, read_ver: u16, write_ver: u16) -> ResourceAccess {
        ResourceAccess {
            resource_index: index,
            version_read: Some(read_ver),
            version_write: Some(write_ver),
            stage: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
            access: vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            condition: None,
        }
    }

    fn img_read(index: u16, version: u16) -> ResourceAccess {
        ResourceAccess {
            resource_index: index,
            version_read: Some(version),
            version_write: None,
            stage: vk::PipelineStageFlags2::FRAGMENT_SHADER,
            access: vk::AccessFlags2::SHADER_SAMPLED_READ,
            layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            condition: None,
        }
    }

    fn make_image(name: &'static str, kind: ResourceKind) -> ImageInfo {
        make_image_with_usage(name, kind, vk::ImageUsageFlags::COLOR_ATTACHMENT)
    }

    /// Image with usage flags suitable for both color attachment and sampled read.
    fn make_image_with_usage(
        name: &'static str,
        kind: ResourceKind,
        usage: vk::ImageUsageFlags,
    ) -> ImageInfo {
        ImageInfo {
            desc: ImageDesc {
                usage,
                ..ImageDesc::default()
            },
            resource: ResourceInfo {
                name,
                kind,
                current_version: 0,
                subresource: None,
                parent_index: None,
            },
        }
    }

    #[allow(dead_code)]
    fn make_buffer(name: &'static str, kind: ResourceKind) -> BufferInfo {
        BufferInfo {
            desc: BufferDesc {
                size: 1024,
                usage: vk::BufferUsageFlags::STORAGE_BUFFER,
            },
            resource: ResourceInfo {
                name,
                kind,
                current_version: 0,
                subresource: None,
                parent_index: None,
            },
        }
    }

    fn no_flags() -> Vec<FlagInfo> {
        Vec::new()
    }

    fn no_draw_slots() -> Vec<Vec<RecordedCommand>> {
        Vec::new()
    }

    // -----------------------------------------------------------------------
    // 1. Condition flag evaluation
    // -----------------------------------------------------------------------

    #[test]
    fn pass_disabled_when_flag_is_false() {
        let flag = ConditionFlag {
            index: 0,
            generation: 1,
        };
        let mut pass_a = make_pass("a");
        pass_a.condition = PassCondition::WhenFlag(flag);
        let pass_b = make_pass("b");

        let flags = vec![FlagInfo {
            name: "f",
            value: false,
        }];
        let (active, _) = evaluate_conditions(&[pass_a, pass_b], &flags);

        assert!(!active[0], "pass a should be disabled");
        assert!(active[1], "pass b should remain active");
    }

    #[test]
    fn pass_disabled_when_not_flag_is_true() {
        let flag = ConditionFlag {
            index: 0,
            generation: 1,
        };
        let mut pass = make_pass("a");
        pass.condition = PassCondition::WhenNotFlag(flag);

        let flags = vec![FlagInfo {
            name: "f",
            value: true,
        }];
        let (active, _) = evaluate_conditions(&[pass], &flags);

        assert!(!active[0]);
    }

    #[test]
    fn conditional_access_stripped_when_flag_false() {
        let flag = ConditionFlag {
            index: 0,
            generation: 1,
        };
        let mut pass = make_pass("a");
        pass.accesses.push(ResourceAccess {
            resource_index: 0,
            version_read: Some(0),
            version_write: None,
            stage: vk::PipelineStageFlags2::FRAGMENT_SHADER,
            access: vk::AccessFlags2::SHADER_SAMPLED_READ,
            layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            condition: Some(flag),
        });
        pass.accesses.push(img_read(1, 0));

        let flags = vec![FlagInfo {
            name: "f",
            value: false,
        }];
        let (active, accesses) = evaluate_conditions(&[pass], &flags);

        assert!(active[0]);
        assert_eq!(accesses[0].len(), 1, "conditional access should be stripped");
        assert_eq!(accesses[0][0].resource_index, 1);
    }

    #[test]
    fn transitive_disable_propagates() {
        // pass0 (disabled by flag) writes image 0 v1.
        // pass1 reads image 0 v1 (non-conditional) → should be transitively disabled.
        let flag = ConditionFlag {
            index: 0,
            generation: 1,
        };
        let mut p0 = pass_with("p0", &[img_write(0, 0, 1)]);
        p0.condition = PassCondition::WhenFlag(flag);
        let p1 = pass_with("p1", &[img_read(0, 1)]);

        let flags = vec![FlagInfo {
            name: "f",
            value: false,
        }];
        let (mut active, accesses) = evaluate_conditions(&[p0, p1], &flags);
        let writers = build_all_writers_map(&[
            pass_with("p0", &[img_write(0, 0, 1)]),
            pass_with("p1", &[img_read(0, 1)]),
        ]);
        transitive_disable(&mut active, &accesses, &writers);

        assert!(!active[0], "p0 disabled by flag");
        assert!(!active[1], "p1 transitively disabled");
    }

    // -----------------------------------------------------------------------
    // 2. DAG construction and topological sort
    // -----------------------------------------------------------------------

    #[test]
    fn basic_dag_and_topo_sort() {
        // p0 writes img0 v1, p1 reads v1 and writes v2, p2 reads v2 and
        // writes img1 v1 (imported → output, so it survives dead-pass elim).
        let images = vec![
            make_image_with_usage(
                "img",
                ResourceKind::Imported,
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::SAMPLED,
            ),
            make_image("out", ResourceKind::Imported),
        ];
        let buffers: Vec<BufferInfo> = Vec::new();

        let passes = vec![
            pass_with("p0", &[img_write(0, 0, 1)]),
            pass_with("p1", &[img_write(0, 1, 2)]),
            pass_with("p2", &[img_read(0, 2), img_write(1, 0, 1)]),
        ];

        let result = compile(&passes, &images, &buffers, &no_flags(), &no_draw_slots(), 1)
            .unwrap();

        let pos = |n: &str| {
            result
                .pass_order
                .iter()
                .position(|&i| passes[i].name == n)
                .unwrap()
        };
        assert!(pos("p0") < pos("p1"));
        assert!(pos("p1") < pos("p2"));
    }

    #[test]
    fn independent_passes_all_scheduled() {
        // Two passes that each write their own imported image — no ordering
        // between them, both should appear.
        let images = vec![
            make_image("a", ResourceKind::Imported),
            make_image("b", ResourceKind::Imported),
        ];
        let passes = vec![
            pass_with("pa", &[img_write(0, 0, 1)]),
            pass_with("pb", &[img_write(1, 0, 1)]),
        ];

        let result = compile(&passes, &images, &[], &no_flags(), &no_draw_slots(), 1).unwrap();
        assert_eq!(result.pass_order.len(), 2);
    }

    // -----------------------------------------------------------------------
    // 3. Dead-pass elimination
    // -----------------------------------------------------------------------

    #[test]
    fn dead_pass_is_eliminated() {
        let images = vec![
            make_image("output", ResourceKind::Imported),
            make_image("temp", ResourceKind::Transient),
        ];

        let passes = vec![
            pass_with("kept", &[img_write(0, 0, 1)]),
            pass_with("dead", &[img_write(1, 0, 1)]),
        ];

        let result = compile(&passes, &images, &[], &no_flags(), &no_draw_slots(), 1).unwrap();

        assert_eq!(result.pass_order.len(), 1);
        assert_eq!(passes[result.pass_order[0]].name, "kept");
    }

    #[test]
    fn transient_dependency_kept_when_feeding_output() {
        // p0 writes transient img1 v1, p1 reads img1 v1 and writes imported img0 v1.
        let images = vec![
            make_image("output", ResourceKind::Imported),
            make_image_with_usage(
                "temp",
                ResourceKind::Transient,
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::SAMPLED,
            ),
        ];
        let passes = vec![
            pass_with("p0", &[img_write(1, 0, 1)]),
            pass_with("p1", &[img_read(1, 1), img_write(0, 0, 1)]),
        ];

        let result = compile(&passes, &images, &[], &no_flags(), &no_draw_slots(), 1).unwrap();

        assert_eq!(result.pass_order.len(), 2);
    }

    // -----------------------------------------------------------------------
    // 4. Draw-slot splicing
    // -----------------------------------------------------------------------

    #[test]
    fn draw_slot_spliced_into_commands() {
        let mut pass = pass_with("render", &[img_write(0, 0, 1)]);
        let slot = crate::graph::resource::DrawSlot {
            pass_index: 0,
            slot_index: 0,
            generation: 1,
        };
        pass.commands
            .push(RecordedCommand::DrawSlotPlaceholder(slot));
        pass.draw_slots.push((0, false));

        let images = vec![make_image("out", ResourceKind::Imported)];
        let draw_slots = vec![vec![RecordedCommand::Draw {
            vertex_count: 3,
            instance_count: 1,
            first_vertex: 0,
            first_instance: 0,
        }]];

        let result = compile(&[pass], &images, &[], &no_flags(), &draw_slots, 1).unwrap();

        assert_eq!(result.pass_commands.len(), 1);
        assert!(matches!(
            result.pass_commands[0][0],
            RecordedCommand::Draw {
                vertex_count: 3,
                ..
            },
        ));
    }

    #[test]
    fn unfilled_draw_slot_returns_error() {
        let mut pass = pass_with("render", &[img_write(0, 0, 1)]);
        let slot = crate::graph::resource::DrawSlot {
            pass_index: 0,
            slot_index: 0,
            generation: 1,
        };
        pass.commands
            .push(RecordedCommand::DrawSlotPlaceholder(slot));
        pass.draw_slots.push((0, false));

        let images = vec![make_image("out", ResourceKind::Imported)];
        let err = compile(&[pass], &images, &[], &no_flags(), &no_draw_slots(), 1).unwrap_err();
        assert!(matches!(err, GraphError::UnfilledDrawSlot { .. }));
    }

    // -----------------------------------------------------------------------
    // 5. Barrier deduction
    // -----------------------------------------------------------------------

    #[test]
    fn barrier_inserted_for_write_then_read() {
        // writer writes img0 v1.  reader reads img0 v1 and writes img1 v1
        // (imported → output so it survives dead-pass elimination).
        let images = vec![
            make_image_with_usage(
                "img",
                ResourceKind::Imported,
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::SAMPLED,
            ),
            make_image("out", ResourceKind::Imported),
        ];

        let passes = vec![
            pass_with("writer", &[img_write(0, 0, 1)]),
            pass_with("reader", &[img_read(0, 1), img_write(1, 0, 1)]),
        ];

        let result = compile(&passes, &images, &[], &no_flags(), &no_draw_slots(), 1).unwrap();

        // Find position of the reader pass.
        let reader_pos = result
            .pass_order
            .iter()
            .position(|&i| passes[i].name == "reader")
            .unwrap();

        assert!(
            !result.barriers[reader_pos].barriers.is_empty(),
            "expected a barrier before the reader pass",
        );

        let b = &result.barriers[reader_pos].barriers[0];
        assert!(b.is_image);
        assert_eq!(b.src_stage, vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT);
        assert_eq!(b.dst_stage, vk::PipelineStageFlags2::FRAGMENT_SHADER);
        assert_eq!(b.old_layout, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        assert_eq!(b.new_layout, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
    }

    #[test]
    fn no_barrier_for_read_after_read_same_layout() {
        // Two readers of the same version with the same layout.  Each also
        // writes a separate imported image so neither is dead-eliminated.
        let images = vec![
            make_image_with_usage(
                "img",
                ResourceKind::Imported,
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::SAMPLED,
            ),
            make_image("out_a", ResourceKind::Imported),
            make_image("out_b", ResourceKind::Imported),
        ];

        let passes = vec![
            pass_with("writer", &[img_write(0, 0, 1)]),
            pass_with("reader_a", &[img_read(0, 1), img_write(1, 0, 1)]),
            pass_with("reader_b", &[img_read(0, 1), img_write(2, 0, 1)]),
        ];

        let result = compile(&passes, &images, &[], &no_flags(), &no_draw_slots(), 1).unwrap();

        // Count barriers on image 0 only: one write→read, the second
        // read→read (same layout) should produce nothing.
        let img0_barrier_count: usize = result
            .barriers
            .iter()
            .map(|pb| {
                pb.barriers
                    .iter()
                    .filter(|b| b.resource_index == 0)
                    .count()
            })
            .sum();
        assert_eq!(
            img0_barrier_count, 1,
            "only one barrier expected on img0 (write→read)",
        );
    }

    // -----------------------------------------------------------------------
    // 6. Submission batches
    // -----------------------------------------------------------------------

    #[test]
    fn single_queue_produces_single_batch() {
        let images = vec![
            make_image_with_usage(
                "img",
                ResourceKind::Imported,
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::SAMPLED,
            ),
            make_image("out", ResourceKind::Imported),
        ];
        let passes = vec![
            pass_with("a", &[img_write(0, 0, 1)]),
            pass_with("b", &[img_read(0, 1), img_write(1, 0, 1)]),
        ];

        let result = compile(&passes, &images, &[], &no_flags(), &no_draw_slots(), 1).unwrap();

        assert_eq!(result.batches.len(), 1);
        assert_eq!(result.batches[0].queue, QueueType::Graphics);
        assert_eq!(result.batches[0].passes.len(), 2);
    }

    // -----------------------------------------------------------------------
    // 7. Empty graph compiles successfully
    // -----------------------------------------------------------------------

    #[test]
    fn empty_graph_compiles() {
        let result = compile(&[], &[], &[], &no_flags(), &no_draw_slots(), 1).unwrap();

        assert!(result.pass_order.is_empty());
        assert!(result.barriers.is_empty());
        assert!(result.batches.is_empty());
    }
}
