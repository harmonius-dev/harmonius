#ifndef GCRAFT_TYPES_H
#define GCRAFT_TYPES_H

#include <stddef.h>
#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Null-terminated UTF-8 encoded string. Caller retains ownership unless
/// otherwise documented.
typedef const char *GCStringUTF8;

/// Opaque native platform handle (e.g., CAMetalLayer, NSView, HWND).
typedef void *GCNativeHandle;

typedef enum GCErrorCode {
  GC_E_OK = 0,
  GC_E_OUT_OF_MEMORY = -1,
  GC_E_INVALID_ARGUMENT = -2,
  GC_E_INVALID_STATE = -3,
  GC_E_UNSUPPORTED = -4,
  GC_E_DEVICE_LOST = -5,
} GCErrorCode;

typedef struct GCError {
  GCErrorCode code;
  GCStringUTF8 message;
} GCError;

typedef struct GCPlatform *GCPlatformPtr;
typedef struct GCInstance *GCInstancePtr;
typedef struct GCAdapter *GCAdapterPtr;
typedef struct GCDevice *GCDevicePtr;
typedef struct GCQueue *GCQueuePtr;
typedef struct GCCommandBuffer *GCCommandBufferPtr;
typedef struct GCFence *GCFencePtr;
typedef struct GCTimelineSemaphore *GCTimelineSemaphorePtr;
typedef struct GCBuffer *GCBufferPtr;
typedef struct GCTexture *GCTexturePtr;
typedef struct GCSampler *GCSamplerPtr;
typedef struct GCShaderLibrary *GCShaderLibraryPtr;
typedef struct GCDescriptorSetLayout *GCDescriptorSetLayoutPtr;
typedef struct GCDescriptorSet *GCDescriptorSetPtr;
typedef struct GCPipelineLayout *GCPipelineLayoutPtr;
typedef struct GCRenderPipeline *GCRenderPipelinePtr;
typedef struct GCComputePipeline *GCComputePipelinePtr;
typedef struct GCMeshPipeline *GCMeshPipelinePtr;
typedef struct GCRayTracingPipeline *GCRayTracingPipelinePtr;
typedef struct GCAccelerationStructure *GCAccelerationStructurePtr;
typedef struct GCSurface *GCSurfacePtr;
typedef struct GCSwapchain *GCSwapchainPtr;

#ifdef __cplusplus
}
#endif

#endif
