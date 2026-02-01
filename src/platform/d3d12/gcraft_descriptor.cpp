// gcraft_descriptor.cpp - Descriptor set layout, descriptor set, and pipeline layout

#include "gcraft_internal.h"

// ============================================================================
// Descriptor Set Layout
// ============================================================================

extern "C" GCError GCDescriptorSetLayoutCreate(GCDevicePtr device,
                                               const GCDescriptorSetLayoutDesc *desc,
                                               GCDescriptorSetLayoutPtr *out_layout) {
    if (!device || !desc || !out_layout)
        return ep_invalid_argument("device, desc, or out_layout is NULL");
    
    GCDescriptorSetLayout *layout = new (std::nothrow) GCDescriptorSetLayout{};
    if (!layout) return ep_out_of_memory();
    
    layout->ep_device = device;
    
    if (desc->binding_count > 0) {
        layout->bindings.reserve(desc->binding_count);
        for (uint32_t i = 0; i < desc->binding_count; i++) {
            GCDescriptorBindingInfo info = {};
            info.binding = desc->bindings[i].binding;
            info.type = desc->bindings[i].type;
            info.count = desc->bindings[i].count;
            info.stages = desc->bindings[i].stages;
            layout->bindings.push_back(info);
        }
    }
    
    *out_layout = layout;
    return ep_ok();
}

extern "C" GCError GCDescriptorSetLayoutDestroy(GCDescriptorSetLayoutPtr layout) {
    delete layout;
    return ep_ok();
}

// ============================================================================
// Descriptor Set
// ============================================================================

extern "C" GCError GCDescriptorSetCreate(GCDevicePtr device,
                                         GCDescriptorSetLayoutPtr layout,
                                         GCDescriptorSetPtr *out_set) {
    if (!device || !layout || !out_set)
        return ep_invalid_argument("device, layout, or out_set is NULL");
    
    GCDescriptorSet *set = new (std::nothrow) GCDescriptorSet{};
    if (!set) return ep_out_of_memory();
    
    set->ep_device = device;
    set->layout = layout;
    
    if (!layout->bindings.empty()) {
        set->entries.resize(layout->bindings.size());
    }
    
    *out_set = set;
    return ep_ok();
}

extern "C" GCError GCDescriptorSetDestroy(GCDescriptorSetPtr set) {
    delete set;
    return ep_ok();
}

extern "C" GCError GCDescriptorSetUpdateBuffer(GCDescriptorSetPtr set, uint32_t binding,
                                               GCBufferPtr buffer, uint64_t offset,
                                               uint64_t range) {
    if (!set || !buffer) return ep_invalid_argument("set or buffer is NULL");
    
    for (size_t i = 0; i < set->layout->bindings.size(); i++) {
        if (set->layout->bindings[i].binding == binding) {
            GCDevice *device = set->ep_device;
            
            if (set->layout->bindings[i].type == GC_DESCRIPTOR_UNIFORM_BUFFER) {
                // D3D12 requires constant buffer offsets to be 256-byte aligned
                if ((offset % 256) != 0) {
                    return ep_invalid_argument("constant buffer offset must be 256-byte aligned");
                }
                if (offset + range > buffer->size) {
                    return ep_invalid_argument("constant buffer range exceeds buffer size");
                }
                
                // Allocate descriptor in heap
                UINT heap_idx = device->cbv_srv_uav_heap_offset.fetch_add(1);
                D3D12_CPU_DESCRIPTOR_HANDLE cpu_handle = 
                    device->cbv_srv_uav_heap->GetCPUDescriptorHandleForHeapStart();
                cpu_handle.ptr += heap_idx * device->cbv_srv_uav_descriptor_size;
                
                D3D12_CONSTANT_BUFFER_VIEW_DESC cbv_desc = {};
                cbv_desc.BufferLocation = buffer->gpu_address + offset;
                cbv_desc.SizeInBytes = static_cast<UINT>((range + 255) & ~255); // 256-byte aligned size
                device->device->CreateConstantBufferView(&cbv_desc, cpu_handle);
                
                set->entries[i].type = GC_DESCRIPTOR_UNIFORM_BUFFER;
                set->entries[i].value.buffer_info.buffer = buffer->resource.Get();
                set->entries[i].value.buffer_info.offset = offset;
                set->entries[i].value.buffer_info.range = range;
                set->entries[i].value.buffer_info.heap_index = heap_idx;
                
            } else if (set->layout->bindings[i].type == GC_DESCRIPTOR_STORAGE_BUFFER) {
                // Raw UAV buffers require offset and range to be multiples of 4
                if ((offset % 4) != 0) {
                    return ep_invalid_argument("storage buffer offset must be 4-byte aligned");
                }
                if ((range % 4) != 0) {
                    return ep_invalid_argument("storage buffer range must be a multiple of 4");
                }
                if (offset + range > buffer->size) {
                    return ep_invalid_argument("storage buffer range exceeds buffer size");
                }
                
                // Allocate descriptor in heap
                UINT heap_idx = device->cbv_srv_uav_heap_offset.fetch_add(1);
                D3D12_CPU_DESCRIPTOR_HANDLE cpu_handle = 
                    device->cbv_srv_uav_heap->GetCPUDescriptorHandleForHeapStart();
                cpu_handle.ptr += heap_idx * device->cbv_srv_uav_descriptor_size;
                
                D3D12_UNORDERED_ACCESS_VIEW_DESC uav_desc = {};
                uav_desc.Format = DXGI_FORMAT_R32_TYPELESS;
                uav_desc.ViewDimension = D3D12_UAV_DIMENSION_BUFFER;
                uav_desc.Buffer.FirstElement = offset / 4;
                uav_desc.Buffer.NumElements = static_cast<UINT>(range / 4);
                uav_desc.Buffer.Flags = D3D12_BUFFER_UAV_FLAG_RAW;
                device->device->CreateUnorderedAccessView(
                    buffer->resource.Get(), nullptr, &uav_desc, cpu_handle);
                
                set->entries[i].type = GC_DESCRIPTOR_STORAGE_BUFFER;
                set->entries[i].value.buffer_info.buffer = buffer->resource.Get();
                set->entries[i].value.buffer_info.offset = offset;
                set->entries[i].value.buffer_info.range = range;
                set->entries[i].value.buffer_info.heap_index = heap_idx;
            }
            
            return ep_ok();
        }
    }
    
    return ep_invalid_argument("binding not found");
}

extern "C" GCError GCDescriptorSetUpdateTexture(GCDescriptorSetPtr set, uint32_t binding,
                                                GCTexturePtr texture) {
    if (!set || !texture) return ep_invalid_argument("set or texture is NULL");
    
    for (size_t i = 0; i < set->layout->bindings.size(); i++) {
        if (set->layout->bindings[i].binding == binding) {
            if (set->layout->bindings[i].type == GC_DESCRIPTOR_SAMPLED_TEXTURE) {
                set->entries[i].type = GC_DESCRIPTOR_SAMPLED_TEXTURE;
                // Store the heap index for shader binding, not the CPU handle
                set->entries[i].value.texture_info.heap_index = texture->srv_heap_index;
            } else if (set->layout->bindings[i].type == GC_DESCRIPTOR_STORAGE_TEXTURE) {
                set->entries[i].type = GC_DESCRIPTOR_STORAGE_TEXTURE;
                set->entries[i].value.texture_info.heap_index = texture->uav_heap_index;
            }
            return ep_ok();
        }
    }
    
    return ep_invalid_argument("binding not found");
}

extern "C" GCError GCDescriptorSetUpdateSampler(GCDescriptorSetPtr set, uint32_t binding,
                                                GCSamplerPtr sampler) {
    if (!set || !sampler) return ep_invalid_argument("set or sampler is NULL");
    
    for (size_t i = 0; i < set->layout->bindings.size(); i++) {
        if (set->layout->bindings[i].binding == binding) {
            set->entries[i].type = GC_DESCRIPTOR_SAMPLER;
            // Store the sampler's heap index for shader binding
            set->entries[i].value.sampler_info.heap_index = sampler->heap_index;
            return ep_ok();
        }
    }
    
    return ep_invalid_argument("binding not found");
}

extern "C" GCError GCDescriptorSetUpdateAccelerationStructure(GCDescriptorSetPtr set,
                                                              uint32_t binding,
                                                              GCAccelerationStructurePtr as) {
    if (!set || !as) return ep_invalid_argument("set or acceleration structure is NULL");
    
    for (size_t i = 0; i < set->layout->bindings.size(); i++) {
        if (set->layout->bindings[i].binding == binding) {
            set->entries[i].type = GC_DESCRIPTOR_ACCELERATION_STRUCTURE;
            set->entries[i].value.accel = as->gpu_address;
            return ep_ok();
        }
    }
    
    return ep_invalid_argument("binding not found");
}

// ============================================================================
// Pipeline Layout
// ============================================================================

static D3D12_DESCRIPTOR_RANGE_TYPE ep_to_d3d12_range_type(GCDescriptorType type) {
    switch (type) {
        case GC_DESCRIPTOR_UNIFORM_BUFFER:
            return D3D12_DESCRIPTOR_RANGE_TYPE_CBV;
        case GC_DESCRIPTOR_STORAGE_BUFFER:
        case GC_DESCRIPTOR_STORAGE_TEXTURE:
            return D3D12_DESCRIPTOR_RANGE_TYPE_UAV;
        case GC_DESCRIPTOR_SAMPLED_TEXTURE:
        case GC_DESCRIPTOR_ACCELERATION_STRUCTURE:
            return D3D12_DESCRIPTOR_RANGE_TYPE_SRV;
        case GC_DESCRIPTOR_SAMPLER:
            return D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER;
        default:
            return D3D12_DESCRIPTOR_RANGE_TYPE_SRV;
    }
}

static bool is_sampler_type(GCDescriptorType type) {
    return type == GC_DESCRIPTOR_SAMPLER;
}

extern "C" GCError GCPipelineLayoutCreate(GCDevicePtr device,
                                          const GCPipelineLayoutDesc *desc,
                                          GCPipelineLayoutPtr *out_layout) {
    if (!device || !desc || !out_layout)
        return ep_invalid_argument("device, desc, or out_layout is NULL");
    
    GCPipelineLayout *layout = new (std::nothrow) GCPipelineLayout{};
    if (!layout) return ep_out_of_memory();
    
    layout->ep_device = device;
    layout->push_constant_size = desc->push_constant_size;
    layout->push_constant_stages = desc->push_constant_stages;
    layout->push_constant_root_index = 0;
    layout->first_set_root_index = 0;
    
    // Build root signature
    std::vector<D3D12_ROOT_PARAMETER1> root_params;
    std::vector<std::vector<D3D12_DESCRIPTOR_RANGE1>> all_cbv_srv_uav_ranges;
    std::vector<std::vector<D3D12_DESCRIPTOR_RANGE1>> all_sampler_ranges;
    
    UINT current_root_index = 0;
    
    // Root constants for push constants
    if (desc->push_constant_size > 0) {
        layout->push_constant_root_index = current_root_index;
        
        D3D12_ROOT_PARAMETER1 param = {};
        param.ParameterType = D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS;
        param.Constants.ShaderRegister = 0;
        param.Constants.RegisterSpace = 0;
        param.Constants.Num32BitValues = (desc->push_constant_size + 3) / 4;
        param.ShaderVisibility = ep_to_d3d12_visibility(desc->push_constant_stages);
        root_params.push_back(param);
        current_root_index++;
    }
    
    layout->first_set_root_index = current_root_index;
    
    // Descriptor tables for each set - separate CBV/SRV/UAV from samplers
    for (uint32_t set_idx = 0; set_idx < desc->set_layout_count; set_idx++) {
        GCDescriptorSetLayout *set_layout = desc->set_layouts[set_idx];
        if (!set_layout) continue;
        
        layout->set_layouts.push_back(set_layout);
        
        // Separate CBV/SRV/UAV bindings from sampler bindings
        std::vector<D3D12_DESCRIPTOR_RANGE1> cbv_srv_uav_ranges;
        std::vector<D3D12_DESCRIPTOR_RANGE1> sampler_ranges;
        
        for (const auto &binding : set_layout->bindings) {
            D3D12_DESCRIPTOR_RANGE1 range = {};
            range.RangeType = ep_to_d3d12_range_type(binding.type);
            range.NumDescriptors = binding.count;
            range.BaseShaderRegister = binding.binding;
            range.RegisterSpace = set_idx;
            range.Flags = D3D12_DESCRIPTOR_RANGE_FLAG_NONE;
            range.OffsetInDescriptorsFromTableStart = D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND;
            
            if (is_sampler_type(binding.type)) {
                sampler_ranges.push_back(range);
            } else {
                cbv_srv_uav_ranges.push_back(range);
            }
        }
        
        // Add CBV/SRV/UAV descriptor table if there are any
        if (!cbv_srv_uav_ranges.empty()) {
            all_cbv_srv_uav_ranges.push_back(std::move(cbv_srv_uav_ranges));
            
            D3D12_ROOT_PARAMETER1 param = {};
            param.ParameterType = D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE;
            param.DescriptorTable.NumDescriptorRanges = 
                static_cast<UINT>(all_cbv_srv_uav_ranges.back().size());
            param.DescriptorTable.pDescriptorRanges = all_cbv_srv_uav_ranges.back().data();
            param.ShaderVisibility = D3D12_SHADER_VISIBILITY_ALL;
            root_params.push_back(param);
            current_root_index++;
        }
        
        // Add sampler descriptor table if there are any (samplers use separate heap)
        if (!sampler_ranges.empty()) {
            all_sampler_ranges.push_back(std::move(sampler_ranges));
            
            D3D12_ROOT_PARAMETER1 param = {};
            param.ParameterType = D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE;
            param.DescriptorTable.NumDescriptorRanges = 
                static_cast<UINT>(all_sampler_ranges.back().size());
            param.DescriptorTable.pDescriptorRanges = all_sampler_ranges.back().data();
            param.ShaderVisibility = D3D12_SHADER_VISIBILITY_ALL;
            root_params.push_back(param);
            current_root_index++;
        }
    }
    
    // Create root signature
    D3D12_VERSIONED_ROOT_SIGNATURE_DESC root_sig_desc = {};
    root_sig_desc.Version = D3D_ROOT_SIGNATURE_VERSION_1_1;
    root_sig_desc.Desc_1_1.NumParameters = static_cast<UINT>(root_params.size());
    root_sig_desc.Desc_1_1.pParameters = root_params.data();
    root_sig_desc.Desc_1_1.NumStaticSamplers = 0;
    root_sig_desc.Desc_1_1.pStaticSamplers = nullptr;
    root_sig_desc.Desc_1_1.Flags = 
        D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT |
        D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED |
        D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED;
    
    ComPtr<ID3DBlob> signature_blob;
    ComPtr<ID3DBlob> error_blob;
    HRESULT hr = D3D12SerializeVersionedRootSignature(&root_sig_desc,
                                                       &signature_blob,
                                                       &error_blob);
    if (FAILED(hr)) {
        delete layout;
        return ep_from_hresult(hr, "failed to serialize root signature");
    }
    
    hr = device->device->CreateRootSignature(
        0,
        signature_blob->GetBufferPointer(),
        signature_blob->GetBufferSize(),
        IID_PPV_ARGS(&layout->root_signature));
    
    if (FAILED(hr)) {
        delete layout;
        return ep_from_hresult(hr, "failed to create root signature");
    }
    
    *out_layout = layout;
    return ep_ok();
}

extern "C" GCError GCPipelineLayoutDestroy(GCPipelineLayoutPtr layout) {
    delete layout;
    return ep_ok();
}
