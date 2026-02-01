// enthrall_surface.cpp - Surface and swapchain implementation

#include "enthrall_internal.h"

#ifdef __APPLE__
#include <vulkan/vulkan_metal.h>
#endif

using namespace ep::vk;

// ============================================================================
// Surface
// ============================================================================

extern "C" EPError EPSurfaceCreate(EPPlatformPtr platform_ptr, const EPSurfaceDesc* desc,
                                    EPSurfacePtr* out_surface) {
    if (!desc || !out_surface) return invalid_argument("desc or out_surface is NULL");

    // We need an instance to create a surface, but the C API only gives us platform
    // For now, this is a limitation - in practice, surface creation should be tied to instance
    return unsupported("surface creation requires instance - use device-based surface creation");
}

extern "C" EPError EPSurfaceDestroy(EPSurfacePtr surface) {
    delete reinterpret_cast<Surface*>(surface);
    return ok();
}

// ============================================================================
// Swapchain
// ============================================================================

extern "C" EPError EPSwapchainCreate(EPDevicePtr device_ptr, const EPSwapchainDesc* desc,
                                      EPSwapchainPtr* out_swapchain) {
    if (!device_ptr || !desc || !out_swapchain) return invalid_argument("device, desc, or out_swapchain is NULL");
    if (!desc->surface) return invalid_argument("surface is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);
    auto* surface = reinterpret_cast<Surface*>(desc->surface);

    auto swapchain = std::make_unique<Swapchain>();
    swapchain->device = device->shared_from_this();
    // Surface doesn't need shared ownership, just raw pointer in shared_ptr wrapper
    // The user is responsible for keeping Surface alive while Swapchain exists
    swapchain->surface = std::shared_ptr<Surface>(surface, [](Surface*){});
    swapchain->width = desc->width;
    swapchain->height = desc->height;
    swapchain->format = desc->format;

    // Query surface capabilities
    auto [cap_result, capabilities] = device->adapter->physical_device.getSurfaceCapabilitiesKHR(surface->surface.get());
    if (cap_result != ::vk::Result::eSuccess) {
        return from_vk_result(cap_result);
    }

    uint32_t image_count = desc->image_count;
    if (image_count < capabilities.minImageCount) {
        image_count = capabilities.minImageCount;
    }
    if (capabilities.maxImageCount > 0 && image_count > capabilities.maxImageCount) {
        image_count = capabilities.maxImageCount;
    }

    // Choose present mode
    ::vk::PresentModeKHR present_mode = ::vk::PresentModeKHR::eFifo; // Always available, vsync
    if (!desc->vsync) {
        auto [mode_result, present_modes] = device->adapter->physical_device.getSurfacePresentModesKHR(surface->surface.get());
        if (mode_result == ::vk::Result::eSuccess) {
            for (auto mode : present_modes) {
                if (mode == ::vk::PresentModeKHR::eMailbox) {
                    present_mode = mode;
                    break;
                }
                if (mode == ::vk::PresentModeKHR::eImmediate) {
                    present_mode = mode;
                }
            }
        }
    }

    auto [result, vk_swapchain] = device->device->createSwapchainKHRUnique(::vk::SwapchainCreateInfoKHR{
        .surface = surface->surface.get(),
        .minImageCount = image_count,
        .imageFormat = to_vk_format(desc->format),
        .imageColorSpace = ::vk::ColorSpaceKHR::eSrgbNonlinear,
        .imageExtent = {desc->width, desc->height},
        .imageArrayLayers = 1,
        .imageUsage = ::vk::ImageUsageFlagBits::eColorAttachment | ::vk::ImageUsageFlagBits::eTransferDst,
        .imageSharingMode = ::vk::SharingMode::eExclusive,
        .preTransform = capabilities.currentTransform,
        .compositeAlpha = ::vk::CompositeAlphaFlagBitsKHR::eOpaque,
        .presentMode = present_mode,
        .clipped = VK_TRUE,
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    swapchain->swapchain = std::move(vk_swapchain);

    // Get swapchain images
    auto [img_result, images] = device->device->getSwapchainImagesKHR(swapchain->swapchain.get());
    if (img_result != ::vk::Result::eSuccess) {
        return from_vk_result(img_result);
    }
    swapchain->images = std::move(images);

    // Create image views
    for (auto image : swapchain->images) {
        auto [view_result, view] = device->device->createImageViewUnique(::vk::ImageViewCreateInfo{
            .image = image,
            .viewType = ::vk::ImageViewType::e2D,
            .format = to_vk_format(desc->format),
            .subresourceRange = ::vk::ImageSubresourceRange{
                .aspectMask = ::vk::ImageAspectFlagBits::eColor,
                .baseMipLevel = 0,
                .levelCount = 1,
                .baseArrayLayer = 0,
                .layerCount = 1,
            },
        });

        if (view_result != ::vk::Result::eSuccess) {
            return from_vk_result(view_result);
        }

        swapchain->views.push_back(std::move(view));
    }

    *out_swapchain = reinterpret_cast<EPSwapchainPtr>(swapchain.release());
    return ok();
}

extern "C" EPError EPSwapchainDestroy(EPSwapchainPtr swapchain) {
    delete reinterpret_cast<Swapchain*>(swapchain);
    return ok();
}

extern "C" EPError EPSwapchainAcquireNext(EPSwapchainPtr swapchain_ptr,
                                          EPTimelineSemaphorePtr signal_semaphore_ptr,
                                          uint64_t signal_value,
                                          uint32_t* out_image_index) {
    if (!swapchain_ptr || !out_image_index) return invalid_argument("swapchain or out_image_index is NULL");

    auto* swapchain = reinterpret_cast<Swapchain*>(swapchain_ptr);

    // For timeline semaphores with swapchain, we need a binary semaphore bridge
    // This is a simplified implementation - production code would use proper sync
    auto [result, index] = swapchain->device->device->acquireNextImageKHR(
        swapchain->swapchain.get(),
        UINT64_MAX,
        nullptr,  // No binary semaphore
        nullptr   // No fence
    );

    if (result == ::vk::Result::eErrorOutOfDateKHR || result == ::vk::Result::eSuboptimalKHR) {
        return error(EP_E_INVALID_STATE, "swapchain out of date");
    }
    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    swapchain->current_index = index;
    *out_image_index = index;

    // Signal the timeline semaphore from CPU (simplified)
    if (signal_semaphore_ptr) {
        auto* semaphore = reinterpret_cast<TimelineSemaphore*>(signal_semaphore_ptr);
        auto signal_result = swapchain->device->device->signalSemaphore(::vk::SemaphoreSignalInfo{
            .semaphore = semaphore->semaphore.get(),
            .value = signal_value,
        });
        if (signal_result != ::vk::Result::eSuccess) {
            return from_vk_result(signal_result);
        }
    }

    return ok();
}

extern "C" EPError EPSwapchainPresent(EPSwapchainPtr swapchain_ptr,
                                       EPTimelineSemaphorePtr wait_semaphore_ptr,
                                       uint64_t wait_value) {
    if (!swapchain_ptr) return invalid_argument("swapchain is NULL");

    auto* swapchain = reinterpret_cast<Swapchain*>(swapchain_ptr);

    // Wait on timeline semaphore from CPU (simplified)
    if (wait_semaphore_ptr) {
        auto* semaphore = reinterpret_cast<TimelineSemaphore*>(wait_semaphore_ptr);

        ::vk::Semaphore semaphores[] = {semaphore->semaphore.get()};
        uint64_t values[] = {wait_value};

        ::vk::SemaphoreWaitInfo wait_info{
            .semaphoreCount = 1,
            .pSemaphores = semaphores,
            .pValues = values,
        };

        auto wait_result = swapchain->device->device->waitSemaphores(wait_info, UINT64_MAX);
        if (wait_result != ::vk::Result::eSuccess && wait_result != ::vk::Result::eTimeout) {
            return from_vk_result(wait_result);
        }
    }

    ::vk::SwapchainKHR swapchains[] = {swapchain->swapchain.get()};
    uint32_t indices[] = {swapchain->current_index};

    auto result = swapchain->device->graphics_queue.presentKHR(::vk::PresentInfoKHR{
        .swapchainCount = 1,
        .pSwapchains = swapchains,
        .pImageIndices = indices,
    });

    if (result == ::vk::Result::eErrorOutOfDateKHR || result == ::vk::Result::eSuboptimalKHR) {
        return error(EP_E_INVALID_STATE, "swapchain out of date");
    }
    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    return ok();
}
