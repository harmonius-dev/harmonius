//! Vulkan swapchain management.

use ash::khr;
use ash::{vk, Device, Instance};

use super::VulkanError;

/// Swapchain images and synchronization primitives.
pub struct VulkanSwapchain {
    device: Device,
    swapchain_loader: khr::swapchain::Device,
    swapchain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
    format: vk::Format,
    extent: vk::Extent2D,
    image_available: Vec<vk::Semaphore>,
    render_finished: Vec<vk::Semaphore>,
    in_flight: Vec<vk::Fence>,
    frame_index: usize,
    max_frames: usize,
    current_image: u32,
}

impl VulkanSwapchain {
    /// Create a swapchain for `surface` at `width` x `height`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instance: &Instance,
        device: &Device,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        surface_loader: &khr::surface::Instance,
        width: u32,
        height: u32,
        graphics_family: u32,
        present_family: u32,
    ) -> Result<Self, VulkanError> {
        let swapchain_loader = khr::swapchain::Device::new(instance, device);
        let capabilities = unsafe {
            surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        let formats = unsafe {
            surface_loader
                .get_physical_device_surface_formats(physical_device, surface)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        let present_modes = unsafe {
            surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        let surface_format = formats
            .iter()
            .find(|f| {
                f.format == vk::Format::B8G8R8A8_SRGB
                    && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .or_else(|| formats.first())
            .ok_or_else(|| VulkanError::Api("no surface formats".into()))?;
        let format = surface_format.format;
        let present_mode = present_modes
            .iter()
            .copied()
            .find(|m| *m == vk::PresentModeKHR::FIFO)
            .unwrap_or(vk::PresentModeKHR::FIFO);
        let extent = if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            vk::Extent2D {
                width: width.clamp(
                    capabilities.min_image_extent.width,
                    capabilities.max_image_extent.width,
                ),
                height: height.clamp(
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height,
                ),
            }
        };
        let image_count =
            (capabilities.min_image_count + 1).min(if capabilities.max_image_count > 0 {
                capabilities.max_image_count
            } else {
                u32::MAX
            });
        let queue_family_indices = [graphics_family, present_family];
        let sharing = if graphics_family != present_family {
            vk::SharingMode::CONCURRENT
        } else {
            vk::SharingMode::EXCLUSIVE
        };
        let create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(sharing)
            .queue_family_indices(&queue_family_indices)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);
        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&create_info, None)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        let images = unsafe {
            swapchain_loader
                .get_swapchain_images(swapchain)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        let image_views: Result<Vec<_>, VulkanError> = images
            .iter()
            .map(|image| {
                let view_info = vk::ImageViewCreateInfo::default()
                    .image(*image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(format)
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    });
                unsafe {
                    device
                        .create_image_view(&view_info, None)
                        .map_err(|e| VulkanError::Api(e.to_string()))
                }
            })
            .collect();
        let image_views = image_views?;
        let max_frames = 2;
        let mut image_available = Vec::with_capacity(max_frames);
        let mut render_finished = Vec::with_capacity(max_frames);
        let mut in_flight = Vec::with_capacity(max_frames);
        for _ in 0..max_frames {
            let sem_info = vk::SemaphoreCreateInfo::default();
            let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
            image_available.push(unsafe {
                device
                    .create_semaphore(&sem_info, None)
                    .map_err(|e| VulkanError::Api(e.to_string()))?
            });
            render_finished.push(unsafe {
                device
                    .create_semaphore(&sem_info, None)
                    .map_err(|e| VulkanError::Api(e.to_string()))?
            });
            in_flight.push(unsafe {
                device
                    .create_fence(&fence_info, None)
                    .map_err(|e| VulkanError::Api(e.to_string()))?
            });
        }
        Ok(Self {
            device: device.clone(),
            swapchain_loader,
            swapchain,
            images,
            image_views,
            format,
            extent,
            image_available,
            render_finished,
            in_flight,
            frame_index: 0,
            max_frames,
            current_image: 0,
        })
    }

    #[must_use]
    pub fn format(&self) -> vk::Format {
        self.format
    }

    pub fn extent(&self) -> vk::Extent2D {
        self.extent
    }

    #[must_use]
    pub fn image_views(&self) -> &[vk::ImageView] {
        &self.image_views
    }

    #[must_use]
    pub fn loader(&self) -> &khr::swapchain::Device {
        &self.swapchain_loader
    }

    #[must_use]
    pub fn handle(&self) -> vk::SwapchainKHR {
        self.swapchain
    }

    #[must_use]
    pub fn frame_index(&self) -> usize {
        self.frame_index
    }

    #[must_use]
    pub fn image_available_sem(&self) -> vk::Semaphore {
        self.image_available[self.frame_index]
    }

    #[must_use]
    pub fn render_finished_sem(&self) -> vk::Semaphore {
        self.render_finished[self.frame_index]
    }

    #[must_use]
    pub fn in_flight_fence(&self) -> vk::Fence {
        self.in_flight[self.frame_index]
    }

    /// Wait for the current frame fence.
    pub fn wait_frame(&mut self) -> Result<(), VulkanError> {
        let fence = self.in_flight[self.frame_index];
        unsafe {
            self.device
                .wait_for_fences(&[fence], true, u64::MAX)
                .map_err(|e| VulkanError::Api(e.to_string()))?;
            self.device
                .reset_fences(&[fence])
                .map_err(|e| VulkanError::Api(e.to_string()))?;
        }
        Ok(())
    }

    /// Acquire the next swapchain image.
    pub fn acquire_image(&mut self) -> Result<u32, VulkanError> {
        match unsafe {
            self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available[self.frame_index],
                vk::Fence::null(),
            )
        } {
            Ok((image_index, _)) => {
                self.current_image = image_index;
                Ok(image_index)
            }
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) | Err(vk::Result::SUBOPTIMAL_KHR) => {
                Err(VulkanError::Api("swapchain out of date".into()))
            }
            Err(e) => Err(VulkanError::Api(e.to_string())),
        }
    }

    /// Present the last acquired image.
    pub fn present_image(&mut self, present_queue: vk::Queue) -> Result<(), VulkanError> {
        let wait_sems = [self.render_finished[self.frame_index]];
        let swapchains = [self.swapchain];
        let indices = [self.current_image];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&wait_sems)
            .swapchains(&swapchains)
            .image_indices(&indices);
        match unsafe {
            self.swapchain_loader
                .queue_present(present_queue, &present_info)
        } {
            Ok(_) | Err(vk::Result::ERROR_OUT_OF_DATE_KHR) | Err(vk::Result::SUBOPTIMAL_KHR) => {}
            Err(e) => return Err(VulkanError::Api(e.to_string())),
        }
        self.frame_index = (self.frame_index + 1) % self.max_frames;
        Ok(())
    }

    /// Destroy swapchain images and views (keeps sync objects).
    pub fn destroy_swapchain_only(&mut self) {
        unsafe {
            for &view in &self.image_views {
                self.device.destroy_image_view(view, None);
            }
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }
        self.image_views.clear();
        self.images.clear();
    }
}

impl Drop for VulkanSwapchain {
    fn drop(&mut self) {
        unsafe {
            for &sem in &self.image_available {
                self.device.destroy_semaphore(sem, None);
            }
            for &sem in &self.render_finished {
                self.device.destroy_semaphore(sem, None);
            }
            for &fence in &self.in_flight {
                self.device.destroy_fence(fence, None);
            }
            self.destroy_swapchain_only();
        }
    }
}
