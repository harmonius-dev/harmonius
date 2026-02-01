#ifndef GCRAFT_RESOURCE_H
#define GCRAFT_RESOURCE_H

#include "gcraft_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Enumerations
// ============================================================================

typedef enum GCShaderFormat {
  GC_SHADER_MSL = 0,
  GC_SHADER_HLSL = 1,
  GC_SHADER_SPIRV = 2,
} GCShaderFormat;

typedef enum GCBufferUsageFlags : uint32_t {
  GC_BUFFER_USAGE_TRANSFER_SRC_BIT = (1u << 0),
  GC_BUFFER_USAGE_TRANSFER_DST_BIT = (1u << 1),
  GC_BUFFER_USAGE_VERTEX_BIT       = (1u << 2),
  GC_BUFFER_USAGE_INDEX_BIT        = (1u << 3),
  GC_BUFFER_USAGE_UNIFORM_BIT      = (1u << 4),
  GC_BUFFER_USAGE_STORAGE_BIT      = (1u << 5),
  GC_BUFFER_USAGE_INDIRECT_BIT     = (1u << 6),
  GC_BUFFER_USAGE_ACCEL_BIT        = (1u << 7),
} GCBufferUsageFlags;

typedef enum GCTextureUsageFlags : uint32_t {
  GC_TEXTURE_USAGE_SAMPLED_BIT          = (1u << 0),
  GC_TEXTURE_USAGE_STORAGE_BIT          = (1u << 1),
  GC_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT = (1u << 2),
  GC_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT = (1u << 3),
  GC_TEXTURE_USAGE_TRANSFER_SRC_BIT     = (1u << 4),
  GC_TEXTURE_USAGE_TRANSFER_DST_BIT     = (1u << 5),
} GCTextureUsageFlags;

typedef enum GCTextureDimension {
  GC_TEXTURE_DIM_1D = 0,
  GC_TEXTURE_DIM_2D = 1,
  GC_TEXTURE_DIM_3D = 2,
  GC_TEXTURE_DIM_CUBE = 3,
} GCTextureDimension;

typedef enum GCTextureFormat {
  GC_FORMAT_RGBA8_UNORM = 0,
  GC_FORMAT_BGRA8_UNORM = 1,
  GC_FORMAT_RGBA16_FLOAT = 2,
  GC_FORMAT_RGBA32_FLOAT = 3,
  GC_FORMAT_D24S8 = 4,
  GC_FORMAT_D32_FLOAT = 5,
} GCTextureFormat;

typedef enum GCFilter {
  GC_FILTER_NEAREST = 0,
  GC_FILTER_LINEAR = 1,
} GCFilter;

typedef enum GCAddressMode {
  GC_ADDRESS_CLAMP_TO_EDGE = 0,
  GC_ADDRESS_REPEAT = 1,
  GC_ADDRESS_MIRROR_REPEAT = 2,
} GCAddressMode;

typedef enum GCCompareOp {
  GC_COMPARE_NEVER = 0,
  GC_COMPARE_LESS = 1,
  GC_COMPARE_EQUAL = 2,
  GC_COMPARE_LESS_EQUAL = 3,
  GC_COMPARE_GREATER = 4,
  GC_COMPARE_NOT_EQUAL = 5,
  GC_COMPARE_GREATER_EQUAL = 6,
  GC_COMPARE_ALWAYS = 7,
} GCCompareOp;

// ============================================================================
// Structures
// ============================================================================

typedef struct GCBufferDesc {
  uint64_t size;
  GCBufferUsageFlags usage;
  bool host_visible;
} GCBufferDesc;

typedef struct GCTextureDesc {
  GCTextureDimension dimension;
  GCTextureFormat format;
  uint32_t width;
  uint32_t height;
  uint32_t depth;
  uint32_t mip_levels;
  uint32_t array_layers;
  GCTextureUsageFlags usage;
} GCTextureDesc;

typedef struct GCSamplerDesc {
  GCFilter min_filter;
  GCFilter mag_filter;
  GCAddressMode address_u;
  GCAddressMode address_v;
  GCAddressMode address_w;
  GCCompareOp compare_op;
  float max_anisotropy;
} GCSamplerDesc;

typedef struct GCShaderLibraryDesc {
  GCShaderFormat format;
  const uint8_t *data;
  uint64_t size;
  GCStringUTF8 label;
} GCShaderLibraryDesc;

// ============================================================================
// Functions
// ============================================================================

GCError GCBufferCreate(GCDevicePtr device, const GCBufferDesc *desc,
                       GCBufferPtr *out_buffer);
GCError GCBufferDestroy(GCBufferPtr buffer);
GCError GCBufferMap(GCBufferPtr buffer, void **out_data);
GCError GCBufferUnmap(GCBufferPtr buffer);

GCError GCTextureCreate(GCDevicePtr device, const GCTextureDesc *desc,
                        GCTexturePtr *out_texture);
GCError GCTextureDestroy(GCTexturePtr texture);

GCError GCSamplerCreate(GCDevicePtr device, const GCSamplerDesc *desc,
                        GCSamplerPtr *out_sampler);
GCError GCSamplerDestroy(GCSamplerPtr sampler);

GCError GCShaderLibraryCreate(GCDevicePtr device,
                              const GCShaderLibraryDesc *desc,
                              GCShaderLibraryPtr *out_library);
GCError GCShaderLibraryDestroy(GCShaderLibraryPtr library);

#ifdef __cplusplus
}
#endif

#endif
