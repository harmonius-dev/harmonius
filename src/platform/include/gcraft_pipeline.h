#ifndef GCRAFT_PIPELINE_H
#define GCRAFT_PIPELINE_H

#include "gcraft_resource.h"
#include "gcraft_descriptor.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Structures
// ============================================================================

typedef struct GCVertexInputAttributeDesc {
  uint32_t location;
  uint32_t binding;
  GCTextureFormat format;
  uint32_t offset;
} GCVertexInputAttributeDesc;

typedef struct GCVertexInputBindingDesc {
  uint32_t binding;
  uint32_t stride;
  bool per_instance;
} GCVertexInputBindingDesc;

typedef struct GCRasterStateDesc {
  bool depth_test_enable;
  bool depth_write_enable;
  GCCompareOp depth_compare;
} GCRasterStateDesc;

typedef struct GCRenderPipelineDesc {
  GCShaderLibraryPtr library;
  GCStringUTF8 vertex_entry;
  GCStringUTF8 fragment_entry;
  GCStringUTF8 mesh_entry;
  GCStringUTF8 task_entry;
  GCTextureFormat color_formats[8];
  uint32_t color_format_count;
  GCTextureFormat depth_format;
  GCTextureFormat stencil_format;
  const GCVertexInputAttributeDesc *attributes;
  uint32_t attribute_count;
  const GCVertexInputBindingDesc *bindings;
  uint32_t binding_count;
  GCRasterStateDesc raster_state;
  GCPipelineLayoutPtr layout;
} GCRenderPipelineDesc;

typedef struct GCComputePipelineDesc {
  GCShaderLibraryPtr library;
  GCStringUTF8 entry;
  GCPipelineLayoutPtr layout;
} GCComputePipelineDesc;

typedef struct GCMeshPipelineDesc {
  GCShaderLibraryPtr library;
  GCStringUTF8 mesh_entry;
  GCStringUTF8 task_entry;
  GCStringUTF8 fragment_entry;
  GCTextureFormat color_formats[8];
  uint32_t color_format_count;
  GCTextureFormat depth_format;
  GCTextureFormat stencil_format;
  GCRasterStateDesc raster_state;
  GCPipelineLayoutPtr layout;
} GCMeshPipelineDesc;

typedef struct GCRayTracingShaderGroupDesc {
  GCStringUTF8 raygen_entry;
  GCStringUTF8 miss_entry;
  GCStringUTF8 hit_entry;
  GCStringUTF8 any_hit_entry;
  GCStringUTF8 intersection_entry;
} GCRayTracingShaderGroupDesc;

typedef struct GCRayTracingPipelineDesc {
  GCShaderLibraryPtr library;
  uint32_t group_count;
  uint32_t max_recursion_depth;
  GCPipelineLayoutPtr layout;
  GCRayTracingShaderGroupDesc groups[];
} GCRayTracingPipelineDesc;

typedef struct GCAccelerationStructureGeometryDesc {
  GCBufferPtr vertex_buffer;
  uint64_t vertex_offset;
  uint32_t vertex_stride;
  uint32_t vertex_count;
  GCBufferPtr index_buffer;
  uint64_t index_offset;
  uint32_t index_count;
  bool opaque;
} GCAccelerationStructureGeometryDesc;

typedef struct GCAccelerationStructureDesc {
  uint32_t geometry_count;
  bool top_level;
  bool allow_update;
  GCAccelerationStructureGeometryDesc geometries[];
} GCAccelerationStructureDesc;

// ============================================================================
// Functions
// ============================================================================

GCError GCRenderPipelineCreate(GCDevicePtr device,
                               const GCRenderPipelineDesc *desc,
                               GCRenderPipelinePtr *out_pipeline);
GCError GCRenderPipelineDestroy(GCRenderPipelinePtr pipeline);

GCError GCComputePipelineCreate(GCDevicePtr device,
                                const GCComputePipelineDesc *desc,
                                GCComputePipelinePtr *out_pipeline);
GCError GCComputePipelineDestroy(GCComputePipelinePtr pipeline);

GCError GCMeshPipelineCreate(GCDevicePtr device,
                             const GCMeshPipelineDesc *desc,
                             GCMeshPipelinePtr *out_pipeline);
GCError GCMeshPipelineDestroy(GCMeshPipelinePtr pipeline);

GCError GCRayTracingPipelineCreate(GCDevicePtr device,
                                   const GCRayTracingPipelineDesc *desc,
                                   GCRayTracingPipelinePtr *out_pipeline);
GCError GCRayTracingPipelineDestroy(GCRayTracingPipelinePtr pipeline);

GCError GCAccelerationStructureCreate(GCDevicePtr device,
                                      const GCAccelerationStructureDesc *desc,
                                      GCAccelerationStructurePtr *out_as);
GCError GCAccelerationStructureDestroy(GCAccelerationStructurePtr as);

#ifdef __cplusplus
}
#endif

#endif
