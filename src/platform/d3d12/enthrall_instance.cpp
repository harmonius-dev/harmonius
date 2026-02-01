// enthrall_instance.cpp - Platform, instance, and adapter implementation

#include "enthrall_internal.h"

// ============================================================================
// Platform
// ============================================================================

extern "C" EPError EPPlatformCreate(EPPlatformPtr *out_platform) {
    if (!out_platform) return ep_invalid_argument("out_platform is NULL");
    
    EPPlatform *platform = new (std::nothrow) EPPlatform{};
    if (!platform) return ep_out_of_memory();
    
    *out_platform = platform;
    return ep_ok();
}

extern "C" EPError EPPlatformDestroy(EPPlatformPtr platform) {
    delete platform;
    return ep_ok();
}

// ============================================================================
// Instance
// ============================================================================

extern "C" EPError EPInstanceCreate(const EPInstanceDesc *desc, EPInstancePtr *out_instance) {
    if (!desc || !out_instance) return ep_invalid_argument("desc or out_instance is NULL");
    
    EPInstance *instance = new (std::nothrow) EPInstance{};
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
    if (desc->enable_backends & EP_BACKEND_D3D12_BIT) {
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

extern "C" EPError EPInstanceDestroy(EPInstancePtr instance) {
    delete instance;
    return ep_ok();
}

extern "C" EPError EPInstanceEnumerateAdapters(EPInstancePtr instance, uint32_t *io_count,
                                               EPAdapterPtr *out_adapters) {
    if (!instance || !io_count) return ep_invalid_argument("instance or io_count is NULL");
    
    uint32_t adapter_count = static_cast<uint32_t>(instance->adapters.size());
    
    if (!out_adapters) {
        *io_count = adapter_count;
        return ep_ok();
    }
    
    uint32_t count = (*io_count < adapter_count) ? *io_count : adapter_count;
    
    for (uint32_t i = 0; i < count; i++) {
        EPAdapter *adapter = new (std::nothrow) EPAdapter{};
        if (!adapter) return ep_out_of_memory();
        
        adapter->adapter = instance->adapters[i];
        adapter->adapter->GetDesc3(&adapter->desc);
        
        // Fill properties
        WideCharToMultiByte(CP_UTF8, 0, adapter->desc.Description, -1,
                           adapter->properties.name, sizeof(adapter->properties.name),
                           nullptr, nullptr);
        adapter->properties.vendor_id = adapter->desc.VendorId;
        adapter->properties.device_id = adapter->desc.DeviceId;
        adapter->properties.backends = EP_BACKEND_D3D12_BIT;
        
        // Feature detection
        EPFeatureFlags features = EP_FEATURE_COMPUTE_BIT | EP_FEATURE_TIMELINE_SEMAPHORE_BIT;
        
        // Create temporary device to query features
        ComPtr<ID3D12Device8> temp_device;
        if (SUCCEEDED(D3D12CreateDevice(adapter->adapter.Get(), D3D_FEATURE_LEVEL_12_0,
                                        IID_PPV_ARGS(&temp_device)))) {
            // Check mesh shader support (requires SM 6.5)
            D3D12_FEATURE_DATA_D3D12_OPTIONS7 options7 = {};
            if (SUCCEEDED(temp_device->CheckFeatureSupport(
                    D3D12_FEATURE_D3D12_OPTIONS7, &options7, sizeof(options7)))) {
                if (options7.MeshShaderTier != D3D12_MESH_SHADER_TIER_NOT_SUPPORTED) {
                    features |= EP_FEATURE_MESH_SHADER_BIT;
                }
            }
            
            // Check ray tracing support
            D3D12_FEATURE_DATA_D3D12_OPTIONS5 options5 = {};
            if (SUCCEEDED(temp_device->CheckFeatureSupport(
                    D3D12_FEATURE_D3D12_OPTIONS5, &options5, sizeof(options5)))) {
                if (options5.RaytracingTier != D3D12_RAYTRACING_TIER_NOT_SUPPORTED) {
                    features |= EP_FEATURE_RAY_TRACING_BIT;
                }
            }
            
            // Bindless is available via SM 6.6 / resource binding tier 3
            D3D12_FEATURE_DATA_D3D12_OPTIONS options = {};
            if (SUCCEEDED(temp_device->CheckFeatureSupport(
                    D3D12_FEATURE_D3D12_OPTIONS, &options, sizeof(options)))) {
                if (options.ResourceBindingTier >= D3D12_RESOURCE_BINDING_TIER_3) {
                    features |= EP_FEATURE_DESCRIPTOR_INDEXING_BIT | EP_FEATURE_BINDLESS_BIT;
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

extern "C" EPError EPAdapterGetProperties(EPAdapterPtr adapter, EPAdapterProperties *out_properties) {
    if (!adapter || !out_properties) return ep_invalid_argument("adapter or out_properties is NULL");
    *out_properties = adapter->properties;
    return ep_ok();
}
