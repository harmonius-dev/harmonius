//! Multi-draw-indirect optimization.
//!
//! After batch expansion, consecutive `DrawIndexed` commands that share the
//! same bound state can be collapsed into a single `vkCmdDrawIndexedIndirect`
//! call. This module provides the data structures and collapse logic for that
//! optimization.

use crate::graph::pass::RecordedCommand;

/// Mirrors `VkDrawIndexedIndirectCommand` for upload to GPU buffers.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

/// Size of a single `VkDrawIndexedIndirectCommand` in bytes.
pub const INDIRECT_COMMAND_SIZE: usize = std::mem::size_of::<DrawIndexedIndirectCommand>();

/// A batch of consecutive `DrawIndexed` commands that share identical bound
/// state and can be issued as a single `vkCmdDrawIndexedIndirect` call.
#[derive(Clone, Debug)]
pub struct IndirectBatch {
    /// The indirect commands to upload to a GPU buffer.
    pub commands: Vec<DrawIndexedIndirectCommand>,
    /// Index in the original command list where the state binding commands
    /// begin for this batch.
    pub state_bind_start: usize,
    /// Number of state binding commands (from the start index).
    pub state_bind_count: usize,
}

/// Returns `true` if `cmd` is a state-binding command (as opposed to a draw
/// or other action command).
fn is_state_bind(cmd: &RecordedCommand) -> bool {
    matches!(
        cmd,
        RecordedCommand::BindGraphicsPipeline(_)
            | RecordedCommand::BindComputePipeline(_)
            | RecordedCommand::BindDescriptorSets { .. }
            | RecordedCommand::BindVertexBuffers { .. }
            | RecordedCommand::BindIndexBuffer { .. }
            | RecordedCommand::PushConstants { .. }
            | RecordedCommand::SetViewport { .. }
            | RecordedCommand::SetScissor { .. }
    )
}

/// Scan a list of `RecordedCommand`s and identify runs of `DrawIndexed` that
/// can be collapsed into multi-draw-indirect calls.
///
/// Returns a list of [`IndirectBatch`]es. Commands not part of a batch are
/// left unchanged -- the caller uses the batches to decide which commands
/// to replace with indirect draws during execution.
///
/// A run of `DrawIndexed` commands is eligible for collapse when:
/// 1. They are consecutive in the command list (no intervening state changes).
/// 2. There are at least 2 `DrawIndexed` commands in the run (single draws
///    are not worth the indirect overhead).
pub fn collapse_to_indirect(commands: &[RecordedCommand]) -> Vec<IndirectBatch> {
    let mut batches = Vec::new();
    let len = commands.len();
    let mut i = 0;

    while i < len {
        // Phase 1: Consume any leading state-bind commands.
        let state_bind_start = i;
        while i < len && is_state_bind(&commands[i]) {
            i += 1;
        }
        let state_bind_count = i - state_bind_start;

        // Phase 2: Consume a run of consecutive DrawIndexed commands.
        let mut draw_commands = Vec::new();
        while i < len {
            if let RecordedCommand::DrawIndexed {
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            } = &commands[i]
            {
                draw_commands.push(DrawIndexedIndirectCommand {
                    index_count: *index_count,
                    instance_count: *instance_count,
                    first_index: *first_index,
                    vertex_offset: *vertex_offset,
                    first_instance: *first_instance,
                });
                i += 1;
            } else {
                break;
            }
        }

        // Phase 3: If we collected at least 2 draws, emit a batch.
        if draw_commands.len() >= 2 {
            batches.push(IndirectBatch {
                commands: draw_commands,
                state_bind_start,
                state_bind_count,
            });
        } else {
            // Not enough draws to justify indirect -- skip past whatever we
            // consumed so the outer loop makes progress.
            if draw_commands.is_empty() && state_bind_count == 0 {
                // We're sitting on a non-bind, non-DrawIndexed command; advance.
                i += 1;
            }
        }
    }

    batches
}

#[cfg(test)]
mod tests {
    use super::*;
    use ash::vk::{self, Handle};

    // ---- helpers ----

    fn bind_pipeline(pipeline: u64) -> RecordedCommand {
        RecordedCommand::BindGraphicsPipeline(<vk::Pipeline as Handle>::from_raw(pipeline))
    }

    fn draw_indexed(index_count: u32, first_index: u32) -> RecordedCommand {
        RecordedCommand::DrawIndexed {
            index_count,
            instance_count: 1,
            first_index,
            vertex_offset: 0,
            first_instance: 0,
        }
    }

    fn draw(vertex_count: u32) -> RecordedCommand {
        RecordedCommand::Draw {
            vertex_count,
            instance_count: 1,
            first_vertex: 0,
            first_instance: 0,
        }
    }

    // ---- struct layout ----

    #[test]
    fn indirect_command_size_is_20() {
        assert_eq!(INDIRECT_COMMAND_SIZE, 20);
    }

    // ---- collapse_to_indirect ----

    #[test]
    fn empty_input_yields_no_batches() {
        let batches = collapse_to_indirect(&[]);
        assert!(batches.is_empty());
    }

    #[test]
    fn single_draw_indexed_not_collapsed() {
        let cmds = vec![bind_pipeline(1), draw_indexed(36, 0)];
        let batches = collapse_to_indirect(&cmds);
        assert!(batches.is_empty());
    }

    #[test]
    fn two_consecutive_draw_indexed_collapsed() {
        let cmds = vec![
            bind_pipeline(1),
            draw_indexed(36, 0),
            draw_indexed(24, 36),
        ];
        let batches = collapse_to_indirect(&cmds);
        assert_eq!(batches.len(), 1);

        let b = &batches[0];
        assert_eq!(b.commands.len(), 2);
        assert_eq!(b.state_bind_start, 0);
        assert_eq!(b.state_bind_count, 1);

        assert_eq!(b.commands[0].index_count, 36);
        assert_eq!(b.commands[0].first_index, 0);
        assert_eq!(b.commands[1].index_count, 24);
        assert_eq!(b.commands[1].first_index, 36);
    }

    #[test]
    fn intervening_state_change_splits_batches() {
        let cmds = vec![
            bind_pipeline(1),
            draw_indexed(36, 0),
            draw_indexed(24, 36),
            bind_pipeline(2),
            draw_indexed(12, 0),
            draw_indexed(48, 12),
            draw_indexed(6, 60),
        ];
        let batches = collapse_to_indirect(&cmds);
        assert_eq!(batches.len(), 2);

        assert_eq!(batches[0].commands.len(), 2);
        assert_eq!(batches[0].state_bind_start, 0);
        assert_eq!(batches[0].state_bind_count, 1);

        assert_eq!(batches[1].commands.len(), 3);
        assert_eq!(batches[1].state_bind_start, 3);
        assert_eq!(batches[1].state_bind_count, 1);
    }

    #[test]
    fn non_draw_indexed_breaks_run() {
        let cmds = vec![
            bind_pipeline(1),
            draw_indexed(36, 0),
            draw(100), // non-DrawIndexed breaks the run
            draw_indexed(24, 0),
            draw_indexed(12, 24),
        ];
        let batches = collapse_to_indirect(&cmds);
        // First run: only 1 DrawIndexed -> not collapsed.
        // After the Draw, two DrawIndexed without preceding state binds -> collapsed.
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].commands.len(), 2);
        assert_eq!(batches[0].state_bind_count, 0);
    }

    #[test]
    fn draw_indexed_without_preceding_binds() {
        let cmds = vec![
            draw_indexed(36, 0),
            draw_indexed(24, 36),
            draw_indexed(12, 60),
        ];
        let batches = collapse_to_indirect(&cmds);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].commands.len(), 3);
        assert_eq!(batches[0].state_bind_start, 0);
        assert_eq!(batches[0].state_bind_count, 0);
    }

    #[test]
    fn only_state_binds_no_draws() {
        let cmds = vec![bind_pipeline(1), bind_pipeline(2)];
        let batches = collapse_to_indirect(&cmds);
        assert!(batches.is_empty());
    }

    #[test]
    fn is_state_bind_covers_all_bind_variants() {
        use smallvec::smallvec;

        let binds = vec![
            RecordedCommand::BindGraphicsPipeline(vk::Pipeline::null()),
            RecordedCommand::BindComputePipeline(vk::Pipeline::null()),
            RecordedCommand::BindDescriptorSets {
                bind_point: vk::PipelineBindPoint::GRAPHICS,
                layout: vk::PipelineLayout::null(),
                first_set: 0,
                sets: smallvec![],
                dynamic_offsets: smallvec![],
            },
            RecordedCommand::BindVertexBuffers {
                first_binding: 0,
                buffers: smallvec![],
                offsets: smallvec![],
            },
            RecordedCommand::BindIndexBuffer {
                buffer: vk::Buffer::null(),
                offset: 0,
                index_type: vk::IndexType::UINT16,
            },
            RecordedCommand::PushConstants {
                layout: vk::PipelineLayout::null(),
                stages: vk::ShaderStageFlags::VERTEX,
                offset: 0,
                data: smallvec![],
            },
            RecordedCommand::SetViewport {
                first: 0,
                viewports: smallvec![],
            },
            RecordedCommand::SetScissor {
                first: 0,
                scissors: smallvec![],
            },
        ];

        for cmd in &binds {
            assert!(is_state_bind(cmd), "expected state bind: {cmd:?}");
        }

        // Draw variants should not be state binds.
        assert!(!is_state_bind(&draw(1)));
        assert!(!is_state_bind(&draw_indexed(1, 0)));
    }
}
