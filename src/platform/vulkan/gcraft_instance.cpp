// gcraft_instance.cpp - Platform, instance, and adapter implementation

#define VOLK_IMPLEMENTATION
#include <volk.h>
#include "gcraft_internal.h"

VULKAN_HPP_DEFAULT_DISPATCH_LOADER_DYNAMIC_STORAGE

using namespace ep::vk;

// ============================================================================
// Platform
// ============================================================================

extern "C" GCError GCPlatformCreate(GCPlatformPtr* out_platform) {
    if (!out_platform) return invalid_argument("out_platform is NULL");

    // Initialize volk - this loads vkGetInstanceProcAddr and basic instance functions
    if (volkInitialize() != VK_SUCCESS) {
        return error(GC_E_UNSUPPORTED, "failed to initialize vulkan loader");
    }

    // Initialize vulkan-hpp dispatcher with volk's loaded functions
    VULKAN_HPP_DEFAULT_DISPATCHER.init(vkGetInstanceProcAddr);

    auto platform = std::make_unique<Platform>();
    *out_platform = reinterpret_cast<GCPlatformPtr>(platform.release());
    return ok();
}

extern "C" GCError GCPlatformDestroy(GCPlatformPtr platform) {
    delete reinterpret_cast<Platform*>(platform);
    return ok();
}

// ============================================================================
// Instance
// ============================================================================

extern "C" GCError GCInstanceCreate(const GCInstanceDesc* desc, GCInstancePtr* out_instance) {
    if (!desc || !out_instance) return invalid_argument("desc or out_instance is NULL");

    auto instance = std::make_shared<Instance>();
    instance->prevent_destroy = instance;  // Keep alive until GCInstanceDestroy
    instance->enabled_backends = desc->enable_backends;
    instance->validation_enabled = desc->enable_validation;
    instance->debug_names_enabled = desc->enable_debug_names;

    // Query available extensions
    auto [ext_result, available_extensions] = ::vk::enumerateInstanceExtensionProperties();
    
    // Build extension list - only add extensions that are available
    std::vector<const char*> extensions;
    
    auto has_extension = [&](const char* name) {
        for (const auto& ext : available_extensions) {
            if (std::strcmp(ext.extensionName.data(), name) == 0) return true;
        }
        return false;
    };

    // Add surface extensions only if available
    if (has_extension(VK_KHR_SURFACE_EXTENSION_NAME)) {
        extensions.push_back(VK_KHR_SURFACE_EXTENSION_NAME);
    }
#ifdef __APPLE__
    if (has_extension("VK_EXT_metal_surface")) {
        extensions.push_back("VK_EXT_metal_surface");
    }
    if (has_extension("VK_KHR_portability_enumeration")) {
        extensions.push_back("VK_KHR_portability_enumeration");
    }
    if (has_extension("VK_KHR_get_physical_device_properties2")) {
        extensions.push_back("VK_KHR_get_physical_device_properties2");
    }
#endif

    // Build layer list
    std::vector<const char*> layers;
    if (desc->enable_validation) {
        layers.push_back("VK_LAYER_KHRONOS_validation");
    }

    // Query the supported API version from the driver
    uint32_t api_version = VK_API_VERSION_1_0;
    if (vkEnumerateInstanceVersion) {
        vkEnumerateInstanceVersion(&api_version);
    }
    // Cap at 1.3 max
    if (api_version > VK_API_VERSION_1_3) {
        api_version = VK_API_VERSION_1_3;
    }

    ::vk::ApplicationInfo app_info{
        .pApplicationName = "Gcraft",
        .applicationVersion = VK_MAKE_VERSION(1, 0, 0),
        .pEngineName = "Gcraft",
        .engineVersion = VK_MAKE_VERSION(1, 0, 0),
        .apiVersion = api_version,
    };

    ::vk::InstanceCreateFlags flags{};
#ifdef __APPLE__
    if (has_extension("VK_KHR_portability_enumeration")) {
        flags |= ::vk::InstanceCreateFlagBits::eEnumeratePortabilityKHR;
    }
#endif

    ::vk::InstanceCreateInfo create_info{
        .flags = flags,
        .pApplicationInfo = &app_info,
        .enabledLayerCount = static_cast<uint32_t>(layers.size()),
        .ppEnabledLayerNames = layers.data(),
        .enabledExtensionCount = static_cast<uint32_t>(extensions.size()),
        .ppEnabledExtensionNames = extensions.data(),
    };

    auto [result, vk_instance] = ::vk::createInstanceUnique(create_info);
    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }
    instance->instance = std::move(vk_instance);

    // Load instance-level functions
    volkLoadInstance(instance->instance.get());
    VULKAN_HPP_DEFAULT_DISPATCHER.init(instance->instance.get());

    // Enumerate physical devices
    auto [enum_result, devices] = instance->instance->enumeratePhysicalDevices();
    if (enum_result != ::vk::Result::eSuccess) {
        return from_vk_result(enum_result);
    }
    instance->physical_devices = std::move(devices);

    *out_instance = reinterpret_cast<GCInstancePtr>(instance.get());
    return ok();
}

extern "C" GCError GCInstanceDestroy(GCInstancePtr instance_ptr) {
    if (instance_ptr) {
        auto* inst = reinterpret_cast<Instance*>(instance_ptr);
        inst->prevent_destroy.reset();  // Allow destruction
    }
    return ok();
}

extern "C" GCError GCInstanceEnumerateAdapters(GCInstancePtr instance, uint32_t* io_count,
                                               GCAdapterPtr* out_adapters) {
    if (!instance || !io_count) return invalid_argument("instance or io_count is NULL");

    auto* inst = reinterpret_cast<Instance*>(instance);

    if (!out_adapters) {
        *io_count = static_cast<uint32_t>(inst->physical_devices.size());
        return ok();
    }

    uint32_t count = std::min(*io_count, static_cast<uint32_t>(inst->physical_devices.size()));

    for (uint32_t i = 0; i < count; i++) {
        auto adapter = std::make_shared<Adapter>();
        adapter->prevent_destroy = adapter;  // Keep alive until user destroys it
        adapter->instance = inst->shared_from_this();  // Proper owning reference
        adapter->physical_device = inst->physical_devices[i];
        adapter->properties = adapter->physical_device.getProperties();
        adapter->features = adapter->physical_device.getFeatures();
        adapter->memory_properties = adapter->physical_device.getMemoryProperties();

        // Fill EP properties
        std::strncpy(adapter->ep_properties.name, adapter->properties.deviceName.data(),
                     sizeof(adapter->ep_properties.name) - 1);
        adapter->ep_properties.vendor_id = adapter->properties.vendorID;
        adapter->ep_properties.device_id = adapter->properties.deviceID;
        adapter->ep_properties.backends = GC_BACKEND_VULKAN_BIT;

        // Find queue families
        auto queue_families = adapter->physical_device.getQueueFamilyProperties();
        for (uint32_t j = 0; j < queue_families.size(); j++) {
            const auto& qf = queue_families[j];
            if ((qf.queueFlags & ::vk::QueueFlagBits::eGraphics) && adapter->graphics_queue_family == UINT32_MAX) {
                adapter->graphics_queue_family = j;
            }
            if ((qf.queueFlags & ::vk::QueueFlagBits::eCompute) && adapter->compute_queue_family == UINT32_MAX) {
                adapter->compute_queue_family = j;
            }
            if ((qf.queueFlags & ::vk::QueueFlagBits::eTransfer) && adapter->transfer_queue_family == UINT32_MAX) {
                adapter->transfer_queue_family = j;
            }
        }

        // Check for extensions
        auto [ext_result, extensions] = adapter->physical_device.enumerateDeviceExtensionProperties();
        if (ext_result == ::vk::Result::eSuccess) {
            for (const auto& ext : extensions) {
                std::string_view name(ext.extensionName.data());
                if (name == VK_EXT_MESH_SHADER_EXTENSION_NAME) adapter->has_mesh_shader = true;
                if (name == VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME) adapter->has_ray_tracing = true;
                if (name == VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME) adapter->has_timeline_semaphore = true;
                if (name == VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME) adapter->has_dynamic_rendering = true;
            }
        }

        // Feature detection
        uint32_t features = GC_FEATURE_COMPUTE_BIT;
        if (adapter->has_timeline_semaphore) features |= GC_FEATURE_TIMELINE_SEMAPHORE_BIT;
        if (adapter->has_mesh_shader) features |= GC_FEATURE_MESH_SHADER_BIT;
        if (adapter->has_ray_tracing) features |= GC_FEATURE_RAY_TRACING_BIT;
        if (adapter->features.shaderSampledImageArrayDynamicIndexing) {
            features |= GC_FEATURE_DESCRIPTOR_INDEXING_BIT | GC_FEATURE_BINDLESS_BIT;
        }
        adapter->ep_properties.features = static_cast<GCFeatureFlags>(features);

        // Limits
        const auto& limits = adapter->properties.limits;
        adapter->ep_properties.limits.max_texture_dimension_2d = limits.maxImageDimension2D;
        adapter->ep_properties.limits.max_texture_dimension_3d = limits.maxImageDimension3D;
        adapter->ep_properties.limits.max_texture_array_layers = limits.maxImageArrayLayers;
        adapter->ep_properties.limits.max_bindless_resources = limits.maxDescriptorSetSampledImages;
        adapter->ep_properties.limits.max_push_constants_size = limits.maxPushConstantsSize;
        adapter->ep_properties.limits.max_threads_per_threadgroup = limits.maxComputeWorkGroupInvocations;

        out_adapters[i] = reinterpret_cast<GCAdapterPtr>(adapter.get());
    }

    *io_count = count;
    return ok();
}

extern "C" GCError GCAdapterGetProperties(GCAdapterPtr adapter, GCAdapterProperties* out_properties) {
    if (!adapter || !out_properties) return invalid_argument("adapter or out_properties is NULL");
    *out_properties = reinterpret_cast<Adapter*>(adapter)->ep_properties;
    return ok();
}
