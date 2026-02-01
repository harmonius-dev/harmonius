// enthrall_device.cpp - Device and queue implementation

#include "enthrall_internal.h"

// ============================================================================
// Device
// ============================================================================

static EPError create_descriptor_heaps(EPDevice *device) {
    // RTV heap (for render targets)
    D3D12_DESCRIPTOR_HEAP_DESC rtv_heap_desc = {};
    rtv_heap_desc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_RTV;
    rtv_heap_desc.NumDescriptors = 1024;
    rtv_heap_desc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_NONE;
    
    HRESULT hr = device->device->CreateDescriptorHeap(&rtv_heap_desc,
                                                       IID_PPV_ARGS(&device->rtv_heap));
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to create RTV heap");
    
    // DSV heap (for depth/stencil)
    D3D12_DESCRIPTOR_HEAP_DESC dsv_heap_desc = {};
    dsv_heap_desc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_DSV;
    dsv_heap_desc.NumDescriptors = 256;
    dsv_heap_desc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_NONE;
    
    hr = device->device->CreateDescriptorHeap(&dsv_heap_desc,
                                               IID_PPV_ARGS(&device->dsv_heap));
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to create DSV heap");
    
    // CBV/SRV/UAV heap (shader visible)
    D3D12_DESCRIPTOR_HEAP_DESC cbv_srv_uav_heap_desc = {};
    cbv_srv_uav_heap_desc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV;
    cbv_srv_uav_heap_desc.NumDescriptors = 1000000; // Bindless resources
    cbv_srv_uav_heap_desc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE;
    
    hr = device->device->CreateDescriptorHeap(&cbv_srv_uav_heap_desc,
                                               IID_PPV_ARGS(&device->cbv_srv_uav_heap));
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to create CBV/SRV/UAV heap");
    
    // Sampler heap (shader visible)
    D3D12_DESCRIPTOR_HEAP_DESC sampler_heap_desc = {};
    sampler_heap_desc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER;
    sampler_heap_desc.NumDescriptors = 2048;
    sampler_heap_desc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE;
    
    hr = device->device->CreateDescriptorHeap(&sampler_heap_desc,
                                               IID_PPV_ARGS(&device->sampler_heap));
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to create sampler heap");
    
    // Cache descriptor sizes
    device->rtv_descriptor_size = device->device->GetDescriptorHandleIncrementSize(
        D3D12_DESCRIPTOR_HEAP_TYPE_RTV);
    device->dsv_descriptor_size = device->device->GetDescriptorHandleIncrementSize(
        D3D12_DESCRIPTOR_HEAP_TYPE_DSV);
    device->cbv_srv_uav_descriptor_size = device->device->GetDescriptorHandleIncrementSize(
        D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV);
    device->sampler_descriptor_size = device->device->GetDescriptorHandleIncrementSize(
        D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER);
    
    return ep_ok();
}

extern "C" EPError EPDeviceCreate(EPAdapterPtr adapter, const EPDeviceDesc *desc,
                                  EPDevicePtr *out_device) {
    if (!adapter || !desc || !out_device) 
        return ep_invalid_argument("adapter, desc, or out_device is NULL");
    
    // Check required features
    if ((desc->required_features & adapter->properties.features) != desc->required_features) {
        return ep_unsupported("required features not available");
    }
    
    EPDevice *device = new (std::nothrow) EPDevice{};
    if (!device) return ep_out_of_memory();
    
    device->features = adapter->properties.features & 
                      (desc->required_features | desc->optional_features);
    device->validation_enabled = desc->enable_validation;
    device->debug_names_enabled = desc->enable_debug_names;
    
    // Create device
    HRESULT hr = D3D12CreateDevice(adapter->adapter.Get(), D3D_FEATURE_LEVEL_12_0,
                                   IID_PPV_ARGS(&device->device));
    if (FAILED(hr)) {
        delete device;
        return ep_from_hresult(hr, "failed to create D3D12 device");
    }
    
    if (desc->enable_debug_names) {
        device->device->SetName(L"Enthrall Device");
    }
    
    // Create command queues
    D3D12_COMMAND_QUEUE_DESC graphics_queue_desc = {};
    graphics_queue_desc.Type = D3D12_COMMAND_LIST_TYPE_DIRECT;
    graphics_queue_desc.Priority = D3D12_COMMAND_QUEUE_PRIORITY_NORMAL;
    graphics_queue_desc.Flags = D3D12_COMMAND_QUEUE_FLAG_NONE;
    
    hr = device->device->CreateCommandQueue(&graphics_queue_desc,
                                            IID_PPV_ARGS(&device->graphics_queue));
    if (FAILED(hr)) {
        delete device;
        return ep_from_hresult(hr, "failed to create graphics queue");
    }
    
    D3D12_COMMAND_QUEUE_DESC compute_queue_desc = {};
    compute_queue_desc.Type = D3D12_COMMAND_LIST_TYPE_COMPUTE;
    compute_queue_desc.Priority = D3D12_COMMAND_QUEUE_PRIORITY_NORMAL;
    
    hr = device->device->CreateCommandQueue(&compute_queue_desc,
                                            IID_PPV_ARGS(&device->compute_queue));
    if (FAILED(hr)) {
        delete device;
        return ep_from_hresult(hr, "failed to create compute queue");
    }
    
    D3D12_COMMAND_QUEUE_DESC copy_queue_desc = {};
    copy_queue_desc.Type = D3D12_COMMAND_LIST_TYPE_COPY;
    copy_queue_desc.Priority = D3D12_COMMAND_QUEUE_PRIORITY_NORMAL;
    
    hr = device->device->CreateCommandQueue(&copy_queue_desc,
                                            IID_PPV_ARGS(&device->copy_queue));
    if (FAILED(hr)) {
        delete device;
        return ep_from_hresult(hr, "failed to create copy queue");
    }
    
    if (desc->enable_debug_names) {
        device->graphics_queue->SetName(L"Enthrall Graphics Queue");
        device->compute_queue->SetName(L"Enthrall Compute Queue");
        device->copy_queue->SetName(L"Enthrall Copy Queue");
    }
    
    // Create descriptor heaps
    EPError err = create_descriptor_heaps(device);
    if (err.code != EP_E_OK) {
        delete device;
        return err;
    }
    
    // Initialize DirectStorage
    hr = DStorageGetFactory(IID_PPV_ARGS(&device->dstorage_factory));
    if (SUCCEEDED(hr) && device->dstorage_factory) {
        DSTORAGE_QUEUE_DESC dstorage_queue_desc = {};
        dstorage_queue_desc.Capacity = DSTORAGE_MAX_QUEUE_CAPACITY;
        dstorage_queue_desc.Priority = DSTORAGE_PRIORITY_NORMAL;
        dstorage_queue_desc.SourceType = DSTORAGE_REQUEST_SOURCE_FILE;
        dstorage_queue_desc.Device = device->device.Get();
        
        hr = device->dstorage_factory->CreateQueue(&dstorage_queue_desc,
                                                    IID_PPV_ARGS(&device->dstorage_queue));
        if (FAILED(hr)) {
            // DirectStorage is optional, continue without it
            device->dstorage_factory.Reset();
        }
    }
    
    *out_device = device;
    return ep_ok();
}

extern "C" EPError EPDeviceDestroy(EPDevicePtr device) {
    delete device;
    return ep_ok();
}

extern "C" EPError EPDeviceGetQueue(EPDevicePtr device, EPQueueType type, uint32_t index,
                                    EPQueuePtr *out_queue) {
    if (!device || !out_queue) return ep_invalid_argument("device or out_queue is NULL");
    if (index > 0) return ep_invalid_argument("only queue index 0 is supported");
    
    EPQueue *queue = new (std::nothrow) EPQueue{};
    if (!queue) return ep_out_of_memory();
    
    queue->ep_device = device;
    queue->type = type;
    
    switch (type) {
        case EP_QUEUE_GRAPHICS:
            queue->queue = device->graphics_queue;
            break;
        case EP_QUEUE_COMPUTE:
            queue->queue = device->compute_queue;
            break;
        case EP_QUEUE_TRANSFER:
            queue->queue = device->copy_queue;
            break;
        default:
            delete queue;
            return ep_invalid_argument("invalid queue type");
    }
    
    *out_queue = queue;
    return ep_ok();
}

// ============================================================================
// Queue
// ============================================================================

extern "C" EPError EPQueueSubmit(EPQueuePtr queue, const EPSubmitInfo *submit) {
    if (!queue || !submit) return ep_invalid_argument("queue or submit is NULL");
    
    // Wait on semaphores
    for (uint32_t w = 0; w < submit->wait_count; w++) {
        EPTimelineSemaphore *sem = submit->wait_semaphores[w];
        if (sem && sem->fence) {
            queue->queue->Wait(sem->fence.Get(), submit->wait_values[w]);
        }
    }
    
    // Execute command lists
    std::vector<ID3D12CommandList *> cmd_lists;
    cmd_lists.reserve(submit->command_buffer_count);
    
    for (uint32_t i = 0; i < submit->command_buffer_count; i++) {
        EPCommandBuffer *cmd = submit->command_buffers[i];
        if (cmd && cmd->list) {
            cmd_lists.push_back(cmd->list.Get());
        }
    }
    
    if (!cmd_lists.empty()) {
        queue->queue->ExecuteCommandLists(
            static_cast<UINT>(cmd_lists.size()),
            cmd_lists.data());
    }
    
    // Signal semaphores
    for (uint32_t s = 0; s < submit->signal_count; s++) {
        EPTimelineSemaphore *sem = submit->signal_semaphores[s];
        if (sem && sem->fence) {
            queue->queue->Signal(sem->fence.Get(), submit->signal_values[s]);
        }
    }
    
    // Signal fence
    if (submit->fence && submit->fence->fence) {
        uint64_t new_value = submit->fence->value.fetch_add(1) + 1;
        queue->queue->Signal(submit->fence->fence.Get(), new_value);
    }
    
    return ep_ok();
}

extern "C" EPError EPQueueWaitIdle(EPQueuePtr queue) {
    if (!queue) return ep_invalid_argument("queue is NULL");
    
    EPDevice *device = queue->ep_device;
    
    // Create a temporary fence for synchronization
    ComPtr<ID3D12Fence1> fence;
    HRESULT hr = device->device->CreateFence(0, D3D12_FENCE_FLAG_NONE,
                                              IID_PPV_ARGS(&fence));
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to create fence for wait");
    
    hr = queue->queue->Signal(fence.Get(), 1);
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to signal fence");
    
    HANDLE event = CreateEventW(nullptr, FALSE, FALSE, nullptr);
    if (!event) return ep_error(EP_E_INVALID_STATE, "failed to create event");
    
    if (fence->GetCompletedValue() < 1) {
        hr = fence->SetEventOnCompletion(1, event);
        if (SUCCEEDED(hr)) {
            WaitForSingleObject(event, INFINITE);
        }
    }
    
    CloseHandle(event);
    return ep_ok();
}
