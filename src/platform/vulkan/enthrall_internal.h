// enthrall_internal.h - Internal structures and utilities for Vulkan backend
// Uses modern C++ with smart pointers, vulkan-hpp, volk, and VMA

#ifndef ENTHRALL_VULKAN_INTERNAL_H
#define ENTHRALL_VULKAN_INTERNAL_H

// Configure volk before vulkan headers (implementation in enthrall_instance.cpp)
#include <volk/volk.h>

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
#include <vma/vk_mem_alloc.h>

#include <memory>
#include <vector>
#include <string>
#include <optional>
#include <array>
#include <cstring>

// C API headers
extern "C" {
#include "enthrall_types.h"
#include "enthrall_command.h"
#include "enthrall_descriptor.h"
#include "enthrall_device.h"
#include "enthrall_instance.h"
#include "enthrall_pipeline.h"
#include "enthrall_resource.h"
#include "enthrall_surface.h"
#include "enthrall_sync.h"
}

namespace ep::vk {

// ============================================================================
// Error Helpers
// ============================================================================

inline EPError ok() {
    return EPError{.code = EP_E_OK, .message = nullptr};
}

inline EPError error(EPErrorCode code, const char* msg) {
    return EPError{.code = code, .message = msg};
}

inline EPError invalid_argument(const char* msg) {
    return error(EP_E_INVALID_ARGUMENT, msg);
}

inline EPError out_of_memory() {
    return error(EP_E_OUT_OF_MEMORY, "out of memory");
}

inline EPError unsupported(const char* msg) {
    return error(EP_E_UNSUPPORTED, msg);
}

inline EPError from_vk_result(::vk::Result result) {
    switch (result) {
        case ::vk::Result::eSuccess:
            return ok();
        case ::vk::Result::eErrorOutOfHostMemory:
        case ::vk::Result::eErrorOutOfDeviceMemory:
            return out_of_memory();
        case ::vk::Result::eErrorDeviceLost:
            return error(EP_E_DEVICE_LOST, "vulkan device lost");
        case ::vk::Result::eErrorFeatureNotPresent:
            return unsupported("vulkan feature not present");
        case ::vk::Result::eErrorExtensionNotPresent:
            return unsupported("vulkan extension not present");
        case ::vk::Result::eErrorLayerNotPresent:
            return unsupported("vulkan layer not present");
        case ::vk::Result::eErrorIncompatibleDriver:
            return error(EP_E_INVALID_STATE, "vulkan incompatible driver");
        case ::vk::Result::eErrorInitializationFailed:
            return error(EP_E_INVALID_STATE, "vulkan initialization failed");
        default:
            return error(EP_E_INVALID_STATE, "vulkan error");
    }
}

// ============================================================================
// Format Conversion
// ============================================================================

inline ::vk::Format to_vk_format(EPTextureFormat format) {
    switch (format) {
        case EP_FORMAT_RGBA8_UNORM:  return ::vk::Format::eR8G8B8A8Unorm;
        case EP_FORMAT_BGRA8_UNORM:  return ::vk::Format::eB8G8R8A8Unorm;
        case EP_FORMAT_RGBA16_FLOAT: return ::vk::Format::eR16G16B16A16Sfloat;
        case EP_FORMAT_RGBA32_FLOAT: return ::vk::Format::eR32G32B32A32Sfloat;
        case EP_FORMAT_D24S8:        return ::vk::Format::eD24UnormS8Uint;
        case EP_FORMAT_D32_FLOAT:    return ::vk::Format::eD32Sfloat;
        default:                     return ::vk::Format::eUndefined;
    }
}

inline ::vk::Filter to_vk_filter(EPFilter filter) {
    return filter == EP_FILTER_LINEAR ? ::vk::Filter::eLinear : ::vk::Filter::eNearest;
}

inline ::vk::SamplerAddressMode to_vk_address_mode(EPAddressMode mode) {
    switch (mode) {
        case EP_ADDRESS_CLAMP_TO_EDGE:  return ::vk::SamplerAddressMode::eClampToEdge;
        case EP_ADDRESS_REPEAT:         return ::vk::SamplerAddressMode::eRepeat;
        case EP_ADDRESS_MIRROR_REPEAT:  return ::vk::SamplerAddressMode::eMirroredRepeat;
        default:                        return ::vk::SamplerAddressMode::eClampToEdge;
    }
}

inline ::vk::CompareOp to_vk_compare_op(EPCompareOp op) {
    switch (op) {
        case EP_COMPARE_NEVER:         return ::vk::CompareOp::eNever;
        case EP_COMPARE_LESS:          return ::vk::CompareOp::eLess;
        case EP_COMPARE_EQUAL:         return ::vk::CompareOp::eEqual;
        case EP_COMPARE_LESS_EQUAL:    return ::vk::CompareOp::eLessOrEqual;
        case EP_COMPARE_GREATER:       return ::vk::CompareOp::eGreater;
        case EP_COMPARE_NOT_EQUAL:     return ::vk::CompareOp::eNotEqual;
        case EP_COMPARE_GREATER_EQUAL: return ::vk::CompareOp::eGreaterOrEqual;
        case EP_COMPARE_ALWAYS:        return ::vk::CompareOp::eAlways;
        default:                       return ::vk::CompareOp::eAlways;
    }
}

inline ::vk::AttachmentLoadOp to_vk_load_op(EPAttachmentLoadOp op) {
    switch (op) {
        case EP_LOAD_OP_LOAD:      return ::vk::AttachmentLoadOp::eLoad;
        case EP_LOAD_OP_CLEAR:     return ::vk::AttachmentLoadOp::eClear;
        case EP_LOAD_OP_DONT_CARE: return ::vk::AttachmentLoadOp::eDontCare;
        default:                   return ::vk::AttachmentLoadOp::eDontCare;
    }
}

inline ::vk::AttachmentStoreOp to_vk_store_op(EPAttachmentStoreOp op) {
    switch (op) {
        case EP_STORE_OP_STORE:     return ::vk::AttachmentStoreOp::eStore;
        case EP_STORE_OP_DONT_CARE: return ::vk::AttachmentStoreOp::eDontCare;
        default:                    return ::vk::AttachmentStoreOp::eStore;
    }
}

inline ::vk::ImageType to_vk_image_type(EPTextureDimension dim) {
    switch (dim) {
        case EP_TEXTURE_DIM_1D:   return ::vk::ImageType::e1D;
        case EP_TEXTURE_DIM_2D:   return ::vk::ImageType::e2D;
        case EP_TEXTURE_DIM_3D:   return ::vk::ImageType::e3D;
        case EP_TEXTURE_DIM_CUBE: return ::vk::ImageType::e2D;
        default:                  return ::vk::ImageType::e2D;
    }
}

inline ::vk::ImageViewType to_vk_image_view_type(EPTextureDimension dim, uint32_t array_layers) {
    switch (dim) {
        case EP_TEXTURE_DIM_1D:
            return array_layers > 1 ? ::vk::ImageViewType::e1DArray : ::vk::ImageViewType::e1D;
        case EP_TEXTURE_DIM_2D:
            return array_layers > 1 ? ::vk::ImageViewType::e2DArray : ::vk::ImageViewType::e2D;
        case EP_TEXTURE_DIM_3D:
            return ::vk::ImageViewType::e3D;
        case EP_TEXTURE_DIM_CUBE:
            return array_layers > 6 ? ::vk::ImageViewType::eCubeArray : ::vk::ImageViewType::eCube;
        default:
            return ::vk::ImageViewType::e2D;
    }
}

inline ::vk::ImageUsageFlags to_vk_image_usage(EPTextureUsageFlags flags) {
    ::vk::ImageUsageFlags usage;
    if (flags & EP_TEXTURE_USAGE_SAMPLED_BIT)          usage |= ::vk::ImageUsageFlagBits::eSampled;
    if (flags & EP_TEXTURE_USAGE_STORAGE_BIT)          usage |= ::vk::ImageUsageFlagBits::eStorage;
    if (flags & EP_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT) usage |= ::vk::ImageUsageFlagBits::eColorAttachment;
    if (flags & EP_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT) usage |= ::vk::ImageUsageFlagBits::eDepthStencilAttachment;
    if (flags & EP_TEXTURE_USAGE_TRANSFER_SRC_BIT)     usage |= ::vk::ImageUsageFlagBits::eTransferSrc;
    if (flags & EP_TEXTURE_USAGE_TRANSFER_DST_BIT)     usage |= ::vk::ImageUsageFlagBits::eTransferDst;
    return usage;
}

inline ::vk::BufferUsageFlags to_vk_buffer_usage(EPBufferUsageFlags flags) {
    ::vk::BufferUsageFlags usage;
    if (flags & EP_BUFFER_USAGE_TRANSFER_SRC_BIT) usage |= ::vk::BufferUsageFlagBits::eTransferSrc;
    if (flags & EP_BUFFER_USAGE_TRANSFER_DST_BIT) usage |= ::vk::BufferUsageFlagBits::eTransferDst;
    if (flags & EP_BUFFER_USAGE_VERTEX_BIT)       usage |= ::vk::BufferUsageFlagBits::eVertexBuffer;
    if (flags & EP_BUFFER_USAGE_INDEX_BIT)        usage |= ::vk::BufferUsageFlagBits::eIndexBuffer;
    if (flags & EP_BUFFER_USAGE_UNIFORM_BIT)      usage |= ::vk::BufferUsageFlagBits::eUniformBuffer;
    if (flags & EP_BUFFER_USAGE_STORAGE_BIT)      usage |= ::vk::BufferUsageFlagBits::eStorageBuffer;
    if (flags & EP_BUFFER_USAGE_INDIRECT_BIT)     usage |= ::vk::BufferUsageFlagBits::eIndirectBuffer;
    if (flags & EP_BUFFER_USAGE_ACCEL_BIT) {
        usage |= ::vk::BufferUsageFlagBits::eAccelerationStructureBuildInputReadOnlyKHR;
        usage |= ::vk::BufferUsageFlagBits::eShaderDeviceAddress;
    }
    return usage;
}

inline ::vk::ImageLayout to_vk_image_layout(EPTextureLayout layout) {
    switch (layout) {
        case EP_TEXTURE_LAYOUT_UNDEFINED:        return ::vk::ImageLayout::eUndefined;
        case EP_TEXTURE_LAYOUT_GENERAL:          return ::vk::ImageLayout::eGeneral;
        case EP_TEXTURE_LAYOUT_COLOR_ATTACHMENT: return ::vk::ImageLayout::eColorAttachmentOptimal;
        case EP_TEXTURE_LAYOUT_DEPTH_STENCIL:    return ::vk::ImageLayout::eDepthStencilAttachmentOptimal;
        case EP_TEXTURE_LAYOUT_SHADER_READ:      return ::vk::ImageLayout::eShaderReadOnlyOptimal;
        case EP_TEXTURE_LAYOUT_TRANSFER_SRC:     return ::vk::ImageLayout::eTransferSrcOptimal;
        case EP_TEXTURE_LAYOUT_TRANSFER_DST:     return ::vk::ImageLayout::eTransferDstOptimal;
        case EP_TEXTURE_LAYOUT_PRESENT:          return ::vk::ImageLayout::ePresentSrcKHR;
        default:                                 return ::vk::ImageLayout::eUndefined;
    }
}

inline ::vk::PipelineStageFlags to_vk_pipeline_stage(EPPipelineStageFlags flags) {
    ::vk::PipelineStageFlags vk_flags;
    if (flags & EP_STAGE_TOP_OF_PIPE_BIT)        vk_flags |= ::vk::PipelineStageFlagBits::eTopOfPipe;
    if (flags & EP_STAGE_DRAW_INDIRECT_BIT)      vk_flags |= ::vk::PipelineStageFlagBits::eDrawIndirect;
    if (flags & EP_STAGE_VERTEX_INPUT_BIT)       vk_flags |= ::vk::PipelineStageFlagBits::eVertexInput;
    if (flags & EP_STAGE_VERTEX_SHADER_BIT)      vk_flags |= ::vk::PipelineStageFlagBits::eVertexShader;
    if (flags & EP_STAGE_FRAGMENT_SHADER_BIT)    vk_flags |= ::vk::PipelineStageFlagBits::eFragmentShader;
    if (flags & EP_STAGE_COMPUTE_SHADER_BIT)     vk_flags |= ::vk::PipelineStageFlagBits::eComputeShader;
    if (flags & EP_STAGE_MESH_SHADER_BIT)        vk_flags |= ::vk::PipelineStageFlagBits::eMeshShaderEXT;
    if (flags & EP_STAGE_RAY_TRACING_SHADER_BIT) vk_flags |= ::vk::PipelineStageFlagBits::eRayTracingShaderKHR;
    if (flags & EP_STAGE_TRANSFER_BIT)           vk_flags |= ::vk::PipelineStageFlagBits::eTransfer;
    if (flags & EP_STAGE_BOTTOM_OF_PIPE_BIT)     vk_flags |= ::vk::PipelineStageFlagBits::eBottomOfPipe;
    return vk_flags;
}

// Vulkan 1.3 synchronization2 stage flags
inline ::vk::PipelineStageFlags2 to_vk_pipeline_stage2(EPPipelineStageFlags flags) {
    ::vk::PipelineStageFlags2 vk_flags;
    if (flags & EP_STAGE_TOP_OF_PIPE_BIT)        vk_flags |= ::vk::PipelineStageFlagBits2::eTopOfPipe;
    if (flags & EP_STAGE_DRAW_INDIRECT_BIT)      vk_flags |= ::vk::PipelineStageFlagBits2::eDrawIndirect;
    if (flags & EP_STAGE_VERTEX_INPUT_BIT)       vk_flags |= ::vk::PipelineStageFlagBits2::eVertexInput;
    if (flags & EP_STAGE_VERTEX_SHADER_BIT)      vk_flags |= ::vk::PipelineStageFlagBits2::eVertexShader;
    if (flags & EP_STAGE_FRAGMENT_SHADER_BIT)    vk_flags |= ::vk::PipelineStageFlagBits2::eFragmentShader;
    if (flags & EP_STAGE_COMPUTE_SHADER_BIT)     vk_flags |= ::vk::PipelineStageFlagBits2::eComputeShader;
    if (flags & EP_STAGE_MESH_SHADER_BIT)        vk_flags |= ::vk::PipelineStageFlagBits2::eMeshShaderEXT;
    if (flags & EP_STAGE_RAY_TRACING_SHADER_BIT) vk_flags |= ::vk::PipelineStageFlagBits2::eRayTracingShaderKHR;
    if (flags & EP_STAGE_TRANSFER_BIT)           vk_flags |= ::vk::PipelineStageFlagBits2::eTransfer;
    if (flags & EP_STAGE_BOTTOM_OF_PIPE_BIT)     vk_flags |= ::vk::PipelineStageFlagBits2::eBottomOfPipe;
    return vk_flags;
}

inline ::vk::AccessFlags to_vk_access_flags(EPAccessFlags flags) {
    ::vk::AccessFlags vk_flags;
    if (flags & EP_ACCESS_INDIRECT_COMMAND_READ_BIT)  vk_flags |= ::vk::AccessFlagBits::eIndirectCommandRead;
    if (flags & EP_ACCESS_VERTEX_READ_BIT)            vk_flags |= ::vk::AccessFlagBits::eVertexAttributeRead;
    if (flags & EP_ACCESS_INDEX_READ_BIT)             vk_flags |= ::vk::AccessFlagBits::eIndexRead;
    if (flags & EP_ACCESS_UNIFORM_READ_BIT)           vk_flags |= ::vk::AccessFlagBits::eUniformRead;
    if (flags & EP_ACCESS_SHADER_READ_BIT)            vk_flags |= ::vk::AccessFlagBits::eShaderRead;
    if (flags & EP_ACCESS_SHADER_WRITE_BIT)           vk_flags |= ::vk::AccessFlagBits::eShaderWrite;
    if (flags & EP_ACCESS_COLOR_ATTACHMENT_READ_BIT)  vk_flags |= ::vk::AccessFlagBits::eColorAttachmentRead;
    if (flags & EP_ACCESS_COLOR_ATTACHMENT_WRITE_BIT) vk_flags |= ::vk::AccessFlagBits::eColorAttachmentWrite;
    if (flags & EP_ACCESS_DEPTH_STENCIL_READ_BIT)     vk_flags |= ::vk::AccessFlagBits::eDepthStencilAttachmentRead;
    if (flags & EP_ACCESS_DEPTH_STENCIL_WRITE_BIT)    vk_flags |= ::vk::AccessFlagBits::eDepthStencilAttachmentWrite;
    if (flags & EP_ACCESS_TRANSFER_READ_BIT)          vk_flags |= ::vk::AccessFlagBits::eTransferRead;
    if (flags & EP_ACCESS_TRANSFER_WRITE_BIT)         vk_flags |= ::vk::AccessFlagBits::eTransferWrite;
    if (flags & EP_ACCESS_HOST_READ_BIT)              vk_flags |= ::vk::AccessFlagBits::eHostRead;
    if (flags & EP_ACCESS_HOST_WRITE_BIT)             vk_flags |= ::vk::AccessFlagBits::eHostWrite;
    return vk_flags;
}

// Vulkan 1.3 synchronization2 access flags
inline ::vk::AccessFlags2 to_vk_access_flags2(EPAccessFlags flags) {
    ::vk::AccessFlags2 vk_flags;
    if (flags & EP_ACCESS_INDIRECT_COMMAND_READ_BIT)  vk_flags |= ::vk::AccessFlagBits2::eIndirectCommandRead;
    if (flags & EP_ACCESS_VERTEX_READ_BIT)            vk_flags |= ::vk::AccessFlagBits2::eVertexAttributeRead;
    if (flags & EP_ACCESS_INDEX_READ_BIT)             vk_flags |= ::vk::AccessFlagBits2::eIndexRead;
    if (flags & EP_ACCESS_UNIFORM_READ_BIT)           vk_flags |= ::vk::AccessFlagBits2::eUniformRead;
    if (flags & EP_ACCESS_SHADER_READ_BIT)            vk_flags |= ::vk::AccessFlagBits2::eShaderRead;
    if (flags & EP_ACCESS_SHADER_WRITE_BIT)           vk_flags |= ::vk::AccessFlagBits2::eShaderWrite;
    if (flags & EP_ACCESS_COLOR_ATTACHMENT_READ_BIT)  vk_flags |= ::vk::AccessFlagBits2::eColorAttachmentRead;
    if (flags & EP_ACCESS_COLOR_ATTACHMENT_WRITE_BIT) vk_flags |= ::vk::AccessFlagBits2::eColorAttachmentWrite;
    if (flags & EP_ACCESS_DEPTH_STENCIL_READ_BIT)     vk_flags |= ::vk::AccessFlagBits2::eDepthStencilAttachmentRead;
    if (flags & EP_ACCESS_DEPTH_STENCIL_WRITE_BIT)    vk_flags |= ::vk::AccessFlagBits2::eDepthStencilAttachmentWrite;
    if (flags & EP_ACCESS_TRANSFER_READ_BIT)          vk_flags |= ::vk::AccessFlagBits2::eTransferRead;
    if (flags & EP_ACCESS_TRANSFER_WRITE_BIT)         vk_flags |= ::vk::AccessFlagBits2::eTransferWrite;
    if (flags & EP_ACCESS_HOST_READ_BIT)              vk_flags |= ::vk::AccessFlagBits2::eHostRead;
    if (flags & EP_ACCESS_HOST_WRITE_BIT)             vk_flags |= ::vk::AccessFlagBits2::eHostWrite;
    return vk_flags;
}

inline ::vk::ShaderStageFlags to_vk_shader_stage(EPShaderStageFlags flags) {
    ::vk::ShaderStageFlags vk_flags;
    if (flags & EP_STAGE_VERTEX_BIT)   vk_flags |= ::vk::ShaderStageFlagBits::eVertex;
    if (flags & EP_STAGE_FRAGMENT_BIT) vk_flags |= ::vk::ShaderStageFlagBits::eFragment;
    if (flags & EP_STAGE_COMPUTE_BIT)  vk_flags |= ::vk::ShaderStageFlagBits::eCompute;
    if (flags & EP_STAGE_MESH_BIT)     vk_flags |= ::vk::ShaderStageFlagBits::eMeshEXT;
    if (flags & EP_STAGE_TASK_BIT)     vk_flags |= ::vk::ShaderStageFlagBits::eTaskEXT;
    if (flags & EP_STAGE_RAYGEN_BIT)   vk_flags |= ::vk::ShaderStageFlagBits::eRaygenKHR;
    if (flags & EP_STAGE_MISS_BIT)     vk_flags |= ::vk::ShaderStageFlagBits::eMissKHR;
    if (flags & EP_STAGE_HIT_BIT)      vk_flags |= ::vk::ShaderStageFlagBits::eClosestHitKHR;
    return vk_flags;
}

inline ::vk::DescriptorType to_vk_descriptor_type(EPDescriptorType type) {
    switch (type) {
        case EP_DESCRIPTOR_UNIFORM_BUFFER:         return ::vk::DescriptorType::eUniformBuffer;
        case EP_DESCRIPTOR_STORAGE_BUFFER:         return ::vk::DescriptorType::eStorageBuffer;
        case EP_DESCRIPTOR_SAMPLED_TEXTURE:        return ::vk::DescriptorType::eSampledImage;
        case EP_DESCRIPTOR_STORAGE_TEXTURE:        return ::vk::DescriptorType::eStorageImage;
        case EP_DESCRIPTOR_SAMPLER:                return ::vk::DescriptorType::eSampler;
        case EP_DESCRIPTOR_ACCELERATION_STRUCTURE: return ::vk::DescriptorType::eAccelerationStructureKHR;
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

struct Instance {
    ::vk::UniqueInstance instance;
    std::vector<::vk::PhysicalDevice> physical_devices;
    EPBackendFlags enabled_backends = static_cast<EPBackendFlags>(0);
    bool validation_enabled = false;
    bool debug_names_enabled = false;
};

struct Adapter {
    std::shared_ptr<Instance> instance;
    ::vk::PhysicalDevice physical_device;
    ::vk::PhysicalDeviceProperties properties;
    ::vk::PhysicalDeviceFeatures features;
    ::vk::PhysicalDeviceMemoryProperties memory_properties;
    EPAdapterProperties ep_properties{};
    uint32_t graphics_queue_family = UINT32_MAX;
    uint32_t compute_queue_family = UINT32_MAX;
    uint32_t transfer_queue_family = UINT32_MAX;
    bool has_mesh_shader = false;
    bool has_ray_tracing = false;
    bool has_timeline_semaphore = false;
    bool has_dynamic_rendering = false;
};

struct Device {
    std::shared_ptr<Adapter> adapter;
    ::vk::UniqueDevice device;
    UniqueVmaAllocator allocator;
    ::vk::Queue graphics_queue;
    ::vk::Queue compute_queue;
    ::vk::Queue transfer_queue;
    ::vk::UniqueCommandPool graphics_pool;
    ::vk::UniqueCommandPool compute_pool;
    ::vk::UniqueCommandPool transfer_pool;
    ::vk::UniqueDescriptorPool descriptor_pool;
    EPFeatureFlags features = static_cast<EPFeatureFlags>(0);
    bool validation_enabled = false;
    bool debug_names_enabled = false;
};

struct Queue {
    std::shared_ptr<Device> device;
    ::vk::Queue queue;
    ::vk::CommandPool pool;
    EPQueueType type;
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
    EPBufferUsageFlags usage = static_cast<EPBufferUsageFlags>(0);
    bool host_visible = false;
};

struct Texture {
    std::shared_ptr<Device> device;
    ::vk::Image image;  // Owned by VMA
    VmaAllocation allocation = nullptr;
    ::vk::UniqueImageView view;
    EPTextureDesc desc{};
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
    EPDescriptorType type = EP_DESCRIPTOR_UNIFORM_BUFFER;
    uint32_t count = 0;
    EPShaderStageFlags stages = static_cast<EPShaderStageFlags>(0);
};

struct DescriptorSetLayout {
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
    EPShaderStageFlags push_constant_stages = static_cast<EPShaderStageFlags>(0);
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
    EPNativeHandleType handle_type;
    void* handle = nullptr;
};

struct Swapchain {
    std::shared_ptr<Device> device;
    std::shared_ptr<Surface> surface;
    ::vk::UniqueSwapchainKHR swapchain;
    std::vector<::vk::Image> images;
    std::vector<::vk::UniqueImageView> views;
    uint32_t current_index = 0;
    EPTextureFormat format = EP_FORMAT_BGRA8_UNORM;
    uint32_t width = 0;
    uint32_t height = 0;
};

} // namespace ep::vk

#endif // ENTHRALL_VULKAN_INTERNAL_H
