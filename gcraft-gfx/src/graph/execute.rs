//! Execution pipeline for the compiled render graph.
//!
//! This module provides the types and functions needed to replay recorded GPU
//! commands from a compiled render graph, insert the deduced Vulkan barriers,
//! and submit work to the appropriate hardware queues.
//!
//! # Safety invariants cited in this module
//!
//! - **S1**: Barriers deduced from the graph guarantee correct memory and
//!   execution dependencies between passes.
//! - **S5**: The command buffer is in the recording state when command
//!   recording functions are called.
//! - **S6**: The `ash::Device` handle (and any derived Vulkan objects) is
//!   valid and the device has not been destroyed.

use ash::vk;

use crate::graph::barrier::PassBarriers;
use crate::graph::pass::RecordedCommand;
use crate::graph::resource::{BufferHandle, ImageHandle};

// ---------------------------------------------------------------------------
// PhysicalResourceMap
// ---------------------------------------------------------------------------

/// Maps logical resource indices (as used by the render graph) to physical
/// Vulkan handles obtained from the
/// [`ResourcePool`](crate::resource::pool::ResourcePool).
///
/// During execution, the graph acquires concrete `vk::Image` and `vk::Buffer`
/// handles from the pool and stores them here so that [`replay_commands`] and
/// [`record_barriers`] can translate graph-level resource references into real
/// Vulkan objects.
pub(crate) struct PhysicalResourceMap {
    /// Physical images, indexed by the graph's logical resource index.
    pub images: Vec<vk::Image>,
    /// Physical buffers, indexed by the graph's logical resource index.
    pub buffers: Vec<vk::Buffer>,
}

impl PhysicalResourceMap {
    /// Resolve a logical [`ImageHandle`] to its physical `vk::Image`.
    ///
    /// # Panics
    ///
    /// Panics if `handle.index` is out of bounds.
    pub fn resolve_image(&self, handle: ImageHandle) -> vk::Image {
        self.images[handle.index as usize]
    }

    /// Resolve a logical [`BufferHandle`] to its physical `vk::Buffer`.
    ///
    /// # Panics
    ///
    /// Panics if `handle.index` is out of bounds.
    pub fn resolve_buffer(&self, handle: BufferHandle) -> vk::Buffer {
        self.buffers[handle.index as usize]
    }
}

// ---------------------------------------------------------------------------
// replay_commands
// ---------------------------------------------------------------------------

/// Replay recorded GPU commands by issuing the corresponding Vulkan calls.
///
/// This is the core execution loop: each [`RecordedCommand`] variant is
/// translated into the appropriate `ash::Device::cmd_*` FFI call.  Commands
/// that reference graph-level [`BufferHandle`]s are resolved through
/// `physical`; commands that already carry raw `vk::Buffer` / `vk::Pipeline`
/// handles are forwarded directly.
///
/// # Safety
///
/// - `cmd` must be a command buffer in the **recording** state (S5).
/// - `device` must be a valid, non-destroyed `ash::Device` (S6).
/// - All `vk::Pipeline`, `vk::PipelineLayout`, `vk::DescriptorSet`, and
///   other raw Vulkan handles stored in the commands must still be valid.
/// - Physical resources referenced via [`BufferHandle`] must have been
///   resolved in `physical` and the underlying buffers must be valid.
/// - Proper barriers must have been recorded before this call so that buffer
///   and image contents are available (S1).
pub(crate) unsafe fn replay_commands(
    device: &ash::Device,
    cmd: vk::CommandBuffer,
    commands: &[RecordedCommand],
    physical: &PhysicalResourceMap,
) {
    for command in commands {
        match command {
            // -- State binding --------------------------------------------------

            RecordedCommand::BindGraphicsPipeline(pipeline) => {
                // SAFETY: S5 (cmd recording), S6 (device valid)
                unsafe {
                    device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, *pipeline);
                }
            }

            RecordedCommand::BindComputePipeline(pipeline) => {
                // SAFETY: S5, S6
                unsafe {
                    device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, *pipeline);
                }
            }

            RecordedCommand::BindDescriptorSets {
                bind_point,
                layout,
                first_set,
                sets,
                dynamic_offsets,
            } => {
                // SAFETY: S5, S6
                unsafe {
                    device.cmd_bind_descriptor_sets(
                        cmd,
                        *bind_point,
                        *layout,
                        *first_set,
                        sets,
                        dynamic_offsets,
                    );
                }
            }

            RecordedCommand::BindVertexBuffers {
                first_binding,
                buffers,
                offsets,
            } => {
                // SAFETY: S5, S6
                unsafe {
                    device.cmd_bind_vertex_buffers(cmd, *first_binding, buffers, offsets);
                }
            }

            RecordedCommand::BindIndexBuffer {
                buffer,
                offset,
                index_type,
            } => {
                // SAFETY: S5, S6
                unsafe {
                    device.cmd_bind_index_buffer(cmd, *buffer, *offset, *index_type);
                }
            }

            RecordedCommand::PushConstants {
                layout,
                stages,
                offset,
                data,
            } => {
                // SAFETY: S5, S6
                unsafe {
                    device.cmd_push_constants(cmd, *layout, *stages, *offset, data);
                }
            }

            RecordedCommand::SetViewport { first, viewports } => {
                // SAFETY: S5, S6
                unsafe {
                    device.cmd_set_viewport(cmd, *first, viewports);
                }
            }

            RecordedCommand::SetScissor { first, scissors } => {
                // SAFETY: S5, S6
                unsafe {
                    device.cmd_set_scissor(cmd, *first, scissors);
                }
            }

            // -- Direct draws ---------------------------------------------------

            RecordedCommand::Draw {
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            } => {
                // SAFETY: S5, S6
                unsafe {
                    device.cmd_draw(
                        cmd,
                        *vertex_count,
                        *instance_count,
                        *first_vertex,
                        *first_instance,
                    );
                }
            }

            RecordedCommand::DrawIndexed {
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            } => {
                // SAFETY: S5, S6
                unsafe {
                    device.cmd_draw_indexed(
                        cmd,
                        *index_count,
                        *instance_count,
                        *first_index,
                        *vertex_offset,
                        *first_instance,
                    );
                }
            }

            // -- Indirect draws -------------------------------------------------

            RecordedCommand::DrawIndirect {
                buffer,
                offset,
                draw_count,
                stride,
            } => {
                let vk_buf = physical.resolve_buffer(*buffer);
                // SAFETY: S1 (barriers ensure buffer ready), S5, S6
                unsafe {
                    device.cmd_draw_indirect(cmd, vk_buf, *offset, *draw_count, *stride);
                }
            }

            RecordedCommand::DrawIndexedIndirect {
                buffer,
                offset,
                draw_count,
                stride,
            } => {
                let vk_buf = physical.resolve_buffer(*buffer);
                // SAFETY: S1, S5, S6
                unsafe {
                    device.cmd_draw_indexed_indirect(
                        cmd, vk_buf, *offset, *draw_count, *stride,
                    );
                }
            }

            RecordedCommand::DrawIndirectCount {
                command_buffer,
                command_offset,
                count_buffer,
                count_offset,
                max_draw_count,
                stride,
            } => {
                let cmd_vk = physical.resolve_buffer(*command_buffer);
                let cnt_vk = physical.resolve_buffer(*count_buffer);
                // SAFETY: S1, S5, S6
                unsafe {
                    device.cmd_draw_indirect_count(
                        cmd,
                        cmd_vk,
                        *command_offset,
                        cnt_vk,
                        *count_offset,
                        *max_draw_count,
                        *stride,
                    );
                }
            }

            RecordedCommand::DrawIndexedIndirectCount {
                command_buffer,
                command_offset,
                count_buffer,
                count_offset,
                max_draw_count,
                stride,
            } => {
                let cmd_vk = physical.resolve_buffer(*command_buffer);
                let cnt_vk = physical.resolve_buffer(*count_buffer);
                // SAFETY: S1, S5, S6
                unsafe {
                    device.cmd_draw_indexed_indirect_count(
                        cmd,
                        cmd_vk,
                        *command_offset,
                        cnt_vk,
                        *count_offset,
                        *max_draw_count,
                        *stride,
                    );
                }
            }

            // -- Compute --------------------------------------------------------

            RecordedCommand::Dispatch { x, y, z } => {
                // SAFETY: S5, S6
                unsafe {
                    device.cmd_dispatch(cmd, *x, *y, *z);
                }
            }

            RecordedCommand::DispatchIndirect { buffer, offset } => {
                let vk_buf = physical.resolve_buffer(*buffer);
                // SAFETY: S1, S5, S6
                unsafe {
                    device.cmd_dispatch_indirect(cmd, vk_buf, *offset);
                }
            }

            // -- Transfer -------------------------------------------------------

            RecordedCommand::FillBuffer {
                buffer,
                offset,
                size,
                data,
            } => {
                let vk_buf = physical.resolve_buffer(*buffer);
                // SAFETY: S1, S5, S6
                unsafe {
                    device.cmd_fill_buffer(cmd, vk_buf, *offset, *size, *data);
                }
            }

            RecordedCommand::CopyBuffer {
                src,
                src_offset,
                dst,
                dst_offset,
                size,
            } => {
                let src_vk = physical.resolve_buffer(*src);
                let dst_vk = physical.resolve_buffer(*dst);
                let region = vk::BufferCopy {
                    src_offset: *src_offset,
                    dst_offset: *dst_offset,
                    size: *size,
                };
                // SAFETY: S1, S5, S6
                unsafe {
                    device.cmd_copy_buffer(cmd, src_vk, dst_vk, &[region]);
                }
            }

            // -- Placeholder (must not survive compilation) ---------------------

            RecordedCommand::DrawSlotPlaceholder(_) => {
                unreachable!("DrawSlotPlaceholder should have been spliced during compile()");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// record_barriers
// ---------------------------------------------------------------------------

/// Convert deduced [`PassBarriers`] into Vulkan 1.3 pipeline barrier commands.
///
/// Emits a single `vkCmdPipelineBarrier2` call containing all image and buffer
/// memory barriers for the pass.  Queue ownership transfers stored in
/// [`PassBarriers::transfers`] are **not** emitted here — they require split
/// handling (release on the source queue, acquire on the destination queue)
/// orchestrated by the higher-level execution loop.
///
/// # Safety
///
/// - `cmd` must be in the recording state (S5).
/// - `device` must be valid (S6).
/// - All resources referenced in barriers must be valid Vulkan objects whose
///   physical handles are present in `physical`.
/// - The barriers must have been correctly deduced from the graph's dataflow
///   analysis (S1).
pub(crate) unsafe fn record_barriers(
    device: &ash::Device,
    cmd: vk::CommandBuffer,
    barriers: &PassBarriers,
    physical: &PhysicalResourceMap,
    _is_image_resource: &dyn Fn(u16) -> bool,
) {
    if barriers.barriers.is_empty() && barriers.transfers.is_empty() {
        return;
    }

    let mut image_barriers = Vec::new();
    let mut buffer_barriers = Vec::new();

    for b in &barriers.barriers {
        if b.is_image {
            let image = physical.images[b.resource_index as usize];
            let subresource_range = b
                .subresource
                .map(vk::ImageSubresourceRange::from)
                .unwrap_or(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: vk::REMAINING_MIP_LEVELS,
                    base_array_layer: 0,
                    layer_count: vk::REMAINING_ARRAY_LAYERS,
                });

            image_barriers.push(
                vk::ImageMemoryBarrier2::default()
                    .src_stage_mask(b.src_stage)
                    .src_access_mask(b.src_access)
                    .dst_stage_mask(b.dst_stage)
                    .dst_access_mask(b.dst_access)
                    .old_layout(b.old_layout)
                    .new_layout(b.new_layout)
                    .src_queue_family_index(b.src_queue_family)
                    .dst_queue_family_index(b.dst_queue_family)
                    .image(image)
                    .subresource_range(subresource_range),
            );
        } else {
            let buffer = physical.buffers[b.resource_index as usize];

            buffer_barriers.push(
                vk::BufferMemoryBarrier2::default()
                    .src_stage_mask(b.src_stage)
                    .src_access_mask(b.src_access)
                    .dst_stage_mask(b.dst_stage)
                    .dst_access_mask(b.dst_access)
                    .src_queue_family_index(b.src_queue_family)
                    .dst_queue_family_index(b.dst_queue_family)
                    .buffer(buffer)
                    .offset(0)
                    .size(vk::WHOLE_SIZE),
            );
        }
    }

    let dep_info = vk::DependencyInfo::default()
        .image_memory_barriers(&image_barriers)
        .buffer_memory_barriers(&buffer_barriers);

    // SAFETY: S1 (barriers deduced from graph), S5 (cmd recording), S6 (device valid)
    unsafe {
        device.cmd_pipeline_barrier2(cmd, &dep_info);
    }
}

// ---------------------------------------------------------------------------
// Execution skeleton (TODO: implement once CompiledGraph is defined)
// ---------------------------------------------------------------------------

// The full execution method will be implemented as `impl CompiledGraph` once
// the `compile` module defines `CompiledGraph`.  The intended structure is
// outlined below.
//
// ```
// impl CompiledGraph {
//     pub fn execute(
//         &self,
//         device: &crate::device::DeviceContext,
//         pool: &mut crate::resource::pool::ResourcePool,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//
//         // ---------------------------------------------------------------
//         // 1. Allocate physical resources from the pool
//         // ---------------------------------------------------------------
//         //
//         // Walk `self.pass_order` and, for each resource declared in the
//         // graph, call `pool.acquire_image` / `pool.acquire_buffer` to
//         // obtain concrete Vulkan handles.  Store them in a
//         // `PhysicalResourceMap`.
//         //
//         // let mut physical = PhysicalResourceMap {
//         //     images: Vec::with_capacity(image_count),
//         //     buffers: Vec::with_capacity(buffer_count),
//         // };
//         //
//         // for desc in &self.image_descs {
//         //     physical.images.push(pool.acquire_image(&device.allocator, desc));
//         // }
//         // for desc in &self.buffer_descs {
//         //     physical.buffers.push(pool.acquire_buffer(&device.allocator, desc));
//         // }
//
//         // ---------------------------------------------------------------
//         // 2. For each submission batch, record and submit
//         // ---------------------------------------------------------------
//         //
//         // for batch in &self.batches {
//         //     let queue = device.queue_for(batch.queue);
//         //
//         //     // TODO: Allocate command buffer from a per-queue command pool.
//         //     //       Command pools should be cached or ring-buffered per
//         //     //       frame to avoid creation overhead.
//         //     //
//         //     // let cmd = allocate_command_buffer(device, queue.family_index);
//         //     // device.device.begin_command_buffer(cmd, &begin_info)?;
//         //
//         //     for &pass_index in &batch.passes {
//         //         // 2a. Record pre-pass barriers
//         //         // record_barriers(
//         //         //     &device.device, cmd,
//         //         //     &self.barriers[pass_index], &physical,
//         //         //     &|idx| self.is_image_resource(idx),
//         //         // );
//         //
//         //         // 2b. If the pass has color/depth attachments, begin
//         //         //     dynamic rendering (VK_KHR_dynamic_rendering / 1.3)
//         //         //
//         //         // let color_attachments: Vec<vk::RenderingAttachmentInfo> = ...;
//         //         // let rendering_info = vk::RenderingInfo::default()
//         //         //     .render_area(...)
//         //         //     .layer_count(1)
//         //         //     .color_attachments(&color_attachments)
//         //         //     .depth_attachment(&depth_attachment);
//         //         // device.device.cmd_begin_rendering(cmd, &rendering_info);
//         //
//         //         // 2c. Replay the recorded commands
//         //         // replay_commands(
//         //         //     &device.device, cmd,
//         //         //     &self.pass_commands[pass_index], &physical,
//         //         // );
//         //
//         //         // 2d. End rendering if it was begun
//         //         // device.device.cmd_end_rendering(cmd);
//         //     }
//         //
//         //     // device.device.end_command_buffer(cmd)?;
//         //
//         //     // 2e. Submit with timeline semaphore signal/wait ops
//         //     //     derived from self.semaphore_ops.
//         //     //
//         //     // let submit_info = vk::SubmitInfo2::default()
//         //     //     .command_buffer_infos(&[cmd_info])
//         //     //     .wait_semaphore_infos(&wait_infos)
//         //     //     .signal_semaphore_infos(&signal_infos);
//         //     // device.device.queue_submit2(queue.queue, &[submit_info], fence)?;
//         // }
//
//         // ---------------------------------------------------------------
//         // 3. Return
//         // ---------------------------------------------------------------
//         //
//         // Ok(())
//     }
// }
// ```
