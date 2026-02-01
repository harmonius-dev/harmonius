// gcraft_internal.h - Internal structures and utilities for Metal backend
// Requires macOS 15+/iOS 18+ for Metal 4 features

#ifndef GCRAFT_METAL_INTERNAL_H
#define GCRAFT_METAL_INTERNAL_H

#import <Metal/Metal.h>
#import <QuartzCore/CAMetalLayer.h>

#if TARGET_OS_OSX
#import <AppKit/AppKit.h>
#else
#import <UIKit/UIKit.h>
#endif

#include <dispatch/dispatch.h>
#include <pthread.h>
#include <stdlib.h>
#include <string.h>

#include "gcraft_types.h"
#include "gcraft_command.h"
#include "gcraft_descriptor.h"
#include "gcraft_device.h"
#include "gcraft_instance.h"
#include "gcraft_pipeline.h"
#include "gcraft_resource.h"
#include "gcraft_surface.h"
#include "gcraft_sync.h"

#pragma mark - Error Helpers

static inline GCError ep_ok(void) {
  return (GCError){.code = GC_E_OK, .message = NULL};
}

static inline GCError ep_error(GCErrorCode code, const char *msg) {
  return (GCError){.code = code, .message = msg};
}

static inline GCError ep_invalid_argument(const char *msg) {
  return ep_error(GC_E_INVALID_ARGUMENT, msg);
}

static inline GCError ep_out_of_memory(void) {
  return ep_error(GC_E_OUT_OF_MEMORY, "out of memory");
}

static inline GCError ep_unsupported(const char *msg) {
  return ep_error(GC_E_UNSUPPORTED, msg);
}

#pragma mark - Format Conversion

MTLPixelFormat ep_to_mtl_pixel_format(GCTextureFormat format);
MTLSamplerMinMagFilter ep_to_mtl_filter(GCFilter filter);
MTLSamplerAddressMode ep_to_mtl_address(GCAddressMode mode);
MTLCompareFunction ep_to_mtl_compare(GCCompareOp op);
MTLLoadAction ep_to_mtl_load_action(GCAttachmentLoadOp op);
MTLStoreAction ep_to_mtl_store_action(GCAttachmentStoreOp op);
MTLTextureType ep_to_mtl_texture_type(GCTextureDimension dim,
                                      uint32_t array_layers);
MTLTextureUsage ep_to_mtl_texture_usage(GCTextureUsageFlags flags);

#pragma mark - Internal Structures

typedef struct GCPlatform {
  int placeholder;
} GCPlatform;

typedef struct GCInstance {
  GCBackendFlags enabled_backends;
  bool validation_enabled;
  bool debug_names_enabled;
  NSArray<id<MTLDevice>> *devices;
} GCInstance;

typedef struct GCAdapter {
  id<MTLDevice> device;
  GCAdapterProperties properties;
} GCAdapter;

typedef struct GCDevice {
  id<MTLDevice> device;
  id<MTLCommandQueue> graphics_queue;
  id<MTLCommandQueue> compute_queue;
  id<MTLCommandQueue> transfer_queue;
  GCFeatureFlags features;
  bool validation_enabled;
  bool debug_names_enabled;
} GCDevice;

typedef struct GCQueue {
  GCDevice *ep_device;
  id<MTLCommandQueue> queue;
  GCQueueType type;
} GCQueue;

typedef struct GCCommandBuffer {
  GCDevice *ep_device;
  id<MTLCommandBuffer> buffer;
  id<MTLRenderCommandEncoder> render_encoder;
  id<MTLComputeCommandEncoder> compute_encoder;
  id<MTLBlitCommandEncoder> blit_encoder;
  id<MTLAccelerationStructureCommandEncoder> accel_encoder;
  bool is_recording;
} GCCommandBuffer;

typedef struct GCFence {
  GCDevice *ep_device;
  id<MTLSharedEvent> event;
  uint64_t value;
  pthread_mutex_t mutex;
} GCFence;

typedef struct GCTimelineSemaphore {
  GCDevice *ep_device;
  id<MTLSharedEvent> event;
} GCTimelineSemaphore;

typedef struct GCBuffer {
  GCDevice *ep_device;
  id<MTLBuffer> buffer;
  uint64_t size;
  GCBufferUsageFlags usage;
  bool host_visible;
} GCBuffer;

typedef struct GCTexture {
  GCDevice *ep_device;
  id<MTLTexture> texture;
  GCTextureDesc desc;
} GCTexture;

typedef struct GCSampler {
  GCDevice *ep_device;
  id<MTLSamplerState> sampler;
} GCSampler;

typedef struct GCShaderLibrary {
  GCDevice *ep_device;
  id<MTLLibrary> library;
} GCShaderLibrary;

typedef struct GCDescriptorBindingInfo {
  uint32_t binding;
  GCDescriptorType type;
  uint32_t count;
  GCShaderStageFlags stages;
} GCDescriptorBindingInfo;

typedef struct GCDescriptorSetLayout {
  GCDevice *ep_device;
  GCDescriptorBindingInfo *bindings;
  uint32_t binding_count;
} GCDescriptorSetLayout;

typedef union GCDescriptorValue {
  struct {
    id<MTLBuffer> buffer;
    uint64_t offset;
    uint64_t range;
  } buffer_info;
  id<MTLTexture> texture;
  id<MTLSamplerState> sampler;
  id<MTLAccelerationStructure> accel;
} GCDescriptorValue;

typedef struct GCDescriptorSetEntry {
  GCDescriptorType type;
  GCDescriptorValue value;
} GCDescriptorSetEntry;

typedef struct GCDescriptorSet {
  GCDevice *ep_device;
  GCDescriptorSetLayout *layout;
  GCDescriptorSetEntry *entries;
  uint32_t entry_count;
} GCDescriptorSet;

typedef struct GCPipelineLayout {
  GCDevice *ep_device;
  GCDescriptorSetLayout **set_layouts;
  uint32_t set_layout_count;
  uint32_t push_constant_size;
  GCShaderStageFlags push_constant_stages;
} GCPipelineLayout;

typedef struct GCRenderPipeline {
  GCDevice *ep_device;
  id<MTLRenderPipelineState> state;
  id<MTLDepthStencilState> depth_stencil;
} GCRenderPipeline;

typedef struct GCComputePipeline {
  GCDevice *ep_device;
  id<MTLComputePipelineState> state;
} GCComputePipeline;

typedef struct GCMeshPipeline {
  GCDevice *ep_device;
  id<MTLRenderPipelineState> state;
  id<MTLDepthStencilState> depth_stencil;
} GCMeshPipeline;

typedef struct GCRayTracingPipeline {
  GCDevice *ep_device;
  NSArray<id<MTLComputePipelineState>> *states;
  id<MTLVisibleFunctionTable> function_table;
  id<MTLIntersectionFunctionTable> intersection_table;
  uint32_t max_recursion_depth;
} GCRayTracingPipeline;

typedef struct GCAccelerationStructure {
  GCDevice *ep_device;
  id<MTLAccelerationStructure> accel;
  id<MTLBuffer> scratch_buffer;
  bool top_level;
} GCAccelerationStructure;

typedef struct GCSurface {
  GCNativeHandleType handle_type;
  void *handle;
  CAMetalLayer *metal_layer;
} GCSurface;

typedef struct GCSwapchain {
  GCDevice *ep_device;
  GCSurface *surface;
  CAMetalLayer *layer;
  id<CAMetalDrawable> current_drawable;
  NSMutableArray<id<MTLTexture>> *textures;
  uint32_t image_count;
  uint32_t current_index;
  GCTextureFormat format;
  uint32_t width;
  uint32_t height;
} GCSwapchain;

#endif
