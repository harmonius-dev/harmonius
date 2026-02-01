// enthrall_descriptor.m - Descriptor set layout, descriptor set, and pipeline layout implementation

#include "enthrall_internal.h"

#pragma mark - Descriptor Set Layout

EPError EPDescriptorSetLayoutCreate(EPDevicePtr device, const EPDescriptorSetLayoutDesc *desc,
                                    EPDescriptorSetLayoutPtr *out_layout) {
    if (!device || !desc || !out_layout) return ep_invalid_argument("device, desc, or out_layout is NULL");

    EPDescriptorSetLayout *layout = calloc(1, sizeof(EPDescriptorSetLayout));
    if (!layout) return ep_out_of_memory();

    layout->ep_device = device;
    layout->binding_count = desc->binding_count;

    if (desc->binding_count > 0) {
        layout->bindings = calloc(desc->binding_count, sizeof(EPDescriptorBindingInfo));
        if (!layout->bindings) {
            free(layout);
            return ep_out_of_memory();
        }

        for (uint32_t i = 0; i < desc->binding_count; i++) {
            layout->bindings[i].binding = desc->bindings[i].binding;
            layout->bindings[i].type = desc->bindings[i].type;
            layout->bindings[i].count = desc->bindings[i].count;
            layout->bindings[i].stages = desc->bindings[i].stages;
        }
    }

    *out_layout = layout;
    return ep_ok();
}

EPError EPDescriptorSetLayoutDestroy(EPDescriptorSetLayoutPtr layout) {
    if (layout) {
        if (layout->bindings) free(layout->bindings);
        free(layout);
    }
    return ep_ok();
}

#pragma mark - Descriptor Set

EPError EPDescriptorSetCreate(EPDevicePtr device, EPDescriptorSetLayoutPtr layout,
                              EPDescriptorSetPtr *out_set) {
    if (!device || !layout || !out_set) return ep_invalid_argument("device, layout, or out_set is NULL");

    EPDescriptorSet *set = calloc(1, sizeof(EPDescriptorSet));
    if (!set) return ep_out_of_memory();

    set->ep_device = device;
    set->layout = layout;
    set->entry_count = layout->binding_count;

    if (layout->binding_count > 0) {
        set->entries = calloc(layout->binding_count, sizeof(EPDescriptorSetEntry));
        if (!set->entries) {
            free(set);
            return ep_out_of_memory();
        }
    }

    *out_set = set;
    return ep_ok();
}

EPError EPDescriptorSetDestroy(EPDescriptorSetPtr set) {
    if (set) {
        if (set->entries) free(set->entries);
        free(set);
    }
    return ep_ok();
}

EPError EPDescriptorSetUpdateBuffer(EPDescriptorSetPtr set, uint32_t binding,
                                    EPBufferPtr buffer, uint64_t offset, uint64_t range) {
    if (!set || !buffer) return ep_invalid_argument("set or buffer is NULL");

    for (uint32_t i = 0; i < set->entry_count; i++) {
        if (set->layout->bindings[i].binding == binding) {
            set->entries[i].type = set->layout->bindings[i].type;
            set->entries[i].value.buffer_info.buffer = buffer->buffer;
            set->entries[i].value.buffer_info.offset = offset;
            set->entries[i].value.buffer_info.range = range;
            return ep_ok();
        }
    }

    return ep_invalid_argument("binding not found");
}

EPError EPDescriptorSetUpdateTexture(EPDescriptorSetPtr set, uint32_t binding, EPTexturePtr texture) {
    if (!set || !texture) return ep_invalid_argument("set or texture is NULL");

    for (uint32_t i = 0; i < set->entry_count; i++) {
        if (set->layout->bindings[i].binding == binding) {
            set->entries[i].type = set->layout->bindings[i].type;
            set->entries[i].value.texture = texture->texture;
            return ep_ok();
        }
    }

    return ep_invalid_argument("binding not found");
}

EPError EPDescriptorSetUpdateSampler(EPDescriptorSetPtr set, uint32_t binding, EPSamplerPtr sampler) {
    if (!set || !sampler) return ep_invalid_argument("set or sampler is NULL");

    for (uint32_t i = 0; i < set->entry_count; i++) {
        if (set->layout->bindings[i].binding == binding) {
            set->entries[i].type = EP_DESCRIPTOR_SAMPLER;
            set->entries[i].value.sampler = sampler->sampler;
            return ep_ok();
        }
    }

    return ep_invalid_argument("binding not found");
}

EPError EPDescriptorSetUpdateAccelerationStructure(EPDescriptorSetPtr set, uint32_t binding,
                                                   EPAccelerationStructurePtr as) {
    if (!set || !as) return ep_invalid_argument("set or acceleration structure is NULL");

    for (uint32_t i = 0; i < set->entry_count; i++) {
        if (set->layout->bindings[i].binding == binding) {
            set->entries[i].type = EP_DESCRIPTOR_ACCELERATION_STRUCTURE;
            set->entries[i].value.accel = as->accel;
            return ep_ok();
        }
    }

    return ep_invalid_argument("binding not found");
}

#pragma mark - Pipeline Layout

EPError EPPipelineLayoutCreate(EPDevicePtr device, const EPPipelineLayoutDesc *desc,
                               EPPipelineLayoutPtr *out_layout) {
    if (!device || !desc || !out_layout) return ep_invalid_argument("device, desc, or out_layout is NULL");

    EPPipelineLayout *layout = calloc(1, sizeof(EPPipelineLayout));
    if (!layout) return ep_out_of_memory();

    layout->ep_device = device;
    layout->set_layout_count = desc->set_layout_count;
    layout->push_constant_size = desc->push_constant_size;
    layout->push_constant_stages = desc->push_constant_stages;

    if (desc->set_layout_count > 0) {
        layout->set_layouts = calloc(desc->set_layout_count, sizeof(EPDescriptorSetLayout *));
        if (!layout->set_layouts) {
            free(layout);
            return ep_out_of_memory();
        }

        for (uint32_t i = 0; i < desc->set_layout_count; i++) {
            layout->set_layouts[i] = desc->set_layouts[i];
        }
    }

    *out_layout = layout;
    return ep_ok();
}

EPError EPPipelineLayoutDestroy(EPPipelineLayoutPtr layout) {
    if (layout) {
        if (layout->set_layouts) free(layout->set_layouts);
        free(layout);
    }
    return ep_ok();
}
