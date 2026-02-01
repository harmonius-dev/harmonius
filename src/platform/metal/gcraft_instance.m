// gcraft_instance.m - Platform, instance, and adapter implementation

#include "gcraft_internal.h"

#pragma mark - Platform

GCError GCPlatformCreate(GCPlatformPtr *out_platform) {
    if (!out_platform) return ep_invalid_argument("out_platform is NULL");

    GCPlatform *platform = calloc(1, sizeof(GCPlatform));
    if (!platform) return ep_out_of_memory();

    *out_platform = platform;
    return ep_ok();
}

GCError GCPlatformDestroy(GCPlatformPtr platform) {
    if (platform) free(platform);
    return ep_ok();
}

#pragma mark - Instance

GCError GCInstanceCreate(const GCInstanceDesc *desc, GCInstancePtr *out_instance) {
    if (!desc || !out_instance) return ep_invalid_argument("desc or out_instance is NULL");

    @autoreleasepool {
        GCInstance *instance = calloc(1, sizeof(GCInstance));
        if (!instance) return ep_out_of_memory();

        instance->enabled_backends = desc->enable_backends;
        instance->validation_enabled = desc->enable_validation;
        instance->debug_names_enabled = desc->enable_debug_names;

        if (desc->enable_backends & GC_BACKEND_METAL_BIT) {
            instance->devices = [MTLCopyAllDevices() retain];
        }

        *out_instance = instance;
        return ep_ok();
    }
}

GCError GCInstanceDestroy(GCInstancePtr instance) {
    if (instance) {
        @autoreleasepool {
            if (instance->devices) [instance->devices release];
        }
        free(instance);
    }
    return ep_ok();
}

GCError GCInstanceEnumerateAdapters(GCInstancePtr instance, uint32_t *io_count,
                                    GCAdapterPtr *out_adapters) {
    if (!instance || !io_count) return ep_invalid_argument("instance or io_count is NULL");

    @autoreleasepool {
        NSUInteger device_count = instance->devices ? instance->devices.count : 0;

        if (!out_adapters) {
            *io_count = (uint32_t)device_count;
            return ep_ok();
        }

        uint32_t count = (*io_count < device_count) ? *io_count : (uint32_t)device_count;

        for (uint32_t i = 0; i < count; i++) {
            GCAdapter *adapter = calloc(1, sizeof(GCAdapter));
            if (!adapter) return ep_out_of_memory();

            adapter->device = [instance->devices[i] retain];

            // Fill properties
            const char *name = [adapter->device.name UTF8String];
            strncpy(adapter->properties.name, name, sizeof(adapter->properties.name) - 1);
            adapter->properties.vendor_id = 0;
            adapter->properties.device_id = (uint32_t)adapter->device.registryID;
            adapter->properties.backends = GC_BACKEND_METAL_BIT;

            // Feature detection for Metal 3+/4
            GCFeatureFlags features = GC_FEATURE_COMPUTE_BIT | GC_FEATURE_TIMELINE_SEMAPHORE_BIT;

            // Mesh shaders require Apple7 or Mac2 GPU family
            if ([adapter->device supportsFamily:MTLGPUFamilyApple7] ||
                [adapter->device supportsFamily:MTLGPUFamilyMac2]) {
                features |= GC_FEATURE_MESH_SHADER_BIT;
            }
            // Ray tracing requires Apple6 or Mac2 GPU family
            if ([adapter->device supportsFamily:MTLGPUFamilyApple6] ||
                [adapter->device supportsFamily:MTLGPUFamilyMac2]) {
                features |= GC_FEATURE_RAY_TRACING_BIT;
            }
            // Argument buffers tier 2 for bindless
            if (adapter->device.argumentBuffersSupport >= MTLArgumentBuffersTier2) {
                features |= GC_FEATURE_DESCRIPTOR_INDEXING_BIT | GC_FEATURE_BINDLESS_BIT;
            }

            adapter->properties.features = features;

            // Limits
            adapter->properties.limits.max_texture_dimension_2d = 16384;
            adapter->properties.limits.max_texture_dimension_3d = 2048;
            adapter->properties.limits.max_texture_array_layers = 2048;
            adapter->properties.limits.max_bindless_resources = 500000;
            adapter->properties.limits.max_push_constants_size = 4096;
            adapter->properties.limits.max_threads_per_threadgroup = (uint32_t)adapter->device.maxThreadsPerThreadgroup.width;

            out_adapters[i] = adapter;
        }

        *io_count = count;
        return ep_ok();
    }
}

GCError GCAdapterGetProperties(GCAdapterPtr adapter, GCAdapterProperties *out_properties) {
    if (!adapter || !out_properties) return ep_invalid_argument("adapter or out_properties is NULL");
    *out_properties = adapter->properties;
    return ep_ok();
}
