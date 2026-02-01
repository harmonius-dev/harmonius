// gcraft_instance.cpp - Platform, instance, and adapter implementation

#include "gcraft_internal.h"

// ============================================================================
// Platform
// ============================================================================

extern "C" GCError GCPlatformCreate(GCPlatformPtr *out_platform) {
    if (!out_platform) return ep_invalid_argument("out_platform is NULL");
    
    GCPlatform *platform = new (std::nothrow) GCPlatform{};
    if (!platform) return ep_out_of_memory();
    
    *out_platform = platform;
    return ep_ok();
}

extern "C" GCError GCPlatformDestroy(GCPlatformPtr platform) {
    delete platform;
    return ep_ok();
}

// ============================================================================
// Instance
// ============================================================================

extern "C" GCError GCInstanceCreate(const GCInstanceDesc *desc, GCInstancePtr *out_instance) {
    if (!desc || !out_instance) return ep_invalid_argument("desc or out_instance is NULL");
    
    GCInstance *instance = new (std::nothrow) GCInstance{};
    if (!instance) return ep_out_of_memory();
    
    instance->enabled_backends = desc->enable_backends;
    instance->validation_enabled = desc->enable_validation;
    instance->debug_names_enabled = desc->enable_debug_names;
    
    // Enable debug layer if validation requested
    if (desc->enable_validation) {
        HRESULT hr = D3D12GetDebugInterface(IID_PPV_ARGS(&instance->debug));
        if (SUCCEEDED(hr) && instance->debug) {
            instance->debug->EnableDebugLayer();
            instance->debug->SetEnableGPUBasedValidation(TRUE);
            instance->debug->SetEnableSynchronizedCommandQueueValidation(TRUE);
        }
    }
    
    // Create DXGI factory
    UINT factory_flags = 0;
    if (desc->enable_validation) {
        factory_flags |= DXGI_CREATE_FACTORY_DEBUG;
    }
    
    HRESULT hr = CreateDXGIFactory2(factory_flags, IID_PPV_ARGS(&instance->factory));
    if (FAILED(hr)) {
        delete instance;
        return ep_from_hresult(hr, "failed to create DXGI factory");
    }
    
    // Enumerate adapters
    if (desc->enable_backends & GC_BACKEND_D3D12_BIT) {
        ComPtr<IDXGIAdapter1> adapter1;
        for (UINT i = 0; instance->factory->EnumAdapterByGpuPreference(
                i, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
                IID_PPV_ARGS(&adapter1)) != DXGI_ERROR_NOT_FOUND; i++) {
            
            ComPtr<IDXGIAdapter4> adapter4;
            if (SUCCEEDED(adapter1.As(&adapter4))) {
                // Check if this adapter supports D3D12
                if (SUCCEEDED(D3D12CreateDevice(adapter4.Get(), D3D_FEATURE_LEVEL_12_0,
                                                __uuidof(ID3D12Device), nullptr))) {
                    instance->adapters.push_back(adapter4);
                }
            }
        }
    }
    
    *out_instance = instance;
    return ep_ok();
}

extern "C" GCError GCInstanceDestroy(GCInstancePtr instance) {
    delete instance;
    return ep_ok();
}

extern "C" GCError GCInstanceEnumerateAdapters(GCInstancePtr instance, uint32_t *io_count,
                                               GCAdapterPtr *out_adapters) {
    if (!instance || !io_count) return ep_invalid_argument("instance or io_count is NULL");
    
    uint32_t adapter_count = static_cast<uint32_t>(instance->adapters.size());
    
    if (!out_adapters) {
        *io_count = adapter_count;
        return ep_ok();
    }
    
    uint32_t count = (*io_count < adapter_count) ? *io_count : adapter_count;
    
    for (uint32_t i = 0; i < count; i++) {
        GCAdapter *adapter = new (std::nothrow) GCAdapter{};
        if (!adapter) {
            // Clean up any adapters that were successfully allocated in previous iterations
            for (uint32_t j = 0; j < i; ++j) {
                delete out_adapters[j];
                out_adapters[j] = nullptr;
            }
            return ep_out_of_memory();
        }
        
        adapter->adapter = instance->adapters[i];
        adapter->adapter->GetDesc3(&adapter->desc);
        
        // Fill properties
        WideCharToMultiByte(CP_UTF8, 0, adapter->desc.Description, -1,
                           adapter->properties.name, sizeof(adapter->properties.name),
                           nullptr, nullptr);
        adapter->properties.vendor_id = adapter->desc.VendorId;
        adapter->properties.device_id = adapter->desc.DeviceId;
        adapter->properties.backends = GC_BACKEND_D3D12_BIT;
        
        // Feature detection
        GCFeatureFlags features = GC_FEATURE_COMPUTE_BIT | GC_FEATURE_TIMELINE_SEMAPHORE_BIT;
        
        // Create temporary device to query features
        ComPtr<ID3D12Device8> temp_device;
        if (SUCCEEDED(D3D12CreateDevice(adapter->adapter.Get(), D3D_FEATURE_LEVEL_12_0,
                                        IID_PPV_ARGS(&temp_device)))) {
            // Check mesh shader support (requires SM 6.5)
            D3D12_FEATURE_DATA_D3D12_OPTIONS7 options7 = {};
            if (SUCCEEDED(temp_device->CheckFeatureSupport(
                    D3D12_FEATURE_D3D12_OPTIONS7, &options7, sizeof(options7)))) {
                if (options7.MeshShaderTier != D3D12_MESH_SHADER_TIER_NOT_SUPPORTED) {
                    features |= GC_FEATURE_MESH_SHADER_BIT;
                }
            }
            
            // Check ray tracing support
            D3D12_FEATURE_DATA_D3D12_OPTIONS5 options5 = {};
            if (SUCCEEDED(temp_device->CheckFeatureSupport(
                    D3D12_FEATURE_D3D12_OPTIONS5, &options5, sizeof(options5)))) {
                if (options5.RaytracingTier != D3D12_RAYTRACING_TIER_NOT_SUPPORTED) {
                    features |= GC_FEATURE_RAY_TRACING_BIT;
                }
            }
            
            // Bindless is available via SM 6.6 / resource binding tier 3
            D3D12_FEATURE_DATA_D3D12_OPTIONS options = {};
            if (SUCCEEDED(temp_device->CheckFeatureSupport(
                    D3D12_FEATURE_D3D12_OPTIONS, &options, sizeof(options)))) {
                if (options.ResourceBindingTier >= D3D12_RESOURCE_BINDING_TIER_3) {
                    features |= GC_FEATURE_DESCRIPTOR_INDEXING_BIT | GC_FEATURE_BINDLESS_BIT;
                }
            }
        }
        
        adapter->properties.features = features;
        
        // Limits
        adapter->properties.limits.max_texture_dimension_2d = 16384;
        adapter->properties.limits.max_texture_dimension_3d = 2048;
        adapter->properties.limits.max_texture_array_layers = 2048;
        adapter->properties.limits.max_bindless_resources = 1000000; // SM 6.6 allows unbounded
        adapter->properties.limits.max_push_constants_size = 256; // Root constants limit
        adapter->properties.limits.max_threads_per_threadgroup = 1024;
        
        out_adapters[i] = adapter;
    }
    
    *io_count = count;
    return ep_ok();
}

extern "C" GCError GCAdapterGetProperties(GCAdapterPtr adapter, GCAdapterProperties *out_properties) {
    if (!adapter || !out_properties) return ep_invalid_argument("adapter or out_properties is NULL");
    *out_properties = adapter->properties;
    return ep_ok();
}
