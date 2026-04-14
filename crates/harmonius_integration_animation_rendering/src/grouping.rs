//! Instanced skinning batching via sort-only hot path (no hash maps).

use crate::handle::GpuBuffer;
use crate::handle::Handle;
use crate::types::ClipSetId;
use crate::types::InstancedSkinningRow;
use crate::types::SkeletonId;
use crate::types::SkinningDispatch;
use crate::types::SkinningMode;

#[cfg(test)]
use core::sync::atomic::AtomicUsize;
#[cfg(test)]
use core::sync::atomic::Ordering;

#[cfg(test)]
/// Incremented only when [`debug_record_hashmap_touch`] runs; grouping never calls it.
static GROUPING_HASHMAP_HOT_PATH_TOUCHES: AtomicUsize = AtomicUsize::new(0);

/// Test hook simulating a `HashMap` use on the hot path; must stay unused from grouping.
pub fn debug_record_hashmap_touch() {
    #[cfg(test)]
    {
        GROUPING_HASHMAP_HOT_PATH_TOUCHES.fetch_add(1, Ordering::Relaxed);
    }
}

/// Resets the hot-path `HashMap` instrumentation counter (tests).
pub fn reset_grouping_hashmap_touch_count() {
    #[cfg(test)]
    {
        GROUPING_HASHMAP_HOT_PATH_TOUCHES.store(0, Ordering::Relaxed);
    }
}

/// Returns the hot-path `HashMap` instrumentation counter.
///
/// Outside unit tests this always returns `0` (no process-global counter in release builds).
#[must_use]
pub fn grouping_hashmap_touch_count() -> usize {
    #[cfg(test)]
    {
        GROUPING_HASHMAP_HOT_PATH_TOUCHES.load(Ordering::Relaxed)
    }
    #[cfg(not(test))]
    {
        0
    }
}

/// Sorts rows by `(skeleton, clip, entity)` and merges contiguous runs into dispatches.
#[must_use]
pub fn build_skinning_dispatches_sorted(
    rows: &mut [InstancedSkinningRow],
) -> Vec<SkinningDispatch> {
    rows.sort_by(|a, b| {
        (a.skeleton_id, a.clip_set_id, a.entity).cmp(&(b.skeleton_id, b.clip_set_id, b.entity))
    });

    let mut out = Vec::new();
    let mut i = 0_usize;
    while i < rows.len() {
        let row = rows[i];
        let key = (row.skeleton_id, row.clip_set_id);
        let mut count = 1_u32;
        let arena = row.arena_buffer;
        let bones = row.bone_count_per_instance;
        let mode = row.mode;
        i += 1;
        while i < rows.len() {
            let next = rows[i];
            let next_key = (next.skeleton_id, next.clip_set_id);
            if next_key != key {
                break;
            }
            if next.arena_buffer != arena
                || next.bone_count_per_instance != bones
                || next.mode != mode
            {
                break;
            }
            count = count.saturating_add(1);
            i += 1;
        }
        out.push(SkinningDispatch {
            arena_buffer: arena,
            instance_count: count,
            bone_count_per_instance: bones,
            mode,
        });
    }
    out
}

/// Convenience builder for tests covering IR-1.4.5 batching.
#[must_use]
pub fn make_row(
    skeleton_id: u32,
    clip_set_id: u32,
    entity: u32,
    arena_buffer: Handle<GpuBuffer>,
    bone_count_per_instance: u16,
    mode: SkinningMode,
) -> InstancedSkinningRow {
    InstancedSkinningRow {
        skeleton_id: SkeletonId(skeleton_id),
        clip_set_id: ClipSetId(clip_set_id),
        entity: crate::types::EntityId(entity),
        arena_buffer,
        bone_count_per_instance,
        mode,
    }
}
