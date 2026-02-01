// enthrall_internal.h - Internal structures and utilities for Metal backend
// Requires macOS 15+/iOS 18+ for Metal 4 features

#ifndef ENTHRALL_METAL_INTERNAL_H
#define ENTHRALL_METAL_INTERNAL_H

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

#include "enthrall_types.h"
#include "enthrall_command.h"
#include "enthrall_descriptor.h"
#include "enthrall_device.h"
#include "enthrall_instance.h"
#include "enthrall_pipeline.h"
#include "enthrall_resource.h"
#include "enthrall_surface.h"
#include "enthrall_sync.h"

#pragma mark - Error Helpers

static inline EPError ep_ok(void) {
  return (EPError){.code = EP_E_OK, .message = NULL};
}

static inline EPError ep_error(EPErrorCode code, const char *msg) {
  return (EPError){.code = code, .message = msg};
}

static inline EPError ep_invalid_argument(const char *msg) {
  return ep_error(EP_E_INVALID_ARGUMENT, msg);
}

static inline EPError ep_out_of_memory(void) {
  return ep_error(EP_E_OUT_OF_MEMORY, "out of memory");
}

static inline EPError ep_unsupported(const char *msg) {
  return ep_error(EP_E_UNSUPPORTED, msg);
}

#pragma mark - Format Conversion

MTLPixelFormat ep_to_mtl_pixel_format(EPTextureFormat format);
MTLSamplerMinMagFilter ep_to_mtl_filter(EPFilter filter);
MTLSamplerAddressMode ep_to_mtl_address(EPAddressMode mode);
MTLCompareFunction ep_to_mtl_compare(EPCompareOp op);
MTLLoadAction ep_to_mtl_load_action(EPAttachmentLoadOp op);
MTLStoreAction ep_to_mtl_store_action(EPAttachmentStoreOp op);
MTLTextureType ep_to_mtl_texture_type(EPTextureDimension dim,
                                      uint32_t array_layers);
MTLTextureUsage ep_to_mtl_texture_usage(EPTextureUsageFlags flags);

#pragma mark - Internal Structures

typedef struct EPPlatform {
  int placeholder;
} EPPlatform;

typedef struct EPInstance {
  EPBackendFlags enabled_backends;
  bool validation_enabled;
  bool debug_names_enabled;
  NSArray<id<MTLDevice>> *devices;
} EPInstance;

typedef struct EPAdapter {
  id<MTLDevice> device;
  EPAdapterProperties properties;
} EPAdapter;

typedef struct EPDevice {
  id<MTLDevice> device;
  id<MTLCommandQueue> graphics_queue;
  id<MTLCommandQueue> compute_queue;
  id<MTLCommandQueue> transfer_queue;
  EPFeatureFlags features;
  bool validation_enabled;
  bool debug_names_enabled;
} EPDevice;

typedef struct EPQueue {
  EPDevice *ep_device;
  id<MTLCommandQueue> queue;
  EPQueueType type;
} EPQueue;

typedef struct EPCommandBuffer {
  EPDevice *ep_device;
  id<MTLCommandBuffer> buffer;
  id<MTLRenderCommandEncoder> render_encoder;
  id<MTLComputeCommandEncoder> compute_encoder;
  id<MTLBlitCommandEncoder> blit_encoder;
  id<MTLAccelerationStructureCommandEncoder> accel_encoder;
  bool is_recording;
} EPCommandBuffer;

typedef struct EPFence {
  EPDevice *ep_device;
  id<MTLSharedEvent> event;
  uint64_t value;
  pthread_mutex_t mutex;
} EPFence;

typedef struct EPTimelineSemaphore {
  EPDevice *ep_device;
  id<MTLSharedEvent> event;
} EPTimelineSemaphore;

typedef struct EPBuffer {
  EPDevice *ep_device;
  id<MTLBuffer> buffer;
  uint64_t size;
  EPBufferUsageFlags usage;
  bool host_visible;
} EPBuffer;

typedef struct EPTexture {
  EPDevice *ep_device;
  id<MTLTexture> texture;
  EPTextureDesc desc;
} EPTexture;

typedef struct EPSampler {
  EPDevice *ep_device;
  id<MTLSamplerState> sampler;
} EPSampler;

typedef struct EPShaderLibrary {
  EPDevice *ep_device;
  id<MTLLibrary> library;
} EPShaderLibrary;

typedef struct EPDescriptorBindingInfo {
  uint32_t binding;
  EPDescriptorType type;
  uint32_t count;
  EPShaderStageFlags stages;
} EPDescriptorBindingInfo;

typedef struct EPDescriptorSetLayout {
  EPDevice *ep_device;
  EPDescriptorBindingInfo *bindings;
  uint32_t binding_count;
} EPDescriptorSetLayout;

typedef union EPDescriptorValue {
  struct {
    id<MTLBuffer> buffer;
    uint64_t offset;
    uint64_t range;
  } buffer_info;
  id<MTLTexture> texture;
  id<MTLSamplerState> sampler;
  id<MTLAccelerationStructure> accel;
} EPDescriptorValue;

typedef struct EPDescriptorSetEntry {
  EPDescriptorType type;
  EPDescriptorValue value;
} EPDescriptorSetEntry;

typedef struct EPDescriptorSet {
  EPDevice *ep_device;
  EPDescriptorSetLayout *layout;
  EPDescriptorSetEntry *entries;
  uint32_t entry_count;
} EPDescriptorSet;

typedef struct EPPipelineLayout {
  EPDevice *ep_device;
  EPDescriptorSetLayout **set_layouts;
  uint32_t set_layout_count;
  uint32_t push_constant_size;
  EPShaderStageFlags push_constant_stages;
} EPPipelineLayout;

typedef struct EPRenderPipeline {
  EPDevice *ep_device;
  id<MTLRenderPipelineState> state;
  id<MTLDepthStencilState> depth_stencil;
} EPRenderPipeline;

typedef struct EPComputePipeline {
  EPDevice *ep_device;
  id<MTLComputePipelineState> state;
} EPComputePipeline;

typedef struct EPMeshPipeline {
  EPDevice *ep_device;
  id<MTLRenderPipelineState> state;
  id<MTLDepthStencilState> depth_stencil;
} EPMeshPipeline;

typedef struct EPRayTracingPipeline {
  EPDevice *ep_device;
  NSArray<id<MTLComputePipelineState>> *states;
  id<MTLVisibleFunctionTable> function_table;
  id<MTLIntersectionFunctionTable> intersection_table;
  uint32_t max_recursion_depth;
} EPRayTracingPipeline;

typedef struct EPAccelerationStructure {
  EPDevice *ep_device;
  id<MTLAccelerationStructure> accel;
  id<MTLBuffer> scratch_buffer;
  bool top_level;
} EPAccelerationStructure;

typedef struct EPSurface {
  EPNativeHandleType handle_type;
  void *handle;
  CAMetalLayer *metal_layer;
} EPSurface;

typedef struct EPSwapchain {
  EPDevice *ep_device;
  EPSurface *surface;
  CAMetalLayer *layer;
  id<CAMetalDrawable> current_drawable;
  NSMutableArray<id<MTLTexture>> *textures;
  uint32_t image_count;
  uint32_t current_index;
  EPTextureFormat format;
  uint32_t width;
  uint32_t height;
} EPSwapchain;

#endif
