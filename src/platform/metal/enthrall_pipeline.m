// enthrall_pipeline.m - Pipeline and acceleration structure implementation

#include "enthrall_internal.h"

#pragma mark - Render Pipeline

EPError EPRenderPipelineCreate(EPDevicePtr device, const EPRenderPipelineDesc *desc,
                               EPRenderPipelinePtr *out_pipeline) {
    if (!device || !desc || !out_pipeline) return ep_invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library) return ep_invalid_argument("shader library is NULL");

    @autoreleasepool {
        EPRenderPipeline *pipeline = calloc(1, sizeof(EPRenderPipeline));
        if (!pipeline) return ep_out_of_memory();

        pipeline->ep_device = device;

        MTLRenderPipelineDescriptor *mtl_desc = [[MTLRenderPipelineDescriptor alloc] init];

        // Vertex function
        if (desc->vertex_entry) {
            NSString *vertex_name = [NSString stringWithUTF8String:desc->vertex_entry];
            mtl_desc.vertexFunction = [desc->library->library newFunctionWithName:vertex_name];
        }

        // Fragment function
        if (desc->fragment_entry) {
            NSString *fragment_name = [NSString stringWithUTF8String:desc->fragment_entry];
            mtl_desc.fragmentFunction = [desc->library->library newFunctionWithName:fragment_name];
        }

        // Color attachments
        for (uint32_t i = 0; i < desc->color_format_count && i < 8; i++) {
            mtl_desc.colorAttachments[i].pixelFormat = ep_to_mtl_pixel_format(desc->color_formats[i]);
        }

        // Depth/stencil format - only set if using depth-renderable formats
        // D24S8 and D32_FLOAT are depth formats; other formats mean "no depth/stencil"
        if (desc->depth_format == EP_FORMAT_D24S8 || desc->depth_format == EP_FORMAT_D32_FLOAT) {
            mtl_desc.depthAttachmentPixelFormat = ep_to_mtl_pixel_format(desc->depth_format);
        }
        if (desc->stencil_format == EP_FORMAT_D24S8) {
            mtl_desc.stencilAttachmentPixelFormat = ep_to_mtl_pixel_format(desc->stencil_format);
        }

        // Vertex descriptor
        if (desc->attribute_count > 0 && desc->attributes) {
            MTLVertexDescriptor *vertex_desc = [[MTLVertexDescriptor alloc] init];

            for (uint32_t i = 0; i < desc->attribute_count; i++) {
                vertex_desc.attributes[desc->attributes[i].location].format = MTLVertexFormatFloat4;
                vertex_desc.attributes[desc->attributes[i].location].offset = desc->attributes[i].offset;
                vertex_desc.attributes[desc->attributes[i].location].bufferIndex = desc->attributes[i].binding;
            }

            for (uint32_t i = 0; i < desc->binding_count; i++) {
                vertex_desc.layouts[desc->bindings[i].binding].stride = desc->bindings[i].stride;
                vertex_desc.layouts[desc->bindings[i].binding].stepFunction =
                    desc->bindings[i].per_instance ? MTLVertexStepFunctionPerInstance : MTLVertexStepFunctionPerVertex;
            }

            mtl_desc.vertexDescriptor = vertex_desc;
            [vertex_desc release];
        }

        NSError *error = nil;
        pipeline->state = [[device->device newRenderPipelineStateWithDescriptor:mtl_desc error:&error] retain];
        [mtl_desc release];

        if (!pipeline->state) {
            free(pipeline);
            const char *msg = error ? [[error localizedDescription] UTF8String] : "pipeline creation failed";
            return ep_error(EP_E_INVALID_ARGUMENT, msg);
        }

        // Depth stencil state
        if (desc->raster_state.depth_test_enable || desc->raster_state.depth_write_enable) {
            MTLDepthStencilDescriptor *depth_desc = [[MTLDepthStencilDescriptor alloc] init];
            depth_desc.depthCompareFunction = desc->raster_state.depth_test_enable
                ? ep_to_mtl_compare(desc->raster_state.depth_compare)
                : MTLCompareFunctionAlways;
            depth_desc.depthWriteEnabled = desc->raster_state.depth_write_enable;

            pipeline->depth_stencil = [[device->device newDepthStencilStateWithDescriptor:depth_desc] retain];
            [depth_desc release];
        }

        *out_pipeline = pipeline;
        return ep_ok();
    }
}

EPError EPRenderPipelineDestroy(EPRenderPipelinePtr pipeline) {
    if (pipeline) {
        @autoreleasepool {
            if (pipeline->state) [pipeline->state release];
            if (pipeline->depth_stencil) [pipeline->depth_stencil release];
        }
        free(pipeline);
    }
    return ep_ok();
}

#pragma mark - Compute Pipeline

EPError EPComputePipelineCreate(EPDevicePtr device, const EPComputePipelineDesc *desc,
                                EPComputePipelinePtr *out_pipeline) {
    if (!device || !desc || !out_pipeline) return ep_invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library || !desc->entry) return ep_invalid_argument("library or entry is NULL");

    @autoreleasepool {
        EPComputePipeline *pipeline = calloc(1, sizeof(EPComputePipeline));
        if (!pipeline) return ep_out_of_memory();

        pipeline->ep_device = device;

        NSString *entry_name = [NSString stringWithUTF8String:desc->entry];
        id<MTLFunction> function = [desc->library->library newFunctionWithName:entry_name];

        if (!function) {
            free(pipeline);
            return ep_invalid_argument("compute function not found");
        }

        NSError *error = nil;
        pipeline->state = [[device->device newComputePipelineStateWithFunction:function error:&error] retain];
        [function release];

        if (!pipeline->state) {
            free(pipeline);
            const char *msg = error ? [[error localizedDescription] UTF8String] : "compute pipeline creation failed";
            return ep_error(EP_E_INVALID_ARGUMENT, msg);
        }

        *out_pipeline = pipeline;
        return ep_ok();
    }
}

EPError EPComputePipelineDestroy(EPComputePipelinePtr pipeline) {
    if (pipeline) {
        @autoreleasepool {
            if (pipeline->state) [pipeline->state release];
        }
        free(pipeline);
    }
    return ep_ok();
}

#pragma mark - Mesh Pipeline (Metal 3+ Mesh Shaders)

EPError EPMeshPipelineCreate(EPDevicePtr device, const EPMeshPipelineDesc *desc,
                             EPMeshPipelinePtr *out_pipeline) {
    if (!device || !desc || !out_pipeline) return ep_invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library || !desc->mesh_entry) return ep_invalid_argument("library or mesh_entry is NULL");

    // Mesh shaders require Apple7 (A14+) or Mac2 GPU family (Metal 3+)
    // Note: Simulator only supports Apple2 family, so mesh shaders won't work there
    BOOL supports_mesh = [device->device supportsFamily:MTLGPUFamilyApple7] ||
                         [device->device supportsFamily:MTLGPUFamilyMac2];
    if (!supports_mesh) {
        return ep_unsupported("mesh shaders not supported on this device (requires Apple7/Mac2 GPU family)");
    }

    @autoreleasepool {
        EPMeshPipeline *pipeline = calloc(1, sizeof(EPMeshPipeline));
        if (!pipeline) return ep_out_of_memory();

        pipeline->ep_device = device;

        MTLMeshRenderPipelineDescriptor *mtl_desc = [[MTLMeshRenderPipelineDescriptor alloc] init];

        // Mesh function (required)
        NSString *mesh_name = [NSString stringWithUTF8String:desc->mesh_entry];
        mtl_desc.meshFunction = [desc->library->library newFunctionWithName:mesh_name];

        if (!mtl_desc.meshFunction) {
            [mtl_desc release];
            free(pipeline);
            return ep_invalid_argument("mesh function not found");
        }

        // Object/Task function (optional amplification shader)
        if (desc->task_entry) {
            NSString *task_name = [NSString stringWithUTF8String:desc->task_entry];
            mtl_desc.objectFunction = [desc->library->library newFunctionWithName:task_name];
        }

        // Fragment function
        if (desc->fragment_entry) {
            NSString *fragment_name = [NSString stringWithUTF8String:desc->fragment_entry];
            mtl_desc.fragmentFunction = [desc->library->library newFunctionWithName:fragment_name];
        }

        // Color attachments
        for (uint32_t i = 0; i < desc->color_format_count && i < 8; i++) {
            mtl_desc.colorAttachments[i].pixelFormat = ep_to_mtl_pixel_format(desc->color_formats[i]);
        }

        // Depth/stencil format - only set if using depth-renderable formats
        if (desc->depth_format == EP_FORMAT_D24S8 || desc->depth_format == EP_FORMAT_D32_FLOAT) {
            mtl_desc.depthAttachmentPixelFormat = ep_to_mtl_pixel_format(desc->depth_format);
        }
        if (desc->stencil_format == EP_FORMAT_D24S8) {
            mtl_desc.stencilAttachmentPixelFormat = ep_to_mtl_pixel_format(desc->stencil_format);
        }

        NSError *error = nil;
        pipeline->state = [[device->device newRenderPipelineStateWithMeshDescriptor:mtl_desc
                                                                           options:0
                                                                        reflection:nil
                                                                             error:&error] retain];
        [mtl_desc release];

        if (!pipeline->state) {
            free(pipeline);
            const char *msg = error ? [[error localizedDescription] UTF8String] : "mesh pipeline creation failed";
            return ep_error(EP_E_INVALID_ARGUMENT, msg);
        }

        // Depth stencil state
        if (desc->raster_state.depth_test_enable || desc->raster_state.depth_write_enable) {
            MTLDepthStencilDescriptor *depth_desc = [[MTLDepthStencilDescriptor alloc] init];
            depth_desc.depthCompareFunction = desc->raster_state.depth_test_enable
                ? ep_to_mtl_compare(desc->raster_state.depth_compare)
                : MTLCompareFunctionAlways;
            depth_desc.depthWriteEnabled = desc->raster_state.depth_write_enable;

            pipeline->depth_stencil = [[device->device newDepthStencilStateWithDescriptor:depth_desc] retain];
            [depth_desc release];
        }

        *out_pipeline = pipeline;
        return ep_ok();
    }
}

EPError EPMeshPipelineDestroy(EPMeshPipelinePtr pipeline) {
    if (pipeline) {
        @autoreleasepool {
            if (pipeline->state) [pipeline->state release];
            if (pipeline->depth_stencil) [pipeline->depth_stencil release];
        }
        free(pipeline);
    }
    return ep_ok();
}

#pragma mark - Ray Tracing Pipeline

EPError EPRayTracingPipelineCreate(EPDevicePtr device, const EPRayTracingPipelineDesc *desc,
                                   EPRayTracingPipelinePtr *out_pipeline) {
    if (!device || !desc || !out_pipeline) return ep_invalid_argument("device, desc, or out_pipeline is NULL");
    if (!desc->library) return ep_invalid_argument("library is NULL");

    if (!device->device.supportsRaytracing) {
        return ep_unsupported("ray tracing not supported on this device");
    }

    @autoreleasepool {
        EPRayTracingPipeline *pipeline = calloc(1, sizeof(EPRayTracingPipeline));
        if (!pipeline) return ep_out_of_memory();

        pipeline->ep_device = device;
        pipeline->max_recursion_depth = desc->max_recursion_depth;

        NSMutableArray<id<MTLComputePipelineState>> *states = [[NSMutableArray alloc] init];
        NSMutableArray<id<MTLFunction>> *visible_functions = [[NSMutableArray alloc] init];

        for (uint32_t i = 0; i < desc->group_count; i++) {
            const EPRayTracingShaderGroupDesc *group = &desc->groups[i];

            // Create ray generation compute pipeline
            if (group->raygen_entry) {
                NSString *name = [NSString stringWithUTF8String:group->raygen_entry];
                id<MTLFunction> func = [desc->library->library newFunctionWithName:name];
                if (func) {
                    MTLComputePipelineDescriptor *compute_desc = [[MTLComputePipelineDescriptor alloc] init];
                    compute_desc.computeFunction = func;
                    compute_desc.maxCallStackDepth = desc->max_recursion_depth + 1;

                    // Link visible functions for intersection/any-hit
                    MTLLinkedFunctions *linked = [[MTLLinkedFunctions alloc] init];
                    linked.functions = visible_functions;
                    compute_desc.linkedFunctions = linked;

                    NSError *error = nil;
                    id<MTLComputePipelineState> state = [device->device newComputePipelineStateWithDescriptor:compute_desc
                                                                                                      options:0
                                                                                                   reflection:nil
                                                                                                        error:&error];
                    if (state) {
                        [states addObject:state];
                    }

                    [linked release];
                    [compute_desc release];
                    [func release];
                }
            }

            // Collect visible functions for linking
            if (group->hit_entry) {
                NSString *name = [NSString stringWithUTF8String:group->hit_entry];
                id<MTLFunction> func = [desc->library->library newFunctionWithName:name];
                if (func) [visible_functions addObject:func];
            }
            if (group->any_hit_entry) {
                NSString *name = [NSString stringWithUTF8String:group->any_hit_entry];
                id<MTLFunction> func = [desc->library->library newFunctionWithName:name];
                if (func) [visible_functions addObject:func];
            }
            if (group->miss_entry) {
                NSString *name = [NSString stringWithUTF8String:group->miss_entry];
                id<MTLFunction> func = [desc->library->library newFunctionWithName:name];
                if (func) [visible_functions addObject:func];
            }
            if (group->intersection_entry) {
                NSString *name = [NSString stringWithUTF8String:group->intersection_entry];
                id<MTLFunction> func = [desc->library->library newFunctionWithName:name];
                if (func) [visible_functions addObject:func];
            }
        }

        // Create visible function table for intersection functions
        if (visible_functions.count > 0 && states.count > 0) {
            MTLVisibleFunctionTableDescriptor *vft_desc = [[MTLVisibleFunctionTableDescriptor alloc] init];
            vft_desc.functionCount = visible_functions.count;
            pipeline->function_table = [[states[0] newVisibleFunctionTableWithDescriptor:vft_desc] retain];
            [vft_desc release];

            // Populate function table
            for (NSUInteger j = 0; j < visible_functions.count; j++) {
                id<MTLFunctionHandle> handle = [states[0] functionHandleWithFunction:visible_functions[j]];
                if (handle) {
                    [pipeline->function_table setFunction:handle atIndex:j];
                }
            }
        }

        pipeline->states = [states retain];
        [states release];
        [visible_functions release];

        *out_pipeline = pipeline;
        return ep_ok();
    }
}

EPError EPRayTracingPipelineDestroy(EPRayTracingPipelinePtr pipeline) {
    if (pipeline) {
        @autoreleasepool {
            if (pipeline->states) [pipeline->states release];
            if (pipeline->function_table) [pipeline->function_table release];
            if (pipeline->intersection_table) [pipeline->intersection_table release];
        }
        free(pipeline);
    }
    return ep_ok();
}

#pragma mark - Acceleration Structure

EPError EPAccelerationStructureCreate(EPDevicePtr device, const EPAccelerationStructureDesc *desc,
                                      EPAccelerationStructurePtr *out_as) {
    if (!device || !desc || !out_as) return ep_invalid_argument("device, desc, or out_as is NULL");

    // Ray tracing requires Apple6 (A13+) or Mac2 GPU family
    // Note: Simulator only supports Apple2 family, so ray tracing won't work there
    BOOL supports_rt = [device->device supportsFamily:MTLGPUFamilyApple6] ||
                       [device->device supportsFamily:MTLGPUFamilyMac2];
    if (!supports_rt) {
        return ep_unsupported("ray tracing not supported on this device (requires Apple6/Mac2 GPU family)");
    }

    @autoreleasepool {
        EPAccelerationStructure *as = calloc(1, sizeof(EPAccelerationStructure));
        if (!as) return ep_out_of_memory();

        as->ep_device = device;
        as->top_level = desc->top_level;

        if (desc->top_level) {
            // Instance acceleration structure (TLAS)
            MTLInstanceAccelerationStructureDescriptor *instance_desc =
                [[MTLInstanceAccelerationStructureDescriptor alloc] init];
            instance_desc.usage = desc->allow_update
                ? MTLAccelerationStructureUsageRefit
                : MTLAccelerationStructureUsageNone;

            MTLAccelerationStructureSizes sizes =
                [device->device accelerationStructureSizesWithDescriptor:instance_desc];

            as->accel = [[device->device newAccelerationStructureWithSize:sizes.accelerationStructureSize] retain];
            as->scratch_buffer = [[device->device newBufferWithLength:sizes.buildScratchBufferSize
                                                              options:MTLResourceStorageModePrivate] retain];

            [instance_desc release];
        } else {
            // Primitive acceleration structure (BLAS)
            MTLPrimitiveAccelerationStructureDescriptor *prim_desc =
                [[MTLPrimitiveAccelerationStructureDescriptor alloc] init];

            NSMutableArray<MTLAccelerationStructureGeometryDescriptor *> *geometries =
                [[NSMutableArray alloc] init];

            for (uint32_t i = 0; i < desc->geometry_count; i++) {
                const EPAccelerationStructureGeometryDesc *geo = &desc->geometries[i];

                MTLAccelerationStructureTriangleGeometryDescriptor *tri_desc =
                    [[MTLAccelerationStructureTriangleGeometryDescriptor alloc] init];

                if (geo->vertex_buffer) {
                    tri_desc.vertexBuffer = geo->vertex_buffer->buffer;
                    tri_desc.vertexBufferOffset = geo->vertex_offset;
                    tri_desc.vertexStride = geo->vertex_stride;
                    tri_desc.triangleCount = geo->vertex_count / 3;
                }

                if (geo->index_buffer) {
                    tri_desc.indexBuffer = geo->index_buffer->buffer;
                    tri_desc.indexBufferOffset = geo->index_offset;
                    tri_desc.indexType = MTLIndexTypeUInt32;
                    tri_desc.triangleCount = geo->index_count / 3;
                }

                tri_desc.opaque = geo->opaque;

                [geometries addObject:tri_desc];
                [tri_desc release];
            }

            prim_desc.geometryDescriptors = geometries;
            prim_desc.usage = desc->allow_update
                ? MTLAccelerationStructureUsageRefit
                : MTLAccelerationStructureUsageNone;

            MTLAccelerationStructureSizes sizes =
                [device->device accelerationStructureSizesWithDescriptor:prim_desc];

            as->accel = [[device->device newAccelerationStructureWithSize:sizes.accelerationStructureSize] retain];
            as->scratch_buffer = [[device->device newBufferWithLength:sizes.buildScratchBufferSize
                                                              options:MTLResourceStorageModePrivate] retain];

            [geometries release];
            [prim_desc release];
        }

        if (!as->accel) {
            if (as->scratch_buffer) [as->scratch_buffer release];
            free(as);
            return ep_out_of_memory();
        }

        *out_as = as;
        return ep_ok();
    }
}

EPError EPAccelerationStructureDestroy(EPAccelerationStructurePtr as) {
    if (as) {
        @autoreleasepool {
            if (as->accel) [as->accel release];
            if (as->scratch_buffer) [as->scratch_buffer release];
        }
        free(as);
    }
    return ep_ok();
}
