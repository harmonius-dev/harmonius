// enthrall_test.cpp - Unit tests for D3D12 backend
// Run with: zig build test (on Windows)

#include "enthrall_internal.h"
#include <cassert>
#include <cstdio>
#include <cstring>

// ============================================================================
// Test Utilities
// ============================================================================

#define TEST_ASSERT(cond, msg) do { \
    if (!(cond)) { \
        printf("FAIL: %s (%s:%d)\n", msg, __FILE__, __LINE__); \
        return false; \
    } \
} while(0)

#define TEST_OK(err) TEST_ASSERT((err).code == EP_E_OK, (err).message ? (err).message : "unexpected error")

#define RUN_TEST(test_fn) do { \
    printf("Running %s... ", #test_fn); \
    if (test_fn()) { \
        printf("PASS\n"); \
        passed++; \
    } else { \
        printf("FAIL\n"); \
        failed++; \
    } \
} while(0)

static int passed = 0;
static int failed = 0;

// Global test fixtures
static EPPlatformPtr g_platform = nullptr;
static EPInstancePtr g_instance = nullptr;
static EPAdapterPtr g_adapter = nullptr;
static EPDevicePtr g_device = nullptr;

// ============================================================================
// Setup/Teardown
// ============================================================================

static bool setup_test_environment() {
    EPError err;
    
    // Create platform
    err = EPPlatformCreate(&g_platform);
    TEST_OK(err);
    TEST_ASSERT(g_platform != nullptr, "platform should not be null");
    
    // Create instance with D3D12 backend
    EPInstanceDesc instance_desc = {};
    instance_desc.enable_backends = EP_BACKEND_D3D12_BIT;
    instance_desc.enable_validation = true;
    instance_desc.enable_debug_names = true;
    
    err = EPInstanceCreate(&instance_desc, &g_instance);
    TEST_OK(err);
    TEST_ASSERT(g_instance != nullptr, "instance should not be null");
    
    // Enumerate adapters
    uint32_t adapter_count = 0;
    err = EPInstanceEnumerateAdapters(g_instance, &adapter_count, nullptr);
    TEST_OK(err);
    
    if (adapter_count == 0) {
        printf("WARNING: No D3D12 adapters found, skipping device tests\n");
        return true;
    }
    
    err = EPInstanceEnumerateAdapters(g_instance, &adapter_count, &g_adapter);
    TEST_OK(err);
    TEST_ASSERT(g_adapter != nullptr, "adapter should not be null");
    
    // Create device
    EPDeviceDesc device_desc = {};
    device_desc.preferred_backend = EP_BACKEND_D3D12;
    device_desc.required_features = EP_FEATURE_COMPUTE_BIT;
    device_desc.optional_features = EP_FEATURE_MESH_SHADER_BIT | EP_FEATURE_RAY_TRACING_BIT;
    device_desc.enable_validation = true;
    device_desc.enable_debug_names = true;
    
    err = EPDeviceCreate(g_adapter, &device_desc, &g_device);
    TEST_OK(err);
    TEST_ASSERT(g_device != nullptr, "device should not be null");
    
    return true;
}

static void teardown_test_environment() {
    if (g_device) EPDeviceDestroy(g_device);
    if (g_adapter) { /* adapter cleanup handled by instance */ }
    if (g_instance) EPInstanceDestroy(g_instance);
    if (g_platform) EPPlatformDestroy(g_platform);
    
    g_device = nullptr;
    g_adapter = nullptr;
    g_instance = nullptr;
    g_platform = nullptr;
}

// ============================================================================
// Platform Tests
// ============================================================================

static bool test_platform_create_destroy() {
    EPPlatformPtr platform = nullptr;
    EPError err = EPPlatformCreate(&platform);
    TEST_OK(err);
    TEST_ASSERT(platform != nullptr, "platform should not be null");
    
    err = EPPlatformDestroy(platform);
    TEST_OK(err);
    
    return true;
}

static bool test_platform_null_argument() {
    EPError err = EPPlatformCreate(nullptr);
    TEST_ASSERT(err.code == EP_E_INVALID_ARGUMENT, "should fail with invalid argument");
    
    return true;
}

// ============================================================================
// Instance Tests
// ============================================================================

static bool test_instance_create_destroy() {
    EPInstancePtr instance = nullptr;
    EPInstanceDesc desc = {};
    desc.enable_backends = EP_BACKEND_D3D12_BIT;
    
    EPError err = EPInstanceCreate(&desc, &instance);
    TEST_OK(err);
    TEST_ASSERT(instance != nullptr, "instance should not be null");
    
    err = EPInstanceDestroy(instance);
    TEST_OK(err);
    
    return true;
}

static bool test_instance_null_arguments() {
    EPInstancePtr instance = nullptr;
    EPInstanceDesc desc = {};
    
    EPError err = EPInstanceCreate(nullptr, &instance);
    TEST_ASSERT(err.code == EP_E_INVALID_ARGUMENT, "should fail with null desc");
    
    err = EPInstanceCreate(&desc, nullptr);
    TEST_ASSERT(err.code == EP_E_INVALID_ARGUMENT, "should fail with null out_instance");
    
    return true;
}

static bool test_instance_enumerate_adapters() {
    if (!g_instance) return true; // Skip if no instance
    
    uint32_t count = 0;
    EPError err = EPInstanceEnumerateAdapters(g_instance, &count, nullptr);
    TEST_OK(err);
    
    // Count query should work even if no adapters
    return true;
}

// ============================================================================
// Adapter Tests
// ============================================================================

static bool test_adapter_properties() {
    if (!g_adapter) return true; // Skip if no adapter
    
    EPAdapterProperties props = {};
    EPError err = EPAdapterGetProperties(g_adapter, &props);
    TEST_OK(err);
    
    // Name should not be empty
    TEST_ASSERT(strlen(props.name) > 0, "adapter name should not be empty");
    
    // Should report D3D12 backend
    TEST_ASSERT(props.backends & EP_BACKEND_D3D12_BIT, "should support D3D12 backend");
    
    // Should support compute
    TEST_ASSERT(props.features & EP_FEATURE_COMPUTE_BIT, "should support compute");
    
    // Limits should be reasonable
    TEST_ASSERT(props.limits.max_texture_dimension_2d >= 4096, "max 2D texture should be >= 4096");
    TEST_ASSERT(props.limits.max_threads_per_threadgroup >= 64, "max threads should be >= 64");
    
    return true;
}

// ============================================================================
// Device Tests
// ============================================================================

static bool test_device_create_destroy() {
    if (!g_adapter) return true; // Skip if no adapter
    
    EPDevicePtr device = nullptr;
    EPDeviceDesc desc = {};
    desc.preferred_backend = EP_BACKEND_D3D12;
    desc.required_features = EP_FEATURE_COMPUTE_BIT;
    
    EPError err = EPDeviceCreate(g_adapter, &desc, &device);
    TEST_OK(err);
    TEST_ASSERT(device != nullptr, "device should not be null");
    
    err = EPDeviceDestroy(device);
    TEST_OK(err);
    
    return true;
}

static bool test_device_get_queues() {
    if (!g_device) return true; // Skip if no device
    
    EPQueuePtr graphics_queue = nullptr;
    EPQueuePtr compute_queue = nullptr;
    EPQueuePtr transfer_queue = nullptr;
    
    EPError err = EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &graphics_queue);
    TEST_OK(err);
    TEST_ASSERT(graphics_queue != nullptr, "graphics queue should not be null");
    
    err = EPDeviceGetQueue(g_device, EP_QUEUE_COMPUTE, 0, &compute_queue);
    TEST_OK(err);
    TEST_ASSERT(compute_queue != nullptr, "compute queue should not be null");
    
    err = EPDeviceGetQueue(g_device, EP_QUEUE_TRANSFER, 0, &transfer_queue);
    TEST_OK(err);
    TEST_ASSERT(transfer_queue != nullptr, "transfer queue should not be null");
    
    // Cleanup (queues don't have destroy, they reference device queues)
    delete graphics_queue;
    delete compute_queue;
    delete transfer_queue;
    
    return true;
}

static bool test_device_queue_invalid_index() {
    if (!g_device) return true; // Skip if no device
    
    EPQueuePtr queue = nullptr;
    EPError err = EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 1, &queue);
    TEST_ASSERT(err.code == EP_E_INVALID_ARGUMENT, "should fail with invalid queue index");
    
    return true;
}

// ============================================================================
// Buffer Tests
// ============================================================================

static bool test_buffer_create_destroy() {
    if (!g_device) return true;
    
    EPBufferPtr buffer = nullptr;
    EPBufferDesc desc = {};
    desc.size = 1024;
    desc.usage = EP_BUFFER_USAGE_VERTEX_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT;
    desc.host_visible = false;
    
    EPError err = EPBufferCreate(g_device, &desc, &buffer);
    TEST_OK(err);
    TEST_ASSERT(buffer != nullptr, "buffer should not be null");
    
    err = EPBufferDestroy(buffer);
    TEST_OK(err);
    
    return true;
}

static bool test_buffer_host_visible() {
    if (!g_device) return true;
    
    EPBufferPtr buffer = nullptr;
    EPBufferDesc desc = {};
    desc.size = 256;
    desc.usage = EP_BUFFER_USAGE_UNIFORM_BIT;
    desc.host_visible = true;
    
    EPError err = EPBufferCreate(g_device, &desc, &buffer);
    TEST_OK(err);
    TEST_ASSERT(buffer != nullptr, "buffer should not be null");
    TEST_ASSERT(buffer->mapped_ptr != nullptr, "host visible buffer should be mapped");
    
    // Write some data
    memset(buffer->mapped_ptr, 0xAB, 256);
    
    err = EPBufferDestroy(buffer);
    TEST_OK(err);
    
    return true;
}

static bool test_buffer_storage() {
    if (!g_device) return true;
    
    EPBufferPtr buffer = nullptr;
    EPBufferDesc desc = {};
    desc.size = 4096;
    desc.usage = EP_BUFFER_USAGE_STORAGE_BIT;
    desc.host_visible = false;
    
    EPError err = EPBufferCreate(g_device, &desc, &buffer);
    TEST_OK(err);
    TEST_ASSERT(buffer != nullptr, "buffer should not be null");
    TEST_ASSERT(buffer->gpu_address != 0, "buffer should have GPU address");
    
    err = EPBufferDestroy(buffer);
    TEST_OK(err);
    
    return true;
}

// ============================================================================
// Texture Tests
// ============================================================================

static bool test_texture_2d_create() {
    if (!g_device) return true;
    
    EPTexturePtr texture = nullptr;
    EPTextureDesc desc = {};
    desc.dimension = EP_TEXTURE_DIM_2D;
    desc.format = EP_FORMAT_RGBA8_UNORM;
    desc.width = 256;
    desc.height = 256;
    desc.depth = 1;
    desc.mip_levels = 1;
    desc.array_layers = 1;
    desc.usage = EP_TEXTURE_USAGE_SAMPLED_BIT | EP_TEXTURE_USAGE_TRANSFER_DST_BIT;
    
    EPError err = EPTextureCreate(g_device, &desc, &texture);
    TEST_OK(err);
    TEST_ASSERT(texture != nullptr, "texture should not be null");
    
    err = EPTextureDestroy(texture);
    TEST_OK(err);
    
    return true;
}

static bool test_texture_render_target() {
    if (!g_device) return true;
    
    EPTexturePtr texture = nullptr;
    EPTextureDesc desc = {};
    desc.dimension = EP_TEXTURE_DIM_2D;
    desc.format = EP_FORMAT_RGBA8_UNORM;
    desc.width = 1920;
    desc.height = 1080;
    desc.depth = 1;
    desc.mip_levels = 1;
    desc.array_layers = 1;
    desc.usage = EP_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT | EP_TEXTURE_USAGE_SAMPLED_BIT;
    
    EPError err = EPTextureCreate(g_device, &desc, &texture);
    TEST_OK(err);
    TEST_ASSERT(texture != nullptr, "texture should not be null");
    TEST_ASSERT(texture->rtv.ptr != 0, "render target should have RTV");
    
    err = EPTextureDestroy(texture);
    TEST_OK(err);
    
    return true;
}

static bool test_texture_depth_stencil() {
    if (!g_device) return true;
    
    EPTexturePtr texture = nullptr;
    EPTextureDesc desc = {};
    desc.dimension = EP_TEXTURE_DIM_2D;
    desc.format = EP_FORMAT_D32_FLOAT;
    desc.width = 1920;
    desc.height = 1080;
    desc.depth = 1;
    desc.mip_levels = 1;
    desc.array_layers = 1;
    desc.usage = EP_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT;
    
    EPError err = EPTextureCreate(g_device, &desc, &texture);
    TEST_OK(err);
    TEST_ASSERT(texture != nullptr, "texture should not be null");
    TEST_ASSERT(texture->dsv.ptr != 0, "depth texture should have DSV");
    
    err = EPTextureDestroy(texture);
    TEST_OK(err);
    
    return true;
}

// ============================================================================
// Sampler Tests
// ============================================================================

static bool test_sampler_create() {
    if (!g_device) return true;
    
    EPSamplerPtr sampler = nullptr;
    EPSamplerDesc desc = {};
    desc.min_filter = EP_FILTER_LINEAR;
    desc.mag_filter = EP_FILTER_LINEAR;
    desc.address_u = EP_ADDRESS_REPEAT;
    desc.address_v = EP_ADDRESS_REPEAT;
    desc.address_w = EP_ADDRESS_REPEAT;
    desc.compare_op = EP_COMPARE_ALWAYS;
    desc.max_anisotropy = 1.0f;
    
    EPError err = EPSamplerCreate(g_device, &desc, &sampler);
    TEST_OK(err);
    TEST_ASSERT(sampler != nullptr, "sampler should not be null");
    
    err = EPSamplerDestroy(sampler);
    TEST_OK(err);
    
    return true;
}

static bool test_sampler_anisotropic() {
    if (!g_device) return true;
    
    EPSamplerPtr sampler = nullptr;
    EPSamplerDesc desc = {};
    desc.min_filter = EP_FILTER_LINEAR;
    desc.mag_filter = EP_FILTER_LINEAR;
    desc.address_u = EP_ADDRESS_CLAMP_TO_EDGE;
    desc.address_v = EP_ADDRESS_CLAMP_TO_EDGE;
    desc.address_w = EP_ADDRESS_CLAMP_TO_EDGE;
    desc.compare_op = EP_COMPARE_ALWAYS;
    desc.max_anisotropy = 16.0f;
    
    EPError err = EPSamplerCreate(g_device, &desc, &sampler);
    TEST_OK(err);
    TEST_ASSERT(sampler != nullptr, "sampler should not be null");
    
    err = EPSamplerDestroy(sampler);
    TEST_OK(err);
    
    return true;
}

// ============================================================================
// Command Buffer Tests
// ============================================================================

static bool test_command_buffer_create() {
    if (!g_device) return true;
    
    EPCommandBufferPtr cmd = nullptr;
    EPError err = EPCommandBufferCreate(g_device, &cmd);
    TEST_OK(err);
    TEST_ASSERT(cmd != nullptr, "command buffer should not be null");
    
    err = EPCommandBufferDestroy(cmd);
    TEST_OK(err);
    
    return true;
}

static bool test_command_buffer_begin_end() {
    if (!g_device) return true;
    
    EPCommandBufferPtr cmd = nullptr;
    EPError err = EPCommandBufferCreate(g_device, &cmd);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd);
    TEST_OK(err);
    TEST_ASSERT(cmd->is_recording, "should be recording");
    
    err = EPCommandBufferEnd(cmd);
    TEST_OK(err);
    TEST_ASSERT(!cmd->is_recording, "should not be recording");
    
    err = EPCommandBufferDestroy(cmd);
    TEST_OK(err);
    
    return true;
}

static bool test_command_buffer_double_begin() {
    if (!g_device) return true;
    
    EPCommandBufferPtr cmd = nullptr;
    EPError err = EPCommandBufferCreate(g_device, &cmd);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd);
    TEST_OK(err);
    
    // Second begin should fail
    err = EPCommandBufferBegin(cmd);
    TEST_ASSERT(err.code == EP_E_INVALID_STATE, "double begin should fail");
    
    err = EPCommandBufferEnd(cmd);
    TEST_OK(err);
    
    err = EPCommandBufferDestroy(cmd);
    TEST_OK(err);
    
    return true;
}

static bool test_command_buffer_viewport_scissor() {
    if (!g_device) return true;
    
    EPCommandBufferPtr cmd = nullptr;
    EPError err = EPCommandBufferCreate(g_device, &cmd);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd);
    TEST_OK(err);
    
    err = EPCommandSetViewport(cmd, 0.0f, 0.0f, 1920.0f, 1080.0f, 0.0f, 1.0f);
    TEST_OK(err);
    
    err = EPCommandSetScissor(cmd, 0, 0, 1920, 1080);
    TEST_OK(err);
    
    err = EPCommandBufferEnd(cmd);
    TEST_OK(err);
    
    err = EPCommandBufferDestroy(cmd);
    TEST_OK(err);
    
    return true;
}

// ============================================================================
// Fence Tests
// ============================================================================

static bool test_fence_create() {
    if (!g_device) return true;
    
    EPFencePtr fence = nullptr;
    EPError err = EPFenceCreate(g_device, 0, &fence);
    TEST_OK(err);
    TEST_ASSERT(fence != nullptr, "fence should not be null");
    TEST_ASSERT(fence->event != nullptr, "fence should have event");
    
    err = EPFenceDestroy(fence);
    TEST_OK(err);
    
    return true;
}

static bool test_fence_signal_wait() {
    if (!g_device) return true;
    
    EPFencePtr fence = nullptr;
    EPError err = EPFenceCreate(g_device, 0, &fence);
    TEST_OK(err);
    
    // Signal fence from CPU
    err = EPFenceSignal(fence, 1);
    TEST_OK(err);
    
    // Wait should complete immediately
    err = EPFenceWait(fence, 1, 1000000000); // 1 second timeout
    TEST_OK(err);
    
    err = EPFenceDestroy(fence);
    TEST_OK(err);
    
    return true;
}

// ============================================================================
// Timeline Semaphore Tests
// ============================================================================

static bool test_timeline_semaphore_create() {
    if (!g_device) return true;
    
    EPTimelineSemaphorePtr sem = nullptr;
    EPError err = EPTimelineSemaphoreCreate(g_device, 0, &sem);
    TEST_OK(err);
    TEST_ASSERT(sem != nullptr, "semaphore should not be null");
    
    err = EPTimelineSemaphoreDestroy(sem);
    TEST_OK(err);
    
    return true;
}

static bool test_timeline_semaphore_get_value() {
    if (!g_device) return true;
    
    EPTimelineSemaphorePtr sem = nullptr;
    EPError err = EPTimelineSemaphoreCreate(g_device, 42, &sem);
    TEST_OK(err);
    
    uint64_t value = 0;
    err = EPTimelineSemaphoreGetValue(sem, &value);
    TEST_OK(err);
    TEST_ASSERT(value == 42, "initial value should be 42");
    
    err = EPTimelineSemaphoreDestroy(sem);
    TEST_OK(err);
    
    return true;
}

static bool test_timeline_semaphore_signal() {
    if (!g_device) return true;
    
    EPTimelineSemaphorePtr sem = nullptr;
    EPError err = EPTimelineSemaphoreCreate(g_device, 0, &sem);
    TEST_OK(err);
    
    err = EPTimelineSemaphoreSignal(sem, 100);
    TEST_OK(err);
    
    uint64_t value = 0;
    err = EPTimelineSemaphoreGetValue(sem, &value);
    TEST_OK(err);
    TEST_ASSERT(value == 100, "value should be 100 after signal");
    
    err = EPTimelineSemaphoreDestroy(sem);
    TEST_OK(err);
    
    return true;
}

// ============================================================================
// Descriptor Set Layout Tests
// ============================================================================

static bool test_descriptor_set_layout_create() {
    if (!g_device) return true;
    
    EPDescriptorBindingDesc bindings[2] = {};
    bindings[0].binding = 0;
    bindings[0].type = EP_DESCRIPTOR_UNIFORM_BUFFER;
    bindings[0].count = 1;
    bindings[0].stages = EP_STAGE_VERTEX_BIT | EP_STAGE_FRAGMENT_BIT;
    
    bindings[1].binding = 1;
    bindings[1].type = EP_DESCRIPTOR_SAMPLED_TEXTURE;
    bindings[1].count = 1;
    bindings[1].stages = EP_STAGE_FRAGMENT_BIT;
    
    // Create desc with flexible array member
    struct {
        EPDescriptorSetLayoutDesc header;
        EPDescriptorBindingDesc bindings[2];
    } desc = {};
    desc.header.binding_count = 2;
    desc.bindings[0] = bindings[0];
    desc.bindings[1] = bindings[1];
    
    EPDescriptorSetLayoutPtr layout = nullptr;
    EPError err = EPDescriptorSetLayoutCreate(g_device, &desc.header, &layout);
    TEST_OK(err);
    TEST_ASSERT(layout != nullptr, "layout should not be null");
    TEST_ASSERT(layout->bindings.size() == 2, "should have 2 bindings");
    
    err = EPDescriptorSetLayoutDestroy(layout);
    TEST_OK(err);
    
    return true;
}

// ============================================================================
// Descriptor Set Tests
// ============================================================================

static bool test_descriptor_set_create() {
    if (!g_device) return true;
    
    // Create layout first
    EPDescriptorBindingDesc binding = {};
    binding.binding = 0;
    binding.type = EP_DESCRIPTOR_UNIFORM_BUFFER;
    binding.count = 1;
    binding.stages = EP_STAGE_VERTEX_BIT;
    
    struct {
        EPDescriptorSetLayoutDesc header;
        EPDescriptorBindingDesc bindings[1];
    } layout_desc = {};
    layout_desc.header.binding_count = 1;
    layout_desc.bindings[0] = binding;
    
    EPDescriptorSetLayoutPtr layout = nullptr;
    EPError err = EPDescriptorSetLayoutCreate(g_device, &layout_desc.header, &layout);
    TEST_OK(err);
    
    // Create descriptor set
    EPDescriptorSetPtr set = nullptr;
    err = EPDescriptorSetCreate(g_device, layout, &set);
    TEST_OK(err);
    TEST_ASSERT(set != nullptr, "set should not be null");
    
    err = EPDescriptorSetDestroy(set);
    TEST_OK(err);
    
    err = EPDescriptorSetLayoutDestroy(layout);
    TEST_OK(err);
    
    return true;
}

static bool test_descriptor_set_update_buffer() {
    if (!g_device) return true;
    
    // Create layout
    EPDescriptorBindingDesc binding = {};
    binding.binding = 0;
    binding.type = EP_DESCRIPTOR_UNIFORM_BUFFER;
    binding.count = 1;
    binding.stages = EP_STAGE_VERTEX_BIT;
    
    struct {
        EPDescriptorSetLayoutDesc header;
        EPDescriptorBindingDesc bindings[1];
    } layout_desc = {};
    layout_desc.header.binding_count = 1;
    layout_desc.bindings[0] = binding;
    
    EPDescriptorSetLayoutPtr layout = nullptr;
    EPError err = EPDescriptorSetLayoutCreate(g_device, &layout_desc.header, &layout);
    TEST_OK(err);
    
    // Create set
    EPDescriptorSetPtr set = nullptr;
    err = EPDescriptorSetCreate(g_device, layout, &set);
    TEST_OK(err);
    
    // Create buffer
    EPBufferPtr buffer = nullptr;
    EPBufferDesc buf_desc = {};
    buf_desc.size = 256;
    buf_desc.usage = EP_BUFFER_USAGE_UNIFORM_BIT;
    buf_desc.host_visible = true;
    
    err = EPBufferCreate(g_device, &buf_desc, &buffer);
    TEST_OK(err);
    
    // Update descriptor
    err = EPDescriptorSetUpdateBuffer(set, 0, buffer, 0, 256);
    TEST_OK(err);
    
    // Cleanup
    err = EPBufferDestroy(buffer);
    TEST_OK(err);
    err = EPDescriptorSetDestroy(set);
    TEST_OK(err);
    err = EPDescriptorSetLayoutDestroy(layout);
    TEST_OK(err);
    
    return true;
}

// ============================================================================
// Pipeline Layout Tests
// ============================================================================

static bool test_pipeline_layout_create() {
    if (!g_device) return true;
    
    // Create layout with push constants only
    struct {
        EPPipelineLayoutDesc header;
        EPDescriptorSetLayoutPtr layouts[1];
    } desc = {};
    desc.header.set_layout_count = 0;
    desc.header.push_constant_size = 128;
    desc.header.push_constant_stages = EP_STAGE_VERTEX_BIT | EP_STAGE_FRAGMENT_BIT;
    
    EPPipelineLayoutPtr layout = nullptr;
    EPError err = EPPipelineLayoutCreate(g_device, &desc.header, &layout);
    TEST_OK(err);
    TEST_ASSERT(layout != nullptr, "layout should not be null");
    TEST_ASSERT(layout->root_signature != nullptr, "should have root signature");
    
    err = EPPipelineLayoutDestroy(layout);
    TEST_OK(err);
    
    return true;
}

// ============================================================================
// Queue Submit Tests
// ============================================================================

static bool test_queue_submit_empty() {
    if (!g_device) return true;
    
    EPQueuePtr queue = nullptr;
    EPError err = EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &queue);
    TEST_OK(err);
    
    EPSubmitInfo submit = {};
    submit.command_buffer_count = 0;
    submit.command_buffers = nullptr;
    
    err = EPQueueSubmit(queue, &submit);
    TEST_OK(err);
    
    delete queue;
    return true;
}

static bool test_queue_wait_idle() {
    if (!g_device) return true;
    
    EPQueuePtr queue = nullptr;
    EPError err = EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &queue);
    TEST_OK(err);
    
    err = EPQueueWaitIdle(queue);
    TEST_OK(err);
    
    delete queue;
    return true;
}

static bool test_queue_submit_with_command_buffer() {
    if (!g_device) return true;
    
    // Create command buffer
    EPCommandBufferPtr cmd = nullptr;
    EPError err = EPCommandBufferCreate(g_device, &cmd);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd);
    TEST_OK(err);
    
    // Just set viewport/scissor
    err = EPCommandSetViewport(cmd, 0, 0, 800, 600, 0, 1);
    TEST_OK(err);
    err = EPCommandSetScissor(cmd, 0, 0, 800, 600);
    TEST_OK(err);
    
    err = EPCommandBufferEnd(cmd);
    TEST_OK(err);
    
    // Submit
    EPQueuePtr queue = nullptr;
    err = EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &queue);
    TEST_OK(err);
    
    EPCommandBufferPtr cmds[] = { cmd };
    EPSubmitInfo submit = {};
    submit.command_buffer_count = 1;
    submit.command_buffers = cmds;
    
    err = EPQueueSubmit(queue, &submit);
    TEST_OK(err);
    
    err = EPQueueWaitIdle(queue);
    TEST_OK(err);
    
    // Cleanup
    delete queue;
    err = EPCommandBufferDestroy(cmd);
    TEST_OK(err);
    
    return true;
}

// ============================================================================
// Format Conversion Tests
// ============================================================================

static bool test_format_conversions() {
    // Test DXGI format conversion
    TEST_ASSERT(ep_to_dxgi_format(EP_FORMAT_RGBA8_UNORM) == DXGI_FORMAT_R8G8B8A8_UNORM,
                "RGBA8 conversion");
    TEST_ASSERT(ep_to_dxgi_format(EP_FORMAT_BGRA8_UNORM) == DXGI_FORMAT_B8G8R8A8_UNORM,
                "BGRA8 conversion");
    TEST_ASSERT(ep_to_dxgi_format(EP_FORMAT_D32_FLOAT) == DXGI_FORMAT_D32_FLOAT,
                "D32 conversion");
    
    // Test address mode conversion
    TEST_ASSERT(ep_to_d3d12_address(EP_ADDRESS_REPEAT) == D3D12_TEXTURE_ADDRESS_MODE_WRAP,
                "repeat address mode");
    TEST_ASSERT(ep_to_d3d12_address(EP_ADDRESS_CLAMP_TO_EDGE) == D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
                "clamp address mode");
    
    // Test compare function conversion
    TEST_ASSERT(ep_to_d3d12_compare(EP_COMPARE_LESS) == D3D12_COMPARISON_FUNC_LESS,
                "less compare");
    TEST_ASSERT(ep_to_d3d12_compare(EP_COMPARE_ALWAYS) == D3D12_COMPARISON_FUNC_ALWAYS,
                "always compare");
    
    return true;
}

// ============================================================================
// Main
// ============================================================================

int main() {
    printf("=== Enthrall D3D12 Backend Tests ===\n\n");
    
    // Format conversion tests (don't need GPU)
    RUN_TEST(test_format_conversions);
    
    // Platform tests
    RUN_TEST(test_platform_create_destroy);
    RUN_TEST(test_platform_null_argument);
    
    // Instance tests
    RUN_TEST(test_instance_create_destroy);
    RUN_TEST(test_instance_null_arguments);
    
    // Setup test environment for remaining tests
    printf("\nSetting up test environment...\n");
    if (!setup_test_environment()) {
        printf("Failed to setup test environment!\n");
        return 1;
    }
    printf("Test environment ready.\n\n");
    
    // Instance/Adapter tests
    RUN_TEST(test_instance_enumerate_adapters);
    RUN_TEST(test_adapter_properties);
    
    // Device tests
    RUN_TEST(test_device_create_destroy);
    RUN_TEST(test_device_get_queues);
    RUN_TEST(test_device_queue_invalid_index);
    
    // Buffer tests
    RUN_TEST(test_buffer_create_destroy);
    RUN_TEST(test_buffer_host_visible);
    RUN_TEST(test_buffer_storage);
    
    // Texture tests
    RUN_TEST(test_texture_2d_create);
    RUN_TEST(test_texture_render_target);
    RUN_TEST(test_texture_depth_stencil);
    
    // Sampler tests
    RUN_TEST(test_sampler_create);
    RUN_TEST(test_sampler_anisotropic);
    
    // Command buffer tests
    RUN_TEST(test_command_buffer_create);
    RUN_TEST(test_command_buffer_begin_end);
    RUN_TEST(test_command_buffer_double_begin);
    RUN_TEST(test_command_buffer_viewport_scissor);
    
    // Fence tests
    RUN_TEST(test_fence_create);
    RUN_TEST(test_fence_signal_wait);
    
    // Timeline semaphore tests
    RUN_TEST(test_timeline_semaphore_create);
    RUN_TEST(test_timeline_semaphore_get_value);
    RUN_TEST(test_timeline_semaphore_signal);
    
    // Descriptor tests
    RUN_TEST(test_descriptor_set_layout_create);
    RUN_TEST(test_descriptor_set_create);
    RUN_TEST(test_descriptor_set_update_buffer);
    
    // Pipeline layout tests
    RUN_TEST(test_pipeline_layout_create);
    
    // Queue tests
    RUN_TEST(test_queue_submit_empty);
    RUN_TEST(test_queue_wait_idle);
    RUN_TEST(test_queue_submit_with_command_buffer);
    
    // Teardown
    printf("\nTearing down test environment...\n");
    teardown_test_environment();
    
    // Summary
    printf("\n=== Test Summary ===\n");
    printf("Passed: %d\n", passed);
    printf("Failed: %d\n", failed);
    printf("Total:  %d\n", passed + failed);
    
    return failed > 0 ? 1 : 0;
}
