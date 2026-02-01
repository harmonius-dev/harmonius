#ifndef ENTHRALL_TYPES_H
#define ENTHRALL_TYPES_H

#include <stddef.h>
#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Null-terminated UTF-8 encoded string. Caller retains ownership unless
/// otherwise documented.
typedef const char *EPStringUTF8;

/// Opaque native platform handle (e.g., CAMetalLayer, NSView, HWND).
typedef void *EPNativeHandle;

typedef enum EPErrorCode {
  EP_E_OK = 0,
  EP_E_OUT_OF_MEMORY = -1,
  EP_E_INVALID_ARGUMENT = -2,
  EP_E_INVALID_STATE = -3,
  EP_E_UNSUPPORTED = -4,
  EP_E_DEVICE_LOST = -5,
} EPErrorCode;

typedef struct EPError {
  EPErrorCode code;
  EPStringUTF8 message;
} EPError;

typedef struct EPPlatform *EPPlatformPtr;
typedef struct EPInstance *EPInstancePtr;
typedef struct EPAdapter *EPAdapterPtr;
typedef struct EPDevice *EPDevicePtr;
typedef struct EPQueue *EPQueuePtr;
typedef struct EPCommandBuffer *EPCommandBufferPtr;
typedef struct EPFence *EPFencePtr;
typedef struct EPTimelineSemaphore *EPTimelineSemaphorePtr;
typedef struct EPBuffer *EPBufferPtr;
typedef struct EPTexture *EPTexturePtr;
typedef struct EPSampler *EPSamplerPtr;
typedef struct EPShaderLibrary *EPShaderLibraryPtr;
typedef struct EPDescriptorSetLayout *EPDescriptorSetLayoutPtr;
typedef struct EPDescriptorSet *EPDescriptorSetPtr;
typedef struct EPPipelineLayout *EPPipelineLayoutPtr;
typedef struct EPRenderPipeline *EPRenderPipelinePtr;
typedef struct EPComputePipeline *EPComputePipelinePtr;
typedef struct EPMeshPipeline *EPMeshPipelinePtr;
typedef struct EPRayTracingPipeline *EPRayTracingPipelinePtr;
typedef struct EPAccelerationStructure *EPAccelerationStructurePtr;
typedef struct EPSurface *EPSurfacePtr;
typedef struct EPSwapchain *EPSwapchainPtr;

#ifdef __cplusplus
}
#endif

#endif
