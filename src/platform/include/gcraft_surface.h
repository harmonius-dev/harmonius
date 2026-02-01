#ifndef GCRAFT_SURFACE_H
#define GCRAFT_SURFACE_H

#include "gcraft_resource.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Enumerations
// ============================================================================

typedef enum GCNativeHandleType {
  GC_NATIVE_HANDLE_METAL_LAYER = 0,
  GC_NATIVE_HANDLE_NS_VIEW = 1,
  GC_NATIVE_HANDLE_UI_VIEW = 2,
  GC_NATIVE_HANDLE_HWND = 3,
  GC_NATIVE_HANDLE_XCB = 4,
  GC_NATIVE_HANDLE_WAYLAND = 5,
  GC_NATIVE_HANDLE_ANDROID_SURFACE = 6,
} GCNativeHandleType;

// ============================================================================
// Structures
// ============================================================================

typedef struct GCSurfaceDesc {
  GCNativeHandleType handle_type;
  GCNativeHandle handle;
  GCNativeHandle display;
} GCSurfaceDesc;

typedef struct GCSwapchainDesc {
  GCSurfacePtr surface;
  uint32_t width;
  uint32_t height;
  uint32_t image_count;
  GCTextureFormat format;
  bool vsync;
} GCSwapchainDesc;

// ============================================================================
// Functions
// ============================================================================

GCError GCSurfaceCreate(GCPlatformPtr platform, const GCSurfaceDesc *desc,
                        GCSurfacePtr *out_surface);
GCError GCSurfaceDestroy(GCSurfacePtr surface);

GCError GCSwapchainCreate(GCDevicePtr device, const GCSwapchainDesc *desc,
                          GCSwapchainPtr *out_swapchain);
GCError GCSwapchainDestroy(GCSwapchainPtr swapchain);
GCError GCSwapchainAcquireNext(GCSwapchainPtr swapchain,
                               GCTimelineSemaphorePtr signal_semaphore,
                               uint64_t signal_value,
                               uint32_t *out_image_index);
GCError GCSwapchainPresent(GCSwapchainPtr swapchain,
                           GCTimelineSemaphorePtr wait_semaphore,
                           uint64_t wait_value);

#ifdef __cplusplus
}
#endif

#endif
