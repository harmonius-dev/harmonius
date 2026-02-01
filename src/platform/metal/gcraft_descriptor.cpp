// gcraft_descriptor.cpp - Descriptor set layout, descriptor set, and pipeline layout (Metal-cpp)

#include "gcraft_internal.h"

extern "C" {

GCError GCDescriptorSetLayoutCreate(GCDevicePtr device, const GCDescriptorSetLayoutDesc* desc,
                                    GCDescriptorSetLayoutPtr* out_layout) {
  if (!device || !desc || !out_layout) return ep_invalid_argument("device, desc, or out_layout is NULL");

  GCDescriptorSetLayout* layout = static_cast<GCDescriptorSetLayout*>(calloc(1, sizeof(GCDescriptorSetLayout)));
  if (!layout) return ep_out_of_memory();

  layout->ep_device = device;
  layout->binding_count = desc->binding_count;

  if (desc->binding_count > 0) {
    layout->bindings = static_cast<GCDescriptorBindingInfo*>(calloc(desc->binding_count, sizeof(GCDescriptorBindingInfo)));
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
    GCDescriptorSetLayout* lay = static_cast<GCDescriptorSetLayout*>(layout);
    if (lay->bindings) free(lay->bindings);
    free(layout);
  }
  return ep_ok();
}

GCError GCDescriptorSetCreate(GCDevicePtr device, GCDescriptorSetLayoutPtr layout,
                              GCDescriptorSetPtr* out_set) {
  if (!device || !layout || !out_set) return ep_invalid_argument("device, layout, or out_set is NULL");

  GCDescriptorSetLayout* lay = static_cast<GCDescriptorSetLayout*>(layout);
  GCDescriptorSet* set = static_cast<GCDescriptorSet*>(calloc(1, sizeof(GCDescriptorSet)));
  if (!set) return ep_out_of_memory();

  set->ep_device = device;
  set->layout = lay;
  set->entry_count = lay->binding_count;

  if (lay->binding_count > 0) {
    set->entries = static_cast<GCDescriptorSetEntry*>(calloc(lay->binding_count, sizeof(GCDescriptorSetEntry)));
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
    GCDescriptorSet* s = static_cast<GCDescriptorSet*>(set);
    if (s->entries) free(s->entries);
    free(set);
  }
  return ep_ok();
}

GCError GCDescriptorSetUpdateBuffer(GCDescriptorSetPtr set, uint32_t binding,
                                    GCBufferPtr buffer, uint64_t offset, uint64_t range) {
  if (!set || !buffer) return ep_invalid_argument("set or buffer is NULL");
  GCDescriptorSet* s = static_cast<GCDescriptorSet*>(set);
  GCBuffer* buf = static_cast<GCBuffer*>(buffer);

  for (uint32_t i = 0; i < s->entry_count; i++) {
    if (s->layout->bindings[i].binding == binding) {
      s->entries[i].type = s->layout->bindings[i].type;
      s->entries[i].value.buffer_info.buffer = buf->buffer;
      s->entries[i].value.buffer_info.offset = offset;
      s->entries[i].value.buffer_info.range = range;
      return ep_ok();
    }
  }
  return ep_invalid_argument("binding not found");
}

GCError GCDescriptorSetUpdateTexture(GCDescriptorSetPtr set, uint32_t binding, GCTexturePtr texture) {
  if (!set || !texture) return ep_invalid_argument("set or texture is NULL");
  GCDescriptorSet* s = static_cast<GCDescriptorSet*>(set);
  GCTexture* tex = static_cast<GCTexture*>(texture);

  for (uint32_t i = 0; i < s->entry_count; i++) {
    if (s->layout->bindings[i].binding == binding) {
      s->entries[i].type = s->layout->bindings[i].type;
      s->entries[i].value.texture = tex->texture;
      return ep_ok();
    }
  }
  return ep_invalid_argument("binding not found");
}

GCError GCDescriptorSetUpdateSampler(GCDescriptorSetPtr set, uint32_t binding, GCSamplerPtr sampler) {
  if (!set || !sampler) return ep_invalid_argument("set or sampler is NULL");
  GCDescriptorSet* s = static_cast<GCDescriptorSet*>(set);
  GCSampler* samp = static_cast<GCSampler*>(sampler);

  for (uint32_t i = 0; i < s->entry_count; i++) {
    if (s->layout->bindings[i].binding == binding) {
      s->entries[i].type = GC_DESCRIPTOR_SAMPLER;
      s->entries[i].value.sampler = samp->sampler;
      return ep_ok();
    }
  }
  return ep_invalid_argument("binding not found");
}

GCError GCDescriptorSetUpdateAccelerationStructure(GCDescriptorSetPtr set, uint32_t binding,
                                                   GCAccelerationStructurePtr as) {
  if (!set || !as) return ep_invalid_argument("set or acceleration structure is NULL");
  GCDescriptorSet* s = static_cast<GCDescriptorSet*>(set);
  GCAccelerationStructure* accel = static_cast<GCAccelerationStructure*>(as);

  for (uint32_t i = 0; i < s->entry_count; i++) {
    if (s->layout->bindings[i].binding == binding) {
      s->entries[i].type = GC_DESCRIPTOR_ACCELERATION_STRUCTURE;
      s->entries[i].value.accel = accel->accel;
      return ep_ok();
    }
  }
  return ep_invalid_argument("binding not found");
}

GCError GCPipelineLayoutCreate(GCDevicePtr device, const GCPipelineLayoutDesc* desc,
                               GCPipelineLayoutPtr* out_layout) {
  if (!device || !desc || !out_layout) return ep_invalid_argument("device, desc, or out_layout is NULL");

  GCPipelineLayout* layout = static_cast<GCPipelineLayout*>(calloc(1, sizeof(GCPipelineLayout)));
  if (!layout) return ep_out_of_memory();

  layout->ep_device = device;
  layout->set_layout_count = desc->set_layout_count;
  layout->push_constant_size = desc->push_constant_size;
  layout->push_constant_stages = desc->push_constant_stages;

  if (desc->set_layout_count > 0) {
    layout->set_layouts = static_cast<GCDescriptorSetLayout**>(calloc(desc->set_layout_count, sizeof(GCDescriptorSetLayout*)));
    if (!layout->set_layouts) {
      free(layout);
      return ep_out_of_memory();
    }

    for (uint32_t i = 0; i < desc->set_layout_count; i++) {
      layout->set_layouts[i] = static_cast<GCDescriptorSetLayout*>(const_cast<void*>(static_cast<const void*>(desc->set_layouts[i])));
    }
  }

  *out_layout = layout;
  return ep_ok();
}

GCError GCPipelineLayoutDestroy(GCPipelineLayoutPtr layout) {
  if (layout) {
    GCPipelineLayout* lay = static_cast<GCPipelineLayout*>(layout);
    if (lay->set_layouts) free(lay->set_layouts);
    free(layout);
  }
  return ep_ok();
}

}
