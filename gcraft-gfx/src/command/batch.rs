//! Draw command batching and sorting.
//!
//! This module provides the [`Batchable`] trait and two fill functions
//! ([`fill_opaque`] and [`fill_transparent`]) that sort user-provided draw
//! commands, then expand them into [`RecordedCommand`]s with state-change
//! minimization.
//!
//! # How it works
//!
//! Each draw command carries a *batch key* — a `u64` that encodes the full
//! GPU state it needs (pipeline, descriptor sets, vertex buffers, etc.).
//! The fill functions sort commands so that consecutive entries share the
//! same key, then only emit `bind_state()` calls at key transitions.  This
//! avoids redundant `vkCmdBind*` calls and is one of the simplest and most
//! effective CPU-side rendering optimisations.
//!
//! # Opaque vs. transparent
//!
//! * **Opaque** draws are sorted by batch key to maximise state batching.
//!   Draw order doesn't affect correctness because the depth buffer resolves
//!   visibility.
//!
//! * **Transparent** draws are sorted by distance (back-to-front) for correct
//!   alpha blending.  State batching still happens within runs of equal
//!   distance, but the primary sort criterion is visual correctness.

use crate::graph::pass::RecordedCommand;

// ---------------------------------------------------------------------------
// DrawParams
// ---------------------------------------------------------------------------

/// Parameters for a single draw call.
///
/// This is the minimal information needed to emit either a `vkCmdDraw` or a
/// `vkCmdDrawIndexed` command.  The [`Batchable`] trait returns this from
/// [`Batchable::draw_params`] so the fill functions can convert it into a
/// [`RecordedCommand`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawParams {
    /// Non-indexed draw (`vkCmdDraw`).
    Draw {
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    },
    /// Indexed draw (`vkCmdDrawIndexed`).
    Indexed {
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    },
}

// ---------------------------------------------------------------------------
// Batchable trait
// ---------------------------------------------------------------------------

/// Trait for draw commands that can be sorted and batched.
///
/// The batch key determines grouping: consecutive commands with equal batch
/// keys share GPU state bindings, avoiding redundant `vkCmdBind*` calls.
///
/// Implementations define what "same state" means for their rendering path.
/// The only contract: if `a.batch_key() == b.batch_key()`, then `a` and `b`
/// must require identical GPU state (same pipeline, descriptors, vertex
/// buffers, etc.).  Violating this produces incorrect rendering, not UB.
///
/// # Example
///
/// ```ignore
/// struct MeshDraw {
///     pipeline: vk::Pipeline,
///     descriptor_set: vk::DescriptorSet,
///     vertex_buffer: vk::Buffer,
///     index_count: u32,
///     // ...
/// }
///
/// impl Batchable for MeshDraw {
///     fn batch_key(&self) -> u64 {
///         // Combine pipeline + descriptor set into a single key.
///         // Equal keys guarantee identical GPU state.
///         hash(self.pipeline, self.descriptor_set)
///     }
///
///     fn bind_state(&self, commands: &mut Vec<RecordedCommand>) {
///         commands.push(RecordedCommand::BindGraphicsPipeline(self.pipeline));
///         // ... bind descriptor sets, vertex buffers, etc.
///     }
///
///     fn draw_params(&self) -> DrawParams {
///         DrawParams::Indexed { /* ... */ }
///     }
/// }
/// ```
pub trait Batchable {
    /// Key for batching.  Equal keys mean identical GPU state.
    ///
    /// The key must be deterministic: calling `batch_key()` on the same
    /// command must always return the same value.  Keys are compared with
    /// `==`; no ordering relationship is required beyond what `sort` uses.
    fn batch_key(&self) -> u64;

    /// Emit [`RecordedCommand`]s to bind this command's GPU state.
    ///
    /// Called only when `batch_key()` differs from the previous command in
    /// the sorted sequence.  Implementations should push pipeline binds,
    /// descriptor set binds, vertex/index buffer binds, push constants —
    /// anything needed to set up the GPU state for this command's draws.
    fn bind_state(&self, commands: &mut Vec<RecordedCommand>);

    /// The draw call parameters for this command.
    ///
    /// The fill functions convert this into a [`RecordedCommand::Draw`] or
    /// [`RecordedCommand::DrawIndexed`].
    fn draw_params(&self) -> DrawParams;

    /// Camera distance for back-to-front sorting of transparent draws.
    ///
    /// Returns `0.0` by default, which is correct for opaque draws where
    /// distance is unused.  Override this for transparent draw commands to
    /// return the signed distance from the camera (larger = farther away).
    fn distance(&self) -> f32 {
        0.0
    }
}

// ---------------------------------------------------------------------------
// Fill functions
// ---------------------------------------------------------------------------

/// Expand a single draw command into a [`RecordedCommand`].
fn emit_draw(params: DrawParams) -> RecordedCommand {
    match params {
        DrawParams::Draw {
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        } => RecordedCommand::Draw {
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        },
        DrawParams::Indexed {
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        } => RecordedCommand::DrawIndexed {
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        },
    }
}

/// Expand a sorted command slice into [`RecordedCommand`]s with state-change
/// minimisation.
///
/// Iterates the (already sorted) commands, emitting `bind_state()` only when
/// the batch key changes.  Each command always emits its draw call.
fn expand<T: Batchable>(commands: &[T]) -> Vec<RecordedCommand> {
    // Heuristic: each command produces ~2 recorded commands on average
    // (one state bind amortised + one draw).
    let mut recorded = Vec::with_capacity(commands.len() * 2);
    let mut prev_key: u64 = u64::MAX;

    for cmd in commands {
        let key = cmd.batch_key();
        if key != prev_key {
            cmd.bind_state(&mut recorded);
            prev_key = key;
        }
        recorded.push(emit_draw(cmd.draw_params()));
    }

    recorded
}

/// Fill an opaque draw slot.
///
/// Sorts `commands` by [`Batchable::batch_key`] to maximise GPU state
/// batching, then expands them into [`RecordedCommand`]s with state-change
/// minimisation.
///
/// # Sorting strategy
///
/// Opaque geometry is depth-tested, so draw order doesn't affect visual
/// correctness.  Sorting by batch key groups commands that share the same
/// pipeline, descriptor sets, and vertex buffers, minimising the number of
/// `vkCmdBind*` calls between draws.
///
/// # Returns
///
/// A `Vec<RecordedCommand>` containing interleaved state binds and draw
/// calls, ready to be stored in a draw slot and later recorded into a
/// Vulkan command buffer.
pub fn fill_opaque<T: Batchable>(commands: &mut [T]) -> Vec<RecordedCommand> {
    // 1. Sort by batch key for maximum state batching.
    commands.sort_unstable_by_key(|c| c.batch_key());

    // 2. Expand with state-change tracking.
    expand(commands)
}

/// Fill a transparent draw slot.
///
/// Sorts `commands` by [`Batchable::distance`] in descending order
/// (back-to-front) for correct alpha blending, then expands them into
/// [`RecordedCommand`]s with state-change minimisation.
///
/// # Sorting strategy
///
/// Transparent geometry must be drawn back-to-front so that alpha blending
/// composites correctly.  The primary sort key is `distance()` (descending);
/// state batching still occurs within runs of equal distance, but visual
/// correctness takes priority over GPU state efficiency.
///
/// # Returns
///
/// A `Vec<RecordedCommand>` containing interleaved state binds and draw
/// calls, ready to be stored in a draw slot and later recorded into a
/// Vulkan command buffer.
pub fn fill_transparent<T: Batchable>(commands: &mut [T]) -> Vec<RecordedCommand> {
    // 1. Sort by distance descending (back-to-front).
    commands.sort_unstable_by(|a, b| b.distance().total_cmp(&a.distance()));

    // 2. Expand with state-change tracking.
    expand(commands)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use ash::vk::Handle;

    /// Minimal batchable draw for testing.
    struct TestDraw {
        key: u64,
        dist: f32,
        vertex_count: u32,
    }

    impl Batchable for TestDraw {
        fn batch_key(&self) -> u64 {
            self.key
        }

        fn bind_state(&self, commands: &mut Vec<RecordedCommand>) {
            // Use a deterministic pipeline handle derived from the key so we
            // can verify which bind calls were emitted.
            commands.push(RecordedCommand::BindGraphicsPipeline(
                <ash::vk::Pipeline as Handle>::from_raw(self.key),
            ));
        }

        fn draw_params(&self) -> DrawParams {
            DrawParams::Draw {
                vertex_count: self.vertex_count,
                instance_count: 1,
                first_vertex: 0,
                first_instance: 0,
            }
        }

        fn distance(&self) -> f32 {
            self.dist
        }
    }

    #[test]
    fn opaque_groups_by_batch_key() {
        let mut draws = vec![
            TestDraw { key: 2, dist: 0.0, vertex_count: 30 },
            TestDraw { key: 1, dist: 0.0, vertex_count: 10 },
            TestDraw { key: 2, dist: 0.0, vertex_count: 40 },
            TestDraw { key: 1, dist: 0.0, vertex_count: 20 },
        ];

        let recorded = fill_opaque(&mut draws);

        // Expected: bind(1), draw(10), draw(20), bind(2), draw(30), draw(40)
        assert_eq!(recorded.len(), 6);

        // First group: key 1
        assert!(matches!(recorded[0], RecordedCommand::BindGraphicsPipeline(_)));
        assert!(matches!(recorded[1], RecordedCommand::Draw { vertex_count: 10, .. }));
        assert!(matches!(recorded[2], RecordedCommand::Draw { vertex_count: 20, .. }));

        // Second group: key 2
        assert!(matches!(recorded[3], RecordedCommand::BindGraphicsPipeline(_)));
        assert!(matches!(recorded[4], RecordedCommand::Draw { vertex_count: 30, .. }));
        assert!(matches!(recorded[5], RecordedCommand::Draw { vertex_count: 40, .. }));
    }

    #[test]
    fn transparent_sorts_back_to_front() {
        let mut draws = vec![
            TestDraw { key: 1, dist: 5.0, vertex_count: 10 },
            TestDraw { key: 2, dist: 20.0, vertex_count: 20 },
            TestDraw { key: 1, dist: 10.0, vertex_count: 30 },
        ];

        let recorded = fill_transparent(&mut draws);

        // Expected order by distance desc: 20.0, 10.0, 5.0
        // Keys after sort: 2, 1, 1
        // So: bind(2), draw(20), bind(1), draw(30), draw(10)
        assert_eq!(recorded.len(), 5);

        assert!(matches!(recorded[0], RecordedCommand::BindGraphicsPipeline(_)));
        assert!(matches!(recorded[1], RecordedCommand::Draw { vertex_count: 20, .. }));
        assert!(matches!(recorded[2], RecordedCommand::BindGraphicsPipeline(_)));
        assert!(matches!(recorded[3], RecordedCommand::Draw { vertex_count: 30, .. }));
        assert!(matches!(recorded[4], RecordedCommand::Draw { vertex_count: 10, .. }));
    }

    #[test]
    fn empty_input_produces_empty_output() {
        let mut draws: Vec<TestDraw> = Vec::new();
        assert!(fill_opaque(&mut draws).is_empty());
        assert!(fill_transparent(&mut draws).is_empty());
    }

    #[test]
    fn single_command_emits_bind_and_draw() {
        let mut draws = vec![TestDraw { key: 42, dist: 0.0, vertex_count: 6 }];

        let recorded = fill_opaque(&mut draws);

        assert_eq!(recorded.len(), 2);
        assert!(matches!(recorded[0], RecordedCommand::BindGraphicsPipeline(_)));
        assert!(matches!(recorded[1], RecordedCommand::Draw { vertex_count: 6, .. }));
    }

    #[test]
    fn all_same_key_binds_once() {
        let mut draws = vec![
            TestDraw { key: 7, dist: 0.0, vertex_count: 1 },
            TestDraw { key: 7, dist: 0.0, vertex_count: 2 },
            TestDraw { key: 7, dist: 0.0, vertex_count: 3 },
        ];

        let recorded = fill_opaque(&mut draws);

        // One bind + three draws
        assert_eq!(recorded.len(), 4);
        assert!(matches!(recorded[0], RecordedCommand::BindGraphicsPipeline(_)));
        for r in &recorded[1..] {
            assert!(matches!(r, RecordedCommand::Draw { .. }));
        }
    }

    /// Test that indexed draws are correctly emitted.
    struct IndexedTestDraw {
        key: u64,
        index_count: u32,
    }

    impl Batchable for IndexedTestDraw {
        fn batch_key(&self) -> u64 {
            self.key
        }

        fn bind_state(&self, commands: &mut Vec<RecordedCommand>) {
            commands.push(RecordedCommand::BindGraphicsPipeline(
                <ash::vk::Pipeline as Handle>::from_raw(self.key),
            ));
        }

        fn draw_params(&self) -> DrawParams {
            DrawParams::Indexed {
                index_count: self.index_count,
                instance_count: 1,
                first_index: 0,
                vertex_offset: 0,
                first_instance: 0,
            }
        }
    }

    #[test]
    fn indexed_draws_emit_draw_indexed() {
        let mut draws = vec![
            IndexedTestDraw { key: 1, index_count: 36 },
            IndexedTestDraw { key: 1, index_count: 24 },
        ];

        let recorded = fill_opaque(&mut draws);

        assert_eq!(recorded.len(), 3);
        assert!(matches!(recorded[0], RecordedCommand::BindGraphicsPipeline(_)));
        assert!(matches!(
            recorded[1],
            RecordedCommand::DrawIndexed { index_count: 36, .. }
        ));
        assert!(matches!(
            recorded[2],
            RecordedCommand::DrawIndexed { index_count: 24, .. }
        ));
    }
}
