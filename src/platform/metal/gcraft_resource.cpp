// gcraft_resource.cpp - Buffer, texture, sampler, and shader library (Metal-cpp)

#include "gcraft_internal.h"
#include <string>

extern "C" {

GCError GCBufferCreate(GCDevicePtr device, const GCBufferDesc* desc, GCBufferPtr* out_buffer) {
  if (!device || !desc || !out_buffer) return ep_invalid_argument("device, desc, or out_buffer is NULL");

  GCDevice* dev = static_cast<GCDevice*>(device);
  GCBuffer* buffer = static_cast<GCBuffer*>(calloc(1, sizeof(GCBuffer)));
  if (!buffer) return ep_out_of_memory();

  buffer->ep_device = dev;
  buffer->size = desc->size;
  buffer->usage = desc->usage;
  buffer->host_visible = desc->host_visible;

  MTL::ResourceOptions options = MTL::ResourceStorageModePrivate;
  if (desc->host_visible) {
    options = MTL::ResourceStorageModeShared;
  }

  buffer->buffer = dev->device->newBuffer(desc->size, options);
  if (!buffer->buffer) {
    free(buffer);
    return ep_out_of_memory();
  }

  *out_buffer = buffer;
  return ep_ok();
}

GCError GCBufferDestroy(GCBufferPtr buffer) {
  if (buffer) {
    GCBuffer* buf = static_cast<GCBuffer*>(buffer);
    if (buf->buffer) buf->buffer->release();
    free(buffer);
  }
  return ep_ok();
}

GCError GCBufferMap(GCBufferPtr buffer, void** out_data) {
  if (!buffer || !out_data) return ep_invalid_argument("buffer or out_data is NULL");
  GCBuffer* buf = static_cast<GCBuffer*>(buffer);
  if (!buf->host_visible) return ep_unsupported("buffer is not host-visible");
  *out_data = buf->buffer->contents();
  return ep_ok();
}

GCError GCBufferUnmap(GCBufferPtr buffer) {
  if (!buffer) return ep_invalid_argument("buffer is NULL");
  (void)buffer;
  return ep_ok();
}

GCError GCTextureCreate(GCDevicePtr device, const GCTextureDesc* desc, GCTexturePtr* out_texture) {
  if (!device || !desc || !out_texture) return ep_invalid_argument("device, desc, or out_texture is NULL");

  GCDevice* dev = static_cast<GCDevice*>(device);
  GCTexture* texture = static_cast<GCTexture*>(calloc(1, sizeof(GCTexture)));
  if (!texture) return ep_out_of_memory();

  texture->ep_device = dev;
  texture->desc = *desc;

  MTL::TextureDescriptor* mtl_desc = MTL::TextureDescriptor::alloc()->init();
  mtl_desc->setTextureType(ep_to_mtl_texture_type(desc->dimension, desc->array_layers));
  mtl_desc->setPixelFormat(ep_to_mtl_pixel_format(desc->format));
  mtl_desc->setWidth(desc->width);
  mtl_desc->setHeight(desc->height);
  mtl_desc->setDepth(desc->depth > 0 ? desc->depth : 1);
  mtl_desc->setMipmapLevelCount(desc->mip_levels > 0 ? desc->mip_levels : 1);
  mtl_desc->setArrayLength(desc->array_layers > 0 ? desc->array_layers : 1);
  mtl_desc->setUsage(ep_to_mtl_texture_usage(desc->usage));
  mtl_desc->setStorageMode(MTL::StorageModePrivate);

  texture->texture = dev->device->newTexture(mtl_desc);
  mtl_desc->release();

  if (!texture->texture) {
    free(texture);
    return ep_out_of_memory();
  }

  *out_texture = texture;
  return ep_ok();
}

GCError GCTextureDestroy(GCTexturePtr texture) {
  if (texture) {
    GCTexture* tex = static_cast<GCTexture*>(texture);
    if (tex->texture) tex->texture->release();
    free(texture);
  }
  return ep_ok();
}

GCError GCSamplerCreate(GCDevicePtr device, const GCSamplerDesc* desc, GCSamplerPtr* out_sampler) {
  if (!device || !desc || !out_sampler) return ep_invalid_argument("device, desc, or out_sampler is NULL");

  GCDevice* dev = static_cast<GCDevice*>(device);
  GCSampler* sampler = static_cast<GCSampler*>(calloc(1, sizeof(GCSampler)));
  if (!sampler) return ep_out_of_memory();

  sampler->ep_device = dev;

  MTL::SamplerDescriptor* mtl_desc = MTL::SamplerDescriptor::alloc()->init();
  mtl_desc->setMinFilter(ep_to_mtl_filter(desc->min_filter));
  mtl_desc->setMagFilter(ep_to_mtl_filter(desc->mag_filter));
  mtl_desc->setSAddressMode(ep_to_mtl_address(desc->address_u));
  mtl_desc->setTAddressMode(ep_to_mtl_address(desc->address_v));
  mtl_desc->setRAddressMode(ep_to_mtl_address(desc->address_w));
  mtl_desc->setCompareFunction(ep_to_mtl_compare(desc->compare_op));
  mtl_desc->setMaxAnisotropy(static_cast<NS::UInteger>(desc->max_anisotropy));

  sampler->sampler = dev->device->newSamplerState(mtl_desc);
  mtl_desc->release();

  if (!sampler->sampler) {
    free(sampler);
    return ep_out_of_memory();
  }

  *out_sampler = sampler;
  return ep_ok();
}

GCError GCSamplerDestroy(GCSamplerPtr sampler) {
  if (sampler) {
    GCSampler* samp = static_cast<GCSampler*>(sampler);
    if (samp->sampler) samp->sampler->release();
    free(sampler);
  }
  return ep_ok();
}

GCError GCShaderLibraryCreate(GCDevicePtr device, const GCShaderLibraryDesc* desc,
                              GCShaderLibraryPtr* out_library) {
  if (!device || !desc || !out_library) return ep_invalid_argument("device, desc, or out_library is NULL");
  if (desc->format != GC_SHADER_MSL) return ep_unsupported("only MSL shaders supported on Metal");

  GCDevice* dev = static_cast<GCDevice*>(device);
  GCShaderLibrary* library = static_cast<GCShaderLibrary*>(calloc(1, sizeof(GCShaderLibrary)));
  if (!library) return ep_out_of_memory();

  library->ep_device = dev;

  std::string src(reinterpret_cast<const char*>(desc->data), static_cast<size_t>(desc->size));
  NS::String* source = NS::String::string(src.c_str(), NS::UTF8StringEncoding);
  MTL::CompileOptions* options = MTL::CompileOptions::alloc()->init();
  options->setLanguageVersion(MTL::LanguageVersion3_0);

  NS::Error* error = nullptr;
  library->library = dev->device->newLibrary(source, options, &error);
  options->release();

  if (!library->library) {
    const char* msg = error && error->localizedDescription() ? error->localizedDescription()->utf8String() : "shader compilation failed";
    free(library);
    return ep_error(GC_E_INVALID_ARGUMENT, msg);
  }

  if (desc->label && dev->debug_names_enabled) {
    library->library->setLabel(NS::String::string(desc->label, NS::UTF8StringEncoding));
  }

  *out_library = library;
  return ep_ok();
}

GCError GCShaderLibraryDestroy(GCShaderLibraryPtr library) {
  if (library) {
    GCShaderLibrary* lib = static_cast<GCShaderLibrary*>(library);
    if (lib->library) lib->library->release();
    free(library);
  }
  return ep_ok();
}

}
