// enthrall_internal.cpp - Format conversion and utility implementations

#include "enthrall_internal.h"

DXGI_FORMAT ep_to_dxgi_format(EPTextureFormat format) {
    switch (format) {
        case EP_FORMAT_RGBA8_UNORM:   return DXGI_FORMAT_R8G8B8A8_UNORM;
        case EP_FORMAT_BGRA8_UNORM:   return DXGI_FORMAT_B8G8R8A8_UNORM;
        case EP_FORMAT_RGBA16_FLOAT:  return DXGI_FORMAT_R16G16B16A16_FLOAT;
        case EP_FORMAT_RGBA32_FLOAT:  return DXGI_FORMAT_R32G32B32A32_FLOAT;
        case EP_FORMAT_D24S8:         return DXGI_FORMAT_D24_UNORM_S8_UINT;
        case EP_FORMAT_D32_FLOAT:     return DXGI_FORMAT_D32_FLOAT;
        default:                       return DXGI_FORMAT_UNKNOWN;
    }
}

D3D12_FILTER ep_to_d3d12_filter(EPFilter min_filter, EPFilter mag_filter) {
    if (min_filter == EP_FILTER_NEAREST && mag_filter == EP_FILTER_NEAREST) {
        return D3D12_FILTER_MIN_MAG_MIP_POINT;
    } else if (min_filter == EP_FILTER_LINEAR && mag_filter == EP_FILTER_LINEAR) {
        return D3D12_FILTER_MIN_MAG_MIP_LINEAR;
    } else if (min_filter == EP_FILTER_LINEAR && mag_filter == EP_FILTER_NEAREST) {
        return D3D12_FILTER_MIN_LINEAR_MAG_MIP_POINT;
    } else {
        return D3D12_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT;
    }
}

D3D12_TEXTURE_ADDRESS_MODE ep_to_d3d12_address(EPAddressMode mode) {
    switch (mode) {
        case EP_ADDRESS_CLAMP_TO_EDGE: return D3D12_TEXTURE_ADDRESS_MODE_CLAMP;
        case EP_ADDRESS_REPEAT:        return D3D12_TEXTURE_ADDRESS_MODE_WRAP;
        case EP_ADDRESS_MIRROR_REPEAT: return D3D12_TEXTURE_ADDRESS_MODE_MIRROR;
        default:                       return D3D12_TEXTURE_ADDRESS_MODE_CLAMP;
    }
}

D3D12_COMPARISON_FUNC ep_to_d3d12_compare(EPCompareOp op) {
    switch (op) {
        case EP_COMPARE_NEVER:         return D3D12_COMPARISON_FUNC_NEVER;
        case EP_COMPARE_LESS:          return D3D12_COMPARISON_FUNC_LESS;
        case EP_COMPARE_EQUAL:         return D3D12_COMPARISON_FUNC_EQUAL;
        case EP_COMPARE_LESS_EQUAL:    return D3D12_COMPARISON_FUNC_LESS_EQUAL;
        case EP_COMPARE_GREATER:       return D3D12_COMPARISON_FUNC_GREATER;
        case EP_COMPARE_NOT_EQUAL:     return D3D12_COMPARISON_FUNC_NOT_EQUAL;
        case EP_COMPARE_GREATER_EQUAL: return D3D12_COMPARISON_FUNC_GREATER_EQUAL;
        case EP_COMPARE_ALWAYS:        return D3D12_COMPARISON_FUNC_ALWAYS;
        default:                       return D3D12_COMPARISON_FUNC_ALWAYS;
    }
}

D3D12_RESOURCE_STATES ep_to_d3d12_resource_state(EPTextureLayout layout) {
    switch (layout) {
        case EP_TEXTURE_LAYOUT_UNDEFINED:
            return D3D12_RESOURCE_STATE_COMMON;
        case EP_TEXTURE_LAYOUT_GENERAL:
            return D3D12_RESOURCE_STATE_COMMON;
        case EP_TEXTURE_LAYOUT_COLOR_ATTACHMENT:
            return D3D12_RESOURCE_STATE_RENDER_TARGET;
        case EP_TEXTURE_LAYOUT_DEPTH_STENCIL:
            return D3D12_RESOURCE_STATE_DEPTH_WRITE;
        case EP_TEXTURE_LAYOUT_SHADER_READ:
            return D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE |
                   D3D12_RESOURCE_STATE_NON_PIXEL_SHADER_RESOURCE;
        case EP_TEXTURE_LAYOUT_TRANSFER_SRC:
            return D3D12_RESOURCE_STATE_COPY_SOURCE;
        case EP_TEXTURE_LAYOUT_TRANSFER_DST:
            return D3D12_RESOURCE_STATE_COPY_DEST;
        case EP_TEXTURE_LAYOUT_PRESENT:
            return D3D12_RESOURCE_STATE_PRESENT;
        default:
            return D3D12_RESOURCE_STATE_COMMON;
    }
}

D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE ep_to_d3d12_load_op(EPAttachmentLoadOp op) {
    switch (op) {
        case EP_LOAD_OP_LOAD:      return D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_PRESERVE;
        case EP_LOAD_OP_CLEAR:     return D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_CLEAR;
        case EP_LOAD_OP_DONT_CARE: return D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_DISCARD;
        default:                   return D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_DISCARD;
    }
}

D3D12_RENDER_PASS_ENDING_ACCESS_TYPE ep_to_d3d12_store_op(EPAttachmentStoreOp op) {
    switch (op) {
        case EP_STORE_OP_STORE:     return D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_PRESERVE;
        case EP_STORE_OP_DONT_CARE: return D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_DISCARD;
        default:                    return D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_PRESERVE;
    }
}

D3D12_RESOURCE_DIMENSION ep_to_d3d12_dimension(EPTextureDimension dim) {
    switch (dim) {
        case EP_TEXTURE_DIM_1D:   return D3D12_RESOURCE_DIMENSION_TEXTURE1D;
        case EP_TEXTURE_DIM_2D:   return D3D12_RESOURCE_DIMENSION_TEXTURE2D;
        case EP_TEXTURE_DIM_3D:   return D3D12_RESOURCE_DIMENSION_TEXTURE3D;
        case EP_TEXTURE_DIM_CUBE: return D3D12_RESOURCE_DIMENSION_TEXTURE2D;
        default:                  return D3D12_RESOURCE_DIMENSION_TEXTURE2D;
    }
}

D3D12_SHADER_VISIBILITY ep_to_d3d12_visibility(EPShaderStageFlags stages) {
    // If multiple stages, use ALL
    int count = 0;
    if (stages & EP_STAGE_VERTEX_BIT) count++;
    if (stages & EP_STAGE_FRAGMENT_BIT) count++;
    if (stages & EP_STAGE_COMPUTE_BIT) count++;
    if (stages & EP_STAGE_MESH_BIT) count++;
    if (stages & EP_STAGE_TASK_BIT) count++;
    
    if (count > 1) return D3D12_SHADER_VISIBILITY_ALL;
    
    if (stages & EP_STAGE_VERTEX_BIT) return D3D12_SHADER_VISIBILITY_VERTEX;
    if (stages & EP_STAGE_FRAGMENT_BIT) return D3D12_SHADER_VISIBILITY_PIXEL;
    if (stages & EP_STAGE_MESH_BIT) return D3D12_SHADER_VISIBILITY_MESH;
    if (stages & EP_STAGE_TASK_BIT) return D3D12_SHADER_VISIBILITY_AMPLIFICATION;
    
    return D3D12_SHADER_VISIBILITY_ALL;
}
