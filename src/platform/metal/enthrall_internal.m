// enthrall_internal.m - Format conversion utilities for Metal backend

#include "enthrall_internal.h"

#pragma mark - Format Conversion

MTLPixelFormat ep_to_mtl_pixel_format(EPTextureFormat format) {
    switch (format) {
        case EP_FORMAT_RGBA8_UNORM:  return MTLPixelFormatRGBA8Unorm;
        case EP_FORMAT_BGRA8_UNORM:  return MTLPixelFormatBGRA8Unorm;
        case EP_FORMAT_RGBA16_FLOAT: return MTLPixelFormatRGBA16Float;
        case EP_FORMAT_RGBA32_FLOAT: return MTLPixelFormatRGBA32Float;
        case EP_FORMAT_D24S8:        return MTLPixelFormatDepth24Unorm_Stencil8;
        case EP_FORMAT_D32_FLOAT:    return MTLPixelFormatDepth32Float;
        default:                     return MTLPixelFormatInvalid;
    }
}

MTLSamplerMinMagFilter ep_to_mtl_filter(EPFilter filter) {
    switch (filter) {
        case EP_FILTER_NEAREST: return MTLSamplerMinMagFilterNearest;
        case EP_FILTER_LINEAR:  return MTLSamplerMinMagFilterLinear;
        default:                return MTLSamplerMinMagFilterNearest;
    }
}

MTLSamplerAddressMode ep_to_mtl_address(EPAddressMode mode) {
    switch (mode) {
        case EP_ADDRESS_CLAMP_TO_EDGE:  return MTLSamplerAddressModeClampToEdge;
        case EP_ADDRESS_REPEAT:         return MTLSamplerAddressModeRepeat;
        case EP_ADDRESS_MIRROR_REPEAT:  return MTLSamplerAddressModeMirrorRepeat;
        default:                        return MTLSamplerAddressModeClampToEdge;
    }
}

MTLCompareFunction ep_to_mtl_compare(EPCompareOp op) {
    switch (op) {
        case EP_COMPARE_NEVER:         return MTLCompareFunctionNever;
        case EP_COMPARE_LESS:          return MTLCompareFunctionLess;
        case EP_COMPARE_EQUAL:         return MTLCompareFunctionEqual;
        case EP_COMPARE_LESS_EQUAL:    return MTLCompareFunctionLessEqual;
        case EP_COMPARE_GREATER:       return MTLCompareFunctionGreater;
        case EP_COMPARE_NOT_EQUAL:     return MTLCompareFunctionNotEqual;
        case EP_COMPARE_GREATER_EQUAL: return MTLCompareFunctionGreaterEqual;
        case EP_COMPARE_ALWAYS:        return MTLCompareFunctionAlways;
        default:                       return MTLCompareFunctionAlways;
    }
}

MTLLoadAction ep_to_mtl_load_action(EPAttachmentLoadOp op) {
    switch (op) {
        case EP_LOAD_OP_LOAD:      return MTLLoadActionLoad;
        case EP_LOAD_OP_CLEAR:     return MTLLoadActionClear;
        case EP_LOAD_OP_DONT_CARE: return MTLLoadActionDontCare;
        default:                   return MTLLoadActionDontCare;
    }
}

MTLStoreAction ep_to_mtl_store_action(EPAttachmentStoreOp op) {
    switch (op) {
        case EP_STORE_OP_STORE:     return MTLStoreActionStore;
        case EP_STORE_OP_DONT_CARE: return MTLStoreActionDontCare;
        default:                    return MTLStoreActionStore;
    }
}

MTLTextureType ep_to_mtl_texture_type(EPTextureDimension dim, uint32_t array_layers) {
    switch (dim) {
        case EP_TEXTURE_DIM_1D:
            return (array_layers > 1) ? MTLTextureType1DArray : MTLTextureType1D;
        case EP_TEXTURE_DIM_2D:
            return (array_layers > 1) ? MTLTextureType2DArray : MTLTextureType2D;
        case EP_TEXTURE_DIM_3D:
            return MTLTextureType3D;
        case EP_TEXTURE_DIM_CUBE:
            return (array_layers > 6) ? MTLTextureTypeCubeArray : MTLTextureTypeCube;
        default:
            return MTLTextureType2D;
    }
}

MTLTextureUsage ep_to_mtl_texture_usage(EPTextureUsageFlags flags) {
    MTLTextureUsage usage = 0;
    if (flags & EP_TEXTURE_USAGE_SAMPLED_BIT)          usage |= MTLTextureUsageShaderRead;
    if (flags & EP_TEXTURE_USAGE_STORAGE_BIT)          usage |= MTLTextureUsageShaderWrite;
    if (flags & EP_TEXTURE_USAGE_COLOR_ATTACHMENT_BIT) usage |= MTLTextureUsageRenderTarget;
    if (flags & EP_TEXTURE_USAGE_DEPTH_ATTACHMENT_BIT) usage |= MTLTextureUsageRenderTarget;
    return usage;
}
