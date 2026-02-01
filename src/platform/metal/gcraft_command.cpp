// gcraft_command.cpp - Command buffer and encoding implementation (Metal-cpp)

#include "gcraft_internal.h"

static void ep_bind_descriptors_to_render(MTL::RenderCommandEncoder* encoder,
                                          GCDescriptorSet* set,
                                          uint32_t base_index);
static void ep_bind_descriptors_to_compute(MTL::ComputeCommandEncoder* encoder,
                                           GCDescriptorSet* set,
                                           uint32_t base_index);

extern "C" {

GCError GCCommandBufferCreate(GCDevicePtr device, GCCommandBufferPtr* out_command_buffer) {
  if (!device || !out_command_buffer) return ep_invalid_argument("device or out_command_buffer is NULL");

  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(calloc(1, sizeof(GCCommandBuffer)));
  if (!cmd) return ep_out_of_memory();

  cmd->ep_device = device;
  cmd->is_recording = false;

  *out_command_buffer = cmd;
  return ep_ok();
}

GCError GCCommandBufferDestroy(GCCommandBufferPtr command_buffer) {
  if (command_buffer) {
    GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
    if (cmd->buffer) cmd->buffer->release();
    free(command_buffer);
  }
  return ep_ok();
}

GCError GCCommandBufferBegin(GCCommandBufferPtr command_buffer) {
  if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (cmd->is_recording) return ep_error(GC_E_INVALID_STATE, "already recording");

  if (cmd->buffer) cmd->buffer->release();

  cmd->buffer = cmd->ep_device->graphics_queue->commandBuffer();
  if (!cmd->buffer) {
    return ep_error(GC_E_DEVICE_LOST, "failed to create command buffer");
  }
  cmd->buffer->retain();
  cmd->is_recording = true;

  return ep_ok();
}

GCError GCCommandBufferEnd(GCCommandBufferPtr command_buffer) {
  if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (!cmd->is_recording) return ep_error(GC_E_INVALID_STATE, "not recording");

  if (cmd->render_encoder) {
    cmd->render_encoder->endEncoding();
    cmd->render_encoder->release();
    cmd->render_encoder = nullptr;
  }
  if (cmd->compute_encoder) {
    cmd->compute_encoder->endEncoding();
    cmd->compute_encoder->release();
    cmd->compute_encoder = nullptr;
  }
  if (cmd->blit_encoder) {
    cmd->blit_encoder->endEncoding();
    cmd->blit_encoder->release();
    cmd->blit_encoder = nullptr;
  }
  if (cmd->accel_encoder) {
    cmd->accel_encoder->endEncoding();
    cmd->accel_encoder->release();
    cmd->accel_encoder = nullptr;
  }

  cmd->is_recording = false;
  return ep_ok();
}

GCError GCCommandBeginRenderPass(GCCommandBufferPtr command_buffer, const GCRenderPassDesc* desc) {
  if (!command_buffer || !desc) return ep_invalid_argument("command_buffer or desc is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (!cmd->is_recording) return ep_error(GC_E_INVALID_STATE, "not recording");

  if (cmd->render_encoder) {
    cmd->render_encoder->endEncoding();
    cmd->render_encoder->release();
    cmd->render_encoder = nullptr;
  }
  if (cmd->compute_encoder) {
    cmd->compute_encoder->endEncoding();
    cmd->compute_encoder->release();
    cmd->compute_encoder = nullptr;
  }

  MTL::RenderPassDescriptor* pass_desc = MTL::RenderPassDescriptor::alloc()->init();

  for (uint32_t i = 0; i < desc->color_attachment_count; i++) {
    const GCRenderPassColorAttachment* att = &desc->color_attachments[i];
    if (!att->texture) continue;
    GCTexture* tex = static_cast<GCTexture*>(const_cast<void*>(static_cast<const void*>(att->texture)));
    pass_desc->colorAttachments()->object(i)->setTexture(tex->texture);
    pass_desc->colorAttachments()->object(i)->setLoadAction(ep_to_mtl_load_action(att->load_op));
    pass_desc->colorAttachments()->object(i)->setStoreAction(ep_to_mtl_store_action(att->store_op));
    pass_desc->colorAttachments()->object(i)->setClearColor(MTL::ClearColor(att->clear_color[0], att->clear_color[1], att->clear_color[2], att->clear_color[3]));
  }

  if (desc->depth_attachment && desc->depth_attachment->texture) {
    GCTexture* tex = static_cast<GCTexture*>(const_cast<void*>(static_cast<const void*>(desc->depth_attachment->texture)));
    pass_desc->depthAttachment()->setTexture(tex->texture);
    pass_desc->depthAttachment()->setLoadAction(ep_to_mtl_load_action(desc->depth_attachment->load_op));
    pass_desc->depthAttachment()->setStoreAction(ep_to_mtl_store_action(desc->depth_attachment->store_op));
    pass_desc->depthAttachment()->setClearDepth(desc->depth_attachment->clear_depth);
  }

  cmd->render_encoder = cmd->buffer->renderCommandEncoder(pass_desc);
  pass_desc->release();

  if (!cmd->render_encoder) {
    return ep_error(GC_E_DEVICE_LOST, "failed to create render encoder");
  }

  return ep_ok();
}

GCError GCCommandEndRenderPass(GCCommandBufferPtr command_buffer) {
  if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (cmd->render_encoder) {
    cmd->render_encoder->endEncoding();
    cmd->render_encoder->release();
    cmd->render_encoder = nullptr;
  }
  return ep_ok();
}

GCError GCCommandBindRenderPipeline(GCCommandBufferPtr command_buffer, GCRenderPipelinePtr pipeline) {
  if (!command_buffer || !pipeline) return ep_invalid_argument("command_buffer or pipeline is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (!cmd->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

  GCRenderPipeline* pipe = static_cast<GCRenderPipeline*>(pipeline);
  cmd->render_encoder->setRenderPipelineState(pipe->state);
  if (pipe->depth_stencil) {
    cmd->render_encoder->setDepthStencilState(pipe->depth_stencil);
  }
  return ep_ok();
}

GCError GCCommandBindMeshPipeline(GCCommandBufferPtr command_buffer, GCMeshPipelinePtr pipeline) {
  if (!command_buffer || !pipeline) return ep_invalid_argument("command_buffer or pipeline is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (!cmd->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

  GCMeshPipeline* pipe = static_cast<GCMeshPipeline*>(pipeline);
  cmd->render_encoder->setRenderPipelineState(pipe->state);
  if (pipe->depth_stencil) {
    cmd->render_encoder->setDepthStencilState(pipe->depth_stencil);
  }
  return ep_ok();
}

GCError GCCommandBindComputePipeline(GCCommandBufferPtr command_buffer, GCComputePipelinePtr pipeline) {
  if (!command_buffer || !pipeline) return ep_invalid_argument("command_buffer or pipeline is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);

  if (!cmd->compute_encoder) {
    if (cmd->render_encoder) {
      cmd->render_encoder->endEncoding();
      cmd->render_encoder->release();
      cmd->render_encoder = nullptr;
    }
    cmd->compute_encoder = cmd->buffer->computeCommandEncoder();
  }

  GCComputePipeline* pipe = static_cast<GCComputePipeline*>(pipeline);
  cmd->compute_encoder->setComputePipelineState(pipe->state);
  return ep_ok();
}

GCError GCCommandBindRayTracingPipeline(GCCommandBufferPtr command_buffer, GCRayTracingPipelinePtr pipeline) {
  if (!command_buffer || !pipeline) return ep_invalid_argument("command_buffer or pipeline is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  GCRayTracingPipeline* pipe = static_cast<GCRayTracingPipeline*>(pipeline);

  if (!cmd->compute_encoder) {
    if (cmd->render_encoder) {
      cmd->render_encoder->endEncoding();
      cmd->render_encoder->release();
      cmd->render_encoder = nullptr;
    }
    cmd->compute_encoder = cmd->buffer->computeCommandEncoder();
  }

  if (pipe->states && pipe->states->count() > 0) {
    cmd->compute_encoder->setComputePipelineState(static_cast<MTL::ComputePipelineState*>(pipe->states->object(0)));
  }
  if (pipe->function_table) {
    cmd->compute_encoder->setVisibleFunctionTable(pipe->function_table, 0);
  }
  return ep_ok();
}

GCError GCCommandBindDescriptorSet(GCCommandBufferPtr command_buffer, GCPipelineLayoutPtr layout,
                                   uint32_t set_index, GCDescriptorSetPtr set) {
  if (!command_buffer || !set) return ep_invalid_argument("command_buffer or set is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  GCDescriptorSet* s = static_cast<GCDescriptorSet*>(set);

  uint32_t base_index = 0;
  if (layout) {
    GCPipelineLayout* lay = static_cast<GCPipelineLayout*>(layout);
    for (uint32_t i = 0; i < set_index && i < lay->set_layout_count; i++) {
      base_index += lay->set_layouts[i]->binding_count;
    }
  }

  if (cmd->render_encoder) {
    ep_bind_descriptors_to_render(cmd->render_encoder, s, base_index);
  } else if (cmd->compute_encoder) {
    ep_bind_descriptors_to_compute(cmd->compute_encoder, s, base_index);
  }
  return ep_ok();
}

GCError GCCommandPushConstants(GCCommandBufferPtr command_buffer, GCPipelineLayoutPtr layout,
                               GCShaderStageFlags stages, const uint8_t* data, uint32_t size) {
  if (!command_buffer || !data) return ep_invalid_argument("command_buffer or data is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);

  if (cmd->render_encoder) {
    if (stages & (GC_STAGE_VERTEX_BIT | GC_STAGE_MESH_BIT | GC_STAGE_TASK_BIT)) {
      cmd->render_encoder->setVertexBytes(data, size, 30);
    }
    if (stages & GC_STAGE_FRAGMENT_BIT) {
      cmd->render_encoder->setFragmentBytes(data, size, 30);
    }
  } else if (cmd->compute_encoder) {
    cmd->compute_encoder->setBytes(data, size, 30);
  }
  return ep_ok();
}

GCError GCCommandSetViewport(GCCommandBufferPtr command_buffer, float x, float y,
                             float width, float height, float min_depth, float max_depth) {
  if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (!cmd->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

  MTL::Viewport viewport;
  viewport.originX = x;
  viewport.originY = y;
  viewport.width = width;
  viewport.height = height;
  viewport.znear = min_depth;
  viewport.zfar = max_depth;
  cmd->render_encoder->setViewport(viewport);
  return ep_ok();
}

GCError GCCommandSetScissor(GCCommandBufferPtr command_buffer, uint32_t x, uint32_t y,
                            uint32_t width, uint32_t height) {
  if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (!cmd->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

  MTL::ScissorRect scissor;
  scissor.x = x;
  scissor.y = y;
  scissor.width = width;
  scissor.height = height;
  cmd->render_encoder->setScissorRect(scissor);
  return ep_ok();
}

GCError GCCommandDraw(GCCommandBufferPtr command_buffer, uint32_t vertex_count,
                      uint32_t instance_count, uint32_t first_vertex, uint32_t first_instance) {
  if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (!cmd->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

  cmd->render_encoder->drawPrimitives(MTL::PrimitiveTypeTriangle, NS::UInteger(first_vertex), NS::UInteger(vertex_count), NS::UInteger(instance_count), NS::UInteger(first_instance));
  return ep_ok();
}

GCError GCCommandDrawIndexed(GCCommandBufferPtr command_buffer, uint32_t index_count,
                             uint32_t instance_count, uint32_t first_index,
                             int32_t vertex_offset, uint32_t first_instance) {
  (void)command_buffer;
  (void)index_count;
  (void)instance_count;
  (void)first_index;
  (void)vertex_offset;
  (void)first_instance;
  return ep_unsupported("use GCCommandDraw or bind index buffer separately");
}

GCError GCCommandDispatch(GCCommandBufferPtr command_buffer, uint32_t group_x,
                          uint32_t group_y, uint32_t group_z) {
  if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (!cmd->compute_encoder) return ep_error(GC_E_INVALID_STATE, "no active compute encoder");

  MTL::Size threadgroups(group_x, group_y, group_z);
  MTL::Size threads_per_group(32, 1, 1);
  cmd->compute_encoder->dispatchThreadgroups(threadgroups, threads_per_group);
  return ep_ok();
}

GCError GCCommandDispatchMesh(GCCommandBufferPtr command_buffer, uint32_t group_x,
                              uint32_t group_y, uint32_t group_z) {
  if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (!cmd->render_encoder) return ep_error(GC_E_INVALID_STATE, "no active render pass");

  MTL::Size threadgroups(group_x, group_y, group_z);
  MTL::Size threads_per_object(32, 1, 1);
  MTL::Size threads_per_mesh(32, 1, 1);
  cmd->render_encoder->drawMeshThreadgroups(threadgroups, threads_per_object, threads_per_mesh);
  return ep_ok();
}

GCError GCCommandDispatchRays(GCCommandBufferPtr command_buffer, uint32_t width,
                              uint32_t height, uint32_t depth) {
  if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);
  if (!cmd->compute_encoder) return ep_error(GC_E_INVALID_STATE, "no active compute encoder");

  MTL::Size threadgroups((width + 7) / 8, (height + 7) / 8, depth);
  MTL::Size threads_per_group(8, 8, 1);
  cmd->compute_encoder->dispatchThreadgroups(threadgroups, threads_per_group);
  return ep_ok();
}

GCError GCCommandResourceBarrier(GCCommandBufferPtr command_buffer, const GCBarrierDesc* desc) {
  if (!command_buffer || !desc) return ep_invalid_argument("command_buffer or desc is NULL");
  GCCommandBuffer* cmd = static_cast<GCCommandBuffer*>(command_buffer);

  if (cmd->render_encoder) {
    MTL::BarrierScope scope = MTL::BarrierScopeBuffers | MTL::BarrierScopeTextures;
    MTL::RenderStages stages = MTL::RenderStageVertex | MTL::RenderStageFragment;
    cmd->render_encoder->memoryBarrier(scope, stages, stages);
  } else if (cmd->compute_encoder) {
    cmd->compute_encoder->memoryBarrier(MTL::BarrierScopeBuffers | MTL::BarrierScopeTextures);
  }
  return ep_ok();
}

}  // extern "C"

static void ep_bind_descriptors_to_render(MTL::RenderCommandEncoder* encoder,
                                          GCDescriptorSet* set,
                                          uint32_t base_index) {
  for (uint32_t i = 0; i < set->entry_count; i++) {
    uint32_t binding = set->layout->bindings[i].binding + base_index;
    GCDescriptorSetEntry* entry = &set->entries[i];

    switch (entry->type) {
      case GC_DESCRIPTOR_UNIFORM_BUFFER:
      case GC_DESCRIPTOR_STORAGE_BUFFER:
        encoder->setVertexBuffer(entry->value.buffer_info.buffer, entry->value.buffer_info.offset, binding);
        encoder->setFragmentBuffer(entry->value.buffer_info.buffer, entry->value.buffer_info.offset, binding);
        break;
      case GC_DESCRIPTOR_SAMPLED_TEXTURE:
      case GC_DESCRIPTOR_STORAGE_TEXTURE:
        encoder->setVertexTexture(entry->value.texture, binding);
        encoder->setFragmentTexture(entry->value.texture, binding);
        break;
      case GC_DESCRIPTOR_SAMPLER:
        encoder->setVertexSamplerState(entry->value.sampler, binding);
        encoder->setFragmentSamplerState(entry->value.sampler, binding);
        break;
      case GC_DESCRIPTOR_ACCELERATION_STRUCTURE:
        encoder->setVertexAccelerationStructure(entry->value.accel, binding);
        encoder->setFragmentAccelerationStructure(entry->value.accel, binding);
        break;
    }
  }
}

static void ep_bind_descriptors_to_compute(MTL::ComputeCommandEncoder* encoder,
                                           GCDescriptorSet* set,
                                           uint32_t base_index) {
  for (uint32_t i = 0; i < set->entry_count; i++) {
    uint32_t binding = set->layout->bindings[i].binding + base_index;
    GCDescriptorSetEntry* entry = &set->entries[i];

    switch (entry->type) {
      case GC_DESCRIPTOR_UNIFORM_BUFFER:
      case GC_DESCRIPTOR_STORAGE_BUFFER:
        encoder->setBuffer(entry->value.buffer_info.buffer, entry->value.buffer_info.offset, binding);
        break;
      case GC_DESCRIPTOR_SAMPLED_TEXTURE:
      case GC_DESCRIPTOR_STORAGE_TEXTURE:
        encoder->setTexture(entry->value.texture, binding);
        break;
      case GC_DESCRIPTOR_SAMPLER:
        encoder->setSamplerState(entry->value.sampler, binding);
        break;
      case GC_DESCRIPTOR_ACCELERATION_STRUCTURE:
        encoder->setAccelerationStructure(entry->value.accel, binding);
        break;
    }
  }
}
