#ifndef GCRAFT_COMMAND_H
#define GCRAFT_COMMAND_H

#include "gcraft_resource.h"
#include "gcraft_descriptor.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Enumerations
// ============================================================================

typedef enum GCAttachmentLoadOp {
  GC_LOAD_OP_LOAD = 0,
  GC_LOAD_OP_CLEAR = 1,
  GC_LOAD_OP_DONT_CARE = 2,
} GCAttachmentLoadOp;

typedef enum GCAttachmentStoreOp {
  GC_STORE_OP_STORE = 0,
  GC_STORE_OP_DONT_CARE = 1,
} GCAttachmentStoreOp;

typedef enum GCPipelineStageFlags : uint32_t {
  GC_STAGE_TOP_OF_PIPE_BIT        = (1u << 0),
  GC_STAGE_DRAW_INDIRECT_BIT      = (1u << 1),
  GC_STAGE_VERTEX_INPUT_BIT       = (1u << 2),
  GC_STAGE_VERTEX_SHADER_BIT      = (1u << 3),
  GC_STAGE_FRAGMENT_SHADER_BIT    = (1u << 4),
  GC_STAGE_COMPUTE_SHADER_BIT     = (1u << 5),
  GC_STAGE_MESH_SHADER_BIT        = (1u << 6),
  GC_STAGE_RAY_TRACING_SHADER_BIT = (1u << 7),
  GC_STAGE_TRANSFER_BIT           = (1u << 8),
  GC_STAGE_BOTTOM_OF_PIPE_BIT     = (1u << 9),
} GCPipelineStageFlags;

typedef enum GCAccessFlags : uint32_t {
  GC_ACCESS_INDIRECT_COMMAND_READ_BIT  = (1u << 0),
  GC_ACCESS_VERTEX_READ_BIT            = (1u << 1),
  GC_ACCESS_INDEX_READ_BIT             = (1u << 2),
  GC_ACCESS_UNIFORM_READ_BIT           = (1u << 3),
  GC_ACCESS_SHADER_READ_BIT            = (1u << 4),
  GC_ACCESS_SHADER_WRITE_BIT           = (1u << 5),
  GC_ACCESS_COLOR_ATTACHMENT_READ_BIT  = (1u << 6),
  GC_ACCESS_COLOR_ATTACHMENT_WRITE_BIT = (1u << 7),
  GC_ACCESS_DEPTH_STENCIL_READ_BIT     = (1u << 8),
  GC_ACCESS_DEPTH_STENCIL_WRITE_BIT    = (1u << 9),
  GC_ACCESS_TRANSFER_READ_BIT          = (1u << 10),
  GC_ACCESS_TRANSFER_WRITE_BIT         = (1u << 11),
  GC_ACCESS_HOST_READ_BIT              = (1u << 12),
  GC_ACCESS_HOST_WRITE_BIT             = (1u << 13),
} GCAccessFlags;

typedef enum GCTextureLayout {
  GC_TEXTURE_LAYOUT_UNDEFINED = 0,
  GC_TEXTURE_LAYOUT_GENERAL = 1,
  GC_TEXTURE_LAYOUT_COLOR_ATTACHMENT = 2,
  GC_TEXTURE_LAYOUT_DEPTH_STENCIL = 3,
  GC_TEXTURE_LAYOUT_SHADER_READ = 4,
  GC_TEXTURE_LAYOUT_TRANSFER_SRC = 5,
  GC_TEXTURE_LAYOUT_TRANSFER_DST = 6,
  GC_TEXTURE_LAYOUT_PRESENT = 7,
} GCTextureLayout;

// ============================================================================
// Structures
// ============================================================================

typedef struct GCBufferBarrier {
  GCBufferPtr buffer;
  uint64_t offset;
  uint64_t size;
  GCAccessFlags src_access;
  GCAccessFlags dst_access;
} GCBufferBarrier;

typedef struct GCTextureBarrier {
  GCTexturePtr texture;
  GCTextureLayout old_layout;
  GCTextureLayout new_layout;
  GCAccessFlags src_access;
  GCAccessFlags dst_access;
} GCTextureBarrier;

typedef struct GCBarrierDesc {
  GCPipelineStageFlags src_stage;
  GCPipelineStageFlags dst_stage;
  const GCBufferBarrier *buffer_barriers;
  uint32_t buffer_barrier_count;
  const GCTextureBarrier *texture_barriers;
  uint32_t texture_barrier_count;
} GCBarrierDesc;

typedef struct GCRenderPassColorAttachment {
  GCTexturePtr texture;
  GCTextureLayout layout;
  GCAttachmentLoadOp load_op;
  GCAttachmentStoreOp store_op;
  float clear_color[4];
} GCRenderPassColorAttachment;

typedef struct GCRenderPassDepthAttachment {
  GCTexturePtr texture;
  GCTextureLayout layout;
  GCAttachmentLoadOp load_op;
  GCAttachmentStoreOp store_op;
  float clear_depth;
  uint32_t clear_stencil;
} GCRenderPassDepthAttachment;

typedef struct GCRenderPassDesc {
  uint32_t color_attachment_count;
  const GCRenderPassDepthAttachment *depth_attachment;
  GCRenderPassColorAttachment color_attachments[];
} GCRenderPassDesc;

// ============================================================================
// Functions
// ============================================================================

GCError GCCommandBufferCreate(GCDevicePtr device,
                              GCCommandBufferPtr *out_command_buffer);
GCError GCCommandBufferDestroy(GCCommandBufferPtr command_buffer);
GCError GCCommandBufferBegin(GCCommandBufferPtr command_buffer);
GCError GCCommandBufferEnd(GCCommandBufferPtr command_buffer);

GCError GCCommandBeginRenderPass(GCCommandBufferPtr command_buffer,
                                 const GCRenderPassDesc *desc);
GCError GCCommandEndRenderPass(GCCommandBufferPtr command_buffer);
GCError GCCommandBindRenderPipeline(GCCommandBufferPtr command_buffer,
                                    GCRenderPipelinePtr pipeline);
GCError GCCommandBindMeshPipeline(GCCommandBufferPtr command_buffer,
                                  GCMeshPipelinePtr pipeline);
GCError GCCommandBindComputePipeline(GCCommandBufferPtr command_buffer,
                                     GCComputePipelinePtr pipeline);
GCError GCCommandBindRayTracingPipeline(GCCommandBufferPtr command_buffer,
                                        GCRayTracingPipelinePtr pipeline);
GCError GCCommandBindDescriptorSet(GCCommandBufferPtr command_buffer,
                                   GCPipelineLayoutPtr layout, uint32_t set_index,
                                   GCDescriptorSetPtr set);
GCError GCCommandPushConstants(GCCommandBufferPtr command_buffer,
                               GCPipelineLayoutPtr layout,
                               GCShaderStageFlags stages,
                               const uint8_t *data, uint32_t size);
GCError GCCommandSetViewport(GCCommandBufferPtr command_buffer,
                             float x, float y, float width, float height,
                             float min_depth, float max_depth);
GCError GCCommandSetScissor(GCCommandBufferPtr command_buffer,
                            uint32_t x, uint32_t y, uint32_t width,
                            uint32_t height);
GCError GCCommandDraw(GCCommandBufferPtr command_buffer,
                      uint32_t vertex_count, uint32_t instance_count,
                      uint32_t first_vertex, uint32_t first_instance);
GCError GCCommandDrawIndexed(GCCommandBufferPtr command_buffer,
                             uint32_t index_count, uint32_t instance_count,
                             uint32_t first_index, int32_t vertex_offset,
                             uint32_t first_instance);
GCError GCCommandDispatch(GCCommandBufferPtr command_buffer,
                          uint32_t group_x, uint32_t group_y, uint32_t group_z);
GCError GCCommandDispatchMesh(GCCommandBufferPtr command_buffer,
                              uint32_t group_x, uint32_t group_y, uint32_t group_z);
GCError GCCommandDispatchRays(GCCommandBufferPtr command_buffer,
                              uint32_t width, uint32_t height, uint32_t depth);
GCError GCCommandResourceBarrier(GCCommandBufferPtr command_buffer,
                                 const GCBarrierDesc *desc);
GCError GCCommandCopyBuffer(GCCommandBufferPtr command_buffer,
                            GCBufferPtr src, uint64_t src_offset,
                            GCBufferPtr dst, uint64_t dst_offset,
                            uint64_t size);

#ifdef __cplusplus
}
#endif

#endif
