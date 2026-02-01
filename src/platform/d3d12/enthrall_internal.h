// enthrall_internal.h - Internal structures and utilities for D3D12 backend
// Requires Windows 10 1903+ for DirectStorage, DirectX 12 Ultimate features

#ifndef ENTHRALL_D3D12_INTERNAL_H
#define ENTHRALL_D3D12_INTERNAL_H

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

extern "C" {
#include "enthrall_types.h"
#include "enthrall_command.h"
#include "enthrall_descriptor.h"
#include "enthrall_device.h"
#include "enthrall_instance.h"
#include "enthrall_pipeline.h"
#include "enthrall_resource.h"
#include "enthrall_surface.h"
#include "enthrall_sync.h"
}

// ============================================================================
// Error Helpers
// ============================================================================

inline EPError ep_ok() {
    return EPError{EP_E_OK, nullptr};
}

inline EPError ep_error(EPErrorCode code, const char *msg) {
    return EPError{code, msg};
}

inline EPError ep_invalid_argument(const char *msg) {
    return ep_error(EP_E_INVALID_ARGUMENT, msg);
}

inline EPError ep_out_of_memory() {
    return ep_error(EP_E_OUT_OF_MEMORY, "out of memory");
}

inline EPError ep_unsupported(const char *msg) {
    return ep_error(EP_E_UNSUPPORTED, msg);
}

inline EPError ep_device_lost(const char *msg) {
    return ep_error(EP_E_DEVICE_LOST, msg);
}

inline EPError ep_from_hresult(HRESULT hr, const char *msg) {
    if (SUCCEEDED(hr)) return ep_ok();
    if (hr == E_OUTOFMEMORY) return ep_out_of_memory();
    if (hr == DXGI_ERROR_DEVICE_REMOVED || hr == DXGI_ERROR_DEVICE_RESET)
        return ep_device_lost(msg);
    return ep_error(EP_E_INVALID_STATE, msg);
}

// ============================================================================
// Format Conversion
// ============================================================================

DXGI_FORMAT ep_to_dxgi_format(EPTextureFormat format);
D3D12_FILTER ep_to_d3d12_filter(EPFilter min_filter, EPFilter mag_filter);
D3D12_TEXTURE_ADDRESS_MODE ep_to_d3d12_address(EPAddressMode mode);
D3D12_COMPARISON_FUNC ep_to_d3d12_compare(EPCompareOp op);
D3D12_RESOURCE_STATES ep_to_d3d12_resource_state(EPTextureLayout layout);
D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE ep_to_d3d12_load_op(EPAttachmentLoadOp op);
D3D12_RENDER_PASS_ENDING_ACCESS_TYPE ep_to_d3d12_store_op(EPAttachmentStoreOp op);
D3D12_RESOURCE_DIMENSION ep_to_d3d12_dimension(EPTextureDimension dim);
D3D12_SHADER_VISIBILITY ep_to_d3d12_visibility(EPShaderStageFlags stages);

// ============================================================================
// Internal Structures
// ============================================================================

struct EPPlatform {
    int placeholder;
};

struct EPInstance {
    EPBackendFlags enabled_backends;
    bool validation_enabled;
    bool debug_names_enabled;
    ComPtr<IDXGIFactory6> factory;
    ComPtr<ID3D12Debug3> debug;
    std::vector<ComPtr<IDXGIAdapter4>> adapters;
};

struct EPAdapter {
    ComPtr<IDXGIAdapter4> adapter;
    DXGI_ADAPTER_DESC3 desc;
    EPAdapterProperties properties;
};

struct EPDevice {
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
    
    EPFeatureFlags features;
    bool validation_enabled;
    bool debug_names_enabled;
};

struct EPQueue {
    EPDevice *ep_device;
    ComPtr<ID3D12CommandQueue> queue;
    EPQueueType type;
};

struct EPCommandBuffer {
    EPDevice *ep_device;
    ComPtr<ID3D12CommandAllocator> allocator;
    ComPtr<ID3D12GraphicsCommandList7> list;
    bool is_recording;
    bool in_render_pass;
    
    // Current state
    ID3D12RootSignature *current_root_signature;
    D3D12_PRIMITIVE_TOPOLOGY current_topology;
};

struct EPFence {
    EPDevice *ep_device;
    ComPtr<ID3D12Fence1> fence;
    std::atomic<uint64_t> value{0};
    HANDLE event;
    std::mutex mutex;
};

struct EPTimelineSemaphore {
    EPDevice *ep_device;
    ComPtr<ID3D12Fence1> fence;
};

struct EPBuffer {
    EPDevice *ep_device;
    ComPtr<ID3D12Resource> resource;
    D3D12_GPU_VIRTUAL_ADDRESS gpu_address;
    void *mapped_ptr;
    uint64_t size;
    EPBufferUsageFlags usage;
    bool host_visible;
};

struct EPTexture {
    EPDevice *ep_device;
    ComPtr<ID3D12Resource> resource;
    D3D12_CPU_DESCRIPTOR_HANDLE rtv;
    D3D12_CPU_DESCRIPTOR_HANDLE dsv;
    D3D12_CPU_DESCRIPTOR_HANDLE srv;
    D3D12_CPU_DESCRIPTOR_HANDLE uav;
    EPTextureDesc desc;
    D3D12_RESOURCE_STATES current_state;
};

struct EPSampler {
    EPDevice *ep_device;
    D3D12_CPU_DESCRIPTOR_HANDLE sampler;
    D3D12_SAMPLER_DESC desc;
};

struct EPShaderLibrary {
    EPDevice *ep_device;
    ComPtr<IDxcBlob> blob;
    std::vector<uint8_t> bytecode;
};

struct EPDescriptorBindingInfo {
    uint32_t binding;
    EPDescriptorType type;
    uint32_t count;
    EPShaderStageFlags stages;
};

struct EPDescriptorSetLayout {
    EPDevice *ep_device;
    std::vector<EPDescriptorBindingInfo> bindings;
};

union EPDescriptorValue {
    struct {
        ID3D12Resource *buffer;
        uint64_t offset;
        uint64_t range;
    } buffer_info;
    D3D12_CPU_DESCRIPTOR_HANDLE texture;
    D3D12_CPU_DESCRIPTOR_HANDLE sampler;
    D3D12_GPU_VIRTUAL_ADDRESS accel;
};

struct EPDescriptorSetEntry {
    EPDescriptorType type;
    EPDescriptorValue value;
};

struct EPDescriptorSet {
    EPDevice *ep_device;
    EPDescriptorSetLayout *layout;
    std::vector<EPDescriptorSetEntry> entries;
};

struct EPPipelineLayout {
    EPDevice *ep_device;
    ComPtr<ID3D12RootSignature> root_signature;
    std::vector<EPDescriptorSetLayout *> set_layouts;
    uint32_t push_constant_size;
    EPShaderStageFlags push_constant_stages;
};

struct EPRenderPipeline {
    EPDevice *ep_device;
    ComPtr<ID3D12PipelineState> state;
    ComPtr<ID3D12RootSignature> root_signature;
    D3D12_PRIMITIVE_TOPOLOGY topology;
};

struct EPComputePipeline {
    EPDevice *ep_device;
    ComPtr<ID3D12PipelineState> state;
    ComPtr<ID3D12RootSignature> root_signature;
};

struct EPMeshPipeline {
    EPDevice *ep_device;
    ComPtr<ID3D12PipelineState> state;
    ComPtr<ID3D12RootSignature> root_signature;
};

struct EPRayTracingPipeline {
    EPDevice *ep_device;
    ComPtr<ID3D12StateObject> state;
    ComPtr<ID3D12RootSignature> root_signature;
    ComPtr<ID3D12Resource> shader_table;
    D3D12_GPU_VIRTUAL_ADDRESS raygen_record;
    D3D12_GPU_VIRTUAL_ADDRESS miss_record;
    D3D12_GPU_VIRTUAL_ADDRESS hitgroup_record;
    uint32_t max_recursion_depth;
};

struct EPAccelerationStructure {
    EPDevice *ep_device;
    ComPtr<ID3D12Resource> result;
    ComPtr<ID3D12Resource> scratch;
    D3D12_GPU_VIRTUAL_ADDRESS gpu_address;
    bool top_level;
};

struct EPSurface {
    HWND hwnd;
    EPNativeHandleType handle_type;
};

struct EPSwapchain {
    EPDevice *ep_device;
    EPSurface *surface;
    ComPtr<IDXGISwapChain4> swapchain;
    std::vector<ComPtr<ID3D12Resource>> buffers;
    std::vector<D3D12_CPU_DESCRIPTOR_HANDLE> rtvs;
    uint32_t image_count;
    uint32_t current_index;
    EPTextureFormat format;
    uint32_t width;
    uint32_t height;
};

// ============================================================================
// DirectStorage helpers
// ============================================================================

struct EPDirectStorageRequest {
    ComPtr<IDStorageFile> file;
    EPBuffer *destination;
    uint64_t file_offset;
    uint64_t size;
};

#endif // ENTHRALL_D3D12_INTERNAL_H
