// gcraft_internal.h - Internal structures and utilities for D3D12 backend
// Requires Windows 10 1903+ for DirectStorage, DirectX 12 Ultimate features

#ifndef GCRAFT_D3D12_INTERNAL_H
#define GCRAFT_D3D12_INTERNAL_H

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif

#include <windows.h>
#include <d3d12.h>
#include <dxgi1_6.h>
#include <d3dcompiler.h>
#include <dstorage.h>
#include <dxcapi.h>

#include <wrl/client.h>
using Microsoft::WRL::ComPtr;

#include <cstdlib>
#include <cstring>
#include <mutex>
#include <condition_variable>
#include <vector>
#include <atomic>
#include <string>
#include <new>

extern "C" {
#include "gcraft_types.h"
#include "gcraft_command.h"
#include "gcraft_descriptor.h"
#include "gcraft_device.h"
#include "gcraft_instance.h"
#include "gcraft_pipeline.h"
#include "gcraft_resource.h"
#include "gcraft_surface.h"
#include "gcraft_sync.h"
}

// ============================================================================
// Error Helpers
// ============================================================================

inline GCError ep_ok() {
    return GCError{GC_E_OK, nullptr};
}

inline GCError ep_error(GCErrorCode code, const char *msg) {
    return GCError{code, msg};
}

inline GCError ep_invalid_argument(const char *msg) {
    return ep_error(GC_E_INVALID_ARGUMENT, msg);
}

inline GCError ep_out_of_memory() {
    return ep_error(GC_E_OUT_OF_MEMORY, "out of memory");
}

inline GCError ep_unsupported(const char *msg) {
    return ep_error(GC_E_UNSUPPORTED, msg);
}

inline GCError ep_device_lost(const char *msg) {
    return ep_error(GC_E_DEVICE_LOST, msg);
}

inline GCError ep_from_hresult(HRESULT hr, const char *msg) {
    if (SUCCEEDED(hr)) return ep_ok();
    if (hr == E_OUTOFMEMORY) return ep_out_of_memory();
    if (hr == DXGI_ERROR_DEVICE_REMOVED || hr == DXGI_ERROR_DEVICE_RESET)
        return ep_device_lost(msg);
    return ep_error(GC_E_INVALID_STATE, msg);
}

inline GCError ep_invalid_state(const char *msg) {
    return ep_error(GC_E_INVALID_STATE, msg);
}

// ============================================================================
// Format Conversion
// ============================================================================

DXGI_FORMAT ep_to_dxgi_format(GCTextureFormat format);
D3D12_FILTER ep_to_d3d12_filter(GCFilter min_filter, GCFilter mag_filter);
D3D12_TEXTURE_ADDRESS_MODE ep_to_d3d12_address(GCAddressMode mode);
D3D12_COMPARISON_FUNC ep_to_d3d12_compare(GCCompareOp op);
D3D12_RESOURCE_STATES ep_to_d3d12_resource_state(GCTextureLayout layout);
D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE ep_to_d3d12_load_op(GCAttachmentLoadOp op);
D3D12_RENDER_PASS_ENDING_ACCESS_TYPE ep_to_d3d12_store_op(GCAttachmentStoreOp op);
D3D12_RESOURCE_DIMENSION ep_to_d3d12_dimension(GCTextureDimension dim);
D3D12_SHADER_VISIBILITY ep_to_d3d12_visibility(GCShaderStageFlags stages);

// ============================================================================
// Internal Structures
// ============================================================================

struct GCPlatform {
    int placeholder;
};

struct GCInstance {
    GCBackendFlags enabled_backends;
    bool validation_enabled;
    bool debug_names_enabled;
    ComPtr<IDXGIFactory6> factory;
    ComPtr<ID3D12Debug3> debug;
    std::vector<ComPtr<IDXGIAdapter4>> adapters;
};

struct GCAdapter {
    ComPtr<IDXGIAdapter4> adapter;
    DXGI_ADAPTER_DESC3 desc;
    GCAdapterProperties properties;
};

struct GCDevice {
    ComPtr<ID3D12Device8> device;
    ComPtr<ID3D12CommandQueue> graphics_queue;
    ComPtr<ID3D12CommandQueue> compute_queue;
    ComPtr<ID3D12CommandQueue> copy_queue;
    ComPtr<IDStorageFactory> dstorage_factory;
    ComPtr<IDStorageQueue> dstorage_queue;
    
    // Descriptor heaps
    ComPtr<ID3D12DescriptorHeap> rtv_heap;
    ComPtr<ID3D12DescriptorHeap> dsv_heap;
    ComPtr<ID3D12DescriptorHeap> cbv_srv_uav_heap;
    ComPtr<ID3D12DescriptorHeap> sampler_heap;
    
    UINT rtv_descriptor_size;
    UINT dsv_descriptor_size;
    UINT cbv_srv_uav_descriptor_size;
    UINT sampler_descriptor_size;
    
    std::atomic<UINT> rtv_heap_offset{0};
    std::atomic<UINT> dsv_heap_offset{0};
    std::atomic<UINT> cbv_srv_uav_heap_offset{0};
    std::atomic<UINT> sampler_heap_offset{0};
    
    GCFeatureFlags features;
    bool validation_enabled;
    bool debug_names_enabled;
};

struct GCQueue {
    GCDevice *ep_device;
    ComPtr<ID3D12CommandQueue> queue;
    GCQueueType type;
};

struct GCCommandBuffer {
    GCDevice *ep_device;
    ComPtr<ID3D12CommandAllocator> allocator;
    ComPtr<ID3D12GraphicsCommandList7> list;
    bool is_recording;
    bool in_render_pass;
    
    // Current state
    ID3D12RootSignature *current_root_signature;
    D3D12_PRIMITIVE_TOPOLOGY current_topology;
};

struct GCFence {
    GCDevice *ep_device;
    ComPtr<ID3D12Fence1> fence;
    std::atomic<uint64_t> value{0};
    HANDLE event;
    std::mutex mutex;
};

struct GCTimelineSemaphore {
    GCDevice *ep_device;
    ComPtr<ID3D12Fence1> fence;
};

struct GCBuffer {
    GCDevice *ep_device;
    ComPtr<ID3D12Resource> resource;
    D3D12_GPU_VIRTUAL_ADDRESS gpu_address;
    void *mapped_ptr;
    uint64_t size;
    GCBufferUsageFlags usage;
    bool host_visible;
};

struct GCTexture {
    GCDevice *ep_device;
    ComPtr<ID3D12Resource> resource;
    D3D12_CPU_DESCRIPTOR_HANDLE rtv;
    D3D12_CPU_DESCRIPTOR_HANDLE dsv;
    D3D12_CPU_DESCRIPTOR_HANDLE srv;
    D3D12_CPU_DESCRIPTOR_HANDLE uav;
    // Heap indices for shader-visible descriptors
    UINT srv_heap_index;
    UINT uav_heap_index;
    GCTextureDesc desc;
    D3D12_RESOURCE_STATES current_state;
};

struct GCSampler {
    GCDevice *ep_device;
    D3D12_CPU_DESCRIPTOR_HANDLE sampler;
    UINT heap_index;
    D3D12_SAMPLER_DESC desc;
};

struct GCShaderLibrary {
    GCDevice *ep_device;
    ComPtr<IDxcBlob> blob;
    std::vector<uint8_t> bytecode;
};

struct GCDescriptorBindingInfo {
    uint32_t binding;
    GCDescriptorType type;
    uint32_t count;
    GCShaderStageFlags stages;
};

struct GCDescriptorSetLayout {
    GCDevice *ep_device;
    std::vector<GCDescriptorBindingInfo> bindings;
};

union GCDescriptorValue {
    struct {
        ID3D12Resource *buffer;
        uint64_t offset;
        uint64_t range;
        UINT heap_index;  // Index in CBV/SRV/UAV heap
    } buffer_info;
    struct {
        UINT heap_index;  // Index in CBV/SRV/UAV heap for SRV/UAV
    } texture_info;
    struct {
        UINT heap_index;  // Index in sampler heap
    } sampler_info;
    D3D12_GPU_VIRTUAL_ADDRESS accel;
};

struct GCDescriptorSetEntry {
    GCDescriptorType type;
    GCDescriptorValue value;
};

struct GCDescriptorSet {
    GCDevice *ep_device;
    GCDescriptorSetLayout *layout;
    std::vector<GCDescriptorSetEntry> entries;
};

struct GCPipelineLayout {
    GCDevice *ep_device;
    ComPtr<ID3D12RootSignature> root_signature;
    std::vector<GCDescriptorSetLayout *> set_layouts;
    uint32_t push_constant_size;
    GCShaderStageFlags push_constant_stages;
    UINT push_constant_root_index;  // Root parameter index for push constants
    UINT first_set_root_index;      // Root parameter index for first descriptor set
};

struct GCRenderPipeline {
    GCDevice *ep_device;
    ComPtr<ID3D12PipelineState> state;
    ComPtr<ID3D12RootSignature> root_signature;
    D3D12_PRIMITIVE_TOPOLOGY topology;
};

struct GCComputePipeline {
    GCDevice *ep_device;
    ComPtr<ID3D12PipelineState> state;
    ComPtr<ID3D12RootSignature> root_signature;
};

struct GCMeshPipeline {
    GCDevice *ep_device;
    ComPtr<ID3D12PipelineState> state;
    ComPtr<ID3D12RootSignature> root_signature;
};

struct GCRayTracingPipeline {
    GCDevice *ep_device;
    ComPtr<ID3D12StateObject> state;
    ComPtr<ID3D12RootSignature> root_signature;
    ComPtr<ID3D12Resource> shader_table;
    D3D12_GPU_VIRTUAL_ADDRESS raygen_record;
    D3D12_GPU_VIRTUAL_ADDRESS miss_record;
    D3D12_GPU_VIRTUAL_ADDRESS hitgroup_record;
    uint32_t max_recursion_depth;
};

struct GCAccelerationStructure {
    GCDevice *ep_device;
    ComPtr<ID3D12Resource> result;
    ComPtr<ID3D12Resource> scratch;
    D3D12_GPU_VIRTUAL_ADDRESS gpu_address;
    bool top_level;
};

struct GCSurface {
    HWND hwnd;
    GCNativeHandleType handle_type;
};

struct GCSwapchain {
    GCDevice *ep_device;
    GCSurface *surface;
    ComPtr<IDXGISwapChain4> swapchain;
    std::vector<ComPtr<ID3D12Resource>> buffers;
    std::vector<D3D12_CPU_DESCRIPTOR_HANDLE> rtvs;
    uint32_t image_count;
    uint32_t current_index;
    GCTextureFormat format;
    uint32_t width;
    uint32_t height;
};

// ============================================================================
// DirectStorage helpers
// ============================================================================

struct GCDirectStorageRequest {
    ComPtr<IDStorageFile> file;
    GCBuffer *destination;
    uint64_t file_offset;
    uint64_t size;
};

#endif // GCRAFT_D3D12_INTERNAL_H
