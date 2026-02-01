// test_metal_integration.m - Integration tests for Metal backend
// Run: zig build test-metal
// These tests exercise the actual Metal API without any mocking

#import <Metal/Metal.h>
#import <Foundation/Foundation.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "gcraft_instance.h"
#include "gcraft_device.h"
#include "gcraft_resource.h"
#include "gcraft_command.h"
#include "gcraft_descriptor.h"
#include "gcraft_pipeline.h"
#include "gcraft_sync.h"

// =============================================================================
// Test Utilities
// =============================================================================

#define TEST_ASSERT(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s (line %d): %s\n", __func__, __LINE__, msg); \
        return 1; \
    } \
} while (0)

#define TEST_ASSERT_OK(err) do { \
    if ((err).code != GC_E_OK) { \
        fprintf(stderr, "FAIL: %s (line %d): error %d: %s\n", \
                __func__, __LINE__, (err).code, (err).message ? (err).message : "null"); \
        return 1; \
    } \
} while (0)

#define TEST_ASSERT_ERR(err, expected_code) do { \
    if ((err).code != (expected_code)) { \
        fprintf(stderr, "FAIL: %s (line %d): expected error %d but got %d\n", \
                __func__, __LINE__, (expected_code), (err).code); \
        return 1; \
    } \
} while (0)

#define RUN_TEST(test_fn) do { \
    printf("Running %s... ", #test_fn); \
    fflush(stdout); \
    int result = test_fn(); \
    if (result == 0) { \
        printf("PASS\n"); \
        tests_passed++; \
    } else { \
        tests_failed++; \
    } \
    tests_total++; \
} while (0)

static int tests_total = 0;
static int tests_passed = 0;
static int tests_failed = 0;

// =============================================================================
// Shared Test Fixtures
// =============================================================================

typedef struct {
    GCInstancePtr instance;
    GCAdapterPtr adapter;
    GCDevicePtr device;
    GCQueuePtr graphics_queue;
    GCQueuePtr compute_queue;
} TestContext;

static int setup_test_context(TestContext *ctx) {
    GCError err;
    
    // Create instance
    GCInstanceDesc inst_desc = {
        .enable_backends = GC_BACKEND_METAL_BIT,
        .enable_validation = true,
        .enable_debug_names = true,
    };
    err = GCInstanceCreate(&inst_desc, &ctx->instance);
    if (err.code != GC_E_OK) return -1;
    
    // Enumerate adapters
    uint32_t adapter_count = 0;
    err = GCInstanceEnumerateAdapters(ctx->instance, &adapter_count, NULL);
    if (err.code != GC_E_OK || adapter_count == 0) return -1;
    
    GCAdapterPtr *adapters = malloc(adapter_count * sizeof(GCAdapterPtr));
    err = GCInstanceEnumerateAdapters(ctx->instance, &adapter_count, adapters);
    if (err.code != GC_E_OK) {
        free(adapters);
        return -1;
    }
    ctx->adapter = adapters[0];
    free(adapters);
    
    // Create device
    GCDeviceDesc dev_desc = {
        .preferred_backend = GC_BACKEND_METAL,
        .required_features = GC_FEATURE_COMPUTE_BIT,
        .enable_validation = true,
        .enable_debug_names = true,
    };
    err = GCDeviceCreate(ctx->adapter, &dev_desc, &ctx->device);
    if (err.code != GC_E_OK) return -1;
    
    // Get queues
    err = GCDeviceGetQueue(ctx->device, GC_QUEUE_GRAPHICS, 0, &ctx->graphics_queue);
    if (err.code != GC_E_OK) return -1;
    
    err = GCDeviceGetQueue(ctx->device, GC_QUEUE_COMPUTE, 0, &ctx->compute_queue);
    if (err.code != GC_E_OK) return -1;
    
    return 0;
}

static void teardown_test_context(TestContext *ctx) {
    if (ctx->device) GCDeviceDestroy(ctx->device);
    if (ctx->instance) GCInstanceDestroy(ctx->instance);
}

// =============================================================================
// Instance Tests
// =============================================================================

static int test_instance_create_destroy(void) {
    GCError err;
    GCInstancePtr instance = NULL;
    
    GCInstanceDesc desc = {
        .enable_backends = GC_BACKEND_METAL_BIT,
        .enable_validation = true,
        .enable_debug_names = true,
    };
    
    err = GCInstanceCreate(&desc, &instance);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(instance != NULL, "instance should not be NULL");
    
    err = GCInstanceDestroy(instance);
    TEST_ASSERT_OK(err);
    
    return 0;
}

static int test_instance_enumerate_adapters(void) {
    GCError err;
    GCInstancePtr instance = NULL;
    
    GCInstanceDesc desc = {
        .enable_backends = GC_BACKEND_METAL_BIT,
        .enable_validation = false,
        .enable_debug_names = false,
    };
    
    err = GCInstanceCreate(&desc, &instance);
    TEST_ASSERT_OK(err);
    
    // First call: get count
    uint32_t count = 0;
    err = GCInstanceEnumerateAdapters(instance, &count, NULL);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(count > 0, "should have at least one Metal adapter");
    
    // Second call: get adapters
    GCAdapterPtr *adapters = malloc(count * sizeof(GCAdapterPtr));
    err = GCInstanceEnumerateAdapters(instance, &count, adapters);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(adapters[0] != NULL, "first adapter should not be NULL");
    
    free(adapters);
    GCInstanceDestroy(instance);
    return 0;
}

static int test_adapter_properties(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCAdapterProperties props;
    err = GCAdapterGetProperties(ctx.adapter, &props);
    TEST_ASSERT_OK(err);
    
    // Verify properties are populated
    TEST_ASSERT(strlen(props.name) > 0, "adapter name should not be empty");
    TEST_ASSERT(props.backends & GC_BACKEND_METAL_BIT, "should report Metal support");
    TEST_ASSERT(props.features & GC_FEATURE_COMPUTE_BIT, "should support compute");
    TEST_ASSERT(props.limits.max_texture_dimension_2d > 0, "max texture 2D should be > 0");
    TEST_ASSERT(props.limits.max_threads_per_threadgroup > 0, "max threads should be > 0");
    
    printf("\n  Adapter: %s\n", props.name);
    printf("  Max texture 2D: %u\n", props.limits.max_texture_dimension_2d);
    printf("  Features: 0x%x\n", props.features);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Device Tests
// =============================================================================

static int test_device_create_destroy(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    TEST_ASSERT(ctx.device != NULL, "device should not be NULL");
    TEST_ASSERT(ctx.graphics_queue != NULL, "graphics queue should not be NULL");
    TEST_ASSERT(ctx.compute_queue != NULL, "compute queue should not be NULL");
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_device_queues(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Get transfer queue
    GCQueuePtr transfer_queue;
    err = GCDeviceGetQueue(ctx.device, GC_QUEUE_TRANSFER, 0, &transfer_queue);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(transfer_queue != NULL, "transfer queue should not be NULL");
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Buffer Tests
// =============================================================================

static int test_buffer_create_host_visible(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCBufferDesc desc = {
        .size = 1024,
        .usage = GC_BUFFER_USAGE_UNIFORM_BIT | GC_BUFFER_USAGE_STORAGE_BIT,
        .host_visible = true,
    };
    
    GCBufferPtr buffer = NULL;
    err = GCBufferCreate(ctx.device, &desc, &buffer);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(buffer != NULL, "buffer should not be NULL");
    
    err = GCBufferDestroy(buffer);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_buffer_create_device_local(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCBufferDesc desc = {
        .size = 4096,
        .usage = GC_BUFFER_USAGE_STORAGE_BIT | GC_BUFFER_USAGE_TRANSFER_DST_BIT,
        .host_visible = false,
    };
    
    GCBufferPtr buffer = NULL;
    err = GCBufferCreate(ctx.device, &desc, &buffer);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(buffer != NULL, "buffer should not be NULL");
    
    err = GCBufferDestroy(buffer);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_buffer_various_sizes(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Test various buffer sizes
    uint64_t sizes[] = {1, 256, 4096, 65536, 1024 * 1024};
    for (int i = 0; i < sizeof(sizes) / sizeof(sizes[0]); i++) {
        GCBufferDesc desc = {
            .size = sizes[i],
            .usage = GC_BUFFER_USAGE_STORAGE_BIT,
            .host_visible = true,
        };
        
        GCBufferPtr buffer = NULL;
        err = GCBufferCreate(ctx.device, &desc, &buffer);
        TEST_ASSERT_OK(err);
        
        err = GCBufferDestroy(buffer);
        TEST_ASSERT_OK(err);
    }
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Texture Tests
// =============================================================================

static int test_texture_create_2d(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCTextureDesc desc = {
        .dimension = GC_TEXTURE_DIM_2D,
        .format = GC_FORMAT_RGBA8_UNORM,
        .width = 512,
        .height = 512,
        .depth = 1,
        .mip_levels = 1,
        .array_layers = 1,
        .usage = GC_TEXTURE_USAGE_SAMPLED_BIT | GC_TEXTURE_USAGE_STORAGE_BIT,
    };
    
    GCTexturePtr texture = NULL;
    err = GCTextureCreate(ctx.device, &desc, &texture);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(texture != NULL, "texture should not be NULL");
    
    err = GCTextureDestroy(texture);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_texture_create_depth(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCTextureDesc desc = {
        .dimension = GC_TEXTURE_DIM_2D,
        .format = GC_FORMAT_D32_FLOAT,
        .width = 1024,
        .height = 1024,
        .depth = 1,
        .mip_levels = 1,
        .array_layers = 1,
        .usage = GC_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT | GC_TEXTURE_USAGE_SAMPLED_BIT,
    };
    
    GCTexturePtr texture = NULL;
    err = GCTextureCreate(ctx.device, &desc, &texture);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(texture != NULL, "texture should not be NULL");
    
    err = GCTextureDestroy(texture);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_texture_create_cube(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Note: Metal cube textures have implicit 6 faces, array_layers should be 1
    GCTextureDesc desc = {
        .dimension = GC_TEXTURE_DIM_CUBE,
        .format = GC_FORMAT_RGBA16_FLOAT,
        .width = 256,
        .height = 256,
        .depth = 1,
        .mip_levels = 1,
        .array_layers = 1,  // Metal: 6 faces are implicit for cube type
        .usage = GC_TEXTURE_USAGE_SAMPLED_BIT,
    };
    
    GCTexturePtr texture = NULL;
    err = GCTextureCreate(ctx.device, &desc, &texture);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(texture != NULL, "texture should not be NULL");
    
    err = GCTextureDestroy(texture);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_texture_create_3d(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCTextureDesc desc = {
        .dimension = GC_TEXTURE_DIM_3D,
        .format = GC_FORMAT_RGBA8_UNORM,
        .width = 64,
        .height = 64,
        .depth = 64,
        .mip_levels = 1,
        .array_layers = 1,
        .usage = GC_TEXTURE_USAGE_STORAGE_BIT,
    };
    
    GCTexturePtr texture = NULL;
    err = GCTextureCreate(ctx.device, &desc, &texture);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(texture != NULL, "texture should not be NULL");
    
    err = GCTextureDestroy(texture);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Sampler Tests
// =============================================================================

static int test_sampler_create(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCSamplerDesc desc = {
        .min_filter = GC_FILTER_LINEAR,
        .mag_filter = GC_FILTER_LINEAR,
        .address_u = GC_ADDRESS_REPEAT,
        .address_v = GC_ADDRESS_REPEAT,
        .address_w = GC_ADDRESS_CLAMP_TO_EDGE,
        .compare_op = GC_COMPARE_NEVER,
        .max_anisotropy = 1.0f,
    };
    
    GCSamplerPtr sampler = NULL;
    err = GCSamplerCreate(ctx.device, &desc, &sampler);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(sampler != NULL, "sampler should not be NULL");
    
    err = GCSamplerDestroy(sampler);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_sampler_anisotropic(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCSamplerDesc desc = {
        .min_filter = GC_FILTER_LINEAR,
        .mag_filter = GC_FILTER_LINEAR,
        .address_u = GC_ADDRESS_REPEAT,
        .address_v = GC_ADDRESS_REPEAT,
        .address_w = GC_ADDRESS_REPEAT,
        .compare_op = GC_COMPARE_NEVER,
        .max_anisotropy = 16.0f,
    };
    
    GCSamplerPtr sampler = NULL;
    err = GCSamplerCreate(ctx.device, &desc, &sampler);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(sampler != NULL, "sampler should not be NULL");
    
    err = GCSamplerDestroy(sampler);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Shader Library Tests
// =============================================================================

static const char *simple_compute_shader = 
    "#include <metal_stdlib>\n"
    "using namespace metal;\n"
    "kernel void main_kernel(device float *buffer [[buffer(0)]],\n"
    "                        uint id [[thread_position_in_grid]]) {\n"
    "    buffer[id] = buffer[id] * 2.0f;\n"
    "}\n";

static const char *simple_vertex_fragment_shader =
    "#include <metal_stdlib>\n"
    "using namespace metal;\n"
    "struct VertexOut {\n"
    "    float4 position [[position]];\n"
    "};\n"
    "vertex VertexOut vertex_main(uint vid [[vertex_id]]) {\n"
    "    VertexOut out;\n"
    "    float2 positions[3] = { {0.0, 0.5}, {-0.5, -0.5}, {0.5, -0.5} };\n"
    "    out.position = float4(positions[vid], 0.0, 1.0);\n"
    "    return out;\n"
    "}\n"
    "fragment float4 fragment_main(VertexOut in [[stage_in]]) {\n"
    "    return float4(1.0, 0.0, 0.0, 1.0);\n"
    "}\n";

static const char *invalid_shader = 
    "#include <metal_stdlib>\n"
    "using namespace metal;\n"
    "kernel void main_kernel( {\n"  // Syntax error
    "}\n";

static int test_shader_library_create(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCShaderLibraryDesc desc = {
        .format = GC_SHADER_MSL,
        .data = (const uint8_t *)simple_compute_shader,
        .size = strlen(simple_compute_shader),
        .label = "test_compute_shader",
    };
    
    GCShaderLibraryPtr library = NULL;
    err = GCShaderLibraryCreate(ctx.device, &desc, &library);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(library != NULL, "library should not be NULL");
    
    err = GCShaderLibraryDestroy(library);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_shader_library_invalid(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCShaderLibraryDesc desc = {
        .format = GC_SHADER_MSL,
        .data = (const uint8_t *)invalid_shader,
        .size = strlen(invalid_shader),
        .label = "invalid_shader",
    };
    
    GCShaderLibraryPtr library = NULL;
    err = GCShaderLibraryCreate(ctx.device, &desc, &library);
    // Should fail due to invalid shader syntax
    TEST_ASSERT(err.code != GC_E_OK, "should fail for invalid shader");
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Descriptor Tests
// =============================================================================

static int test_descriptor_set_layout_create(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Create layout with one binding
    GCDescriptorSetLayoutDesc *desc = malloc(
        sizeof(GCDescriptorSetLayoutDesc) + 1 * sizeof(GCDescriptorBindingDesc));
    desc->binding_count = 1;
    desc->bindings[0] = (GCDescriptorBindingDesc){
        .binding = 0,
        .type = GC_DESCRIPTOR_STORAGE_BUFFER,
        .count = 1,
        .stages = GC_STAGE_COMPUTE_BIT,
    };
    
    GCDescriptorSetLayoutPtr layout = NULL;
    err = GCDescriptorSetLayoutCreate(ctx.device, desc, &layout);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(layout != NULL, "layout should not be NULL");
    
    err = GCDescriptorSetLayoutDestroy(layout);
    TEST_ASSERT_OK(err);
    
    free(desc);
    teardown_test_context(&ctx);
    return 0;
}

static int test_descriptor_set_create_and_update(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Create layout
    GCDescriptorSetLayoutDesc *layout_desc = malloc(
        sizeof(GCDescriptorSetLayoutDesc) + 1 * sizeof(GCDescriptorBindingDesc));
    layout_desc->binding_count = 1;
    layout_desc->bindings[0] = (GCDescriptorBindingDesc){
        .binding = 0,
        .type = GC_DESCRIPTOR_STORAGE_BUFFER,
        .count = 1,
        .stages = GC_STAGE_COMPUTE_BIT,
    };
    
    GCDescriptorSetLayoutPtr layout = NULL;
    err = GCDescriptorSetLayoutCreate(ctx.device, layout_desc, &layout);
    TEST_ASSERT_OK(err);
    
    // Create descriptor set
    GCDescriptorSetPtr set = NULL;
    err = GCDescriptorSetCreate(ctx.device, layout, &set);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(set != NULL, "set should not be NULL");
    
    // Create buffer to bind
    GCBufferDesc buf_desc = {
        .size = 1024,
        .usage = GC_BUFFER_USAGE_STORAGE_BIT,
        .host_visible = true,
    };
    GCBufferPtr buffer = NULL;
    err = GCBufferCreate(ctx.device, &buf_desc, &buffer);
    TEST_ASSERT_OK(err);
    
    // Update descriptor set
    err = GCDescriptorSetUpdateBuffer(set, 0, buffer, 0, 1024);
    TEST_ASSERT_OK(err);
    
    // Cleanup
    GCBufferDestroy(buffer);
    GCDescriptorSetDestroy(set);
    GCDescriptorSetLayoutDestroy(layout);
    free(layout_desc);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_pipeline_layout_create(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Create descriptor set layout
    GCDescriptorSetLayoutDesc *layout_desc = malloc(
        sizeof(GCDescriptorSetLayoutDesc) + 1 * sizeof(GCDescriptorBindingDesc));
    layout_desc->binding_count = 1;
    layout_desc->bindings[0] = (GCDescriptorBindingDesc){
        .binding = 0,
        .type = GC_DESCRIPTOR_STORAGE_BUFFER,
        .count = 1,
        .stages = GC_STAGE_COMPUTE_BIT,
    };
    
    GCDescriptorSetLayoutPtr set_layout = NULL;
    err = GCDescriptorSetLayoutCreate(ctx.device, layout_desc, &set_layout);
    TEST_ASSERT_OK(err);
    
    // Create pipeline layout
    GCPipelineLayoutDesc *pipe_desc = malloc(
        sizeof(GCPipelineLayoutDesc) + 1 * sizeof(GCDescriptorSetLayoutPtr));
    pipe_desc->set_layout_count = 1;
    pipe_desc->push_constant_size = 64;
    pipe_desc->push_constant_stages = GC_STAGE_COMPUTE_BIT;
    pipe_desc->set_layouts[0] = set_layout;
    
    GCPipelineLayoutPtr pipe_layout = NULL;
    err = GCPipelineLayoutCreate(ctx.device, pipe_desc, &pipe_layout);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(pipe_layout != NULL, "pipeline layout should not be NULL");
    
    // Cleanup
    GCPipelineLayoutDestroy(pipe_layout);
    GCDescriptorSetLayoutDestroy(set_layout);
    free(pipe_desc);
    free(layout_desc);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Pipeline Tests
// =============================================================================

static int test_compute_pipeline_create(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Create shader library
    GCShaderLibraryDesc lib_desc = {
        .format = GC_SHADER_MSL,
        .data = (const uint8_t *)simple_compute_shader,
        .size = strlen(simple_compute_shader),
        .label = "compute_test",
    };
    
    GCShaderLibraryPtr library = NULL;
    err = GCShaderLibraryCreate(ctx.device, &lib_desc, &library);
    TEST_ASSERT_OK(err);
    
    // Create pipeline layout
    GCPipelineLayoutDesc *layout_desc = malloc(sizeof(GCPipelineLayoutDesc));
    layout_desc->set_layout_count = 0;
    layout_desc->push_constant_size = 0;
    layout_desc->push_constant_stages = 0;
    
    GCPipelineLayoutPtr layout = NULL;
    err = GCPipelineLayoutCreate(ctx.device, layout_desc, &layout);
    TEST_ASSERT_OK(err);
    
    // Create compute pipeline
    GCComputePipelineDesc pipe_desc = {
        .library = library,
        .entry = "main_kernel",
        .layout = layout,
    };
    
    GCComputePipelinePtr pipeline = NULL;
    err = GCComputePipelineCreate(ctx.device, &pipe_desc, &pipeline);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(pipeline != NULL, "pipeline should not be NULL");
    
    // Cleanup
    GCComputePipelineDestroy(pipeline);
    GCPipelineLayoutDestroy(layout);
    GCShaderLibraryDestroy(library);
    free(layout_desc);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_render_pipeline_create(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Create shader library
    GCShaderLibraryDesc lib_desc = {
        .format = GC_SHADER_MSL,
        .data = (const uint8_t *)simple_vertex_fragment_shader,
        .size = strlen(simple_vertex_fragment_shader),
        .label = "render_test",
    };
    
    GCShaderLibraryPtr library = NULL;
    err = GCShaderLibraryCreate(ctx.device, &lib_desc, &library);
    TEST_ASSERT_OK(err);
    
    // Create pipeline layout
    GCPipelineLayoutDesc *layout_desc = malloc(sizeof(GCPipelineLayoutDesc));
    layout_desc->set_layout_count = 0;
    layout_desc->push_constant_size = 0;
    layout_desc->push_constant_stages = 0;
    
    GCPipelineLayoutPtr layout = NULL;
    err = GCPipelineLayoutCreate(ctx.device, layout_desc, &layout);
    TEST_ASSERT_OK(err);
    
    // Create render pipeline
    GCRenderPipelineDesc pipe_desc = {
        .library = library,
        .vertex_entry = "vertex_main",
        .fragment_entry = "fragment_main",
        .mesh_entry = NULL,
        .task_entry = NULL,
        .color_formats = {GC_FORMAT_RGBA8_UNORM},
        .color_format_count = 1,
        .depth_format = GC_FORMAT_D32_FLOAT,
        .stencil_format = 0,
        .attributes = NULL,
        .attribute_count = 0,
        .bindings = NULL,
        .binding_count = 0,
        .raster_state = {
            .depth_test_enable = true,
            .depth_write_enable = true,
            .depth_compare = GC_COMPARE_LESS,
        },
        .layout = layout,
    };
    
    GCRenderPipelinePtr pipeline = NULL;
    err = GCRenderPipelineCreate(ctx.device, &pipe_desc, &pipeline);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(pipeline != NULL, "pipeline should not be NULL");
    
    // Cleanup
    GCRenderPipelineDestroy(pipeline);
    GCPipelineLayoutDestroy(layout);
    GCShaderLibraryDestroy(library);
    free(layout_desc);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Command Buffer Tests
// =============================================================================

static int test_command_buffer_create(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCCommandBufferPtr cmd = NULL;
    err = GCCommandBufferCreate(ctx.device, &cmd);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(cmd != NULL, "command buffer should not be NULL");
    
    err = GCCommandBufferDestroy(cmd);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_command_buffer_begin_end(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCCommandBufferPtr cmd = NULL;
    err = GCCommandBufferCreate(ctx.device, &cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferBegin(cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferEnd(cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferDestroy(cmd);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Synchronization Tests
// =============================================================================

static int test_fence_create_destroy(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCFencePtr fence = NULL;
    err = GCFenceCreate(ctx.device, 0, &fence);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(fence != NULL, "fence should not be NULL");
    
    err = GCFenceDestroy(fence);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_fence_signal_wait(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCFencePtr fence = NULL;
    err = GCFenceCreate(ctx.device, 0, &fence);
    TEST_ASSERT_OK(err);
    
    // Signal the fence from CPU
    err = GCFenceSignal(fence, 1);
    TEST_ASSERT_OK(err);
    
    // Wait should return immediately
    err = GCFenceWait(fence, 1, 1000000000); // 1 second timeout
    TEST_ASSERT_OK(err);
    
    err = GCFenceDestroy(fence);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_timeline_semaphore_create_destroy(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCTimelineSemaphorePtr sem = NULL;
    err = GCTimelineSemaphoreCreate(ctx.device, 0, &sem);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(sem != NULL, "semaphore should not be NULL");
    
    err = GCTimelineSemaphoreDestroy(sem);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

static int test_timeline_semaphore_get_value(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCTimelineSemaphorePtr sem = NULL;
    err = GCTimelineSemaphoreCreate(ctx.device, 42, &sem);
    TEST_ASSERT_OK(err);
    
    uint64_t value = 0;
    err = GCTimelineSemaphoreGetValue(sem, &value);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(value == 42, "initial value should be 42");
    
    // Signal to a new value
    err = GCTimelineSemaphoreSignal(sem, 100);
    TEST_ASSERT_OK(err);
    
    err = GCTimelineSemaphoreGetValue(sem, &value);
    TEST_ASSERT_OK(err);
    TEST_ASSERT(value == 100, "value should be 100 after signal");
    
    err = GCTimelineSemaphoreDestroy(sem);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// GPU Compute Execution Tests (Real Data Verification)
// =============================================================================

static const char *double_values_shader = 
    "#include <metal_stdlib>\n"
    "using namespace metal;\n"
    "kernel void double_values(device float *data [[buffer(0)]],\n"
    "                          uint id [[thread_position_in_grid]]) {\n"
    "    data[id] = data[id] * 2.0f;\n"
    "}\n";

static int test_compute_execution_verify_output(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    const uint32_t element_count = 256;
    const uint64_t buffer_size = element_count * sizeof(float);
    
    // Create host-visible buffer with test data
    GCBufferDesc buf_desc = {
        .size = buffer_size,
        .usage = GC_BUFFER_USAGE_STORAGE_BIT,
        .host_visible = true,
    };
    GCBufferPtr buffer = NULL;
    err = GCBufferCreate(ctx.device, &buf_desc, &buffer);
    TEST_ASSERT_OK(err);
    
    // Write initial data to buffer - we need to access the internal Metal buffer
    // This test requires access to the underlying MTLBuffer to write data
    // For now, we use a workaround by accessing the internal structure
    {
        @autoreleasepool {
            // Access internal buffer (this is an integration test, so we can peek)
            struct GCBuffer {
                void *ep_device;
                id<MTLBuffer> mtl_buffer;
                uint64_t size;
                uint32_t usage;
                bool host_visible;
            };
            struct GCBuffer *buf = (struct GCBuffer *)buffer;
            
            float *data = [buf->mtl_buffer contents];
            for (uint32_t i = 0; i < element_count; i++) {
                data[i] = (float)i;  // 0, 1, 2, 3, ...
            }
        }
    }
    
    // Create shader library
    GCShaderLibraryDesc lib_desc = {
        .format = GC_SHADER_MSL,
        .data = (const uint8_t *)double_values_shader,
        .size = strlen(double_values_shader),
        .label = "double_values",
    };
    GCShaderLibraryPtr library = NULL;
    err = GCShaderLibraryCreate(ctx.device, &lib_desc, &library);
    TEST_ASSERT_OK(err);
    
    // Create descriptor set layout
    GCDescriptorSetLayoutDesc *layout_desc = malloc(
        sizeof(GCDescriptorSetLayoutDesc) + 1 * sizeof(GCDescriptorBindingDesc));
    layout_desc->binding_count = 1;
    layout_desc->bindings[0] = (GCDescriptorBindingDesc){
        .binding = 0,
        .type = GC_DESCRIPTOR_STORAGE_BUFFER,
        .count = 1,
        .stages = GC_STAGE_COMPUTE_BIT,
    };
    GCDescriptorSetLayoutPtr set_layout = NULL;
    err = GCDescriptorSetLayoutCreate(ctx.device, layout_desc, &set_layout);
    TEST_ASSERT_OK(err);
    
    // Create pipeline layout
    GCPipelineLayoutDesc *pipe_layout_desc = malloc(
        sizeof(GCPipelineLayoutDesc) + sizeof(GCDescriptorSetLayoutPtr));
    pipe_layout_desc->set_layout_count = 1;
    pipe_layout_desc->push_constant_size = 0;
    pipe_layout_desc->push_constant_stages = 0;
    pipe_layout_desc->set_layouts[0] = set_layout;
    GCPipelineLayoutPtr pipe_layout = NULL;
    err = GCPipelineLayoutCreate(ctx.device, pipe_layout_desc, &pipe_layout);
    TEST_ASSERT_OK(err);
    
    // Create descriptor set
    GCDescriptorSetPtr desc_set = NULL;
    err = GCDescriptorSetCreate(ctx.device, set_layout, &desc_set);
    TEST_ASSERT_OK(err);
    
    // Update descriptor set with buffer
    err = GCDescriptorSetUpdateBuffer(desc_set, 0, buffer, 0, buffer_size);
    TEST_ASSERT_OK(err);
    
    // Create compute pipeline
    GCComputePipelineDesc pipe_desc = {
        .library = library,
        .entry = "double_values",
        .layout = pipe_layout,
    };
    GCComputePipelinePtr pipeline = NULL;
    err = GCComputePipelineCreate(ctx.device, &pipe_desc, &pipeline);
    TEST_ASSERT_OK(err);
    
    // Create command buffer and record
    GCCommandBufferPtr cmd = NULL;
    err = GCCommandBufferCreate(ctx.device, &cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferBegin(cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBindComputePipeline(cmd, pipeline);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBindDescriptorSet(cmd, pipe_layout, 0, desc_set);
    TEST_ASSERT_OK(err);
    
    // Dispatch: element_count / 32 (assuming threadgroup size 32)
    uint32_t groups = (element_count + 31) / 32;
    err = GCCommandDispatch(cmd, groups, 1, 1);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferEnd(cmd);
    TEST_ASSERT_OK(err);
    
    // Create fence for synchronization
    GCFencePtr fence = NULL;
    err = GCFenceCreate(ctx.device, 0, &fence);
    TEST_ASSERT_OK(err);
    
    // Submit
    GCSubmitInfo submit = {
        .command_buffers = &cmd,
        .command_buffer_count = 1,
        .wait_semaphores = NULL,
        .wait_values = NULL,
        .wait_count = 0,
        .signal_semaphores = NULL,
        .signal_values = NULL,
        .signal_count = 0,
        .fence = fence,
    };
    err = GCQueueSubmit(ctx.compute_queue, &submit);
    TEST_ASSERT_OK(err);
    
    // Wait for completion
    err = GCFenceWait(fence, 1, 5000000000); // 5 second timeout
    TEST_ASSERT_OK(err);
    
    // Verify output data
    {
        @autoreleasepool {
            struct GCBuffer {
                void *ep_device;
                id<MTLBuffer> mtl_buffer;
                uint64_t size;
                uint32_t usage;
                bool host_visible;
            };
            struct GCBuffer *buf = (struct GCBuffer *)buffer;
            
            float *data = [buf->mtl_buffer contents];
            bool all_correct = true;
            for (uint32_t i = 0; i < element_count; i++) {
                float expected = (float)i * 2.0f;  // Should be doubled
                if (fabsf(data[i] - expected) > 0.0001f) {
                    printf("\n  Mismatch at [%u]: expected %f, got %f", i, expected, data[i]);
                    all_correct = false;
                }
            }
            TEST_ASSERT(all_correct, "compute output data should be doubled");
        }
    }
    
    // Cleanup
    GCFenceDestroy(fence);
    GCCommandBufferDestroy(cmd);
    GCComputePipelineDestroy(pipeline);
    GCDescriptorSetDestroy(desc_set);
    GCPipelineLayoutDestroy(pipe_layout);
    GCDescriptorSetLayoutDestroy(set_layout);
    GCShaderLibraryDestroy(library);
    GCBufferDestroy(buffer);
    free(pipe_layout_desc);
    free(layout_desc);
    
    teardown_test_context(&ctx);
    return 0;
}

static const char *add_constant_shader = 
    "#include <metal_stdlib>\n"
    "using namespace metal;\n"
    "struct PushConstants {\n"
    "    float value;\n"
    "};\n"
    "kernel void add_constant(device float *data [[buffer(0)]],\n"
    "                         constant PushConstants &pc [[buffer(30)]],\n"
    "                         uint id [[thread_position_in_grid]]) {\n"
    "    data[id] = data[id] + pc.value;\n"
    "}\n";

static int test_compute_with_push_constants(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    const uint32_t element_count = 64;
    const uint64_t buffer_size = element_count * sizeof(float);
    const float add_value = 10.0f;
    
    // Create buffer
    GCBufferDesc buf_desc = {
        .size = buffer_size,
        .usage = GC_BUFFER_USAGE_STORAGE_BIT,
        .host_visible = true,
    };
    GCBufferPtr buffer = NULL;
    err = GCBufferCreate(ctx.device, &buf_desc, &buffer);
    TEST_ASSERT_OK(err);
    
    // Initialize buffer
    {
        @autoreleasepool {
            struct GCBuffer {
                void *ep_device;
                id<MTLBuffer> mtl_buffer;
                uint64_t size;
                uint32_t usage;
                bool host_visible;
            };
            struct GCBuffer *buf = (struct GCBuffer *)buffer;
            
            float *data = [buf->mtl_buffer contents];
            for (uint32_t i = 0; i < element_count; i++) {
                data[i] = (float)i;
            }
        }
    }
    
    // Create shader
    GCShaderLibraryDesc lib_desc = {
        .format = GC_SHADER_MSL,
        .data = (const uint8_t *)add_constant_shader,
        .size = strlen(add_constant_shader),
        .label = "add_constant",
    };
    GCShaderLibraryPtr library = NULL;
    err = GCShaderLibraryCreate(ctx.device, &lib_desc, &library);
    TEST_ASSERT_OK(err);
    
    // Create descriptor set layout
    GCDescriptorSetLayoutDesc *layout_desc = malloc(
        sizeof(GCDescriptorSetLayoutDesc) + sizeof(GCDescriptorBindingDesc));
    layout_desc->binding_count = 1;
    layout_desc->bindings[0] = (GCDescriptorBindingDesc){
        .binding = 0,
        .type = GC_DESCRIPTOR_STORAGE_BUFFER,
        .count = 1,
        .stages = GC_STAGE_COMPUTE_BIT,
    };
    GCDescriptorSetLayoutPtr set_layout = NULL;
    err = GCDescriptorSetLayoutCreate(ctx.device, layout_desc, &set_layout);
    TEST_ASSERT_OK(err);
    
    // Create pipeline layout with push constants
    GCPipelineLayoutDesc *pipe_layout_desc = malloc(
        sizeof(GCPipelineLayoutDesc) + sizeof(GCDescriptorSetLayoutPtr));
    pipe_layout_desc->set_layout_count = 1;
    pipe_layout_desc->push_constant_size = sizeof(float);
    pipe_layout_desc->push_constant_stages = GC_STAGE_COMPUTE_BIT;
    pipe_layout_desc->set_layouts[0] = set_layout;
    GCPipelineLayoutPtr pipe_layout = NULL;
    err = GCPipelineLayoutCreate(ctx.device, pipe_layout_desc, &pipe_layout);
    TEST_ASSERT_OK(err);
    
    // Create descriptor set
    GCDescriptorSetPtr desc_set = NULL;
    err = GCDescriptorSetCreate(ctx.device, set_layout, &desc_set);
    TEST_ASSERT_OK(err);
    err = GCDescriptorSetUpdateBuffer(desc_set, 0, buffer, 0, buffer_size);
    TEST_ASSERT_OK(err);
    
    // Create pipeline
    GCComputePipelineDesc pipe_desc = {
        .library = library,
        .entry = "add_constant",
        .layout = pipe_layout,
    };
    GCComputePipelinePtr pipeline = NULL;
    err = GCComputePipelineCreate(ctx.device, &pipe_desc, &pipeline);
    TEST_ASSERT_OK(err);
    
    // Record and submit
    GCCommandBufferPtr cmd = NULL;
    err = GCCommandBufferCreate(ctx.device, &cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferBegin(cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBindComputePipeline(cmd, pipeline);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBindDescriptorSet(cmd, pipe_layout, 0, desc_set);
    TEST_ASSERT_OK(err);
    
    err = GCCommandPushConstants(cmd, pipe_layout, GC_STAGE_COMPUTE_BIT, 
                                 (const uint8_t *)&add_value, sizeof(add_value));
    TEST_ASSERT_OK(err);
    
    err = GCCommandDispatch(cmd, (element_count + 31) / 32, 1, 1);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferEnd(cmd);
    TEST_ASSERT_OK(err);
    
    // Submit with fence
    GCFencePtr fence = NULL;
    err = GCFenceCreate(ctx.device, 0, &fence);
    TEST_ASSERT_OK(err);
    
    GCSubmitInfo submit = {
        .command_buffers = &cmd,
        .command_buffer_count = 1,
        .fence = fence,
    };
    err = GCQueueSubmit(ctx.compute_queue, &submit);
    TEST_ASSERT_OK(err);
    
    err = GCFenceWait(fence, 1, 5000000000);
    TEST_ASSERT_OK(err);
    
    // Verify results
    {
        @autoreleasepool {
            struct GCBuffer {
                void *ep_device;
                id<MTLBuffer> mtl_buffer;
                uint64_t size;
                uint32_t usage;
                bool host_visible;
            };
            struct GCBuffer *buf = (struct GCBuffer *)buffer;
            
            float *data = [buf->mtl_buffer contents];
            bool all_correct = true;
            for (uint32_t i = 0; i < element_count; i++) {
                float expected = (float)i + add_value;
                if (fabsf(data[i] - expected) > 0.0001f) {
                    printf("\n  Mismatch at [%u]: expected %f, got %f", i, expected, data[i]);
                    all_correct = false;
                }
            }
            TEST_ASSERT(all_correct, "compute output should have constant added");
        }
    }
    
    // Cleanup
    GCFenceDestroy(fence);
    GCCommandBufferDestroy(cmd);
    GCComputePipelineDestroy(pipeline);
    GCDescriptorSetDestroy(desc_set);
    GCPipelineLayoutDestroy(pipe_layout);
    GCDescriptorSetLayoutDestroy(set_layout);
    GCShaderLibraryDestroy(library);
    GCBufferDestroy(buffer);
    free(pipe_layout_desc);
    free(layout_desc);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Queue Submission Tests
// =============================================================================

static int test_queue_submit_empty(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    GCCommandBufferPtr cmd = NULL;
    err = GCCommandBufferCreate(ctx.device, &cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferBegin(cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferEnd(cmd);
    TEST_ASSERT_OK(err);
    
    GCSubmitInfo submit = {
        .command_buffers = &cmd,
        .command_buffer_count = 1,
        .fence = NULL,
    };
    
    err = GCQueueSubmit(ctx.graphics_queue, &submit);
    TEST_ASSERT_OK(err);
    
    err = GCQueueWaitIdle(ctx.graphics_queue);
    TEST_ASSERT_OK(err);
    
    GCCommandBufferDestroy(cmd);
    teardown_test_context(&ctx);
    return 0;
}

static int test_queue_wait_idle(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Submit multiple command buffers
    for (int i = 0; i < 5; i++) {
        GCCommandBufferPtr cmd = NULL;
        err = GCCommandBufferCreate(ctx.device, &cmd);
        TEST_ASSERT_OK(err);
        
        err = GCCommandBufferBegin(cmd);
        TEST_ASSERT_OK(err);
        err = GCCommandBufferEnd(cmd);
        TEST_ASSERT_OK(err);
        
        GCSubmitInfo submit = {
            .command_buffers = &cmd,
            .command_buffer_count = 1,
            .fence = NULL,
        };
        
        err = GCQueueSubmit(ctx.graphics_queue, &submit);
        TEST_ASSERT_OK(err);
        
        GCCommandBufferDestroy(cmd);
    }
    
    // Wait for all to complete
    err = GCQueueWaitIdle(ctx.graphics_queue);
    TEST_ASSERT_OK(err);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Render Pass Tests
// =============================================================================

static int test_render_pass_begin_end(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Create color attachment texture
    GCTextureDesc tex_desc = {
        .dimension = GC_TEXTURE_DIM_2D,
        .format = GC_FORMAT_RGBA8_UNORM,
        .width = 256,
        .height = 256,
        .depth = 1,
        .mip_levels = 1,
        .array_layers = 1,
        .usage = GC_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT,
    };
    GCTexturePtr color_tex = NULL;
    err = GCTextureCreate(ctx.device, &tex_desc, &color_tex);
    TEST_ASSERT_OK(err);
    
    // Create depth texture
    GCTextureDesc depth_desc = {
        .dimension = GC_TEXTURE_DIM_2D,
        .format = GC_FORMAT_D32_FLOAT,
        .width = 256,
        .height = 256,
        .depth = 1,
        .mip_levels = 1,
        .array_layers = 1,
        .usage = GC_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT,
    };
    GCTexturePtr depth_tex = NULL;
    err = GCTextureCreate(ctx.device, &depth_desc, &depth_tex);
    TEST_ASSERT_OK(err);
    
    // Create command buffer
    GCCommandBufferPtr cmd = NULL;
    err = GCCommandBufferCreate(ctx.device, &cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferBegin(cmd);
    TEST_ASSERT_OK(err);
    
    // Begin render pass
    GCRenderPassDepthAttachment depth_attach = {
        .texture = depth_tex,
        .layout = GC_TEXTURE_LAYOUT_DEPTH_STENCIL,
        .load_op = GC_LOAD_OP_CLEAR,
        .store_op = GC_STORE_OP_STORE,
        .clear_depth = 1.0f,
        .clear_stencil = 0,
    };
    
    GCRenderPassDesc *pass_desc = malloc(sizeof(GCRenderPassDesc) + sizeof(GCRenderPassColorAttachment));
    pass_desc->color_attachment_count = 1;
    pass_desc->depth_attachment = &depth_attach;
    pass_desc->color_attachments[0] = (GCRenderPassColorAttachment){
        .texture = color_tex,
        .layout = GC_TEXTURE_LAYOUT_COLOR_ATTACHMENT,
        .load_op = GC_LOAD_OP_CLEAR,
        .store_op = GC_STORE_OP_STORE,
        .clear_color = {0.2f, 0.3f, 0.4f, 1.0f},
    };
    
    err = GCCommandBeginRenderPass(cmd, pass_desc);
    TEST_ASSERT_OK(err);
    
    // Set viewport and scissor
    err = GCCommandSetViewport(cmd, 0, 0, 256, 256, 0, 1);
    TEST_ASSERT_OK(err);
    
    err = GCCommandSetScissor(cmd, 0, 0, 256, 256);
    TEST_ASSERT_OK(err);
    
    err = GCCommandEndRenderPass(cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferEnd(cmd);
    TEST_ASSERT_OK(err);
    
    // Submit
    GCSubmitInfo submit = {
        .command_buffers = &cmd,
        .command_buffer_count = 1,
        .fence = NULL,
    };
    err = GCQueueSubmit(ctx.graphics_queue, &submit);
    TEST_ASSERT_OK(err);
    
    err = GCQueueWaitIdle(ctx.graphics_queue);
    TEST_ASSERT_OK(err);
    
    // Cleanup
    free(pass_desc);
    GCCommandBufferDestroy(cmd);
    GCTextureDestroy(depth_tex);
    GCTextureDestroy(color_tex);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Resource Barrier Tests
// =============================================================================

static int test_resource_barrier(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Create buffer
    GCBufferDesc buf_desc = {
        .size = 1024,
        .usage = GC_BUFFER_USAGE_STORAGE_BIT,
        .host_visible = true,
    };
    GCBufferPtr buffer = NULL;
    err = GCBufferCreate(ctx.device, &buf_desc, &buffer);
    TEST_ASSERT_OK(err);
    
    // Create command buffer
    GCCommandBufferPtr cmd = NULL;
    err = GCCommandBufferCreate(ctx.device, &cmd);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferBegin(cmd);
    TEST_ASSERT_OK(err);
    
    // Issue barrier
    GCBufferBarrier buf_barrier = {
        .buffer = buffer,
        .offset = 0,
        .size = 1024,
        .src_access = GC_ACCESS_SHADER_WRITE_BIT,
        .dst_access = GC_ACCESS_SHADER_READ_BIT,
    };
    
    GCBarrierDesc barrier_desc = {
        .src_stage = GC_STAGE_COMPUTE_SHADER_BIT,
        .dst_stage = GC_STAGE_COMPUTE_SHADER_BIT,
        .buffer_barriers = &buf_barrier,
        .buffer_barrier_count = 1,
        .texture_barriers = NULL,
        .texture_barrier_count = 0,
    };
    
    err = GCCommandResourceBarrier(cmd, &barrier_desc);
    TEST_ASSERT_OK(err);
    
    err = GCCommandBufferEnd(cmd);
    TEST_ASSERT_OK(err);
    
    // Submit
    GCSubmitInfo submit = {
        .command_buffers = &cmd,
        .command_buffer_count = 1,
        .fence = NULL,
    };
    err = GCQueueSubmit(ctx.compute_queue, &submit);
    TEST_ASSERT_OK(err);
    
    err = GCQueueWaitIdle(ctx.compute_queue);
    TEST_ASSERT_OK(err);
    
    // Cleanup
    GCCommandBufferDestroy(cmd);
    GCBufferDestroy(buffer);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Error Handling Tests
// =============================================================================

static int test_null_argument_handling(void) {
    GCError err;
    
    // Test NULL instance output
    err = GCInstanceCreate(NULL, NULL);
    TEST_ASSERT_ERR(err, GC_E_INVALID_ARGUMENT);
    
    // Test NULL adapter
    GCAdapterProperties props;
    err = GCAdapterGetProperties(NULL, &props);
    TEST_ASSERT_ERR(err, GC_E_INVALID_ARGUMENT);
    
    return 0;
}

static int test_invalid_shader_entry_point(void) {
    GCError err;
    TestContext ctx = {0};
    
    if (setup_test_context(&ctx) != 0) {
        return 1;
    }
    
    // Create valid shader library
    GCShaderLibraryDesc lib_desc = {
        .format = GC_SHADER_MSL,
        .data = (const uint8_t *)simple_compute_shader,
        .size = strlen(simple_compute_shader),
        .label = "test",
    };
    GCShaderLibraryPtr library = NULL;
    err = GCShaderLibraryCreate(ctx.device, &lib_desc, &library);
    TEST_ASSERT_OK(err);
    
    // Create pipeline layout
    GCPipelineLayoutDesc *layout_desc = malloc(sizeof(GCPipelineLayoutDesc));
    layout_desc->set_layout_count = 0;
    layout_desc->push_constant_size = 0;
    layout_desc->push_constant_stages = 0;
    GCPipelineLayoutPtr layout = NULL;
    err = GCPipelineLayoutCreate(ctx.device, layout_desc, &layout);
    TEST_ASSERT_OK(err);
    
    // Try to create pipeline with non-existent entry point
    GCComputePipelineDesc pipe_desc = {
        .library = library,
        .entry = "nonexistent_function",
        .layout = layout,
    };
    GCComputePipelinePtr pipeline = NULL;
    err = GCComputePipelineCreate(ctx.device, &pipe_desc, &pipeline);
    TEST_ASSERT(err.code != GC_E_OK, "should fail for invalid entry point");
    
    // Cleanup
    GCPipelineLayoutDestroy(layout);
    GCShaderLibraryDestroy(library);
    free(layout_desc);
    
    teardown_test_context(&ctx);
    return 0;
}

// =============================================================================
// Main
// =============================================================================

int main(int argc, const char *argv[]) {
    @autoreleasepool {
        printf("=== Gcraft Metal Backend Integration Tests ===\n\n");
        
        // Instance tests
        printf("--- Instance Tests ---\n");
        RUN_TEST(test_instance_create_destroy);
        RUN_TEST(test_instance_enumerate_adapters);
        RUN_TEST(test_adapter_properties);
        
        // Device tests
        printf("\n--- Device Tests ---\n");
        RUN_TEST(test_device_create_destroy);
        RUN_TEST(test_device_queues);
        
        // Buffer tests
        printf("\n--- Buffer Tests ---\n");
        RUN_TEST(test_buffer_create_host_visible);
        RUN_TEST(test_buffer_create_device_local);
        RUN_TEST(test_buffer_various_sizes);
        
        // Texture tests
        printf("\n--- Texture Tests ---\n");
        RUN_TEST(test_texture_create_2d);
        RUN_TEST(test_texture_create_depth);
        RUN_TEST(test_texture_create_cube);
        RUN_TEST(test_texture_create_3d);
        
        // Sampler tests
        printf("\n--- Sampler Tests ---\n");
        RUN_TEST(test_sampler_create);
        RUN_TEST(test_sampler_anisotropic);
        
        // Shader tests
        printf("\n--- Shader Tests ---\n");
        RUN_TEST(test_shader_library_create);
        RUN_TEST(test_shader_library_invalid);
        
        // Descriptor tests
        printf("\n--- Descriptor Tests ---\n");
        RUN_TEST(test_descriptor_set_layout_create);
        RUN_TEST(test_descriptor_set_create_and_update);
        RUN_TEST(test_pipeline_layout_create);
        
        // Pipeline tests
        printf("\n--- Pipeline Tests ---\n");
        RUN_TEST(test_compute_pipeline_create);
        RUN_TEST(test_render_pipeline_create);
        
        // Command buffer tests
        printf("\n--- Command Buffer Tests ---\n");
        RUN_TEST(test_command_buffer_create);
        RUN_TEST(test_command_buffer_begin_end);
        
        // Synchronization tests
        printf("\n--- Synchronization Tests ---\n");
        RUN_TEST(test_fence_create_destroy);
        RUN_TEST(test_fence_signal_wait);
        RUN_TEST(test_timeline_semaphore_create_destroy);
        RUN_TEST(test_timeline_semaphore_get_value);
        
        // GPU execution tests
        printf("\n--- GPU Execution Tests ---\n");
        RUN_TEST(test_compute_execution_verify_output);
        RUN_TEST(test_compute_with_push_constants);
        
        // Queue tests
        printf("\n--- Queue Tests ---\n");
        RUN_TEST(test_queue_submit_empty);
        RUN_TEST(test_queue_wait_idle);
        
        // Render pass tests
        printf("\n--- Render Pass Tests ---\n");
        RUN_TEST(test_render_pass_begin_end);
        
        // Resource barrier tests
        printf("\n--- Resource Barrier Tests ---\n");
        RUN_TEST(test_resource_barrier);
        
        // Error handling tests
        printf("\n--- Error Handling Tests ---\n");
        RUN_TEST(test_null_argument_handling);
        RUN_TEST(test_invalid_shader_entry_point);
        
        printf("\n=== Test Summary ===\n");
        printf("Total:  %d\n", tests_total);
        printf("Passed: %d\n", tests_passed);
        printf("Failed: %d\n", tests_failed);
        
        return tests_failed > 0 ? 1 : 0;
    }
}
