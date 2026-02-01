// gcraft_internal.h - Internal structures and utilities for Metal backend (C++ / Metal-cpp)
// Requires macOS 15+/iOS 18+ for Metal 4 features

#ifndef GCRAFT_METAL_INTERNAL_H
#define GCRAFT_METAL_INTERNAL_H

#include <Foundation/Foundation.hpp>
#include <Metal/Metal.hpp>
#include <QuartzCore/QuartzCore.hpp>

#include <cstdlib>
#include <cstring>
#include <dispatch/dispatch.h>
#include <pthread.h>

#include "gcraft_types.h"
#include "gcraft_command.h"
#include "gcraft_descriptor.h"
#include "gcraft_device.h"
#include "gcraft_instance.h"
#include "gcraft_pipeline.h"
#include "gcraft_resource.h"
#include "gcraft_surface.h"
#include "gcraft_sync.h"

// Error helpers
inline GCError ep_ok() {
  return GCError{GC_E_OK, nullptr};
}
inline GCError ep_error(GCErrorCode code, const char* msg) {
  return GCError{code, msg};
}
inline GCError ep_invalid_argument(const char* msg) {
  return ep_error(GC_E_INVALID_ARGUMENT, msg);
}
inline GCError ep_out_of_memory() {
  return ep_error(GC_E_OUT_OF_MEMORY, "out of memory");
}
inline GCError ep_unsupported(const char* msg) {
  return ep_error(GC_E_UNSUPPORTED, msg);
}

// Format conversion (implemented in gcraft_internal.cpp)
MTL::PixelFormat ep_to_mtl_pixel_format(GCTextureFormat format);
MTL::SamplerMinMagFilter ep_to_mtl_filter(GCFilter filter);
MTL::SamplerAddressMode ep_to_mtl_address(GCAddressMode mode);
MTL::CompareFunction ep_to_mtl_compare(GCCompareOp op);
MTL::LoadAction ep_to_mtl_load_action(GCAttachmentLoadOp op);
MTL::StoreAction ep_to_mtl_store_action(GCAttachmentStoreOp op);
MTL::TextureType ep_to_mtl_texture_type(GCTextureDimension dim, uint32_t array_layers);
MTL::TextureUsage ep_to_mtl_texture_usage(GCTextureUsageFlags flags);

// Internal structures (C++ types from Metal-cpp)
struct GCPlatform {
  int placeholder;
};

struct GCInstance {
  GCBackendFlags enabled_backends;
  bool validation_enabled;
  bool debug_names_enabled;
  NS::Array* devices;
};

struct GCAdapter {
  MTL::Device* device;
  GCAdapterProperties properties;
};

struct GCDevice {
  MTL::Device* device;
  MTL::CommandQueue* graphics_queue;
  MTL::CommandQueue* compute_queue;
  MTL::CommandQueue* transfer_queue;
  GCFeatureFlags features;
  bool validation_enabled;
  bool debug_names_enabled;
};

struct GCQueue {
  GCDevice* ep_device;
  MTL::CommandQueue* queue;
  GCQueueType type;
};

struct GCCommandBuffer {
  GCDevice* ep_device;
  MTL::CommandBuffer* buffer;
  MTL::RenderCommandEncoder* render_encoder;
  MTL::ComputeCommandEncoder* compute_encoder;
  MTL::BlitCommandEncoder* blit_encoder;
  MTL::AccelerationStructureCommandEncoder* accel_encoder;
  bool is_recording;
};

struct GCFence {
  GCDevice* ep_device;
  MTL::SharedEvent* event;
  uint64_t value;
  pthread_mutex_t mutex;
};

struct GCTimelineSemaphore {
  GCDevice* ep_device;
  MTL::SharedEvent* event;
};

struct GCBuffer {
  GCDevice* ep_device;
  MTL::Buffer* buffer;
  uint64_t size;
  GCBufferUsageFlags usage;
  bool host_visible;
};

struct GCTexture {
  GCDevice* ep_device;
  MTL::Texture* texture;
  GCTextureDesc desc;
};

struct GCSampler {
  GCDevice* ep_device;
  MTL::SamplerState* sampler;
};

struct GCShaderLibrary {
  GCDevice* ep_device;
  MTL::Library* library;
};

struct GCDescriptorBindingInfo {
  uint32_t binding;
  GCDescriptorType type;
  uint32_t count;
  GCShaderStageFlags stages;
};

struct GCDescriptorSetLayout {
  GCDevice* ep_device;
  GCDescriptorBindingInfo* bindings;
  uint32_t binding_count;
};

union GCDescriptorValue {
  struct {
    MTL::Buffer* buffer;
    uint64_t offset;
    uint64_t range;
  } buffer_info;
  MTL::Texture* texture;
  MTL::SamplerState* sampler;
  MTL::AccelerationStructure* accel;
};

struct GCDescriptorSetEntry {
  GCDescriptorType type;
  GCDescriptorValue value;
};

struct GCDescriptorSet {
  GCDevice* ep_device;
  GCDescriptorSetLayout* layout;
  GCDescriptorSetEntry* entries;
  uint32_t entry_count;
};

struct GCPipelineLayout {
  GCDevice* ep_device;
  GCDescriptorSetLayout** set_layouts;
  uint32_t set_layout_count;
  uint32_t push_constant_size;
  GCShaderStageFlags push_constant_stages;
};

struct GCRenderPipeline {
  GCDevice* ep_device;
  MTL::RenderPipelineState* state;
  MTL::DepthStencilState* depth_stencil;
};

struct GCComputePipeline {
  GCDevice* ep_device;
  MTL::ComputePipelineState* state;
};

struct GCMeshPipeline {
  GCDevice* ep_device;
  MTL::RenderPipelineState* state;
  MTL::DepthStencilState* depth_stencil;
};

struct GCRayTracingPipeline {
  GCDevice* ep_device;
  NS::Array* states;
  MTL::VisibleFunctionTable* function_table;
  MTL::IntersectionFunctionTable* intersection_table;
  uint32_t max_recursion_depth;
};

struct GCAccelerationStructure {
  GCDevice* ep_device;
  MTL::AccelerationStructure* accel;
  MTL::Buffer* scratch_buffer;
  bool top_level;
};

struct GCSurface {
  GCNativeHandleType handle_type;
  void* handle;
  CA::MetalLayer* metal_layer;
};

struct GCSwapchain {
  GCDevice* ep_device;
  GCSurface* surface;
  CA::MetalLayer* layer;
  CA::MetalDrawable* current_drawable;
  NS::Array* textures;
  uint32_t image_count;
  uint32_t current_index;
  GCTextureFormat format;
  uint32_t width;
  uint32_t height;
};

#endif
