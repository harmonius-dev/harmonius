//! Minimal Vulkan triangle renderer (bootstrap).

use std::ffi::CStr;

use ash::vk;
use ash::{Device, Entry, Instance};
use bytemuck::{Pod, Zeroable};
use harmonius_core::EngineError;
use raw_window_handle::{
    HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
};

use crate::shader::create_shader_module;

const MAX_FRAMES_IN_FLIGHT: usize = 2;

/// Colored vertex for the bootstrap triangle.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 3],
}

const TRIANGLE_VERTICES: [Vertex; 3] = [
    Vertex {
        pos: [0.0, -0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        pos: [0.5, 0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        pos: [-0.5, 0.5],
        color: [0.0, 0.0, 1.0],
    },
];

/// Vulkan swapchain triangle demo.
pub struct VulkanTriangle {
    entry: Entry,
    instance: Instance,
    surface_loader: ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
    device: Device,
    graphics_queue: vk::Queue,
    swapchain_loader: ash::khr::swapchain::Device,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    image_available: Vec<vk::Semaphore>,
    render_finished: Vec<vk::Semaphore>,
    in_flight: Vec<vk::Fence>,
    current_frame: usize,
}

impl VulkanTriangle {
    /// Creates renderer state for the given surface and compiled SPIR-V shaders.
    pub unsafe fn new(
        window: &impl HasWindowHandle,
        display: &impl HasDisplayHandle,
        width: u32,
        height: u32,
        vert_spv: &[u8],
        frag_spv: &[u8],
    ) -> Result<Self, EngineError> {
        let entry = Entry::load().map_err(|e| EngineError::Gpu(format!("Entry::load: {e}")))?;
        let (instance, surface_loader, surface) =
            create_instance_and_surface(&entry, window, display)?;

        let (physical_device, queue_family) =
            pick_physical_device(&instance, surface)?;
        let (device, graphics_queue) =
            create_logical_device(&instance, physical_device, queue_family)?;

        let mut renderer = Self {
            entry,
            instance,
            surface_loader,
            surface,
            physical_device,
            device,
            graphics_queue,
            swapchain_loader: ash::khr::swapchain::Device::new(&instance, &device),
            swapchain: vk::SwapchainKHR::null(),
            swapchain_images: Vec::new(),
            swapchain_format: vk::Format::UNDEFINED,
            swapchain_extent: vk::Extent2D::default(),
            render_pass: vk::RenderPass::null(),
            pipeline_layout: vk::PipelineLayout::null(),
            pipeline: vk::Pipeline::null(),
            framebuffers: Vec::new(),
            command_pool: vk::CommandPool::null(),
            command_buffers: Vec::new(),
            vertex_buffer: vk::Buffer::null(),
            vertex_buffer_memory: vk::DeviceMemory::null(),
            image_available: Vec::new(),
            render_finished: Vec::new(),
            in_flight: Vec::new(),
            current_frame: 0,
        };

        renderer.recreate_swapchain(width, height)?;
        renderer.create_pipeline(vert_spv, frag_spv)?;
        renderer.create_vertex_buffer()?;
        renderer.create_sync_objects()?;
        renderer.record_command_buffers()?;
        Ok(renderer)
    }

    /// Recreates swapchain and dependent resources after a resize.
    pub unsafe fn resize(&mut self, width: u32, height: u32) -> Result<(), EngineError> {
        self.wait_idle()?;
        self.cleanup_swapchain();
        self.recreate_swapchain(width, height)?;
        self.record_command_buffers()?;
        Ok(())
    }

    /// Renders one frame. Returns `true` when successful.
    pub unsafe fn draw_frame(&mut self) -> Result<bool, EngineError> {
        let frame = self.current_frame;
        self.device
            .wait_for_fences(&[self.in_flight[frame]], true, u64::MAX)
            .map_err(|e| EngineError::Gpu(format!("wait_for_fences: {e}")))?;

        let image_index = match self.swapchain_loader.acquire_next_image(
            self.swapchain,
            u64::MAX,
            self.image_available[frame],
            vk::Fence::null(),
        ) {
            Ok(index) => index,
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) | Err(vk::Result::SUBOPTIMAL_KHR) => {
                return Ok(false);
            }
            Err(e) => {
                return Err(EngineError::Gpu(format!("acquire_next_image: {e}")));
            }
        };

        self.device
            .reset_fences(&[self.in_flight[frame]])
            .map_err(|e| EngineError::Gpu(format!("reset_fences: {e}")))?;

        let cmd = self.command_buffers[frame];
        self.device
            .reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty())
            .map_err(|e| EngineError::Gpu(format!("reset_command_buffer: {e}")))?;

        let begin = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        self.device
            .begin_command_buffer(cmd, &begin)
            .map_err(|e| EngineError::Gpu(format!("begin_command_buffer: {e}")))?;

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
                extent: self.swapchain_extent,
            })
            .clear_values(&[clear]);

        self.device.cmd_begin_render_pass(cmd, &render_pass_info, vk::SubpassContents::INLINE);
        self.device.cmd_bind_pipeline(
            cmd,
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline,
        );
        let vertex_buffers = [self.vertex_buffer];
        let offsets = [0_u64];
        self.device
            .cmd_bind_vertex_buffers(cmd, 0, &vertex_buffers, &offsets);
        self.device.cmd_draw(cmd, 3, 1, 0, 0);
        self.device.cmd_end_render_pass(cmd);

        self.device
            .end_command_buffer(cmd)
            .map_err(|e| EngineError::Gpu(format!("end_command_buffer: {e}")))?;

        let wait_sems = [self.image_available[frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_sems = [self.render_finished[frame]];
        let cmd_bufs = [cmd];
        let submit = vk::SubmitInfo::default()
            .wait_semaphores(&wait_sems)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&cmd_bufs)
            .signal_semaphores(&signal_sems);

        self.device
            .queue_submit(self.graphics_queue, &[submit], self.in_flight[frame])
            .map_err(|e| EngineError::Gpu(format!("queue_submit: {e}")))?;

        let swapchains = [self.swapchain];
        let indices = [image_index];
        let present = vk::PresentInfoKHR::default()
            .wait_semaphores(&signal_sems)
            .swapchains(&swapchains)
            .image_indices(&indices);

        match self.swapchain_loader.queue_present(self.graphics_queue, &present) {
            Ok(true) => {}
            Ok(false) | Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => return Ok(false),
            Err(e) => return Err(EngineError::Gpu(format!("queue_present: {e}"))),
        }

        self.current_frame = (frame + 1) % MAX_FRAMES_IN_FLIGHT;
        Ok(true)
    }

    unsafe fn wait_idle(&self) -> Result<(), EngineError> {
        self.device
            .device_wait_idle()
            .map_err(|e| EngineError::Gpu(format!("device_wait_idle: {e}")))
    }

    unsafe fn recreate_swapchain(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<(), EngineError> {
        let caps = self
            .surface_loader
            .get_physical_device_surface_capabilities(self.physical_device, self.surface)
            .map_err(|e| EngineError::Gpu(format!("surface capabilities: {e}")))?;

        let formats = self
            .surface_loader
            .get_physical_device_surface_formats(self.physical_device, self.surface)
            .map_err(|e| EngineError::Gpu(format!("surface formats: {e}")))?;

        let format = formats
            .iter()
            .find(|f| {
                f.format == vk::Format::B8G8R8A8_SRGB
                    && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .copied()
            .unwrap_or(formats[0]);

        let extent = if caps.current_extent.width != u32::MAX {
            caps.current_extent
        } else {
            vk::Extent2D {
                width: width.clamp(caps.min_image_extent.width, caps.max_image_extent.width),
                height: height.clamp(caps.min_image_extent.height, caps.max_image_extent.height),
            }
        };

        let image_count = (caps.min_image_count + 1).min(caps.max_image_count);

        let create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(self.surface)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(caps.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO)
            .clipped(true);

        self.swapchain = self
            .swapchain_loader
            .create_swapchain(&create_info, None)
            .map_err(|e| EngineError::Gpu(format!("create_swapchain: {e}")))?;

        self.swapchain_images = self
            .swapchain_loader
            .get_swapchain_images(self.swapchain)
            .map_err(|e| EngineError::Gpu(format!("get_swapchain_images: {e}")))?;

        self.swapchain_format = format.format;
        self.swapchain_extent = extent;

        self.create_render_pass()?;
        self.create_framebuffers()?;
        self.create_command_pool_and_buffers()?;
        Ok(())
    }

    unsafe fn create_render_pass(&mut self) -> Result<(), EngineError> {
        let color_attachment = vk::AttachmentDescription::default()
            .format(self.swapchain_format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .initial_layout(vk::InitialLayout::UNDEFINED)
            .final_layout(vk::FinalLayout::PRESENT_SRC_KHR);

        let color_ref = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&[color_ref]);

        let dependency = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

        let render_pass_info = vk::RenderPassCreateInfo::default()
            .attachments(&[color_attachment])
            .subpasses(&[subpass])
            .dependencies(&[dependency]);

        self.render_pass = self
            .device
            .create_render_pass(&render_pass_info, None)
            .map_err(|e| EngineError::Gpu(format!("create_render_pass: {e}")))?;
        Ok(())
    }

    unsafe fn create_framebuffers(&mut self) -> Result<(), EngineError> {
        let mut framebuffers = Vec::with_capacity(self.swapchain_images.len());
        for &image in &self.swapchain_images {
            let view_info = vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(self.swapchain_format)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });
            let view = self
                .device
                .create_image_view(&view_info, None)
                .map_err(|e| EngineError::Gpu(format!("create_image_view: {e}")))?;

            let fb_info = vk::FramebufferCreateInfo::default()
                .render_pass(self.render_pass)
                .attachments(&[view])
                .width(self.swapchain_extent.width)
                .height(self.swapchain_extent.height)
                .layers(1);

            let fb = self
                .device
                .create_framebuffer(&fb_info, None)
                .map_err(|e| EngineError::Gpu(format!("create_framebuffer: {e}")))?;
            framebuffers.push(fb);
        }
        self.framebuffers = framebuffers;
        Ok(())
    }

    unsafe fn create_pipeline(
        &mut self,
        vert_spv: &[u8],
        frag_spv: &[u8],
    ) -> Result<(), EngineError> {
        let vert_module = create_shader_module(&self.device, vert_spv)?;
        let frag_module = create_shader_module(&self.device, frag_spv)?;

        let entry_point = c"main";
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vert_module)
                .name(entry_point),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(frag_module)
                .name(entry_point),
        ];

        let binding = vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(std::mem::size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX);

        let attributes = [
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(0),
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(8),
        ];

        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&[binding])
            .vertex_attribute_descriptions(&attributes);

        let input_assembly =
            vk::PipelineInputAssemblyStateCreateInfo::default()
                .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

        let viewport = vk::Viewport::default()
            .width(self.swapchain_extent.width as f32)
            .height(self.swapchain_extent.height as f32)
            .max_depth(1.0);
        let scissor = vk::Rect2D {
            extent: self.swapchain_extent,
            ..Default::default()
        };
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&[viewport])
            .scissors(&[scissor]);

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);

        let multisample = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA);

        let color_blend = vk::PipelineColorBlendStateCreateInfo::default()
            .attachments(&[color_blend_attachment]);

        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic = vk::PipelineDynamicStateCreateInfo::default()
            .dynamic_states(&dynamic_states);

        let layout_info = vk::PipelineLayoutCreateInfo::default();
        self.pipeline_layout = self
            .device
            .create_pipeline_layout(&layout_info, None)
            .map_err(|e| EngineError::Gpu(format!("create_pipeline_layout: {e}")))?;

        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisample)
            .color_blend_state(&color_blend)
            .dynamic_state(&dynamic)
            .layout(self.pipeline_layout)
            .render_pass(self.render_pass)
            .subpass(0);

        self.pipeline = self
            .device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
            .map_err(|(_, e)| EngineError::Gpu(format!("create_graphics_pipelines: {e}")))?[0];

        self.device.destroy_shader_module(vert_module, None);
        self.device.destroy_shader_module(frag_module, None);
        Ok(())
    }

    unsafe fn create_vertex_buffer(&mut self) -> Result<(), EngineError> {
        let size = (TRIANGLE_VERTICES.len() * std::mem::size_of::<Vertex>()) as u64;
        let buffer_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        self.vertex_buffer = self
            .device
            .create_buffer(&buffer_info, None)
            .map_err(|e| EngineError::Gpu(format!("create_buffer: {e}")))?;

        let requirements = self.device.get_buffer_memory_requirements(self.vertex_buffer);
        let memory_type = find_memory_type(
            &self.instance,
            self.physical_device,
            requirements.memory_type_bits,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(memory_type);

        self.vertex_buffer_memory = self
            .device
            .allocate_memory(&alloc_info, None)
            .map_err(|e| EngineError::Gpu(format!("allocate_memory: {e}")))?;

        self.device
            .bind_buffer_memory(self.vertex_buffer, self.vertex_buffer_memory, 0)
            .map_err(|e| EngineError::Gpu(format!("bind_buffer_memory: {e}")))?;

        let data = self
            .device
            .map_memory(
                self.vertex_buffer_memory,
                0,
                size,
                vk::MemoryMapFlags::empty(),
            )
            .map_err(|e| EngineError::Gpu(format!("map_memory: {e}")))? as *mut Vertex;

        std::ptr::copy_nonoverlapping(
            TRIANGLE_VERTICES.as_ptr(),
            data,
            TRIANGLE_VERTICES.len(),
        );
        self.device.unmap_memory(self.vertex_buffer_memory);
        Ok(())
    }

    unsafe fn create_command_pool_and_buffers(&mut self) -> Result<(), EngineError> {
        let pool_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        self.command_pool = self
            .device
            .create_command_pool(&pool_info, None)
            .map_err(|e| EngineError::Gpu(format!("create_command_pool: {e}")))?;

        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(MAX_FRAMES_IN_FLIGHT as u32);

        self.command_buffers = self
            .device
            .allocate_command_buffers(&alloc_info)
            .map_err(|e| EngineError::Gpu(format!("allocate_command_buffers: {e}")))?;
        Ok(())
    }

    unsafe fn record_command_buffers(&mut self) -> Result<(), EngineError> {
        // Command buffers are recorded per-frame in draw_frame for simplicity.
        Ok(())
    }

    unsafe fn create_sync_objects(&mut self) -> Result<(), EngineError> {
        let sem_info = vk::SemaphoreCreateInfo::default();
        let fence_info =
            vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            self.image_available.push(
                self.device
                    .create_semaphore(&sem_info, None)
                    .map_err(|e| EngineError::Gpu(format!("create_semaphore: {e}")))?,
            );
            self.render_finished.push(
                self.device
                    .create_semaphore(&sem_info, None)
                    .map_err(|e| EngineError::Gpu(format!("create_semaphore: {e}")))?,
            );
            self.in_flight.push(
                self.device
                    .create_fence(&fence_info, None)
                    .map_err(|e| EngineError::Gpu(format!("create_fence: {e}")))?,
            );
        }
        Ok(())
    }

    unsafe fn cleanup_swapchain(&mut self) {
        for &fb in &self.framebuffers {
            self.device.destroy_framebuffer(fb, None);
        }
        self.framebuffers.clear();
        if self.render_pass != vk::RenderPass::null() {
            self.device.destroy_render_pass(self.render_pass, None);
            self.render_pass = vk::RenderPass::null();
        }
        if self.swapchain != vk::SwapchainKHR::null() {
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
            self.swapchain = vk::SwapchainKHR::null();
        }
    }
}

impl Drop for VulkanTriangle {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.device_wait_idle();
            self.cleanup_swapchain();
            if self.pipeline != vk::Pipeline::null() {
                self.device.destroy_pipeline(self.pipeline, None);
            }
            if self.pipeline_layout != vk::PipelineLayout::null() {
                self.device
                    .destroy_pipeline_layout(self.pipeline_layout, None);
            }
            self.device.destroy_buffer(self.vertex_buffer, None);
            self.device
                .free_memory(self.vertex_buffer_memory, None);
            for &s in &self.image_available {
                self.device.destroy_semaphore(s, None);
            }
            for &s in &self.render_finished {
                self.device.destroy_semaphore(s, None);
            }
            for &f in &self.in_flight {
                self.device.destroy_fence(f, None);
            }
            if self.command_pool != vk::CommandPool::null() {
                self.device.destroy_command_pool(self.command_pool, None);
            }
            self.surface_loader.destroy_surface(self.surface, None);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

unsafe fn create_instance_and_surface(
    entry: &Entry,
    window: &impl HasWindowHandle,
    display: &impl HasDisplayHandle,
) -> Result<(Instance, ash::khr::surface::Instance, vk::SurfaceKHR), EngineError> {
    let mut extensions = vec![ash::khr::surface::NAME.as_ptr()];

    #[cfg(target_os = "macos")]
    {
        extensions.push(ash::khr::portability_enumeration::NAME.as_ptr());
        extensions.push(ash::ext::metal_surface::NAME.as_ptr());
    }
    #[cfg(target_os = "linux")]
    {
        extensions.push(ash::khr::xlib_surface::NAME.as_ptr());
    }

    let app_info = vk::ApplicationInfo::default()
        .application_name(c"harmonius")
        .api_version(vk::make_api_version(0, 1, 3, 0));

    let mut create_flags = vk::InstanceCreateFlags::empty();
    #[cfg(target_os = "macos")]
    {
        create_flags |=
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_EXTENSIONS_KHR;
    }

    let create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .flags(create_flags)
        .enabled_extension_names(&extensions);

    let instance = entry
        .create_instance(&create_info, None)
        .map_err(|e| EngineError::Gpu(format!("create_instance: {e}")))?;

    let surface_loader = ash::khr::surface::Instance::new(entry, &instance);
    let surface = create_platform_surface(&instance, window, display)?;
    Ok((instance, surface_loader, surface))
}

unsafe fn create_platform_surface(
    instance: &Instance,
    window: &impl HasWindowHandle,
    display: &impl HasDisplayHandle,
) -> Result<vk::SurfaceKHR, EngineError> {
    let window_handle = window
        .window_handle()
        .map_err(|e| EngineError::Gpu(format!("window handle: {e}")))?;
    let display_handle = display
        .display_handle()
        .map_err(|e| EngineError::Gpu(format!("display handle: {e}")))?;

    #[cfg(target_os = "linux")]
    {
        let raw = window_handle.as_raw();
        let RawWindowHandle::Xlib(xlib) = raw else {
            return Err(EngineError::Gpu("expected Xlib window handle".into()));
        };
        let display = display_handle.as_raw();
        let RawDisplayHandle::Xlib(xlib_display) = display else {
            return Err(EngineError::Gpu("expected Xlib display handle".into()));
        };
        let create_info = vk::XlibSurfaceCreateInfoKHR::default()
            .dpy(xlib_display.display.as_ptr())
            .window(xlib.window.get());
        let loader = ash::khr::xlib_surface::Instance::new(
            &ash::Entry::load().unwrap(),
            instance,
        );
        return loader
            .create_xlib_surface(&create_info, None)
            .map_err(|e| EngineError::Gpu(format!("create_xlib_surface: {e}")));
    }

    #[cfg(target_os = "macos")]
    {
        use std::ptr::NonNull;

        use objc2::runtime::AnyObject;
        use objc2_quartz_core::CAMetalLayer;
        use raw_window_handle::AppKitWindowHandle;

        let raw = window_handle.as_raw();
        let RawWindowHandle::AppKit(appkit) = raw else {
            return Err(EngineError::Gpu("expected AppKit window handle".into()));
        };
        let AppKitWindowHandle { ns_view } = appkit;
        let view: &AnyObject = unsafe { ns_view.as_ref() };
        let layer = CAMetalLayer::new();
        let _: () = unsafe {
            objc2::msg_send![view, setWantsLayer: true];
            objc2::msg_send![view, setLayer: &*layer];
        };
        let create_info = vk::MetalSurfaceCreateInfoEXT::default().p_layer(
            NonNull::new(layer.as_ptr() as *mut _).expect("metal layer"),
        );
        let loader = ash::ext::metal_surface::Instance::new(
            &ash::Entry::load().unwrap(),
            instance,
        );
        return loader
            .create_metal_surface(&create_info, None)
            .map_err(|e| EngineError::Gpu(format!("create_metal_surface: {e}")));
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        let _ = (window_handle, display_handle);
        Err(EngineError::Gpu("unsupported platform".into()))
    }
}

fn pick_physical_device(
    instance: &Instance,
    surface: vk::SurfaceKHR,
) -> Result<(vk::PhysicalDevice, u32), EngineError> {
    let surface_loader = ash::khr::surface::Instance::new(
        &ash::Entry::load().map_err(|e| EngineError::Gpu(e.to_string()))?,
        instance,
    );
    let devices = unsafe { instance.enumerate_physical_devices() }
        .map_err(|e| EngineError::Gpu(format!("enumerate_physical_devices: {e}")))?;

    for pd in devices {
        let props = unsafe { instance.get_physical_device_properties(pd) };
        let name = unsafe { CStr::from_ptr(props.device_name.as_ptr()) }
            .to_string_lossy();
        let extensions = unsafe { instance.enumerate_device_extension_properties(pd) }
            .map_err(|e| EngineError::Gpu(format!("device extensions: {e}")))?;
        let has_swapchain = extensions.iter().any(|ext| {
            let name = unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) };
            name.to_bytes() == b"VK_KHR_swapchain"
        });
        if !has_swapchain {
            continue;
        }
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(pd) };
        let graphics = queue_families
            .iter()
            .enumerate()
            .find(|(_, q)| {
                q.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    && surface_loader
                        .get_physical_device_surface_support(
                            pd,
                            _ as u32,
                            surface,
                        )
                        .unwrap_or(false)
            })
            .map(|(i, _)| i as u32);
        if let Some(family) = graphics {
            eprintln!("Using GPU: {name}");
            return Ok((pd, family));
        }
    }
    Err(EngineError::Gpu("no suitable Vulkan device".into()))
}

unsafe fn create_logical_device(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
) -> Result<(Device, vk::Queue), EngineError> {
    let queue_info = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family)
        .queue_priorities(&[1.0])];

    let device_extensions = [ash::khr::swapchain::NAME.as_ptr()];

    #[cfg(target_os = "macos")]
    let device_extensions = [
        ash::khr::swapchain::NAME.as_ptr(),
        ash::khr::portability_subset::NAME.as_ptr(),
    ];

    let create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_info)
        .enabled_extension_names(&device_extensions);

    let device = instance
        .create_device(physical_device, &create_info, None)
        .map_err(|e| EngineError::Gpu(format!("create_device: {e}")))?;

    let queue = device.get_device_queue(queue_family, 0);
    Ok((device, queue))
}

fn find_memory_type(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
) -> Result<u32, EngineError> {
    let mem_props = unsafe { instance.get_physical_device_memory_properties(physical_device) };
    for (i, mem_type) in mem_props.memory_types.iter().enumerate() {
        if (type_filter & (1 << i)) != 0
            && mem_type.property_flags.contains(properties)
        {
            return Ok(i as u32);
        }
    }
    Err(EngineError::Gpu("failed to find suitable memory type".into()))
}
