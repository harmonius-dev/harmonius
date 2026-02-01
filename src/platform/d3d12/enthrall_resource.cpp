// enthrall_resource.cpp - Buffer, texture, sampler, and shader library implementation

#include "enthrall_internal.h"

// ============================================================================
// Buffer
// ============================================================================

extern "C" EPError EPBufferCreate(EPDevicePtr device, const EPBufferDesc *desc,
                                  EPBufferPtr *out_buffer) {
    if (!device || !desc || !out_buffer)
        return ep_invalid_argument("device, desc, or out_buffer is NULL");
    
    EPBuffer *buffer = new (std::nothrow) EPBuffer{};
    if (!buffer) return ep_out_of_memory();
    
    buffer->ep_device = device;
    buffer->size = desc->size;
    buffer->usage = desc->usage;
    buffer->host_visible = desc->host_visible;
    buffer->mapped_ptr = nullptr;
    
    D3D12_RESOURCE_DESC resource_desc = {};
    resource_desc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
    resource_desc.Alignment = 0;
    resource_desc.Width = desc->size;
    resource_desc.Height = 1;
    resource_desc.DepthOrArraySize = 1;
    resource_desc.MipLevels = 1;
    resource_desc.Format = DXGI_FORMAT_UNKNOWN;
    resource_desc.SampleDesc.Count = 1;
    resource_desc.SampleDesc.Quality = 0;
    resource_desc.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;
    resource_desc.Flags = D3D12_RESOURCE_FLAG_NONE;
    
    // Set UAV flag if storage buffer
    if (desc->usage & EP_BUFFER_USAGE_STORAGE_BIT) {
        resource_desc.Flags |= D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS;
    }
    
    D3D12_HEAP_PROPERTIES heap_props = {};
    D3D12_RESOURCE_STATES initial_state;
    
    if (desc->host_visible) {
        heap_props.Type = D3D12_HEAP_TYPE_UPLOAD;
        initial_state = D3D12_RESOURCE_STATE_GENERIC_READ;
    } else {
        heap_props.Type = D3D12_HEAP_TYPE_DEFAULT;
        initial_state = D3D12_RESOURCE_STATE_COMMON;
    }
    
    HRESULT hr = device->device->CreateCommittedResource(
        &heap_props,
        D3D12_HEAP_FLAG_NONE,
        &resource_desc,
        initial_state,
        nullptr,
        IID_PPV_ARGS(&buffer->resource));
    
    if (FAILED(hr)) {
        delete buffer;
        return ep_from_hresult(hr, "failed to create buffer");
    }
    
    buffer->gpu_address = buffer->resource->GetGPUVirtualAddress();
    
    // Map upload buffers persistently
    if (desc->host_visible) {
        D3D12_RANGE read_range = {0, 0}; // No CPU read
        hr = buffer->resource->Map(0, &read_range, &buffer->mapped_ptr);
        if (FAILED(hr)) {
            delete buffer;
            return ep_from_hresult(hr, "failed to map buffer");
        }
    }
    
    *out_buffer = buffer;
    return ep_ok();
}

extern "C" EPError EPBufferDestroy(EPBufferPtr buffer) {
    if (buffer) {
        if (buffer->mapped_ptr && buffer->resource) {
            buffer->resource->Unmap(0, nullptr);
        }
    }
    delete buffer;
    return ep_ok();
}

// ============================================================================
// Texture
// ============================================================================

extern "C" EPError EPTextureCreate(EPDevicePtr device, const EPTextureDesc *desc,
                                   EPTexturePtr *out_texture) {
    if (!device || !desc || !out_texture)
        return ep_invalid_argument("device, desc, or out_texture is NULL");
    
    EPTexture *texture = new (std::nothrow) EPTexture{};
    if (!texture) return ep_out_of_memory();
    
    texture->ep_device = device;
    texture->desc = *desc;
    
    D3D12_RESOURCE_DESC resource_desc = {};
    resource_desc.Dimension = ep_to_d3d12_dimension(desc->dimension);
    resource_desc.Alignment = 0;
    resource_desc.Width = desc->width;
    resource_desc.Height = desc->height;
    resource_desc.DepthOrArraySize = (desc->dimension == EP_TEXTURE_DIM_3D)
        ? static_cast<UINT16>(desc->depth)
        : static_cast<UINT16>(desc->array_layers > 0 ? desc->array_layers : 1);
    resource_desc.MipLevels = static_cast<UINT16>(desc->mip_levels > 0 ? desc->mip_levels : 1);
    resource_desc.Format = ep_to_dxgi_format(desc->format);
    resource_desc.SampleDesc.Count = 1;
    resource_desc.SampleDesc.Quality = 0;
    resource_desc.Layout = D3D12_TEXTURE_LAYOUT_UNKNOWN;
    resource_desc.Flags = D3D12_RESOURCE_FLAG_NONE;
    
    // Set appropriate flags based on usage
    if (desc->usage & EP_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT) {
        resource_desc.Flags |= D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET;
    }
    if (desc->usage & EP_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT) {
        resource_desc.Flags |= D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL;
    }
    if (desc->usage & EP_TEXTURE_USAGE_STORAGE_BIT) {
        resource_desc.Flags |= D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS;
    }
    
    D3D12_HEAP_PROPERTIES heap_props = {};
    heap_props.Type = D3D12_HEAP_TYPE_DEFAULT;
    
    D3D12_RESOURCE_STATES initial_state = D3D12_RESOURCE_STATE_COMMON;
    
    D3D12_CLEAR_VALUE *clear_value_ptr = nullptr;
    D3D12_CLEAR_VALUE clear_value = {};
    
    if (desc->usage & EP_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT) {
        clear_value.Format = resource_desc.Format;
        clear_value.Color[0] = 0.0f;
        clear_value.Color[1] = 0.0f;
        clear_value.Color[2] = 0.0f;
        clear_value.Color[3] = 1.0f;
        clear_value_ptr = &clear_value;
    } else if (desc->usage & EP_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT) {
        clear_value.Format = resource_desc.Format;
        clear_value.DepthStencil.Depth = 1.0f;
        clear_value.DepthStencil.Stencil = 0;
        clear_value_ptr = &clear_value;
    }
    
    HRESULT hr = device->device->CreateCommittedResource(
        &heap_props,
        D3D12_HEAP_FLAG_NONE,
        &resource_desc,
        initial_state,
        clear_value_ptr,
        IID_PPV_ARGS(&texture->resource));
    
    if (FAILED(hr)) {
        delete texture;
        return ep_from_hresult(hr, "failed to create texture");
    }
    
    texture->current_state = initial_state;
    
    // Create views
    if (desc->usage & EP_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT) {
        UINT rtv_offset = device->rtv_heap_offset.fetch_add(1);
        texture->rtv = device->rtv_heap->GetCPUDescriptorHandleForHeapStart();
        texture->rtv.ptr += rtv_offset * device->rtv_descriptor_size;
        
        D3D12_RENDER_TARGET_VIEW_DESC rtv_desc = {};
        rtv_desc.Format = resource_desc.Format;
        rtv_desc.ViewDimension = D3D12_RTV_DIMENSION_TEXTURE2D;
        rtv_desc.Texture2D.MipSlice = 0;
        
        device->device->CreateRenderTargetView(texture->resource.Get(), &rtv_desc, texture->rtv);
    }
    
    if (desc->usage & EP_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT) {
        UINT dsv_offset = device->dsv_heap_offset.fetch_add(1);
        texture->dsv = device->dsv_heap->GetCPUDescriptorHandleForHeapStart();
        texture->dsv.ptr += dsv_offset * device->dsv_descriptor_size;
        
        D3D12_DEPTH_STENCIL_VIEW_DESC dsv_desc = {};
        dsv_desc.Format = resource_desc.Format;
        dsv_desc.ViewDimension = D3D12_DSV_DIMENSION_TEXTURE2D;
        dsv_desc.Texture2D.MipSlice = 0;
        
        device->device->CreateDepthStencilView(texture->resource.Get(), &dsv_desc, texture->dsv);
    }
    
    if (desc->usage & EP_TEXTURE_USAGE_SAMPLED_BIT) {
        UINT srv_offset = device->cbv_srv_uav_heap_offset.fetch_add(1);
        texture->srv = device->cbv_srv_uav_heap->GetCPUDescriptorHandleForHeapStart();
        texture->srv.ptr += srv_offset * device->cbv_srv_uav_descriptor_size;
        
        D3D12_SHADER_RESOURCE_VIEW_DESC srv_desc = {};
        srv_desc.Format = resource_desc.Format;
        srv_desc.Shader4ComponentMapping = D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING;
        
        switch (desc->dimension) {
            case EP_TEXTURE_DIM_1D:
                srv_desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE1D;
                srv_desc.Texture1D.MipLevels = resource_desc.MipLevels;
                break;
            case EP_TEXTURE_DIM_2D:
                srv_desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2D;
                srv_desc.Texture2D.MipLevels = resource_desc.MipLevels;
                break;
            case EP_TEXTURE_DIM_3D:
                srv_desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE3D;
                srv_desc.Texture3D.MipLevels = resource_desc.MipLevels;
                break;
            case EP_TEXTURE_DIM_CUBE:
                srv_desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURECUBE;
                srv_desc.TextureCube.MipLevels = resource_desc.MipLevels;
                break;
        }
        
        device->device->CreateShaderResourceView(texture->resource.Get(), &srv_desc, texture->srv);
    }
    
    if (desc->usage & EP_TEXTURE_USAGE_STORAGE_BIT) {
        UINT uav_offset = device->cbv_srv_uav_heap_offset.fetch_add(1);
        texture->uav = device->cbv_srv_uav_heap->GetCPUDescriptorHandleForHeapStart();
        texture->uav.ptr += uav_offset * device->cbv_srv_uav_descriptor_size;
        
        D3D12_UNORDERED_ACCESS_VIEW_DESC uav_desc = {};
        uav_desc.Format = resource_desc.Format;
        uav_desc.ViewDimension = D3D12_UAV_DIMENSION_TEXTURE2D;
        uav_desc.Texture2D.MipSlice = 0;
        
        device->device->CreateUnorderedAccessView(texture->resource.Get(), nullptr,
                                                   &uav_desc, texture->uav);
    }
    
    *out_texture = texture;
    return ep_ok();
}

extern "C" EPError EPTextureDestroy(EPTexturePtr texture) {
    delete texture;
    return ep_ok();
}

// ============================================================================
// Sampler
// ============================================================================

extern "C" EPError EPSamplerCreate(EPDevicePtr device, const EPSamplerDesc *desc,
                                   EPSamplerPtr *out_sampler) {
    if (!device || !desc || !out_sampler)
        return ep_invalid_argument("device, desc, or out_sampler is NULL");
    
    EPSampler *sampler = new (std::nothrow) EPSampler{};
    if (!sampler) return ep_out_of_memory();
    
    sampler->ep_device = device;
    
    D3D12_SAMPLER_DESC sampler_desc = {};
    sampler_desc.Filter = ep_to_d3d12_filter(desc->min_filter, desc->mag_filter);
    sampler_desc.AddressU = ep_to_d3d12_address(desc->address_u);
    sampler_desc.AddressV = ep_to_d3d12_address(desc->address_v);
    sampler_desc.AddressW = ep_to_d3d12_address(desc->address_w);
    sampler_desc.MipLODBias = 0.0f;
    sampler_desc.MaxAnisotropy = static_cast<UINT>(desc->max_anisotropy);
    sampler_desc.ComparisonFunc = ep_to_d3d12_compare(desc->compare_op);
    sampler_desc.BorderColor[0] = 0.0f;
    sampler_desc.BorderColor[1] = 0.0f;
    sampler_desc.BorderColor[2] = 0.0f;
    sampler_desc.BorderColor[3] = 1.0f;
    sampler_desc.MinLOD = 0.0f;
    sampler_desc.MaxLOD = D3D12_FLOAT32_MAX;
    
    // Use anisotropic filter if max_anisotropy > 1
    if (desc->max_anisotropy > 1.0f) {
        sampler_desc.Filter = D3D12_FILTER_ANISOTROPIC;
    }
    
    sampler->desc = sampler_desc;
    
    UINT sampler_offset = device->sampler_heap_offset.fetch_add(1);
    sampler->sampler = device->sampler_heap->GetCPUDescriptorHandleForHeapStart();
    sampler->sampler.ptr += sampler_offset * device->sampler_descriptor_size;
    
    device->device->CreateSampler(&sampler_desc, sampler->sampler);
    
    *out_sampler = sampler;
    return ep_ok();
}

extern "C" EPError EPSamplerDestroy(EPSamplerPtr sampler) {
    delete sampler;
    return ep_ok();
}

// ============================================================================
// Shader Library
// ============================================================================

extern "C" EPError EPShaderLibraryCreate(EPDevicePtr device, const EPShaderLibraryDesc *desc,
                                         EPShaderLibraryPtr *out_library) {
    if (!device || !desc || !out_library)
        return ep_invalid_argument("device, desc, or out_library is NULL");
    if (desc->format != EP_SHADER_HLSL)
        return ep_unsupported("only HLSL shaders supported on D3D12");
    
    EPShaderLibrary *library = new (std::nothrow) EPShaderLibrary{};
    if (!library) return ep_out_of_memory();
    
    library->ep_device = device;
    
    // For pre-compiled DXIL bytecode, just store it
    if (desc->size > 4 && memcmp(desc->data, "DXBC", 4) == 0) {
        library->bytecode.assign(desc->data, desc->data + desc->size);
    } else {
        // Assume it's HLSL source - compile with DXC
        ComPtr<IDxcCompiler3> compiler;
        ComPtr<IDxcUtils> utils;
        
        HRESULT hr = DxcCreateInstance(CLSID_DxcCompiler, IID_PPV_ARGS(&compiler));
        if (FAILED(hr)) {
            delete library;
            return ep_from_hresult(hr, "failed to create DXC compiler");
        }
        
        hr = DxcCreateInstance(CLSID_DxcUtils, IID_PPV_ARGS(&utils));
        if (FAILED(hr)) {
            delete library;
            return ep_from_hresult(hr, "failed to create DXC utils");
        }
        
        // Create source blob
        ComPtr<IDxcBlobEncoding> source_blob;
        hr = utils->CreateBlob(desc->data, static_cast<UINT32>(desc->size),
                               DXC_CP_UTF8, &source_blob);
        if (FAILED(hr)) {
            delete library;
            return ep_from_hresult(hr, "failed to create source blob");
        }
        
        // Compile arguments
        std::vector<LPCWSTR> args;
        args.push_back(L"-T");
        args.push_back(L"lib_6_6"); // Library target for SM 6.6
        args.push_back(L"-E");
        args.push_back(L"main");
        
        if (device->debug_names_enabled) {
            args.push_back(L"-Zi"); // Debug info
            args.push_back(L"-Qembed_debug");
        }
        
        DxcBuffer source_buffer = {};
        source_buffer.Ptr = source_blob->GetBufferPointer();
        source_buffer.Size = source_blob->GetBufferSize();
        source_buffer.Encoding = DXC_CP_UTF8;
        
        ComPtr<IDxcResult> result;
        hr = compiler->Compile(&source_buffer, args.data(), static_cast<UINT32>(args.size()),
                               nullptr, IID_PPV_ARGS(&result));
        
        if (SUCCEEDED(hr)) {
            result->GetStatus(&hr);
        }
        
        if (FAILED(hr)) {
            ComPtr<IDxcBlobUtf8> errors;
            if (SUCCEEDED(result->GetOutput(DXC_OUT_ERRORS, IID_PPV_ARGS(&errors), nullptr))) {
                if (errors && errors->GetStringLength() > 0) {
                    // Could log errors here
                }
            }
            delete library;
            return ep_error(EP_E_INVALID_ARGUMENT, "shader compilation failed");
        }
        
        ComPtr<IDxcBlob> compiled_blob;
        hr = result->GetOutput(DXC_OUT_OBJECT, IID_PPV_ARGS(&compiled_blob), nullptr);
        if (FAILED(hr) || !compiled_blob) {
            delete library;
            return ep_from_hresult(hr, "failed to get compiled shader");
        }
        
        library->blob = compiled_blob;
        library->bytecode.assign(
            static_cast<const uint8_t *>(compiled_blob->GetBufferPointer()),
            static_cast<const uint8_t *>(compiled_blob->GetBufferPointer()) + 
                compiled_blob->GetBufferSize());
    }
    
    *out_library = library;
    return ep_ok();
}

extern "C" EPError EPShaderLibraryDestroy(EPShaderLibraryPtr library) {
    delete library;
    return ep_ok();
}
