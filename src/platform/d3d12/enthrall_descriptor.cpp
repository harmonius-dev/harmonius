// enthrall_descriptor.cpp - Descriptor set layout, descriptor set, and pipeline layout

#include "enthrall_internal.h"

// ============================================================================
// Descriptor Set Layout
// ============================================================================

extern "C" EPError EPDescriptorSetLayoutCreate(EPDevicePtr device,
                                               const EPDescriptorSetLayoutDesc *desc,
                                               EPDescriptorSetLayoutPtr *out_layout) {
    if (!device || !desc || !out_layout)
        return ep_invalid_argument("device, desc, or out_layout is NULL");
    
    EPDescriptorSetLayout *layout = new (std::nothrow) EPDescriptorSetLayout{};
    if (!layout) return ep_out_of_memory();
    
    layout->ep_device = device;
    
    if (desc->binding_count > 0) {
        layout->bindings.reserve(desc->binding_count);
        for (uint32_t i = 0; i < desc->binding_count; i++) {
            EPDescriptorBindingInfo info = {};
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

extern "C" EPError EPDescriptorSetLayoutDestroy(EPDescriptorSetLayoutPtr layout) {
    delete layout;
    return ep_ok();
}

// ============================================================================
// Descriptor Set
// ============================================================================

extern "C" EPError EPDescriptorSetCreate(EPDevicePtr device,
                                         EPDescriptorSetLayoutPtr layout,
                                         EPDescriptorSetPtr *out_set) {
    if (!device || !layout || !out_set)
        return ep_invalid_argument("device, layout, or out_set is NULL");
    
    EPDescriptorSet *set = new (std::nothrow) EPDescriptorSet{};
    if (!set) return ep_out_of_memory();
    
    set->ep_device = device;
    set->layout = layout;
    
    if (!layout->bindings.empty()) {
        set->entries.resize(layout->bindings.size());
    }
    
    *out_set = set;
    return ep_ok();
}

extern "C" EPError EPDescriptorSetDestroy(EPDescriptorSetPtr set) {
    delete set;
    return ep_ok();
}

extern "C" EPError EPDescriptorSetUpdateBuffer(EPDescriptorSetPtr set, uint32_t binding,
                                               EPBufferPtr buffer, uint64_t offset,
                                               uint64_t range) {
    if (!set || !buffer) return ep_invalid_argument("set or buffer is NULL");
    
    for (size_t i = 0; i < set->layout->bindings.size(); i++) {
        if (set->layout->bindings[i].binding == binding) {
            set->entries[i].type = set->layout->bindings[i].type;
            set->entries[i].value.buffer_info.buffer = buffer->resource.Get();
            set->entries[i].value.buffer_info.offset = offset;
            set->entries[i].value.buffer_info.range = range;
            
            // Create CBV/SRV/UAV as appropriate
            EPDevice *device = set->ep_device;
            
            D3D12_CPU_DESCRIPTOR_HANDLE cpu_handle = 
                device->cbv_srv_uav_heap->GetCPUDescriptorHandleForHeapStart();
            UINT offset_idx = device->cbv_srv_uav_heap_offset.fetch_add(1);
            cpu_handle.ptr += offset_idx * device->cbv_srv_uav_descriptor_size;
            
            if (set->layout->bindings[i].type == EP_DESCRIPTOR_UNIFORM_BUFFER) {
                D3D12_CONSTANT_BUFFER_VIEW_DESC cbv_desc = {};
                cbv_desc.BufferLocation = buffer->gpu_address + offset;
                cbv_desc.SizeInBytes = static_cast<UINT>((range + 255) & ~255); // 256-byte aligned
                device->device->CreateConstantBufferView(&cbv_desc, cpu_handle);
            } else if (set->layout->bindings[i].type == EP_DESCRIPTOR_STORAGE_BUFFER) {
                D3D12_UNORDERED_ACCESS_VIEW_DESC uav_desc = {};
                uav_desc.Format = DXGI_FORMAT_R32_TYPELESS;
                uav_desc.ViewDimension = D3D12_UAV_DIMENSION_BUFFER;
                uav_desc.Buffer.FirstElement = offset / 4;
                uav_desc.Buffer.NumElements = static_cast<UINT>(range / 4);
                uav_desc.Buffer.Flags = D3D12_BUFFER_UAV_FLAG_RAW;
                device->device->CreateUnorderedAccessView(
                    buffer->resource.Get(), nullptr, &uav_desc, cpu_handle);
            }
            
            return ep_ok();
        }
    }
    
    return ep_invalid_argument("binding not found");
}

extern "C" EPError EPDescriptorSetUpdateTexture(EPDescriptorSetPtr set, uint32_t binding,
                                                EPTexturePtr texture) {
    if (!set || !texture) return ep_invalid_argument("set or texture is NULL");
    
    for (size_t i = 0; i < set->layout->bindings.size(); i++) {
        if (set->layout->bindings[i].binding == binding) {
            if (set->layout->bindings[i].type == EP_DESCRIPTOR_SAMPLED_TEXTURE) {
                set->entries[i].type = EP_DESCRIPTOR_SAMPLED_TEXTURE;
                set->entries[i].value.texture = texture->srv;
            } else if (set->layout->bindings[i].type == EP_DESCRIPTOR_STORAGE_TEXTURE) {
                set->entries[i].type = EP_DESCRIPTOR_STORAGE_TEXTURE;
                set->entries[i].value.texture = texture->uav;
            }
            return ep_ok();
        }
    }
    
    return ep_invalid_argument("binding not found");
}

extern "C" EPError EPDescriptorSetUpdateSampler(EPDescriptorSetPtr set, uint32_t binding,
                                                EPSamplerPtr sampler) {
    if (!set || !sampler) return ep_invalid_argument("set or sampler is NULL");
    
    for (size_t i = 0; i < set->layout->bindings.size(); i++) {
        if (set->layout->bindings[i].binding == binding) {
            set->entries[i].type = EP_DESCRIPTOR_SAMPLER;
            set->entries[i].value.sampler = sampler->sampler;
            return ep_ok();
        }
    }
    
    return ep_invalid_argument("binding not found");
}

extern "C" EPError EPDescriptorSetUpdateAccelerationStructure(EPDescriptorSetPtr set,
                                                              uint32_t binding,
                                                              EPAccelerationStructurePtr as) {
    if (!set || !as) return ep_invalid_argument("set or acceleration structure is NULL");
    
    for (size_t i = 0; i < set->layout->bindings.size(); i++) {
        if (set->layout->bindings[i].binding == binding) {
            set->entries[i].type = EP_DESCRIPTOR_ACCELERATION_STRUCTURE;
            set->entries[i].value.accel = as->gpu_address;
            return ep_ok();
        }
    }
    
    return ep_invalid_argument("binding not found");
}

// ============================================================================
// Pipeline Layout
// ============================================================================

static D3D12_DESCRIPTOR_RANGE_TYPE ep_to_d3d12_range_type(EPDescriptorType type) {
    switch (type) {
        case EP_DESCRIPTOR_UNIFORM_BUFFER:
            return D3D12_DESCRIPTOR_RANGE_TYPE_CBV;
        case EP_DESCRIPTOR_STORAGE_BUFFER:
        case EP_DESCRIPTOR_STORAGE_TEXTURE:
            return D3D12_DESCRIPTOR_RANGE_TYPE_UAV;
        case EP_DESCRIPTOR_SAMPLED_TEXTURE:
        case EP_DESCRIPTOR_ACCELERATION_STRUCTURE:
            return D3D12_DESCRIPTOR_RANGE_TYPE_SRV;
        case EP_DESCRIPTOR_SAMPLER:
            return D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER;
        default:
            return D3D12_DESCRIPTOR_RANGE_TYPE_SRV;
    }
}

extern "C" EPError EPPipelineLayoutCreate(EPDevicePtr device,
                                          const EPPipelineLayoutDesc *desc,
                                          EPPipelineLayoutPtr *out_layout) {
    if (!device || !desc || !out_layout)
        return ep_invalid_argument("device, desc, or out_layout is NULL");
    
    EPPipelineLayout *layout = new (std::nothrow) EPPipelineLayout{};
    if (!layout) return ep_out_of_memory();
    
    layout->ep_device = device;
    layout->push_constant_size = desc->push_constant_size;
    layout->push_constant_stages = desc->push_constant_stages;
    
    // Build root signature
    std::vector<D3D12_ROOT_PARAMETER1> root_params;
    std::vector<std::vector<D3D12_DESCRIPTOR_RANGE1>> all_ranges;
    
    // Root constants for push constants
    if (desc->push_constant_size > 0) {
        D3D12_ROOT_PARAMETER1 param = {};
        param.ParameterType = D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS;
        param.Constants.ShaderRegister = 0;
        param.Constants.RegisterSpace = 0;
        param.Constants.Num32BitValues = (desc->push_constant_size + 3) / 4;
        param.ShaderVisibility = ep_to_d3d12_visibility(desc->push_constant_stages);
        root_params.push_back(param);
    }
    
    // Descriptor tables for each set
    for (uint32_t set_idx = 0; set_idx < desc->set_layout_count; set_idx++) {
        EPDescriptorSetLayout *set_layout = desc->set_layouts[set_idx];
        if (!set_layout) continue;
        
        layout->set_layouts.push_back(set_layout);
        
        // Create descriptor ranges for this set
        std::vector<D3D12_DESCRIPTOR_RANGE1> ranges;
        for (const auto &binding : set_layout->bindings) {
            D3D12_DESCRIPTOR_RANGE1 range = {};
            range.RangeType = ep_to_d3d12_range_type(binding.type);
            range.NumDescriptors = binding.count;
            range.BaseShaderRegister = binding.binding;
            range.RegisterSpace = set_idx;
            range.Flags = D3D12_DESCRIPTOR_RANGE_FLAG_NONE;
            range.OffsetInDescriptorsFromTableStart = D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND;
            ranges.push_back(range);
        }
        
        if (!ranges.empty()) {
            all_ranges.push_back(std::move(ranges));
            
            D3D12_ROOT_PARAMETER1 param = {};
            param.ParameterType = D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE;
            param.DescriptorTable.NumDescriptorRanges = 
                static_cast<UINT>(all_ranges.back().size());
            param.DescriptorTable.pDescriptorRanges = all_ranges.back().data();
            param.ShaderVisibility = D3D12_SHADER_VISIBILITY_ALL;
            root_params.push_back(param);
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

extern "C" EPError EPPipelineLayoutDestroy(EPPipelineLayoutPtr layout) {
    delete layout;
    return ep_ok();
}
