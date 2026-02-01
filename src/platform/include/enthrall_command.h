#ifndef ENTHRALL_COMMAND_H
#define ENTHRALL_COMMAND_H

#include "enthrall_resource.h"
#include "enthrall_descriptor.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Enumerations
// ============================================================================

typedef enum EPAttachmentLoadOp {
  EP_LOAD_OP_LOAD = 0,
  EP_LOAD_OP_CLEAR = 1,
  EP_LOAD_OP_DONT_CARE = 2,
} EPAttachmentLoadOp;

typedef enum EPAttachmentStoreOp {
  EP_STORE_OP_STORE = 0,
  EP_STORE_OP_DONT_CARE = 1,
} EPAttachmentStoreOp;

typedef enum EPPipelineStageFlags : uint32_t {
  EP_STAGE_TOP_OF_PIPE_BIT        = (1u << 0),
  EP_STAGE_DRAW_INDIRECT_BIT      = (1u << 1),
  EP_STAGE_VERTEX_INPUT_BIT       = (1u << 2),
  EP_STAGE_VERTEX_SHADER_BIT      = (1u << 3),
  EP_STAGE_FRAGMENT_SHADER_BIT    = (1u << 4),
  EP_STAGE_COMPUTE_SHADER_BIT     = (1u << 5),
  EP_STAGE_MESH_SHADER_BIT        = (1u << 6),
  EP_STAGE_RAY_TRACING_SHADER_BIT = (1u << 7),
  EP_STAGE_TRANSFER_BIT           = (1u << 8),
  EP_STAGE_BOTTOM_OF_PIPE_BIT     = (1u << 9),
} EPPipelineStageFlags;

typedef enum EPAccessFlags : uint32_t {
  EP_ACCESS_INDIRECT_COMMAND_READ_BIT  = (1u << 0),
  EP_ACCESS_VERTEX_READ_BIT            = (1u << 1),
  EP_ACCESS_INDEX_READ_BIT             = (1u << 2),
  EP_ACCESS_UNIFORM_READ_BIT           = (1u << 3),
  EP_ACCESS_SHADER_READ_BIT            = (1u << 4),
  EP_ACCESS_SHADER_WRITE_BIT           = (1u << 5),
  EP_ACCESS_COLOR_ATTACHMENT_READ_BIT  = (1u << 6),
  EP_ACCESS_COLOR_ATTACHMENT_WRITE_BIT = (1u << 7),
  EP_ACCESS_DEPTH_STENCIL_READ_BIT     = (1u << 8),
  EP_ACCESS_DEPTH_STENCIL_WRITE_BIT    = (1u << 9),
  EP_ACCESS_TRANSFER_READ_BIT          = (1u << 10),
  EP_ACCESS_TRANSFER_WRITE_BIT         = (1u << 11),
  EP_ACCESS_HOST_READ_BIT              = (1u << 12),
  EP_ACCESS_HOST_WRITE_BIT             = (1u << 13),
} EPAccessFlags;

typedef enum EPTextureLayout {
  EP_TEXTURE_LAYOUT_UNDEFINED = 0,
  EP_TEXTURE_LAYOUT_GENERAL = 1,
  EP_TEXTURE_LAYOUT_COLOR_ATTACHMENT = 2,
  EP_TEXTURE_LAYOUT_DEPTH_STENCIL = 3,
  EP_TEXTURE_LAYOUT_SHADER_READ = 4,
  EP_TEXTURE_LAYOUT_TRANSFER_SRC = 5,
  EP_TEXTURE_LAYOUT_TRANSFER_DST = 6,
  EP_TEXTURE_LAYOUT_PRESENT = 7,
} EPTextureLayout;

// ============================================================================
// Structures
// ============================================================================

typedef struct EPBufferBarrier {
  EPBufferPtr buffer;
  uint64_t offset;
  uint64_t size;
  EPAccessFlags src_access;
  EPAccessFlags dst_access;
} EPBufferBarrier;

typedef struct EPTextureBarrier {
  EPTexturePtr texture;
  EPTextureLayout old_layout;
  EPTextureLayout new_layout;
  EPAccessFlags src_access;
  EPAccessFlags dst_access;
} EPTextureBarrier;

typedef struct EPBarrierDesc {
  EPPipelineStageFlags src_stage;
  EPPipelineStageFlags dst_stage;
  const EPBufferBarrier *buffer_barriers;
  uint32_t buffer_barrier_count;
  const EPTextureBarrier *texture_barriers;
  uint32_t texture_barrier_count;
} EPBarrierDesc;

typedef struct EPRenderPassColorAttachment {
  EPTexturePtr texture;
  EPTextureLayout layout;
  EPAttachmentLoadOp load_op;
  EPAttachmentStoreOp store_op;
  float clear_color[4];
} EPRenderPassColorAttachment;

typedef struct EPRenderPassDepthAttachment {
  EPTexturePtr texture;
  EPTextureLayout layout;
  EPAttachmentLoadOp load_op;
  EPAttachmentStoreOp store_op;
  float clear_depth;
  uint32_t clear_stencil;
} EPRenderPassDepthAttachment;

typedef struct EPRenderPassDesc {
  uint32_t color_attachment_count;
  const EPRenderPassDepthAttachment *depth_attachment;
  EPRenderPassColorAttachment color_attachments[];
} EPRenderPassDesc;

// ============================================================================
// Functions
// ============================================================================

EPError EPCommandBufferCreate(EPDevicePtr device,
                              EPCommandBufferPtr *out_command_buffer);
EPError EPCommandBufferDestroy(EPCommandBufferPtr command_buffer);
EPError EPCommandBufferBegin(EPCommandBufferPtr command_buffer);
EPError EPCommandBufferEnd(EPCommandBufferPtr command_buffer);

EPError EPCommandBeginRenderPass(EPCommandBufferPtr command_buffer,
                                 const EPRenderPassDesc *desc);
EPError EPCommandEndRenderPass(EPCommandBufferPtr command_buffer);
EPError EPCommandBindRenderPipeline(EPCommandBufferPtr command_buffer,
                                    EPRenderPipelinePtr pipeline);
EPError EPCommandBindMeshPipeline(EPCommandBufferPtr command_buffer,
                                  EPMeshPipelinePtr pipeline);
EPError EPCommandBindComputePipeline(EPCommandBufferPtr command_buffer,
                                     EPComputePipelinePtr pipeline);
EPError EPCommandBindRayTracingPipeline(EPCommandBufferPtr command_buffer,
                                        EPRayTracingPipelinePtr pipeline);
EPError EPCommandBindDescriptorSet(EPCommandBufferPtr command_buffer,
                                   EPPipelineLayoutPtr layout, uint32_t set_index,
                                   EPDescriptorSetPtr set);
EPError EPCommandPushConstants(EPCommandBufferPtr command_buffer,
                               EPPipelineLayoutPtr layout,
                               EPShaderStageFlags stages,
                               const uint8_t *data, uint32_t size);
EPError EPCommandSetViewport(EPCommandBufferPtr command_buffer,
                             float x, float y, float width, float height,
                             float min_depth, float max_depth);
EPError EPCommandSetScissor(EPCommandBufferPtr command_buffer,
                            uint32_t x, uint32_t y, uint32_t width,
                            uint32_t height);
EPError EPCommandDraw(EPCommandBufferPtr command_buffer,
                      uint32_t vertex_count, uint32_t instance_count,
                      uint32_t first_vertex, uint32_t first_instance);
EPError EPCommandDrawIndexed(EPCommandBufferPtr command_buffer,
                             uint32_t index_count, uint32_t instance_count,
                             uint32_t first_index, int32_t vertex_offset,
                             uint32_t first_instance);
EPError EPCommandDispatch(EPCommandBufferPtr command_buffer,
                          uint32_t group_x, uint32_t group_y, uint32_t group_z);
EPError EPCommandDispatchMesh(EPCommandBufferPtr command_buffer,
                              uint32_t group_x, uint32_t group_y, uint32_t group_z);
EPError EPCommandDispatchRays(EPCommandBufferPtr command_buffer,
                              uint32_t width, uint32_t height, uint32_t depth);
EPError EPCommandResourceBarrier(EPCommandBufferPtr command_buffer,
                                 const EPBarrierDesc *desc);

#ifdef __cplusplus
}
#endif

#endif
