// gcraft_device.cpp - Device and queue implementation

#include "gcraft_internal.h"

using namespace ep::vk;

// ============================================================================
// Device
// ============================================================================

extern "C" GCError GCDeviceCreate(GCAdapterPtr adapter_ptr, const GCDeviceDesc* desc,
                                   GCDevicePtr* out_device) {
    if (!adapter_ptr || !desc || !out_device) return invalid_argument("adapter, desc, or out_device is NULL");

    auto* adapter = reinterpret_cast<Adapter*>(adapter_ptr);

    // Check required features
    if ((desc->required_features & adapter->ep_properties.features) != desc->required_features) {
        return unsupported("required features not available");
    }

    auto device = std::make_shared<Device>();
    device->prevent_destroy = device;  // Keep alive until GCDeviceDestroy
    device->features = static_cast<GCFeatureFlags>(
        static_cast<uint32_t>(adapter->ep_properties.features) &
        (static_cast<uint32_t>(desc->required_features) | static_cast<uint32_t>(desc->optional_features)));
    device->validation_enabled = desc->enable_validation;
    device->debug_names_enabled = desc->enable_debug_names;

    // Queue create infos
    float queue_priority = 1.0f;
    std::vector<::vk::DeviceQueueCreateInfo> queue_infos;

    // Track unique queue families
    std::vector<uint32_t> unique_families;
    if (adapter->graphics_queue_family != UINT32_MAX) unique_families.push_back(adapter->graphics_queue_family);
    if (adapter->compute_queue_family != UINT32_MAX &&
        adapter->compute_queue_family != adapter->graphics_queue_family) {
        unique_families.push_back(adapter->compute_queue_family);
    }
    if (adapter->transfer_queue_family != UINT32_MAX &&
        adapter->transfer_queue_family != adapter->graphics_queue_family &&
        adapter->transfer_queue_family != adapter->compute_queue_family) {
        unique_families.push_back(adapter->transfer_queue_family);
    }

    for (uint32_t family : unique_families) {
        queue_infos.push_back(::vk::DeviceQueueCreateInfo{
            .queueFamilyIndex = family,
            .queueCount = 1,
            .pQueuePriorities = &queue_priority,
        });
    }

    // Query available extensions
    auto [ext_result, available_extensions] = adapter->physical_device.enumerateDeviceExtensionProperties();
    
    auto has_extension = [&](const char* name) {
        for (const auto& ext : available_extensions) {
            if (std::strcmp(ext.extensionName.data(), name) == 0) return true;
        }
        return false;
    };

    // Extensions - only add if available
    std::vector<const char*> extensions;
    
    if (has_extension(VK_KHR_SWAPCHAIN_EXTENSION_NAME)) {
        extensions.push_back(VK_KHR_SWAPCHAIN_EXTENSION_NAME);
    }
#ifdef __APPLE__
    if (has_extension("VK_KHR_portability_subset")) {
        extensions.push_back("VK_KHR_portability_subset");
    }
#endif

    // Add optional extensions based on features (only if available)
    bool has_timeline = adapter->has_timeline_semaphore && has_extension(VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME);
    bool has_dynamic = adapter->has_dynamic_rendering && has_extension(VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME);
    bool has_sync2 = has_extension("VK_KHR_synchronization2");
    
    if (has_timeline) extensions.push_back(VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME);
    if (has_dynamic) extensions.push_back(VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME);
    if (has_sync2) extensions.push_back("VK_KHR_synchronization2");
    if (adapter->has_mesh_shader && has_extension(VK_EXT_MESH_SHADER_EXTENSION_NAME)) {
        extensions.push_back(VK_EXT_MESH_SHADER_EXTENSION_NAME);
    }
    if (adapter->has_ray_tracing) {
        if (has_extension(VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME) &&
            has_extension(VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME) &&
            has_extension(VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME) &&
            has_extension(VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME)) {
            extensions.push_back(VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME);
            extensions.push_back(VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME);
            extensions.push_back(VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME);
            extensions.push_back(VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME);
        }
    }

    // Enable features
    ::vk::PhysicalDeviceFeatures2 features2{
        .features = adapter->features,
    };

    ::vk::PhysicalDeviceTimelineSemaphoreFeatures timeline_features{
        .timelineSemaphore = has_timeline ? VK_TRUE : VK_FALSE,
    };

    ::vk::PhysicalDeviceDynamicRenderingFeatures dynamic_rendering_features{
        .dynamicRendering = has_dynamic ? VK_TRUE : VK_FALSE,
    };

    ::vk::PhysicalDeviceSynchronization2Features sync2_features{
        .synchronization2 = has_sync2 ? VK_TRUE : VK_FALSE,
    };

    ::vk::PhysicalDeviceMeshShaderFeaturesEXT mesh_features{
        .taskShader = adapter->has_mesh_shader,
        .meshShader = adapter->has_mesh_shader,
    };

    ::vk::PhysicalDeviceRayTracingPipelineFeaturesKHR rt_features{
        .rayTracingPipeline = adapter->has_ray_tracing,
    };

    ::vk::PhysicalDeviceAccelerationStructureFeaturesKHR accel_features{
        .accelerationStructure = adapter->has_ray_tracing,
    };

    ::vk::PhysicalDeviceBufferDeviceAddressFeatures bda_features{
        .bufferDeviceAddress = adapter->has_ray_tracing,
    };

    // Chain features
    void** next_chain = &features2.pNext;
    if (has_timeline) {
        *next_chain = &timeline_features;
        next_chain = &timeline_features.pNext;
    }
    if (has_dynamic) {
        *next_chain = &dynamic_rendering_features;
        next_chain = &dynamic_rendering_features.pNext;
    }
    if (has_sync2) {
        *next_chain = &sync2_features;
        next_chain = &sync2_features.pNext;
    }
    if (adapter->has_mesh_shader) {
        *next_chain = &mesh_features;
        next_chain = &mesh_features.pNext;
    }
    if (adapter->has_ray_tracing) {
        *next_chain = &rt_features;
        next_chain = &rt_features.pNext;
        *next_chain = &accel_features;
        next_chain = &accel_features.pNext;
        *next_chain = &bda_features;
        next_chain = &bda_features.pNext;
    }

    ::vk::DeviceCreateInfo create_info{
        .pNext = &features2,
        .queueCreateInfoCount = static_cast<uint32_t>(queue_infos.size()),
        .pQueueCreateInfos = queue_infos.data(),
        .enabledExtensionCount = static_cast<uint32_t>(extensions.size()),
        .ppEnabledExtensionNames = extensions.data(),
    };

    auto [result, vk_device] = adapter->physical_device.createDeviceUnique(create_info);
    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }
    device->device = std::move(vk_device);

    // Load device-level functions
    volkLoadDevice(device->device.get());
    VULKAN_HPP_DEFAULT_DISPATCHER.init(device->device.get());

    // Get queues
    device->graphics_queue = device->device->getQueue(adapter->graphics_queue_family, 0);
    device->compute_queue = device->device->getQueue(
        adapter->compute_queue_family != UINT32_MAX ? adapter->compute_queue_family : adapter->graphics_queue_family, 0);
    device->transfer_queue = device->device->getQueue(
        adapter->transfer_queue_family != UINT32_MAX ? adapter->transfer_queue_family : adapter->graphics_queue_family, 0);

    // Create command pools
    auto create_pool = [&](uint32_t family) -> ::vk::ResultValue<::vk::UniqueCommandPool> {
        return device->device->createCommandPoolUnique(::vk::CommandPoolCreateInfo{
            .flags = ::vk::CommandPoolCreateFlagBits::eResetCommandBuffer,
            .queueFamilyIndex = family,
        });
    };

    auto [gpool_result, gpool] = create_pool(adapter->graphics_queue_family);
    if (gpool_result != ::vk::Result::eSuccess) return from_vk_result(gpool_result);
    device->graphics_pool = std::move(gpool);

    auto [cpool_result, cpool] = create_pool(
        adapter->compute_queue_family != UINT32_MAX ? adapter->compute_queue_family : adapter->graphics_queue_family);
    if (cpool_result != ::vk::Result::eSuccess) return from_vk_result(cpool_result);
    device->compute_pool = std::move(cpool);

    auto [tpool_result, tpool] = create_pool(
        adapter->transfer_queue_family != UINT32_MAX ? adapter->transfer_queue_family : adapter->graphics_queue_family);
    if (tpool_result != ::vk::Result::eSuccess) return from_vk_result(tpool_result);
    device->transfer_pool = std::move(tpool);

    // Create descriptor pool
    std::array<::vk::DescriptorPoolSize, 6> pool_sizes = {{
        {::vk::DescriptorType::eUniformBuffer, 1000},
        {::vk::DescriptorType::eStorageBuffer, 1000},
        {::vk::DescriptorType::eSampledImage, 1000},
        {::vk::DescriptorType::eStorageImage, 1000},
        {::vk::DescriptorType::eSampler, 1000},
        {::vk::DescriptorType::eAccelerationStructureKHR, 100},
    }};

    auto [dpool_result, dpool] = device->device->createDescriptorPoolUnique(::vk::DescriptorPoolCreateInfo{
        .flags = ::vk::DescriptorPoolCreateFlagBits::eFreeDescriptorSet,
        .maxSets = 1000,
        .poolSizeCount = static_cast<uint32_t>(pool_sizes.size()),
        .pPoolSizes = pool_sizes.data(),
    });
    if (dpool_result != ::vk::Result::eSuccess) return from_vk_result(dpool_result);
    device->descriptor_pool = std::move(dpool);

    // Initialize VMA
    VmaVulkanFunctions vma_funcs{
        .vkGetInstanceProcAddr = vkGetInstanceProcAddr,
        .vkGetDeviceProcAddr = vkGetDeviceProcAddr,
    };

    VmaAllocatorCreateInfo allocator_info{
        .flags = adapter->has_ray_tracing ? VMA_ALLOCATOR_CREATE_BUFFER_DEVICE_ADDRESS_BIT : 0u,
        .physicalDevice = adapter->physical_device,
        .device = device->device.get(),
        .pVulkanFunctions = &vma_funcs,
        .instance = adapter->instance->instance.get(),
        .vulkanApiVersion = VK_API_VERSION_1_3,
    };

    VmaAllocator allocator;
    if (vmaCreateAllocator(&allocator_info, &allocator) != VK_SUCCESS) {
        return error(GC_E_OUT_OF_MEMORY, "failed to create VMA allocator");
    }
    device->allocator.reset(allocator);

    // Store adapter reference (proper owning reference)
    device->adapter = adapter->shared_from_this();

    *out_device = reinterpret_cast<GCDevicePtr>(device.get());
    return ok();
}

extern "C" GCError GCDeviceDestroy(GCDevicePtr device_ptr) {
    if (device_ptr) {
        auto* device = reinterpret_cast<Device*>(device_ptr);
        device->device->waitIdle();
        device->prevent_destroy.reset();  // Allow destruction
    }
    return ok();
}

extern "C" GCError GCDeviceGetQueue(GCDevicePtr device_ptr, GCQueueType type, uint32_t index,
                                     GCQueuePtr* out_queue) {
    if (!device_ptr || !out_queue) return invalid_argument("device or out_queue is NULL");
    if (index > 0) return invalid_argument("only queue index 0 is supported");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    auto queue = std::make_unique<Queue>();
    queue->device = device->shared_from_this();  // Proper owning reference
    queue->type = type;

    switch (type) {
        case GC_QUEUE_GRAPHICS:
            queue->queue = device->graphics_queue;
            queue->pool = device->graphics_pool.get();
            queue->family_index = device->adapter->graphics_queue_family;
            break;
        case GC_QUEUE_COMPUTE:
            queue->queue = device->compute_queue;
            queue->pool = device->compute_pool.get();
            queue->family_index = device->adapter->compute_queue_family;
            break;
        case GC_QUEUE_TRANSFER:
            queue->queue = device->transfer_queue;
            queue->pool = device->transfer_pool.get();
            queue->family_index = device->adapter->transfer_queue_family;
            break;
        default:
            return invalid_argument("invalid queue type");
    }

    *out_queue = reinterpret_cast<GCQueuePtr>(queue.release());
    return ok();
}

// ============================================================================
// Queue
// ============================================================================

extern "C" GCError GCQueueSubmit(GCQueuePtr queue_ptr, const GCSubmitInfo* submit) {
    if (!queue_ptr || !submit) return invalid_argument("queue or submit is NULL");

    auto* queue = reinterpret_cast<Queue*>(queue_ptr);

    std::vector<::vk::CommandBufferSubmitInfo> cmd_infos;
    for (uint32_t i = 0; i < submit->command_buffer_count; i++) {
        auto* cmd = reinterpret_cast<CommandBuffer*>(submit->command_buffers[i]);
        cmd_infos.push_back(::vk::CommandBufferSubmitInfo{
            .commandBuffer = cmd->buffer.get(),
        });
    }

    std::vector<::vk::SemaphoreSubmitInfo> wait_infos;
    for (uint32_t i = 0; i < submit->wait_count; i++) {
        auto* sem = reinterpret_cast<TimelineSemaphore*>(submit->wait_semaphores[i]);
        wait_infos.push_back(::vk::SemaphoreSubmitInfo{
            .semaphore = sem->semaphore.get(),
            .value = submit->wait_values[i],
            .stageMask = ::vk::PipelineStageFlagBits2::eAllCommands,
        });
    }

    std::vector<::vk::SemaphoreSubmitInfo> signal_infos;
    for (uint32_t i = 0; i < submit->signal_count; i++) {
        auto* sem = reinterpret_cast<TimelineSemaphore*>(submit->signal_semaphores[i]);
        signal_infos.push_back(::vk::SemaphoreSubmitInfo{
            .semaphore = sem->semaphore.get(),
            .value = submit->signal_values[i],
            .stageMask = ::vk::PipelineStageFlagBits2::eAllCommands,
        });
    }

    // Add fence signal if provided
    if (submit->fence) {
        auto* fence = reinterpret_cast<Fence*>(submit->fence);
        fence->value++;
        signal_infos.push_back(::vk::SemaphoreSubmitInfo{
            .semaphore = fence->timeline_semaphore.get(),
            .value = fence->value,
            .stageMask = ::vk::PipelineStageFlagBits2::eAllCommands,
        });
    }

    ::vk::SubmitInfo2 submit_info{
        .waitSemaphoreInfoCount = static_cast<uint32_t>(wait_infos.size()),
        .pWaitSemaphoreInfos = wait_infos.data(),
        .commandBufferInfoCount = static_cast<uint32_t>(cmd_infos.size()),
        .pCommandBufferInfos = cmd_infos.data(),
        .signalSemaphoreInfoCount = static_cast<uint32_t>(signal_infos.size()),
        .pSignalSemaphoreInfos = signal_infos.data(),
    };

    auto result = queue->queue.submit2(1, &submit_info, nullptr);
    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }
    return ok();
}

extern "C" GCError GCQueueWaitIdle(GCQueuePtr queue_ptr) {
    if (!queue_ptr) return invalid_argument("queue is NULL");

    auto result = reinterpret_cast<Queue*>(queue_ptr)->queue.waitIdle();
    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }
    return ok();
}
