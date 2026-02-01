// gcraft_resource.cpp - Buffer, texture, sampler, and shader library implementation

#include "gcraft_internal.h"

using namespace ep::vk;

// ============================================================================
// Buffer
// ============================================================================

extern "C" GCError GCBufferCreate(GCDevicePtr device_ptr, const GCBufferDesc* desc, GCBufferPtr* out_buffer) {
    if (!device_ptr || !desc || !out_buffer) return invalid_argument("device, desc, or out_buffer is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    auto buffer = std::make_unique<Buffer>();
    buffer->device = device->shared_from_this();
    buffer->size = desc->size;
    buffer->usage = desc->usage;
    buffer->host_visible = desc->host_visible;

    VkBufferCreateInfo buffer_info{
        .sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
        .size = desc->size,
        .usage = static_cast<VkBufferUsageFlags>(to_vk_buffer_usage(desc->usage)),
        .sharingMode = VK_SHARING_MODE_EXCLUSIVE,
    };

    VmaAllocationCreateInfo alloc_info{
        // Use RANDOM_BIT to support both read and write access on host-visible buffers
        .flags = desc->host_visible ? VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT : 0u,
        .usage = desc->host_visible ? VMA_MEMORY_USAGE_AUTO_PREFER_HOST : VMA_MEMORY_USAGE_AUTO_PREFER_DEVICE,
    };

    VkBuffer vk_buffer;
    if (vmaCreateBuffer(device->allocator.get(), &buffer_info, &alloc_info,
                        &vk_buffer, &buffer->allocation, nullptr) != VK_SUCCESS) {
        return out_of_memory();
    }

    buffer->buffer = vk_buffer;
    *out_buffer = reinterpret_cast<GCBufferPtr>(buffer.release());
    return ok();
}

extern "C" GCError GCBufferDestroy(GCBufferPtr buffer_ptr) {
    if (buffer_ptr) {
        auto* buffer = reinterpret_cast<Buffer*>(buffer_ptr);
        vmaDestroyBuffer(buffer->device->allocator.get(),
                         static_cast<VkBuffer>(buffer->buffer),
                         buffer->allocation);
        delete buffer;
    }
    return ok();
}

extern "C" GCError GCBufferMap(GCBufferPtr buffer_ptr, void** out_data) {
    if (!buffer_ptr || !out_data) return invalid_argument("buffer or out_data is NULL");

    auto* buffer = reinterpret_cast<Buffer*>(buffer_ptr);
    if (!buffer->host_visible) {
        return invalid_argument("buffer is not host-visible");
    }

    VkResult result = vmaMapMemory(buffer->device->allocator.get(), buffer->allocation, out_data);
    if (result != VK_SUCCESS) {
        return from_vk_result(static_cast<::vk::Result>(result));
    }

    // Invalidate to ensure GPU writes are visible to CPU (for non-coherent memory)
    vmaInvalidateAllocation(buffer->device->allocator.get(), buffer->allocation, 0, VK_WHOLE_SIZE);
    return ok();
}

extern "C" GCError GCBufferUnmap(GCBufferPtr buffer_ptr) {
    if (!buffer_ptr) return invalid_argument("buffer is NULL");

    auto* buffer = reinterpret_cast<Buffer*>(buffer_ptr);
    if (!buffer->host_visible) {
        return invalid_argument("buffer is not host-visible");
    }

    // Flush to ensure CPU writes are visible to GPU (for non-coherent memory)
    vmaFlushAllocation(buffer->device->allocator.get(), buffer->allocation, 0, VK_WHOLE_SIZE);
    vmaUnmapMemory(buffer->device->allocator.get(), buffer->allocation);
    return ok();
}

// ============================================================================
// Texture
// ============================================================================

extern "C" GCError GCTextureCreate(GCDevicePtr device_ptr, const GCTextureDesc* desc, GCTexturePtr* out_texture) {
    if (!device_ptr || !desc || !out_texture) return invalid_argument("device, desc, or out_texture is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    auto texture = std::make_unique<Texture>();
    texture->device = device->shared_from_this();
    texture->desc = *desc;

    bool is_cube = desc->dimension == GC_TEXTURE_DIM_CUBE;
    uint32_t array_layers = desc->array_layers > 0 ? desc->array_layers : 1;
    if (is_cube && array_layers < 6) array_layers = 6;

    VkImageCreateInfo image_info{
        .sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
        .flags = is_cube ? VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT : 0u,
        .imageType = static_cast<VkImageType>(to_vk_image_type(desc->dimension)),
        .format = static_cast<VkFormat>(to_vk_format(desc->format)),
        .extent = {desc->width, desc->height, desc->depth > 0 ? desc->depth : 1},
        .mipLevels = desc->mip_levels > 0 ? desc->mip_levels : 1,
        .arrayLayers = array_layers,
        .samples = VK_SAMPLE_COUNT_1_BIT,
        .tiling = VK_IMAGE_TILING_OPTIMAL,
        .usage = static_cast<VkImageUsageFlags>(to_vk_image_usage(desc->usage)),
        .sharingMode = VK_SHARING_MODE_EXCLUSIVE,
        .initialLayout = VK_IMAGE_LAYOUT_UNDEFINED,
    };

    VmaAllocationCreateInfo alloc_info{
        .usage = VMA_MEMORY_USAGE_GPU_ONLY,
    };

    VkImage vk_image;
    if (vmaCreateImage(device->allocator.get(), &image_info, &alloc_info,
                       &vk_image, &texture->allocation, nullptr) != VK_SUCCESS) {
        return out_of_memory();
    }

    texture->image = vk_image;

    // Create image view
    bool is_depth = desc->format == GC_FORMAT_D24S8 || desc->format == GC_FORMAT_D32_FLOAT;
    ::vk::ImageAspectFlags aspect = is_depth
        ? ::vk::ImageAspectFlagBits::eDepth
        : ::vk::ImageAspectFlagBits::eColor;

    auto [result, view] = device->device->createImageViewUnique(::vk::ImageViewCreateInfo{
        .image = texture->image,
        .viewType = to_vk_image_view_type(desc->dimension, array_layers),
        .format = to_vk_format(desc->format),
        .subresourceRange = ::vk::ImageSubresourceRange{
            .aspectMask = aspect,
            .baseMipLevel = 0,
            .levelCount = desc->mip_levels > 0 ? desc->mip_levels : 1,
            .baseArrayLayer = 0,
            .layerCount = array_layers,
        },
    });

    if (result != ::vk::Result::eSuccess) {
        vmaDestroyImage(device->allocator.get(), vk_image, texture->allocation);
        return from_vk_result(result);
    }

    texture->view = std::move(view);
    *out_texture = reinterpret_cast<GCTexturePtr>(texture.release());
    return ok();
}

extern "C" GCError GCTextureDestroy(GCTexturePtr texture_ptr) {
    if (texture_ptr) {
        auto* texture = reinterpret_cast<Texture*>(texture_ptr);
        // View is destroyed automatically via UniqueImageView
        vmaDestroyImage(texture->device->allocator.get(),
                        static_cast<VkImage>(texture->image),
                        texture->allocation);
        delete texture;
    }
    return ok();
}

// ============================================================================
// Sampler
// ============================================================================

extern "C" GCError GCSamplerCreate(GCDevicePtr device_ptr, const GCSamplerDesc* desc, GCSamplerPtr* out_sampler) {
    if (!device_ptr || !desc || !out_sampler) return invalid_argument("device, desc, or out_sampler is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    auto sampler = std::make_unique<Sampler>();
    sampler->device = device->shared_from_this();

    auto [result, vk_sampler] = device->device->createSamplerUnique(::vk::SamplerCreateInfo{
        .magFilter = to_vk_filter(desc->mag_filter),
        .minFilter = to_vk_filter(desc->min_filter),
        .mipmapMode = desc->min_filter == GC_FILTER_LINEAR
            ? ::vk::SamplerMipmapMode::eLinear
            : ::vk::SamplerMipmapMode::eNearest,
        .addressModeU = to_vk_address_mode(desc->address_u),
        .addressModeV = to_vk_address_mode(desc->address_v),
        .addressModeW = to_vk_address_mode(desc->address_w),
        .anisotropyEnable = desc->max_anisotropy > 1.0f,
        .maxAnisotropy = desc->max_anisotropy,
        .compareEnable = desc->compare_op != GC_COMPARE_ALWAYS,
        .compareOp = to_vk_compare_op(desc->compare_op),
        .maxLod = VK_LOD_CLAMP_NONE,
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    sampler->sampler = std::move(vk_sampler);
    *out_sampler = reinterpret_cast<GCSamplerPtr>(sampler.release());
    return ok();
}

extern "C" GCError GCSamplerDestroy(GCSamplerPtr sampler) {
    delete reinterpret_cast<Sampler*>(sampler);
    return ok();
}

// ============================================================================
// Shader Library
// ============================================================================

extern "C" GCError GCShaderLibraryCreate(GCDevicePtr device_ptr, const GCShaderLibraryDesc* desc,
                                          GCShaderLibraryPtr* out_library) {
    if (!device_ptr || !desc || !out_library) return invalid_argument("device, desc, or out_library is NULL");
    if (desc->format != GC_SHADER_SPIRV) return unsupported("only SPIR-V shaders supported on Vulkan");
    if (!desc->data || desc->size == 0) return invalid_argument("shader data is empty");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    auto library = std::make_unique<ShaderLibrary>();
    library->device = device->shared_from_this();

    // Copy SPIR-V data
    library->spirv_data.assign(desc->data, desc->data + desc->size);

    auto [result, module] = device->device->createShaderModuleUnique(::vk::ShaderModuleCreateInfo{
        .codeSize = desc->size,
        .pCode = reinterpret_cast<const uint32_t*>(desc->data),
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    library->module = std::move(module);
    *out_library = reinterpret_cast<GCShaderLibraryPtr>(library.release());
    return ok();
}

extern "C" GCError GCShaderLibraryDestroy(GCShaderLibraryPtr library) {
    delete reinterpret_cast<ShaderLibrary*>(library);
    return ok();
}
