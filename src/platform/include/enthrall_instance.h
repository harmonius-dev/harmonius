#ifndef ENTHRALL_INSTANCE_H
#define ENTHRALL_INSTANCE_H

#include "enthrall_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum EPBackendFlags : uint32_t {
  EP_BACKEND_METAL_BIT  = (1u << 0),
  EP_BACKEND_D3D12_BIT  = (1u << 1),
  EP_BACKEND_VULKAN_BIT = (1u << 2),
} EPBackendFlags;

typedef enum EPBackend {
  EP_BACKEND_METAL = 0,
  EP_BACKEND_D3D12 = 1,
  EP_BACKEND_VULKAN = 2,
} EPBackend;

typedef enum EPFeatureFlags : uint32_t {
  EP_FEATURE_COMPUTE_BIT             = (1u << 0),
  EP_FEATURE_TIMELINE_SEMAPHORE_BIT  = (1u << 1),
  EP_FEATURE_MESH_SHADER_BIT         = (1u << 2),
  EP_FEATURE_RAY_TRACING_BIT         = (1u << 3),
  EP_FEATURE_DESCRIPTOR_INDEXING_BIT = (1u << 4),
  EP_FEATURE_BINDLESS_BIT            = (1u << 5),
} EPFeatureFlags;

typedef struct EPInstanceDesc {
  EPBackendFlags enable_backends;
  bool enable_validation;
  bool enable_debug_names;
} EPInstanceDesc;

typedef struct EPDeviceLimits {
  uint32_t max_texture_dimension_2d;
  uint32_t max_texture_dimension_3d;
  uint32_t max_texture_array_layers;
  uint32_t max_bindless_resources;
  uint32_t max_push_constants_size;
  uint32_t max_threads_per_threadgroup;
} EPDeviceLimits;

typedef struct EPAdapterProperties {
  char name[256];
  uint32_t vendor_id;
  uint32_t device_id;
  EPBackendFlags backends;
  EPFeatureFlags features;
  EPDeviceLimits limits;
} EPAdapterProperties;

EPError EPPlatformCreate(EPPlatformPtr *out_platform);
EPError EPPlatformDestroy(EPPlatformPtr platform);

EPError EPInstanceCreate(const EPInstanceDesc *desc, EPInstancePtr *out_instance);
EPError EPInstanceDestroy(EPInstancePtr instance);
EPError EPInstanceEnumerateAdapters(EPInstancePtr instance, uint32_t *io_count,
                                    EPAdapterPtr *out_adapters);
EPError EPAdapterGetProperties(EPAdapterPtr adapter,
                               EPAdapterProperties *out_properties);

#ifdef __cplusplus
}
#endif

#endif
