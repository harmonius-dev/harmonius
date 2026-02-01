#ifndef ENTHRALL_PIPELINE_H
#define ENTHRALL_PIPELINE_H

#include "enthrall_resource.h"
#include "enthrall_descriptor.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Structures
// ============================================================================

typedef struct EPVertexInputAttributeDesc {
  uint32_t location;
  uint32_t binding;
  EPTextureFormat format;
  uint32_t offset;
} EPVertexInputAttributeDesc;

typedef struct EPVertexInputBindingDesc {
  uint32_t binding;
  uint32_t stride;
  bool per_instance;
} EPVertexInputBindingDesc;

typedef struct EPRasterStateDesc {
  bool depth_test_enable;
  bool depth_write_enable;
  EPCompareOp depth_compare;
} EPRasterStateDesc;

typedef struct EPRenderPipelineDesc {
  EPShaderLibraryPtr library;
  EPStringUTF8 vertex_entry;
  EPStringUTF8 fragment_entry;
  EPStringUTF8 mesh_entry;
  EPStringUTF8 task_entry;
  EPTextureFormat color_formats[8];
  uint32_t color_format_count;
  EPTextureFormat depth_format;
  EPTextureFormat stencil_format;
  const EPVertexInputAttributeDesc *attributes;
  uint32_t attribute_count;
  const EPVertexInputBindingDesc *bindings;
  uint32_t binding_count;
  EPRasterStateDesc raster_state;
  EPPipelineLayoutPtr layout;
} EPRenderPipelineDesc;

typedef struct EPComputePipelineDesc {
  EPShaderLibraryPtr library;
  EPStringUTF8 entry;
  EPPipelineLayoutPtr layout;
} EPComputePipelineDesc;

typedef struct EPMeshPipelineDesc {
  EPShaderLibraryPtr library;
  EPStringUTF8 mesh_entry;
  EPStringUTF8 task_entry;
  EPStringUTF8 fragment_entry;
  EPTextureFormat color_formats[8];
  uint32_t color_format_count;
  EPTextureFormat depth_format;
  EPTextureFormat stencil_format;
  EPRasterStateDesc raster_state;
  EPPipelineLayoutPtr layout;
} EPMeshPipelineDesc;

typedef struct EPRayTracingShaderGroupDesc {
  EPStringUTF8 raygen_entry;
  EPStringUTF8 miss_entry;
  EPStringUTF8 hit_entry;
  EPStringUTF8 any_hit_entry;
  EPStringUTF8 intersection_entry;
} EPRayTracingShaderGroupDesc;

typedef struct EPRayTracingPipelineDesc {
  EPShaderLibraryPtr library;
  uint32_t group_count;
  uint32_t max_recursion_depth;
  EPPipelineLayoutPtr layout;
  EPRayTracingShaderGroupDesc groups[];
} EPRayTracingPipelineDesc;

typedef struct EPAccelerationStructureGeometryDesc {
  EPBufferPtr vertex_buffer;
  uint64_t vertex_offset;
  uint32_t vertex_stride;
  uint32_t vertex_count;
  EPBufferPtr index_buffer;
  uint64_t index_offset;
  uint32_t index_count;
  bool opaque;
} EPAccelerationStructureGeometryDesc;

typedef struct EPAccelerationStructureDesc {
  uint32_t geometry_count;
  bool top_level;
  bool allow_update;
  EPAccelerationStructureGeometryDesc geometries[];
} EPAccelerationStructureDesc;

// ============================================================================
// Functions
// ============================================================================

EPError EPRenderPipelineCreate(EPDevicePtr device,
                               const EPRenderPipelineDesc *desc,
                               EPRenderPipelinePtr *out_pipeline);
EPError EPRenderPipelineDestroy(EPRenderPipelinePtr pipeline);

EPError EPComputePipelineCreate(EPDevicePtr device,
                                const EPComputePipelineDesc *desc,
                                EPComputePipelinePtr *out_pipeline);
EPError EPComputePipelineDestroy(EPComputePipelinePtr pipeline);

EPError EPMeshPipelineCreate(EPDevicePtr device,
                             const EPMeshPipelineDesc *desc,
                             EPMeshPipelinePtr *out_pipeline);
EPError EPMeshPipelineDestroy(EPMeshPipelinePtr pipeline);

EPError EPRayTracingPipelineCreate(EPDevicePtr device,
                                   const EPRayTracingPipelineDesc *desc,
                                   EPRayTracingPipelinePtr *out_pipeline);
EPError EPRayTracingPipelineDestroy(EPRayTracingPipelinePtr pipeline);

EPError EPAccelerationStructureCreate(EPDevicePtr device,
                                      const EPAccelerationStructureDesc *desc,
                                      EPAccelerationStructurePtr *out_as);
EPError EPAccelerationStructureDestroy(EPAccelerationStructurePtr as);

#ifdef __cplusplus
}
#endif

#endif
