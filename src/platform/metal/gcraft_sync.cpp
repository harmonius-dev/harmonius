// gcraft_sync.cpp - Fence and timeline semaphore (Metal-cpp)

#include "gcraft_internal.h"

extern "C" {

GCError GCFenceCreate(GCDevicePtr device, uint64_t initial_value, GCFencePtr* out_fence) {
  if (!device || !out_fence) return ep_invalid_argument("device or out_fence is NULL");

  GCDevice* dev = static_cast<GCDevice*>(device);
  GCFence* fence = static_cast<GCFence*>(calloc(1, sizeof(GCFence)));
  if (!fence) return ep_out_of_memory();

  fence->ep_device = dev;
  fence->event = dev->device->newSharedEvent();
  if (!fence->event) {
    free(fence);
    return ep_error(GC_E_DEVICE_LOST, "failed to create shared event");
  }
  fence->event->setSignaledValue(initial_value);
  fence->value = initial_value;
  pthread_mutex_init(&fence->mutex, nullptr);

  *out_fence = fence;
  return ep_ok();
}

GCError GCFenceDestroy(GCFencePtr fence) {
  if (fence) {
    GCFence* f = static_cast<GCFence*>(fence);
    if (f->event) f->event->release();
    pthread_mutex_destroy(&f->mutex);
    free(fence);
  }
  return ep_ok();
}

GCError GCFenceWait(GCFencePtr fence, uint64_t value, uint64_t timeout_ns) {
  if (!fence) return ep_invalid_argument("fence is NULL");

  GCFence* f = static_cast<GCFence*>(fence);
  dispatch_semaphore_t sema = dispatch_semaphore_create(0);
  MTL::SharedEventListener* listener = MTL::SharedEventListener::alloc()->init();

  f->event->notifyListener(listener, value, ^(MTL::SharedEvent* ev, uint64_t val) {
    (void)ev;
    (void)val;
    dispatch_semaphore_signal(sema);
  });

  dispatch_time_t timeout = (timeout_ns == UINT64_MAX)
    ? DISPATCH_TIME_FOREVER
    : dispatch_time(DISPATCH_TIME_NOW, static_cast<int64_t>(timeout_ns));

  long result = dispatch_semaphore_wait(sema, timeout);
  listener->release();

  if (result != 0) {
    return ep_error(GC_E_INVALID_STATE, "fence wait timed out");
  }
  return ep_ok();
}

GCError GCFenceSignal(GCFencePtr fence, uint64_t value) {
  if (!fence) return ep_invalid_argument("fence is NULL");

  GCFence* f = static_cast<GCFence*>(fence);
  pthread_mutex_lock(&f->mutex);
  f->value = value;
  f->event->setSignaledValue(value);
  pthread_mutex_unlock(&f->mutex);
  return ep_ok();
}

GCError GCTimelineSemaphoreCreate(GCDevicePtr device, uint64_t initial_value,
                                  GCTimelineSemaphorePtr* out_semaphore) {
  if (!device || !out_semaphore) return ep_invalid_argument("device or out_semaphore is NULL");

  GCDevice* dev = static_cast<GCDevice*>(device);
  GCTimelineSemaphore* semaphore = static_cast<GCTimelineSemaphore*>(calloc(1, sizeof(GCTimelineSemaphore)));
  if (!semaphore) return ep_out_of_memory();

  semaphore->ep_device = dev;
  semaphore->event = dev->device->newSharedEvent();
  if (!semaphore->event) {
    free(semaphore);
    return ep_error(GC_E_DEVICE_LOST, "failed to create shared event");
  }
  semaphore->event->setSignaledValue(initial_value);

  *out_semaphore = semaphore;
  return ep_ok();
}

GCError GCTimelineSemaphoreDestroy(GCTimelineSemaphorePtr semaphore) {
  if (semaphore) {
    GCTimelineSemaphore* sem = static_cast<GCTimelineSemaphore*>(semaphore);
    if (sem->event) sem->event->release();
    free(semaphore);
  }
  return ep_ok();
}

GCError GCTimelineSemaphoreGetValue(GCTimelineSemaphorePtr semaphore, uint64_t* out_value) {
  if (!semaphore || !out_value) return ep_invalid_argument("semaphore or out_value is NULL");
  GCTimelineSemaphore* sem = static_cast<GCTimelineSemaphore*>(semaphore);
  *out_value = sem->event->signaledValue();
  return ep_ok();
}

GCError GCTimelineSemaphoreWait(GCTimelineSemaphorePtr semaphore, uint64_t value, uint64_t timeout_ns) {
  if (!semaphore) return ep_invalid_argument("semaphore is NULL");

  GCTimelineSemaphore* sem = static_cast<GCTimelineSemaphore*>(semaphore);
  dispatch_semaphore_t sema = dispatch_semaphore_create(0);
  MTL::SharedEventListener* listener = MTL::SharedEventListener::alloc()->init();

  sem->event->notifyListener(listener, value, ^(MTL::SharedEvent* ev, uint64_t val) {
    (void)ev;
    (void)val;
    dispatch_semaphore_signal(sema);
  });

  dispatch_time_t timeout = (timeout_ns == UINT64_MAX)
    ? DISPATCH_TIME_FOREVER
    : dispatch_time(DISPATCH_TIME_NOW, static_cast<int64_t>(timeout_ns));

  long result = dispatch_semaphore_wait(sema, timeout);
  listener->release();

  if (result != 0) {
    return ep_error(GC_E_INVALID_STATE, "semaphore wait timed out");
  }
  return ep_ok();
}

GCError GCTimelineSemaphoreSignal(GCTimelineSemaphorePtr semaphore, uint64_t value) {
  if (!semaphore) return ep_invalid_argument("semaphore is NULL");
  GCTimelineSemaphore* sem = static_cast<GCTimelineSemaphore*>(semaphore);
  sem->event->setSignaledValue(value);
  return ep_ok();
}

}
