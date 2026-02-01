#ifndef ENTHRALL_SURFACE_H
#define ENTHRALL_SURFACE_H

#include "enthrall_resource.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Enumerations
// ============================================================================

typedef enum EPNativeHandleType {
  EP_NATIVE_HANDLE_METAL_LAYER = 0,
  EP_NATIVE_HANDLE_NS_VIEW = 1,
  EP_NATIVE_HANDLE_UI_VIEW = 2,
  EP_NATIVE_HANDLE_HWND = 3,
  EP_NATIVE_HANDLE_XCB = 4,
  EP_NATIVE_HANDLE_WAYLAND = 5,
  EP_NATIVE_HANDLE_ANDROID_SURFACE = 6,
} EPNativeHandleType;

// ============================================================================
// Structures
// ============================================================================

typedef struct EPSurfaceDesc {
  EPNativeHandleType handle_type;
  EPNativeHandle handle;
  EPNativeHandle display;
} EPSurfaceDesc;

typedef struct EPSwapchainDesc {
  EPSurfacePtr surface;
  uint32_t width;
  uint32_t height;
  uint32_t image_count;
  EPTextureFormat format;
  bool vsync;
} EPSwapchainDesc;

// ============================================================================
// Functions
// ============================================================================

EPError EPSurfaceCreate(EPPlatformPtr platform, const EPSurfaceDesc *desc,
                        EPSurfacePtr *out_surface);
EPError EPSurfaceDestroy(EPSurfacePtr surface);

EPError EPSwapchainCreate(EPDevicePtr device, const EPSwapchainDesc *desc,
                          EPSwapchainPtr *out_swapchain);
EPError EPSwapchainDestroy(EPSwapchainPtr swapchain);
EPError EPSwapchainAcquireNext(EPSwapchainPtr swapchain,
                               EPTimelineSemaphorePtr signal_semaphore,
                               uint64_t signal_value,
                               uint32_t *out_image_index);
EPError EPSwapchainPresent(EPSwapchainPtr swapchain,
                           EPTimelineSemaphorePtr wait_semaphore,
                           uint64_t wait_value);

#ifdef __cplusplus
}
#endif

#endif
