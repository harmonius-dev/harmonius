//! Vulkan instance and logical device.

use ash::khr;
use ash::vk;
use ash::vk::Handle;
use ash::{Entry, Instance};
use harmonius_platform::SurfaceHandle;

use super::surface::create_vulkan_surface;
use super::VulkanError;

/// Owns Vulkan instance, device, and queues.
pub struct VulkanContext {
    entry: Entry,
    instance: Instance,
    surface: vk::SurfaceKHR,
    surface_loader: khr::surface::Instance,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    graphics_queue_family: u32,
    present_queue_family: u32,
}

impl VulkanContext {
    /// Create Vulkan objects for windowed rendering.
    pub fn new(surface_handle: &SurfaceHandle) -> Result<Self, VulkanError> {
        let entry = unsafe { Entry::load().map_err(|e| VulkanError::Loader(e.to_string()))? };
        let instance = Self::create_instance(&entry)?;
        let surface_loader = khr::surface::Instance::new(&entry, &instance);
        let surface = create_vulkan_surface(&entry, &instance, &surface_loader, surface_handle)?;
        let (physical_device, graphics_family, present_family) =
            Self::pick_device(&instance, &surface_loader, surface)?;
        let (device, graphics_queue, present_queue) =
            Self::create_device(&instance, physical_device, graphics_family, present_family)?;
        Ok(Self {
            entry,
            instance,
            surface,
            surface_loader,
            physical_device,
            device,
            graphics_queue,
            present_queue,
            graphics_queue_family: graphics_family,
            present_queue_family: present_family,
        })
    }

    /// Headless instance + device for unit tests (no swapchain).
    pub fn new_headless() -> Result<Self, VulkanError> {
        let entry = unsafe { Entry::load().map_err(|e| VulkanError::Loader(e.to_string()))? };
        let instance = Self::create_instance(&entry)?;
        let surface_loader = khr::surface::Instance::new(&entry, &instance);
        let devices = unsafe {
            instance
                .enumerate_physical_devices()
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        let physical_device = devices.into_iter().next().ok_or(VulkanError::NoDevice)?;
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        let graphics_family = queue_families
            .iter()
            .enumerate()
            .find(|(_, q)| q.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|(i, _)| i as u32)
            .ok_or(VulkanError::NoDevice)?;
        let (device, graphics_queue, _) =
            Self::create_device(&instance, physical_device, graphics_family, graphics_family)?;
        Ok(Self {
            entry,
            instance,
            surface: vk::SurfaceKHR::null(),
            surface_loader,
            physical_device,
            device,
            graphics_queue,
            present_queue: graphics_queue,
            graphics_queue_family: graphics_family,
            present_queue_family: graphics_family,
        })
    }

    fn create_instance(entry: &Entry) -> Result<Instance, VulkanError> {
        let app_info = vk::ApplicationInfo::default()
            .application_name(c"Harmonius")
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(c"Harmonius")
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::API_VERSION_1_2);
        let mut extensions = vec![khr::surface::NAME.as_ptr()];
        let mut flags = vk::InstanceCreateFlags::empty();
        #[cfg(target_os = "macos")]
        {
            extensions.push(ash::ext::metal_surface::NAME.as_ptr());
            extensions.push(ash::khr::portability_enumeration::NAME.as_ptr());
            flags |= vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;
        }
        #[cfg(target_os = "linux")]
        {
            extensions.push(ash::khr::xlib_surface::NAME.as_ptr());
        }
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extensions)
            .flags(flags);
        unsafe {
            entry
                .create_instance(&create_info, None)
                .map_err(|e| VulkanError::Api(e.to_string()))
        }
    }

    fn pick_device(
        instance: &Instance,
        surface_loader: &khr::surface::Instance,
        surface: vk::SurfaceKHR,
    ) -> Result<(vk::PhysicalDevice, u32, u32), VulkanError> {
        let devices = unsafe {
            instance
                .enumerate_physical_devices()
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        for device in devices {
            let props = unsafe { instance.get_physical_device_properties(device) };
            #[cfg(target_os = "macos")]
            if props.device_type == vk::PhysicalDeviceType::CPU {
                continue;
            }
            let families = unsafe { instance.get_physical_device_queue_family_properties(device) };
            let graphics = families.iter().enumerate().find_map(|(i, q)| {
                if q.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    Some(i as u32)
                } else {
                    None
                }
            });
            let present = families.iter().enumerate().find_map(|(i, _)| unsafe {
                surface_loader
                    .get_physical_device_surface_support(device, i as u32, surface)
                    .ok()
                    .filter(|supported| *supported)
                    .map(|_| i as u32)
            });
            if let (Some(g), Some(p)) = (graphics, present) {
                return Ok((device, g, p));
            }
        }
        Err(VulkanError::NoDevice)
    }

    fn create_device(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        graphics_family: u32,
        present_family: u32,
    ) -> Result<(ash::Device, vk::Queue, vk::Queue), VulkanError> {
        let mut families = vec![graphics_family];
        if present_family != graphics_family {
            families.push(present_family);
        }
        let priorities = [1.0_f32];
        let mut queue_infos = Vec::new();
        for &family in &families {
            queue_infos.push(
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(family)
                    .queue_priorities(&priorities),
            );
        }
        let mut device_extensions = vec![khr::swapchain::NAME.as_ptr()];
        #[cfg(target_os = "macos")]
        {
            device_extensions.push(ash::khr::portability_subset::NAME.as_ptr());
        }
        let features = vk::PhysicalDeviceFeatures::default();
        let create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&device_extensions)
            .enabled_features(&features);
        let device = unsafe {
            instance
                .create_device(physical_device, &create_info, None)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        let graphics_queue = unsafe { device.get_device_queue(graphics_family, 0) };
        let present_queue = unsafe { device.get_device_queue(present_family, 0) };
        Ok((device, graphics_queue, present_queue))
    }

    #[must_use]
    pub fn entry(&self) -> &Entry {
        &self.entry
    }

    #[must_use]
    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    #[must_use]
    pub fn device(&self) -> &ash::Device {
        &self.device
    }

    #[must_use]
    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }

    #[must_use]
    pub fn surface(&self) -> vk::SurfaceKHR {
        self.surface
    }

    #[must_use]
    pub fn surface_loader(&self) -> &khr::surface::Instance {
        &self.surface_loader
    }

    #[must_use]
    pub fn graphics_queue(&self) -> vk::Queue {
        self.graphics_queue
    }

    #[must_use]
    pub fn present_queue(&self) -> vk::Queue {
        self.present_queue
    }

    #[must_use]
    pub fn graphics_queue_family(&self) -> u32 {
        self.graphics_queue_family
    }

    #[must_use]
    pub fn present_queue_family(&self) -> u32 {
        self.present_queue_family
    }
}

impl Drop for VulkanContext {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            if self.surface.as_raw() != 0 {
                self.surface_loader.destroy_surface(self.surface, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shader::ShaderModule;

    #[test]
    fn test_spirv_shader_module_create() {
        let bytes = include_bytes!("../../../tests/data/empty.vert.spv");
        let spirv: Vec<u32> = bytes
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();
        let ctx = VulkanContext::new_headless().expect("headless Vulkan");
        let module = ShaderModule::from_spirv(&ctx, &spirv).expect("shader module");
        unsafe {
            ctx.device().destroy_shader_module(module.module, None);
        }
    }
}
