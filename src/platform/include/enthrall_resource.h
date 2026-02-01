#ifndef ENTHRALL_RESOURCE_H
#define ENTHRALL_RESOURCE_H

#include "enthrall_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Enumerations
// ============================================================================

typedef enum EPShaderFormat {
  EP_SHADER_MSL = 0,
  EP_SHADER_HLSL = 1,
  EP_SHADER_SPIRV = 2,
} EPShaderFormat;

typedef enum EPBufferUsageFlags : uint32_t {
  EP_BUFFER_USAGE_TRANSFER_SRC_BIT = (1u << 0),
  EP_BUFFER_USAGE_TRANSFER_DST_BIT = (1u << 1),
  EP_BUFFER_USAGE_VERTEX_BIT       = (1u << 2),
  EP_BUFFER_USAGE_INDEX_BIT        = (1u << 3),
  EP_BUFFER_USAGE_UNIFORM_BIT      = (1u << 4),
  EP_BUFFER_USAGE_STORAGE_BIT      = (1u << 5),
  EP_BUFFER_USAGE_INDIRECT_BIT     = (1u << 6),
  EP_BUFFER_USAGE_ACCEL_BIT        = (1u << 7),
} EPBufferUsageFlags;

typedef enum EPTextureUsageFlags : uint32_t {
  EP_TEXTURE_USAGE_SAMPLED_BIT          = (1u << 0),
  EP_TEXTURE_USAGE_STORAGE_BIT          = (1u << 1),
  EP_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT = (1u << 2),
  EP_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT = (1u << 3),
  EP_TEXTURE_USAGE_TRANSFER_SRC_BIT     = (1u << 4),
  EP_TEXTURE_USAGE_TRANSFER_DST_BIT     = (1u << 5),
} EPTextureUsageFlags;

typedef enum EPTextureDimension {
  EP_TEXTURE_DIM_1D = 0,
  EP_TEXTURE_DIM_2D = 1,
  EP_TEXTURE_DIM_3D = 2,
  EP_TEXTURE_DIM_CUBE = 3,
} EPTextureDimension;

typedef enum EPTextureFormat {
  EP_FORMAT_RGBA8_UNORM = 0,
  EP_FORMAT_BGRA8_UNORM = 1,
  EP_FORMAT_RGBA16_FLOAT = 2,
  EP_FORMAT_RGBA32_FLOAT = 3,
  EP_FORMAT_D24S8 = 4,
  EP_FORMAT_D32_FLOAT = 5,
} EPTextureFormat;

typedef enum EPFilter {
  EP_FILTER_NEAREST = 0,
  EP_FILTER_LINEAR = 1,
} EPFilter;

typedef enum EPAddressMode {
  EP_ADDRESS_CLAMP_TO_EDGE = 0,
  EP_ADDRESS_REPEAT = 1,
  EP_ADDRESS_MIRROR_REPEAT = 2,
} EPAddressMode;

typedef enum EPCompareOp {
  EP_COMPARE_NEVER = 0,
  EP_COMPARE_LESS = 1,
  EP_COMPARE_EQUAL = 2,
  EP_COMPARE_LESS_EQUAL = 3,
  EP_COMPARE_GREATER = 4,
  EP_COMPARE_NOT_EQUAL = 5,
  EP_COMPARE_GREATER_EQUAL = 6,
  EP_COMPARE_ALWAYS = 7,
} EPCompareOp;

// ============================================================================
// Structures
// ============================================================================

typedef struct EPBufferDesc {
  uint64_t size;
  EPBufferUsageFlags usage;
  bool host_visible;
} EPBufferDesc;

typedef struct EPTextureDesc {
  EPTextureDimension dimension;
  EPTextureFormat format;
  uint32_t width;
  uint32_t height;
  uint32_t depth;
  uint32_t mip_levels;
  uint32_t array_layers;
  EPTextureUsageFlags usage;
} EPTextureDesc;

typedef struct EPSamplerDesc {
  EPFilter min_filter;
  EPFilter mag_filter;
  EPAddressMode address_u;
  EPAddressMode address_v;
  EPAddressMode address_w;
  EPCompareOp compare_op;
  float max_anisotropy;
} EPSamplerDesc;

typedef struct EPShaderLibraryDesc {
  EPShaderFormat format;
  const uint8_t *data;
  uint64_t size;
  EPStringUTF8 label;
} EPShaderLibraryDesc;

// ============================================================================
// Functions
// ============================================================================

EPError EPBufferCreate(EPDevicePtr device, const EPBufferDesc *desc,
                       EPBufferPtr *out_buffer);
EPError EPBufferDestroy(EPBufferPtr buffer);
EPError EPBufferMap(EPBufferPtr buffer, void **out_data);
EPError EPBufferUnmap(EPBufferPtr buffer);

EPError EPTextureCreate(EPDevicePtr device, const EPTextureDesc *desc,
                        EPTexturePtr *out_texture);
EPError EPTextureDestroy(EPTexturePtr texture);

EPError EPSamplerCreate(EPDevicePtr device, const EPSamplerDesc *desc,
                        EPSamplerPtr *out_sampler);
EPError EPSamplerDestroy(EPSamplerPtr sampler);

EPError EPShaderLibraryCreate(EPDevicePtr device,
                              const EPShaderLibraryDesc *desc,
                              EPShaderLibraryPtr *out_library);
EPError EPShaderLibraryDestroy(EPShaderLibraryPtr library);

#ifdef __cplusplus
}
#endif

#endif
