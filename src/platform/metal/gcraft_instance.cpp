// gcraft_instance.cpp - Platform, instance, and adapter implementation (Metal-cpp)

#include "gcraft_internal.h"

extern "C" {

GCError GCPlatformCreate(GCPlatformPtr* out_platform) {
  if (!out_platform) return ep_invalid_argument("out_platform is NULL");

  GCPlatform* platform = static_cast<GCPlatform*>(calloc(1, sizeof(GCPlatform)));
  if (!platform) return ep_out_of_memory();

  *out_platform = platform;
  return ep_ok();
}

GCError GCPlatformDestroy(GCPlatformPtr platform) {
  if (platform) free(platform);
  return ep_ok();
}

GCError GCInstanceCreate(const GCInstanceDesc* desc, GCInstancePtr* out_instance) {
  if (!desc || !out_instance) return ep_invalid_argument("desc or out_instance is NULL");

  NS::AutoreleasePool* pool = NS::AutoreleasePool::alloc()->init();

  GCInstance* instance = static_cast<GCInstance*>(calloc(1, sizeof(GCInstance)));
  if (!instance) {
    pool->drain();
    return ep_out_of_memory();
  }

  instance->enabled_backends = desc->enable_backends;
  instance->validation_enabled = desc->enable_validation;
  instance->debug_names_enabled = desc->enable_debug_names;

  if (desc->enable_backends & GC_BACKEND_METAL_BIT) {
    instance->devices = MTL::CopyAllDevices();
    if (instance->devices) instance->devices->retain();
  }

  *out_instance = instance;
  pool->drain();
  return ep_ok();
}

GCError GCInstanceDestroy(GCInstancePtr instance) {
  if (instance) {
    if (instance->devices) instance->devices->release();
    free(instance);
  }
  return ep_ok();
}

GCError GCInstanceEnumerateAdapters(GCInstancePtr instance, uint32_t* io_count,
                                    GCAdapterPtr* out_adapters) {
  if (!instance || !io_count) return ep_invalid_argument("instance or io_count is NULL");

  NS::AutoreleasePool* pool = NS::AutoreleasePool::alloc()->init();

  size_t device_count = instance->devices ? instance->devices->count() : 0;

  if (!out_adapters) {
    *io_count = static_cast<uint32_t>(device_count);
    pool->drain();
    return ep_ok();
  }

  uint32_t count = (*io_count < device_count) ? *io_count : static_cast<uint32_t>(device_count);

  for (uint32_t i = 0; i < count; i++) {
    GCAdapter* adapter = static_cast<GCAdapter*>(calloc(1, sizeof(GCAdapter)));
    if (!adapter) {
      pool->drain();
      return ep_out_of_memory();
    }

    MTL::Device* dev = static_cast<MTL::Device*>(instance->devices->object(i));
    adapter->device = dev;
    adapter->device->retain();

    const char* name = dev->name() ? dev->name()->utf8String() : "";
    strncpy(adapter->properties.name, name, sizeof(adapter->properties.name) - 1);
    adapter->properties.name[sizeof(adapter->properties.name) - 1] = '\0';
    adapter->properties.vendor_id = 0;
    adapter->properties.device_id = static_cast<uint32_t>(dev->registryID());
    adapter->properties.backends = GC_BACKEND_METAL_BIT;

    GCFeatureFlags features = static_cast<GCFeatureFlags>(GC_FEATURE_COMPUTE_BIT | GC_FEATURE_TIMELINE_SEMAPHORE_BIT);

    if (dev->supportsFamily(MTL::GPUFamilyApple7) || dev->supportsFamily(MTL::GPUFamilyMac2)) {
      features = static_cast<GCFeatureFlags>(features | GC_FEATURE_MESH_SHADER_BIT);
    }
    if (dev->supportsFamily(MTL::GPUFamilyApple6) || dev->supportsFamily(MTL::GPUFamilyMac2)) {
      features = static_cast<GCFeatureFlags>(features | GC_FEATURE_RAY_TRACING_BIT);
    }
    if (dev->argumentBuffersSupport() >= MTL::ArgumentBuffersTier2) {
      features = static_cast<GCFeatureFlags>(features | GC_FEATURE_DESCRIPTOR_INDEXING_BIT | GC_FEATURE_BINDLESS_BIT);
    }

    adapter->properties.features = features;

    adapter->properties.limits.max_texture_dimension_2d = 16384;
    adapter->properties.limits.max_texture_dimension_3d = 2048;
    adapter->properties.limits.max_texture_array_layers = 2048;
    adapter->properties.limits.max_bindless_resources = 500000;
    adapter->properties.limits.max_push_constants_size = 4096;
    adapter->properties.limits.max_threads_per_threadgroup = static_cast<uint32_t>(dev->maxThreadsPerThreadgroup().width);

    out_adapters[i] = adapter;
  }

  *io_count = count;
  pool->drain();
  return ep_ok();
}

GCError GCAdapterGetProperties(GCAdapterPtr adapter, GCAdapterProperties* out_properties) {
  if (!adapter || !out_properties) return ep_invalid_argument("adapter or out_properties is NULL");
  *out_properties = adapter->properties;
  return ep_ok();
}

}
