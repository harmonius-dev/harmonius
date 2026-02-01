// enthrall_command.cpp - Command buffer and encoding implementation

#include "enthrall_internal.h"

using namespace ep::vk;

// ============================================================================
// Command Buffer
// ============================================================================

extern "C" EPError EPCommandBufferCreate(EPDevicePtr device_ptr, EPCommandBufferPtr* out_command_buffer) {
    if (!device_ptr || !out_command_buffer) return invalid_argument("device or out_command_buffer is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    auto cmd = std::make_unique<CommandBuffer>();
    cmd->device = device->shared_from_this();

    auto [result, buffers] = device->device->allocateCommandBuffersUnique(::vk::CommandBufferAllocateInfo{
        .commandPool = device->graphics_pool.get(),
        .level = ::vk::CommandBufferLevel::ePrimary,
        .commandBufferCount = 1,
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    cmd->buffer = std::move(buffers[0]);
    *out_command_buffer = reinterpret_cast<EPCommandBufferPtr>(cmd.release());
    return ok();
}

extern "C" EPError EPCommandBufferDestroy(EPCommandBufferPtr command_buffer) {
    delete reinterpret_cast<CommandBuffer*>(command_buffer);
    return ok();
}

extern "C" EPError EPCommandBufferBegin(EPCommandBufferPtr command_buffer) {
    if (!command_buffer) return invalid_argument("command_buffer is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    if (cmd->is_recording) return error(EP_E_INVALID_STATE, "already recording");

    auto result = cmd->buffer->begin(::vk::CommandBufferBeginInfo{
        .flags = ::vk::CommandBufferUsageFlagBits::eOneTimeSubmit,
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    cmd->is_recording = true;
    return ok();
}

extern "C" EPError EPCommandBufferEnd(EPCommandBufferPtr command_buffer) {
    if (!command_buffer) return invalid_argument("command_buffer is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    if (!cmd->is_recording) return error(EP_E_INVALID_STATE, "not recording");

    auto result = cmd->buffer->end();
    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    cmd->is_recording = false;
    return ok();
}

// ============================================================================
// Render Pass (Dynamic Rendering)
// ============================================================================

extern "C" EPError EPCommandBeginRenderPass(EPCommandBufferPtr command_buffer, const EPRenderPassDesc* desc) {
    if (!command_buffer || !desc) return invalid_argument("command_buffer or desc is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    if (!cmd->is_recording) return error(EP_E_INVALID_STATE, "not recording");

    // Build color attachments
    std::vector<::vk::RenderingAttachmentInfo> color_attachments;
    uint32_t width = 0, height = 0;

    for (uint32_t i = 0; i < desc->color_attachment_count; i++) {
        const auto& att = desc->color_attachments[i];
        if (!att.texture) continue;

        auto* texture = reinterpret_cast<Texture*>(att.texture);
        if (width == 0) {
            width = texture->desc.width;
            height = texture->desc.height;
        }

        color_attachments.push_back(::vk::RenderingAttachmentInfo{
            .imageView = texture->view.get(),
            .imageLayout = to_vk_image_layout(att.layout),
            .loadOp = to_vk_load_op(att.load_op),
            .storeOp = to_vk_store_op(att.store_op),
            .clearValue = ::vk::ClearValue{.color = ::vk::ClearColorValue{
                std::array<float, 4>{att.clear_color[0], att.clear_color[1], att.clear_color[2], att.clear_color[3]}}},
        });
    }

    // Build depth attachment
    ::vk::RenderingAttachmentInfo depth_attachment_storage;
    ::vk::RenderingAttachmentInfo* depth_attachment_ptr = nullptr;

    if (desc->depth_attachment && desc->depth_attachment->texture) {
        auto* texture = reinterpret_cast<Texture*>(desc->depth_attachment->texture);
        if (width == 0) {
            width = texture->desc.width;
            height = texture->desc.height;
        }

        depth_attachment_storage = ::vk::RenderingAttachmentInfo{
            .imageView = texture->view.get(),
            .imageLayout = to_vk_image_layout(desc->depth_attachment->layout),
            .loadOp = to_vk_load_op(desc->depth_attachment->load_op),
            .storeOp = to_vk_store_op(desc->depth_attachment->store_op),
            .clearValue = ::vk::ClearValue{.depthStencil = ::vk::ClearDepthStencilValue{
                desc->depth_attachment->clear_depth, desc->depth_attachment->clear_stencil}},
        };
        depth_attachment_ptr = &depth_attachment_storage;
    }

    ::vk::RenderingInfo rendering_info{
        .renderArea = ::vk::Rect2D{.offset = {0, 0}, .extent = {width, height}},
        .layerCount = 1,
        .colorAttachmentCount = static_cast<uint32_t>(color_attachments.size()),
        .pColorAttachments = color_attachments.data(),
        .pDepthAttachment = depth_attachment_ptr,
    };

    cmd->buffer->beginRendering(rendering_info);
    cmd->current_bind_point = ::vk::PipelineBindPoint::eGraphics;
    return ok();
}

extern "C" EPError EPCommandEndRenderPass(EPCommandBufferPtr command_buffer) {
    if (!command_buffer) return invalid_argument("command_buffer is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    cmd->buffer->endRendering();
    return ok();
}

// ============================================================================
// Pipeline Binding
// ============================================================================

extern "C" EPError EPCommandBindRenderPipeline(EPCommandBufferPtr command_buffer, EPRenderPipelinePtr pipeline) {
    if (!command_buffer || !pipeline) return invalid_argument("command_buffer or pipeline is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    auto* pipe = reinterpret_cast<RenderPipeline*>(pipeline);

    cmd->buffer->bindPipeline(::vk::PipelineBindPoint::eGraphics, pipe->pipeline.get());
    cmd->current_bind_point = ::vk::PipelineBindPoint::eGraphics;
    cmd->current_layout = pipe->layout ? pipe->layout->layout.get() : ::vk::PipelineLayout{};
    return ok();
}

extern "C" EPError EPCommandBindMeshPipeline(EPCommandBufferPtr command_buffer, EPMeshPipelinePtr pipeline) {
    if (!command_buffer || !pipeline) return invalid_argument("command_buffer or pipeline is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    auto* pipe = reinterpret_cast<MeshPipeline*>(pipeline);

    cmd->buffer->bindPipeline(::vk::PipelineBindPoint::eGraphics, pipe->pipeline.get());
    cmd->current_bind_point = ::vk::PipelineBindPoint::eGraphics;
    cmd->current_layout = pipe->layout ? pipe->layout->layout.get() : ::vk::PipelineLayout{};
    return ok();
}

extern "C" EPError EPCommandBindComputePipeline(EPCommandBufferPtr command_buffer, EPComputePipelinePtr pipeline) {
    if (!command_buffer || !pipeline) return invalid_argument("command_buffer or pipeline is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    auto* pipe = reinterpret_cast<ComputePipeline*>(pipeline);

    cmd->buffer->bindPipeline(::vk::PipelineBindPoint::eCompute, pipe->pipeline.get());
    cmd->current_bind_point = ::vk::PipelineBindPoint::eCompute;
    cmd->current_layout = pipe->layout ? pipe->layout->layout.get() : ::vk::PipelineLayout{};
    return ok();
}

extern "C" EPError EPCommandBindRayTracingPipeline(EPCommandBufferPtr command_buffer, EPRayTracingPipelinePtr pipeline) {
    if (!command_buffer || !pipeline) return invalid_argument("command_buffer or pipeline is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    auto* pipe = reinterpret_cast<RayTracingPipeline*>(pipeline);

    cmd->buffer->bindPipeline(::vk::PipelineBindPoint::eRayTracingKHR, pipe->pipeline.get());
    cmd->current_bind_point = ::vk::PipelineBindPoint::eRayTracingKHR;
    cmd->current_layout = pipe->layout ? pipe->layout->layout.get() : ::vk::PipelineLayout{};
    return ok();
}

// ============================================================================
// Descriptor Binding
// ============================================================================

extern "C" EPError EPCommandBindDescriptorSet(EPCommandBufferPtr command_buffer, EPPipelineLayoutPtr layout,
                                               uint32_t set_index, EPDescriptorSetPtr set) {
    if (!command_buffer || !set) return invalid_argument("command_buffer or set is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    auto* desc_set = reinterpret_cast<DescriptorSet*>(set);

    ::vk::PipelineLayout pipe_layout = layout
        ? reinterpret_cast<PipelineLayout*>(layout)->layout.get()
        : cmd->current_layout;

    cmd->buffer->bindDescriptorSets(cmd->current_bind_point, pipe_layout, set_index, desc_set->set, {});
    return ok();
}

extern "C" EPError EPCommandPushConstants(EPCommandBufferPtr command_buffer, EPPipelineLayoutPtr layout,
                                          EPShaderStageFlags stages, const uint8_t* data, uint32_t size) {
    if (!command_buffer || !data) return invalid_argument("command_buffer or data is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);

    ::vk::PipelineLayout pipe_layout = layout
        ? reinterpret_cast<PipelineLayout*>(layout)->layout.get()
        : cmd->current_layout;

    cmd->buffer->pushConstants(pipe_layout, to_vk_shader_stage(stages), 0, size, data);
    return ok();
}

// ============================================================================
// Viewport/Scissor
// ============================================================================

extern "C" EPError EPCommandSetViewport(EPCommandBufferPtr command_buffer, float x, float y,
                                        float width, float height, float min_depth, float max_depth) {
    if (!command_buffer) return invalid_argument("command_buffer is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    cmd->buffer->setViewport(0, ::vk::Viewport{x, y, width, height, min_depth, max_depth});
    return ok();
}

extern "C" EPError EPCommandSetScissor(EPCommandBufferPtr command_buffer, uint32_t x, uint32_t y,
                                       uint32_t width, uint32_t height) {
    if (!command_buffer) return invalid_argument("command_buffer is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    cmd->buffer->setScissor(0, ::vk::Rect2D{{static_cast<int32_t>(x), static_cast<int32_t>(y)}, {width, height}});
    return ok();
}

// ============================================================================
// Draw/Dispatch
// ============================================================================

extern "C" EPError EPCommandDraw(EPCommandBufferPtr command_buffer, uint32_t vertex_count,
                                  uint32_t instance_count, uint32_t first_vertex, uint32_t first_instance) {
    if (!command_buffer) return invalid_argument("command_buffer is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    cmd->buffer->draw(vertex_count, instance_count, first_vertex, first_instance);
    return ok();
}

extern "C" EPError EPCommandDrawIndexed(EPCommandBufferPtr command_buffer, uint32_t index_count,
                                         uint32_t instance_count, uint32_t first_index,
                                         int32_t vertex_offset, uint32_t first_instance) {
    if (!command_buffer) return invalid_argument("command_buffer is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    cmd->buffer->drawIndexed(index_count, instance_count, first_index, vertex_offset, first_instance);
    return ok();
}

extern "C" EPError EPCommandDispatch(EPCommandBufferPtr command_buffer, uint32_t group_x,
                                     uint32_t group_y, uint32_t group_z) {
    if (!command_buffer) return invalid_argument("command_buffer is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    cmd->buffer->dispatch(group_x, group_y, group_z);
    return ok();
}

extern "C" EPError EPCommandDispatchMesh(EPCommandBufferPtr command_buffer, uint32_t group_x,
                                         uint32_t group_y, uint32_t group_z) {
    if (!command_buffer) return invalid_argument("command_buffer is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    cmd->buffer->drawMeshTasksEXT(group_x, group_y, group_z);
    return ok();
}

extern "C" EPError EPCommandDispatchRays(EPCommandBufferPtr command_buffer, uint32_t width,
                                          uint32_t height, uint32_t depth) {
    if (!command_buffer) return invalid_argument("command_buffer is NULL");

    // Ray dispatch requires the SBT regions from the pipeline
    // This is a simplified version - full impl would track current RT pipeline
    return unsupported("ray dispatch requires bound ray tracing pipeline with SBT");
}

extern "C" EPError EPCommandResourceBarrier(EPCommandBufferPtr command_buffer, const EPBarrierDesc* desc) {
    if (!command_buffer || !desc) return invalid_argument("command_buffer or desc is NULL");

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);

    std::vector<::vk::BufferMemoryBarrier2> buffer_barriers;
    for (uint32_t i = 0; i < desc->buffer_barrier_count; i++) {
        const auto& b = desc->buffer_barriers[i];
        auto* buffer = reinterpret_cast<Buffer*>(b.buffer);

        buffer_barriers.push_back(::vk::BufferMemoryBarrier2{
            .srcStageMask = to_vk_pipeline_stage2(desc->src_stage),
            .srcAccessMask = to_vk_access_flags2(b.src_access),
            .dstStageMask = to_vk_pipeline_stage2(desc->dst_stage),
            .dstAccessMask = to_vk_access_flags2(b.dst_access),
            .buffer = buffer->buffer,
            .offset = b.offset,
            .size = b.size == 0 ? VK_WHOLE_SIZE : b.size,
        });
    }

    std::vector<::vk::ImageMemoryBarrier2> image_barriers;
    for (uint32_t i = 0; i < desc->texture_barrier_count; i++) {
        const auto& t = desc->texture_barriers[i];
        auto* texture = reinterpret_cast<Texture*>(t.texture);

        bool is_depth = texture->desc.format == EP_FORMAT_D24S8 || texture->desc.format == EP_FORMAT_D32_FLOAT;

        image_barriers.push_back(::vk::ImageMemoryBarrier2{
            .srcStageMask = to_vk_pipeline_stage2(desc->src_stage),
            .srcAccessMask = to_vk_access_flags2(t.src_access),
            .dstStageMask = to_vk_pipeline_stage2(desc->dst_stage),
            .dstAccessMask = to_vk_access_flags2(t.dst_access),
            .oldLayout = to_vk_image_layout(t.old_layout),
            .newLayout = to_vk_image_layout(t.new_layout),
            .image = texture->image,
            .subresourceRange = ::vk::ImageSubresourceRange{
                .aspectMask = is_depth ? ::vk::ImageAspectFlagBits::eDepth : ::vk::ImageAspectFlagBits::eColor,
                .baseMipLevel = 0,
                .levelCount = VK_REMAINING_MIP_LEVELS,
                .baseArrayLayer = 0,
                .layerCount = VK_REMAINING_ARRAY_LAYERS,
            },
        });

        texture->current_layout = to_vk_image_layout(t.new_layout);
    }

    cmd->buffer->pipelineBarrier2(::vk::DependencyInfo{
        .bufferMemoryBarrierCount = static_cast<uint32_t>(buffer_barriers.size()),
        .pBufferMemoryBarriers = buffer_barriers.data(),
        .imageMemoryBarrierCount = static_cast<uint32_t>(image_barriers.size()),
        .pImageMemoryBarriers = image_barriers.data(),
    });

    return ok();
}

extern "C" EPError EPCommandCopyBuffer(EPCommandBufferPtr command_buffer,
                                        EPBufferPtr src_ptr, uint64_t src_offset,
                                        EPBufferPtr dst_ptr, uint64_t dst_offset,
                                        uint64_t size) {
    if (!command_buffer || !src_ptr || !dst_ptr) {
        return invalid_argument("command_buffer, src, or dst is NULL");
    }

    auto* cmd = reinterpret_cast<CommandBuffer*>(command_buffer);
    auto* src = reinterpret_cast<Buffer*>(src_ptr);
    auto* dst = reinterpret_cast<Buffer*>(dst_ptr);

    ::vk::BufferCopy region{
        .srcOffset = src_offset,
        .dstOffset = dst_offset,
        .size = size,
    };

    cmd->buffer->copyBuffer(src->buffer, dst->buffer, region);
    return ok();
}
