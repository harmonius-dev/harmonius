// gcraft_command.cpp - Command buffer and encoding implementation

#include "gcraft_internal.h"

// ============================================================================
// Command Buffer
// ============================================================================

extern "C" GCError GCCommandBufferCreate(GCDevicePtr device,
                                         GCCommandBufferPtr *out_command_buffer) {
    if (!device || !out_command_buffer) 
        return ep_invalid_argument("device or out_command_buffer is NULL");
    
    GCCommandBuffer *cmd = new (std::nothrow) GCCommandBuffer{};
    if (!cmd) return ep_out_of_memory();
    
    cmd->ep_device = device;
    cmd->is_recording = false;
    cmd->in_render_pass = false;
    cmd->current_root_signature = nullptr;
    cmd->current_topology = D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
    
    // Create command allocator
    HRESULT hr = device->device->CreateCommandAllocator(
        D3D12_COMMAND_LIST_TYPE_DIRECT,
        IID_PPV_ARGS(&cmd->allocator));
    if (FAILED(hr)) {
        delete cmd;
        return ep_from_hresult(hr, "failed to create command allocator");
    }
    
    // Create command list (closed state)
    hr = device->device->CreateCommandList1(
        0, D3D12_COMMAND_LIST_TYPE_DIRECT,
        D3D12_COMMAND_LIST_FLAG_NONE,
        IID_PPV_ARGS(&cmd->list));
    if (FAILED(hr)) {
        delete cmd;
        return ep_from_hresult(hr, "failed to create command list");
    }
    
    *out_command_buffer = cmd;
    return ep_ok();
}

extern "C" GCError GCCommandBufferDestroy(GCCommandBufferPtr command_buffer) {
    delete command_buffer;
    return ep_ok();
}

extern "C" GCError GCCommandBufferBegin(GCCommandBufferPtr command_buffer) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (command_buffer->is_recording) 
        return ep_error(GC_E_INVALID_STATE, "already recording");
    
    HRESULT hr = command_buffer->allocator->Reset();
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to reset allocator");
    
    hr = command_buffer->list->Reset(command_buffer->allocator.Get(), nullptr);
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to reset command list");
    
    // Set descriptor heaps
    ID3D12DescriptorHeap *heaps[] = {
        command_buffer->ep_device->cbv_srv_uav_heap.Get(),
        command_buffer->ep_device->sampler_heap.Get()
    };
    command_buffer->list->SetDescriptorHeaps(2, heaps);
    
    command_buffer->is_recording = true;
    command_buffer->in_render_pass = false;
    
    return ep_ok();
}

extern "C" GCError GCCommandBufferEnd(GCCommandBufferPtr command_buffer) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (!command_buffer->is_recording) 
        return ep_error(GC_E_INVALID_STATE, "not recording");
    
    // End any active render pass
    if (command_buffer->in_render_pass) {
        command_buffer->list->EndRenderPass();
        command_buffer->in_render_pass = false;
    }
    
    HRESULT hr = command_buffer->list->Close();
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to close command list");
    
    command_buffer->is_recording = false;
    return ep_ok();
}

// ============================================================================
// Command Encoding: Render Pass
// ============================================================================

extern "C" GCError GCCommandBeginRenderPass(GCCommandBufferPtr command_buffer,
                                            const GCRenderPassDesc *desc) {
    if (!command_buffer || !desc) 
        return ep_invalid_argument("command_buffer or desc is NULL");
    if (!command_buffer->is_recording) 
        return ep_error(GC_E_INVALID_STATE, "not recording");
    
    // End any existing render pass
    if (command_buffer->in_render_pass) {
        command_buffer->list->EndRenderPass();
    }
    
    // Build render pass description
    std::vector<D3D12_RENDER_PASS_RENDER_TARGET_DESC> rt_descs;
    rt_descs.reserve(desc->color_attachment_count);
    
    for (uint32_t i = 0; i < desc->color_attachment_count; i++) {
        const GCRenderPassColorAttachment *att = &desc->color_attachments[i];
        if (!att->texture) continue;
        
        D3D12_RENDER_PASS_RENDER_TARGET_DESC rt_desc = {};
        rt_desc.cpuDescriptor = att->texture->rtv;
        
        rt_desc.BeginningAccess.Type = ep_to_d3d12_load_op(att->load_op);
        if (att->load_op == GC_LOAD_OP_CLEAR) {
            rt_desc.BeginningAccess.Clear.ClearValue.Format = 
                ep_to_dxgi_format(att->texture->desc.format);
            memcpy(rt_desc.BeginningAccess.Clear.ClearValue.Color,
                   att->clear_color, sizeof(float) * 4);
        }
        
        rt_desc.EndingAccess.Type = ep_to_d3d12_store_op(att->store_op);
        
        rt_descs.push_back(rt_desc);
    }
    
    D3D12_RENDER_PASS_DEPTH_STENCIL_DESC ds_desc = {};
    D3D12_RENDER_PASS_DEPTH_STENCIL_DESC *ds_desc_ptr = nullptr;
    
    if (desc->depth_attachment && desc->depth_attachment->texture) {
        ds_desc.cpuDescriptor = desc->depth_attachment->texture->dsv;
        
        ds_desc.DepthBeginningAccess.Type = 
            ep_to_d3d12_load_op(desc->depth_attachment->load_op);
        if (desc->depth_attachment->load_op == GC_LOAD_OP_CLEAR) {
            ds_desc.DepthBeginningAccess.Clear.ClearValue.Format = 
                ep_to_dxgi_format(desc->depth_attachment->texture->desc.format);
            ds_desc.DepthBeginningAccess.Clear.ClearValue.DepthStencil.Depth = 
                desc->depth_attachment->clear_depth;
            ds_desc.DepthBeginningAccess.Clear.ClearValue.DepthStencil.Stencil = 
                static_cast<UINT8>(desc->depth_attachment->clear_stencil);
        }
        
        ds_desc.DepthEndingAccess.Type = 
            ep_to_d3d12_store_op(desc->depth_attachment->store_op);
        
        ds_desc.StencilBeginningAccess.Type = 
            ep_to_d3d12_load_op(desc->depth_attachment->load_op);
        ds_desc.StencilEndingAccess.Type = 
            ep_to_d3d12_store_op(desc->depth_attachment->store_op);
        
        ds_desc_ptr = &ds_desc;
    }
    
    command_buffer->list->BeginRenderPass(
        static_cast<UINT>(rt_descs.size()),
        rt_descs.data(),
        ds_desc_ptr,
        D3D12_RENDER_PASS_FLAG_NONE);
    
    command_buffer->in_render_pass = true;
    return ep_ok();
}

extern "C" GCError GCCommandEndRenderPass(GCCommandBufferPtr command_buffer) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    if (command_buffer->in_render_pass) {
        command_buffer->list->EndRenderPass();
        command_buffer->in_render_pass = false;
    }
    
    return ep_ok();
}

// ============================================================================
// Command Encoding: Pipeline Binding
// ============================================================================

extern "C" GCError GCCommandBindRenderPipeline(GCCommandBufferPtr command_buffer,
                                               GCRenderPipelinePtr pipeline) {
    if (!command_buffer || !pipeline) 
        return ep_invalid_argument("command_buffer or pipeline is NULL");
    
    command_buffer->list->SetPipelineState(pipeline->state.Get());
    command_buffer->list->SetGraphicsRootSignature(pipeline->root_signature.Get());
    command_buffer->list->IASetPrimitiveTopology(pipeline->topology);
    command_buffer->current_root_signature = pipeline->root_signature.Get();
    command_buffer->current_topology = pipeline->topology;
    
    return ep_ok();
}

extern "C" GCError GCCommandBindMeshPipeline(GCCommandBufferPtr command_buffer,
                                             GCMeshPipelinePtr pipeline) {
    if (!command_buffer || !pipeline) 
        return ep_invalid_argument("command_buffer or pipeline is NULL");
    
    command_buffer->list->SetPipelineState(pipeline->state.Get());
    command_buffer->list->SetGraphicsRootSignature(pipeline->root_signature.Get());
    command_buffer->current_root_signature = pipeline->root_signature.Get();
    
    return ep_ok();
}

extern "C" GCError GCCommandBindComputePipeline(GCCommandBufferPtr command_buffer,
                                                GCComputePipelinePtr pipeline) {
    if (!command_buffer || !pipeline) 
        return ep_invalid_argument("command_buffer or pipeline is NULL");
    
    command_buffer->list->SetPipelineState(pipeline->state.Get());
    command_buffer->list->SetComputeRootSignature(pipeline->root_signature.Get());
    command_buffer->current_root_signature = pipeline->root_signature.Get();
    
    return ep_ok();
}

extern "C" GCError GCCommandBindRayTracingPipeline(GCCommandBufferPtr command_buffer,
                                                    GCRayTracingPipelinePtr pipeline) {
    if (!command_buffer || !pipeline) 
        return ep_invalid_argument("command_buffer or pipeline is NULL");
    
    command_buffer->list->SetPipelineState1(pipeline->state.Get());
    command_buffer->list->SetComputeRootSignature(pipeline->root_signature.Get());
    command_buffer->current_root_signature = pipeline->root_signature.Get();
    
    return ep_ok();
}

// ============================================================================
// Command Encoding: Descriptor Binding
// ============================================================================

extern "C" GCError GCCommandBindDescriptorSet(GCCommandBufferPtr command_buffer,
                                              GCPipelineLayoutPtr layout,
                                              uint32_t set_index,
                                              GCDescriptorSetPtr set) {
    if (!command_buffer || !layout || !set) 
        return ep_invalid_argument("command_buffer, layout, or set is NULL");
    
    if (set_index >= layout->set_layouts.size()) {
        return ep_invalid_argument("set_index out of range");
    }
    
    // Calculate root parameter index accounting for push constants
    // Push constants (if present) take the first root parameter
    // Then descriptor tables follow
    UINT root_param_index = layout->first_set_root_index + set_index;
    
    // For each descriptor in the set, we need to bind to the correct offset
    // in the shader-visible heap. Get the base GPU descriptor handle for the set.
    D3D12_GPU_DESCRIPTOR_HANDLE cbv_srv_uav_gpu_handle = 
        command_buffer->ep_device->cbv_srv_uav_heap->GetGPUDescriptorHandleForHeapStart();
    D3D12_GPU_DESCRIPTOR_HANDLE sampler_gpu_handle = 
        command_buffer->ep_device->sampler_heap->GetGPUDescriptorHandleForHeapStart();
    
    // Find the first descriptor index in this set to compute the table start
    // For simplicity, we use the first buffer/texture entry's heap index
    bool has_cbv_srv_uav = false;
    bool has_samplers = false;
    UINT first_cbv_srv_uav_index = 0;
    UINT first_sampler_index = 0;
    
    for (const auto &entry : set->entries) {
        if (entry.type == GC_DESCRIPTOR_SAMPLER) {
            if (!has_samplers) {
                first_sampler_index = entry.value.sampler_info.heap_index;
                has_samplers = true;
            }
        } else if (entry.type == GC_DESCRIPTOR_UNIFORM_BUFFER || 
                   entry.type == GC_DESCRIPTOR_STORAGE_BUFFER) {
            if (!has_cbv_srv_uav) {
                first_cbv_srv_uav_index = entry.value.buffer_info.heap_index;
                has_cbv_srv_uav = true;
            }
        } else if (entry.type == GC_DESCRIPTOR_SAMPLED_TEXTURE ||
                   entry.type == GC_DESCRIPTOR_STORAGE_TEXTURE) {
            if (!has_cbv_srv_uav) {
                first_cbv_srv_uav_index = entry.value.texture_info.heap_index;
                has_cbv_srv_uav = true;
            }
        }
    }
    
    // Bind CBV/SRV/UAV descriptor table if there are any such descriptors
    if (has_cbv_srv_uav) {
        cbv_srv_uav_gpu_handle.ptr += first_cbv_srv_uav_index * 
            command_buffer->ep_device->cbv_srv_uav_descriptor_size;
        
        if (command_buffer->in_render_pass) {
            command_buffer->list->SetGraphicsRootDescriptorTable(root_param_index, cbv_srv_uav_gpu_handle);
        } else {
            command_buffer->list->SetComputeRootDescriptorTable(root_param_index, cbv_srv_uav_gpu_handle);
        }
        root_param_index++;
    }
    
    // Bind sampler descriptor table if there are any samplers
    if (has_samplers) {
        sampler_gpu_handle.ptr += first_sampler_index * 
            command_buffer->ep_device->sampler_descriptor_size;
        
        if (command_buffer->in_render_pass) {
            command_buffer->list->SetGraphicsRootDescriptorTable(root_param_index, sampler_gpu_handle);
        } else {
            command_buffer->list->SetComputeRootDescriptorTable(root_param_index, sampler_gpu_handle);
        }
    }
    
    return ep_ok();
}

extern "C" GCError GCCommandPushConstants(GCCommandBufferPtr command_buffer,
                                          GCPipelineLayoutPtr layout,
                                          GCShaderStageFlags stages,
                                          const uint8_t *data, uint32_t size) {
    if (!command_buffer || !layout || !data) 
        return ep_invalid_argument("command_buffer, layout, or data is NULL");
    
    // Validate that the requested push constant size fits the layout
    if (size > layout->push_constant_size) {
        return ep_invalid_argument("push constant size exceeds pipeline layout limit");
    }
    
    if (layout->push_constant_size == 0) {
        return ep_invalid_argument("pipeline layout has no push constants");
    }
    
    // Push constants in D3D12 are implemented via root constants
    // Use the root parameter index assigned by GCPipelineLayoutCreate
    UINT num_32bit_values = (size + 3) / 4;
    UINT root_param_index = layout->push_constant_root_index;
    
    if (command_buffer->in_render_pass) {
        command_buffer->list->SetGraphicsRoot32BitConstants(
            root_param_index, num_32bit_values, data, 0);
    } else {
        command_buffer->list->SetComputeRoot32BitConstants(
            root_param_index, num_32bit_values, data, 0);
    }
    
    return ep_ok();
}

// ============================================================================
// Command Encoding: Viewport/Scissor
// ============================================================================

extern "C" GCError GCCommandSetViewport(GCCommandBufferPtr command_buffer,
                                        float x, float y, float width, float height,
                                        float min_depth, float max_depth) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    D3D12_VIEWPORT viewport = {x, y, width, height, min_depth, max_depth};
    command_buffer->list->RSSetViewports(1, &viewport);
    
    return ep_ok();
}

extern "C" GCError GCCommandSetScissor(GCCommandBufferPtr command_buffer,
                                       uint32_t x, uint32_t y,
                                       uint32_t width, uint32_t height) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    D3D12_RECT scissor = {
        static_cast<LONG>(x),
        static_cast<LONG>(y),
        static_cast<LONG>(x + width),
        static_cast<LONG>(y + height)
    };
    command_buffer->list->RSSetScissorRects(1, &scissor);
    
    return ep_ok();
}

// ============================================================================
// Command Encoding: Draw/Dispatch
// ============================================================================

extern "C" GCError GCCommandDraw(GCCommandBufferPtr command_buffer,
                                 uint32_t vertex_count, uint32_t instance_count,
                                 uint32_t first_vertex, uint32_t first_instance) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    command_buffer->list->DrawInstanced(
        vertex_count, instance_count, first_vertex, first_instance);
    
    return ep_ok();
}

extern "C" GCError GCCommandDrawIndexed(GCCommandBufferPtr command_buffer,
                                        uint32_t index_count, uint32_t instance_count,
                                        uint32_t first_index, int32_t vertex_offset,
                                        uint32_t first_instance) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    command_buffer->list->DrawIndexedInstanced(
        index_count, instance_count, first_index, vertex_offset, first_instance);
    
    return ep_ok();
}

extern "C" GCError GCCommandDispatch(GCCommandBufferPtr command_buffer,
                                     uint32_t group_x, uint32_t group_y, uint32_t group_z) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    command_buffer->list->Dispatch(group_x, group_y, group_z);
    
    return ep_ok();
}

extern "C" GCError GCCommandDispatchMesh(GCCommandBufferPtr command_buffer,
                                         uint32_t group_x, uint32_t group_y, uint32_t group_z) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    command_buffer->list->DispatchMesh(group_x, group_y, group_z);
    
    return ep_ok();
}

extern "C" GCError GCCommandDispatchRays(GCCommandBufferPtr command_buffer,
                                         uint32_t width, uint32_t height, uint32_t depth) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    // Ray tracing dispatch requires properly configured shader tables (raygen, miss,
    // hit group, callable) from a bound ray tracing pipeline. Full shader table
    // management is not yet implemented, so reject the call rather than issuing
    // an invalid DispatchRays that would cause D3D12 validation errors or GPU faults.
    (void)width;
    (void)height;
    (void)depth;
    
    return ep_invalid_state("DispatchRays is not supported: ray tracing shader tables are not configured");
}

// ============================================================================
// Command Encoding: Barriers
// ============================================================================

extern "C" GCError GCCommandResourceBarrier(GCCommandBufferPtr command_buffer,
                                            const GCBarrierDesc *desc) {
    if (!command_buffer || !desc) 
        return ep_invalid_argument("command_buffer or desc is NULL");
    
    std::vector<D3D12_RESOURCE_BARRIER> barriers;
    barriers.reserve(desc->buffer_barrier_count + desc->texture_barrier_count);
    
    // Buffer barriers
    for (uint32_t i = 0; i < desc->buffer_barrier_count; i++) {
        const GCBufferBarrier *buf_barrier = &desc->buffer_barriers[i];
        if (!buf_barrier->buffer) continue;
        
        D3D12_RESOURCE_BARRIER barrier = {};
        barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_UAV;
        barrier.UAV.pResource = buf_barrier->buffer->resource.Get();
        barriers.push_back(barrier);
    }
    
    // Texture barriers
    for (uint32_t i = 0; i < desc->texture_barrier_count; i++) {
        const GCTextureBarrier *tex_barrier = &desc->texture_barriers[i];
        if (!tex_barrier->texture) continue;
        
        D3D12_RESOURCE_STATES before = ep_to_d3d12_resource_state(tex_barrier->old_layout);
        D3D12_RESOURCE_STATES after = ep_to_d3d12_resource_state(tex_barrier->new_layout);
        
        if (before != after) {
            D3D12_RESOURCE_BARRIER barrier = {};
            barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
            barrier.Transition.pResource = tex_barrier->texture->resource.Get();
            barrier.Transition.StateBefore = before;
            barrier.Transition.StateAfter = after;
            barrier.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
            barriers.push_back(barrier);
            
            tex_barrier->texture->current_state = after;
        }
    }
    
    if (!barriers.empty()) {
        command_buffer->list->ResourceBarrier(
            static_cast<UINT>(barriers.size()),
            barriers.data());
    }
    
    return ep_ok();
}
