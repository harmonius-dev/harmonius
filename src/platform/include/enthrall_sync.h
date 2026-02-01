#ifndef ENTHRALL_SYNC_H
#define ENTHRALL_SYNC_H

#include "enthrall_types.h"

#ifdef __cplusplus
extern "C" {
#endif

EPError EPFenceCreate(EPDevicePtr device, uint64_t initial_value,
                      EPFencePtr *out_fence);
EPError EPFenceDestroy(EPFencePtr fence);
EPError EPFenceWait(EPFencePtr fence, uint64_t value, uint64_t timeout_ns);
EPError EPFenceSignal(EPFencePtr fence, uint64_t value);

EPError EPTimelineSemaphoreCreate(EPDevicePtr device, uint64_t initial_value,
                                  EPTimelineSemaphorePtr *out_semaphore);
EPError EPTimelineSemaphoreDestroy(EPTimelineSemaphorePtr semaphore);
EPError EPTimelineSemaphoreGetValue(EPTimelineSemaphorePtr semaphore,
                                    uint64_t *out_value);
EPError EPTimelineSemaphoreWait(EPTimelineSemaphorePtr semaphore,
                                uint64_t value, uint64_t timeout_ns);
EPError EPTimelineSemaphoreSignal(EPTimelineSemaphorePtr semaphore,
                                  uint64_t value);

#ifdef __cplusplus
}
#endif

#endif
