#ifndef ENTHRALL_DESCRIPTOR_H
#define ENTHRALL_DESCRIPTOR_H

#include "enthrall_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Enumerations
// ============================================================================

typedef enum EPShaderStageFlags : uint32_t {
  EP_STAGE_VERTEX_BIT   = (1u << 0),
  EP_STAGE_FRAGMENT_BIT = (1u << 1),
  EP_STAGE_COMPUTE_BIT  = (1u << 2),
  EP_STAGE_MESH_BIT     = (1u << 3),
  EP_STAGE_TASK_BIT     = (1u << 4),
  EP_STAGE_RAYGEN_BIT   = (1u << 5),
  EP_STAGE_MISS_BIT     = (1u << 6),
  EP_STAGE_HIT_BIT      = (1u << 7),
} EPShaderStageFlags;

typedef enum EPDescriptorType {
  EP_DESCRIPTOR_UNIFORM_BUFFER = 0,
  EP_DESCRIPTOR_STORAGE_BUFFER = 1,
  EP_DESCRIPTOR_SAMPLED_TEXTURE = 2,
  EP_DESCRIPTOR_STORAGE_TEXTURE = 3,
  EP_DESCRIPTOR_SAMPLER = 4,
  EP_DESCRIPTOR_ACCELERATION_STRUCTURE = 5,
} EPDescriptorType;

// ============================================================================
// Structures
// ============================================================================

typedef struct EPDescriptorBindingDesc {
  uint32_t binding;
  EPDescriptorType type;
  uint32_t count;
  EPShaderStageFlags stages;
} EPDescriptorBindingDesc;

typedef struct EPDescriptorSetLayoutDesc {
  uint32_t binding_count;
  EPDescriptorBindingDesc bindings[];
} EPDescriptorSetLayoutDesc;

typedef struct EPPipelineLayoutDesc {
  uint32_t set_layout_count;
  uint32_t push_constant_size;
  EPShaderStageFlags push_constant_stages;
  EPDescriptorSetLayoutPtr set_layouts[];
} EPPipelineLayoutDesc;

// ============================================================================
// Functions
// ============================================================================

EPError EPDescriptorSetLayoutCreate(EPDevicePtr device,
                                    const EPDescriptorSetLayoutDesc *desc,
                                    EPDescriptorSetLayoutPtr *out_layout);
EPError EPDescriptorSetLayoutDestroy(EPDescriptorSetLayoutPtr layout);

EPError EPDescriptorSetCreate(EPDevicePtr device,
                              EPDescriptorSetLayoutPtr layout,
                              EPDescriptorSetPtr *out_set);
EPError EPDescriptorSetDestroy(EPDescriptorSetPtr set);
EPError EPDescriptorSetUpdateBuffer(EPDescriptorSetPtr set, uint32_t binding,
                                    EPBufferPtr buffer, uint64_t offset,
                                    uint64_t range);
EPError EPDescriptorSetUpdateTexture(EPDescriptorSetPtr set, uint32_t binding,
                                     EPTexturePtr texture);
EPError EPDescriptorSetUpdateSampler(EPDescriptorSetPtr set, uint32_t binding,
                                     EPSamplerPtr sampler);
EPError EPDescriptorSetUpdateAccelerationStructure(EPDescriptorSetPtr set,
                                                   uint32_t binding,
                                                   EPAccelerationStructurePtr as);

EPError EPPipelineLayoutCreate(EPDevicePtr device,
                               const EPPipelineLayoutDesc *desc,
                               EPPipelineLayoutPtr *out_layout);
EPError EPPipelineLayoutDestroy(EPPipelineLayoutPtr layout);

#ifdef __cplusplus
}
#endif

#endif
