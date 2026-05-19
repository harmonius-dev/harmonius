//! Triangle demo Vulkan renderer.

use ash::vk;
use harmonius_platform::SurfaceHandle;

use super::context::VulkanContext;
use super::swapchain::VulkanSwapchain;
use super::VulkanError;
use crate::shader::ShaderModule;

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 3],
}

/// Full Vulkan triangle renderer for bootstrap demo.
pub struct VulkanRenderer {
    context: VulkanContext,
    swapchain: VulkanSwapchain,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
    vertex_buffer: vk::Buffer,
    vertex_memory: vk::DeviceMemory,
    vert_shader: vk::ShaderModule,
    frag_shader: vk::ShaderModule,
}

impl VulkanRenderer {
    /// Initialize Vulkan and the triangle pipeline.
    pub fn new(
        surface: SurfaceHandle,
        vert_spirv: &[u32],
        frag_spirv: &[u32],
    ) -> Result<Self, VulkanError> {
        let context = VulkanContext::new(&surface)?;
        let size = surface.initial_size;
        let swapchain = VulkanSwapchain::new(
            context.instance(),
            context.device(),
            context.physical_device(),
            context.surface(),
            context.surface_loader(),
            size.width,
            size.height,
            context.graphics_queue_family(),
            context.present_queue_family(),
        )?;
        let vert = ShaderModule::from_spirv(&context, vert_spirv)?;
        let frag = ShaderModule::from_spirv(&context, frag_spirv)?;
        let (render_pass, pipeline_layout, pipeline) =
            Self::create_pipeline(&context, &swapchain, vert.module, frag.module)?;
        let framebuffers = Self::create_framebuffers(context.device(), render_pass, &swapchain)?;
        let cmd_count = framebuffers.len().max(2);
        let (command_pool, command_buffers) = Self::create_command_buffers(&context, cmd_count)?;
        let (vertex_buffer, vertex_memory) = Self::create_vertex_buffer(&context)?;
        Ok(Self {
            context,
            swapchain,
            render_pass,
            pipeline_layout,
            pipeline,
            framebuffers,
            command_pool,
            command_buffers,
            vertex_buffer,
            vertex_memory,
            vert_shader: vert.module,
            frag_shader: frag.module,
        })
    }

    #[must_use]
    pub fn context(&self) -> &VulkanContext {
        &self.context
    }

    /// Recreate swapchain after resize.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), VulkanError> {
        unsafe {
            self.context
                .device()
                .device_wait_idle()
                .map_err(|e| VulkanError::Api(e.to_string()))?;
        }
        for &fb in &self.framebuffers {
            unsafe {
                self.context.device().destroy_framebuffer(fb, None);
            }
        }
        self.framebuffers.clear();
        // Old swapchain is destroyed in `Drop` when replaced; do not call
        // `destroy_swapchain_only` here or Vulkan handles are freed twice.
        self.swapchain = VulkanSwapchain::new(
            self.context.instance(),
            self.context.device(),
            self.context.physical_device(),
            self.context.surface(),
            self.context.surface_loader(),
            width,
            height,
            self.context.graphics_queue_family(),
            self.context.present_queue_family(),
        )?;
        self.framebuffers =
            Self::create_framebuffers(self.context.device(), self.render_pass, &self.swapchain)?;
        Ok(())
    }

    /// Record and submit one frame.
    pub fn draw_frame(&mut self) -> Result<(), VulkanError> {
        self.swapchain.wait_frame()?;
        let image_index = self.swapchain.acquire_image()?;
        let cmd = self.command_buffers[image_index as usize % self.command_buffers.len()];
        let extent = self.swapchain.extent();
        unsafe {
            self.context
                .device()
                .reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty())
                .map_err(|e| VulkanError::Api(e.to_string()))?;
        }
        let begin = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            self.context
                .device()
                .begin_command_buffer(cmd, &begin)
                .map_err(|e| VulkanError::Api(e.to_string()))?;
        }
        let clear = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.1, 0.1, 0.12, 1.0],
            },
        };
        let render_pass_info = vk::RenderPassBeginInfo::default()
            .render_pass(self.render_pass)
            .framebuffer(self.framebuffers[image_index as usize])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent,
            })
            .clear_values(std::slice::from_ref(&clear));
        unsafe {
            self.context.device().cmd_begin_render_pass(
                cmd,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );
            self.context.device().cmd_bind_pipeline(
                cmd,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline,
            );
            let vertex_buffers = [self.vertex_buffer];
            let offsets = [0_u64];
            self.context
                .device()
                .cmd_bind_vertex_buffers(cmd, 0, &vertex_buffers, &offsets);
            self.context.device().cmd_set_viewport(
                cmd,
                0,
                &[vk::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: extent.width as f32,
                    height: extent.height as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                }],
            );
            self.context.device().cmd_set_scissor(
                cmd,
                0,
                &[vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent,
                }],
            );
            self.context.device().cmd_draw(cmd, 3, 1, 0, 0);
            self.context.device().cmd_end_render_pass(cmd);
            self.context
                .device()
                .end_command_buffer(cmd)
                .map_err(|e| VulkanError::Api(e.to_string()))?;
        }
        let wait_sems = [self.swapchain.image_available_sem()];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_sems = [self.swapchain.render_finished_sem()];
        let cmd_bufs = [cmd];
        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(&wait_sems)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&cmd_bufs)
            .signal_semaphores(&signal_sems);
        unsafe {
            self.context
                .device()
                .queue_submit(
                    self.context.graphics_queue(),
                    &[submit_info],
                    self.swapchain.in_flight_fence(),
                )
                .map_err(|e| VulkanError::Api(e.to_string()))?;
        }
        self.swapchain.present_image(self.context.present_queue())?;
        Ok(())
    }

    fn create_pipeline(
        context: &VulkanContext,
        swapchain: &VulkanSwapchain,
        vert: vk::ShaderModule,
        frag: vk::ShaderModule,
    ) -> Result<(vk::RenderPass, vk::PipelineLayout, vk::Pipeline), VulkanError> {
        let device = context.device();
        let format = swapchain.format();
        let color_attachment = vk::AttachmentDescription::default()
            .format(format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);
        let color_ref = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(std::slice::from_ref(&color_ref));
        let dependency = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);
        let render_pass_info = vk::RenderPassCreateInfo::default()
            .attachments(std::slice::from_ref(&color_attachment))
            .subpasses(std::slice::from_ref(&subpass))
            .dependencies(std::slice::from_ref(&dependency));
        let render_pass = unsafe {
            device
                .create_render_pass(&render_pass_info, None)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        let layout_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = unsafe {
            device
                .create_pipeline_layout(&layout_info, None)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        let entry = c"main";
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vert)
                .name(entry),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(frag)
                .name(entry),
        ];
        let binding = vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(std::mem::size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX);
        let attrs = [
            vk::VertexInputAttributeDescription::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(0),
            vk::VertexInputAttributeDescription::default()
                .location(1)
                .binding(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(8),
        ];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(std::slice::from_ref(&binding))
            .vertex_attribute_descriptions(&attrs);
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);
        let raster = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);
        let multisample = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA);
        let color_blend = vk::PipelineColorBlendStateCreateInfo::default()
            .attachments(std::slice::from_ref(&color_blend_attachment));
        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&raster)
            .multisample_state(&multisample)
            .color_blend_state(&color_blend)
            .dynamic_state(&dynamic)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0);
        let pipelines = unsafe {
            device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    std::slice::from_ref(&pipeline_info),
                    None,
                )
                .map_err(|e| VulkanError::Api(format!("{:?}", e.1)))?
        };
        let pipeline = pipelines[0];
        Ok((render_pass, pipeline_layout, pipeline))
    }

    fn create_framebuffers(
        device: &ash::Device,
        render_pass: vk::RenderPass,
        swapchain: &VulkanSwapchain,
    ) -> Result<Vec<vk::Framebuffer>, VulkanError> {
        let extent = swapchain.extent();
        swapchain
            .image_views()
            .iter()
            .map(|view| {
                let info = vk::FramebufferCreateInfo::default()
                    .render_pass(render_pass)
                    .attachments(std::slice::from_ref(view))
                    .width(extent.width)
                    .height(extent.height)
                    .layers(1);
                unsafe {
                    device
                        .create_framebuffer(&info, None)
                        .map_err(|e| VulkanError::Api(e.to_string()))
                }
            })
            .collect()
    }

    fn create_command_buffers(
        context: &VulkanContext,
        count: usize,
    ) -> Result<(vk::CommandPool, Vec<vk::CommandBuffer>), VulkanError> {
        let pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(context.graphics_queue_family())
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let pool = unsafe {
            context
                .device()
                .create_command_pool(&pool_info, None)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(count as u32);
        let buffers = unsafe {
            context
                .device()
                .allocate_command_buffers(&alloc_info)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        Ok((pool, buffers))
    }

    fn create_vertex_buffer(
        context: &VulkanContext,
    ) -> Result<(vk::Buffer, vk::DeviceMemory), VulkanError> {
        let vertices = [
            Vertex {
                pos: [0.0, -0.5],
                color: [1.0, 0.2, 0.2],
            },
            Vertex {
                pos: [-0.5, 0.5],
                color: [0.2, 1.0, 0.2],
            },
            Vertex {
                pos: [0.5, 0.5],
                color: [0.2, 0.4, 1.0],
            },
        ];
        let size = (std::mem::size_of_val(&vertices)) as vk::DeviceSize;
        let buffer_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let buffer = unsafe {
            context
                .device()
                .create_buffer(&buffer_info, None)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        let requirements = unsafe { context.device().get_buffer_memory_requirements(buffer) };
        let mem_type = Self::find_memory_type(
            context,
            requirements.memory_type_bits,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;
        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(mem_type);
        let memory = unsafe {
            context
                .device()
                .allocate_memory(&alloc_info, None)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        unsafe {
            context
                .device()
                .bind_buffer_memory(buffer, memory, 0)
                .map_err(|e| VulkanError::Api(e.to_string()))?;
            let data = context
                .device()
                .map_memory(memory, 0, size, vk::MemoryMapFlags::empty())
                .map_err(|e| VulkanError::Api(e.to_string()))?;
            std::ptr::copy_nonoverlapping(
                vertices.as_ptr().cast::<std::ffi::c_void>(),
                data.cast(),
                std::mem::size_of_val(&vertices),
            );
            context.device().unmap_memory(memory);
        }
        Ok((buffer, memory))
    }

    fn find_memory_type(
        context: &VulkanContext,
        type_filter: u32,
        properties: vk::MemoryPropertyFlags,
    ) -> Result<u32, VulkanError> {
        let mem_props = unsafe {
            context
                .instance()
                .get_physical_device_memory_properties(context.physical_device())
        };
        for i in 0..mem_props.memory_type_count {
            if (type_filter & (1 << i)) != 0
                && mem_props.memory_types[i as usize]
                    .property_flags
                    .contains(properties)
            {
                return Ok(i);
            }
        }
        Err(VulkanError::Api("suitable memory type not found".into()))
    }
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        let device = self.context.device();
        unsafe {
            let _ = device.device_wait_idle();
            device.destroy_buffer(self.vertex_buffer, None);
            device.free_memory(self.vertex_memory, None);
            device.destroy_shader_module(self.vert_shader, None);
            device.destroy_shader_module(self.frag_shader, None);
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_render_pass(self.render_pass, None);
            for &fb in &self.framebuffers {
                device.destroy_framebuffer(fb, None);
            }
            device.destroy_command_pool(self.command_pool, None);
        }
    }
}
