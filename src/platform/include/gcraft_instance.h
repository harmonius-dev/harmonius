#ifndef GCRAFT_INSTANCE_H
#define GCRAFT_INSTANCE_H

#include "gcraft_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum GCBackendFlags : uint32_t {
  GC_BACKEND_METAL_BIT  = (1u << 0),
  GC_BACKEND_D3D12_BIT  = (1u << 1),
  GC_BACKEND_VULKAN_BIT = (1u << 2),
} GCBackendFlags;

typedef enum GCBackend {
  GC_BACKEND_METAL = 0,
  GC_BACKEND_D3D12 = 1,
  GC_BACKEND_VULKAN = 2,
} GCBackend;

typedef enum GCFeatureFlags : uint32_t {
  GC_FEATURE_COMPUTE_BIT             = (1u << 0),
  GC_FEATURE_TIMELINE_SEMAPHORE_BIT  = (1u << 1),
  GC_FEATURE_MESH_SHADER_BIT         = (1u << 2),
  GC_FEATURE_RAY_TRACING_BIT         = (1u << 3),
  GC_FEATURE_DESCRIPTOR_INDEXING_BIT = (1u << 4),
  GC_FEATURE_BINDLESS_BIT            = (1u << 5),
} GCFeatureFlags;

typedef struct GCInstanceDesc {
  GCBackendFlags enable_backends;
  bool enable_validation;
  bool enable_debug_names;
} GCInstanceDesc;

typedef struct GCDeviceLimits {
  uint32_t max_texture_dimension_2d;
  uint32_t max_texture_dimension_3d;
  uint32_t max_texture_array_layers;
  uint32_t max_bindless_resources;
  uint32_t max_push_constants_size;
  uint32_t max_threads_per_threadgroup;
} GCDeviceLimits;

typedef struct GCAdapterProperties {
  char name[256];
  uint32_t vendor_id;
  uint32_t device_id;
  GCBackendFlags backends;
  GCFeatureFlags features;
  GCDeviceLimits limits;
} GCAdapterProperties;

GCError GCPlatformCreate(GCPlatformPtr *out_platform);
GCError GCPlatformDestroy(GCPlatformPtr platform);

GCError GCInstanceCreate(const GCInstanceDesc *desc, GCInstancePtr *out_instance);
GCError GCInstanceDestroy(GCInstancePtr instance);
GCError GCInstanceEnumerateAdapters(GCInstancePtr instance, uint32_t *io_count,
                                    GCAdapterPtr *out_adapters);
GCError GCAdapterGetProperties(GCAdapterPtr adapter,
                               GCAdapterProperties *out_properties);

#ifdef __cplusplus
}
#endif

#endif
