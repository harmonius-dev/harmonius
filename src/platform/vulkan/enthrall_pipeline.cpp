// enthrall_pipeline.cpp - Pipeline and acceleration structure implementation

#include "enthrall_internal.h"

using namespace ep::vk;

// ============================================================================
// Render Pipeline
// ============================================================================

extern "C" EPError EPRenderPipelineCreate(EPDevicePtr device_ptr, const EPRenderPipelineDesc* desc,
                                           EPRenderPipelinePtr* out_pipeline) {
    if (!device_ptr || !desc || !out_pipeline) return invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library) return invalid_argument("shader library is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);
    auto* shader = reinterpret_cast<ShaderLibrary*>(desc->library);

    auto pipeline = std::make_unique<RenderPipeline>();
    pipeline->device = device->shared_from_this();

    if (desc->layout) {
        pipeline->layout = std::shared_ptr<PipelineLayout>(
            reinterpret_cast<PipelineLayout*>(desc->layout), [](PipelineLayout*){});
    }

    // Shader stages
    std::vector<::vk::PipelineShaderStageCreateInfo> stages;

    if (desc->vertex_entry) {
        stages.push_back(::vk::PipelineShaderStageCreateInfo{
            .stage = ::vk::ShaderStageFlagBits::eVertex,
            .module = shader->module.get(),
            .pName = desc->vertex_entry,
        });
    }

    if (desc->fragment_entry) {
        stages.push_back(::vk::PipelineShaderStageCreateInfo{
            .stage = ::vk::ShaderStageFlagBits::eFragment,
            .module = shader->module.get(),
            .pName = desc->fragment_entry,
        });
    }

    // Vertex input
    std::vector<::vk::VertexInputBindingDescription> binding_descs;
    std::vector<::vk::VertexInputAttributeDescription> attribute_descs;

    for (uint32_t i = 0; i < desc->binding_count; i++) {
        binding_descs.push_back(::vk::VertexInputBindingDescription{
            .binding = desc->bindings[i].binding,
            .stride = desc->bindings[i].stride,
            .inputRate = desc->bindings[i].per_instance
                ? ::vk::VertexInputRate::eInstance
                : ::vk::VertexInputRate::eVertex,
        });
    }

    for (uint32_t i = 0; i < desc->attribute_count; i++) {
        attribute_descs.push_back(::vk::VertexInputAttributeDescription{
            .location = desc->attributes[i].location,
            .binding = desc->attributes[i].binding,
            .format = to_vk_format(desc->attributes[i].format),
            .offset = desc->attributes[i].offset,
        });
    }

    ::vk::PipelineVertexInputStateCreateInfo vertex_input{
        .vertexBindingDescriptionCount = static_cast<uint32_t>(binding_descs.size()),
        .pVertexBindingDescriptions = binding_descs.data(),
        .vertexAttributeDescriptionCount = static_cast<uint32_t>(attribute_descs.size()),
        .pVertexAttributeDescriptions = attribute_descs.data(),
    };

    ::vk::PipelineInputAssemblyStateCreateInfo input_assembly{
        .topology = ::vk::PrimitiveTopology::eTriangleList,
    };

    ::vk::PipelineViewportStateCreateInfo viewport_state{
        .viewportCount = 1,
        .scissorCount = 1,
    };

    ::vk::PipelineRasterizationStateCreateInfo rasterization{
        .polygonMode = ::vk::PolygonMode::eFill,
        .cullMode = ::vk::CullModeFlagBits::eBack,
        .frontFace = ::vk::FrontFace::eCounterClockwise,
        .lineWidth = 1.0f,
    };

    ::vk::PipelineMultisampleStateCreateInfo multisample{
        .rasterizationSamples = ::vk::SampleCountFlagBits::e1,
    };

    ::vk::PipelineDepthStencilStateCreateInfo depth_stencil{
        .depthTestEnable = desc->raster_state.depth_test_enable,
        .depthWriteEnable = desc->raster_state.depth_write_enable,
        .depthCompareOp = to_vk_compare_op(desc->raster_state.depth_compare),
    };

    std::vector<::vk::PipelineColorBlendAttachmentState> blend_attachments(desc->color_format_count);
    for (auto& att : blend_attachments) {
        att = ::vk::PipelineColorBlendAttachmentState{
            .colorWriteMask = ::vk::ColorComponentFlagBits::eR | ::vk::ColorComponentFlagBits::eG |
                              ::vk::ColorComponentFlagBits::eB | ::vk::ColorComponentFlagBits::eA,
        };
    }

    ::vk::PipelineColorBlendStateCreateInfo color_blend{
        .attachmentCount = static_cast<uint32_t>(blend_attachments.size()),
        .pAttachments = blend_attachments.data(),
    };

    std::array<::vk::DynamicState, 2> dynamic_states = {
        ::vk::DynamicState::eViewport,
        ::vk::DynamicState::eScissor,
    };

    ::vk::PipelineDynamicStateCreateInfo dynamic_state{
        .dynamicStateCount = static_cast<uint32_t>(dynamic_states.size()),
        .pDynamicStates = dynamic_states.data(),
    };

    // Dynamic rendering format info
    std::vector<::vk::Format> color_formats;
    for (uint32_t i = 0; i < desc->color_format_count; i++) {
        color_formats.push_back(to_vk_format(desc->color_formats[i]));
    }

    ::vk::PipelineRenderingCreateInfo rendering_info{
        .colorAttachmentCount = static_cast<uint32_t>(color_formats.size()),
        .pColorAttachmentFormats = color_formats.data(),
        .depthAttachmentFormat = to_vk_format(desc->depth_format),
        .stencilAttachmentFormat = to_vk_format(desc->stencil_format),
    };

    ::vk::GraphicsPipelineCreateInfo create_info{
        .pNext = &rendering_info,
        .stageCount = static_cast<uint32_t>(stages.size()),
        .pStages = stages.data(),
        .pVertexInputState = &vertex_input,
        .pInputAssemblyState = &input_assembly,
        .pViewportState = &viewport_state,
        .pRasterizationState = &rasterization,
        .pMultisampleState = &multisample,
        .pDepthStencilState = &depth_stencil,
        .pColorBlendState = &color_blend,
        .pDynamicState = &dynamic_state,
        .layout = pipeline->layout ? pipeline->layout->layout.get() : ::vk::PipelineLayout{},
    };

    auto [result, vk_pipeline] = device->device->createGraphicsPipelineUnique(nullptr, create_info);
    if (result != ::vk::Result::eSuccess && result != ::vk::Result::ePipelineCompileRequiredEXT) {
        return from_vk_result(result);
    }

    pipeline->pipeline = std::move(vk_pipeline);
    *out_pipeline = reinterpret_cast<EPRenderPipelinePtr>(pipeline.release());
    return ok();
}

extern "C" EPError EPRenderPipelineDestroy(EPRenderPipelinePtr pipeline) {
    delete reinterpret_cast<RenderPipeline*>(pipeline);
    return ok();
}

// ============================================================================
// Compute Pipeline
// ============================================================================

extern "C" EPError EPComputePipelineCreate(EPDevicePtr device_ptr, const EPComputePipelineDesc* desc,
                                            EPComputePipelinePtr* out_pipeline) {
    if (!device_ptr || !desc || !out_pipeline) return invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library || !desc->entry) return invalid_argument("library or entry is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);
    auto* shader = reinterpret_cast<ShaderLibrary*>(desc->library);

    auto pipeline = std::make_unique<ComputePipeline>();
    pipeline->device = device->shared_from_this();

    if (desc->layout) {
        pipeline->layout = std::shared_ptr<PipelineLayout>(
            reinterpret_cast<PipelineLayout*>(desc->layout), [](PipelineLayout*){});
    }

    ::vk::ComputePipelineCreateInfo create_info{
        .stage = ::vk::PipelineShaderStageCreateInfo{
            .stage = ::vk::ShaderStageFlagBits::eCompute,
            .module = shader->module.get(),
            .pName = desc->entry,
        },
        .layout = pipeline->layout ? pipeline->layout->layout.get() : ::vk::PipelineLayout{},
    };

    auto [result, vk_pipeline] = device->device->createComputePipelineUnique(nullptr, create_info);
    if (result != ::vk::Result::eSuccess && result != ::vk::Result::ePipelineCompileRequiredEXT) {
        return from_vk_result(result);
    }

    pipeline->pipeline = std::move(vk_pipeline);
    *out_pipeline = reinterpret_cast<EPComputePipelinePtr>(pipeline.release());
    return ok();
}

extern "C" EPError EPComputePipelineDestroy(EPComputePipelinePtr pipeline) {
    delete reinterpret_cast<ComputePipeline*>(pipeline);
    return ok();
}

// ============================================================================
// Mesh Pipeline
// ============================================================================

extern "C" EPError EPMeshPipelineCreate(EPDevicePtr device_ptr, const EPMeshPipelineDesc* desc,
                                         EPMeshPipelinePtr* out_pipeline) {
    if (!device_ptr || !desc || !out_pipeline) return invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library || !desc->mesh_entry) return invalid_argument("library or mesh_entry is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    if (!(device->features & EP_FEATURE_MESH_SHADER_BIT)) {
        return unsupported("mesh shaders not supported on this device");
    }

    auto* shader = reinterpret_cast<ShaderLibrary*>(desc->library);

    auto pipeline = std::make_unique<MeshPipeline>();
    pipeline->device = device->shared_from_this();

    if (desc->layout) {
        pipeline->layout = std::shared_ptr<PipelineLayout>(
            reinterpret_cast<PipelineLayout*>(desc->layout), [](PipelineLayout*){});
    }

    std::vector<::vk::PipelineShaderStageCreateInfo> stages;

    if (desc->task_entry) {
        stages.push_back(::vk::PipelineShaderStageCreateInfo{
            .stage = ::vk::ShaderStageFlagBits::eTaskEXT,
            .module = shader->module.get(),
            .pName = desc->task_entry,
        });
    }

    stages.push_back(::vk::PipelineShaderStageCreateInfo{
        .stage = ::vk::ShaderStageFlagBits::eMeshEXT,
        .module = shader->module.get(),
        .pName = desc->mesh_entry,
    });

    if (desc->fragment_entry) {
        stages.push_back(::vk::PipelineShaderStageCreateInfo{
            .stage = ::vk::ShaderStageFlagBits::eFragment,
            .module = shader->module.get(),
            .pName = desc->fragment_entry,
        });
    }

    ::vk::PipelineViewportStateCreateInfo viewport_state{
        .viewportCount = 1,
        .scissorCount = 1,
    };

    ::vk::PipelineRasterizationStateCreateInfo rasterization{
        .polygonMode = ::vk::PolygonMode::eFill,
        .cullMode = ::vk::CullModeFlagBits::eBack,
        .frontFace = ::vk::FrontFace::eCounterClockwise,
        .lineWidth = 1.0f,
    };

    ::vk::PipelineMultisampleStateCreateInfo multisample{
        .rasterizationSamples = ::vk::SampleCountFlagBits::e1,
    };

    ::vk::PipelineDepthStencilStateCreateInfo depth_stencil{
        .depthTestEnable = desc->raster_state.depth_test_enable,
        .depthWriteEnable = desc->raster_state.depth_write_enable,
        .depthCompareOp = to_vk_compare_op(desc->raster_state.depth_compare),
    };

    std::vector<::vk::PipelineColorBlendAttachmentState> blend_attachments(desc->color_format_count);
    for (auto& att : blend_attachments) {
        att = ::vk::PipelineColorBlendAttachmentState{
            .colorWriteMask = ::vk::ColorComponentFlagBits::eR | ::vk::ColorComponentFlagBits::eG |
                              ::vk::ColorComponentFlagBits::eB | ::vk::ColorComponentFlagBits::eA,
        };
    }

    ::vk::PipelineColorBlendStateCreateInfo color_blend{
        .attachmentCount = static_cast<uint32_t>(blend_attachments.size()),
        .pAttachments = blend_attachments.data(),
    };

    std::array<::vk::DynamicState, 2> dynamic_states = {
        ::vk::DynamicState::eViewport,
        ::vk::DynamicState::eScissor,
    };

    ::vk::PipelineDynamicStateCreateInfo dynamic_state{
        .dynamicStateCount = static_cast<uint32_t>(dynamic_states.size()),
        .pDynamicStates = dynamic_states.data(),
    };

    std::vector<::vk::Format> color_formats;
    for (uint32_t i = 0; i < desc->color_format_count; i++) {
        color_formats.push_back(to_vk_format(desc->color_formats[i]));
    }

    ::vk::PipelineRenderingCreateInfo rendering_info{
        .colorAttachmentCount = static_cast<uint32_t>(color_formats.size()),
        .pColorAttachmentFormats = color_formats.data(),
        .depthAttachmentFormat = to_vk_format(desc->depth_format),
        .stencilAttachmentFormat = to_vk_format(desc->stencil_format),
    };

    ::vk::GraphicsPipelineCreateInfo create_info{
        .pNext = &rendering_info,
        .stageCount = static_cast<uint32_t>(stages.size()),
        .pStages = stages.data(),
        .pViewportState = &viewport_state,
        .pRasterizationState = &rasterization,
        .pMultisampleState = &multisample,
        .pDepthStencilState = &depth_stencil,
        .pColorBlendState = &color_blend,
        .pDynamicState = &dynamic_state,
        .layout = pipeline->layout ? pipeline->layout->layout.get() : ::vk::PipelineLayout{},
    };

    auto [result, vk_pipeline] = device->device->createGraphicsPipelineUnique(nullptr, create_info);
    if (result != ::vk::Result::eSuccess && result != ::vk::Result::ePipelineCompileRequiredEXT) {
        return from_vk_result(result);
    }

    pipeline->pipeline = std::move(vk_pipeline);
    *out_pipeline = reinterpret_cast<EPMeshPipelinePtr>(pipeline.release());
    return ok();
}

extern "C" EPError EPMeshPipelineDestroy(EPMeshPipelinePtr pipeline) {
    delete reinterpret_cast<MeshPipeline*>(pipeline);
    return ok();
}

// ============================================================================
// Ray Tracing Pipeline
// ============================================================================

extern "C" EPError EPRayTracingPipelineCreate(EPDevicePtr device_ptr, const EPRayTracingPipelineDesc* desc,
                                               EPRayTracingPipelinePtr* out_pipeline) {
    if (!device_ptr || !desc || !out_pipeline) return invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library) return invalid_argument("library is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    if (!(device->features & EP_FEATURE_RAY_TRACING_BIT)) {
        return unsupported("ray tracing not supported on this device");
    }

    auto* shader = reinterpret_cast<ShaderLibrary*>(desc->library);

    auto pipeline = std::make_unique<RayTracingPipeline>();
    pipeline->device = device->shared_from_this();
    pipeline->max_recursion_depth = desc->max_recursion_depth;

    if (desc->layout) {
        pipeline->layout = std::shared_ptr<PipelineLayout>(
            reinterpret_cast<PipelineLayout*>(desc->layout), [](PipelineLayout*){});
    }

    std::vector<::vk::PipelineShaderStageCreateInfo> stages;
    std::vector<::vk::RayTracingShaderGroupCreateInfoKHR> groups;

    for (uint32_t i = 0; i < desc->group_count; i++) {
        const auto& g = desc->groups[i];

        if (g.raygen_entry) {
            uint32_t stage_idx = static_cast<uint32_t>(stages.size());
            stages.push_back(::vk::PipelineShaderStageCreateInfo{
                .stage = ::vk::ShaderStageFlagBits::eRaygenKHR,
                .module = shader->module.get(),
                .pName = g.raygen_entry,
            });
            groups.push_back(::vk::RayTracingShaderGroupCreateInfoKHR{
                .type = ::vk::RayTracingShaderGroupTypeKHR::eGeneral,
                .generalShader = stage_idx,
                .closestHitShader = VK_SHADER_UNUSED_KHR,
                .anyHitShader = VK_SHADER_UNUSED_KHR,
                .intersectionShader = VK_SHADER_UNUSED_KHR,
            });
        }

        if (g.miss_entry) {
            uint32_t stage_idx = static_cast<uint32_t>(stages.size());
            stages.push_back(::vk::PipelineShaderStageCreateInfo{
                .stage = ::vk::ShaderStageFlagBits::eMissKHR,
                .module = shader->module.get(),
                .pName = g.miss_entry,
            });
            groups.push_back(::vk::RayTracingShaderGroupCreateInfoKHR{
                .type = ::vk::RayTracingShaderGroupTypeKHR::eGeneral,
                .generalShader = stage_idx,
                .closestHitShader = VK_SHADER_UNUSED_KHR,
                .anyHitShader = VK_SHADER_UNUSED_KHR,
                .intersectionShader = VK_SHADER_UNUSED_KHR,
            });
        }

        if (g.hit_entry) {
            uint32_t hit_idx = static_cast<uint32_t>(stages.size());
            stages.push_back(::vk::PipelineShaderStageCreateInfo{
                .stage = ::vk::ShaderStageFlagBits::eClosestHitKHR,
                .module = shader->module.get(),
                .pName = g.hit_entry,
            });

            uint32_t any_hit_idx = VK_SHADER_UNUSED_KHR;
            if (g.any_hit_entry) {
                any_hit_idx = static_cast<uint32_t>(stages.size());
                stages.push_back(::vk::PipelineShaderStageCreateInfo{
                    .stage = ::vk::ShaderStageFlagBits::eAnyHitKHR,
                    .module = shader->module.get(),
                    .pName = g.any_hit_entry,
                });
            }

            uint32_t intersection_idx = VK_SHADER_UNUSED_KHR;
            ::vk::RayTracingShaderGroupTypeKHR group_type = ::vk::RayTracingShaderGroupTypeKHR::eTrianglesHitGroup;

            if (g.intersection_entry) {
                intersection_idx = static_cast<uint32_t>(stages.size());
                stages.push_back(::vk::PipelineShaderStageCreateInfo{
                    .stage = ::vk::ShaderStageFlagBits::eIntersectionKHR,
                    .module = shader->module.get(),
                    .pName = g.intersection_entry,
                });
                group_type = ::vk::RayTracingShaderGroupTypeKHR::eProceduralHitGroup;
            }

            groups.push_back(::vk::RayTracingShaderGroupCreateInfoKHR{
                .type = group_type,
                .generalShader = VK_SHADER_UNUSED_KHR,
                .closestHitShader = hit_idx,
                .anyHitShader = any_hit_idx,
                .intersectionShader = intersection_idx,
            });
        }
    }

    ::vk::RayTracingPipelineCreateInfoKHR create_info{
        .stageCount = static_cast<uint32_t>(stages.size()),
        .pStages = stages.data(),
        .groupCount = static_cast<uint32_t>(groups.size()),
        .pGroups = groups.data(),
        .maxPipelineRayRecursionDepth = desc->max_recursion_depth,
        .layout = pipeline->layout ? pipeline->layout->layout.get() : ::vk::PipelineLayout{},
    };

    auto [result, vk_pipeline] = device->device->createRayTracingPipelineKHRUnique(nullptr, nullptr, create_info);
    if (result != ::vk::Result::eSuccess && result != ::vk::Result::ePipelineCompileRequiredEXT) {
        return from_vk_result(result);
    }

    pipeline->pipeline = std::move(vk_pipeline);

    // TODO: Build shader binding table

    *out_pipeline = reinterpret_cast<EPRayTracingPipelinePtr>(pipeline.release());
    return ok();
}

extern "C" EPError EPRayTracingPipelineDestroy(EPRayTracingPipelinePtr pipeline) {
    delete reinterpret_cast<RayTracingPipeline*>(pipeline);
    return ok();
}

// ============================================================================
// Acceleration Structure
// ============================================================================

extern "C" EPError EPAccelerationStructureCreate(EPDevicePtr device_ptr, const EPAccelerationStructureDesc* desc,
                                                  EPAccelerationStructurePtr* out_as) {
    if (!device_ptr || !desc || !out_as) return invalid_argument("device, desc, or out_as is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    if (!(device->features & EP_FEATURE_RAY_TRACING_BIT)) {
        return unsupported("ray tracing not supported on this device");
    }

    auto as = std::make_unique<AccelerationStructure>();
    as->device = device->shared_from_this();
    as->top_level = desc->top_level;

    // Build geometry info
    std::vector<::vk::AccelerationStructureGeometryKHR> geometries;
    std::vector<uint32_t> primitive_counts;

    if (!desc->top_level) {
        for (uint32_t i = 0; i < desc->geometry_count; i++) {
            const auto& g = desc->geometries[i];

            auto* vertex_buffer = reinterpret_cast<Buffer*>(g.vertex_buffer);
            ::vk::DeviceAddress vertex_address = device->device->getBufferAddress(
                ::vk::BufferDeviceAddressInfo{.buffer = vertex_buffer->buffer});

            ::vk::AccelerationStructureGeometryTrianglesDataKHR triangles{
                .vertexFormat = ::vk::Format::eR32G32B32Sfloat,
                .vertexData = ::vk::DeviceOrHostAddressConstKHR{.deviceAddress = vertex_address + g.vertex_offset},
                .vertexStride = g.vertex_stride,
                .maxVertex = g.vertex_count - 1,
            };

            uint32_t primitive_count = g.vertex_count / 3;

            if (g.index_buffer) {
                auto* index_buffer = reinterpret_cast<Buffer*>(g.index_buffer);
                ::vk::DeviceAddress index_address = device->device->getBufferAddress(
                    ::vk::BufferDeviceAddressInfo{.buffer = index_buffer->buffer});

                triangles.indexType = ::vk::IndexType::eUint32;
                triangles.indexData = ::vk::DeviceOrHostAddressConstKHR{.deviceAddress = index_address + g.index_offset};
                primitive_count = g.index_count / 3;
            }

            geometries.push_back(::vk::AccelerationStructureGeometryKHR{
                .geometryType = ::vk::GeometryTypeKHR::eTriangles,
                .geometry = ::vk::AccelerationStructureGeometryDataKHR{.triangles = triangles},
                .flags = g.opaque ? ::vk::GeometryFlagBitsKHR::eOpaque : ::vk::GeometryFlagsKHR{},
            });

            primitive_counts.push_back(primitive_count);
        }
    }

    ::vk::AccelerationStructureTypeKHR type = desc->top_level
        ? ::vk::AccelerationStructureTypeKHR::eTopLevel
        : ::vk::AccelerationStructureTypeKHR::eBottomLevel;

    ::vk::AccelerationStructureBuildGeometryInfoKHR build_info{
        .type = type,
        .flags = desc->allow_update
            ? ::vk::BuildAccelerationStructureFlagBitsKHR::eAllowUpdate
            : ::vk::BuildAccelerationStructureFlagsKHR{},
        .mode = ::vk::BuildAccelerationStructureModeKHR::eBuild,
        .geometryCount = static_cast<uint32_t>(geometries.size()),
        .pGeometries = geometries.data(),
    };

    // Get size requirements
    auto sizes = device->device->getAccelerationStructureBuildSizesKHR(
        ::vk::AccelerationStructureBuildTypeKHR::eDevice,
        build_info,
        primitive_counts);

    // Create buffer for acceleration structure
    VkBufferCreateInfo buffer_info{
        .sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
        .size = sizes.accelerationStructureSize,
        .usage = VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT,
    };

    VmaAllocationCreateInfo alloc_info{
        .usage = VMA_MEMORY_USAGE_GPU_ONLY,
    };

    as->buffer = std::make_unique<Buffer>();
    as->buffer->device = as->device;
    as->buffer->size = sizes.accelerationStructureSize;

    VkBuffer vk_buffer;
    if (vmaCreateBuffer(device->allocator.get(), &buffer_info, &alloc_info,
                        &vk_buffer, &as->buffer->allocation, nullptr) != VK_SUCCESS) {
        return out_of_memory();
    }
    as->buffer->buffer = vk_buffer;

    // Create acceleration structure
    auto [result, vk_accel] = device->device->createAccelerationStructureKHRUnique(::vk::AccelerationStructureCreateInfoKHR{
        .buffer = as->buffer->buffer,
        .size = sizes.accelerationStructureSize,
        .type = type,
    });

    if (result != ::vk::Result::eSuccess) {
        vmaDestroyBuffer(device->allocator.get(), vk_buffer, as->buffer->allocation);
        return from_vk_result(result);
    }

    as->accel = std::move(vk_accel);

    VkAccelerationStructureDeviceAddressInfoKHR addr_info{
        .sType = VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR,
        .accelerationStructure = as->accel.get(),
    };
    as->device_address = vkGetAccelerationStructureDeviceAddressKHR(device->device.get(), &addr_info);

    // Create scratch buffer
    VkBufferCreateInfo scratch_info{
        .sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
        .size = sizes.buildScratchSize,
        .usage = VK_BUFFER_USAGE_STORAGE_BUFFER_BIT | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT,
    };

    as->scratch_buffer = std::make_unique<Buffer>();
    as->scratch_buffer->device = as->device;
    as->scratch_buffer->size = sizes.buildScratchSize;

    VkBuffer scratch_vk_buffer;
    if (vmaCreateBuffer(device->allocator.get(), &scratch_info, &alloc_info,
                        &scratch_vk_buffer, &as->scratch_buffer->allocation, nullptr) != VK_SUCCESS) {
        vmaDestroyBuffer(device->allocator.get(), vk_buffer, as->buffer->allocation);
        return out_of_memory();
    }
    as->scratch_buffer->buffer = scratch_vk_buffer;

    *out_as = reinterpret_cast<EPAccelerationStructurePtr>(as.release());
    return ok();
}

extern "C" EPError EPAccelerationStructureDestroy(EPAccelerationStructurePtr as_ptr) {
    if (as_ptr) {
        auto* as = reinterpret_cast<AccelerationStructure*>(as_ptr);

        // Clean up VMA buffers
        if (as->buffer && as->buffer->allocation) {
            vmaDestroyBuffer(as->device->allocator.get(),
                             static_cast<VkBuffer>(as->buffer->buffer),
                             as->buffer->allocation);
        }
        if (as->scratch_buffer && as->scratch_buffer->allocation) {
            vmaDestroyBuffer(as->device->allocator.get(),
                             static_cast<VkBuffer>(as->scratch_buffer->buffer),
                             as->scratch_buffer->allocation);
        }

        delete as;
    }
    return ok();
}
