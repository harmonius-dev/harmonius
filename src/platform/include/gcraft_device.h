#ifndef GCRAFT_DEVICE_H
#define GCRAFT_DEVICE_H

#include "gcraft_instance.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Enumerations
// ============================================================================

typedef enum GCQueueType {
  GC_QUEUE_GRAPHICS = 0,
  GC_QUEUE_COMPUTE = 1,
  GC_QUEUE_TRANSFER = 2,
} GCQueueType;

// ============================================================================
// Structures
// ============================================================================

typedef struct GCDeviceDesc {
  GCBackend preferred_backend;
  GCFeatureFlags required_features;
  GCFeatureFlags optional_features;
  bool enable_validation;
  bool enable_debug_names;
} GCDeviceDesc;

typedef struct GCSubmitInfo {
  const GCCommandBufferPtr *command_buffers;
  uint32_t command_buffer_count;
  const GCTimelineSemaphorePtr *wait_semaphores;
  const uint64_t *wait_values;
  uint32_t wait_count;
  const GCTimelineSemaphorePtr *signal_semaphores;
  const uint64_t *signal_values;
  uint32_t signal_count;
  GCFencePtr fence;
} GCSubmitInfo;

// ============================================================================
// Functions
// ============================================================================

GCError GCDeviceCreate(GCAdapterPtr adapter, const GCDeviceDesc *desc,
                       GCDevicePtr *out_device);
GCError GCDeviceDestroy(GCDevicePtr device);
GCError GCDeviceGetQueue(GCDevicePtr device, GCQueueType type, uint32_t index,
                         GCQueuePtr *out_queue);

GCError GCQueueSubmit(GCQueuePtr queue, const GCSubmitInfo *submit);
GCError GCQueueWaitIdle(GCQueuePtr queue);

#ifdef __cplusplus
}
#endif

#endif
