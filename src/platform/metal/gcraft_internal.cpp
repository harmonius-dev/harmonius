// gcraft_internal.cpp - Format conversion utilities for Metal backend

#include "gcraft_internal.h"

MTL::PixelFormat ep_to_mtl_pixel_format(GCTextureFormat format) {
  switch (format) {
  case GC_FORMAT_RGBA8_UNORM:
    return MTL::PixelFormatRGBA8Unorm;
  case GC_FORMAT_BGRA8_UNORM:
    return MTL::PixelFormatBGRA8Unorm;
  case GC_FORMAT_RGBA16_FLOAT:
    return MTL::PixelFormatRGBA16Float;
  case GC_FORMAT_RGBA32_FLOAT:
    return MTL::PixelFormatRGBA32Float;
  case GC_FORMAT_D24S8:
    return MTL::PixelFormatDepth24Unorm_Stencil8;
  case GC_FORMAT_D32_FLOAT:
    return MTL::PixelFormatDepth32Float;
  default:
    return MTL::PixelFormatInvalid;
  }
}

MTL::SamplerMinMagFilter ep_to_mtl_filter(GCFilter filter) {
  switch (filter) {
  case GC_FILTER_NEAREST:
    return MTL::SamplerMinMagFilterNearest;
  case GC_FILTER_LINEAR:
    return MTL::SamplerMinMagFilterLinear;
  default:
    return MTL::SamplerMinMagFilterNearest;
  }
}

MTL::SamplerAddressMode ep_to_mtl_address(GCAddressMode mode) {
  switch (mode) {
  case GC_ADDRESS_CLAMP_TO_EDGE:
    return MTL::SamplerAddressModeClampToEdge;
  case GC_ADDRESS_REPEAT:
    return MTL::SamplerAddressModeRepeat;
  case GC_ADDRESS_MIRROR_REPEAT:
    return MTL::SamplerAddressModeMirrorRepeat;
  default:
    return MTL::SamplerAddressModeClampToEdge;
  }
}

MTL::CompareFunction ep_to_mtl_compare(GCCompareOp op) {
  switch (op) {
  case GC_COMPARE_NEVER:
    return MTL::CompareFunctionNever;
  case GC_COMPARE_LESS:
    return MTL::CompareFunctionLess;
  case GC_COMPARE_EQUAL:
    return MTL::CompareFunctionEqual;
  case GC_COMPARE_LESS_EQUAL:
    return MTL::CompareFunctionLessEqual;
  case GC_COMPARE_GREATER:
    return MTL::CompareFunctionGreater;
  case GC_COMPARE_NOT_EQUAL:
    return MTL::CompareFunctionNotEqual;
  case GC_COMPARE_GREATER_EQUAL:
    return MTL::CompareFunctionGreaterEqual;
  case GC_COMPARE_ALWAYS:
    return MTL::CompareFunctionAlways;
  default:
    return MTL::CompareFunctionAlways;
  }
}

MTL::LoadAction ep_to_mtl_load_action(GCAttachmentLoadOp op) {
  switch (op) {
  case GC_LOAD_OP_LOAD:
    return MTL::LoadActionLoad;
  case GC_LOAD_OP_CLEAR:
    return MTL::LoadActionClear;
  case GC_LOAD_OP_DONT_CARE:
    return MTL::LoadActionDontCare;
  default:
    return MTL::LoadActionDontCare;
  }
}

MTL::StoreAction ep_to_mtl_store_action(GCAttachmentStoreOp op) {
  switch (op) {
  case GC_STORE_OP_STORE:
    return MTL::StoreActionStore;
  case GC_STORE_OP_DONT_CARE:
    return MTL::StoreActionDontCare;
  default:
    return MTL::StoreActionStore;
  }
}

MTL::TextureType ep_to_mtl_texture_type(GCTextureDimension dim,
                                        uint32_t array_layers) {
  switch (dim) {
  case GC_TEXTURE_DIM_1D:
    return (array_layers > 1) ? MTL::TextureType1DArray : MTL::TextureType1D;
  case GC_TEXTURE_DIM_2D:
    return (array_layers > 1) ? MTL::TextureType2DArray : MTL::TextureType2D;
  case GC_TEXTURE_DIM_3D:
    return MTL::TextureType3D;
  case GC_TEXTURE_DIM_CUBE:
    return (array_layers > 6) ? MTL::TextureTypeCubeArray
                              : MTL::TextureTypeCube;
  default:
    return MTL::TextureType2D;
  }
}

MTL::TextureUsage ep_to_mtl_texture_usage(GCTextureUsageFlags flags) {
  MTL::TextureUsage usage = 0;
  if (flags & GC_TEXTURE_USAGE_SAMPLED_BIT)
    usage |= MTL::TextureUsageShaderRead;
  if (flags & GC_TEXTURE_USAGE_STORAGE_BIT)
    usage |= MTL::TextureUsageShaderWrite;
  if (flags & GC_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT)
    usage |= MTL::TextureUsageRenderTarget;
  if (flags & GC_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT)
    usage |= MTL::TextureUsageRenderTarget;
  return usage;
}
