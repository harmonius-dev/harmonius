// gcraft_internal.m - Format conversion utilities for Metal backend

#include "gcraft_internal.h"

#pragma mark - Format Conversion

MTLPixelFormat ep_to_mtl_pixel_format(GCTextureFormat format) {
    switch (format) {
        case GC_FORMAT_RGBA8_UNORM:  return MTLPixelFormatRGBA8Unorm;
        case GC_FORMAT_BGRA8_UNORM:  return MTLPixelFormatBGRA8Unorm;
        case GC_FORMAT_RGBA16_FLOAT: return MTLPixelFormatRGBA16Float;
        case GC_FORMAT_RGBA32_FLOAT: return MTLPixelFormatRGBA32Float;
        case GC_FORMAT_D24S8:        return MTLPixelFormatDepth24Unorm_Stencil8;
        case GC_FORMAT_D32_FLOAT:    return MTLPixelFormatDepth32Float;
        default:                     return MTLPixelFormatInvalid;
    }
}

MTLSamplerMinMagFilter ep_to_mtl_filter(GCFilter filter) {
    switch (filter) {
        case GC_FILTER_NEAREST: return MTLSamplerMinMagFilterNearest;
        case GC_FILTER_LINEAR:  return MTLSamplerMinMagFilterLinear;
        default:                return MTLSamplerMinMagFilterNearest;
    }
}

MTLSamplerAddressMode ep_to_mtl_address(GCAddressMode mode) {
    switch (mode) {
        case GC_ADDRESS_CLAMP_TO_EDGE:  return MTLSamplerAddressModeClampToEdge;
        case GC_ADDRESS_REPEAT:         return MTLSamplerAddressModeRepeat;
        case GC_ADDRESS_MIRROR_REPEAT:  return MTLSamplerAddressModeMirrorRepeat;
        default:                        return MTLSamplerAddressModeClampToEdge;
    }
}

MTLCompareFunction ep_to_mtl_compare(GCCompareOp op) {
    switch (op) {
        case GC_COMPARE_NEVER:         return MTLCompareFunctionNever;
        case GC_COMPARE_LESS:          return MTLCompareFunctionLess;
        case GC_COMPARE_EQUAL:         return MTLCompareFunctionEqual;
        case GC_COMPARE_LESS_EQUAL:    return MTLCompareFunctionLessEqual;
        case GC_COMPARE_GREATER:       return MTLCompareFunctionGreater;
        case GC_COMPARE_NOT_EQUAL:     return MTLCompareFunctionNotEqual;
        case GC_COMPARE_GREATER_EQUAL: return MTLCompareFunctionGreaterEqual;
        case GC_COMPARE_ALWAYS:        return MTLCompareFunctionAlways;
        default:                       return MTLCompareFunctionAlways;
    }
}

MTLLoadAction ep_to_mtl_load_action(GCAttachmentLoadOp op) {
    switch (op) {
        case GC_LOAD_OP_LOAD:      return MTLLoadActionLoad;
        case GC_LOAD_OP_CLEAR:     return MTLLoadActionClear;
        case GC_LOAD_OP_DONT_CARE: return MTLLoadActionDontCare;
        default:                   return MTLLoadActionDontCare;
    }
}

MTLStoreAction ep_to_mtl_store_action(GCAttachmentStoreOp op) {
    switch (op) {
        case GC_STORE_OP_STORE:     return MTLStoreActionStore;
        case GC_STORE_OP_DONT_CARE: return MTLStoreActionDontCare;
        default:                    return MTLStoreActionStore;
    }
}

MTLTextureType ep_to_mtl_texture_type(GCTextureDimension dim, uint32_t array_layers) {
    switch (dim) {
        case GC_TEXTURE_DIM_1D:
            return (array_layers > 1) ? MTLTextureType1DArray : MTLTextureType1D;
        case GC_TEXTURE_DIM_2D:
            return (array_layers > 1) ? MTLTextureType2DArray : MTLTextureType2D;
        case GC_TEXTURE_DIM_3D:
            return MTLTextureType3D;
        case GC_TEXTURE_DIM_CUBE:
            return (array_layers > 6) ? MTLTextureTypeCubeArray : MTLTextureTypeCube;
        default:
            return MTLTextureType2D;
    }
}

MTLTextureUsage ep_to_mtl_texture_usage(GCTextureUsageFlags flags) {
    MTLTextureUsage usage = 0;
    if (flags & GC_TEXTURE_USAGE_SAMPLED_BIT)          usage |= MTLTextureUsageShaderRead;
    if (flags & GC_TEXTURE_USAGE_STORAGE_BIT)          usage |= MTLTextureUsageShaderWrite;
    if (flags & GC_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT) usage |= MTLTextureUsageRenderTarget;
    if (flags & GC_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT) usage |= MTLTextureUsageRenderTarget;
    return usage;
}
