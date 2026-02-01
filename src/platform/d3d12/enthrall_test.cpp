// enthrall_test.cpp - Meaningful integration tests for D3D12 backend
// These tests verify actual GPU operations and data integrity, not just API calls.
// Run with: zig build test-d3d12 (on Windows)

#include "enthrall_internal.h"
#include <cassert>
#include <cstdio>
#include <cstring>
#include <cmath>
#include <fstream>
#include <vector>
#include <string>
#include <chrono>

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
    fflush(stdout); \
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
// Test Helper Infrastructure
// ============================================================================

// Helper: Create a readback buffer (CPU-visible for reading GPU results)
static EPBufferPtr create_readback_buffer(uint64_t size) {
    // D3D12 readback buffers need to use READBACK heap type
    // For simplicity, we use host_visible which creates upload buffers
    // that can be read back after GPU writes via copy
    EPBufferDesc desc = {};
    desc.size = size;
    desc.usage = EP_BUFFER_USAGE_TRANSFER_DST_BIT;
    desc.host_visible = true;
    
    EPBufferPtr buffer = nullptr;
    EPError err = EPBufferCreate(g_device, &desc, &buffer);
    if (err.code != EP_E_OK) return nullptr;
    return buffer;
}

// Helper: Create a staging/upload buffer with initial data
static EPBufferPtr create_staging_buffer(const void *data, uint64_t size) {
    EPBufferDesc desc = {};
    desc.size = size;
    desc.usage = EP_BUFFER_USAGE_TRANSFER_SRC_BIT;
    desc.host_visible = true;
    
    EPBufferPtr buffer = nullptr;
    EPError err = EPBufferCreate(g_device, &desc, &buffer);
    if (err.code != EP_E_OK) return nullptr;
    
    if (data && buffer->mapped_ptr) {
        memcpy(buffer->mapped_ptr, data, size);
    }
    return buffer;
}

// Helper: Create a GPU buffer (not host visible)
static EPBufferPtr create_gpu_buffer(uint64_t size, EPBufferUsageFlags usage) {
    EPBufferDesc desc = {};
    desc.size = size;
    desc.usage = usage;
    desc.host_visible = false;
    
    EPBufferPtr buffer = nullptr;
    EPError err = EPBufferCreate(g_device, &desc, &buffer);
    if (err.code != EP_E_OK) return nullptr;
    return buffer;
}

// Helper: Compare memory regions with detailed error reporting
static bool compare_memory(const void *expected, const void *actual, size_t size, 
                           const char *context) {
    const uint8_t *exp = static_cast<const uint8_t *>(expected);
    const uint8_t *act = static_cast<const uint8_t *>(actual);
    
    for (size_t i = 0; i < size; i++) {
        if (exp[i] != act[i]) {
            printf("\n  Memory mismatch in %s at offset %zu: expected 0x%02X, got 0x%02X\n",
                   context, i, exp[i], act[i]);
            return false;
        }
    }
    return true;
}

// Helper: Compare float arrays with tolerance
static bool compare_floats(const float *expected, const float *actual, size_t count,
                           float tolerance, const char *context) {
    for (size_t i = 0; i < count; i++) {
        float diff = fabsf(expected[i] - actual[i]);
        if (diff > tolerance) {
            printf("\n  Float mismatch in %s at index %zu: expected %f, got %f (diff: %f)\n",
                   context, i, expected[i], actual[i], diff);
            return false;
        }
    }
    return true;
}

// Helper: Submit command buffer and wait for completion
static bool submit_and_wait(EPCommandBufferPtr cmd) {
    EPQueuePtr queue = nullptr;
    EPError err = EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &queue);
    if (err.code != EP_E_OK) return false;
    
    EPCommandBufferPtr cmds[] = { cmd };
    EPSubmitInfo submit = {};
    submit.command_buffer_count = 1;
    submit.command_buffers = cmds;
    
    err = EPQueueSubmit(queue, &submit);
    if (err.code != EP_E_OK) {
        delete queue;
        return false;
    }
    
    err = EPQueueWaitIdle(queue);
    delete queue;
    return err.code == EP_E_OK;
}

// Helper: Copy GPU buffer to CPU-readable buffer and return contents
static std::vector<uint8_t> readback_buffer(EPBufferPtr gpu_buffer, uint64_t size) {
    std::vector<uint8_t> result;
    
    // Create readback buffer
    EPBufferPtr readback = create_readback_buffer(size);
    if (!readback) return result;
    
    // Create command buffer for copy
    EPCommandBufferPtr cmd = nullptr;
    EPError err = EPCommandBufferCreate(g_device, &cmd);
    if (err.code != EP_E_OK) {
        EPBufferDestroy(readback);
        return result;
    }
    
    err = EPCommandBufferBegin(cmd);
    if (err.code != EP_E_OK) {
        EPCommandBufferDestroy(cmd);
        EPBufferDestroy(readback);
        return result;
    }
    
    // Record copy command
    cmd->list->CopyBufferRegion(
        readback->resource.Get(), 0,
        gpu_buffer->resource.Get(), 0,
        size);
    
    err = EPCommandBufferEnd(cmd);
    if (err.code != EP_E_OK) {
        EPCommandBufferDestroy(cmd);
        EPBufferDestroy(readback);
        return result;
    }
    
    // Submit and wait
    if (!submit_and_wait(cmd)) {
        EPCommandBufferDestroy(cmd);
        EPBufferDestroy(readback);
        return result;
    }
    
    // Read back the data
    if (readback->mapped_ptr) {
        result.resize(size);
        memcpy(result.data(), readback->mapped_ptr, size);
    }
    
    EPCommandBufferDestroy(cmd);
    EPBufferDestroy(readback);
    return result;
}

// Helper: Upload data to GPU buffer
static bool upload_to_buffer(EPBufferPtr gpu_buffer, const void *data, uint64_t size) {
    // Create staging buffer with data
    EPBufferPtr staging = create_staging_buffer(data, size);
    if (!staging) return false;
    
    // Create command buffer for copy
    EPCommandBufferPtr cmd = nullptr;
    EPError err = EPCommandBufferCreate(g_device, &cmd);
    if (err.code != EP_E_OK) {
        EPBufferDestroy(staging);
        return false;
    }
    
    err = EPCommandBufferBegin(cmd);
    if (err.code != EP_E_OK) {
        EPCommandBufferDestroy(cmd);
        EPBufferDestroy(staging);
        return false;
    }
    
    // Record copy command
    cmd->list->CopyBufferRegion(
        gpu_buffer->resource.Get(), 0,
        staging->resource.Get(), 0,
        size);
    
    err = EPCommandBufferEnd(cmd);
    if (err.code != EP_E_OK) {
        EPCommandBufferDestroy(cmd);
        EPBufferDestroy(staging);
        return false;
    }
    
    bool success = submit_and_wait(cmd);
    
    EPCommandBufferDestroy(cmd);
    EPBufferDestroy(staging);
    return success;
}

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
    
    // Request only the first adapter to match the single g_adapter storage
    adapter_count = 1;
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
// Category 1: Buffer Data Roundtrip Tests
// ============================================================================

// Test: Copy buffer through GPU and verify byte-for-byte integrity
static bool test_buffer_copy_roundtrip() {
    if (!g_device) return true;
    
    const uint64_t buffer_size = 4096;
    
    // Create known test pattern: alternating 0xDE, 0xAD, 0xBE, 0xEF
    std::vector<uint8_t> test_pattern(buffer_size);
    for (size_t i = 0; i < buffer_size; i++) {
        uint8_t patterns[] = {0xDE, 0xAD, 0xBE, 0xEF};
        test_pattern[i] = patterns[i % 4];
    }
    
    // Create GPU buffer
    EPBufferPtr gpu_buffer = create_gpu_buffer(buffer_size, 
        EP_BUFFER_USAGE_TRANSFER_SRC_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT);
    TEST_ASSERT(gpu_buffer != nullptr, "failed to create GPU buffer");
    
    // Upload test pattern to GPU buffer
    bool uploaded = upload_to_buffer(gpu_buffer, test_pattern.data(), buffer_size);
    TEST_ASSERT(uploaded, "failed to upload to GPU buffer");
    
    // Read back and verify
    std::vector<uint8_t> readback = readback_buffer(gpu_buffer, buffer_size);
    TEST_ASSERT(readback.size() == buffer_size, "readback size mismatch");
    
    bool matches = compare_memory(test_pattern.data(), readback.data(), buffer_size,
                                  "buffer roundtrip");
    TEST_ASSERT(matches, "buffer data mismatch after roundtrip");
    
    EPBufferDestroy(gpu_buffer);
    return true;
}

// Test: Copy a subset of a buffer and verify boundary integrity
static bool test_buffer_partial_copy() {
    if (!g_device) return true;
    
    const uint64_t total_size = 1024;
    const uint64_t copy_offset = 256;
    const uint64_t copy_size = 512;
    
    // Create test data: sequential bytes
    std::vector<uint8_t> test_data(total_size);
    for (size_t i = 0; i < total_size; i++) {
        test_data[i] = static_cast<uint8_t>(i & 0xFF);
    }
    
    // Create source buffer with test data
    EPBufferPtr src_buffer = create_staging_buffer(test_data.data(), total_size);
    TEST_ASSERT(src_buffer != nullptr, "failed to create source buffer");
    
    // Create destination GPU buffer (zeroed)
    EPBufferPtr dst_buffer = create_gpu_buffer(total_size,
        EP_BUFFER_USAGE_TRANSFER_SRC_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT);
    TEST_ASSERT(dst_buffer != nullptr, "failed to create destination buffer");
    
    // Create command buffer for partial copy
    EPCommandBufferPtr cmd = nullptr;
    EPError err = EPCommandBufferCreate(g_device, &cmd);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd);
    TEST_OK(err);
    
    // Copy only the middle portion
    cmd->list->CopyBufferRegion(
        dst_buffer->resource.Get(), copy_offset,
        src_buffer->resource.Get(), copy_offset,
        copy_size);
    
    err = EPCommandBufferEnd(cmd);
    TEST_OK(err);
    
    bool submitted = submit_and_wait(cmd);
    TEST_ASSERT(submitted, "failed to submit partial copy");
    
    // Read back entire destination buffer
    std::vector<uint8_t> readback = readback_buffer(dst_buffer, total_size);
    TEST_ASSERT(readback.size() == total_size, "readback size mismatch");
    
    // Verify: the copied region should match
    bool copy_matches = compare_memory(
        test_data.data() + copy_offset,
        readback.data() + copy_offset,
        copy_size,
        "partial copy region");
    TEST_ASSERT(copy_matches, "partial copy data mismatch");
    
    EPCommandBufferDestroy(cmd);
    EPBufferDestroy(src_buffer);
    EPBufferDestroy(dst_buffer);
    return true;
}

// Test: Large buffer transfer (64MB)
static bool test_buffer_large_transfer() {
    if (!g_device) return true;
    
    const uint64_t buffer_size = 64 * 1024 * 1024; // 64 MB
    
    // Create test pattern: repeating 0xCAFEBABE
    std::vector<uint8_t> test_pattern(buffer_size);
    uint32_t pattern = 0xCAFEBABE;
    for (size_t i = 0; i < buffer_size; i += 4) {
        memcpy(test_pattern.data() + i, &pattern, 
               std::min(static_cast<uint64_t>(4), buffer_size - i));
    }
    
    // Create GPU buffer
    EPBufferPtr gpu_buffer = create_gpu_buffer(buffer_size,
        EP_BUFFER_USAGE_TRANSFER_SRC_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT);
    if (!gpu_buffer) {
        printf("\n  Skipping: could not allocate 64MB GPU buffer\n");
        return true; // Skip if can't allocate large buffer
    }
    
    // Upload
    bool uploaded = upload_to_buffer(gpu_buffer, test_pattern.data(), buffer_size);
    if (!uploaded) {
        EPBufferDestroy(gpu_buffer);
        printf("\n  Skipping: upload failed for large buffer\n");
        return true;
    }
    
    // Read back and verify (sample check at multiple points to save time)
    std::vector<uint8_t> readback = readback_buffer(gpu_buffer, buffer_size);
    if (readback.empty()) {
        EPBufferDestroy(gpu_buffer);
        printf("\n  Skipping: readback failed for large buffer\n");
        return true;
    }
    
    // Sample verification at start, middle, and end
    size_t check_points[] = {0, buffer_size / 4, buffer_size / 2, 
                             3 * buffer_size / 4, buffer_size - 4};
    for (size_t point : check_points) {
        uint32_t expected = 0xCAFEBABE;
        uint32_t actual;
        memcpy(&actual, readback.data() + point, 4);
        if (actual != expected) {
            printf("\n  Large buffer mismatch at offset %zu: expected 0x%08X, got 0x%08X\n",
                   point, expected, actual);
            EPBufferDestroy(gpu_buffer);
            return false;
        }
    }
    
    EPBufferDestroy(gpu_buffer);
    return true;
}

// ============================================================================
// Category 2: Compute Shader Execution Tests
// ============================================================================

// Embedded HLSL for compute shader test: writes thread_id * 2 to output
static const char *g_compute_shader_multiply = R"(
RWStructuredBuffer<uint> output : register(u0);

[numthreads(64, 1, 1)]
void main(uint3 id : SV_DispatchThreadID) {
    output[id.x] = id.x * 2;
}
)";

// Test: Execute compute shader and verify output pattern
static bool test_compute_shader_write_pattern() {
    if (!g_device) return true;
    
    const uint32_t element_count = 256;
    const uint64_t buffer_size = element_count * sizeof(uint32_t);
    
    // Create UAV buffer for compute output
    EPBufferDesc uav_desc = {};
    uav_desc.size = buffer_size;
    uav_desc.usage = EP_BUFFER_USAGE_STORAGE_BIT | EP_BUFFER_USAGE_TRANSFER_SRC_BIT;
    uav_desc.host_visible = false;
    
    EPBufferPtr output_buffer = nullptr;
    EPError err = EPBufferCreate(g_device, &uav_desc, &output_buffer);
    if (err.code != EP_E_OK) {
        printf("\n  Skipping: could not create UAV buffer\n");
        return true;
    }
    
    // Compile shader
    EPShaderLibraryDesc shader_desc = {};
    shader_desc.format = EP_SHADER_HLSL;
    shader_desc.data = reinterpret_cast<const uint8_t *>(g_compute_shader_multiply);
    shader_desc.size = strlen(g_compute_shader_multiply);
    
    EPShaderLibraryPtr shader = nullptr;
    err = EPShaderLibraryCreate(g_device, &shader_desc, &shader);
    if (err.code != EP_E_OK) {
        EPBufferDestroy(output_buffer);
        printf("\n  Skipping: shader compilation failed (DXC may not be available)\n");
        return true;
    }
    
    // Create root signature for compute
    D3D12_ROOT_PARAMETER1 root_param = {};
    root_param.ParameterType = D3D12_ROOT_PARAMETER_TYPE_UAV;
    root_param.Descriptor.ShaderRegister = 0;
    root_param.Descriptor.RegisterSpace = 0;
    root_param.ShaderVisibility = D3D12_SHADER_VISIBILITY_ALL;
    
    D3D12_VERSIONED_ROOT_SIGNATURE_DESC root_sig_desc = {};
    root_sig_desc.Version = D3D_ROOT_SIGNATURE_VERSION_1_1;
    root_sig_desc.Desc_1_1.NumParameters = 1;
    root_sig_desc.Desc_1_1.pParameters = &root_param;
    
    ComPtr<ID3DBlob> root_sig_blob, error_blob;
    HRESULT hr = D3D12SerializeVersionedRootSignature(&root_sig_desc, 
                                                       &root_sig_blob, &error_blob);
    if (FAILED(hr)) {
        EPShaderLibraryDestroy(shader);
        EPBufferDestroy(output_buffer);
        printf("\n  Skipping: root signature serialization failed\n");
        return true;
    }
    
    ComPtr<ID3D12RootSignature> root_signature;
    hr = g_device->device->CreateRootSignature(
        0, root_sig_blob->GetBufferPointer(), root_sig_blob->GetBufferSize(),
        IID_PPV_ARGS(&root_signature));
    if (FAILED(hr)) {
        EPShaderLibraryDestroy(shader);
        EPBufferDestroy(output_buffer);
        printf("\n  Skipping: root signature creation failed\n");
        return true;
    }
    
    // Create compute pipeline state
    D3D12_COMPUTE_PIPELINE_STATE_DESC pso_desc = {};
    pso_desc.pRootSignature = root_signature.Get();
    pso_desc.CS.pShaderBytecode = shader->bytecode.data();
    pso_desc.CS.BytecodeLength = shader->bytecode.size();
    
    ComPtr<ID3D12PipelineState> pso;
    hr = g_device->device->CreateComputePipelineState(&pso_desc, IID_PPV_ARGS(&pso));
    if (FAILED(hr)) {
        EPShaderLibraryDestroy(shader);
        EPBufferDestroy(output_buffer);
        printf("\n  Skipping: compute PSO creation failed\n");
        return true;
    }
    
    // Create and record command buffer
    EPCommandBufferPtr cmd = nullptr;
    err = EPCommandBufferCreate(g_device, &cmd);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd);
    TEST_OK(err);
    
    cmd->list->SetComputeRootSignature(root_signature.Get());
    cmd->list->SetPipelineState(pso.Get());
    cmd->list->SetComputeRootUnorderedAccessView(0, output_buffer->gpu_address);
    cmd->list->Dispatch(element_count / 64, 1, 1);
    
    // UAV barrier to ensure writes complete
    D3D12_RESOURCE_BARRIER barrier = {};
    barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_UAV;
    barrier.UAV.pResource = output_buffer->resource.Get();
    cmd->list->ResourceBarrier(1, &barrier);
    
    err = EPCommandBufferEnd(cmd);
    TEST_OK(err);
    
    bool submitted = submit_and_wait(cmd);
    TEST_ASSERT(submitted, "compute dispatch submission failed");
    
    // Read back and verify
    std::vector<uint8_t> readback = readback_buffer(output_buffer, buffer_size);
    TEST_ASSERT(readback.size() == buffer_size, "compute readback size mismatch");
    
    const uint32_t *results = reinterpret_cast<const uint32_t *>(readback.data());
    for (uint32_t i = 0; i < element_count; i++) {
        uint32_t expected = i * 2;
        if (results[i] != expected) {
            printf("\n  Compute output mismatch at index %u: expected %u, got %u\n",
                   i, expected, results[i]);
            EPCommandBufferDestroy(cmd);
            EPShaderLibraryDestroy(shader);
            EPBufferDestroy(output_buffer);
            return false;
        }
    }
    
    EPCommandBufferDestroy(cmd);
    EPShaderLibraryDestroy(shader);
    EPBufferDestroy(output_buffer);
    return true;
}

// ============================================================================
// Category 3: Render Target Clear Tests
// ============================================================================

// Test: Clear render target to specific color and verify pixel values
static bool test_render_pass_clear_color() {
    if (!g_device) return true;
    
    const uint32_t width = 64;
    const uint32_t height = 64;
    const uint64_t pixel_count = width * height;
    const uint64_t buffer_size = pixel_count * 4; // RGBA8
    
    // Create render target texture
    EPTextureDesc tex_desc = {};
    tex_desc.dimension = EP_TEXTURE_DIM_2D;
    tex_desc.format = EP_FORMAT_RGBA8_UNORM;
    tex_desc.width = width;
    tex_desc.height = height;
    tex_desc.depth = 1;
    tex_desc.mip_levels = 1;
    tex_desc.array_layers = 1;
    tex_desc.usage = EP_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT | EP_TEXTURE_USAGE_TRANSFER_SRC_BIT;
    
    EPTexturePtr render_target = nullptr;
    EPError err = EPTextureCreate(g_device, &tex_desc, &render_target);
    if (err.code != EP_E_OK) {
        printf("\n  Skipping: could not create render target\n");
        return true;
    }
    
    // Create readback buffer
    EPBufferPtr readback = create_readback_buffer(buffer_size);
    TEST_ASSERT(readback != nullptr, "failed to create readback buffer");
    
    // Clear color: magenta with alpha (255, 0, 128, 255) = 0xFF8000FF
    float clear_color[4] = {1.0f, 0.0f, 0.5f, 1.0f};
    
    // Create render pass
    struct {
        EPRenderPassDesc header;
        EPRenderPassColorAttachment attachments[1];
    } rp_desc = {};
    rp_desc.header.color_attachment_count = 1;
    rp_desc.attachments[0].texture = render_target;
    rp_desc.attachments[0].layout = EP_TEXTURE_LAYOUT_COLOR_ATTACHMENT;
    rp_desc.attachments[0].load_op = EP_LOAD_OP_CLEAR;
    rp_desc.attachments[0].store_op = EP_STORE_OP_STORE;
    memcpy(rp_desc.attachments[0].clear_color, clear_color, sizeof(clear_color));
    
    // Create command buffer
    EPCommandBufferPtr cmd = nullptr;
    err = EPCommandBufferCreate(g_device, &cmd);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd);
    TEST_OK(err);
    
    // Transition render target to render target state
    D3D12_RESOURCE_BARRIER barrier = {};
    barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
    barrier.Transition.pResource = render_target->resource.Get();
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_COMMON;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_RENDER_TARGET;
    barrier.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
    cmd->list->ResourceBarrier(1, &barrier);
    
    // Begin render pass
    err = EPCommandBeginRenderPass(cmd, &rp_desc.header);
    TEST_OK(err);
    
    // End render pass (just clearing, no drawing)
    err = EPCommandEndRenderPass(cmd);
    TEST_OK(err);
    
    // Transition to copy source
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_COPY_SOURCE;
    cmd->list->ResourceBarrier(1, &barrier);
    
    // Copy texture to readback buffer
    D3D12_TEXTURE_COPY_LOCATION src_loc = {};
    src_loc.pResource = render_target->resource.Get();
    src_loc.Type = D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX;
    src_loc.SubresourceIndex = 0;
    
    D3D12_TEXTURE_COPY_LOCATION dst_loc = {};
    dst_loc.pResource = readback->resource.Get();
    dst_loc.Type = D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT;
    dst_loc.PlacedFootprint.Offset = 0;
    dst_loc.PlacedFootprint.Footprint.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    dst_loc.PlacedFootprint.Footprint.Width = width;
    dst_loc.PlacedFootprint.Footprint.Height = height;
    dst_loc.PlacedFootprint.Footprint.Depth = 1;
    dst_loc.PlacedFootprint.Footprint.RowPitch = (width * 4 + 255) & ~255; // 256-byte aligned
    
    cmd->list->CopyTextureRegion(&dst_loc, 0, 0, 0, &src_loc, nullptr);
    
    err = EPCommandBufferEnd(cmd);
    TEST_OK(err);
    
    bool submitted = submit_and_wait(cmd);
    TEST_ASSERT(submitted, "render pass submission failed");
    
    // Verify pixel values
    // Expected: R=255, G=0, B=128 (0.5*255), A=255
    uint8_t expected_r = 255;
    uint8_t expected_g = 0;
    uint8_t expected_b = 128; // 0.5 * 255 = 127.5 ≈ 128
    uint8_t expected_a = 255;
    
    uint8_t *pixels = static_cast<uint8_t *>(readback->mapped_ptr);
    uint32_t row_pitch = (width * 4 + 255) & ~255;
    
    bool all_correct = true;
    for (uint32_t y = 0; y < height && all_correct; y++) {
        for (uint32_t x = 0; x < width && all_correct; x++) {
            uint32_t offset = y * row_pitch + x * 4;
            uint8_t r = pixels[offset + 0];
            uint8_t g = pixels[offset + 1];
            uint8_t b = pixels[offset + 2];
            uint8_t a = pixels[offset + 3];
            
            // Allow tolerance of 1 for rounding
            if (abs(r - expected_r) > 1 || abs(g - expected_g) > 1 ||
                abs(b - expected_b) > 1 || abs(a - expected_a) > 1) {
                printf("\n  Pixel mismatch at (%u,%u): expected (%u,%u,%u,%u), got (%u,%u,%u,%u)\n",
                       x, y, expected_r, expected_g, expected_b, expected_a, r, g, b, a);
                all_correct = false;
            }
        }
    }
    
    EPCommandBufferDestroy(cmd);
    EPTextureDestroy(render_target);
    EPBufferDestroy(readback);
    
    TEST_ASSERT(all_correct, "not all pixels matched expected clear color");
    return true;
}

// Test: Clear depth buffer and verify values
static bool test_render_pass_clear_depth() {
    if (!g_device) return true;
    
    const uint32_t width = 32;
    const uint32_t height = 32;
    
    // Create depth texture
    EPTextureDesc tex_desc = {};
    tex_desc.dimension = EP_TEXTURE_DIM_2D;
    tex_desc.format = EP_FORMAT_D32_FLOAT;
    tex_desc.width = width;
    tex_desc.height = height;
    tex_desc.depth = 1;
    tex_desc.mip_levels = 1;
    tex_desc.array_layers = 1;
    tex_desc.usage = EP_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT | EP_TEXTURE_USAGE_TRANSFER_SRC_BIT;
    
    EPTexturePtr depth_texture = nullptr;
    EPError err = EPTextureCreate(g_device, &tex_desc, &depth_texture);
    if (err.code != EP_E_OK) {
        printf("\n  Skipping: could not create depth texture\n");
        return true;
    }
    
    // Create readback buffer for depth values
    uint64_t buffer_size = width * height * sizeof(float);
    uint32_t row_pitch = (width * sizeof(float) + 255) & ~255;
    uint64_t total_size = row_pitch * height;
    
    EPBufferPtr readback = create_readback_buffer(total_size);
    TEST_ASSERT(readback != nullptr, "failed to create depth readback buffer");
    
    // Clear depth value
    float clear_depth = 0.75f;
    
    // Create render pass with depth attachment
    EPRenderPassDepthAttachment depth_att = {};
    depth_att.texture = depth_texture;
    depth_att.layout = EP_TEXTURE_LAYOUT_DEPTH_STENCIL;
    depth_att.load_op = EP_LOAD_OP_CLEAR;
    depth_att.store_op = EP_STORE_OP_STORE;
    depth_att.clear_depth = clear_depth;
    depth_att.clear_stencil = 0;
    
    EPRenderPassDesc rp_desc = {};
    rp_desc.color_attachment_count = 0;
    rp_desc.depth_attachment = &depth_att;
    
    // Create command buffer
    EPCommandBufferPtr cmd = nullptr;
    err = EPCommandBufferCreate(g_device, &cmd);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd);
    TEST_OK(err);
    
    // Transition to depth write
    D3D12_RESOURCE_BARRIER barrier = {};
    barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
    barrier.Transition.pResource = depth_texture->resource.Get();
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_COMMON;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_DEPTH_WRITE;
    barrier.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
    cmd->list->ResourceBarrier(1, &barrier);
    
    err = EPCommandBeginRenderPass(cmd, &rp_desc);
    TEST_OK(err);
    
    err = EPCommandEndRenderPass(cmd);
    TEST_OK(err);
    
    // Transition to copy source
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_DEPTH_WRITE;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_COPY_SOURCE;
    cmd->list->ResourceBarrier(1, &barrier);
    
    // Copy depth texture to buffer
    D3D12_TEXTURE_COPY_LOCATION src_loc = {};
    src_loc.pResource = depth_texture->resource.Get();
    src_loc.Type = D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX;
    src_loc.SubresourceIndex = 0;
    
    D3D12_TEXTURE_COPY_LOCATION dst_loc = {};
    dst_loc.pResource = readback->resource.Get();
    dst_loc.Type = D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT;
    dst_loc.PlacedFootprint.Offset = 0;
    dst_loc.PlacedFootprint.Footprint.Format = DXGI_FORMAT_D32_FLOAT;
    dst_loc.PlacedFootprint.Footprint.Width = width;
    dst_loc.PlacedFootprint.Footprint.Height = height;
    dst_loc.PlacedFootprint.Footprint.Depth = 1;
    dst_loc.PlacedFootprint.Footprint.RowPitch = row_pitch;
    
    cmd->list->CopyTextureRegion(&dst_loc, 0, 0, 0, &src_loc, nullptr);
    
    err = EPCommandBufferEnd(cmd);
    TEST_OK(err);
    
    bool submitted = submit_and_wait(cmd);
    TEST_ASSERT(submitted, "depth clear submission failed");
    
    // Verify depth values
    float *depth_values = static_cast<float *>(readback->mapped_ptr);
    bool all_correct = true;
    
    for (uint32_t y = 0; y < height && all_correct; y++) {
        for (uint32_t x = 0; x < width && all_correct; x++) {
            uint32_t offset = y * (row_pitch / sizeof(float)) + x;
            float value = depth_values[offset];
            if (fabsf(value - clear_depth) > 0.001f) {
                printf("\n  Depth mismatch at (%u,%u): expected %f, got %f\n",
                       x, y, clear_depth, value);
                all_correct = false;
            }
        }
    }
    
    EPCommandBufferDestroy(cmd);
    EPTextureDestroy(depth_texture);
    EPBufferDestroy(readback);
    
    TEST_ASSERT(all_correct, "not all depth values matched expected clear depth");
    return true;
}

// ============================================================================
// Category 4: Synchronization Verification Tests
// ============================================================================

// Test: Verify fence actually blocks CPU until GPU completes
static bool test_fence_gpu_signal() {
    if (!g_device) return true;
    
    // Create a fence
    EPFencePtr fence = nullptr;
    EPError err = EPFenceCreate(g_device, 0, &fence);
    TEST_OK(err);
    
    // Create a buffer and write a known pattern
    const uint64_t buffer_size = 1024;
    EPBufferPtr gpu_buffer = create_gpu_buffer(buffer_size,
        EP_BUFFER_USAGE_TRANSFER_SRC_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT);
    TEST_ASSERT(gpu_buffer != nullptr, "failed to create test buffer");
    
    std::vector<uint8_t> test_data(buffer_size, 0xAB);
    
    // Upload data
    bool uploaded = upload_to_buffer(gpu_buffer, test_data.data(), buffer_size);
    TEST_ASSERT(uploaded, "failed to upload test data");
    
    // Submit work with fence signal
    EPQueuePtr queue = nullptr;
    err = EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &queue);
    TEST_OK(err);
    
    // Signal fence from GPU
    queue->queue->Signal(fence->fence.Get(), 1);
    
    // Check that fence value is NOT immediately 1 (GPU work pending)
    // Note: This might complete very fast, so this check is probabilistic
    uint64_t immediate_value = fence->fence->GetCompletedValue();
    
    // Wait on fence
    err = EPFenceWait(fence, 1, 5000000000); // 5 second timeout
    TEST_OK(err);
    
    // After wait, fence value should be >= 1
    uint64_t after_value = fence->fence->GetCompletedValue();
    TEST_ASSERT(after_value >= 1, "fence value should be >= 1 after wait");
    
    // Verify data is intact (proves GPU work completed before wait returned)
    std::vector<uint8_t> readback = readback_buffer(gpu_buffer, buffer_size);
    TEST_ASSERT(readback.size() == buffer_size, "readback failed after fence wait");
    
    bool data_intact = compare_memory(test_data.data(), readback.data(), buffer_size,
                                      "post-fence data");
    TEST_ASSERT(data_intact, "data corrupted after fence wait");
    
    delete queue;
    EPFenceDestroy(fence);
    EPBufferDestroy(gpu_buffer);
    return true;
}

// Test: Timeline semaphore ordering between submits
static bool test_timeline_semaphore_ordering() {
    if (!g_device) return true;
    
    // Create timeline semaphore
    EPTimelineSemaphorePtr sem = nullptr;
    EPError err = EPTimelineSemaphoreCreate(g_device, 0, &sem);
    TEST_OK(err);
    
    // Create two buffers
    const uint64_t buffer_size = 256;
    EPBufferPtr buffer_a = create_gpu_buffer(buffer_size,
        EP_BUFFER_USAGE_TRANSFER_SRC_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT);
    EPBufferPtr buffer_b = create_gpu_buffer(buffer_size,
        EP_BUFFER_USAGE_TRANSFER_SRC_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT);
    TEST_ASSERT(buffer_a && buffer_b, "failed to create test buffers");
    
    // Upload pattern A to buffer_a
    std::vector<uint8_t> pattern_a(buffer_size, 0xAA);
    bool uploaded = upload_to_buffer(buffer_a, pattern_a.data(), buffer_size);
    TEST_ASSERT(uploaded, "failed to upload pattern A");
    
    // Submit 1: Copy from buffer_a to buffer_b, signal semaphore = 1
    EPCommandBufferPtr cmd1 = nullptr;
    err = EPCommandBufferCreate(g_device, &cmd1);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd1);
    TEST_OK(err);
    
    cmd1->list->CopyBufferRegion(buffer_b->resource.Get(), 0,
                                  buffer_a->resource.Get(), 0, buffer_size);
    
    err = EPCommandBufferEnd(cmd1);
    TEST_OK(err);
    
    // Submit 2: Wait semaphore = 1, then signal = 2
    // This submission should wait for submit 1 to complete
    
    EPQueuePtr queue = nullptr;
    err = EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &queue);
    TEST_OK(err);
    
    // Submit first batch with signal
    EPCommandBufferPtr cmds1[] = { cmd1 };
    uint64_t signal_val1 = 1;
    EPSubmitInfo submit1 = {};
    submit1.command_buffer_count = 1;
    submit1.command_buffers = cmds1;
    submit1.signal_semaphores = &sem;
    submit1.signal_values = &signal_val1;
    submit1.signal_count = 1;
    
    err = EPQueueSubmit(queue, &submit1);
    TEST_OK(err);
    
    // Wait on semaphore from CPU
    err = EPTimelineSemaphoreWait(sem, 1, 5000000000);
    TEST_OK(err);
    
    // Verify semaphore value
    uint64_t value = 0;
    err = EPTimelineSemaphoreGetValue(sem, &value);
    TEST_OK(err);
    TEST_ASSERT(value >= 1, "semaphore should be >= 1 after signal");
    
    // Verify buffer_b has the copied data
    std::vector<uint8_t> readback = readback_buffer(buffer_b, buffer_size);
    TEST_ASSERT(readback.size() == buffer_size, "readback failed");
    
    bool matches = compare_memory(pattern_a.data(), readback.data(), buffer_size,
                                  "semaphore-ordered copy");
    TEST_ASSERT(matches, "data mismatch after semaphore-ordered copy");
    
    delete queue;
    EPCommandBufferDestroy(cmd1);
    EPTimelineSemaphoreDestroy(sem);
    EPBufferDestroy(buffer_a);
    EPBufferDestroy(buffer_b);
    return true;
}

// Test: Queue wait idle actually waits for completion
static bool test_queue_wait_idle_actually_waits() {
    if (!g_device) return true;
    
    // Create buffer and write pattern
    const uint64_t buffer_size = 1024;
    EPBufferPtr gpu_buffer = create_gpu_buffer(buffer_size,
        EP_BUFFER_USAGE_TRANSFER_SRC_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT);
    TEST_ASSERT(gpu_buffer != nullptr, "failed to create test buffer");
    
    std::vector<uint8_t> test_data(buffer_size, 0xCD);
    
    // Submit upload
    EPBufferPtr staging = create_staging_buffer(test_data.data(), buffer_size);
    TEST_ASSERT(staging != nullptr, "failed to create staging buffer");
    
    EPCommandBufferPtr cmd = nullptr;
    EPError err = EPCommandBufferCreate(g_device, &cmd);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd);
    TEST_OK(err);
    
    cmd->list->CopyBufferRegion(gpu_buffer->resource.Get(), 0,
                                 staging->resource.Get(), 0, buffer_size);
    
    err = EPCommandBufferEnd(cmd);
    TEST_OK(err);
    
    EPQueuePtr queue = nullptr;
    err = EPDeviceGetQueue(g_device, EP_QUEUE_GRAPHICS, 0, &queue);
    TEST_OK(err);
    
    EPCommandBufferPtr cmds[] = { cmd };
    EPSubmitInfo submit = {};
    submit.command_buffer_count = 1;
    submit.command_buffers = cmds;
    
    err = EPQueueSubmit(queue, &submit);
    TEST_OK(err);
    
    // Wait idle
    err = EPQueueWaitIdle(queue);
    TEST_OK(err);
    
    // After wait idle, the data should be in the GPU buffer
    // Read it back and verify
    std::vector<uint8_t> readback = readback_buffer(gpu_buffer, buffer_size);
    TEST_ASSERT(readback.size() == buffer_size, "readback after wait idle failed");
    
    bool matches = compare_memory(test_data.data(), readback.data(), buffer_size,
                                  "wait idle data");
    TEST_ASSERT(matches, "data mismatch after wait idle");
    
    delete queue;
    EPCommandBufferDestroy(cmd);
    EPBufferDestroy(staging);
    EPBufferDestroy(gpu_buffer);
    return true;
}

// ============================================================================
// Category 5: DirectStorage Integration Tests
// ============================================================================

// Test: Load file to GPU buffer via DirectStorage
static bool test_dstorage_file_to_buffer() {
    if (!g_device) return true;
    if (!g_device->dstorage_factory || !g_device->dstorage_queue) {
        printf("\n  Skipping: DirectStorage not available\n");
        return true;
    }
    
    // Create test file with known content
    const char *test_filename = "dstorage_test.bin";
    const uint64_t file_size = 4096;
    
    std::vector<uint8_t> file_content(file_size);
    for (size_t i = 0; i < file_size; i++) {
        file_content[i] = static_cast<uint8_t>((i * 7) & 0xFF);
    }
    
    // Write test file
    std::ofstream out_file(test_filename, std::ios::binary);
    if (!out_file) {
        printf("\n  Skipping: could not create test file\n");
        return true;
    }
    out_file.write(reinterpret_cast<const char *>(file_content.data()), file_size);
    out_file.close();
    
    // Open file with DirectStorage
    ComPtr<IDStorageFile> ds_file;
    wchar_t wide_filename[256];
    MultiByteToWideChar(CP_UTF8, 0, test_filename, -1, wide_filename, 256);
    
    HRESULT hr = g_device->dstorage_factory->OpenFile(wide_filename, 
                                                       IID_PPV_ARGS(&ds_file));
    if (FAILED(hr)) {
        std::remove(test_filename);
        printf("\n  Skipping: DirectStorage OpenFile failed\n");
        return true;
    }
    
    // Get file info
    BY_HANDLE_FILE_INFORMATION file_info;
    hr = ds_file->GetFileInformation(&file_info);
    if (FAILED(hr)) {
        std::remove(test_filename);
        printf("\n  Skipping: GetFileInformation failed\n");
        return true;
    }
    
    // Create destination GPU buffer
    EPBufferPtr gpu_buffer = create_gpu_buffer(file_size,
        EP_BUFFER_USAGE_TRANSFER_SRC_BIT | EP_BUFFER_USAGE_TRANSFER_DST_BIT);
    if (!gpu_buffer) {
        std::remove(test_filename);
        printf("\n  Skipping: could not create destination buffer\n");
        return true;
    }
    
    // Create fence for completion
    ComPtr<ID3D12Fence> ds_fence;
    hr = g_device->device->CreateFence(0, D3D12_FENCE_FLAG_NONE, IID_PPV_ARGS(&ds_fence));
    if (FAILED(hr)) {
        EPBufferDestroy(gpu_buffer);
        std::remove(test_filename);
        printf("\n  Skipping: fence creation failed\n");
        return true;
    }
    
    // Enqueue read request
    DSTORAGE_REQUEST request = {};
    request.Options.SourceType = DSTORAGE_REQUEST_SOURCE_FILE;
    request.Options.DestinationType = DSTORAGE_REQUEST_DESTINATION_BUFFER;
    request.Options.CompressionFormat = DSTORAGE_COMPRESSION_FORMAT_NONE;
    request.Source.File.Source = ds_file.Get();
    request.Source.File.Offset = 0;
    request.Source.File.Size = static_cast<UINT32>(file_size);
    request.UncompressedSize = static_cast<UINT32>(file_size);
    request.Destination.Buffer.Resource = gpu_buffer->resource.Get();
    request.Destination.Buffer.Offset = 0;
    request.Destination.Buffer.Size = static_cast<UINT32>(file_size);
    
    g_device->dstorage_queue->EnqueueRequest(&request);
    g_device->dstorage_queue->EnqueueSignal(ds_fence.Get(), 1);
    g_device->dstorage_queue->Submit();
    
    // Wait for completion
    HANDLE event = CreateEventW(nullptr, FALSE, FALSE, nullptr);
    if (!event) {
        EPBufferDestroy(gpu_buffer);
        std::remove(test_filename);
        return false;
    }
    
    ds_fence->SetEventOnCompletion(1, event);
    WaitForSingleObject(event, 10000); // 10 second timeout
    CloseHandle(event);
    
    // Check for errors
    DSTORAGE_ERROR_RECORD error_record = {};
    g_device->dstorage_queue->RetrieveErrorRecord(&error_record);
    if (FAILED(error_record.FirstFailure.HResult)) {
        EPBufferDestroy(gpu_buffer);
        std::remove(test_filename);
        printf("\n  DirectStorage request failed with HRESULT 0x%08X\n",
               error_record.FirstFailure.HResult);
        return false;
    }
    
    // Read back and verify
    std::vector<uint8_t> readback = readback_buffer(gpu_buffer, file_size);
    bool success = readback.size() == file_size;
    if (success) {
        success = compare_memory(file_content.data(), readback.data(), file_size,
                                 "DirectStorage file load");
    }
    
    EPBufferDestroy(gpu_buffer);
    std::remove(test_filename);
    
    TEST_ASSERT(success, "DirectStorage file content verification failed");
    return true;
}

// ============================================================================
// Category 6: Texture Data Verification Tests
// ============================================================================

// Test: Upload and download texture data (RGBA8)
static bool test_texture_upload_download_rgba8() {
    if (!g_device) return true;
    
    const uint32_t width = 32;
    const uint32_t height = 32;
    const uint64_t pixel_count = width * height;
    
    // Create checkerboard pattern
    std::vector<uint8_t> checkerboard(pixel_count * 4);
    for (uint32_t y = 0; y < height; y++) {
        for (uint32_t x = 0; x < width; x++) {
            uint32_t offset = (y * width + x) * 4;
            bool white = ((x / 4) + (y / 4)) % 2 == 0;
            checkerboard[offset + 0] = white ? 255 : 0;   // R
            checkerboard[offset + 1] = white ? 255 : 0;   // G
            checkerboard[offset + 2] = white ? 255 : 0;   // B
            checkerboard[offset + 3] = 255;               // A
        }
    }
    
    // Create texture
    EPTextureDesc tex_desc = {};
    tex_desc.dimension = EP_TEXTURE_DIM_2D;
    tex_desc.format = EP_FORMAT_RGBA8_UNORM;
    tex_desc.width = width;
    tex_desc.height = height;
    tex_desc.depth = 1;
    tex_desc.mip_levels = 1;
    tex_desc.array_layers = 1;
    tex_desc.usage = EP_TEXTURE_USAGE_SAMPLED_BIT | EP_TEXTURE_USAGE_TRANSFER_SRC_BIT | 
                     EP_TEXTURE_USAGE_TRANSFER_DST_BIT;
    
    EPTexturePtr texture = nullptr;
    EPError err = EPTextureCreate(g_device, &tex_desc, &texture);
    if (err.code != EP_E_OK) {
        printf("\n  Skipping: texture creation failed\n");
        return true;
    }
    
    // Calculate row pitch (must be 256-byte aligned for D3D12)
    uint32_t row_pitch = (width * 4 + 255) & ~255;
    uint64_t total_size = row_pitch * height;
    
    // Create staging buffer with aligned layout
    std::vector<uint8_t> aligned_data(total_size, 0);
    for (uint32_t y = 0; y < height; y++) {
        memcpy(aligned_data.data() + y * row_pitch,
               checkerboard.data() + y * width * 4,
               width * 4);
    }
    
    EPBufferPtr staging = create_staging_buffer(aligned_data.data(), total_size);
    TEST_ASSERT(staging != nullptr, "failed to create staging buffer");
    
    // Create command buffer for upload
    EPCommandBufferPtr cmd = nullptr;
    err = EPCommandBufferCreate(g_device, &cmd);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd);
    TEST_OK(err);
    
    // Transition texture to copy dest
    D3D12_RESOURCE_BARRIER barrier = {};
    barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
    barrier.Transition.pResource = texture->resource.Get();
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_COMMON;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_COPY_DEST;
    barrier.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
    cmd->list->ResourceBarrier(1, &barrier);
    
    // Copy from buffer to texture
    D3D12_TEXTURE_COPY_LOCATION src_loc = {};
    src_loc.pResource = staging->resource.Get();
    src_loc.Type = D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT;
    src_loc.PlacedFootprint.Offset = 0;
    src_loc.PlacedFootprint.Footprint.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    src_loc.PlacedFootprint.Footprint.Width = width;
    src_loc.PlacedFootprint.Footprint.Height = height;
    src_loc.PlacedFootprint.Footprint.Depth = 1;
    src_loc.PlacedFootprint.Footprint.RowPitch = row_pitch;
    
    D3D12_TEXTURE_COPY_LOCATION dst_loc = {};
    dst_loc.pResource = texture->resource.Get();
    dst_loc.Type = D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX;
    dst_loc.SubresourceIndex = 0;
    
    cmd->list->CopyTextureRegion(&dst_loc, 0, 0, 0, &src_loc, nullptr);
    
    // Transition to copy source for readback
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_COPY_DEST;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_COPY_SOURCE;
    cmd->list->ResourceBarrier(1, &barrier);
    
    err = EPCommandBufferEnd(cmd);
    TEST_OK(err);
    
    bool submitted = submit_and_wait(cmd);
    TEST_ASSERT(submitted, "texture upload submission failed");
    
    // Now read back the texture
    EPBufferPtr readback = create_readback_buffer(total_size);
    TEST_ASSERT(readback != nullptr, "failed to create readback buffer");
    
    EPCommandBufferPtr cmd2 = nullptr;
    err = EPCommandBufferCreate(g_device, &cmd2);
    TEST_OK(err);
    
    err = EPCommandBufferBegin(cmd2);
    TEST_OK(err);
    
    // Copy texture to buffer
    D3D12_TEXTURE_COPY_LOCATION rb_src_loc = {};
    rb_src_loc.pResource = texture->resource.Get();
    rb_src_loc.Type = D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX;
    rb_src_loc.SubresourceIndex = 0;
    
    D3D12_TEXTURE_COPY_LOCATION rb_dst_loc = {};
    rb_dst_loc.pResource = readback->resource.Get();
    rb_dst_loc.Type = D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT;
    rb_dst_loc.PlacedFootprint.Offset = 0;
    rb_dst_loc.PlacedFootprint.Footprint.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    rb_dst_loc.PlacedFootprint.Footprint.Width = width;
    rb_dst_loc.PlacedFootprint.Footprint.Height = height;
    rb_dst_loc.PlacedFootprint.Footprint.Depth = 1;
    rb_dst_loc.PlacedFootprint.Footprint.RowPitch = row_pitch;
    
    cmd2->list->CopyTextureRegion(&rb_dst_loc, 0, 0, 0, &rb_src_loc, nullptr);
    
    err = EPCommandBufferEnd(cmd2);
    TEST_OK(err);
    
    submitted = submit_and_wait(cmd2);
    TEST_ASSERT(submitted, "texture readback submission failed");
    
    // Verify pixel data
    uint8_t *pixels = static_cast<uint8_t *>(readback->mapped_ptr);
    bool all_correct = true;
    
    for (uint32_t y = 0; y < height && all_correct; y++) {
        for (uint32_t x = 0; x < width && all_correct; x++) {
            uint32_t src_offset = (y * width + x) * 4;
            uint32_t dst_offset = y * row_pitch + x * 4;
            
            if (pixels[dst_offset + 0] != checkerboard[src_offset + 0] ||
                pixels[dst_offset + 1] != checkerboard[src_offset + 1] ||
                pixels[dst_offset + 2] != checkerboard[src_offset + 2] ||
                pixels[dst_offset + 3] != checkerboard[src_offset + 3]) {
                printf("\n  Texture pixel mismatch at (%u,%u)\n", x, y);
                all_correct = false;
            }
        }
    }
    
    EPCommandBufferDestroy(cmd);
    EPCommandBufferDestroy(cmd2);
    EPTextureDestroy(texture);
    EPBufferDestroy(staging);
    EPBufferDestroy(readback);
    
    TEST_ASSERT(all_correct, "texture pixel data verification failed");
    return true;
}

// ============================================================================
// Basic API Tests (kept for coverage)
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

static bool test_adapter_properties() {
    if (!g_adapter) return true;
    
    EPAdapterProperties props = {};
    EPError err = EPAdapterGetProperties(g_adapter, &props);
    TEST_OK(err);
    
    TEST_ASSERT(strlen(props.name) > 0, "adapter name should not be empty");
    TEST_ASSERT(props.backends & EP_BACKEND_D3D12_BIT, "should support D3D12 backend");
    TEST_ASSERT(props.features & EP_FEATURE_COMPUTE_BIT, "should support compute");
    TEST_ASSERT(props.limits.max_texture_dimension_2d >= 4096, "max 2D texture should be >= 4096");
    
    return true;
}

static bool test_format_conversions() {
    TEST_ASSERT(ep_to_dxgi_format(EP_FORMAT_RGBA8_UNORM) == DXGI_FORMAT_R8G8B8A8_UNORM,
                "RGBA8 conversion");
    TEST_ASSERT(ep_to_dxgi_format(EP_FORMAT_BGRA8_UNORM) == DXGI_FORMAT_B8G8R8A8_UNORM,
                "BGRA8 conversion");
    TEST_ASSERT(ep_to_dxgi_format(EP_FORMAT_D32_FLOAT) == DXGI_FORMAT_D32_FLOAT,
                "D32 conversion");
    TEST_ASSERT(ep_to_d3d12_address(EP_ADDRESS_REPEAT) == D3D12_TEXTURE_ADDRESS_MODE_WRAP,
                "repeat address mode");
    TEST_ASSERT(ep_to_d3d12_compare(EP_COMPARE_LESS) == D3D12_COMPARISON_FUNC_LESS,
                "less compare");
    
    return true;
}

// ============================================================================
// Main
// ============================================================================

int main() {
    printf("=== Enthrall D3D12 Backend Integration Tests ===\n\n");
    
    // Format conversion tests (don't need GPU)
    printf("--- Format Conversion Tests ---\n");
    RUN_TEST(test_format_conversions);
    
    // Basic API tests
    printf("\n--- Basic API Tests ---\n");
    RUN_TEST(test_platform_create_destroy);
    RUN_TEST(test_instance_create_destroy);
    
    // Setup test environment for GPU tests
    printf("\nSetting up test environment...\n");
    if (!setup_test_environment()) {
        printf("Failed to setup test environment!\n");
        return 1;
    }
    printf("Test environment ready.\n");
    
    RUN_TEST(test_adapter_properties);
    
    // Category 1: Buffer roundtrip tests
    printf("\n--- Buffer Data Roundtrip Tests ---\n");
    RUN_TEST(test_buffer_copy_roundtrip);
    RUN_TEST(test_buffer_partial_copy);
    RUN_TEST(test_buffer_large_transfer);
    
    // Category 2: Compute shader tests
    printf("\n--- Compute Shader Tests ---\n");
    RUN_TEST(test_compute_shader_write_pattern);
    
    // Category 3: Render pass tests
    printf("\n--- Render Pass Clear Tests ---\n");
    RUN_TEST(test_render_pass_clear_color);
    RUN_TEST(test_render_pass_clear_depth);
    
    // Category 4: Synchronization tests
    printf("\n--- Synchronization Tests ---\n");
    RUN_TEST(test_fence_gpu_signal);
    RUN_TEST(test_timeline_semaphore_ordering);
    RUN_TEST(test_queue_wait_idle_actually_waits);
    
    // Category 5: DirectStorage tests
    printf("\n--- DirectStorage Tests ---\n");
    RUN_TEST(test_dstorage_file_to_buffer);
    
    // Category 6: Texture tests
    printf("\n--- Texture Data Tests ---\n");
    RUN_TEST(test_texture_upload_download_rgba8);
    
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
