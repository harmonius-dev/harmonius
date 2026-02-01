// gcraft_sync.m - Fence and timeline semaphore implementation

#include "gcraft_internal.h"

#pragma mark - Fence

GCError GCFenceCreate(GCDevicePtr device, uint64_t initial_value, GCFencePtr *out_fence) {
    if (!device || !out_fence) return ep_invalid_argument("device or out_fence is NULL");

    @autoreleasepool {
        GCFence *fence = calloc(1, sizeof(GCFence));
        if (!fence) return ep_out_of_memory();

        fence->ep_device = device;
        fence->event = [[device->device newSharedEvent] retain];
        fence->event.signaledValue = initial_value;
        fence->value = initial_value;
        pthread_mutex_init(&fence->mutex, NULL);

        if (!fence->event) {
            free(fence);
            return ep_error(GC_E_DEVICE_LOST, "failed to create shared event");
        }

        *out_fence = fence;
        return ep_ok();
    }
}

GCError GCFenceDestroy(GCFencePtr fence) {
    if (fence) {
        @autoreleasepool {
            if (fence->event) [fence->event release];
        }
        pthread_mutex_destroy(&fence->mutex);
        free(fence);
    }
    return ep_ok();
}

GCError GCFenceWait(GCFencePtr fence, uint64_t value, uint64_t timeout_ns) {
    if (!fence) return ep_invalid_argument("fence is NULL");

    @autoreleasepool {
        dispatch_semaphore_t sema = dispatch_semaphore_create(0);
        MTLSharedEventListener *listener = [[MTLSharedEventListener alloc] init];

        [fence->event notifyListener:listener atValue:value block:^(id<MTLSharedEvent> ev, uint64_t val) {
            dispatch_semaphore_signal(sema);
        }];

        dispatch_time_t timeout = (timeout_ns == UINT64_MAX)
            ? DISPATCH_TIME_FOREVER
            : dispatch_time(DISPATCH_TIME_NOW, (int64_t)timeout_ns);

        long result = dispatch_semaphore_wait(sema, timeout);
        [listener release];

        if (result != 0) {
            return ep_error(GC_E_INVALID_STATE, "fence wait timed out");
        }
    }

    return ep_ok();
}

GCError GCFenceSignal(GCFencePtr fence, uint64_t value) {
    if (!fence) return ep_invalid_argument("fence is NULL");

    pthread_mutex_lock(&fence->mutex);
    fence->value = value;
    fence->event.signaledValue = value;
    pthread_mutex_unlock(&fence->mutex);

    return ep_ok();
}

#pragma mark - Timeline Semaphore

GCError GCTimelineSemaphoreCreate(GCDevicePtr device, uint64_t initial_value,
                                  GCTimelineSemaphorePtr *out_semaphore) {
    if (!device || !out_semaphore) return ep_invalid_argument("device or out_semaphore is NULL");

    @autoreleasepool {
        GCTimelineSemaphore *semaphore = calloc(1, sizeof(GCTimelineSemaphore));
        if (!semaphore) return ep_out_of_memory();

        semaphore->ep_device = device;
        semaphore->event = [[device->device newSharedEvent] retain];
        semaphore->event.signaledValue = initial_value;

        if (!semaphore->event) {
            free(semaphore);
            return ep_error(GC_E_DEVICE_LOST, "failed to create shared event");
        }

        *out_semaphore = semaphore;
        return ep_ok();
    }
}

GCError GCTimelineSemaphoreDestroy(GCTimelineSemaphorePtr semaphore) {
    if (semaphore) {
        @autoreleasepool {
            if (semaphore->event) [semaphore->event release];
        }
        free(semaphore);
    }
    return ep_ok();
}

GCError GCTimelineSemaphoreGetValue(GCTimelineSemaphorePtr semaphore, uint64_t *out_value) {
    if (!semaphore || !out_value) return ep_invalid_argument("semaphore or out_value is NULL");
    *out_value = semaphore->event.signaledValue;
    return ep_ok();
}

GCError GCTimelineSemaphoreWait(GCTimelineSemaphorePtr semaphore, uint64_t value, uint64_t timeout_ns) {
    if (!semaphore) return ep_invalid_argument("semaphore is NULL");

    @autoreleasepool {
        dispatch_semaphore_t sema = dispatch_semaphore_create(0);
        MTLSharedEventListener *listener = [[MTLSharedEventListener alloc] init];

        [semaphore->event notifyListener:listener atValue:value block:^(id<MTLSharedEvent> ev, uint64_t val) {
            dispatch_semaphore_signal(sema);
        }];

        dispatch_time_t timeout = (timeout_ns == UINT64_MAX)
            ? DISPATCH_TIME_FOREVER
            : dispatch_time(DISPATCH_TIME_NOW, (int64_t)timeout_ns);

        long result = dispatch_semaphore_wait(sema, timeout);
        [listener release];

        if (result != 0) {
            return ep_error(GC_E_INVALID_STATE, "semaphore wait timed out");
        }
    }

    return ep_ok();
}

GCError GCTimelineSemaphoreSignal(GCTimelineSemaphorePtr semaphore, uint64_t value) {
    if (!semaphore) return ep_invalid_argument("semaphore is NULL");
    semaphore->event.signaledValue = value;
    return ep_ok();
}
