// enthrall_command.cpp - Command buffer and encoding implementation

#include "enthrall_internal.h"

// ============================================================================
// Command Buffer
// ============================================================================

extern "C" EPError EPCommandBufferCreate(EPDevicePtr device,
                                         EPCommandBufferPtr *out_command_buffer) {
    if (!device || !out_command_buffer) 
        return ep_invalid_argument("device or out_command_buffer is NULL");
    
    EPCommandBuffer *cmd = new (std::nothrow) EPCommandBuffer{};
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

extern "C" EPError EPCommandBufferDestroy(EPCommandBufferPtr command_buffer) {
    delete command_buffer;
    return ep_ok();
}

extern "C" EPError EPCommandBufferBegin(EPCommandBufferPtr command_buffer) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (command_buffer->is_recording) 
        return ep_error(EP_E_INVALID_STATE, "already recording");
    
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

extern "C" EPError EPCommandBufferEnd(EPCommandBufferPtr command_buffer) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    if (!command_buffer->is_recording) 
        return ep_error(EP_E_INVALID_STATE, "not recording");
    
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

extern "C" EPError EPCommandBeginRenderPass(EPCommandBufferPtr command_buffer,
                                            const EPRenderPassDesc *desc) {
    if (!command_buffer || !desc) 
        return ep_invalid_argument("command_buffer or desc is NULL");
    if (!command_buffer->is_recording) 
        return ep_error(EP_E_INVALID_STATE, "not recording");
    
    // End any existing render pass
    if (command_buffer->in_render_pass) {
        command_buffer->list->EndRenderPass();
    }
    
    // Build render pass description
    std::vector<D3D12_RENDER_PASS_RENDER_TARGET_DESC> rt_descs;
    rt_descs.reserve(desc->color_attachment_count);
    
    for (uint32_t i = 0; i < desc->color_attachment_count; i++) {
        const EPRenderPassColorAttachment *att = &desc->color_attachments[i];
        if (!att->texture) continue;
        
        D3D12_RENDER_PASS_RENDER_TARGET_DESC rt_desc = {};
        rt_desc.cpuDescriptor = att->texture->rtv;
        
        rt_desc.BeginningAccess.Type = ep_to_d3d12_load_op(att->load_op);
        if (att->load_op == EP_LOAD_OP_CLEAR) {
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
        if (desc->depth_attachment->load_op == EP_LOAD_OP_CLEAR) {
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

extern "C" EPError EPCommandEndRenderPass(EPCommandBufferPtr command_buffer) {
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

extern "C" EPError EPCommandBindRenderPipeline(EPCommandBufferPtr command_buffer,
                                               EPRenderPipelinePtr pipeline) {
    if (!command_buffer || !pipeline) 
        return ep_invalid_argument("command_buffer or pipeline is NULL");
    
    command_buffer->list->SetPipelineState(pipeline->state.Get());
    command_buffer->list->SetGraphicsRootSignature(pipeline->root_signature.Get());
    command_buffer->list->IASetPrimitiveTopology(pipeline->topology);
    command_buffer->current_root_signature = pipeline->root_signature.Get();
    command_buffer->current_topology = pipeline->topology;
    
    return ep_ok();
}

extern "C" EPError EPCommandBindMeshPipeline(EPCommandBufferPtr command_buffer,
                                             EPMeshPipelinePtr pipeline) {
    if (!command_buffer || !pipeline) 
        return ep_invalid_argument("command_buffer or pipeline is NULL");
    
    command_buffer->list->SetPipelineState(pipeline->state.Get());
    command_buffer->list->SetGraphicsRootSignature(pipeline->root_signature.Get());
    command_buffer->current_root_signature = pipeline->root_signature.Get();
    
    return ep_ok();
}

extern "C" EPError EPCommandBindComputePipeline(EPCommandBufferPtr command_buffer,
                                                EPComputePipelinePtr pipeline) {
    if (!command_buffer || !pipeline) 
        return ep_invalid_argument("command_buffer or pipeline is NULL");
    
    command_buffer->list->SetPipelineState(pipeline->state.Get());
    command_buffer->list->SetComputeRootSignature(pipeline->root_signature.Get());
    command_buffer->current_root_signature = pipeline->root_signature.Get();
    
    return ep_ok();
}

extern "C" EPError EPCommandBindRayTracingPipeline(EPCommandBufferPtr command_buffer,
                                                    EPRayTracingPipelinePtr pipeline) {
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

extern "C" EPError EPCommandBindDescriptorSet(EPCommandBufferPtr command_buffer,
                                              EPPipelineLayoutPtr layout,
                                              uint32_t set_index,
                                              EPDescriptorSetPtr set) {
    if (!command_buffer || !set) 
        return ep_invalid_argument("command_buffer or set is NULL");
    
    // In D3D12, we bind descriptor tables to root parameters
    // The root parameter index corresponds to the set index
    
    // For now, use a simplified model where each set maps to a root parameter
    // containing a descriptor table
    
    // Calculate GPU descriptor handle for this set
    D3D12_GPU_DESCRIPTOR_HANDLE gpu_handle = 
        command_buffer->ep_device->cbv_srv_uav_heap->GetGPUDescriptorHandleForHeapStart();
    
    // This is a simplified binding - real implementation would track descriptor offsets
    if (command_buffer->in_render_pass) {
        command_buffer->list->SetGraphicsRootDescriptorTable(set_index, gpu_handle);
    } else {
        command_buffer->list->SetComputeRootDescriptorTable(set_index, gpu_handle);
    }
    
    return ep_ok();
}

extern "C" EPError EPCommandPushConstants(EPCommandBufferPtr command_buffer,
                                          EPPipelineLayoutPtr layout,
                                          EPShaderStageFlags stages,
                                          const uint8_t *data, uint32_t size) {
    if (!command_buffer || !data) 
        return ep_invalid_argument("command_buffer or data is NULL");
    
    // Push constants in D3D12 are implemented via root constants
    // Typically at root parameter 0
    UINT num_32bit_values = (size + 3) / 4;
    
    if (command_buffer->in_render_pass) {
        command_buffer->list->SetGraphicsRoot32BitConstants(
            0, num_32bit_values, data, 0);
    } else {
        command_buffer->list->SetComputeRoot32BitConstants(
            0, num_32bit_values, data, 0);
    }
    
    return ep_ok();
}

// ============================================================================
// Command Encoding: Viewport/Scissor
// ============================================================================

extern "C" EPError EPCommandSetViewport(EPCommandBufferPtr command_buffer,
                                        float x, float y, float width, float height,
                                        float min_depth, float max_depth) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    D3D12_VIEWPORT viewport = {x, y, width, height, min_depth, max_depth};
    command_buffer->list->RSSetViewports(1, &viewport);
    
    return ep_ok();
}

extern "C" EPError EPCommandSetScissor(EPCommandBufferPtr command_buffer,
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

extern "C" EPError EPCommandDraw(EPCommandBufferPtr command_buffer,
                                 uint32_t vertex_count, uint32_t instance_count,
                                 uint32_t first_vertex, uint32_t first_instance) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    command_buffer->list->DrawInstanced(
        vertex_count, instance_count, first_vertex, first_instance);
    
    return ep_ok();
}

extern "C" EPError EPCommandDrawIndexed(EPCommandBufferPtr command_buffer,
                                        uint32_t index_count, uint32_t instance_count,
                                        uint32_t first_index, int32_t vertex_offset,
                                        uint32_t first_instance) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    command_buffer->list->DrawIndexedInstanced(
        index_count, instance_count, first_index, vertex_offset, first_instance);
    
    return ep_ok();
}

extern "C" EPError EPCommandDispatch(EPCommandBufferPtr command_buffer,
                                     uint32_t group_x, uint32_t group_y, uint32_t group_z) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    command_buffer->list->Dispatch(group_x, group_y, group_z);
    
    return ep_ok();
}

extern "C" EPError EPCommandDispatchMesh(EPCommandBufferPtr command_buffer,
                                         uint32_t group_x, uint32_t group_y, uint32_t group_z) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    command_buffer->list->DispatchMesh(group_x, group_y, group_z);
    
    return ep_ok();
}

extern "C" EPError EPCommandDispatchRays(EPCommandBufferPtr command_buffer,
                                         uint32_t width, uint32_t height, uint32_t depth) {
    if (!command_buffer) return ep_invalid_argument("command_buffer is NULL");
    
    // Need to have a ray tracing pipeline bound
    // This is a simplified implementation
    D3D12_DISPATCH_RAYS_DESC dispatch_desc = {};
    dispatch_desc.Width = width;
    dispatch_desc.Height = height;
    dispatch_desc.Depth = depth;
    
    // Shader table addresses would need to be set from the bound pipeline
    // For now, this is a placeholder
    
    command_buffer->list->DispatchRays(&dispatch_desc);
    
    return ep_ok();
}

// ============================================================================
// Command Encoding: Barriers
// ============================================================================

extern "C" EPError EPCommandResourceBarrier(EPCommandBufferPtr command_buffer,
                                            const EPBarrierDesc *desc) {
    if (!command_buffer || !desc) 
        return ep_invalid_argument("command_buffer or desc is NULL");
    
    std::vector<D3D12_RESOURCE_BARRIER> barriers;
    barriers.reserve(desc->buffer_barrier_count + desc->texture_barrier_count);
    
    // Buffer barriers
    for (uint32_t i = 0; i < desc->buffer_barrier_count; i++) {
        const EPBufferBarrier *buf_barrier = &desc->buffer_barriers[i];
        if (!buf_barrier->buffer) continue;
        
        D3D12_RESOURCE_BARRIER barrier = {};
        barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_UAV;
        barrier.UAV.pResource = buf_barrier->buffer->resource.Get();
        barriers.push_back(barrier);
    }
    
    // Texture barriers
    for (uint32_t i = 0; i < desc->texture_barrier_count; i++) {
        const EPTextureBarrier *tex_barrier = &desc->texture_barriers[i];
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
