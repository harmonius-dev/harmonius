// gcraft_command.m - Command buffer and encoding implementation

#include "gcraft_internal.h"

#pragma mark - Command Buffer

GCError GCCommandBufferCreate(GCDevicePtr device, GCCommandBufferPtr *out_command_buffer) {
    if (!device || !out_command_buffer) return ep_invalid_argument("device or out_command_buffer is NULL");

    GCCommandBuffer *cmd = calloc(1, sizeof(GCCommandBuffer));
    if (!cmd) return ep_out_of_memory();

    cmd->ep_device = device;
    cmd->is_recording = false;

    *out_command_buffer = cmd;
    return ep_ok();
}

GCError GCCommandBufferDestroy(GCCommandBufferPtr command_buffer) {
    if (command_buffer) {
        @autoreleasepool {
            if (command_buffer->buffer) [command_buffer->buffer release];
        }
        free(command_buffer);
    }
    return ep_ok();
}

GCError GCCommandBufferBegin(GCCommandBufferPtr command_buffer) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (command_buffer->is_recording) return ep_error(GC_E_INVALID_STATE, "already recording");

    @autoreleasepool {
        if (command_buffer->buffer) [command_buffer->buffer release];

        command_buffer->buffer = [[command_buffer->ep_device->graphics_queue commandBuffer] retain];
        if (!command_buffer->buffer) {
            return ep_error(GC_E_DEVICE_LOST, "failed to create command buffer");
        }

        command_buffer->is_recording = true;
    }

    return ep_ok();
}

GCError GCCommandBufferEnd(GCCommandBufferPtr command_buffer) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (!command_buffer->is_recording) return ep_error(GC_E_INVALID_STATE, "not recording");

    @autoreleasepool {
        // End any active encoders
        if (command_buffer->render_encoder) {
            [command_buffer->render_encoder endEncoding];
            command_buffer->render_encoder = nil;
        }
        if (command_buffer->compute_encoder) {
            [command_buffer->compute_encoder endEncoding];
            command_buffer->compute_encoder = nil;
        }
        if (command_buffer->blit_encoder) {
            [command_buffer->blit_encoder endEncoding];
            command_buffer->blit_encoder = nil;
        }
        if (command_buffer->accel_encoder) {
            [command_buffer->accel_encoder endEncoding];
            command_buffer->accel_encoder = nil;
        }
    }

    command_buffer->is_recording = false;
    return ep_ok();
}

#pragma mark - Command Encoding: Render Pass

GCError GCCommandBeginRenderPass(GCCommandBufferPtr command_buffer, const GCRenderPassDesc *desc) {
    if (!command_buffer || !desc) return ep_invalid_argument("command_buffer or desc is NULL");
    if (!command_buffer->is_recording) return ep_error(GC_E_INVALID_STATE, "not recording");

    @autoreleasepool {
        // End any existing encoder
        if (command_buffer->render_encoder) {
            [command_buffer->render_encoder endEncoding];
            command_buffer->render_encoder = nil;
        }
        if (command_buffer->compute_encoder) {
            [command_buffer->compute_encoder endEncoding];
            command_buffer->compute_encoder = nil;
        }

        MTLRenderPassDescriptor *pass_desc = [[MTLRenderPassDescriptor alloc] init];

        for (uint32_t i = 0; i < desc->color_attachment_count; i++) {
            const GCRenderPassColorAttachment *att = &desc->color_attachments[i];
            if (!att->texture) continue;

            pass_desc.colorAttachments[i].texture = att->texture->texture;
            pass_desc.colorAttachments[i].loadAction = ep_to_mtl_load_action(att->load_op);
            pass_desc.colorAttachments[i].storeAction = ep_to_mtl_store_action(att->store_op);
            pass_desc.colorAttachments[i].clearColor = MTLClearColorMake(
                att->clear_color[0], att->clear_color[1], att->clear_color[2], att->clear_color[3]);
        }

        if (desc->depth_attachment && desc->depth_attachment->texture) {
            pass_desc.depthAttachment.texture = desc->depth_attachment->texture->texture;
            pass_desc.depthAttachment.loadAction = ep_to_mtl_load_action(desc->depth_attachment->load_op);
            pass_desc.depthAttachment.storeAction = ep_to_mtl_store_action(desc->depth_attachment->store_op);
            pass_desc.depthAttachment.clearDepth = desc->depth_attachment->clear_depth;
        }

        command_buffer->render_encoder = [[command_buffer->buffer renderCommandEncoderWithDescriptor:pass_desc] retain];
        [pass_desc release];

        if (!command_buffer->render_encoder) {
            return ep_error(GC_E_DEVICE_LOST, "failed to create render encoder");
        }
    }

    return ep_ok();
}

GCError GCCommandEndRenderPass(GCCommandBufferPtr command_buffer) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");

    @autoreleasepool {
        if (command_buffer->render_encoder) {
            [command_buffer->render_encoder endEncoding];
            command_buffer->render_encoder = nil;
        }
    }

    return ep_ok();
}

#pragma mark - Command Encoding: Pipeline Binding

GCError GCCommandBindRenderPipeline(GCCommandBufferPtr command_buffer, GCRenderPipelinePtr pipeline) {
    if (!command_buffer || !pipeline) return ep_invalid_argument("command_buffer or pipeline is NULL");
    if (!command_buffer->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

    @autoreleasepool {
        [command_buffer->render_encoder setRenderPipelineState:pipeline->state];
        if (pipeline->depth_stencil) {
            [command_buffer->render_encoder setDepthStencilState:pipeline->depth_stencil];
        }
    }

    return ep_ok();
}

GCError GCCommandBindMeshPipeline(GCCommandBufferPtr command_buffer, GCMeshPipelinePtr pipeline) {
    if (!command_buffer || !pipeline) return ep_invalid_argument("command_buffer or pipeline is NULL");
    if (!command_buffer->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

    @autoreleasepool {
        [command_buffer->render_encoder setRenderPipelineState:pipeline->state];
        if (pipeline->depth_stencil) {
            [command_buffer->render_encoder setDepthStencilState:pipeline->depth_stencil];
        }
    }

    return ep_ok();
}

GCError GCCommandBindComputePipeline(GCCommandBufferPtr command_buffer, GCComputePipelinePtr pipeline) {
    if (!command_buffer || !pipeline) return ep_invalid_argument("command_buffer or pipeline is NULL");

    @autoreleasepool {
        // Create compute encoder if needed
        if (!command_buffer->compute_encoder) {
            if (command_buffer->render_encoder) {
                [command_buffer->render_encoder endEncoding];
                command_buffer->render_encoder = nil;
            }
            command_buffer->compute_encoder = [[command_buffer->buffer computeCommandEncoder] retain];
        }

        [command_buffer->compute_encoder setComputePipelineState:pipeline->state];
    }

    return ep_ok();
}

GCError GCCommandBindRayTracingPipeline(GCCommandBufferPtr command_buffer, GCRayTracingPipelinePtr pipeline) {
    if (!command_buffer || !pipeline) return ep_invalid_argument("command_buffer or pipeline is NULL");

    @autoreleasepool {
        // Ray tracing uses compute encoder in Metal
        if (!command_buffer->compute_encoder) {
            if (command_buffer->render_encoder) {
                [command_buffer->render_encoder endEncoding];
                command_buffer->render_encoder = nil;
            }
            command_buffer->compute_encoder = [[command_buffer->buffer computeCommandEncoder] retain];
        }

        if (pipeline->states.count > 0) {
            [command_buffer->compute_encoder setComputePipelineState:pipeline->states[0]];
        }
        if (pipeline->function_table) {
            [command_buffer->compute_encoder setVisibleFunctionTable:pipeline->function_table atBufferIndex:0];
        }
    }

    return ep_ok();
}

#pragma mark - Command Encoding: Descriptor Binding

static void ep_bind_descriptors_to_render(id<MTLRenderCommandEncoder> encoder,
                                          GCDescriptorSet *set,
                                          uint32_t base_index) {
    for (uint32_t i = 0; i < set->entry_count; i++) {
        uint32_t binding = set->layout->bindings[i].binding + base_index;
        GCDescriptorSetEntry *entry = &set->entries[i];

        switch (entry->type) {
            case GC_DESCRIPTOR_UNIFORM_BUFFER:
            case GC_DESCRIPTOR_STORAGE_BUFFER:
                [encoder setVertexBuffer:entry->value.buffer_info.buffer
                                  offset:entry->value.buffer_info.offset
                                 atIndex:binding];
                [encoder setFragmentBuffer:entry->value.buffer_info.buffer
                                    offset:entry->value.buffer_info.offset
                                   atIndex:binding];
                break;
            case GC_DESCRIPTOR_SAMPLED_TEXTURE:
            case GC_DESCRIPTOR_STORAGE_TEXTURE:
                [encoder setVertexTexture:entry->value.texture atIndex:binding];
                [encoder setFragmentTexture:entry->value.texture atIndex:binding];
                break;
            case GC_DESCRIPTOR_SAMPLER:
                [encoder setVertexSamplerState:entry->value.sampler atIndex:binding];
                [encoder setFragmentSamplerState:entry->value.sampler atIndex:binding];
                break;
            case GC_DESCRIPTOR_ACCELERATION_STRUCTURE:
                [encoder setVertexAccelerationStructure:entry->value.accel atBufferIndex:binding];
                [encoder setFragmentAccelerationStructure:entry->value.accel atBufferIndex:binding];
                break;
        }
    }
}

static void ep_bind_descriptors_to_compute(id<MTLComputeCommandEncoder> encoder,
                                           GCDescriptorSet *set,
                                           uint32_t base_index) {
    for (uint32_t i = 0; i < set->entry_count; i++) {
        uint32_t binding = set->layout->bindings[i].binding + base_index;
        GCDescriptorSetEntry *entry = &set->entries[i];

        switch (entry->type) {
            case GC_DESCRIPTOR_UNIFORM_BUFFER:
            case GC_DESCRIPTOR_STORAGE_BUFFER:
                [encoder setBuffer:entry->value.buffer_info.buffer
                            offset:entry->value.buffer_info.offset
                           atIndex:binding];
                break;
            case GC_DESCRIPTOR_SAMPLED_TEXTURE:
            case GC_DESCRIPTOR_STORAGE_TEXTURE:
                [encoder setTexture:entry->value.texture atIndex:binding];
                break;
            case GC_DESCRIPTOR_SAMPLER:
                [encoder setSamplerState:entry->value.sampler atIndex:binding];
                break;
            case GC_DESCRIPTOR_ACCELERATION_STRUCTURE:
                [encoder setAccelerationStructure:entry->value.accel atBufferIndex:binding];
                break;
        }
    }
}

GCError GCCommandBindDescriptorSet(GCCommandBufferPtr command_buffer, GCPipelineLayoutPtr layout,
                                   uint32_t set_index, GCDescriptorSetPtr set) {
    if (!command_buffer || !set) return ep_invalid_argument("command_buffer or set is NULL");

    @autoreleasepool {
        // Calculate base binding index for this set
        uint32_t base_index = 0;
        if (layout) {
            for (uint32_t i = 0; i < set_index && i < layout->set_layout_count; i++) {
                base_index += layout->set_layouts[i]->binding_count;
            }
        }

        if (command_buffer->render_encoder) {
            ep_bind_descriptors_to_render(command_buffer->render_encoder, set, base_index);
        } else if (command_buffer->compute_encoder) {
            ep_bind_descriptors_to_compute(command_buffer->compute_encoder, set, base_index);
        }
    }

    return ep_ok();
}

GCError GCCommandPushConstants(GCCommandBufferPtr command_buffer, GCPipelineLayoutPtr layout,
                               GCShaderStageFlags stages, const uint8_t *data, uint32_t size) {
    if (!command_buffer || !data) return ep_invalid_argument("command_buffer or data is NULL");

    @autoreleasepool {
        // Push constants are emulated via setBytes in Metal
        if (command_buffer->render_encoder) {
            if (stages & (GC_STAGE_VERTEX_BIT | GC_STAGE_MESH_BIT | GC_STAGE_TASK_BIT)) {
                [command_buffer->render_encoder setVertexBytes:data length:size atIndex:30];
            }
            if (stages & GC_STAGE_FRAGMENT_BIT) {
                [command_buffer->render_encoder setFragmentBytes:data length:size atIndex:30];
            }
        } else if (command_buffer->compute_encoder) {
            [command_buffer->compute_encoder setBytes:data length:size atIndex:30];
        }
    }

    return ep_ok();
}

#pragma mark - Command Encoding: Viewport/Scissor

GCError GCCommandSetViewport(GCCommandBufferPtr command_buffer, float x, float y,
                             float width, float height, float min_depth, float max_depth) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (!command_buffer->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

    @autoreleasepool {
        MTLViewport viewport = { x, y, width, height, min_depth, max_depth };
        [command_buffer->render_encoder setViewport:viewport];
    }

    return ep_ok();
}

GCError GCCommandSetScissor(GCCommandBufferPtr command_buffer, uint32_t x, uint32_t y,
                            uint32_t width, uint32_t height) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (!command_buffer->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

    @autoreleasepool {
        MTLScissorRect scissor = { x, y, width, height };
        [command_buffer->render_encoder setScissorRect:scissor];
    }

    return ep_ok();
}

#pragma mark - Command Encoding: Draw/Dispatch

GCError GCCommandDraw(GCCommandBufferPtr command_buffer, uint32_t vertex_count,
                      uint32_t instance_count, uint32_t first_vertex, uint32_t first_instance) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (!command_buffer->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

    @autoreleasepool {
        [command_buffer->render_encoder drawPrimitives:MTLPrimitiveTypeTriangle
                                           vertexStart:first_vertex
                                           vertexCount:vertex_count
                                         instanceCount:instance_count
                                          baseInstance:first_instance];
    }

    return ep_ok();
}

GCError GCCommandDrawIndexed(GCCommandBufferPtr command_buffer, uint32_t index_count,
                             uint32_t instance_count, uint32_t first_index,
                             int32_t vertex_offset, uint32_t first_instance) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (!command_buffer->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

    // Note: Index buffer must be bound separately via setVertexBuffer
    // This is a simplified implementation
    return ep_unsupported("use GCCommandDraw or bind index buffer separately");
}

GCError GCCommandDispatch(GCCommandBufferPtr command_buffer, uint32_t group_x,
                          uint32_t group_y, uint32_t group_z) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (!command_buffer->compute_encoder) return ep_error(GC_E_INVALID_STATE, "no active compute encoder");

    @autoreleasepool {
        MTLSize threadgroups = MTLSizeMake(group_x, group_y, group_z);
        MTLSize threads_per_group = MTLSizeMake(32, 1, 1); // Default, should come from pipeline
        [command_buffer->compute_encoder dispatchThreadgroups:threadgroups
                                        threadsPerThreadgroup:threads_per_group];
    }

    return ep_ok();
}

GCError GCCommandDispatchMesh(GCCommandBufferPtr command_buffer, uint32_t group_x,
                              uint32_t group_y, uint32_t group_z) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (!command_buffer->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

    @autoreleasepool {
        MTLSize threadgroups = MTLSizeMake(group_x, group_y, group_z);
        MTLSize threads_per_object = MTLSizeMake(32, 1, 1);
        MTLSize threads_per_mesh = MTLSizeMake(32, 1, 1);

        [command_buffer->render_encoder drawMeshThreadgroups:threadgroups
                                   threadsPerObjectThreadgroup:threads_per_object
                                     threadsPerMeshThreadgroup:threads_per_mesh];
    }

    return ep_ok();
}

GCError GCCommandDispatchRays(GCCommandBufferPtr command_buffer, uint32_t width,
                              uint32_t height, uint32_t depth) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (!command_buffer->compute_encoder) return ep_error(GC_E_INVALID_STATE, "no active compute encoder");

    @autoreleasepool {
        // Ray dispatch is done via compute threadgroups in Metal
        MTLSize threadgroups = MTLSizeMake((width + 7) / 8, (height + 7) / 8, depth);
        MTLSize threads_per_group = MTLSizeMake(8, 8, 1);
        [command_buffer->compute_encoder dispatchThreadgroups:threadgroups
                                        threadsPerThreadgroup:threads_per_group];
    }

    return ep_ok();
}

GCError GCCommandResourceBarrier(GCCommandBufferPtr command_buffer, const GCBarrierDesc *desc) {
    if (!command_buffer || !desc) return ep_invalid_argument("command_buffer or desc is NULL");

    @autoreleasepool {
        // Metal handles most synchronization automatically within command buffers
        // Explicit barriers are mainly needed for UAV hazards

        if (command_buffer->render_encoder) {
            // Memory barrier for render
            MTLBarrierScope scope = MTLBarrierScopeBuffers | MTLBarrierScopeTextures;
            MTLRenderStages stages = MTLRenderStageVertex | MTLRenderStageFragment;
            [command_buffer->render_encoder memoryBarrierWithScope:scope
                                                       afterStages:stages
                                                      beforeStages:stages];
        } else if (command_buffer->compute_encoder) {
            [command_buffer->compute_encoder memoryBarrierWithScope:MTLBarrierScopeBuffers | MTLBarrierScopeTextures];
        }
    }

    return ep_ok();
}
