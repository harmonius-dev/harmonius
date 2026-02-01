#ifndef ENTHRALL_DEVICE_H
#define ENTHRALL_DEVICE_H

#include "enthrall_instance.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Enumerations
// ============================================================================

typedef enum EPQueueType {
  EP_QUEUE_GRAPHICS = 0,
  EP_QUEUE_COMPUTE = 1,
  EP_QUEUE_TRANSFER = 2,
} EPQueueType;

// ============================================================================
// Structures
// ============================================================================

typedef struct EPDeviceDesc {
  EPBackend preferred_backend;
  EPFeatureFlags required_features;
  EPFeatureFlags optional_features;
  bool enable_validation;
  bool enable_debug_names;
} EPDeviceDesc;

typedef struct EPSubmitInfo {
  const EPCommandBufferPtr *command_buffers;
  uint32_t command_buffer_count;
  const EPTimelineSemaphorePtr *wait_semaphores;
  const uint64_t *wait_values;
  uint32_t wait_count;
  const EPTimelineSemaphorePtr *signal_semaphores;
  const uint64_t *signal_values;
  uint32_t signal_count;
  EPFencePtr fence;
} EPSubmitInfo;

// ============================================================================
// Functions
// ============================================================================

EPError EPDeviceCreate(EPAdapterPtr adapter, const EPDeviceDesc *desc,
                       EPDevicePtr *out_device);
EPError EPDeviceDestroy(EPDevicePtr device);
EPError EPDeviceGetQueue(EPDevicePtr device, EPQueueType type, uint32_t index,
                         EPQueuePtr *out_queue);

EPError EPQueueSubmit(EPQueuePtr queue, const EPSubmitInfo *submit);
EPError EPQueueWaitIdle(EPQueuePtr queue);

#ifdef __cplusplus
}
#endif

#endif
