// gcraft_pipeline.cpp - Pipeline and acceleration structure implementation

#include "gcraft_internal.h"

// ============================================================================
// Helper: Create default root signature for pipelines without explicit layout
// ============================================================================

static HRESULT create_default_root_signature(ID3D12Device8 *device,
                                              ComPtr<ID3D12RootSignature> &out_sig) {
    D3D12_ROOT_PARAMETER1 params[2] = {};
    
    // Root constants at slot 0
    params[0].ParameterType = D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS;
    params[0].Constants.ShaderRegister = 0;
    params[0].Constants.RegisterSpace = 0;
    params[0].Constants.Num32BitValues = 64; // 256 bytes
    params[0].ShaderVisibility = D3D12_SHADER_VISIBILITY_ALL;
    
    D3D12_VERSIONED_ROOT_SIGNATURE_DESC root_sig_desc = {};
    root_sig_desc.Version = D3D_ROOT_SIGNATURE_VERSION_1_1;
    root_sig_desc.Desc_1_1.NumParameters = 1;
    root_sig_desc.Desc_1_1.pParameters = params;
    root_sig_desc.Desc_1_1.Flags = 
        D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT |
        D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED |
        D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED;
    
    ComPtr<ID3DBlob> signature_blob;
    ComPtr<ID3DBlob> error_blob;
    HRESULT hr = D3D12SerializeVersionedRootSignature(&root_sig_desc,
                                                       &signature_blob,
                                                       &error_blob);
    if (FAILED(hr)) return hr;
    
    return device->CreateRootSignature(
        0,
        signature_blob->GetBufferPointer(),
        signature_blob->GetBufferSize(),
        IID_PPV_ARGS(&out_sig));
}

// ============================================================================
// Render Pipeline
// ============================================================================

static DXGI_FORMAT ep_to_vertex_format(GCTextureFormat format) {
    switch (format) {
        case GC_FORMAT_RGBA8_UNORM:   return DXGI_FORMAT_R8G8B8A8_UNORM;
        case GC_FORMAT_BGRA8_UNORM:   return DXGI_FORMAT_B8G8R8A8_UNORM;
        case GC_FORMAT_RGBA16_FLOAT:  return DXGI_FORMAT_R16G16B16A16_FLOAT;
        case GC_FORMAT_RGBA32_FLOAT:  return DXGI_FORMAT_R32G32B32A32_FLOAT;
        default:                       return DXGI_FORMAT_R32G32B32A32_FLOAT;
    }
}

extern "C" GCError GCRenderPipelineCreate(GCDevicePtr device,
                                          const GCRenderPipelineDesc *desc,
                                          GCRenderPipelinePtr *out_pipeline) {
    if (!device || !desc || !out_pipeline)
        return ep_invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library)
        return ep_invalid_argument("shader library is NULL");
    
    GCRenderPipeline *pipeline = new (std::nothrow) GCRenderPipeline{};
    if (!pipeline) return ep_out_of_memory();
    
    pipeline->ep_device = device;
    pipeline->topology = D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
    
    // Get or create root signature
    if (desc->layout && desc->layout->root_signature) {
        pipeline->root_signature = desc->layout->root_signature;
    } else {
        HRESULT hr = create_default_root_signature(device->device.Get(),
                                                    pipeline->root_signature);
        if (FAILED(hr)) {
            delete pipeline;
            return ep_from_hresult(hr, "failed to create default root signature");
        }
    }
    
    // Build pipeline state description
    D3D12_GRAPHICS_PIPELINE_STATE_DESC pso_desc = {};
    pso_desc.pRootSignature = pipeline->root_signature.Get();
    
    // Vertex shader
    if (desc->vertex_entry && !desc->library->bytecode.empty()) {
        pso_desc.VS.pShaderBytecode = desc->library->bytecode.data();
        pso_desc.VS.BytecodeLength = desc->library->bytecode.size();
    }
    
    // Pixel shader
    if (desc->fragment_entry && !desc->library->bytecode.empty()) {
        // Note: In a real implementation, we'd compile separate shaders
        // For now, assume the bytecode contains the full shader
        pso_desc.PS.pShaderBytecode = desc->library->bytecode.data();
        pso_desc.PS.BytecodeLength = desc->library->bytecode.size();
    }
    
    // Input layout
    std::vector<D3D12_INPUT_ELEMENT_DESC> input_elements;
    if (desc->attribute_count > 0 && desc->attributes) {
        input_elements.reserve(desc->attribute_count);
        for (uint32_t i = 0; i < desc->attribute_count; i++) {
            D3D12_INPUT_ELEMENT_DESC element = {};
            element.SemanticName = "TEXCOORD"; // Generic semantic
            element.SemanticIndex = desc->attributes[i].location;
            element.Format = ep_to_vertex_format(desc->attributes[i].format);
            element.InputSlot = desc->attributes[i].binding;
            element.AlignedByteOffset = desc->attributes[i].offset;
            element.InputSlotClass = D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA;
            element.InstanceDataStepRate = 0;
            
            // Check if this binding is per-instance
            if (desc->bindings) {
                for (uint32_t b = 0; b < desc->binding_count; b++) {
                    if (desc->bindings[b].binding == desc->attributes[i].binding &&
                        desc->bindings[b].per_instance) {
                        element.InputSlotClass = D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA;
                        element.InstanceDataStepRate = 1;
                        break;
                    }
                }
            }
            
            input_elements.push_back(element);
        }
        
        pso_desc.InputLayout.pInputElementDescs = input_elements.data();
        pso_desc.InputLayout.NumElements = static_cast<UINT>(input_elements.size());
    }
    
    // Rasterizer state
    pso_desc.RasterizerState.FillMode = D3D12_FILL_MODE_SOLID;
    pso_desc.RasterizerState.CullMode = D3D12_CULL_MODE_BACK;
    pso_desc.RasterizerState.FrontCounterClockwise = FALSE;
    pso_desc.RasterizerState.DepthBias = 0;
    pso_desc.RasterizerState.DepthBiasClamp = 0.0f;
    pso_desc.RasterizerState.SlopeScaledDepthBias = 0.0f;
    pso_desc.RasterizerState.DepthClipEnable = TRUE;
    pso_desc.RasterizerState.MultisampleEnable = FALSE;
    pso_desc.RasterizerState.AntialiasedLineEnable = FALSE;
    pso_desc.RasterizerState.ForcedSampleCount = 0;
    pso_desc.RasterizerState.ConservativeRaster = D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF;
    
    // Blend state (simple alpha blending)
    pso_desc.BlendState.AlphaToCoverageEnable = FALSE;
    pso_desc.BlendState.IndependentBlendEnable = FALSE;
    for (uint32_t i = 0; i < 8; i++) {
        pso_desc.BlendState.RenderTarget[i].BlendEnable = FALSE;
        pso_desc.BlendState.RenderTarget[i].RenderTargetWriteMask = D3D12_COLOR_WRITE_ENABLE_ALL;
    }
    
    // Depth stencil state
    pso_desc.DepthStencilState.DepthEnable = desc->raster_state.depth_test_enable;
    pso_desc.DepthStencilState.DepthWriteMask = desc->raster_state.depth_write_enable
        ? D3D12_DEPTH_WRITE_MASK_ALL
        : D3D12_DEPTH_WRITE_MASK_ZERO;
    pso_desc.DepthStencilState.DepthFunc = ep_to_d3d12_compare(desc->raster_state.depth_compare);
    pso_desc.DepthStencilState.StencilEnable = FALSE;
    
    // Render targets
    pso_desc.NumRenderTargets = desc->color_format_count;
    for (uint32_t i = 0; i < desc->color_format_count && i < 8; i++) {
        pso_desc.RTVFormats[i] = ep_to_dxgi_format(desc->color_formats[i]);
    }
    
    // Depth format
    if (desc->depth_format == GC_FORMAT_D24S8 || desc->depth_format == GC_FORMAT_D32_FLOAT) {
        pso_desc.DSVFormat = ep_to_dxgi_format(desc->depth_format);
    }
    
    pso_desc.SampleMask = UINT_MAX;
    pso_desc.PrimitiveTopologyType = D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE;
    pso_desc.SampleDesc.Count = 1;
    pso_desc.SampleDesc.Quality = 0;
    
    HRESULT hr = device->device->CreateGraphicsPipelineState(&pso_desc,
                                                              IID_PPV_ARGS(&pipeline->state));
    if (FAILED(hr)) {
        delete pipeline;
        return ep_from_hresult(hr, "failed to create graphics pipeline state");
    }
    
    *out_pipeline = pipeline;
    return ep_ok();
}

extern "C" GCError GCRenderPipelineDestroy(GCRenderPipelinePtr pipeline) {
    delete pipeline;
    return ep_ok();
}

// ============================================================================
// Compute Pipeline
// ============================================================================

extern "C" GCError GCComputePipelineCreate(GCDevicePtr device,
                                           const GCComputePipelineDesc *desc,
                                           GCComputePipelinePtr *out_pipeline) {
    if (!device || !desc || !out_pipeline)
        return ep_invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library || !desc->entry)
        return ep_invalid_argument("library or entry is NULL");
    
    GCComputePipeline *pipeline = new (std::nothrow) GCComputePipeline{};
    if (!pipeline) return ep_out_of_memory();
    
    pipeline->ep_device = device;
    
    // Get or create root signature
    if (desc->layout && desc->layout->root_signature) {
        pipeline->root_signature = desc->layout->root_signature;
    } else {
        HRESULT hr = create_default_root_signature(device->device.Get(),
                                                    pipeline->root_signature);
        if (FAILED(hr)) {
            delete pipeline;
            return ep_from_hresult(hr, "failed to create default root signature");
        }
    }
    
    D3D12_COMPUTE_PIPELINE_STATE_DESC pso_desc = {};
    pso_desc.pRootSignature = pipeline->root_signature.Get();
    pso_desc.CS.pShaderBytecode = desc->library->bytecode.data();
    pso_desc.CS.BytecodeLength = desc->library->bytecode.size();
    
    HRESULT hr = device->device->CreateComputePipelineState(&pso_desc,
                                                             IID_PPV_ARGS(&pipeline->state));
    if (FAILED(hr)) {
        delete pipeline;
        return ep_from_hresult(hr, "failed to create compute pipeline state");
    }
    
    *out_pipeline = pipeline;
    return ep_ok();
}

extern "C" GCError GCComputePipelineDestroy(GCComputePipelinePtr pipeline) {
    delete pipeline;
    return ep_ok();
}

// ============================================================================
// Mesh Pipeline (DirectX 12 Ultimate)
// ============================================================================

// Pipeline state stream subobject types for mesh shaders (plain D3D12 structs)
// These replace the CD3DX12 helpers that require d3dx12.h

template<typename T, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE Type>
struct alignas(void*) PipelineStateStreamSubobject {
    D3D12_PIPELINE_STATE_SUBOBJECT_TYPE type = Type;
    T value = {};
};

extern "C" GCError GCMeshPipelineCreate(GCDevicePtr device,
                                        const GCMeshPipelineDesc *desc,
                                        GCMeshPipelinePtr *out_pipeline) {
    if (!device || !desc || !out_pipeline)
        return ep_invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library || !desc->mesh_entry)
        return ep_invalid_argument("library or mesh_entry is NULL");
    
    // Check mesh shader support
    D3D12_FEATURE_DATA_D3D12_OPTIONS7 options7 = {};
    HRESULT hr = device->device->CheckFeatureSupport(D3D12_FEATURE_D3D12_OPTIONS7,
                                                      &options7, sizeof(options7));
    if (FAILED(hr) || options7.MeshShaderTier == D3D12_MESH_SHADER_TIER_NOT_SUPPORTED) {
        return ep_unsupported("mesh shaders not supported on this device");
    }
    
    GCMeshPipeline *pipeline = new (std::nothrow) GCMeshPipeline{};
    if (!pipeline) return ep_out_of_memory();
    
    pipeline->ep_device = device;
    
    // Get or create root signature
    if (desc->layout && desc->layout->root_signature) {
        pipeline->root_signature = desc->layout->root_signature;
    } else {
        hr = create_default_root_signature(device->device.Get(), pipeline->root_signature);
        if (FAILED(hr)) {
            delete pipeline;
            return ep_from_hresult(hr, "failed to create default root signature");
        }
    }
    
    // Use pipeline state stream for mesh shaders using plain D3D12 structures
    struct alignas(void*) PSO_STREAM {
        PipelineStateStreamSubobject<ID3D12RootSignature*, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE> pRootSignature;
        PipelineStateStreamSubobject<D3D12_SHADER_BYTECODE, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_AS> AS;
        PipelineStateStreamSubobject<D3D12_SHADER_BYTECODE, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MS> MS;
        PipelineStateStreamSubobject<D3D12_SHADER_BYTECODE, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS> PS;
        PipelineStateStreamSubobject<D3D12_RASTERIZER_DESC, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER> Rasterizer;
        PipelineStateStreamSubobject<D3D12_BLEND_DESC, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND> Blend;
        PipelineStateStreamSubobject<D3D12_DEPTH_STENCIL_DESC, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL> DepthStencil;
        PipelineStateStreamSubobject<D3D12_RT_FORMAT_ARRAY, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS> RTFormats;
        PipelineStateStreamSubobject<DXGI_FORMAT, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT> DSFormat;
        PipelineStateStreamSubobject<DXGI_SAMPLE_DESC, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC> SampleDesc;
    } pso_stream = {};
    
    pso_stream.pRootSignature.value = pipeline->root_signature.Get();
    
    // Mesh shader
    if (!desc->library->bytecode.empty()) {
        pso_stream.MS.value.pShaderBytecode = desc->library->bytecode.data();
        pso_stream.MS.value.BytecodeLength = desc->library->bytecode.size();
    }
    
    // Amplification/task shader (optional)
    if (desc->task_entry && !desc->library->bytecode.empty()) {
        pso_stream.AS.value.pShaderBytecode = desc->library->bytecode.data();
        pso_stream.AS.value.BytecodeLength = desc->library->bytecode.size();
    }
    
    // Pixel shader
    if (desc->fragment_entry && !desc->library->bytecode.empty()) {
        pso_stream.PS.value.pShaderBytecode = desc->library->bytecode.data();
        pso_stream.PS.value.BytecodeLength = desc->library->bytecode.size();
    }
    
    // Rasterizer
    pso_stream.Rasterizer.value.FillMode = D3D12_FILL_MODE_SOLID;
    pso_stream.Rasterizer.value.CullMode = D3D12_CULL_MODE_BACK;
    pso_stream.Rasterizer.value.DepthClipEnable = TRUE;
    
    // Blend
    pso_stream.Blend.value.RenderTarget[0].RenderTargetWriteMask = D3D12_COLOR_WRITE_ENABLE_ALL;
    
    // Depth stencil
    pso_stream.DepthStencil.value.DepthEnable = desc->raster_state.depth_test_enable;
    pso_stream.DepthStencil.value.DepthWriteMask = desc->raster_state.depth_write_enable
        ? D3D12_DEPTH_WRITE_MASK_ALL : D3D12_DEPTH_WRITE_MASK_ZERO;
    pso_stream.DepthStencil.value.DepthFunc = ep_to_d3d12_compare(desc->raster_state.depth_compare);
    
    // Render targets
    pso_stream.RTFormats.value.NumRenderTargets = desc->color_format_count;
    for (uint32_t i = 0; i < desc->color_format_count && i < 8; i++) {
        pso_stream.RTFormats.value.RTFormats[i] = ep_to_dxgi_format(desc->color_formats[i]);
    }
    
    // Depth format
    if (desc->depth_format == GC_FORMAT_D24S8 || desc->depth_format == GC_FORMAT_D32_FLOAT) {
        pso_stream.DSFormat.value = ep_to_dxgi_format(desc->depth_format);
    }
    
    // Sample desc
    pso_stream.SampleDesc.value.Count = 1;
    pso_stream.SampleDesc.value.Quality = 0;
    
    D3D12_PIPELINE_STATE_STREAM_DESC stream_desc = {};
    stream_desc.SizeInBytes = sizeof(pso_stream);
    stream_desc.pPipelineStateSubobjectStream = &pso_stream;
    
    hr = device->device->CreatePipelineState(&stream_desc, IID_PPV_ARGS(&pipeline->state));
    if (FAILED(hr)) {
        delete pipeline;
        return ep_from_hresult(hr, "failed to create mesh pipeline state");
    }
    
    *out_pipeline = pipeline;
    return ep_ok();
}

extern "C" GCError GCMeshPipelineDestroy(GCMeshPipelinePtr pipeline) {
    delete pipeline;
    return ep_ok();
}

// ============================================================================
// Ray Tracing Pipeline (DXR 1.1)
// ============================================================================

extern "C" GCError GCRayTracingPipelineCreate(GCDevicePtr device,
                                               const GCRayTracingPipelineDesc *desc,
                                               GCRayTracingPipelinePtr *out_pipeline) {
    if (!device || !desc || !out_pipeline)
        return ep_invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library)
        return ep_invalid_argument("library is NULL");
    
    // Check ray tracing support
    D3D12_FEATURE_DATA_D3D12_OPTIONS5 options5 = {};
    HRESULT hr = device->device->CheckFeatureSupport(D3D12_FEATURE_D3D12_OPTIONS5,
                                                      &options5, sizeof(options5));
    if (FAILED(hr) || options5.RaytracingTier == D3D12_RAYTRACING_TIER_NOT_SUPPORTED) {
        return ep_unsupported("ray tracing not supported on this device");
    }
    
    GCRayTracingPipeline *pipeline = new (std::nothrow) GCRayTracingPipeline{};
    if (!pipeline) return ep_out_of_memory();
    
    pipeline->ep_device = device;
    pipeline->max_recursion_depth = desc->max_recursion_depth;
    
    // Get or create root signature
    if (desc->layout && desc->layout->root_signature) {
        pipeline->root_signature = desc->layout->root_signature;
    } else {
        hr = create_default_root_signature(device->device.Get(), pipeline->root_signature);
        if (FAILED(hr)) {
            delete pipeline;
            return ep_from_hresult(hr, "failed to create default root signature");
        }
    }
    
    // Build state object
    std::vector<D3D12_STATE_SUBOBJECT> subobjects;
    std::vector<D3D12_DXIL_LIBRARY_DESC> dxil_libs;
    std::vector<D3D12_HIT_GROUP_DESC> hit_groups;
    std::vector<std::wstring> export_names;
    
    // DXIL library
    D3D12_DXIL_LIBRARY_DESC lib_desc = {};
    lib_desc.DXILLibrary.pShaderBytecode = desc->library->bytecode.data();
    lib_desc.DXILLibrary.BytecodeLength = desc->library->bytecode.size();
    lib_desc.NumExports = 0; // Export all
    dxil_libs.push_back(lib_desc);
    
    D3D12_STATE_SUBOBJECT lib_subobject = {};
    lib_subobject.Type = D3D12_STATE_SUBOBJECT_TYPE_DXIL_LIBRARY;
    lib_subobject.pDesc = &dxil_libs.back();
    subobjects.push_back(lib_subobject);
    
    // Hit groups
    for (uint32_t i = 0; i < desc->group_count; i++) {
        const GCRayTracingShaderGroupDesc *group = &desc->groups[i];
        
        if (group->hit_entry) {
            D3D12_HIT_GROUP_DESC hit_group = {};
            hit_group.Type = D3D12_HIT_GROUP_TYPE_TRIANGLES;
            
            // Convert entry names to wide strings
            // This is simplified - real impl would handle all entries
            
            hit_groups.push_back(hit_group);
            
            D3D12_STATE_SUBOBJECT hit_subobject = {};
            hit_subobject.Type = D3D12_STATE_SUBOBJECT_TYPE_HIT_GROUP;
            hit_subobject.pDesc = &hit_groups.back();
            subobjects.push_back(hit_subobject);
        }
    }
    
    // Shader config
    D3D12_RAYTRACING_SHADER_CONFIG shader_config = {};
    shader_config.MaxPayloadSizeInBytes = 32;
    shader_config.MaxAttributeSizeInBytes = 8;
    
    D3D12_STATE_SUBOBJECT shader_config_subobject = {};
    shader_config_subobject.Type = D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_SHADER_CONFIG;
    shader_config_subobject.pDesc = &shader_config;
    subobjects.push_back(shader_config_subobject);
    
    // Global root signature
    D3D12_GLOBAL_ROOT_SIGNATURE global_root_sig = {};
    global_root_sig.pGlobalRootSignature = pipeline->root_signature.Get();
    
    D3D12_STATE_SUBOBJECT global_root_sig_subobject = {};
    global_root_sig_subobject.Type = D3D12_STATE_SUBOBJECT_TYPE_GLOBAL_ROOT_SIGNATURE;
    global_root_sig_subobject.pDesc = &global_root_sig;
    subobjects.push_back(global_root_sig_subobject);
    
    // Pipeline config
    D3D12_RAYTRACING_PIPELINE_CONFIG1 pipeline_config = {};
    pipeline_config.MaxTraceRecursionDepth = desc->max_recursion_depth;
    pipeline_config.Flags = D3D12_RAYTRACING_PIPELINE_FLAG_NONE;
    
    D3D12_STATE_SUBOBJECT pipeline_config_subobject = {};
    pipeline_config_subobject.Type = D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG1;
    pipeline_config_subobject.pDesc = &pipeline_config;
    subobjects.push_back(pipeline_config_subobject);
    
    // Create state object
    D3D12_STATE_OBJECT_DESC state_object_desc = {};
    state_object_desc.Type = D3D12_STATE_OBJECT_TYPE_RAYTRACING_PIPELINE;
    state_object_desc.NumSubobjects = static_cast<UINT>(subobjects.size());
    state_object_desc.pSubobjects = subobjects.data();
    
    hr = device->device->CreateStateObject(&state_object_desc, IID_PPV_ARGS(&pipeline->state));
    if (FAILED(hr)) {
        delete pipeline;
        return ep_from_hresult(hr, "failed to create ray tracing state object");
    }
    
    *out_pipeline = pipeline;
    return ep_ok();
}

extern "C" GCError GCRayTracingPipelineDestroy(GCRayTracingPipelinePtr pipeline) {
    delete pipeline;
    return ep_ok();
}

// ============================================================================
// Acceleration Structure
// ============================================================================

extern "C" GCError GCAccelerationStructureCreate(GCDevicePtr device,
                                                  const GCAccelerationStructureDesc *desc,
                                                  GCAccelerationStructurePtr *out_as) {
    if (!device || !desc || !out_as)
        return ep_invalid_argument("device, desc, or out_as is NULL");
    
    // Check ray tracing support
    D3D12_FEATURE_DATA_D3D12_OPTIONS5 options5 = {};
    HRESULT hr = device->device->CheckFeatureSupport(D3D12_FEATURE_D3D12_OPTIONS5,
                                                      &options5, sizeof(options5));
    if (FAILED(hr) || options5.RaytracingTier == D3D12_RAYTRACING_TIER_NOT_SUPPORTED) {
        return ep_unsupported("ray tracing not supported on this device");
    }
    
    GCAccelerationStructure *as = new (std::nothrow) GCAccelerationStructure{};
    if (!as) return ep_out_of_memory();
    
    as->ep_device = device;
    as->top_level = desc->top_level;
    
    D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS inputs = {};
    inputs.Type = desc->top_level
        ? D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL
        : D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL;
    inputs.Flags = desc->allow_update
        ? D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_UPDATE
        : D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_NONE;
    
    std::vector<D3D12_RAYTRACING_GEOMETRY_DESC> geometries;
    
    if (!desc->top_level) {
        geometries.reserve(desc->geometry_count);
        for (uint32_t i = 0; i < desc->geometry_count; i++) {
            const GCAccelerationStructureGeometryDesc *geo = &desc->geometries[i];
            
            D3D12_RAYTRACING_GEOMETRY_DESC geo_desc = {};
            geo_desc.Type = D3D12_RAYTRACING_GEOMETRY_TYPE_TRIANGLES;
            geo_desc.Flags = geo->opaque
                ? D3D12_RAYTRACING_GEOMETRY_FLAG_OPAQUE
                : D3D12_RAYTRACING_GEOMETRY_FLAG_NONE;
            
            if (geo->vertex_buffer) {
                geo_desc.Triangles.VertexBuffer.StartAddress = 
                    geo->vertex_buffer->gpu_address + geo->vertex_offset;
                geo_desc.Triangles.VertexBuffer.StrideInBytes = geo->vertex_stride;
                geo_desc.Triangles.VertexCount = geo->vertex_count;
                geo_desc.Triangles.VertexFormat = DXGI_FORMAT_R32G32B32_FLOAT;
            }
            
            if (geo->index_buffer) {
                geo_desc.Triangles.IndexBuffer = 
                    geo->index_buffer->gpu_address + geo->index_offset;
                geo_desc.Triangles.IndexCount = geo->index_count;
                geo_desc.Triangles.IndexFormat = DXGI_FORMAT_R32_UINT;
            }
            
            geometries.push_back(geo_desc);
        }
        
        inputs.NumDescs = static_cast<UINT>(geometries.size());
        inputs.pGeometryDescs = geometries.data();
        inputs.DescsLayout = D3D12_ELEMENTS_LAYOUT_ARRAY;
    } else {
        inputs.NumDescs = desc->geometry_count; // Instance count for TLAS
        inputs.DescsLayout = D3D12_ELEMENTS_LAYOUT_ARRAY;
    }
    
    // Get prebuild info
    D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO prebuild_info = {};
    device->device->GetRaytracingAccelerationStructurePrebuildInfo(&inputs, &prebuild_info);
    
    // Create result buffer
    D3D12_RESOURCE_DESC result_desc = {};
    result_desc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
    result_desc.Width = prebuild_info.ResultDataMaxSizeInBytes;
    result_desc.Height = 1;
    result_desc.DepthOrArraySize = 1;
    result_desc.MipLevels = 1;
    result_desc.Format = DXGI_FORMAT_UNKNOWN;
    result_desc.SampleDesc.Count = 1;
    result_desc.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;
    result_desc.Flags = D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS;
    
    D3D12_HEAP_PROPERTIES heap_props = {};
    heap_props.Type = D3D12_HEAP_TYPE_DEFAULT;
    
    hr = device->device->CreateCommittedResource(
        &heap_props,
        D3D12_HEAP_FLAG_NONE,
        &result_desc,
        D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE,
        nullptr,
        IID_PPV_ARGS(&as->result));
    
    if (FAILED(hr)) {
        delete as;
        return ep_from_hresult(hr, "failed to create acceleration structure result buffer");
    }
    
    // Create scratch buffer
    D3D12_RESOURCE_DESC scratch_desc = result_desc;
    scratch_desc.Width = prebuild_info.ScratchDataSizeInBytes;
    
    hr = device->device->CreateCommittedResource(
        &heap_props,
        D3D12_HEAP_FLAG_NONE,
        &scratch_desc,
        D3D12_RESOURCE_STATE_COMMON,
        nullptr,
        IID_PPV_ARGS(&as->scratch));
    
    if (FAILED(hr)) {
        delete as;
        return ep_from_hresult(hr, "failed to create acceleration structure scratch buffer");
    }
    
    as->gpu_address = as->result->GetGPUVirtualAddress();
    
    *out_as = as;
    return ep_ok();
}

extern "C" GCError GCAccelerationStructureDestroy(GCAccelerationStructurePtr as) {
    delete as;
    return ep_ok();
}
