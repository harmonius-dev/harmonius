// gcraft_device.m - Device and queue implementation

#include "gcraft_internal.h"

#pragma mark - Device

GCError GCDeviceCreate(GCAdapterPtr adapter, const GCDeviceDesc *desc,
                       GCDevicePtr *out_device) {
    if (!adapter || !desc || !out_device) return ep_invalid_argument("adapter, desc, or out_device is NULL");

    @autoreleasepool {
        // Check required features
        if ((desc->required_features & adapter->properties.features) != desc->required_features) {
            return ep_unsupported("required features not available");
        }

        GCDevice *device = calloc(1, sizeof(GCDevice));
        if (!device) return ep_out_of_memory();

        device->device = [adapter->device retain];
        device->features = adapter->properties.features & (desc->required_features | desc->optional_features);
        device->validation_enabled = desc->enable_validation;
        device->debug_names_enabled = desc->enable_debug_names;

        device->graphics_queue = [[device->device newCommandQueue] retain];
        device->compute_queue = [[device->device newCommandQueue] retain];
        device->transfer_queue = [[device->device newCommandQueue] retain];

        if (!device->graphics_queue || !device->compute_queue || !device->transfer_queue) {
            if (device->graphics_queue) [device->graphics_queue release];
            if (device->compute_queue) [device->compute_queue release];
            if (device->transfer_queue) [device->transfer_queue release];
            [device->device release];
            free(device);
            return ep_error(GC_E_DEVICE_LOST, "failed to create command queues");
        }

        if (desc->enable_debug_names) {
            device->graphics_queue.label = @"Gcraft Graphics Queue";
            device->compute_queue.label = @"Gcraft Compute Queue";
            device->transfer_queue.label = @"Gcraft Transfer Queue";
        }

        *out_device = device;
        return ep_ok();
    }
}

GCError GCDeviceDestroy(GCDevicePtr device) {
    if (device) {
        @autoreleasepool {
            if (device->graphics_queue) [device->graphics_queue release];
            if (device->compute_queue) [device->compute_queue release];
            if (device->transfer_queue) [device->transfer_queue release];
            if (device->device) [device->device release];
        }
        free(device);
    }
    return ep_ok();
}

GCError GCDeviceGetQueue(GCDevicePtr device, GCQueueType type, uint32_t index,
                         GCQueuePtr *out_queue) {
    if (!device || !out_queue) return ep_invalid_argument("device or out_queue is NULL");
    if (index > 0) return ep_invalid_argument("only queue index 0 is supported");

    GCQueue *queue = calloc(1, sizeof(GCQueue));
    if (!queue) return ep_out_of_memory();

    queue->ep_device = device;
    queue->type = type;

    @autoreleasepool {
        switch (type) {
            case GC_QUEUE_GRAPHICS:
                queue->queue = [device->graphics_queue retain];
                break;
            case GC_QUEUE_COMPUTE:
                queue->queue = [device->compute_queue retain];
                break;
            case GC_QUEUE_TRANSFER:
                queue->queue = [device->transfer_queue retain];
                break;
            default:
                free(queue);
                return ep_invalid_argument("invalid queue type");
        }
    }

    *out_queue = queue;
    return ep_ok();
}

#pragma mark - Queue

GCError GCQueueSubmit(GCQueuePtr queue, const GCSubmitInfo *submit) {
    if (!queue || !submit) return ep_invalid_argument("queue or submit is NULL");

    @autoreleasepool {
        for (uint32_t i = 0; i < submit->command_buffer_count; i++) {
            GCCommandBuffer *cmd = submit->command_buffers[i];
            if (!cmd || !cmd->buffer) continue;

            // Encode wait semaphores
            for (uint32_t w = 0; w < submit->wait_count; w++) {
                GCTimelineSemaphore *sem = submit->wait_semaphores[w];
                if (sem && sem->event) {
                    [cmd->buffer encodeWaitForEvent:sem->event value:submit->wait_values[w]];
                }
            }

            // Encode signal semaphores
            for (uint32_t s = 0; s < submit->signal_count; s++) {
                GCTimelineSemaphore *sem = submit->signal_semaphores[s];
                if (sem && sem->event) {
                    [cmd->buffer encodeSignalEvent:sem->event value:submit->signal_values[s]];
                }
            }

            // Signal fence if provided
            if (submit->fence && submit->fence->event) {
                pthread_mutex_lock(&submit->fence->mutex);
                submit->fence->value++;
                uint64_t fence_value = submit->fence->value;
                pthread_mutex_unlock(&submit->fence->mutex);
                [cmd->buffer encodeSignalEvent:submit->fence->event value:fence_value];
            }

            [cmd->buffer commit];
        }
    }

    return ep_ok();
}

GCError GCQueueWaitIdle(GCQueuePtr queue) {
    if (!queue) return ep_invalid_argument("queue is NULL");

    @autoreleasepool {
        id<MTLCommandBuffer> barrier = [queue->queue commandBuffer];
        [barrier commit];
        [barrier waitUntilCompleted];
    }

    return ep_ok();
}
