// enthrall_sync.cpp - Fence and timeline semaphore implementation

#include "enthrall_internal.h"

// ============================================================================
// Fence
// ============================================================================

extern "C" EPError EPFenceCreate(EPDevicePtr device, uint64_t initial_value,
                                 EPFencePtr *out_fence) {
    if (!device || !out_fence) return ep_invalid_argument("device or out_fence is NULL");
    
    EPFence *fence = new (std::nothrow) EPFence{};
    if (!fence) return ep_out_of_memory();
    
    fence->ep_device = device;
    fence->value.store(initial_value);
    
    HRESULT hr = device->device->CreateFence(initial_value, D3D12_FENCE_FLAG_NONE,
                                              IID_PPV_ARGS(&fence->fence));
    if (FAILED(hr)) {
        delete fence;
        return ep_from_hresult(hr, "failed to create fence");
    }
    
    fence->event = CreateEventW(nullptr, FALSE, FALSE, nullptr);
    if (!fence->event) {
        delete fence;
        return ep_error(EP_E_INVALID_STATE, "failed to create event");
    }
    
    *out_fence = fence;
    return ep_ok();
}

extern "C" EPError EPFenceDestroy(EPFencePtr fence) {
    if (fence) {
        if (fence->event) {
            CloseHandle(fence->event);
        }
    }
    delete fence;
    return ep_ok();
}

extern "C" EPError EPFenceWait(EPFencePtr fence, uint64_t value, uint64_t timeout_ns) {
    if (!fence) return ep_invalid_argument("fence is NULL");
    
    if (fence->fence->GetCompletedValue() >= value) {
        return ep_ok();
    }
    
    HRESULT hr = fence->fence->SetEventOnCompletion(value, fence->event);
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to set event on completion");
    
    DWORD timeout_ms = (timeout_ns == UINT64_MAX) 
        ? INFINITE 
        : static_cast<DWORD>(timeout_ns / 1000000);
    
    DWORD result = WaitForSingleObject(fence->event, timeout_ms);
    
    if (result == WAIT_TIMEOUT) {
        return ep_error(EP_E_INVALID_STATE, "fence wait timed out");
    } else if (result != WAIT_OBJECT_0) {
        return ep_error(EP_E_INVALID_STATE, "fence wait failed");
    }
    
    return ep_ok();
}

extern "C" EPError EPFenceSignal(EPFencePtr fence, uint64_t value) {
    if (!fence) return ep_invalid_argument("fence is NULL");
    
    std::lock_guard<std::mutex> lock(fence->mutex);
    fence->value.store(value);
    
    HRESULT hr = fence->fence->Signal(value);
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to signal fence");
    
    return ep_ok();
}

// ============================================================================
// Timeline Semaphore
// ============================================================================

extern "C" EPError EPTimelineSemaphoreCreate(EPDevicePtr device, uint64_t initial_value,
                                             EPTimelineSemaphorePtr *out_semaphore) {
    if (!device || !out_semaphore) 
        return ep_invalid_argument("device or out_semaphore is NULL");
    
    EPTimelineSemaphore *semaphore = new (std::nothrow) EPTimelineSemaphore{};
    if (!semaphore) return ep_out_of_memory();
    
    semaphore->ep_device = device;
    
    HRESULT hr = device->device->CreateFence(initial_value, D3D12_FENCE_FLAG_NONE,
                                              IID_PPV_ARGS(&semaphore->fence));
    if (FAILED(hr)) {
        delete semaphore;
        return ep_from_hresult(hr, "failed to create timeline semaphore");
    }
    
    *out_semaphore = semaphore;
    return ep_ok();
}

extern "C" EPError EPTimelineSemaphoreDestroy(EPTimelineSemaphorePtr semaphore) {
    delete semaphore;
    return ep_ok();
}

extern "C" EPError EPTimelineSemaphoreGetValue(EPTimelineSemaphorePtr semaphore,
                                               uint64_t *out_value) {
    if (!semaphore || !out_value) 
        return ep_invalid_argument("semaphore or out_value is NULL");
    
    *out_value = semaphore->fence->GetCompletedValue();
    return ep_ok();
}

extern "C" EPError EPTimelineSemaphoreWait(EPTimelineSemaphorePtr semaphore,
                                           uint64_t value, uint64_t timeout_ns) {
    if (!semaphore) return ep_invalid_argument("semaphore is NULL");
    
    if (semaphore->fence->GetCompletedValue() >= value) {
        return ep_ok();
    }
    
    HANDLE event = CreateEventW(nullptr, FALSE, FALSE, nullptr);
    if (!event) return ep_error(EP_E_INVALID_STATE, "failed to create event");
    
    HRESULT hr = semaphore->fence->SetEventOnCompletion(value, event);
    if (FAILED(hr)) {
        CloseHandle(event);
        return ep_from_hresult(hr, "failed to set event on completion");
    }
    
    DWORD timeout_ms = (timeout_ns == UINT64_MAX) 
        ? INFINITE 
        : static_cast<DWORD>(timeout_ns / 1000000);
    
    DWORD result = WaitForSingleObject(event, timeout_ms);
    CloseHandle(event);
    
    if (result == WAIT_TIMEOUT) {
        return ep_error(EP_E_INVALID_STATE, "semaphore wait timed out");
    } else if (result != WAIT_OBJECT_0) {
        return ep_error(EP_E_INVALID_STATE, "semaphore wait failed");
    }
    
    return ep_ok();
}

extern "C" EPError EPTimelineSemaphoreSignal(EPTimelineSemaphorePtr semaphore,
                                             uint64_t value) {
    if (!semaphore) return ep_invalid_argument("semaphore is NULL");
    
    HRESULT hr = semaphore->fence->Signal(value);
    if (FAILED(hr)) return ep_from_hresult(hr, "failed to signal semaphore");
    
    return ep_ok();
}

// ============================================================================
// Surface and Swapchain
// ============================================================================

extern "C" EPError EPSurfaceCreate(EPPlatformPtr platform, const EPSurfaceDesc *desc,
                                   EPSurfacePtr *out_surface) {
    if (!desc || !out_surface) return ep_invalid_argument("desc or out_surface is NULL");
    if (desc->handle_type != EP_NATIVE_HANDLE_HWND) 
        return ep_unsupported("only HWND surfaces supported on D3D12");
    
    EPSurface *surface = new (std::nothrow) EPSurface{};
    if (!surface) return ep_out_of_memory();
    
    surface->hwnd = static_cast<HWND>(desc->handle);
    surface->handle_type = desc->handle_type;
    
    *out_surface = surface;
    return ep_ok();
}

extern "C" EPError EPSurfaceDestroy(EPSurfacePtr surface) {
    delete surface;
    return ep_ok();
}

extern "C" EPError EPSwapchainCreate(EPDevicePtr device, const EPSwapchainDesc *desc,
                                     EPSwapchainPtr *out_swapchain) {
    if (!device || !desc || !out_swapchain)
        return ep_invalid_argument("device, desc, or out_swapchain is NULL");
    if (!desc->surface)
        return ep_invalid_argument("surface is NULL");
    
    EPSwapchain *swapchain = new (std::nothrow) EPSwapchain{};
    if (!swapchain) return ep_out_of_memory();
    
    swapchain->ep_device = device;
    swapchain->surface = desc->surface;
    swapchain->image_count = desc->image_count;
    swapchain->format = desc->format;
    swapchain->width = desc->width;
    swapchain->height = desc->height;
    swapchain->current_index = 0;
    
    // Get DXGI factory from device's adapter
    ComPtr<IDXGIFactory6> factory;
    HRESULT hr = CreateDXGIFactory2(0, IID_PPV_ARGS(&factory));
    if (FAILED(hr)) {
        delete swapchain;
        return ep_from_hresult(hr, "failed to create DXGI factory");
    }
    
    DXGI_SWAP_CHAIN_DESC1 swapchain_desc = {};
    swapchain_desc.Width = desc->width;
    swapchain_desc.Height = desc->height;
    swapchain_desc.Format = ep_to_dxgi_format(desc->format);
    swapchain_desc.Stereo = FALSE;
    swapchain_desc.SampleDesc.Count = 1;
    swapchain_desc.SampleDesc.Quality = 0;
    swapchain_desc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    swapchain_desc.BufferCount = desc->image_count;
    swapchain_desc.Scaling = DXGI_SCALING_STRETCH;
    swapchain_desc.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;
    swapchain_desc.AlphaMode = DXGI_ALPHA_MODE_UNSPECIFIED;
    swapchain_desc.Flags = desc->vsync ? 0 : DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING;
    
    ComPtr<IDXGISwapChain1> swapchain1;
    hr = factory->CreateSwapChainForHwnd(
        device->graphics_queue.Get(),
        desc->surface->hwnd,
        &swapchain_desc,
        nullptr,
        nullptr,
        &swapchain1);
    
    if (FAILED(hr)) {
        delete swapchain;
        return ep_from_hresult(hr, "failed to create swapchain");
    }
    
    hr = swapchain1.As(&swapchain->swapchain);
    if (FAILED(hr)) {
        delete swapchain;
        return ep_from_hresult(hr, "failed to query swapchain interface");
    }
    
    // Disable Alt+Enter fullscreen toggle
    factory->MakeWindowAssociation(desc->surface->hwnd, DXGI_MWA_NO_ALT_ENTER);
    
    // Get back buffers and create RTVs
    swapchain->buffers.resize(desc->image_count);
    swapchain->rtvs.resize(desc->image_count);
    
    for (uint32_t i = 0; i < desc->image_count; i++) {
        hr = swapchain->swapchain->GetBuffer(i, IID_PPV_ARGS(&swapchain->buffers[i]));
        if (FAILED(hr)) {
            delete swapchain;
            return ep_from_hresult(hr, "failed to get swapchain buffer");
        }
        
        UINT rtv_offset = device->rtv_heap_offset.fetch_add(1);
        swapchain->rtvs[i] = device->rtv_heap->GetCPUDescriptorHandleForHeapStart();
        swapchain->rtvs[i].ptr += rtv_offset * device->rtv_descriptor_size;
        
        device->device->CreateRenderTargetView(swapchain->buffers[i].Get(), nullptr,
                                                swapchain->rtvs[i]);
    }
    
    *out_swapchain = swapchain;
    return ep_ok();
}

extern "C" EPError EPSwapchainDestroy(EPSwapchainPtr swapchain) {
    delete swapchain;
    return ep_ok();
}

extern "C" EPError EPSwapchainAcquireNext(EPSwapchainPtr swapchain,
                                          EPTimelineSemaphorePtr signal_semaphore,
                                          uint64_t signal_value,
                                          uint32_t *out_image_index) {
    if (!swapchain || !out_image_index)
        return ep_invalid_argument("swapchain or out_image_index is NULL");
    
    swapchain->current_index = swapchain->swapchain->GetCurrentBackBufferIndex();
    *out_image_index = swapchain->current_index;
    
    // Signal semaphore immediately since D3D12 doesn't have acquire semaphores
    if (signal_semaphore) {
        swapchain->ep_device->graphics_queue->Signal(signal_semaphore->fence.Get(),
                                                      signal_value);
    }
    
    return ep_ok();
}

extern "C" EPError EPSwapchainPresent(EPSwapchainPtr swapchain,
                                      EPTimelineSemaphorePtr wait_semaphore,
                                      uint64_t wait_value) {
    if (!swapchain) return ep_invalid_argument("swapchain is NULL");
    
    // Wait on semaphore before present
    if (wait_semaphore) {
        swapchain->ep_device->graphics_queue->Wait(wait_semaphore->fence.Get(), wait_value);
    }
    
    UINT sync_interval = 1; // VSync on
    UINT present_flags = 0;
    
    HRESULT hr = swapchain->swapchain->Present(sync_interval, present_flags);
    if (FAILED(hr)) return ep_from_hresult(hr, "present failed");
    
    return ep_ok();
}
