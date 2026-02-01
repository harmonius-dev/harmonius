#ifndef GCRAFT_SYNC_H
#define GCRAFT_SYNC_H

#include "gcraft_types.h"

#ifdef __cplusplus
extern "C" {
#endif

GCError GCFenceCreate(GCDevicePtr device, uint64_t initial_value,
                      GCFencePtr *out_fence);
GCError GCFenceDestroy(GCFencePtr fence);
GCError GCFenceWait(GCFencePtr fence, uint64_t value, uint64_t timeout_ns);
GCError GCFenceSignal(GCFencePtr fence, uint64_t value);

GCError GCTimelineSemaphoreCreate(GCDevicePtr device, uint64_t initial_value,
                                  GCTimelineSemaphorePtr *out_semaphore);
GCError GCTimelineSemaphoreDestroy(GCTimelineSemaphorePtr semaphore);
GCError GCTimelineSemaphoreGetValue(GCTimelineSemaphorePtr semaphore,
                                    uint64_t *out_value);
GCError GCTimelineSemaphoreWait(GCTimelineSemaphorePtr semaphore,
                                uint64_t value, uint64_t timeout_ns);
GCError GCTimelineSemaphoreSignal(GCTimelineSemaphorePtr semaphore,
                                  uint64_t value);

#ifdef __cplusplus
}
#endif

#endif
