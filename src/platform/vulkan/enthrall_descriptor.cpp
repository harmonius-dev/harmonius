// enthrall_descriptor.cpp - Descriptor set layout, descriptor set, and pipeline layout implementation

#include "enthrall_internal.h"

using namespace ep::vk;

// ============================================================================
// Descriptor Set Layout
// ============================================================================

extern "C" EPError EPDescriptorSetLayoutCreate(EPDevicePtr device_ptr, const EPDescriptorSetLayoutDesc* desc,
                                                EPDescriptorSetLayoutPtr* out_layout) {
    if (!device_ptr || !desc || !out_layout) return invalid_argument("device, desc, or out_layout is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    auto layout = std::make_unique<DescriptorSetLayout>();
    layout->device = std::shared_ptr<Device>(device, [](Device*){});

    std::vector<::vk::DescriptorSetLayoutBinding> vk_bindings;
    vk_bindings.reserve(desc->binding_count);

    for (uint32_t i = 0; i < desc->binding_count; i++) {
        const auto& b = desc->bindings[i];

        vk_bindings.push_back(::vk::DescriptorSetLayoutBinding{
            .binding = b.binding,
            .descriptorType = to_vk_descriptor_type(b.type),
            .descriptorCount = b.count > 0 ? b.count : 1,
            .stageFlags = to_vk_shader_stage(b.stages),
        });

        layout->bindings.push_back(DescriptorBindingInfo{
            .binding = b.binding,
            .type = b.type,
            .count = b.count,
            .stages = b.stages,
        });
    }

    auto [result, vk_layout] = device->device->createDescriptorSetLayoutUnique(::vk::DescriptorSetLayoutCreateInfo{
        .bindingCount = static_cast<uint32_t>(vk_bindings.size()),
        .pBindings = vk_bindings.data(),
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    layout->layout = std::move(vk_layout);
    *out_layout = reinterpret_cast<EPDescriptorSetLayoutPtr>(layout.release());
    return ok();
}

extern "C" EPError EPDescriptorSetLayoutDestroy(EPDescriptorSetLayoutPtr layout) {
    delete reinterpret_cast<DescriptorSetLayout*>(layout);
    return ok();
}

// ============================================================================
// Descriptor Set
// ============================================================================

extern "C" EPError EPDescriptorSetCreate(EPDevicePtr device_ptr, EPDescriptorSetLayoutPtr layout_ptr,
                                          EPDescriptorSetPtr* out_set) {
    if (!device_ptr || !layout_ptr || !out_set) return invalid_argument("device, layout, or out_set is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);
    auto* layout = reinterpret_cast<DescriptorSetLayout*>(layout_ptr);

    auto set = std::make_unique<DescriptorSet>();
    set->device = std::shared_ptr<Device>(device, [](Device*){});
    set->layout = std::shared_ptr<DescriptorSetLayout>(layout, [](DescriptorSetLayout*){});

    ::vk::DescriptorSetLayout layouts[] = {layout->layout.get()};
    auto [result, sets] = device->device->allocateDescriptorSets(::vk::DescriptorSetAllocateInfo{
        .descriptorPool = device->descriptor_pool.get(),
        .descriptorSetCount = 1,
        .pSetLayouts = layouts,
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    set->set = sets[0];
    *out_set = reinterpret_cast<EPDescriptorSetPtr>(set.release());
    return ok();
}

extern "C" EPError EPDescriptorSetDestroy(EPDescriptorSetPtr set) {
    if (set) {
        auto* s = reinterpret_cast<DescriptorSet*>(set);
        // Free from pool
        s->device->device->freeDescriptorSets(s->device->descriptor_pool.get(), s->set);
        delete s;
    }
    return ok();
}

extern "C" EPError EPDescriptorSetUpdateBuffer(EPDescriptorSetPtr set_ptr, uint32_t binding,
                                                EPBufferPtr buffer_ptr, uint64_t offset, uint64_t range) {
    if (!set_ptr || !buffer_ptr) return invalid_argument("set or buffer is NULL");

    auto* set = reinterpret_cast<DescriptorSet*>(set_ptr);
    auto* buffer = reinterpret_cast<Buffer*>(buffer_ptr);

    // Find the binding info to get the descriptor type
    ::vk::DescriptorType desc_type = ::vk::DescriptorType::eUniformBuffer;
    for (const auto& b : set->layout->bindings) {
        if (b.binding == binding) {
            desc_type = to_vk_descriptor_type(b.type);
            break;
        }
    }

    ::vk::DescriptorBufferInfo buffer_info{
        .buffer = buffer->buffer,
        .offset = offset,
        .range = range == 0 ? VK_WHOLE_SIZE : range,
    };

    set->device->device->updateDescriptorSets(::vk::WriteDescriptorSet{
        .dstSet = set->set,
        .dstBinding = binding,
        .dstArrayElement = 0,
        .descriptorCount = 1,
        .descriptorType = desc_type,
        .pBufferInfo = &buffer_info,
    }, {});

    return ok();
}

extern "C" EPError EPDescriptorSetUpdateTexture(EPDescriptorSetPtr set_ptr, uint32_t binding,
                                                 EPTexturePtr texture_ptr) {
    if (!set_ptr || !texture_ptr) return invalid_argument("set or texture is NULL");

    auto* set = reinterpret_cast<DescriptorSet*>(set_ptr);
    auto* texture = reinterpret_cast<Texture*>(texture_ptr);

    // Find the binding info to get the descriptor type
    ::vk::DescriptorType desc_type = ::vk::DescriptorType::eSampledImage;
    for (const auto& b : set->layout->bindings) {
        if (b.binding == binding) {
            desc_type = to_vk_descriptor_type(b.type);
            break;
        }
    }

    ::vk::DescriptorImageInfo image_info{
        .imageView = texture->view.get(),
        .imageLayout = desc_type == ::vk::DescriptorType::eStorageImage
            ? ::vk::ImageLayout::eGeneral
            : ::vk::ImageLayout::eShaderReadOnlyOptimal,
    };

    set->device->device->updateDescriptorSets(::vk::WriteDescriptorSet{
        .dstSet = set->set,
        .dstBinding = binding,
        .dstArrayElement = 0,
        .descriptorCount = 1,
        .descriptorType = desc_type,
        .pImageInfo = &image_info,
    }, {});

    return ok();
}

extern "C" EPError EPDescriptorSetUpdateSampler(EPDescriptorSetPtr set_ptr, uint32_t binding,
                                                 EPSamplerPtr sampler_ptr) {
    if (!set_ptr || !sampler_ptr) return invalid_argument("set or sampler is NULL");

    auto* set = reinterpret_cast<DescriptorSet*>(set_ptr);
    auto* sampler = reinterpret_cast<Sampler*>(sampler_ptr);

    ::vk::DescriptorImageInfo image_info{
        .sampler = sampler->sampler.get(),
    };

    set->device->device->updateDescriptorSets(::vk::WriteDescriptorSet{
        .dstSet = set->set,
        .dstBinding = binding,
        .dstArrayElement = 0,
        .descriptorCount = 1,
        .descriptorType = ::vk::DescriptorType::eSampler,
        .pImageInfo = &image_info,
    }, {});

    return ok();
}

extern "C" EPError EPDescriptorSetUpdateAccelerationStructure(EPDescriptorSetPtr set_ptr, uint32_t binding,
                                                               EPAccelerationStructurePtr as_ptr) {
    if (!set_ptr || !as_ptr) return invalid_argument("set or acceleration structure is NULL");

    auto* set = reinterpret_cast<DescriptorSet*>(set_ptr);
    auto* as = reinterpret_cast<AccelerationStructure*>(as_ptr);

    ::vk::AccelerationStructureKHR accel = as->accel.get();
    ::vk::WriteDescriptorSetAccelerationStructureKHR accel_info{
        .accelerationStructureCount = 1,
        .pAccelerationStructures = &accel,
    };

    ::vk::WriteDescriptorSet write{
        .pNext = &accel_info,
        .dstSet = set->set,
        .dstBinding = binding,
        .dstArrayElement = 0,
        .descriptorCount = 1,
        .descriptorType = ::vk::DescriptorType::eAccelerationStructureKHR,
    };

    set->device->device->updateDescriptorSets(write, {});
    return ok();
}

// ============================================================================
// Pipeline Layout
// ============================================================================

extern "C" EPError EPPipelineLayoutCreate(EPDevicePtr device_ptr, const EPPipelineLayoutDesc* desc,
                                           EPPipelineLayoutPtr* out_layout) {
    if (!device_ptr || !desc || !out_layout) return invalid_argument("device, desc, or out_layout is NULL");

    auto* device = reinterpret_cast<Device*>(device_ptr);

    auto layout = std::make_unique<PipelineLayout>();
    layout->device = std::shared_ptr<Device>(device, [](Device*){});
    layout->push_constant_size = desc->push_constant_size;
    layout->push_constant_stages = desc->push_constant_stages;

    std::vector<::vk::DescriptorSetLayout> vk_layouts;
    for (uint32_t i = 0; i < desc->set_layout_count; i++) {
        auto* set_layout = reinterpret_cast<DescriptorSetLayout*>(desc->set_layouts[i]);
        vk_layouts.push_back(set_layout->layout.get());
        layout->set_layouts.push_back(std::shared_ptr<DescriptorSetLayout>(set_layout, [](DescriptorSetLayout*){}));
    }

    ::vk::PushConstantRange push_range;
    uint32_t push_range_count = 0;
    if (desc->push_constant_size > 0) {
        push_range = ::vk::PushConstantRange{
            .stageFlags = to_vk_shader_stage(desc->push_constant_stages),
            .offset = 0,
            .size = desc->push_constant_size,
        };
        push_range_count = 1;
    }

    auto [result, vk_layout] = device->device->createPipelineLayoutUnique(::vk::PipelineLayoutCreateInfo{
        .setLayoutCount = static_cast<uint32_t>(vk_layouts.size()),
        .pSetLayouts = vk_layouts.data(),
        .pushConstantRangeCount = push_range_count,
        .pPushConstantRanges = push_range_count > 0 ? &push_range : nullptr,
    });

    if (result != ::vk::Result::eSuccess) {
        return from_vk_result(result);
    }

    layout->layout = std::move(vk_layout);
    *out_layout = reinterpret_cast<EPPipelineLayoutPtr>(layout.release());
    return ok();
}

extern "C" EPError EPPipelineLayoutDestroy(EPPipelineLayoutPtr layout) {
    delete reinterpret_cast<PipelineLayout*>(layout);
    return ok();
}
