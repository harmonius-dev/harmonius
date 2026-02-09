//! Shared test utilities for creating a real Vulkan context.
//!
//! This module is only compiled in `#[cfg(test)]` mode and provides a
//! [`TestVulkanContext`] that initializes a real Vulkan instance, device,
//! and VMA allocator for integration testing.

use ash::vk;
use std::mem::ManuallyDrop;

/// A Vulkan context for integration tests.
///
/// Creates a real Vulkan 1.3 instance, picks the first suitable physical
/// device, and creates a logical device with a graphics queue and a VMA
/// allocator. Cleans up everything on drop.
pub struct TestVulkanContext {
    /// VMA allocator. Wrapped in ManuallyDrop so we can destroy it before
    /// the device in our Drop impl.
    pub allocator: ManuallyDrop<vk_mem::Allocator>,
    pub device: ash::Device,
    pub instance: ash::Instance,
    #[allow(dead_code)]
    pub entry: ash::Entry,
    pub physical_device: vk::PhysicalDevice,
    pub graphics_queue: vk::Queue,
    pub graphics_family: u32,
}

impl TestVulkanContext {
    /// Try to create a Vulkan test context. Returns `None` if Vulkan is
    /// unavailable (no driver, no GPU, etc.).
    pub fn try_new() -> Option<Self> {
        // SAFETY: Test harness. Vulkan loader contract; entry is used to create instance only.
        let entry = unsafe { ash::Entry::load() }.ok()?;

        // Create instance with Vulkan 1.3.
        let app_info = vk::ApplicationInfo::default()
            .application_name(c"gcraft-gfx-tests")
            .application_version(vk::make_api_version(0, 0, 1, 0))
            .engine_name(c"gcraft")
            .engine_version(vk::make_api_version(0, 0, 1, 0))
            .api_version(vk::API_VERSION_1_3);

        let create_info = vk::InstanceCreateInfo::default().application_info(&app_info);

        // SAFETY: Test harness. Valid create_info; instance destroyed on drop or on error path.
        let instance = unsafe { entry.create_instance(&create_info, None) }.ok()?;

        // Pick first physical device that supports Vulkan 1.3.
        // SAFETY: Test harness. Instance valid; enumerate returns owned handles.
        let physical_devices = unsafe { instance.enumerate_physical_devices() }.ok()?;
        if physical_devices.is_empty() {
            // SAFETY: Instance valid; cleanup on early return.
            unsafe { instance.destroy_instance(None) };
            return None;
        }

        // Prefer a discrete GPU, fall back to any 1.3-capable device.
        // SAFETY: Test harness. Instance and pd valid; read-only property query.
        let physical_device = physical_devices
            .iter()
            .find(|&&pd| {
                let props = unsafe { instance.get_physical_device_properties(pd) };
                props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
                    && props.api_version >= vk::API_VERSION_1_3
            })
            .or_else(|| {
                physical_devices.iter().find(|&&pd| {
                    let props = unsafe { instance.get_physical_device_properties(pd) };
                    props.api_version >= vk::API_VERSION_1_3
                })
            })
            .copied();

        let physical_device = match physical_device {
            Some(pd) => pd,
            None => {
                // SAFETY: Instance valid; cleanup on early return.
                unsafe { instance.destroy_instance(None) };
                return None;
            }
        };

        // Find graphics queue family.
        // SAFETY: Test harness. Instance and physical_device valid; read-only query.
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let graphics_family = queue_families
            .iter()
            .enumerate()
            .find(|(_, props)| props.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|(i, _)| i as u32);

        let graphics_family = match graphics_family {
            Some(f) => f,
            None => {
                // SAFETY: Instance valid; cleanup on early return.
                unsafe { instance.destroy_instance(None) };
                return None;
            }
        };

        // Create logical device with Vulkan 1.3 features.
        let queue_priority = [1.0f32];
        let queue_create_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(graphics_family)
            .queue_priorities(&queue_priority);
        let queue_create_infos = [queue_create_info];

        let mut features_13 = vk::PhysicalDeviceVulkan13Features::default()
            .synchronization2(true)
            .dynamic_rendering(true);

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .push_next(&mut features_13);

        // SAFETY: Test harness. Valid device_create_info; device destroyed on error or drop.
        let device = match unsafe {
            instance.create_device(physical_device, &device_create_info, None)
        } {
            Ok(d) => d,
            Err(_) => {
                // SAFETY: Instance valid; cleanup on early return.
                unsafe { instance.destroy_instance(None) };
                return None;
            }
        };

        // SAFETY: Test harness. Device valid; graphics_family from enumerate; queue index 0.
        let graphics_queue = unsafe { device.get_device_queue(graphics_family, 0) };

        // Create VMA allocator.
        // SAFETY: Test harness. instance, device, physical_device valid (S6-style); allocator destroyed before device in Drop.
        let allocator_ci = vk_mem::AllocatorCreateInfo::new(&instance, &device, physical_device);
        let allocator = match unsafe { vk_mem::Allocator::new(allocator_ci) } {
            Ok(a) => a,
            Err(_) => {
                // SAFETY: Device and instance valid; cleanup on early return (allocator not created).
                unsafe {
                    device.destroy_device(None);
                    instance.destroy_instance(None);
                };
                return None;
            }
        };

        Some(Self {
            allocator: ManuallyDrop::new(allocator),
            device,
            instance,
            entry,
            physical_device,
            graphics_queue,
            graphics_family,
        })
    }
}

impl Drop for TestVulkanContext {
    fn drop(&mut self) {
        // SAFETY: Test harness. Device/instance/allocator valid; destroy order: idle → allocator → device → instance (no use-after-free).
        unsafe {
            self.device.device_wait_idle().ok();
            // Drop the allocator first (before destroying the device).
            ManuallyDrop::drop(&mut self.allocator);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

/// Macro to skip a test if Vulkan is not available.
///
/// Usage:
/// ```ignore
/// #[test]
/// fn my_vulkan_test() {
///     let ctx = require_vulkan!();
///     // ... use ctx ...
/// }
/// ```
#[macro_export]
macro_rules! require_vulkan {
    () => {
        match $crate::test_utils::TestVulkanContext::try_new() {
            Some(ctx) => ctx,
            None => {
                eprintln!("Skipping test: Vulkan not available");
                return;
            }
        }
    };
}
