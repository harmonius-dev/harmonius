use ash::vk;
use smallvec::SmallVec;

use crate::graph::resource::{BufferHandle, BufferInfo, ConditionFlag, DrawSlot, ImageHandle, ImageInfo};

// ---------------------------------------------------------------------------
// QueueType
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QueueType {
    Graphics,
    AsyncCompute,
    Transfer,
}

// ---------------------------------------------------------------------------
// RecordedCommand
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum RecordedCommand {
    // State binding
    BindGraphicsPipeline(vk::Pipeline),
    BindComputePipeline(vk::Pipeline),
    BindDescriptorSets {
        bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        sets: SmallVec<[vk::DescriptorSet; 4]>,
        dynamic_offsets: SmallVec<[u32; 4]>,
    },
    BindVertexBuffers {
        first_binding: u32,
        buffers: SmallVec<[vk::Buffer; 2]>,
        offsets: SmallVec<[vk::DeviceSize; 2]>,
    },
    BindIndexBuffer {
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        index_type: vk::IndexType,
    },
    PushConstants {
        layout: vk::PipelineLayout,
        stages: vk::ShaderStageFlags,
        offset: u32,
        data: SmallVec<[u8; 128]>,
    },
    SetViewport {
        first: u32,
        viewports: SmallVec<[vk::Viewport; 1]>,
    },
    SetScissor {
        first: u32,
        scissors: SmallVec<[vk::Rect2D; 1]>,
    },

    // Direct draws
    Draw {
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    },
    DrawIndexed {
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    },

    // Indirect draws
    DrawIndirect {
        buffer: BufferHandle,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    },
    DrawIndexedIndirect {
        buffer: BufferHandle,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    },
    DrawIndirectCount {
        command_buffer: BufferHandle,
        command_offset: vk::DeviceSize,
        count_buffer: BufferHandle,
        count_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    },
    DrawIndexedIndirectCount {
        command_buffer: BufferHandle,
        command_offset: vk::DeviceSize,
        count_buffer: BufferHandle,
        count_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    },

    // Compute
    Dispatch {
        x: u32,
        y: u32,
        z: u32,
    },
    DispatchIndirect {
        buffer: BufferHandle,
        offset: vk::DeviceSize,
    },

    // Transfer
    FillBuffer {
        buffer: BufferHandle,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        data: u32,
    },
    CopyBuffer {
        src: BufferHandle,
        src_offset: vk::DeviceSize,
        dst: BufferHandle,
        dst_offset: vk::DeviceSize,
        size: vk::DeviceSize,
    },

    // Dynamic draw slot placeholder
    DrawSlotPlaceholder(DrawSlot),
}

// ---------------------------------------------------------------------------
// PassCondition
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub(crate) enum PassCondition {
    Always,
    WhenFlag(ConditionFlag),
    WhenNotFlag(ConditionFlag),
}

// ---------------------------------------------------------------------------
// ResourceAccess
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub(crate) struct ResourceAccess {
    pub resource_index: u16,
    pub version_read: Option<u16>,
    pub version_write: Option<u16>,
    pub stage: vk::PipelineStageFlags2,
    pub access: vk::AccessFlags2,
    pub layout: vk::ImageLayout,
    pub condition: Option<ConditionFlag>,
}

// ---------------------------------------------------------------------------
// AttachmentInfo
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub(crate) struct AttachmentInfo {
    pub resource_index: u16,
    pub version: u16,
    pub load_op: vk::AttachmentLoadOp,
    pub store_op: vk::AttachmentStoreOp,
    pub format: vk::Format,
}

// ---------------------------------------------------------------------------
// PassNode
// ---------------------------------------------------------------------------

pub(crate) struct PassNode {
    pub name: &'static str,
    pub queue: QueueType,
    pub condition: PassCondition,
    pub accesses: SmallVec<[ResourceAccess; 8]>,
    pub color_attachments: SmallVec<[AttachmentInfo; 4]>,
    pub depth_attachment: Option<AttachmentInfo>,
    pub commands: Vec<RecordedCommand>,
    pub draw_slots: SmallVec<[(u16, bool); 2]>,
}

// ---------------------------------------------------------------------------
// PassBuilder
// ---------------------------------------------------------------------------

/// Minimum indirect draw command stride (bytes). Sized to fit
/// `VkDrawIndexedIndirectCommand` (5 × u32 = 20 bytes) so a single buffer can
/// be used for both indexed and non-indexed indirect draws.
const MIN_INDIRECT_STRIDE: u32 = 20;

pub struct PassBuilder<'a> {
    pub(crate) graph_passes: &'a mut Vec<PassNode>,
    pub(crate) graph_generation: u32,
    pub(crate) graph_images: &'a Vec<ImageInfo>,
    pub(crate) graph_buffers: &'a Vec<BufferInfo>,
    pub(crate) draw_slot_counter: &'a mut u16,
    name: &'static str,
    queue: QueueType,
    condition: PassCondition,
    accesses: SmallVec<[ResourceAccess; 8]>,
    color_attachments: SmallVec<[AttachmentInfo; 4]>,
    depth_attachment: Option<AttachmentInfo>,
    commands: Vec<RecordedCommand>,
    draw_slots: SmallVec<[(u16, bool); 2]>,
    built: bool,
}

impl<'a> PassBuilder<'a> {
    pub(crate) fn new(
        graph_passes: &'a mut Vec<PassNode>,
        graph_generation: u32,
        graph_images: &'a Vec<ImageInfo>,
        graph_buffers: &'a Vec<BufferInfo>,
        draw_slot_counter: &'a mut u16,
        name: &'static str,
        queue: QueueType,
    ) -> Self {
        Self {
            graph_passes,
            graph_generation,
            graph_images,
            graph_buffers,
            draw_slot_counter,
            name,
            queue,
            condition: PassCondition::Always,
            accesses: SmallVec::new(),
            color_attachments: SmallVec::new(),
            depth_attachment: None,
            commands: Vec::new(),
            draw_slots: SmallVec::new(),
            built: false,
        }
    }

    // -- Condition methods --------------------------------------------------

    pub fn when(mut self, flag: ConditionFlag) -> Self {
        assert!(
            matches!(self.condition, PassCondition::Always),
            "PassBuilder `{}`: condition already set",
            self.name,
        );
        self.condition = PassCondition::WhenFlag(flag);
        self
    }

    pub fn when_not(mut self, flag: ConditionFlag) -> Self {
        assert!(
            matches!(self.condition, PassCondition::Always),
            "PassBuilder `{}`: condition already set",
            self.name,
        );
        self.condition = PassCondition::WhenNotFlag(flag);
        self
    }

    // -- Resource declaration: reads ----------------------------------------

    pub fn sample_image(mut self, handle: ImageHandle, stage: vk::PipelineStageFlags2) -> Self {
        self.accesses.push(ResourceAccess {
            resource_index: handle.index,
            version_read: Some(handle.version),
            version_write: None,
            stage,
            access: vk::AccessFlags2::SHADER_SAMPLED_READ,
            layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            condition: None,
        });
        self
    }

    pub fn read_buffer(mut self, handle: BufferHandle, stage: vk::PipelineStageFlags2) -> Self {
        self.accesses.push(ResourceAccess {
            resource_index: handle.index,
            version_read: Some(handle.version),
            version_write: None,
            stage,
            access: vk::AccessFlags2::SHADER_STORAGE_READ,
            layout: vk::ImageLayout::UNDEFINED,
            condition: None,
        });
        self
    }

    pub fn read_indirect_buffer(mut self, handle: BufferHandle) -> Self {
        self.accesses.push(ResourceAccess {
            resource_index: handle.index,
            version_read: Some(handle.version),
            version_write: None,
            stage: vk::PipelineStageFlags2::DRAW_INDIRECT,
            access: vk::AccessFlags2::INDIRECT_COMMAND_READ,
            layout: vk::ImageLayout::UNDEFINED,
            condition: None,
        });
        self
    }

    pub fn sample_image_when(
        mut self,
        handle: ImageHandle,
        flag: ConditionFlag,
        stage: vk::PipelineStageFlags2,
    ) -> Self {
        self.accesses.push(ResourceAccess {
            resource_index: handle.index,
            version_read: Some(handle.version),
            version_write: None,
            stage,
            access: vk::AccessFlags2::SHADER_SAMPLED_READ,
            layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            condition: Some(flag),
        });
        self
    }

    pub fn read_buffer_when(
        mut self,
        handle: BufferHandle,
        flag: ConditionFlag,
        stage: vk::PipelineStageFlags2,
    ) -> Self {
        self.accesses.push(ResourceAccess {
            resource_index: handle.index,
            version_read: Some(handle.version),
            version_write: None,
            stage,
            access: vk::AccessFlags2::SHADER_STORAGE_READ,
            layout: vk::ImageLayout::UNDEFINED,
            condition: Some(flag),
        });
        self
    }

    // -- Resource declaration: writes ---------------------------------------

    pub fn write_storage_image(mut self, handle: ImageHandle) -> (Self, ImageHandle) {
        let new_version = handle.version + 1;
        self.accesses.push(ResourceAccess {
            resource_index: handle.index,
            version_read: Some(handle.version),
            version_write: Some(new_version),
            stage: vk::PipelineStageFlags2::COMPUTE_SHADER,
            access: vk::AccessFlags2::SHADER_STORAGE_WRITE,
            layout: vk::ImageLayout::GENERAL,
            condition: None,
        });
        let new_handle = ImageHandle {
            index: handle.index,
            version: new_version,
            generation: self.graph_generation,
        };
        (self, new_handle)
    }

    pub fn write_storage_buffer(mut self, handle: BufferHandle) -> (Self, BufferHandle) {
        let new_version = handle.version + 1;
        self.accesses.push(ResourceAccess {
            resource_index: handle.index,
            version_read: Some(handle.version),
            version_write: Some(new_version),
            stage: vk::PipelineStageFlags2::COMPUTE_SHADER,
            access: vk::AccessFlags2::SHADER_STORAGE_WRITE,
            layout: vk::ImageLayout::UNDEFINED,
            condition: None,
        });
        let new_handle = BufferHandle {
            index: handle.index,
            version: new_version,
            generation: self.graph_generation,
        };
        (self, new_handle)
    }

    // -- Resource declaration: attachments ----------------------------------

    pub fn color_attachment(
        mut self,
        handle: ImageHandle,
        load_op: vk::AttachmentLoadOp,
    ) -> (Self, ImageHandle) {
        let new_version = handle.version + 1;
        let format = self.graph_images[handle.index as usize].desc.format;

        self.accesses.push(ResourceAccess {
            resource_index: handle.index,
            version_read: Some(handle.version),
            version_write: Some(new_version),
            stage: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
            access: vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            condition: None,
        });

        self.color_attachments.push(AttachmentInfo {
            resource_index: handle.index,
            version: new_version,
            load_op,
            store_op: vk::AttachmentStoreOp::STORE,
            format,
        });

        let new_handle = ImageHandle {
            index: handle.index,
            version: new_version,
            generation: self.graph_generation,
        };
        (self, new_handle)
    }

    pub fn depth_attachment(
        mut self,
        handle: ImageHandle,
        load_op: vk::AttachmentLoadOp,
    ) -> (Self, ImageHandle) {
        let new_version = handle.version + 1;
        let format = self.graph_images[handle.index as usize].desc.format;

        self.accesses.push(ResourceAccess {
            resource_index: handle.index,
            version_read: Some(handle.version),
            version_write: Some(new_version),
            stage: vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS
                | vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
            access: vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
            layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            condition: None,
        });

        self.depth_attachment = Some(AttachmentInfo {
            resource_index: handle.index,
            version: new_version,
            load_op,
            store_op: vk::AttachmentStoreOp::STORE,
            format,
        });

        let new_handle = ImageHandle {
            index: handle.index,
            version: new_version,
            generation: self.graph_generation,
        };
        (self, new_handle)
    }

    // -- Command methods: state binding -------------------------------------

    pub fn bind_graphics_pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.commands
            .push(RecordedCommand::BindGraphicsPipeline(pipeline));
        self
    }

    pub fn bind_compute_pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.commands
            .push(RecordedCommand::BindComputePipeline(pipeline));
        self
    }

    pub fn bind_descriptor_sets(
        mut self,
        bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        sets: &[vk::DescriptorSet],
        dynamic_offsets: &[u32],
    ) -> Self {
        self.commands.push(RecordedCommand::BindDescriptorSets {
            bind_point,
            layout,
            first_set,
            sets: SmallVec::from_slice(sets),
            dynamic_offsets: SmallVec::from_slice(dynamic_offsets),
        });
        self
    }

    pub fn bind_vertex_buffers(
        mut self,
        first_binding: u32,
        buffers: &[vk::Buffer],
        offsets: &[vk::DeviceSize],
    ) -> Self {
        assert_eq!(
            buffers.len(),
            offsets.len(),
            "bind_vertex_buffers: buffers.len() ({}) != offsets.len() ({})",
            buffers.len(),
            offsets.len(),
        );
        self.commands.push(RecordedCommand::BindVertexBuffers {
            first_binding,
            buffers: SmallVec::from_slice(buffers),
            offsets: SmallVec::from_slice(offsets),
        });
        self
    }

    pub fn bind_index_buffer(
        mut self,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        index_type: vk::IndexType,
    ) -> Self {
        self.commands.push(RecordedCommand::BindIndexBuffer {
            buffer,
            offset,
            index_type,
        });
        self
    }

    pub fn push_constants(
        mut self,
        layout: vk::PipelineLayout,
        stages: vk::ShaderStageFlags,
        offset: u32,
        data: &[u8],
    ) -> Self {
        assert!(
            !data.is_empty(),
            "push_constants: data must not be empty",
        );
        assert!(
            offset % 4 == 0,
            "push_constants: offset ({offset}) must be 4-byte aligned",
        );
        assert!(
            data.len() % 4 == 0,
            "push_constants: data length ({}) must be 4-byte aligned",
            data.len(),
        );
        self.commands.push(RecordedCommand::PushConstants {
            layout,
            stages,
            offset,
            data: SmallVec::from_slice(data),
        });
        self
    }

    pub fn set_viewport(mut self, first: u32, viewports: &[vk::Viewport]) -> Self {
        self.commands.push(RecordedCommand::SetViewport {
            first,
            viewports: SmallVec::from_slice(viewports),
        });
        self
    }

    pub fn set_scissor(mut self, first: u32, scissors: &[vk::Rect2D]) -> Self {
        self.commands.push(RecordedCommand::SetScissor {
            first,
            scissors: SmallVec::from_slice(scissors),
        });
        self
    }

    // -- Command methods: direct draws --------------------------------------

    pub fn draw(
        mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> Self {
        self.commands.push(RecordedCommand::Draw {
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        });
        self
    }

    pub fn draw_indexed(
        mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) -> Self {
        self.commands.push(RecordedCommand::DrawIndexed {
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        });
        self
    }

    // -- Command methods: indirect draws ------------------------------------

    fn assert_indirect_stride(stride: u32) {
        assert!(
            stride % 4 == 0,
            "indirect stride ({stride}) must be 4-byte aligned",
        );
        assert!(
            stride >= MIN_INDIRECT_STRIDE,
            "indirect stride ({stride}) must be >= {MIN_INDIRECT_STRIDE} bytes",
        );
    }

    pub fn draw_indirect(
        mut self,
        buffer: BufferHandle,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) -> Self {
        Self::assert_indirect_stride(stride);
        self.commands.push(RecordedCommand::DrawIndirect {
            buffer,
            offset,
            draw_count,
            stride,
        });
        self
    }

    pub fn draw_indexed_indirect(
        mut self,
        buffer: BufferHandle,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) -> Self {
        Self::assert_indirect_stride(stride);
        self.commands.push(RecordedCommand::DrawIndexedIndirect {
            buffer,
            offset,
            draw_count,
            stride,
        });
        self
    }

    pub fn draw_indirect_count(
        mut self,
        command_buffer: BufferHandle,
        command_offset: vk::DeviceSize,
        count_buffer: BufferHandle,
        count_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> Self {
        Self::assert_indirect_stride(stride);
        self.commands.push(RecordedCommand::DrawIndirectCount {
            command_buffer,
            command_offset,
            count_buffer,
            count_offset,
            max_draw_count,
            stride,
        });
        self
    }

    pub fn draw_indexed_indirect_count(
        mut self,
        command_buffer: BufferHandle,
        command_offset: vk::DeviceSize,
        count_buffer: BufferHandle,
        count_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> Self {
        Self::assert_indirect_stride(stride);
        self.commands
            .push(RecordedCommand::DrawIndexedIndirectCount {
                command_buffer,
                command_offset,
                count_buffer,
                count_offset,
                max_draw_count,
                stride,
            });
        self
    }

    // -- Command methods: compute -------------------------------------------

    pub fn dispatch(mut self, x: u32, y: u32, z: u32) -> Self {
        self.commands.push(RecordedCommand::Dispatch { x, y, z });
        self
    }

    pub fn dispatch_indirect(
        mut self,
        buffer: BufferHandle,
        offset: vk::DeviceSize,
    ) -> Self {
        self.commands
            .push(RecordedCommand::DispatchIndirect { buffer, offset });
        self
    }

    // -- Command methods: transfer ------------------------------------------

    pub fn fill_buffer(
        mut self,
        buffer: BufferHandle,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        data: u32,
    ) -> Self {
        self.commands.push(RecordedCommand::FillBuffer {
            buffer,
            offset,
            size,
            data,
        });
        self
    }

    pub fn copy_buffer(
        mut self,
        src: BufferHandle,
        src_offset: vk::DeviceSize,
        dst: BufferHandle,
        dst_offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> Self {
        self.commands.push(RecordedCommand::CopyBuffer {
            src,
            src_offset,
            dst,
            dst_offset,
            size,
        });
        self
    }

    // -- Command methods: draw slots ----------------------------------------

    pub fn opaque_draw_slot(mut self) -> (Self, DrawSlot) {
        let pass_index = self.graph_passes.len() as u16;
        let slot_index = *self.draw_slot_counter;
        *self.draw_slot_counter += 1;

        let slot = DrawSlot {
            pass_index,
            slot_index,
            generation: self.graph_generation,
        };

        self.draw_slots.push((slot_index, false));
        self.commands
            .push(RecordedCommand::DrawSlotPlaceholder(slot));
        (self, slot)
    }

    pub fn transparent_draw_slot(mut self) -> (Self, DrawSlot) {
        let pass_index = self.graph_passes.len() as u16;
        let slot_index = *self.draw_slot_counter;
        *self.draw_slot_counter += 1;

        let slot = DrawSlot {
            pass_index,
            slot_index,
            generation: self.graph_generation,
        };

        self.draw_slots.push((slot_index, true));
        self.commands
            .push(RecordedCommand::DrawSlotPlaceholder(slot));
        (self, slot)
    }

    // -- Build --------------------------------------------------------------

    pub fn build(mut self) {
        self.built = true;
        self.graph_passes.push(PassNode {
            name: self.name,
            queue: self.queue,
            condition: std::mem::replace(&mut self.condition, PassCondition::Always),
            accesses: std::mem::take(&mut self.accesses),
            color_attachments: std::mem::take(&mut self.color_attachments),
            depth_attachment: self.depth_attachment.take(),
            commands: std::mem::take(&mut self.commands),
            draw_slots: std::mem::take(&mut self.draw_slots),
        });
    }
}

// ---------------------------------------------------------------------------
// Drop guard
// ---------------------------------------------------------------------------

impl<'a> Drop for PassBuilder<'a> {
    fn drop(&mut self) {
        if !self.built {
            debug_assert!(
                false,
                "PassBuilder for `{}` dropped without calling build()",
                self.name,
            );
            log::warn!(
                "PassBuilder for `{}` dropped without build()",
                self.name,
            );
        }
    }
}
