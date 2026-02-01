// enthrall_sync.cpp - Fence and timeline semaphore implementation

#include "enthrall_internal.h"

using namespace ep::vk;

// ============================================================================
// Fence (implemented as timeline semaphore)
// ============================================================================

extern "C" EPError EPFenceCreate(EPDevicePtr device_ptr, uint64_t initial_value, EPFencePtr* out_fence) {
    if (!device_ptr || !out_fence) return invalid_argument("device or out_fence is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    auto fence = std::make_unique<Fence>();
    fence->device = std::shared_ptr<Device>(device, [](Device*){});
    fence->value = initial_value;

    ::vk::SemaphoreTypeCreateInfo type_info{
        .semaphoreType = ::vk::SemaphoreType::eTimeline,
        .initialValue = initial_value,
    };

    auto [result, semaphore] = device->device->createSemaphoreUnique(::vk::SemaphoreCreateInfo{
        .pNext = &type_info,
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    fence->timeline_semaphore = std::move(semaphore);
    *out_fence = reinterpret_cast<EPFencePtr>(fence.release());
    return ok();
}

extern "C" EPError EPFenceDestroy(EPFencePtr fence) {
    delete reinterpret_cast<Fence*>(fence);
    return ok();
}

extern "C" EPError EPFenceWait(EPFencePtr fence_ptr, uint64_t value, uint64_t timeout_ns) {
    if (!fence_ptr) return invalid_argument("fence is NULL");

    auto* fence = reinterpret_cast<Fence*>(fence_ptr);

    ::vk::Semaphore semaphores[] = {fence->timeline_semaphore.get()};
    uint64_t values[] = {value};

    ::vk::SemaphoreWaitInfo wait_info{
        .semaphoreCount = 1,
        .pSemaphores = semaphores,
        .pValues = values,
    };

    auto result = fence->device->device->waitSemaphores(wait_info, timeout_ns);
    if (result == ::vk::Result::eTimeout) {
        return error(EP_E_INVALID_STATE, "fence wait timed out");
    }
    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }
    return ok();
}

extern "C" EPError EPFenceSignal(EPFencePtr fence_ptr, uint64_t value) {
    if (!fence_ptr) return invalid_argument("fence is NULL");

    auto* fence = reinterpret_cast<Fence*>(fence_ptr);

    auto result = fence->device->device->signalSemaphore(::vk::SemaphoreSignalInfo{
        .semaphore = fence->timeline_semaphore.get(),
        .value = value,
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    fence->value = value;
    return ok();
}

// ============================================================================
// Timeline Semaphore
// ============================================================================

extern "C" EPError EPTimelineSemaphoreCreate(EPDevicePtr device_ptr, uint64_t initial_value,
                                              EPTimelineSemaphorePtr* out_semaphore) {
    if (!device_ptr || !out_semaphore) return invalid_argument("device or out_semaphore is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    auto semaphore = std::make_unique<TimelineSemaphore>();
    semaphore->device = std::shared_ptr<Device>(device, [](Device*){});

    ::vk::SemaphoreTypeCreateInfo type_info{
        .semaphoreType = ::vk::SemaphoreType::eTimeline,
        .initialValue = initial_value,
    };

    auto [result, vk_semaphore] = device->device->createSemaphoreUnique(::vk::SemaphoreCreateInfo{
        .pNext = &type_info,
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    semaphore->semaphore = std::move(vk_semaphore);
    *out_semaphore = reinterpret_cast<EPTimelineSemaphorePtr>(semaphore.release());
    return ok();
}

extern "C" EPError EPTimelineSemaphoreDestroy(EPTimelineSemaphorePtr semaphore) {
    delete reinterpret_cast<TimelineSemaphore*>(semaphore);
    return ok();
}

extern "C" EPError EPTimelineSemaphoreGetValue(EPTimelineSemaphorePtr semaphore_ptr, uint64_t* out_value) {
    if (!semaphore_ptr || !out_value) return invalid_argument("semaphore or out_value is NULL");

    auto* semaphore = reinterpret_cast<TimelineSemaphore*>(semaphore_ptr);

    auto [result, value] = semaphore->device->device->getSemaphoreCounterValue(semaphore->semaphore.get());
    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    *out_value = value;
    return ok();
}

extern "C" EPError EPTimelineSemaphoreWait(EPTimelineSemaphorePtr semaphore_ptr, uint64_t value, uint64_t timeout_ns) {
    if (!semaphore_ptr) return invalid_argument("semaphore is NULL");

    auto* semaphore = reinterpret_cast<TimelineSemaphore*>(semaphore_ptr);

    ::vk::Semaphore semaphores[] = {semaphore->semaphore.get()};
    uint64_t values[] = {value};

    ::vk::SemaphoreWaitInfo wait_info{
        .semaphoreCount = 1,
        .pSemaphores = semaphores,
        .pValues = values,
    };

    auto result = semaphore->device->device->waitSemaphores(wait_info, timeout_ns);
    if (result == ::vk::Result::eTimeout) {
        return error(EP_E_INVALID_STATE, "semaphore wait timed out");
    }
    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }
    return ok();
}

extern "C" EPError EPTimelineSemaphoreSignal(EPTimelineSemaphorePtr semaphore_ptr, uint64_t value) {
    if (!semaphore_ptr) return invalid_argument("semaphore is NULL");

    auto* semaphore = reinterpret_cast<TimelineSemaphore*>(semaphore_ptr);

    auto result = semaphore->device->device->signalSemaphore(::vk::SemaphoreSignalInfo{
        .semaphore = semaphore->semaphore.get(),
        .value = value,
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }
    return ok();
}
