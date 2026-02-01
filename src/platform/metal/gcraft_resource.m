// gcraft_resource.m - Buffer, texture, sampler, and shader library implementation

#include "gcraft_internal.h"

#pragma mark - Buffer

GCError GCBufferCreate(GCDevicePtr device, const GCBufferDesc *desc, GCBufferPtr *out_buffer) {
    if (!device || !desc || !out_buffer) return ep_invalid_argument("device, desc, or out_buffer is NULL");

    @autoreleasepool {
        GCBuffer *buffer = calloc(1, sizeof(GCBuffer));
        if (!buffer) return ep_out_of_memory();

        buffer->ep_device = device;
        buffer->size = desc->size;
        buffer->usage = desc->usage;
        buffer->host_visible = desc->host_visible;

        MTLResourceOptions options = MTLResourceStorageModePrivate;
        if (desc->host_visible) {
            options = MTLResourceStorageModeShared;
        }

        buffer->buffer = [[device->device newBufferWithLength:desc->size options:options] retain];
        if (!buffer->buffer) {
            free(buffer);
            return ep_out_of_memory();
        }

        *out_buffer = buffer;
        return ep_ok();
    }
}

GCError GCBufferDestroy(GCBufferPtr buffer) {
    if (buffer) {
        @autoreleasepool {
            if (buffer->buffer) [buffer->buffer release];
        }
        free(buffer);
    }
    return ep_ok();
}

#pragma mark - Texture

GCError GCTextureCreate(GCDevicePtr device, const GCTextureDesc *desc, GCTexturePtr *out_texture) {
    if (!device || !desc || !out_texture) return ep_invalid_argument("device, desc, or out_texture is NULL");

    @autoreleasepool {
        GCTexture *texture = calloc(1, sizeof(GCTexture));
        if (!texture) return ep_out_of_memory();

        texture->ep_device = device;
        texture->desc = *desc;

        MTLTextureDescriptor *mtl_desc = [[MTLTextureDescriptor alloc] init];
        mtl_desc.textureType = ep_to_mtl_texture_type(desc->dimension, desc->array_layers);
        mtl_desc.pixelFormat = ep_to_mtl_pixel_format(desc->format);
        mtl_desc.width = desc->width;
        mtl_desc.height = desc->height;
        mtl_desc.depth = desc->depth > 0 ? desc->depth : 1;
        mtl_desc.mipmapLevelCount = desc->mip_levels > 0 ? desc->mip_levels : 1;
        mtl_desc.arrayLength = desc->array_layers > 0 ? desc->array_layers : 1;
        mtl_desc.usage = ep_to_mtl_texture_usage(desc->usage);
        mtl_desc.storageMode = MTLStorageModePrivate;

        texture->texture = [[device->device newTextureWithDescriptor:mtl_desc] retain];
        [mtl_desc release];

        if (!texture->texture) {
            free(texture);
            return ep_out_of_memory();
        }

        *out_texture = texture;
        return ep_ok();
    }
}

GCError GCTextureDestroy(GCTexturePtr texture) {
    if (texture) {
        @autoreleasepool {
            if (texture->texture) [texture->texture release];
        }
        free(texture);
    }
    return ep_ok();
}

#pragma mark - Sampler

GCError GCSamplerCreate(GCDevicePtr device, const GCSamplerDesc *desc, GCSamplerPtr *out_sampler) {
    if (!device || !desc || !out_sampler) return ep_invalid_argument("device, desc, or out_sampler is NULL");

    @autoreleasepool {
        GCSampler *sampler = calloc(1, sizeof(GCSampler));
        if (!sampler) return ep_out_of_memory();

        sampler->ep_device = device;

        MTLSamplerDescriptor *mtl_desc = [[MTLSamplerDescriptor alloc] init];
        mtl_desc.minFilter = ep_to_mtl_filter(desc->min_filter);
        mtl_desc.magFilter = ep_to_mtl_filter(desc->mag_filter);
        mtl_desc.sAddressMode = ep_to_mtl_address(desc->address_u);
        mtl_desc.tAddressMode = ep_to_mtl_address(desc->address_v);
        mtl_desc.rAddressMode = ep_to_mtl_address(desc->address_w);
        mtl_desc.compareFunction = ep_to_mtl_compare(desc->compare_op);
        mtl_desc.maxAnisotropy = (NSUInteger)desc->max_anisotropy;

        sampler->sampler = [[device->device newSamplerStateWithDescriptor:mtl_desc] retain];
        [mtl_desc release];

        if (!sampler->sampler) {
            free(sampler);
            return ep_out_of_memory();
        }

        *out_sampler = sampler;
        return ep_ok();
    }
}

GCError GCSamplerDestroy(GCSamplerPtr sampler) {
    if (sampler) {
        @autoreleasepool {
            if (sampler->sampler) [sampler->sampler release];
        }
        free(sampler);
    }
    return ep_ok();
}

#pragma mark - Shader Library

GCError GCShaderLibraryCreate(GCDevicePtr device, const GCShaderLibraryDesc *desc,
                              GCShaderLibraryPtr *out_library) {
    if (!device || !desc || !out_library) return ep_invalid_argument("device, desc, or out_library is NULL");
    if (desc->format != GC_SHADER_MSL) return ep_unsupported("only MSL shaders supported on Metal");

    @autoreleasepool {
        GCShaderLibrary *library = calloc(1, sizeof(GCShaderLibrary));
        if (!library) return ep_out_of_memory();

        library->ep_device = device;

        NSError *error = nil;
        NSString *source = [[NSString alloc] initWithBytes:desc->data
                                                    length:desc->size
                                                  encoding:NSUTF8StringEncoding];

        MTLCompileOptions *options = [[MTLCompileOptions alloc] init];
        options.languageVersion = MTLLanguageVersion3_2;

        library->library = [[device->device newLibraryWithSource:source options:options error:&error] retain];
        [source release];
        [options release];

        if (!library->library) {
            free(library);
            const char *msg = error ? [[error localizedDescription] UTF8String] : "shader compilation failed";
            return ep_error(GC_E_INVALID_ARGUMENT, msg);
        }

        if (desc->label && device->debug_names_enabled) {
            library->library.label = [NSString stringWithUTF8String:desc->label];
        }

        *out_library = library;
        return ep_ok();
    }
}

GCError GCShaderLibraryDestroy(GCShaderLibraryPtr library) {
    if (library) {
        @autoreleasepool {
            if (library->library) [library->library release];
        }
        free(library);
    }
    return ep_ok();
}
