// gcraft_descriptor.m - Descriptor set layout, descriptor set, and pipeline layout implementation

#include "gcraft_internal.h"

#pragma mark - Descriptor Set Layout

GCError GCDescriptorSetLayoutCreate(GCDevicePtr device, const GCDescriptorSetLayoutDesc *desc,
                                    GCDescriptorSetLayoutPtr *out_layout) {
    if (!device || !desc || !out_layout) return ep_invalid_argument("device, desc, or out_layout is NULL");

    GCDescriptorSetLayout *layout = calloc(1, sizeof(GCDescriptorSetLayout));
    if (!layout) return ep_out_of_memory();

    layout->ep_device = device;
    layout->binding_count = desc->binding_count;

    if (desc->binding_count > 0) {
        layout->bindings = calloc(desc->binding_count, sizeof(GCDescriptorBindingInfo));
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

GCError GCDescriptorSetLayoutDestroy(GCDescriptorSetLayoutPtr layout) {
    if (layout) {
        if (layout->bindings) free(layout->bindings);
        free(layout);
    }
    return ep_ok();
}

#pragma mark - Descriptor Set

GCError GCDescriptorSetCreate(GCDevicePtr device, GCDescriptorSetLayoutPtr layout,
                              GCDescriptorSetPtr *out_set) {
    if (!device || !layout || !out_set) return ep_invalid_argument("device, layout, or out_set is NULL");

    GCDescriptorSet *set = calloc(1, sizeof(GCDescriptorSet));
    if (!set) return ep_out_of_memory();

    set->ep_device = device;
    set->layout = layout;
    set->entry_count = layout->binding_count;

    if (layout->binding_count > 0) {
        set->entries = calloc(layout->binding_count, sizeof(GCDescriptorSetEntry));
        if (!set->entries) {
            free(set);
            return ep_out_of_memory();
        }
    }

    *out_set = set;
    return ep_ok();
}

GCError GCDescriptorSetDestroy(GCDescriptorSetPtr set) {
    if (set) {
        if (set->entries) free(set->entries);
        free(set);
    }
    return ep_ok();
}

GCError GCDescriptorSetUpdateBuffer(GCDescriptorSetPtr set, uint32_t binding,
                                    GCBufferPtr buffer, uint64_t offset, uint64_t range) {
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

GCError GCDescriptorSetUpdateTexture(GCDescriptorSetPtr set, uint32_t binding, GCTexturePtr texture) {
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

GCError GCDescriptorSetUpdateSampler(GCDescriptorSetPtr set, uint32_t binding, GCSamplerPtr sampler) {
    if (!set || !sampler) return ep_invalid_argument("set or sampler is NULL");

    for (uint32_t i = 0; i < set->entry_count; i++) {
        if (set->layout->bindings[i].binding == binding) {
            set->entries[i].type = GC_DESCRIPTOR_SAMPLER;
            set->entries[i].value.sampler = sampler->sampler;
            return ep_ok();
        }
    }

    return ep_invalid_argument("binding not found");
}

GCError GCDescriptorSetUpdateAccelerationStructure(GCDescriptorSetPtr set, uint32_t binding,
                                                   GCAccelerationStructurePtr as) {
    if (!set || !as) return ep_invalid_argument("set or acceleration structure is NULL");

    for (uint32_t i = 0; i < set->entry_count; i++) {
        if (set->layout->bindings[i].binding == binding) {
            set->entries[i].type = GC_DESCRIPTOR_ACCELERATION_STRUCTURE;
            set->entries[i].value.accel = as->accel;
            return ep_ok();
        }
    }

    return ep_invalid_argument("binding not found");
}

#pragma mark - Pipeline Layout

GCError GCPipelineLayoutCreate(GCDevicePtr device, const GCPipelineLayoutDesc *desc,
                               GCPipelineLayoutPtr *out_layout) {
    if (!device || !desc || !out_layout) return ep_invalid_argument("device, desc, or out_layout is NULL");

    GCPipelineLayout *layout = calloc(1, sizeof(GCPipelineLayout));
    if (!layout) return ep_out_of_memory();

    layout->ep_device = device;
    layout->set_layout_count = desc->set_layout_count;
    layout->push_constant_size = desc->push_constant_size;
    layout->push_constant_stages = desc->push_constant_stages;

    if (desc->set_layout_count > 0) {
        layout->set_layouts = calloc(desc->set_layout_count, sizeof(GCDescriptorSetLayout *));
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

GCError GCPipelineLayoutDestroy(GCPipelineLayoutPtr layout) {
    if (layout) {
        if (layout->set_layouts) free(layout->set_layouts);
        free(layout);
    }
    return ep_ok();
}
