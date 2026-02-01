// gcraft_pipeline.cpp - Pipeline and acceleration structure (Metal-cpp)

#include "gcraft_internal.h"
#include <vector>

extern "C" {

GCError GCRenderPipelineCreate(GCDevicePtr device,
                               const GCRenderPipelineDesc *desc,
                               GCRenderPipelinePtr *out_pipeline) {
  if (!device || !desc || !out_pipeline)
    return ep_invalid_argument("device, desc, or out_pipeline is NULL");
  if (!desc->library)
    return ep_invalid_argument("shader library is NULL");

  GCDevice *dev = static_cast<GCDevice *>(device);
  GCShaderLibrary *lib = static_cast<GCShaderLibrary *>(desc->library);

  GCRenderPipeline *pipeline =
      static_cast<GCRenderPipeline *>(calloc(1, sizeof(GCRenderPipeline)));
  if (!pipeline)
    return ep_out_of_memory();

  pipeline->ep_device = dev;

  MTL::RenderPipelineDescriptor *mtl_desc =
      MTL::RenderPipelineDescriptor::alloc()->init();

  if (desc->vertex_entry) {
    NS::String *vertex_name =
        NS::String::string(desc->vertex_entry, NS::UTF8StringEncoding);
    MTL::Function *vert_fn = lib->library->newFunction(vertex_name);
    mtl_desc->setVertexFunction(vert_fn);
    if (vert_fn)
      vert_fn->release();
  }
  if (desc->fragment_entry) {
    NS::String *fragment_name =
        NS::String::string(desc->fragment_entry, NS::UTF8StringEncoding);
    MTL::Function *frag_fn = lib->library->newFunction(fragment_name);
    mtl_desc->setFragmentFunction(frag_fn);
    if (frag_fn)
      frag_fn->release();
  }

  for (uint32_t i = 0; i < desc->color_format_count && i < 8; i++) {
    mtl_desc->colorAttachments()->object(i)->setPixelFormat(
        ep_to_mtl_pixel_format(desc->color_formats[i]));
  }
  if (desc->depth_format == GC_FORMAT_D24S8 ||
      desc->depth_format == GC_FORMAT_D32_FLOAT) {
    mtl_desc->setDepthAttachmentPixelFormat(
        ep_to_mtl_pixel_format(desc->depth_format));
  }
  if (desc->stencil_format == GC_FORMAT_D24S8) {
    mtl_desc->setStencilAttachmentPixelFormat(
        ep_to_mtl_pixel_format(desc->stencil_format));
  }

  if (desc->attribute_count > 0 && desc->attributes) {
    MTL::VertexDescriptor *vertex_desc = MTL::VertexDescriptor::alloc()->init();
    for (uint32_t i = 0; i < desc->attribute_count; i++) {
      vertex_desc->attributes()
          ->object(desc->attributes[i].location)
          ->setFormat(MTL::VertexFormatFloat4);
      vertex_desc->attributes()
          ->object(desc->attributes[i].location)
          ->setOffset(desc->attributes[i].offset);
      vertex_desc->attributes()
          ->object(desc->attributes[i].location)
          ->setBufferIndex(desc->attributes[i].binding);
    }
    for (uint32_t i = 0; i < desc->binding_count; i++) {
      vertex_desc->layouts()
          ->object(desc->bindings[i].binding)
          ->setStride(desc->bindings[i].stride);
      vertex_desc->layouts()
          ->object(desc->bindings[i].binding)
          ->setStepFunction(desc->bindings[i].per_instance
                                ? MTL::VertexStepFunctionPerInstance
                                : MTL::VertexStepFunctionPerVertex);
    }
    mtl_desc->setVertexDescriptor(vertex_desc);
    vertex_desc->release();
  }

  NS::Error *error = nullptr;
  pipeline->state = dev->device->newRenderPipelineState(mtl_desc, &error);
  mtl_desc->release();

  if (!pipeline->state) {
    const char *msg = error && error->localizedDescription()
                          ? error->localizedDescription()->utf8String()
                          : "pipeline creation failed";
    free(pipeline);
    return ep_error(GC_E_INVALID_ARGUMENT, msg);
  }

  if (desc->raster_state.depth_test_enable ||
      desc->raster_state.depth_write_enable) {
    MTL::DepthStencilDescriptor *depth_desc =
        MTL::DepthStencilDescriptor::alloc()->init();
    depth_desc->setDepthCompareFunction(
        desc->raster_state.depth_test_enable
            ? ep_to_mtl_compare(desc->raster_state.depth_compare)
            : MTL::CompareFunctionAlways);
    depth_desc->setDepthWriteEnabled(desc->raster_state.depth_write_enable);
    pipeline->depth_stencil = dev->device->newDepthStencilState(depth_desc);
    depth_desc->release();
  }

  *out_pipeline = pipeline;
  return ep_ok();
}

GCError GCRenderPipelineDestroy(GCRenderPipelinePtr pipeline) {
  if (pipeline) {
    GCRenderPipeline *pipe = static_cast<GCRenderPipeline *>(pipeline);
    if (pipe->state)
      pipe->state->release();
    if (pipe->depth_stencil)
      pipe->depth_stencil->release();
    free(pipeline);
  }
  return ep_ok();
}

GCError GCComputePipelineCreate(GCDevicePtr device,
                                const GCComputePipelineDesc *desc,
                                GCComputePipelinePtr *out_pipeline) {
  if (!device || !desc || !out_pipeline)
    return ep_invalid_argument("device, desc, or out_pipeline is NULL");
  if (!desc->library || !desc->entry)
    return ep_invalid_argument("library or entry is NULL");

  GCDevice *dev = static_cast<GCDevice *>(device);
  GCShaderLibrary *lib = static_cast<GCShaderLibrary *>(desc->library);

  GCComputePipeline *pipeline =
      static_cast<GCComputePipeline *>(calloc(1, sizeof(GCComputePipeline)));
  if (!pipeline)
    return ep_out_of_memory();

  pipeline->ep_device = dev;

  NS::String *entry_name =
      NS::String::string(desc->entry, NS::UTF8StringEncoding);
  MTL::Function *function = lib->library->newFunction(entry_name);

  if (!function) {
    free(pipeline);
    return ep_invalid_argument("compute function not found");
  }

  NS::Error *error = nullptr;
  pipeline->state = dev->device->newComputePipelineState(function, &error);
  function->release();

  if (!pipeline->state) {
    const char *msg = error && error->localizedDescription()
                          ? error->localizedDescription()->utf8String()
                          : "compute pipeline creation failed";
    free(pipeline);
    return ep_error(GC_E_INVALID_ARGUMENT, msg);
  }

  *out_pipeline = pipeline;
  return ep_ok();
}

GCError GCComputePipelineDestroy(GCComputePipelinePtr pipeline) {
  if (pipeline) {
    GCComputePipeline *pipe = static_cast<GCComputePipeline *>(pipeline);
    if (pipe->state)
      pipe->state->release();
    free(pipeline);
  }
  return ep_ok();
}

GCError GCMeshPipelineCreate(GCDevicePtr device, const GCMeshPipelineDesc *desc,
                             GCMeshPipelinePtr *out_pipeline) {
  if (!device || !desc || !out_pipeline)
    return ep_invalid_argument("device, desc, or out_pipeline is NULL");
  if (!desc->library || !desc->mesh_entry)
    return ep_invalid_argument("library or mesh_entry is NULL");

  GCDevice *dev = static_cast<GCDevice *>(device);
  if (!dev->device->supportsFamily(MTL::GPUFamilyApple7) &&
      !dev->device->supportsFamily(MTL::GPUFamilyMac2)) {
    return ep_unsupported("mesh shaders not supported on this device (requires "
                          "Apple7/Mac2 GPU family)");
  }

  GCShaderLibrary *lib = static_cast<GCShaderLibrary *>(desc->library);

  GCMeshPipeline *pipeline =
      static_cast<GCMeshPipeline *>(calloc(1, sizeof(GCMeshPipeline)));
  if (!pipeline)
    return ep_out_of_memory();

  pipeline->ep_device = dev;

  MTL::MeshRenderPipelineDescriptor *mtl_desc =
      MTL::MeshRenderPipelineDescriptor::alloc()->init();

  NS::String *mesh_name =
      NS::String::string(desc->mesh_entry, NS::UTF8StringEncoding);
  MTL::Function *mesh_fn = lib->library->newFunction(mesh_name);
  mtl_desc->setMeshFunction(mesh_fn);
  if (mesh_fn)
    mesh_fn->release();

  if (!mtl_desc->meshFunction()) {
    mtl_desc->release();
    free(pipeline);
    return ep_invalid_argument("mesh function not found");
  }

  if (desc->task_entry) {
    NS::String *task_name =
        NS::String::string(desc->task_entry, NS::UTF8StringEncoding);
    MTL::Function *task_fn = lib->library->newFunction(task_name);
    mtl_desc->setObjectFunction(task_fn);
    if (task_fn)
      task_fn->release();
  }
  if (desc->fragment_entry) {
    NS::String *fragment_name =
        NS::String::string(desc->fragment_entry, NS::UTF8StringEncoding);
    MTL::Function *frag_fn = lib->library->newFunction(fragment_name);
    mtl_desc->setFragmentFunction(frag_fn);
    if (frag_fn)
      frag_fn->release();
  }

  for (uint32_t i = 0; i < desc->color_format_count && i < 8; i++) {
    mtl_desc->colorAttachments()->object(i)->setPixelFormat(
        ep_to_mtl_pixel_format(desc->color_formats[i]));
  }
  if (desc->depth_format == GC_FORMAT_D24S8 ||
      desc->depth_format == GC_FORMAT_D32_FLOAT) {
    mtl_desc->setDepthAttachmentPixelFormat(
        ep_to_mtl_pixel_format(desc->depth_format));
  }
  if (desc->stencil_format == GC_FORMAT_D24S8) {
    mtl_desc->setStencilAttachmentPixelFormat(
        ep_to_mtl_pixel_format(desc->stencil_format));
  }

  NS::Error *error = nullptr;
  pipeline->state = dev->device->newRenderPipelineState(
      mtl_desc, MTL::PipelineOption(0), nullptr, &error);
  mtl_desc->release();

  if (!pipeline->state) {
    const char *msg = error && error->localizedDescription()
                          ? error->localizedDescription()->utf8String()
                          : "mesh pipeline creation failed";
    free(pipeline);
    return ep_error(GC_E_INVALID_ARGUMENT, msg);
  }

  if (desc->raster_state.depth_test_enable ||
      desc->raster_state.depth_write_enable) {
    MTL::DepthStencilDescriptor *depth_desc =
        MTL::DepthStencilDescriptor::alloc()->init();
    depth_desc->setDepthCompareFunction(
        desc->raster_state.depth_test_enable
            ? ep_to_mtl_compare(desc->raster_state.depth_compare)
            : MTL::CompareFunctionAlways);
    depth_desc->setDepthWriteEnabled(desc->raster_state.depth_write_enable);
    pipeline->depth_stencil = dev->device->newDepthStencilState(depth_desc);
    depth_desc->release();
  }

  *out_pipeline = pipeline;
  return ep_ok();
}

GCError GCMeshPipelineDestroy(GCMeshPipelinePtr pipeline) {
  if (pipeline) {
    GCMeshPipeline *pipe = static_cast<GCMeshPipeline *>(pipeline);
    if (pipe->state)
      pipe->state->release();
    if (pipe->depth_stencil)
      pipe->depth_stencil->release();
    free(pipeline);
  }
  return ep_ok();
}

GCError GCRayTracingPipelineCreate(GCDevicePtr device,
                                   const GCRayTracingPipelineDesc *desc,
                                   GCRayTracingPipelinePtr *out_pipeline) {
  if (!device || !desc || !out_pipeline)
    return ep_invalid_argument("device, desc, or out_pipeline is NULL");
  if (!desc->library)
    return ep_invalid_argument("library is NULL");

  GCDevice *dev = static_cast<GCDevice *>(device);
  if (!dev->device->supportsRaytracing()) {
    return ep_unsupported("ray tracing not supported on this device");
  }

  GCShaderLibrary *lib = static_cast<GCShaderLibrary *>(desc->library);

  GCRayTracingPipeline *pipeline = static_cast<GCRayTracingPipeline *>(
      calloc(1, sizeof(GCRayTracingPipeline)));
  if (!pipeline)
    return ep_out_of_memory();

  pipeline->ep_device = dev;
  pipeline->max_recursion_depth = desc->max_recursion_depth;

  std::vector<MTL::ComputePipelineState *> states_vec;
  std::vector<MTL::Function *> visible_functions_vec;

  for (uint32_t i = 0; i < desc->group_count; i++) {
    const GCRayTracingShaderGroupDesc *group = &desc->groups[i];

    if (group->hit_entry) {
      NS::String *name =
          NS::String::string(group->hit_entry, NS::UTF8StringEncoding);
      MTL::Function *func = lib->library->newFunction(name);
      if (func) {
        visible_functions_vec.push_back(func);
        func->release();
      }
    }
    if (group->any_hit_entry) {
      NS::String *name =
          NS::String::string(group->any_hit_entry, NS::UTF8StringEncoding);
      MTL::Function *func = lib->library->newFunction(name);
      if (func) {
        visible_functions_vec.push_back(func);
        func->release();
      }
    }
    if (group->miss_entry) {
      NS::String *name =
          NS::String::string(group->miss_entry, NS::UTF8StringEncoding);
      MTL::Function *func = lib->library->newFunction(name);
      if (func) {
        visible_functions_vec.push_back(func);
        func->release();
      }
    }
    if (group->intersection_entry) {
      NS::String *name =
          NS::String::string(group->intersection_entry, NS::UTF8StringEncoding);
      MTL::Function *func = lib->library->newFunction(name);
      if (func) {
        visible_functions_vec.push_back(func);
        func->release();
      }
    }
  }

  NS::Array *visible_functions =
      visible_functions_vec.empty()
          ? nullptr
          : NS::Array::array(
                reinterpret_cast<const NS::Object *const *>(
                    visible_functions_vec.data()),
                static_cast<NS::UInteger>(visible_functions_vec.size()));

  for (uint32_t i = 0; i < desc->group_count; i++) {
    const GCRayTracingShaderGroupDesc *group = &desc->groups[i];

    if (group->raygen_entry) {
      NS::String *name =
          NS::String::string(group->raygen_entry, NS::UTF8StringEncoding);
      MTL::Function *func = lib->library->newFunction(name);
      if (func) {
        MTL::ComputePipelineDescriptor *compute_desc =
            MTL::ComputePipelineDescriptor::alloc()->init();
        compute_desc->setComputeFunction(func);
        compute_desc->setMaxCallStackDepth(desc->max_recursion_depth + 1);
        if (visible_functions) {
          MTL::LinkedFunctions *linked = MTL::LinkedFunctions::alloc()->init();
          linked->setFunctions(visible_functions);
          compute_desc->setLinkedFunctions(linked);
          linked->release();
        }

        NS::Error *err = nullptr;
        MTL::ComputePipelineState *state = dev->device->newComputePipelineState(
            compute_desc, MTL::PipelineOption(0), nullptr, &err);
        if (state) {
          states_vec.push_back(state);
          state->release();
        }
        compute_desc->release();
        func->release();
      }
    }
  }

  if (!states_vec.empty()) {
    pipeline->states = NS::Array::array(
        reinterpret_cast<const NS::Object *const *>(states_vec.data()),
        static_cast<NS::UInteger>(states_vec.size()));
    pipeline->states->retain();
  }

  if (visible_functions && visible_functions->count() > 0 &&
      !states_vec.empty()) {
    MTL::VisibleFunctionTableDescriptor *vft_desc =
        MTL::VisibleFunctionTableDescriptor::alloc()->init();
    vft_desc->setFunctionCount(visible_functions->count());
    pipeline->function_table = states_vec[0]->newVisibleFunctionTable(vft_desc);
    vft_desc->release();

    for (size_t j = 0; j < visible_functions->count(); j++) {
      MTL::FunctionHandle *handle = states_vec[0]->functionHandle(
          static_cast<MTL::Function *>(visible_functions->object(j)));
      if (handle) {
        pipeline->function_table->setFunction(handle, j);
        handle->release();
      }
    }
  }

  *out_pipeline = pipeline;
  return ep_ok();
}

GCError GCRayTracingPipelineDestroy(GCRayTracingPipelinePtr pipeline) {
  if (pipeline) {
    GCRayTracingPipeline *pipe = static_cast<GCRayTracingPipeline *>(pipeline);
    if (pipe->states)
      pipe->states->release();
    if (pipe->function_table)
      pipe->function_table->release();
    if (pipe->intersection_table)
      pipe->intersection_table->release();
    free(pipeline);
  }
  return ep_ok();
}

GCError GCAccelerationStructureCreate(GCDevicePtr device,
                                      const GCAccelerationStructureDesc *desc,
                                      GCAccelerationStructurePtr *out_as) {
  if (!device || !desc || !out_as)
    return ep_invalid_argument("device, desc, or out_as is NULL");

  GCDevice *dev = static_cast<GCDevice *>(device);
  if (!dev->device->supportsFamily(MTL::GPUFamilyApple6) &&
      !dev->device->supportsFamily(MTL::GPUFamilyMac2)) {
    return ep_unsupported("ray tracing not supported on this device (requires "
                          "Apple6/Mac2 GPU family)");
  }

  GCAccelerationStructure *as = static_cast<GCAccelerationStructure *>(
      calloc(1, sizeof(GCAccelerationStructure)));
  if (!as)
    return ep_out_of_memory();

  as->ep_device = dev;
  as->top_level = desc->top_level;

  if (desc->top_level) {
    MTL::InstanceAccelerationStructureDescriptor *instance_desc =
        MTL::InstanceAccelerationStructureDescriptor::alloc()->init();
    instance_desc->setUsage(desc->allow_update
                                ? MTL::AccelerationStructureUsageRefit
                                : MTL::AccelerationStructureUsage(0));

    MTL::AccelerationStructureSizes sizes =
        dev->device->accelerationStructureSizes(instance_desc);

    as->accel =
        dev->device->newAccelerationStructure(sizes.accelerationStructureSize);
    as->scratch_buffer = dev->device->newBuffer(
        sizes.buildScratchBufferSize, MTL::ResourceStorageModePrivate);
    instance_desc->release();
  } else {
    MTL::PrimitiveAccelerationStructureDescriptor *prim_desc =
        MTL::PrimitiveAccelerationStructureDescriptor::alloc()->init();

    std::vector<MTL::AccelerationStructureTriangleGeometryDescriptor *>
        geometries_vec;
    for (uint32_t i = 0; i < desc->geometry_count; i++) {
      const GCAccelerationStructureGeometryDesc *geo = &desc->geometries[i];

      MTL::AccelerationStructureTriangleGeometryDescriptor *tri_desc =
          MTL::AccelerationStructureTriangleGeometryDescriptor::alloc()->init();

      if (geo->vertex_buffer) {
        GCBuffer *buf = static_cast<GCBuffer *>(geo->vertex_buffer);
        tri_desc->setVertexBuffer(buf->buffer);
        tri_desc->setVertexBufferOffset(geo->vertex_offset);
        tri_desc->setVertexStride(geo->vertex_stride);
        tri_desc->setTriangleCount(geo->vertex_count / 3);
      }
      if (geo->index_buffer) {
        GCBuffer *buf = static_cast<GCBuffer *>(geo->index_buffer);
        tri_desc->setIndexBuffer(buf->buffer);
        tri_desc->setIndexBufferOffset(geo->index_offset);
        tri_desc->setIndexType(MTL::IndexTypeUInt32);
        tri_desc->setTriangleCount(geo->index_count / 3);
      }
      tri_desc->setOpaque(geo->opaque);
      geometries_vec.push_back(tri_desc);
      tri_desc->release();
    }
    NS::Array *geometries = NS::Array::array(
        reinterpret_cast<const NS::Object *const *>(geometries_vec.data()),
        static_cast<NS::UInteger>(geometries_vec.size()));
    prim_desc->setGeometryDescriptors(geometries);
    prim_desc->setUsage(desc->allow_update
                            ? MTL::AccelerationStructureUsageRefit
                            : MTL::AccelerationStructureUsage(0));

    MTL::AccelerationStructureSizes sizes =
        dev->device->accelerationStructureSizes(prim_desc);

    as->accel =
        dev->device->newAccelerationStructure(sizes.accelerationStructureSize);
    as->scratch_buffer = dev->device->newBuffer(
        sizes.buildScratchBufferSize, MTL::ResourceStorageModePrivate);
    prim_desc->release();
  }

  if (!as->accel) {
    if (as->scratch_buffer)
      as->scratch_buffer->release();
    free(as);
    return ep_out_of_memory();
  }

  *out_as = as;
  return ep_ok();
}

GCError GCAccelerationStructureDestroy(GCAccelerationStructurePtr as) {
  if (as) {
    GCAccelerationStructure *a = static_cast<GCAccelerationStructure *>(as);
    if (a->accel)
      a->accel->release();
    if (a->scratch_buffer)
      a->scratch_buffer->release();
    free(as);
  }
  return ep_ok();
}
}
