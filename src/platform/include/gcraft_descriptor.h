#ifndef GCRAFT_DESCRIPTOR_H
#define GCRAFT_DESCRIPTOR_H

#include "gcraft_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Enumerations
// ============================================================================

typedef enum GCShaderStageFlags : uint32_t {
  GC_STAGE_VERTEX_BIT   = (1u << 0),
  GC_STAGE_FRAGMENT_BIT = (1u << 1),
  GC_STAGE_COMPUTE_BIT  = (1u << 2),
  GC_STAGE_MESH_BIT     = (1u << 3),
  GC_STAGE_TASK_BIT     = (1u << 4),
  GC_STAGE_RAYGEN_BIT   = (1u << 5),
  GC_STAGE_MISS_BIT     = (1u << 6),
  GC_STAGE_HIT_BIT      = (1u << 7),
} GCShaderStageFlags;

typedef enum GCDescriptorType {
  GC_DESCRIPTOR_UNIFORM_BUFFER = 0,
  GC_DESCRIPTOR_STORAGE_BUFFER = 1,
  GC_DESCRIPTOR_SAMPLED_TEXTURE = 2,
  GC_DESCRIPTOR_STORAGE_TEXTURE = 3,
  GC_DESCRIPTOR_SAMPLER = 4,
  GC_DESCRIPTOR_ACCELERATION_STRUCTURE = 5,
} GCDescriptorType;

// ============================================================================
// Structures
// ============================================================================

typedef struct GCDescriptorBindingDesc {
  uint32_t binding;
  GCDescriptorType type;
  uint32_t count;
  GCShaderStageFlags stages;
} GCDescriptorBindingDesc;

typedef struct GCDescriptorSetLayoutDesc {
  uint32_t binding_count;
  GCDescriptorBindingDesc bindings[];
} GCDescriptorSetLayoutDesc;

typedef struct GCPipelineLayoutDesc {
  uint32_t set_layout_count;
  uint32_t push_constant_size;
  GCShaderStageFlags push_constant_stages;
  GCDescriptorSetLayoutPtr set_layouts[];
} GCPipelineLayoutDesc;

// ============================================================================
// Functions
// ============================================================================

GCError GCDescriptorSetLayoutCreate(GCDevicePtr device,
                                    const GCDescriptorSetLayoutDesc *desc,
                                    GCDescriptorSetLayoutPtr *out_layout);
GCError GCDescriptorSetLayoutDestroy(GCDescriptorSetLayoutPtr layout);

GCError GCDescriptorSetCreate(GCDevicePtr device,
                              GCDescriptorSetLayoutPtr layout,
                              GCDescriptorSetPtr *out_set);
GCError GCDescriptorSetDestroy(GCDescriptorSetPtr set);
GCError GCDescriptorSetUpdateBuffer(GCDescriptorSetPtr set, uint32_t binding,
                                    GCBufferPtr buffer, uint64_t offset,
                                    uint64_t range);
GCError GCDescriptorSetUpdateTexture(GCDescriptorSetPtr set, uint32_t binding,
                                     GCTexturePtr texture);
GCError GCDescriptorSetUpdateSampler(GCDescriptorSetPtr set, uint32_t binding,
                                     GCSamplerPtr sampler);
GCError GCDescriptorSetUpdateAccelerationStructure(GCDescriptorSetPtr set,
                                                   uint32_t binding,
                                                   GCAccelerationStructurePtr as);

GCError GCPipelineLayoutCreate(GCDevicePtr device,
                               const GCPipelineLayoutDesc *desc,
                               GCPipelineLayoutPtr *out_layout);
GCError GCPipelineLayoutDestroy(GCPipelineLayoutPtr layout);

#ifdef __cplusplus
}
#endif

#endif
