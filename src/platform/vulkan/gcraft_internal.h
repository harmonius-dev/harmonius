// gcraft_internal.h - Internal structures and utilities for Vulkan backend
// Uses modern C++ with smart pointers, vulkan-hpp, volk, and VMA

#ifndef GCRAFT_VULKAN_INTERNAL_H
#define GCRAFT_VULKAN_INTERNAL_H

// Configure volk before vulkan headers (implementation in gcraft_instance.cpp)
#include <volk.h>

// Configure vulkan-hpp for no-exceptions mode and dynamic dispatch
#define VULKAN_HPP_DISPATCH_LOADER_DYNAMIC 1
#define VULKAN_HPP_NO_CONSTRUCTORS
#define VULKAN_HPP_NO_STRUCT_SETTERS
#define VULKAN_HPP_NO_EXCEPTIONS
#define VULKAN_HPP_ASSERT_ON_RESULT(...)
#include <vulkan/vulkan.hpp>

// Configure VMA
#define VMA_STATIC_VULKAN_FUNCTIONS 0
#define VMA_DYNAMIC_VULKAN_FUNCTIONS 1
#include <vk_mem_alloc.h>

#include <memory>
#include <vector>
#include <string>
#include <optional>
#include <array>
#include <cstring>

#include "gcraft_types.h"
#include "gcraft_command.h"
#include "gcraft_descriptor.h"
#include "gcraft_device.h"
#include "gcraft_instance.h"
#include "gcraft_pipeline.h"
#include "gcraft_resource.h"
#include "gcraft_surface.h"
#include "gcraft_sync.h"

namespace ep::vk {

// ============================================================================
// Error Helpers
// ============================================================================

inline GCError ok() {
    return GCError{.code = GC_E_OK, .message = nullptr};
}

inline GCError error(GCErrorCode code, const char* msg) {
    return GCError{.code = code, .message = msg};
}

inline GCError invalid_argument(const char* msg) {
    return error(GC_E_INVALID_ARGUMENT, msg);
}

inline GCError out_of_memory() {
    return error(GC_E_OUT_OF_MEMORY, "out of memory");
}

inline GCError unsupported(const char* msg) {
    return error(GC_E_UNSUPPORTED, msg);
}

inline GCError from_vk_result(::vk::Result result) {
    switch (result) {
        case ::vk::Result::eSuccess:
            return ok();
        case ::vk::Result::eErrorOutOfHostMemory:
        case ::vk::Result::eErrorOutOfDeviceMemory:
            return out_of_memory();
        case ::vk::Result::eErrorDeviceLost:
            return error(GC_E_DEVICE_LOST, "vulkan device lost");
        case ::vk::Result::eErrorFeatureNotPresent:
            return unsupported("vulkan feature not present");
        case ::vk::Result::eErrorExtensionNotPresent:
            return unsupported("vulkan extension not present");
        case ::vk::Result::eErrorLayerNotPresent:
            return unsupported("vulkan layer not present");
        case ::vk::Result::eErrorIncompatibleDriver:
            return error(GC_E_INVALID_STATE, "vulkan incompatible driver");
        case ::vk::Result::eErrorInitializationFailed:
            return error(GC_E_INVALID_STATE, "vulkan initialization failed");
        default:
            return error(GC_E_INVALID_STATE, "vulkan error");
    }
}

// ============================================================================
// Format Conversion
// ============================================================================

inline ::vk::Format to_vk_format(GCTextureFormat format) {
    switch (format) {
        case GC_FORMAT_RGBA8_UNORM:  return ::vk::Format::eR8G8B8A8Unorm;
        case GC_FORMAT_BGRA8_UNORM:  return ::vk::Format::eB8G8R8A8Unorm;
        case GC_FORMAT_RGBA16_FLOAT: return ::vk::Format::eR16G16B16A16Sfloat;
        case GC_FORMAT_RGBA32_FLOAT: return ::vk::Format::eR32G32B32A32Sfloat;
        case GC_FORMAT_D24S8:        return ::vk::Format::eD24UnormS8Uint;
        case GC_FORMAT_D32_FLOAT:    return ::vk::Format::eD32Sfloat;
        default:                     return ::vk::Format::eUndefined;
    }
}

inline ::vk::Filter to_vk_filter(GCFilter filter) {
    return filter == GC_FILTER_LINEAR ? ::vk::Filter::eLinear : ::vk::Filter::eNearest;
}

inline ::vk::SamplerAddressMode to_vk_address_mode(GCAddressMode mode) {
    switch (mode) {
        case GC_ADDRESS_CLAMP_TO_EDGE:  return ::vk::SamplerAddressMode::eClampToEdge;
        case GC_ADDRESS_REPEAT:         return ::vk::SamplerAddressMode::eRepeat;
        case GC_ADDRESS_MIRROR_REPEAT:  return ::vk::SamplerAddressMode::eMirroredRepeat;
        default:                        return ::vk::SamplerAddressMode::eClampToEdge;
    }
}

inline ::vk::CompareOp to_vk_compare_op(GCCompareOp op) {
    switch (op) {
        case GC_COMPARE_NEVER:         return ::vk::CompareOp::eNever;
        case GC_COMPARE_LESS:          return ::vk::CompareOp::eLess;
        case GC_COMPARE_EQUAL:         return ::vk::CompareOp::eEqual;
        case GC_COMPARE_LESS_EQUAL:    return ::vk::CompareOp::eLessOrEqual;
        case GC_COMPARE_GREATER:       return ::vk::CompareOp::eGreater;
        case GC_COMPARE_NOT_EQUAL:     return ::vk::CompareOp::eNotEqual;
        case GC_COMPARE_GREATER_EQUAL: return ::vk::CompareOp::eGreaterOrEqual;
        case GC_COMPARE_ALWAYS:        return ::vk::CompareOp::eAlways;
        default:                       return ::vk::CompareOp::eAlways;
    }
}

inline ::vk::AttachmentLoadOp to_vk_load_op(GCAttachmentLoadOp op) {
    switch (op) {
        case GC_LOAD_OP_LOAD:      return ::vk::AttachmentLoadOp::eLoad;
        case GC_LOAD_OP_CLEAR:     return ::vk::AttachmentLoadOp::eClear;
        case GC_LOAD_OP_DONT_CARE: return ::vk::AttachmentLoadOp::eDontCare;
        default:                   return ::vk::AttachmentLoadOp::eDontCare;
    }
}

inline ::vk::AttachmentStoreOp to_vk_store_op(GCAttachmentStoreOp op) {
    switch (op) {
        case GC_STORE_OP_STORE:     return ::vk::AttachmentStoreOp::eStore;
        case GC_STORE_OP_DONT_CARE: return ::vk::AttachmentStoreOp::eDontCare;
        default:                    return ::vk::AttachmentStoreOp::eStore;
    }
}

inline ::vk::ImageType to_vk_image_type(GCTextureDimension dim) {
    switch (dim) {
        case GC_TEXTURE_DIM_1D:   return ::vk::ImageType::e1D;
        case GC_TEXTURE_DIM_2D:   return ::vk::ImageType::e2D;
        case GC_TEXTURE_DIM_3D:   return ::vk::ImageType::e3D;
        case GC_TEXTURE_DIM_CUBE: return ::vk::ImageType::e2D;
        default:                  return ::vk::ImageType::e2D;
    }
}

inline ::vk::ImageViewType to_vk_image_view_type(GCTextureDimension dim, uint32_t array_layers) {
    switch (dim) {
        case GC_TEXTURE_DIM_1D:
            return array_layers > 1 ? ::vk::ImageViewType::e1DArray : ::vk::ImageViewType::e1D;
        case GC_TEXTURE_DIM_2D:
            return array_layers > 1 ? ::vk::ImageViewType::e2DArray : ::vk::ImageViewType::e2D;
        case GC_TEXTURE_DIM_3D:
            return ::vk::ImageViewType::e3D;
        case GC_TEXTURE_DIM_CUBE:
            return array_layers > 6 ? ::vk::ImageViewType::eCubeArray : ::vk::ImageViewType::eCube;
        default:
            return ::vk::ImageViewType::e2D;
    }
}

inline ::vk::ImageUsageFlags to_vk_image_usage(GCTextureUsageFlags flags) {
    ::vk::ImageUsageFlags usage;
    if (flags & GC_TEXTURE_USAGE_SAMPLED_BIT)          usage |= ::vk::ImageUsageFlagBits::eSampled;
    if (flags & GC_TEXTURE_USAGE_STORAGE_BIT)          usage |= ::vk::ImageUsageFlagBits::eStorage;
    if (flags & GC_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT) usage |= ::vk::ImageUsageFlagBits::eColorAttachment;
    if (flags & GC_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT) usage |= ::vk::ImageUsageFlagBits::eDepthStencilAttachment;
    if (flags & GC_TEXTURE_USAGE_TRANSFER_SRC_BIT)     usage |= ::vk::ImageUsageFlagBits::eTransferSrc;
    if (flags & GC_TEXTURE_USAGE_TRANSFER_DST_BIT)     usage |= ::vk::ImageUsageFlagBits::eTransferDst;
    return usage;
}

inline ::vk::BufferUsageFlags to_vk_buffer_usage(GCBufferUsageFlags flags) {
    ::vk::BufferUsageFlags usage;
    if (flags & GC_BUFFER_USAGE_TRANSFER_SRC_BIT) usage |= ::vk::BufferUsageFlagBits::eTransferSrc;
    if (flags & GC_BUFFER_USAGE_TRANSFER_DST_BIT) usage |= ::vk::BufferUsageFlagBits::eTransferDst;
    if (flags & GC_BUFFER_USAGE_VERTEX_BIT)       usage |= ::vk::BufferUsageFlagBits::eVertexBuffer;
    if (flags & GC_BUFFER_USAGE_INDEX_BIT)        usage |= ::vk::BufferUsageFlagBits::eIndexBuffer;
    if (flags & GC_BUFFER_USAGE_UNIFORM_BIT)      usage |= ::vk::BufferUsageFlagBits::eUniformBuffer;
    if (flags & GC_BUFFER_USAGE_STORAGE_BIT)      usage |= ::vk::BufferUsageFlagBits::eStorageBuffer;
    if (flags & GC_BUFFER_USAGE_INDIRECT_BIT)     usage |= ::vk::BufferUsageFlagBits::eIndirectBuffer;
    if (flags & GC_BUFFER_USAGE_ACCEL_BIT) {
        usage |= ::vk::BufferUsageFlagBits::eAccelerationStructureBuildInputReadOnlyKHR;
        usage |= ::vk::BufferUsageFlagBits::eShaderDeviceAddress;
    }
    return usage;
}

inline ::vk::ImageLayout to_vk_image_layout(GCTextureLayout layout) {
    switch (layout) {
        case GC_TEXTURE_LAYOUT_UNDEFINED:        return ::vk::ImageLayout::eUndefined;
        case GC_TEXTURE_LAYOUT_GENERAL:          return ::vk::ImageLayout::eGeneral;
        case GC_TEXTURE_LAYOUT_COLOR_ATTACHMENT: return ::vk::ImageLayout::eColorAttachmentOptimal;
        case GC_TEXTURE_LAYOUT_DEPTH_STENCIL:    return ::vk::ImageLayout::eDepthStencilAttachmentOptimal;
        case GC_TEXTURE_LAYOUT_SHADER_READ:      return ::vk::ImageLayout::eShaderReadOnlyOptimal;
        case GC_TEXTURE_LAYOUT_TRANSFER_SRC:     return ::vk::ImageLayout::eTransferSrcOptimal;
        case GC_TEXTURE_LAYOUT_TRANSFER_DST:     return ::vk::ImageLayout::eTransferDstOptimal;
        case GC_TEXTURE_LAYOUT_PRESENT:          return ::vk::ImageLayout::ePresentSrcKHR;
        default:                                 return ::vk::ImageLayout::eUndefined;
    }
}

inline ::vk::PipelineStageFlags to_vk_pipeline_stage(GCPipelineStageFlags flags) {
    ::vk::PipelineStageFlags vk_flags;
    if (flags & GC_STAGE_TOP_OF_PIPE_BIT)        vk_flags |= ::vk::PipelineStageFlagBits::eTopOfPipe;
    if (flags & GC_STAGE_DRAW_INDIRECT_BIT)      vk_flags |= ::vk::PipelineStageFlagBits::eDrawIndirect;
    if (flags & GC_STAGE_VERTEX_INPUT_BIT)       vk_flags |= ::vk::PipelineStageFlagBits::eVertexInput;
    if (flags & GC_STAGE_VERTEX_SHADER_BIT)      vk_flags |= ::vk::PipelineStageFlagBits::eVertexShader;
    if (flags & GC_STAGE_FRAGMENT_SHADER_BIT)    vk_flags |= ::vk::PipelineStageFlagBits::eFragmentShader;
    if (flags & GC_STAGE_COMPUTE_SHADER_BIT)     vk_flags |= ::vk::PipelineStageFlagBits::eComputeShader;
    if (flags & GC_STAGE_MESH_SHADER_BIT)        vk_flags |= ::vk::PipelineStageFlagBits::eMeshShaderEXT;
    if (flags & GC_STAGE_RAY_TRACING_SHADER_BIT) vk_flags |= ::vk::PipelineStageFlagBits::eRayTracingShaderKHR;
    if (flags & GC_STAGE_TRANSFER_BIT)           vk_flags |= ::vk::PipelineStageFlagBits::eTransfer;
    if (flags & GC_STAGE_BOTTOM_OF_PIPE_BIT)     vk_flags |= ::vk::PipelineStageFlagBits::eBottomOfPipe;
    return vk_flags;
}

// Vulkan 1.3 synchronization2 stage flags
inline ::vk::PipelineStageFlags2 to_vk_pipeline_stage2(GCPipelineStageFlags flags) {
    ::vk::PipelineStageFlags2 vk_flags;
    if (flags & GC_STAGE_TOP_OF_PIPE_BIT)        vk_flags |= ::vk::PipelineStageFlagBits2::eTopOfPipe;
    if (flags & GC_STAGE_DRAW_INDIRECT_BIT)      vk_flags |= ::vk::PipelineStageFlagBits2::eDrawIndirect;
    if (flags & GC_STAGE_VERTEX_INPUT_BIT)       vk_flags |= ::vk::PipelineStageFlagBits2::eVertexInput;
    if (flags & GC_STAGE_VERTEX_SHADER_BIT)      vk_flags |= ::vk::PipelineStageFlagBits2::eVertexShader;
    if (flags & GC_STAGE_FRAGMENT_SHADER_BIT)    vk_flags |= ::vk::PipelineStageFlagBits2::eFragmentShader;
    if (flags & GC_STAGE_COMPUTE_SHADER_BIT)     vk_flags |= ::vk::PipelineStageFlagBits2::eComputeShader;
    if (flags & GC_STAGE_MESH_SHADER_BIT)        vk_flags |= ::vk::PipelineStageFlagBits2::eMeshShaderEXT;
    if (flags & GC_STAGE_RAY_TRACING_SHADER_BIT) vk_flags |= ::vk::PipelineStageFlagBits2::eRayTracingShaderKHR;
    if (flags & GC_STAGE_TRANSFER_BIT)           vk_flags |= ::vk::PipelineStageFlagBits2::eTransfer;
    if (flags & GC_STAGE_BOTTOM_OF_PIPE_BIT)     vk_flags |= ::vk::PipelineStageFlagBits2::eBottomOfPipe;
    return vk_flags;
}

inline ::vk::AccessFlags to_vk_access_flags(GCAccessFlags flags) {
    ::vk::AccessFlags vk_flags;
    if (flags & GC_ACCESS_INDIRECT_COMMAND_READ_BIT)  vk_flags |= ::vk::AccessFlagBits::eIndirectCommandRead;
    if (flags & GC_ACCESS_VERTEX_READ_BIT)            vk_flags |= ::vk::AccessFlagBits::eVertexAttributeRead;
    if (flags & GC_ACCESS_INDEX_READ_BIT)             vk_flags |= ::vk::AccessFlagBits::eIndexRead;
    if (flags & GC_ACCESS_UNIFORM_READ_BIT)           vk_flags |= ::vk::AccessFlagBits::eUniformRead;
    if (flags & GC_ACCESS_SHADER_READ_BIT)            vk_flags |= ::vk::AccessFlagBits::eShaderRead;
    if (flags & GC_ACCESS_SHADER_WRITE_BIT)           vk_flags |= ::vk::AccessFlagBits::eShaderWrite;
    if (flags & GC_ACCESS_COLOR_ATTACHMENT_READ_BIT)  vk_flags |= ::vk::AccessFlagBits::eColorAttachmentRead;
    if (flags & GC_ACCESS_COLOR_ATTACHMENT_WRITE_BIT) vk_flags |= ::vk::AccessFlagBits::eColorAttachmentWrite;
    if (flags & GC_ACCESS_DEPTH_STENCIL_READ_BIT)     vk_flags |= ::vk::AccessFlagBits::eDepthStencilAttachmentRead;
    if (flags & GC_ACCESS_DEPTH_STENCIL_WRITE_BIT)    vk_flags |= ::vk::AccessFlagBits::eDepthStencilAttachmentWrite;
    if (flags & GC_ACCESS_TRANSFER_READ_BIT)          vk_flags |= ::vk::AccessFlagBits::eTransferRead;
    if (flags & GC_ACCESS_TRANSFER_WRITE_BIT)         vk_flags |= ::vk::AccessFlagBits::eTransferWrite;
    if (flags & GC_ACCESS_HOST_READ_BIT)              vk_flags |= ::vk::AccessFlagBits::eHostRead;
    if (flags & GC_ACCESS_HOST_WRITE_BIT)             vk_flags |= ::vk::AccessFlagBits::eHostWrite;
    return vk_flags;
}

// Vulkan 1.3 synchronization2 access flags
inline ::vk::AccessFlags2 to_vk_access_flags2(GCAccessFlags flags) {
    ::vk::AccessFlags2 vk_flags;
    if (flags & GC_ACCESS_INDIRECT_COMMAND_READ_BIT)  vk_flags |= ::vk::AccessFlagBits2::eIndirectCommandRead;
    if (flags & GC_ACCESS_VERTEX_READ_BIT)            vk_flags |= ::vk::AccessFlagBits2::eVertexAttributeRead;
    if (flags & GC_ACCESS_INDEX_READ_BIT)             vk_flags |= ::vk::AccessFlagBits2::eIndexRead;
    if (flags & GC_ACCESS_UNIFORM_READ_BIT)           vk_flags |= ::vk::AccessFlagBits2::eUniformRead;
    if (flags & GC_ACCESS_SHADER_READ_BIT)            vk_flags |= ::vk::AccessFlagBits2::eShaderRead;
    if (flags & GC_ACCESS_SHADER_WRITE_BIT)           vk_flags |= ::vk::AccessFlagBits2::eShaderWrite;
    if (flags & GC_ACCESS_COLOR_ATTACHMENT_READ_BIT)  vk_flags |= ::vk::AccessFlagBits2::eColorAttachmentRead;
    if (flags & GC_ACCESS_COLOR_ATTACHMENT_WRITE_BIT) vk_flags |= ::vk::AccessFlagBits2::eColorAttachmentWrite;
    if (flags & GC_ACCESS_DEPTH_STENCIL_READ_BIT)     vk_flags |= ::vk::AccessFlagBits2::eDepthStencilAttachmentRead;
    if (flags & GC_ACCESS_DEPTH_STENCIL_WRITE_BIT)    vk_flags |= ::vk::AccessFlagBits2::eDepthStencilAttachmentWrite;
    if (flags & GC_ACCESS_TRANSFER_READ_BIT)          vk_flags |= ::vk::AccessFlagBits2::eTransferRead;
    if (flags & GC_ACCESS_TRANSFER_WRITE_BIT)         vk_flags |= ::vk::AccessFlagBits2::eTransferWrite;
    if (flags & GC_ACCESS_HOST_READ_BIT)              vk_flags |= ::vk::AccessFlagBits2::eHostRead;
    if (flags & GC_ACCESS_HOST_WRITE_BIT)             vk_flags |= ::vk::AccessFlagBits2::eHostWrite;
    return vk_flags;
}

inline ::vk::ShaderStageFlags to_vk_shader_stage(GCShaderStageFlags flags) {
    ::vk::ShaderStageFlags vk_flags;
    if (flags & GC_STAGE_VERTEX_BIT)   vk_flags |= ::vk::ShaderStageFlagBits::eVertex;
    if (flags & GC_STAGE_FRAGMENT_BIT) vk_flags |= ::vk::ShaderStageFlagBits::eFragment;
    if (flags & GC_STAGE_COMPUTE_BIT)  vk_flags |= ::vk::ShaderStageFlagBits::eCompute;
    if (flags & GC_STAGE_MESH_BIT)     vk_flags |= ::vk::ShaderStageFlagBits::eMeshEXT;
    if (flags & GC_STAGE_TASK_BIT)     vk_flags |= ::vk::ShaderStageFlagBits::eTaskEXT;
    if (flags & GC_STAGE_RAYGEN_BIT)   vk_flags |= ::vk::ShaderStageFlagBits::eRaygenKHR;
    if (flags & GC_STAGE_MISS_BIT)     vk_flags |= ::vk::ShaderStageFlagBits::eMissKHR;
    if (flags & GC_STAGE_HIT_BIT)      vk_flags |= ::vk::ShaderStageFlagBits::eClosestHitKHR;
    return vk_flags;
}

inline ::vk::DescriptorType to_vk_descriptor_type(GCDescriptorType type) {
    switch (type) {
        case GC_DESCRIPTOR_UNIFORM_BUFFER:         return ::vk::DescriptorType::eUniformBuffer;
        case GC_DESCRIPTOR_STORAGE_BUFFER:         return ::vk::DescriptorType::eStorageBuffer;
        case GC_DESCRIPTOR_SAMPLED_TEXTURE:        return ::vk::DescriptorType::eSampledImage;
        case GC_DESCRIPTOR_STORAGE_TEXTURE:        return ::vk::DescriptorType::eStorageImage;
        case GC_DESCRIPTOR_SAMPLER:                return ::vk::DescriptorType::eSampler;
        case GC_DESCRIPTOR_ACCELERATION_STRUCTURE: return ::vk::DescriptorType::eAccelerationStructureKHR;
        default:                                   return ::vk::DescriptorType::eUniformBuffer;
    }
}

// ============================================================================
// Forward Declarations
// ============================================================================

struct Platform;
struct Instance;
struct Adapter;
struct Device;
struct Queue;
struct CommandBuffer;
struct Fence;
struct TimelineSemaphore;
struct Buffer;
struct Texture;
struct Sampler;
struct ShaderLibrary;
struct DescriptorSetLayout;
struct DescriptorSet;
struct PipelineLayout;
struct RenderPipeline;
struct ComputePipeline;
struct MeshPipeline;
struct RayTracingPipeline;
struct AccelerationStructure;
struct Surface;
struct Swapchain;

// ============================================================================
// VMA Deleter for unique_ptr
// ============================================================================

struct VmaAllocatorDeleter {
    void operator()(VmaAllocator allocator) const {
        if (allocator) vmaDestroyAllocator(allocator);
    }
};

using UniqueVmaAllocator = std::unique_ptr<std::remove_pointer_t<VmaAllocator>, VmaAllocatorDeleter>;

// ============================================================================
// Internal Structures
// ============================================================================

struct Platform {
    // volk is initialized globally, no per-platform state needed
};

// Instance uses enable_shared_from_this so Adapters can hold owning references
// The 'prevent_destroy' member keeps the object alive until explicitly destroyed
struct Instance : public std::enable_shared_from_this<Instance> {
    std::shared_ptr<Instance> prevent_destroy;  // Self-reference for C API lifetime
    ::vk::UniqueInstance instance;
    std::vector<::vk::PhysicalDevice> physical_devices;
    GCBackendFlags enabled_backends = static_cast<GCBackendFlags>(0);
    bool validation_enabled = false;
    bool debug_names_enabled = false;
};

// Adapter uses enable_shared_from_this so Devices can hold owning references
struct Adapter : public std::enable_shared_from_this<Adapter> {
    std::shared_ptr<Adapter> prevent_destroy;  // Self-reference for C API lifetime
    std::shared_ptr<Instance> instance;  // Keeps Instance alive
    ::vk::PhysicalDevice physical_device;
    ::vk::PhysicalDeviceProperties properties;
    ::vk::PhysicalDeviceFeatures features;
    ::vk::PhysicalDeviceMemoryProperties memory_properties;
    GCAdapterProperties ep_properties{};
    uint32_t graphics_queue_family = UINT32_MAX;
    uint32_t compute_queue_family = UINT32_MAX;
    uint32_t transfer_queue_family = UINT32_MAX;
    bool has_mesh_shader = false;
    bool has_ray_tracing = false;
    bool has_timeline_semaphore = false;
    bool has_dynamic_rendering = false;
};

// Device uses enable_shared_from_this so child objects can hold owning references
struct Device : public std::enable_shared_from_this<Device> {
    std::shared_ptr<Device> prevent_destroy;  // Self-reference for C API lifetime
    std::shared_ptr<Adapter> adapter;  // Keeps Adapter alive
    ::vk::UniqueDevice device;
    UniqueVmaAllocator allocator;
    ::vk::Queue graphics_queue;
    ::vk::Queue compute_queue;
    ::vk::Queue transfer_queue;
    ::vk::UniqueCommandPool graphics_pool;
    ::vk::UniqueCommandPool compute_pool;
    ::vk::UniqueCommandPool transfer_pool;
    ::vk::UniqueDescriptorPool descriptor_pool;
    GCFeatureFlags features = static_cast<GCFeatureFlags>(0);
    bool validation_enabled = false;
    bool debug_names_enabled = false;
};

struct Queue {
    std::shared_ptr<Device> device;
    ::vk::Queue queue;
    ::vk::CommandPool pool;
    GCQueueType type;
    uint32_t family_index = 0;
};

struct CommandBuffer {
    std::shared_ptr<Device> device;
    ::vk::UniqueCommandBuffer buffer;
    bool is_recording = false;
    ::vk::PipelineBindPoint current_bind_point = ::vk::PipelineBindPoint::eGraphics;
    ::vk::PipelineLayout current_layout;
};

struct Fence {
    std::shared_ptr<Device> device;
    ::vk::UniqueSemaphore timeline_semaphore;
    uint64_t value = 0;
};

struct TimelineSemaphore {
    std::shared_ptr<Device> device;
    ::vk::UniqueSemaphore semaphore;
};

struct Buffer {
    std::shared_ptr<Device> device;
    ::vk::Buffer buffer;  // Owned by VMA
    VmaAllocation allocation = nullptr;
    uint64_t size = 0;
    GCBufferUsageFlags usage = static_cast<GCBufferUsageFlags>(0);
    bool host_visible = false;
};

struct Texture {
    std::shared_ptr<Device> device;
    ::vk::Image image;  // Owned by VMA
    VmaAllocation allocation = nullptr;
    ::vk::UniqueImageView view;
    GCTextureDesc desc{};
    ::vk::ImageLayout current_layout = ::vk::ImageLayout::eUndefined;
};

struct Sampler {
    std::shared_ptr<Device> device;
    ::vk::UniqueSampler sampler;
};

struct ShaderLibrary {
    std::shared_ptr<Device> device;
    ::vk::UniqueShaderModule module;
    std::vector<uint8_t> spirv_data;
};

struct DescriptorBindingInfo {
    uint32_t binding = 0;
    GCDescriptorType type = GC_DESCRIPTOR_UNIFORM_BUFFER;
    uint32_t count = 0;
    GCShaderStageFlags stages = static_cast<GCShaderStageFlags>(0);
};

// DescriptorSetLayout uses enable_shared_from_this so DescriptorSets can hold owning references
struct DescriptorSetLayout : public std::enable_shared_from_this<DescriptorSetLayout> {
    std::shared_ptr<DescriptorSetLayout> prevent_destroy;  // Self-reference for C API lifetime
    std::shared_ptr<Device> device;
    ::vk::UniqueDescriptorSetLayout layout;
    std::vector<DescriptorBindingInfo> bindings;
};

struct DescriptorSet {
    std::shared_ptr<Device> device;
    std::shared_ptr<DescriptorSetLayout> layout;
    ::vk::DescriptorSet set;  // Owned by descriptor pool
};

struct PipelineLayout {
    std::shared_ptr<Device> device;
    ::vk::UniquePipelineLayout layout;
    std::vector<std::shared_ptr<DescriptorSetLayout>> set_layouts;
    uint32_t push_constant_size = 0;
    GCShaderStageFlags push_constant_stages = static_cast<GCShaderStageFlags>(0);
};

struct RenderPipeline {
    std::shared_ptr<Device> device;
    ::vk::UniquePipeline pipeline;
    std::shared_ptr<PipelineLayout> layout;
};

struct ComputePipeline {
    std::shared_ptr<Device> device;
    ::vk::UniquePipeline pipeline;
    std::shared_ptr<PipelineLayout> layout;
};

struct MeshPipeline {
    std::shared_ptr<Device> device;
    ::vk::UniquePipeline pipeline;
    std::shared_ptr<PipelineLayout> layout;
};

struct RayTracingPipeline {
    std::shared_ptr<Device> device;
    ::vk::UniquePipeline pipeline;
    std::shared_ptr<PipelineLayout> layout;
    std::unique_ptr<Buffer> sbt_buffer;
    ::vk::StridedDeviceAddressRegionKHR raygen_region{};
    ::vk::StridedDeviceAddressRegionKHR miss_region{};
    ::vk::StridedDeviceAddressRegionKHR hit_region{};
    ::vk::StridedDeviceAddressRegionKHR callable_region{};
    uint32_t max_recursion_depth = 1;
};

struct AccelerationStructure {
    std::shared_ptr<Device> device;
    ::vk::UniqueAccelerationStructureKHR accel;
    std::unique_ptr<Buffer> buffer;
    std::unique_ptr<Buffer> scratch_buffer;
    ::vk::DeviceAddress device_address = 0;
    bool top_level = false;
};

struct Surface {
    std::shared_ptr<Instance> instance;
    ::vk::UniqueSurfaceKHR surface;
    GCNativeHandleType handle_type;
    void* handle = nullptr;
};

struct Swapchain {
    std::shared_ptr<Device> device;
    std::shared_ptr<Surface> surface;
    ::vk::UniqueSwapchainKHR swapchain;
    std::vector<::vk::Image> images;
    std::vector<::vk::UniqueImageView> views;
    uint32_t current_index = 0;
    GCTextureFormat format = GC_FORMAT_BGRA8_UNORM;
    uint32_t width = 0;
    uint32_t height = 0;
};

} // namespace ep::vk

#endif // GCRAFT_VULKAN_INTERNAL_H
