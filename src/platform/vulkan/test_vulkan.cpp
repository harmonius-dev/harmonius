// test_vulkan.cpp - Tests for Vulkan backend
// These tests verify core functionality of the enthrall Vulkan implementation

#include "enthrall_internal.h"
#include <cstdio>
#include <cstdlib>
#include <cassert>

// Test helpers
#define TEST(name) static void test_##name()
#define RUN_TEST(name) do { \
    printf("  Running %s... ", #name); \
    test_##name(); \
    printf("OK\n"); \
} while(0)

#define EXPECT_OK(expr) do { \
    EPError err = (expr); \
    if (err.code != EP_E_OK) { \
        printf("FAILED: %s returned error %d: %s\n", #expr, err.code, err.message ? err.message : "no message"); \
        abort(); \
    } \
} while(0)

#define EXPECT_NOT_NULL(ptr) do { \
    if ((ptr) == nullptr) { \
        printf("FAILED: %s is NULL\n", #ptr); \
        abort(); \
    } \
} while(0)

// Global state for tests
static EPPlatformPtr g_platform = nullptr;
static EPInstancePtr g_instance = nullptr;
static EPAdapterPtr g_adapter = nullptr;
static EPDevicePtr g_device = nullptr;

// ============================================================================
// Platform Tests
// ============================================================================

TEST(platform_create_destroy) {
    EPPlatformPtr platform = nullptr;
    EXPECT_OK(EPPlatformCreate(&platform));
    EXPECT_NOT_NULL(platform);
    EXPECT_OK(EPPlatformDestroy(platform));
}

// ============================================================================
// Instance Tests
// ============================================================================

TEST(instance_create_destroy) {
    EPInstanceDesc desc{
        .enable_backends = EP_BACKEND_VULKAN_BIT,
        .enable_validation = false,
        .enable_debug_names = false,
    };
    
    EPInstancePtr instance = nullptr;
    EXPECT_OK(EPInstanceCreate(&desc, &instance));
    EXPECT_NOT_NULL(instance);
    EXPECT_OK(EPInstanceDestroy(instance));
}

TEST(instance_enumerate_adapters) {
    EPInstanceDesc desc{
        .enable_backends = EP_BACKEND_VULKAN_BIT,
        .enable_validation = false,
        .enable_debug_names = false,
    };
    
    EPInstancePtr instance = nullptr;
    EXPECT_OK(EPInstanceCreate(&desc, &instance));
    
    uint32_t count = 0;
    EXPECT_OK(EPInstanceEnumerateAdapters(instance, &count, nullptr));
    printf("(found %u adapters) ", count);
    
    if (count > 0) {
        EPAdapterPtr* adapters = new EPAdapterPtr[count];
        EXPECT_OK(EPInstanceEnumerateAdapters(instance, &count, adapters));
        
        for (uint32_t i = 0; i < count; i++) {
            EPAdapterProperties props{};
            EXPECT_OK(EPAdapterGetProperties(adapters[i], &props));
            printf("\n    Adapter %u: %s (vendor: 0x%04x) ", i, props.name, props.vendor_id);
        }
        
        delete[] adapters;
    }
    
    EXPECT_OK(EPInstanceDestroy(instance));
}

// ============================================================================
// Device Tests
// ============================================================================

TEST(device_create_destroy) {
    // Use global adapter
    EPDeviceDesc desc{
        .required_features = EP_FEATURE_COMPUTE_BIT,
        .optional_features = static_cast<EPFeatureFlags>(0),
        .enable_validation = false,
        .enable_debug_names = false,
    };
    
    EPDevicePtr device = nullptr;
    EXPECT_OK(EPDeviceCreate(g_adapter, &desc, &device));
    EXPECT_NOT_NULL(device);
    EXPECT_OK(EPDeviceDestroy(device));
}

TEST(device_get_queues) {
    EPQueuePtr graphics_queue = nullptr;
    EPQueuePtr compute_queue = nullptr;
    
    EXPECT_OK(EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &graphics_queue));
    EXPECT_NOT_NULL(graphics_queue);
    
    EXPECT_OK(EPDeviceGetQueue(g_device, EP_QUEUE_COMPUTE, 0, &compute_queue));
    EXPECT_NOT_NULL(compute_queue);
}

// ============================================================================
// Buffer Tests
// ============================================================================

TEST(buffer_create_destroy) {
    EPBufferDesc desc{
        .size = 1024,
        .usage = EP_BUFFER_USAGE_UNIFORM_BIT,
        .host_visible = true,
    };
    
    EPBufferPtr buffer = nullptr;
    EXPECT_OK(EPBufferCreate(g_device, &desc, &buffer));
    EXPECT_NOT_NULL(buffer);
    EXPECT_OK(EPBufferDestroy(buffer));
}

TEST(buffer_various_usages) {
    EPBufferUsageFlags usages[] = {
        EP_BUFFER_USAGE_VERTEX_BIT,
        EP_BUFFER_USAGE_INDEX_BIT,
        EP_BUFFER_USAGE_UNIFORM_BIT,
        EP_BUFFER_USAGE_STORAGE_BIT,
        static_cast<EPBufferUsageFlags>(EP_BUFFER_USAGE_TRANSFER_SRC_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT),
    };
    
    for (auto usage : usages) {
        EPBufferDesc desc{
            .size = 256,
            .usage = usage,
            .host_visible = false,
        };
        
        EPBufferPtr buffer = nullptr;
        EXPECT_OK(EPBufferCreate(g_device, &desc, &buffer));
        EXPECT_NOT_NULL(buffer);
        EXPECT_OK(EPBufferDestroy(buffer));
    }
}

// ============================================================================
// Texture Tests
// ============================================================================

TEST(texture_create_destroy) {
    EPTextureDesc desc{
        .dimension = EP_TEXTURE_DIM_2D,
        .format = EP_FORMAT_RGBA8_UNORM,
        .width = 256,
        .height = 256,
        .depth = 1,
        .mip_levels = 1,
        .array_layers = 1,
        .usage = static_cast<EPTextureUsageFlags>(EP_TEXTURE_USAGE_SAMPLED_BIT | EP_TEXTURE_USAGE_TRANSFER_DST_BIT),
    };
    
    EPTexturePtr texture = nullptr;
    EXPECT_OK(EPTextureCreate(g_device, &desc, &texture));
    EXPECT_NOT_NULL(texture);
    EXPECT_OK(EPTextureDestroy(texture));
}

TEST(texture_various_formats) {
    EPTextureFormat formats[] = {
        EP_FORMAT_RGBA8_UNORM,
        EP_FORMAT_BGRA8_UNORM,
        EP_FORMAT_RGBA16_FLOAT,
        EP_FORMAT_RGBA32_FLOAT,
    };
    
    for (auto format : formats) {
        EPTextureDesc desc{
            .dimension = EP_TEXTURE_DIM_2D,
            .format = format,
            .width = 64,
            .height = 64,
            .depth = 1,
            .mip_levels = 1,
            .array_layers = 1,
            .usage = EP_TEXTURE_USAGE_SAMPLED_BIT,
        };
        
        EPTexturePtr texture = nullptr;
        EXPECT_OK(EPTextureCreate(g_device, &desc, &texture));
        EXPECT_NOT_NULL(texture);
        EXPECT_OK(EPTextureDestroy(texture));
    }
}

TEST(texture_depth_formats) {
    // D32_FLOAT is widely supported; D24S8 may not be on Apple Silicon
    EPTextureDesc desc{
        .dimension = EP_TEXTURE_DIM_2D,
        .format = EP_FORMAT_D32_FLOAT,
        .width = 512,
        .height = 512,
        .depth = 1,
        .mip_levels = 1,
        .array_layers = 1,
        .usage = EP_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT,
    };
    
    EPTexturePtr texture = nullptr;
    EXPECT_OK(EPTextureCreate(g_device, &desc, &texture));
    EXPECT_NOT_NULL(texture);
    EXPECT_OK(EPTextureDestroy(texture));
}

// ============================================================================
// Sampler Tests
// ============================================================================

TEST(sampler_create_destroy) {
    EPSamplerDesc desc{
        .min_filter = EP_FILTER_LINEAR,
        .mag_filter = EP_FILTER_LINEAR,
        .address_u = EP_ADDRESS_REPEAT,
        .address_v = EP_ADDRESS_REPEAT,
        .address_w = EP_ADDRESS_REPEAT,
        .compare_op = EP_COMPARE_ALWAYS,
        .max_anisotropy = 1.0f,
    };
    
    EPSamplerPtr sampler = nullptr;
    EXPECT_OK(EPSamplerCreate(g_device, &desc, &sampler));
    EXPECT_NOT_NULL(sampler);
    EXPECT_OK(EPSamplerDestroy(sampler));
}

TEST(sampler_anisotropic) {
    EPSamplerDesc desc{
        .min_filter = EP_FILTER_LINEAR,
        .mag_filter = EP_FILTER_LINEAR,
        .address_u = EP_ADDRESS_REPEAT,
        .address_v = EP_ADDRESS_REPEAT,
        .address_w = EP_ADDRESS_REPEAT,
        .compare_op = EP_COMPARE_ALWAYS,
        .max_anisotropy = 16.0f,
    };
    
    EPSamplerPtr sampler = nullptr;
    EXPECT_OK(EPSamplerCreate(g_device, &desc, &sampler));
    EXPECT_NOT_NULL(sampler);
    EXPECT_OK(EPSamplerDestroy(sampler));
}

// ============================================================================
// Command Buffer Tests
// ============================================================================

TEST(command_buffer_create_destroy) {
    EPCommandBufferPtr cmd = nullptr;
    EXPECT_OK(EPCommandBufferCreate(g_device, &cmd));
    EXPECT_NOT_NULL(cmd);
    EXPECT_OK(EPCommandBufferDestroy(cmd));
}

TEST(command_buffer_begin_end) {
    EPCommandBufferPtr cmd = nullptr;
    EXPECT_OK(EPCommandBufferCreate(g_device, &cmd));
    
    EXPECT_OK(EPCommandBufferBegin(cmd));
    EXPECT_OK(EPCommandBufferEnd(cmd));
    
    EXPECT_OK(EPCommandBufferDestroy(cmd));
}

// ============================================================================
// Fence Tests
// ============================================================================

TEST(fence_create_destroy) {
    EPFencePtr fence = nullptr;
    EXPECT_OK(EPFenceCreate(g_device, 0, &fence));
    EXPECT_NOT_NULL(fence);
    EXPECT_OK(EPFenceDestroy(fence));
}

TEST(fence_signal_wait) {
    EPFencePtr fence = nullptr;
    EXPECT_OK(EPFenceCreate(g_device, 0, &fence));
    
    // Signal from CPU
    EXPECT_OK(EPFenceSignal(fence, 1));
    
    // Wait should complete immediately since we just signaled
    EXPECT_OK(EPFenceWait(fence, 1, 1000000000)); // 1 second timeout
    
    EXPECT_OK(EPFenceDestroy(fence));
}

// ============================================================================
// Timeline Semaphore Tests
// ============================================================================

TEST(timeline_semaphore_create_destroy) {
    EPTimelineSemaphorePtr sem = nullptr;
    EXPECT_OK(EPTimelineSemaphoreCreate(g_device, 0, &sem));
    EXPECT_NOT_NULL(sem);
    EXPECT_OK(EPTimelineSemaphoreDestroy(sem));
}

TEST(timeline_semaphore_signal_wait) {
    EPTimelineSemaphorePtr sem = nullptr;
    EXPECT_OK(EPTimelineSemaphoreCreate(g_device, 0, &sem));
    
    EXPECT_OK(EPTimelineSemaphoreSignal(sem, 5));
    
    uint64_t value = 0;
    EXPECT_OK(EPTimelineSemaphoreGetValue(sem, &value));
    assert(value == 5);
    
    EXPECT_OK(EPTimelineSemaphoreWait(sem, 5, 1000000000));
    
    EXPECT_OK(EPTimelineSemaphoreDestroy(sem));
}

// ============================================================================
// Descriptor Set Layout Tests
// ============================================================================

// Helper to create descriptor set layout desc with flexible array
struct DescSetLayoutDesc2 {
    uint32_t binding_count;
    EPDescriptorBindingDesc bindings[2];
};

struct DescSetLayoutDesc1 {
    uint32_t binding_count;
    EPDescriptorBindingDesc bindings[1];
};

TEST(descriptor_set_layout_create_destroy) {
    DescSetLayoutDesc2 desc{
        .binding_count = 2,
        .bindings = {
            {
                .binding = 0,
                .type = EP_DESCRIPTOR_UNIFORM_BUFFER,
                .count = 1,
                .stages = EP_STAGE_VERTEX_BIT,
            },
            {
                .binding = 1,
                .type = EP_DESCRIPTOR_SAMPLED_TEXTURE,
                .count = 1,
                .stages = EP_STAGE_FRAGMENT_BIT,
            },
        },
    };
    
    EPDescriptorSetLayoutPtr layout = nullptr;
    EXPECT_OK(EPDescriptorSetLayoutCreate(g_device, reinterpret_cast<EPDescriptorSetLayoutDesc*>(&desc), &layout));
    EXPECT_NOT_NULL(layout);
    EXPECT_OK(EPDescriptorSetLayoutDestroy(layout));
}

// ============================================================================
// Pipeline Layout Tests
// ============================================================================

// Helper for pipeline layout with flexible array
struct PipelineLayoutDesc1 {
    uint32_t set_layout_count;
    uint32_t push_constant_size;
    EPShaderStageFlags push_constant_stages;
    EPDescriptorSetLayoutPtr set_layouts[1];
};

TEST(pipeline_layout_create_destroy) {
    DescSetLayoutDesc1 set_desc{
        .binding_count = 1,
        .bindings = {
            {
                .binding = 0,
                .type = EP_DESCRIPTOR_UNIFORM_BUFFER,
                .count = 1,
                .stages = static_cast<EPShaderStageFlags>(EP_STAGE_VERTEX_BIT | EP_STAGE_FRAGMENT_BIT),
            },
        },
    };
    
    EPDescriptorSetLayoutPtr set_layout = nullptr;
    EXPECT_OK(EPDescriptorSetLayoutCreate(g_device, reinterpret_cast<EPDescriptorSetLayoutDesc*>(&set_desc), &set_layout));
    
    PipelineLayoutDesc1 desc{
        .set_layout_count = 1,
        .push_constant_size = 64,
        .push_constant_stages = static_cast<EPShaderStageFlags>(EP_STAGE_VERTEX_BIT | EP_STAGE_FRAGMENT_BIT),
        .set_layouts = {set_layout},
    };
    
    EPPipelineLayoutPtr pipeline_layout = nullptr;
    EXPECT_OK(EPPipelineLayoutCreate(g_device, reinterpret_cast<EPPipelineLayoutDesc*>(&desc), &pipeline_layout));
    EXPECT_NOT_NULL(pipeline_layout);
    
    EXPECT_OK(EPPipelineLayoutDestroy(pipeline_layout));
    EXPECT_OK(EPDescriptorSetLayoutDestroy(set_layout));
}

// ============================================================================
// Descriptor Set Tests
// ============================================================================

TEST(descriptor_set_create_update_destroy) {
    DescSetLayoutDesc1 layout_desc{
        .binding_count = 1,
        .bindings = {
            {
                .binding = 0,
                .type = EP_DESCRIPTOR_UNIFORM_BUFFER,
                .count = 1,
                .stages = EP_STAGE_VERTEX_BIT,
            },
        },
    };
    
    EPDescriptorSetLayoutPtr layout = nullptr;
    EXPECT_OK(EPDescriptorSetLayoutCreate(g_device, reinterpret_cast<EPDescriptorSetLayoutDesc*>(&layout_desc), &layout));
    
    EPDescriptorSetPtr set = nullptr;
    EXPECT_OK(EPDescriptorSetCreate(g_device, layout, &set));
    EXPECT_NOT_NULL(set);
    
    // Create a buffer to bind
    EPBufferDesc buffer_desc{
        .size = 256,
        .usage = EP_BUFFER_USAGE_UNIFORM_BIT,
        .host_visible = true,
    };
    
    EPBufferPtr buffer = nullptr;
    EXPECT_OK(EPBufferCreate(g_device, &buffer_desc, &buffer));
    
    // Update descriptor
    EXPECT_OK(EPDescriptorSetUpdateBuffer(set, 0, buffer, 0, 256));
    
    EXPECT_OK(EPBufferDestroy(buffer));
    EXPECT_OK(EPDescriptorSetDestroy(set));
    EXPECT_OK(EPDescriptorSetLayoutDestroy(layout));
}

// ============================================================================
// Queue Submit Tests
// ============================================================================

TEST(queue_submit_empty) {
    EPQueuePtr queue = nullptr;
    EXPECT_OK(EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &queue));
    
    EPCommandBufferPtr cmd = nullptr;
    EXPECT_OK(EPCommandBufferCreate(g_device, &cmd));
    
    EXPECT_OK(EPCommandBufferBegin(cmd));
    EXPECT_OK(EPCommandBufferEnd(cmd));
    
    EPCommandBufferPtr cmds[] = {cmd};
    EPSubmitInfo submit{
        .command_buffers = cmds,
        .command_buffer_count = 1,
        .wait_semaphores = nullptr,
        .wait_values = nullptr,
        .wait_count = 0,
        .signal_semaphores = nullptr,
        .signal_values = nullptr,
        .signal_count = 0,
        .fence = nullptr,
    };
    
    EXPECT_OK(EPQueueSubmit(queue, &submit));
    EXPECT_OK(EPQueueWaitIdle(queue));
    
    EXPECT_OK(EPCommandBufferDestroy(cmd));
}

TEST(queue_submit_with_fence) {
    EPQueuePtr queue = nullptr;
    EXPECT_OK(EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &queue));
    
    EPFencePtr fence = nullptr;
    EXPECT_OK(EPFenceCreate(g_device, 0, &fence));
    
    EPCommandBufferPtr cmd = nullptr;
    EXPECT_OK(EPCommandBufferCreate(g_device, &cmd));
    
    EXPECT_OK(EPCommandBufferBegin(cmd));
    EXPECT_OK(EPCommandBufferEnd(cmd));
    
    EPCommandBufferPtr cmds[] = {cmd};
    EPSubmitInfo submit{
        .command_buffers = cmds,
        .command_buffer_count = 1,
        .wait_semaphores = nullptr,
        .wait_values = nullptr,
        .wait_count = 0,
        .signal_semaphores = nullptr,
        .signal_values = nullptr,
        .signal_count = 0,
        .fence = fence,
    };
    
    EXPECT_OK(EPQueueSubmit(queue, &submit));
    EXPECT_OK(EPFenceWait(fence, 1, 5000000000)); // 5 second timeout
    
    EXPECT_OK(EPCommandBufferDestroy(cmd));
    EXPECT_OK(EPFenceDestroy(fence));
}

// ============================================================================
// Functional Tests - These verify actual GPU work
// ============================================================================

// SPIR-V bytecode for compute shader: values[idx] = values[idx] * 2
static const uint8_t multiply_shader_spirv[] = {
    0x03, 0x02, 0x23, 0x07, 0x00, 0x00, 0x01, 0x00, 0x0b, 0x00, 0x08, 0x00,
    0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x02, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x47, 0x4c, 0x53, 0x4c, 0x2e, 0x73, 0x74, 0x64, 0x2e, 0x34, 0x35, 0x30,
    0x00, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x06, 0x00, 0x05, 0x00, 0x00, 0x00,
    0x04, 0x00, 0x00, 0x00, 0x6d, 0x61, 0x69, 0x6e, 0x00, 0x00, 0x00, 0x00,
    0x0b, 0x00, 0x00, 0x00, 0x10, 0x00, 0x06, 0x00, 0x04, 0x00, 0x00, 0x00,
    0x11, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x00, 0x00,
    0xc2, 0x01, 0x00, 0x00, 0x05, 0x00, 0x04, 0x00, 0x04, 0x00, 0x00, 0x00,
    0x6d, 0x61, 0x69, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x03, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x69, 0x64, 0x78, 0x00, 0x05, 0x00, 0x08, 0x00,
    0x0b, 0x00, 0x00, 0x00, 0x67, 0x6c, 0x5f, 0x47, 0x6c, 0x6f, 0x62, 0x61,
    0x6c, 0x49, 0x6e, 0x76, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49,
    0x44, 0x00, 0x00, 0x00, 0x05, 0x00, 0x04, 0x00, 0x11, 0x00, 0x00, 0x00,
    0x44, 0x61, 0x74, 0x61, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x05, 0x00,
    0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x73, 0x00, 0x00, 0x05, 0x00, 0x03, 0x00, 0x13, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x47, 0x00, 0x04, 0x00, 0x0b, 0x00, 0x00, 0x00,
    0x0b, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x00, 0x47, 0x00, 0x04, 0x00,
    0x10, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
    0x47, 0x00, 0x03, 0x00, 0x11, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
    0x48, 0x00, 0x05, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47, 0x00, 0x04, 0x00,
    0x13, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x47, 0x00, 0x04, 0x00, 0x13, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x47, 0x00, 0x04, 0x00, 0x20, 0x00, 0x00, 0x00,
    0x0b, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x13, 0x00, 0x02, 0x00,
    0x02, 0x00, 0x00, 0x00, 0x21, 0x00, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x00, 0x00, 0x15, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00,
    0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x04, 0x00,
    0x07, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
    0x17, 0x00, 0x04, 0x00, 0x09, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x00, 0x20, 0x00, 0x04, 0x00, 0x0a, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x3b, 0x00, 0x04, 0x00,
    0x0a, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x2b, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x04, 0x00, 0x0d, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x1d, 0x00, 0x03, 0x00,
    0x10, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x03, 0x00,
    0x11, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x20, 0x00, 0x04, 0x00,
    0x12, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
    0x3b, 0x00, 0x04, 0x00, 0x12, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x00, 0x00, 0x15, 0x00, 0x04, 0x00, 0x14, 0x00, 0x00, 0x00,
    0x20, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x2b, 0x00, 0x04, 0x00,
    0x14, 0x00, 0x00, 0x00, 0x15, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x20, 0x00, 0x04, 0x00, 0x18, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
    0x06, 0x00, 0x00, 0x00, 0x2b, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00,
    0x1b, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x2b, 0x00, 0x04, 0x00,
    0x06, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00,
    0x2b, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x06, 0x00, 0x09, 0x00, 0x00, 0x00,
    0x20, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00,
    0x1f, 0x00, 0x00, 0x00, 0x36, 0x00, 0x05, 0x00, 0x02, 0x00, 0x00, 0x00,
    0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
    0xf8, 0x00, 0x02, 0x00, 0x05, 0x00, 0x00, 0x00, 0x3b, 0x00, 0x04, 0x00,
    0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00,
    0x41, 0x00, 0x05, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00,
    0x0b, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x3d, 0x00, 0x04, 0x00,
    0x06, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00,
    0x3e, 0x00, 0x03, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00,
    0x3d, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x3d, 0x00, 0x04, 0x00, 0x06, 0x00, 0x00, 0x00,
    0x17, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x41, 0x00, 0x06, 0x00,
    0x18, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00,
    0x15, 0x00, 0x00, 0x00, 0x17, 0x00, 0x00, 0x00, 0x3d, 0x00, 0x04, 0x00,
    0x06, 0x00, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00,
    0x84, 0x00, 0x05, 0x00, 0x06, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x00,
    0x1a, 0x00, 0x00, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x41, 0x00, 0x06, 0x00,
    0x18, 0x00, 0x00, 0x00, 0x1d, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00,
    0x15, 0x00, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x03, 0x00,
    0x1d, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x00, 0xfd, 0x00, 0x01, 0x00,
    0x38, 0x00, 0x01, 0x00
};
static const uint32_t multiply_shader_spirv_size = sizeof(multiply_shader_spirv);

TEST(buffer_data_roundtrip) {
    // Test: write data to host-visible buffer, read it back, verify it matches
    const uint32_t num_values = 256;
    const uint64_t buffer_size = num_values * sizeof(uint32_t);

    EPBufferDesc desc{
        .size = buffer_size,
        .usage = EP_BUFFER_USAGE_STORAGE_BIT,
        .host_visible = true,
    };

    EPBufferPtr buffer = nullptr;
    EXPECT_OK(EPBufferCreate(g_device, &desc, &buffer));

    // Map and write data
    void* mapped = nullptr;
    EXPECT_OK(EPBufferMap(buffer, &mapped));
    EXPECT_NOT_NULL(mapped);

    uint32_t* data = static_cast<uint32_t*>(mapped);
    for (uint32_t i = 0; i < num_values; i++) {
        data[i] = i * 3 + 7;  // Some non-trivial pattern
    }

    EXPECT_OK(EPBufferUnmap(buffer));

    // Map again and verify data
    mapped = nullptr;
    EXPECT_OK(EPBufferMap(buffer, &mapped));
    data = static_cast<uint32_t*>(mapped);

    for (uint32_t i = 0; i < num_values; i++) {
        uint32_t expected = i * 3 + 7;
        if (data[i] != expected) {
            printf("FAILED: data[%u] = %u, expected %u\n", i, data[i], expected);
            abort();
        }
    }

    EXPECT_OK(EPBufferUnmap(buffer));
    EXPECT_OK(EPBufferDestroy(buffer));
}

TEST(buffer_copy_roundtrip) {
    // Test: write to staging buffer, copy to device buffer, copy back, verify
    const uint32_t num_values = 128;
    const uint64_t buffer_size = num_values * sizeof(uint32_t);

    // Create host-visible staging buffer
    EPBufferDesc staging_desc{
        .size = buffer_size,
        .usage = static_cast<EPBufferUsageFlags>(EP_BUFFER_USAGE_TRANSFER_SRC_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT),
        .host_visible = true,
    };
    EPBufferPtr staging = nullptr;
    EXPECT_OK(EPBufferCreate(g_device, &staging_desc, &staging));

    // Create device-local buffer
    EPBufferDesc device_desc{
        .size = buffer_size,
        .usage = static_cast<EPBufferUsageFlags>(EP_BUFFER_USAGE_TRANSFER_SRC_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT),
        .host_visible = false,
    };
    EPBufferPtr device_buffer = nullptr;
    EXPECT_OK(EPBufferCreate(g_device, &device_desc, &device_buffer));

    // Write test data to staging buffer
    void* mapped = nullptr;
    EXPECT_OK(EPBufferMap(staging, &mapped));
    uint32_t* data = static_cast<uint32_t*>(mapped);
    for (uint32_t i = 0; i < num_values; i++) {
        data[i] = i * 5 + 13;
    }
    EXPECT_OK(EPBufferUnmap(staging));

    // Get queue and create command buffer
    EPQueuePtr queue = nullptr;
    EXPECT_OK(EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &queue));

    EPCommandBufferPtr cmd = nullptr;
    EXPECT_OK(EPCommandBufferCreate(g_device, &cmd));

    // Record: copy staging -> device -> staging
    EXPECT_OK(EPCommandBufferBegin(cmd));
    EXPECT_OK(EPCommandCopyBuffer(cmd, staging, 0, device_buffer, 0, buffer_size));

    // Barrier: transfer write -> transfer read
    EPBufferBarrier barrier1{
        .buffer = device_buffer,
        .offset = 0,
        .size = buffer_size,
        .src_access = EP_ACCESS_TRANSFER_WRITE_BIT,
        .dst_access = EP_ACCESS_TRANSFER_READ_BIT,
    };
    EPBarrierDesc barrier_desc1{
        .src_stage = EP_STAGE_TRANSFER_BIT,
        .dst_stage = EP_STAGE_TRANSFER_BIT,
        .buffer_barriers = &barrier1,
        .buffer_barrier_count = 1,
        .texture_barriers = nullptr,
        .texture_barrier_count = 0,
    };
    EXPECT_OK(EPCommandResourceBarrier(cmd, &barrier_desc1));

    EXPECT_OK(EPCommandCopyBuffer(cmd, device_buffer, 0, staging, 0, buffer_size));
    EXPECT_OK(EPCommandBufferEnd(cmd));

    // Submit and wait
    EPFencePtr fence = nullptr;
    EXPECT_OK(EPFenceCreate(g_device, 0, &fence));

    EPCommandBufferPtr cmds[] = {cmd};
    EPSubmitInfo submit{
        .command_buffers = cmds,
        .command_buffer_count = 1,
        .wait_semaphores = nullptr,
        .wait_values = nullptr,
        .wait_count = 0,
        .signal_semaphores = nullptr,
        .signal_values = nullptr,
        .signal_count = 0,
        .fence = fence,
    };
    EXPECT_OK(EPQueueSubmit(queue, &submit));
    EXPECT_OK(EPFenceWait(fence, 1, 5000000000));

    // Read back and verify
    EXPECT_OK(EPBufferMap(staging, &mapped));
    data = static_cast<uint32_t*>(mapped);
    for (uint32_t i = 0; i < num_values; i++) {
        uint32_t expected = i * 5 + 13;
        if (data[i] != expected) {
            printf("FAILED: data[%u] = %u, expected %u\n", i, data[i], expected);
            abort();
        }
    }
    EXPECT_OK(EPBufferUnmap(staging));

    // Cleanup
    EXPECT_OK(EPFenceDestroy(fence));
    EXPECT_OK(EPCommandBufferDestroy(cmd));
    EXPECT_OK(EPBufferDestroy(device_buffer));
    EXPECT_OK(EPBufferDestroy(staging));
}

TEST(compute_shader_execution) {
    // Test: run compute shader that multiplies each value by 2, verify output
    const uint32_t num_values = 256;  // Must match shader local_size_x * dispatch groups
    const uint64_t buffer_size = num_values * sizeof(uint32_t);

    // Create host-visible storage buffer
    EPBufferDesc buffer_desc{
        .size = buffer_size,
        .usage = EP_BUFFER_USAGE_STORAGE_BIT,
        .host_visible = true,
    };
    EPBufferPtr buffer = nullptr;
    EXPECT_OK(EPBufferCreate(g_device, &buffer_desc, &buffer));

    // Write initial data: 1, 2, 3, ...
    void* mapped = nullptr;
    EXPECT_OK(EPBufferMap(buffer, &mapped));
    uint32_t* data = static_cast<uint32_t*>(mapped);
    for (uint32_t i = 0; i < num_values; i++) {
        data[i] = i + 1;
    }
    EXPECT_OK(EPBufferUnmap(buffer));

    // Create shader library from SPIR-V
    EPShaderLibraryDesc shader_desc{
        .format = EP_SHADER_SPIRV,
        .data = multiply_shader_spirv,
        .size = multiply_shader_spirv_size,
        .label = "multiply_shader",
    };
    EPShaderLibraryPtr shader = nullptr;
    EXPECT_OK(EPShaderLibraryCreate(g_device, &shader_desc, &shader));

    // Create descriptor set layout with one storage buffer binding
    DescSetLayoutDesc1 layout_desc{
        .binding_count = 1,
        .bindings = {
            {
                .binding = 0,
                .type = EP_DESCRIPTOR_STORAGE_BUFFER,
                .count = 1,
                .stages = EP_STAGE_COMPUTE_BIT,
            },
        },
    };
    EPDescriptorSetLayoutPtr set_layout = nullptr;
    EXPECT_OK(EPDescriptorSetLayoutCreate(g_device, reinterpret_cast<EPDescriptorSetLayoutDesc*>(&layout_desc), &set_layout));

    // Create pipeline layout
    PipelineLayoutDesc1 pipe_layout_desc{
        .set_layout_count = 1,
        .push_constant_size = 0,
        .push_constant_stages = static_cast<EPShaderStageFlags>(0),
        .set_layouts = {set_layout},
    };
    EPPipelineLayoutPtr pipeline_layout = nullptr;
    EXPECT_OK(EPPipelineLayoutCreate(g_device, reinterpret_cast<EPPipelineLayoutDesc*>(&pipe_layout_desc), &pipeline_layout));

    // Create compute pipeline
    EPComputePipelineDesc compute_desc{
        .library = shader,
        .entry = "main",
        .layout = pipeline_layout,
    };
    EPComputePipelinePtr pipeline = nullptr;
    EXPECT_OK(EPComputePipelineCreate(g_device, &compute_desc, &pipeline));

    // Create descriptor set and bind buffer
    EPDescriptorSetPtr desc_set = nullptr;
    EXPECT_OK(EPDescriptorSetCreate(g_device, set_layout, &desc_set));
    EXPECT_OK(EPDescriptorSetUpdateBuffer(desc_set, 0, buffer, 0, buffer_size));

    // Get queue and create command buffer
    EPQueuePtr queue = nullptr;
    EXPECT_OK(EPDeviceGetQueue(g_device, EP_QUEUE_COMPUTE, 0, &queue));

    EPCommandBufferPtr cmd = nullptr;
    EXPECT_OK(EPCommandBufferCreate(g_device, &cmd));

    // Record compute dispatch
    EXPECT_OK(EPCommandBufferBegin(cmd));

    // Barrier: host write -> shader read
    EPBufferBarrier barrier_pre{
        .buffer = buffer,
        .offset = 0,
        .size = buffer_size,
        .src_access = EP_ACCESS_HOST_WRITE_BIT,
        .dst_access = EP_ACCESS_SHADER_READ_BIT,
    };
    EPBarrierDesc barrier_desc_pre{
        .src_stage = EP_STAGE_TOP_OF_PIPE_BIT,
        .dst_stage = EP_STAGE_COMPUTE_SHADER_BIT,
        .buffer_barriers = &barrier_pre,
        .buffer_barrier_count = 1,
        .texture_barriers = nullptr,
        .texture_barrier_count = 0,
    };
    EXPECT_OK(EPCommandResourceBarrier(cmd, &barrier_desc_pre));

    EXPECT_OK(EPCommandBindComputePipeline(cmd, pipeline));
    EXPECT_OK(EPCommandBindDescriptorSet(cmd, pipeline_layout, 0, desc_set));
    EXPECT_OK(EPCommandDispatch(cmd, 4, 1, 1));  // 4 * 64 = 256 invocations

    // Barrier: shader write -> host read
    EPBufferBarrier barrier_post{
        .buffer = buffer,
        .offset = 0,
        .size = buffer_size,
        .src_access = EP_ACCESS_SHADER_WRITE_BIT,
        .dst_access = EP_ACCESS_HOST_READ_BIT,
    };
    EPBarrierDesc barrier_desc_post{
        .src_stage = EP_STAGE_COMPUTE_SHADER_BIT,
        .dst_stage = EP_STAGE_BOTTOM_OF_PIPE_BIT,
        .buffer_barriers = &barrier_post,
        .buffer_barrier_count = 1,
        .texture_barriers = nullptr,
        .texture_barrier_count = 0,
    };
    EXPECT_OK(EPCommandResourceBarrier(cmd, &barrier_desc_post));

    EXPECT_OK(EPCommandBufferEnd(cmd));

    // Submit and wait
    EPFencePtr fence = nullptr;
    EXPECT_OK(EPFenceCreate(g_device, 0, &fence));

    EPCommandBufferPtr cmds[] = {cmd};
    EPSubmitInfo submit{
        .command_buffers = cmds,
        .command_buffer_count = 1,
        .wait_semaphores = nullptr,
        .wait_values = nullptr,
        .wait_count = 0,
        .signal_semaphores = nullptr,
        .signal_values = nullptr,
        .signal_count = 0,
        .fence = fence,
    };
    EXPECT_OK(EPQueueSubmit(queue, &submit));
    EXPECT_OK(EPFenceWait(fence, 1, 5000000000));

    // Read back and verify: each value should be doubled
    EXPECT_OK(EPBufferMap(buffer, &mapped));
    data = static_cast<uint32_t*>(mapped);
    for (uint32_t i = 0; i < num_values; i++) {
        uint32_t expected = (i + 1) * 2;
        if (data[i] != expected) {
            printf("FAILED: data[%u] = %u, expected %u\n", i, data[i], expected);
            abort();
        }
    }
    EXPECT_OK(EPBufferUnmap(buffer));

    printf("(verified %u computed values) ", num_values);

    // Cleanup
    EXPECT_OK(EPFenceDestroy(fence));
    EXPECT_OK(EPCommandBufferDestroy(cmd));
    EXPECT_OK(EPComputePipelineDestroy(pipeline));
    EXPECT_OK(EPDescriptorSetDestroy(desc_set));
    EXPECT_OK(EPPipelineLayoutDestroy(pipeline_layout));
    EXPECT_OK(EPDescriptorSetLayoutDestroy(set_layout));
    EXPECT_OK(EPShaderLibraryDestroy(shader));
    EXPECT_OK(EPBufferDestroy(buffer));
}

// ============================================================================
// Main
// ============================================================================

void setup_global_state() {
    printf("Setting up global test state...\n");
    
    EXPECT_OK(EPPlatformCreate(&g_platform));
    
    EPInstanceDesc instance_desc{
        .enable_backends = EP_BACKEND_VULKAN_BIT,
        .enable_validation = false,
        .enable_debug_names = false,
    };
    EXPECT_OK(EPInstanceCreate(&instance_desc, &g_instance));
    
    uint32_t adapter_count = 1;
    EXPECT_OK(EPInstanceEnumerateAdapters(g_instance, &adapter_count, &g_adapter));
    
    if (adapter_count == 0) {
        printf("No Vulkan adapters found! Tests cannot run.\n");
        exit(1);
    }
    
    EPAdapterProperties props{};
    EXPECT_OK(EPAdapterGetProperties(g_adapter, &props));
    printf("Using adapter: %s\n", props.name);
    
    EPDeviceDesc device_desc{
        .required_features = EP_FEATURE_COMPUTE_BIT,
        .optional_features = static_cast<EPFeatureFlags>(
            EP_FEATURE_TIMELINE_SEMAPHORE_BIT |
            EP_FEATURE_MESH_SHADER_BIT |
            EP_FEATURE_RAY_TRACING_BIT
        ),
        .enable_validation = false,
        .enable_debug_names = false,
    };
    EXPECT_OK(EPDeviceCreate(g_adapter, &device_desc, &g_device));
    
    printf("Global state ready.\n\n");
}

void teardown_global_state() {
    printf("\nTearing down global test state...\n");
    EXPECT_OK(EPDeviceDestroy(g_device));
    EXPECT_OK(EPInstanceDestroy(g_instance));
    EXPECT_OK(EPPlatformDestroy(g_platform));
    printf("Done.\n");
}

int main() {
    printf("=== Enthrall Vulkan Backend Tests ===\n\n");
    
    setup_global_state();
    
    printf("Platform Tests:\n");
    RUN_TEST(platform_create_destroy);
    
    printf("\nInstance Tests:\n");
    RUN_TEST(instance_create_destroy);
    RUN_TEST(instance_enumerate_adapters);
    
    printf("\nDevice Tests:\n");
    RUN_TEST(device_create_destroy);
    RUN_TEST(device_get_queues);
    
    printf("\nBuffer Tests:\n");
    RUN_TEST(buffer_create_destroy);
    RUN_TEST(buffer_various_usages);
    
    printf("\nTexture Tests:\n");
    RUN_TEST(texture_create_destroy);
    RUN_TEST(texture_various_formats);
    RUN_TEST(texture_depth_formats);
    
    printf("\nSampler Tests:\n");
    RUN_TEST(sampler_create_destroy);
    RUN_TEST(sampler_anisotropic);
    
    printf("\nCommand Buffer Tests:\n");
    RUN_TEST(command_buffer_create_destroy);
    RUN_TEST(command_buffer_begin_end);
    
    printf("\nFence Tests:\n");
    RUN_TEST(fence_create_destroy);
    RUN_TEST(fence_signal_wait);
    
    printf("\nTimeline Semaphore Tests:\n");
    RUN_TEST(timeline_semaphore_create_destroy);
    RUN_TEST(timeline_semaphore_signal_wait);
    
    printf("\nDescriptor Set Layout Tests:\n");
    RUN_TEST(descriptor_set_layout_create_destroy);
    
    printf("\nPipeline Layout Tests:\n");
    RUN_TEST(pipeline_layout_create_destroy);
    
    printf("\nDescriptor Set Tests:\n");
    RUN_TEST(descriptor_set_create_update_destroy);
    
    printf("\nQueue Submit Tests:\n");
    RUN_TEST(queue_submit_empty);
    RUN_TEST(queue_submit_with_fence);
    
    printf("\nFunctional Tests (GPU Verification):\n");
    RUN_TEST(buffer_data_roundtrip);
    RUN_TEST(buffer_copy_roundtrip);
    RUN_TEST(compute_shader_execution);
    
    teardown_global_state();
    
    printf("\n=== All tests passed! ===\n");
    return 0;
}
