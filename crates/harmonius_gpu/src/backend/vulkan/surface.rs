//! Platform surface creation.

use ash::khr;
use ash::vk;
use ash::{Entry, Instance};
use harmonius_platform::{SurfaceHandle, SurfaceNativeExt};

use super::VulkanError;

/// Create a `VkSurfaceKHR` from a platform `SurfaceHandle`.
pub fn create_vulkan_surface(
    entry: &Entry,
    instance: &Instance,
    _loader: &khr::surface::Instance,
    handle: &SurfaceHandle,
) -> Result<vk::SurfaceKHR, VulkanError> {
    #[cfg(target_os = "linux")]
    {
        let window = handle
            .window_handle()
            .map_err(|e| VulkanError::Surface(e.to_string()))?;
        let display = handle
            .display_handle()
            .map_err(|e| VulkanError::Surface(e.to_string()))?;
        let raw_window = window.as_raw();
        let raw_display = display.as_raw();
        let (window_id, display_ptr) = match (raw_window, raw_display) {
            (
                raw_window_handle::RawWindowHandle::Xlib(xlib),
                raw_window_handle::RawDisplayHandle::Xlib(dpy),
            ) => (xlib.window.get(), dpy.display.as_ptr()),
            _ => return Err(VulkanError::Surface("expected Xlib handles".into())),
        };
        let create_info = vk::XlibSurfaceCreateInfoKHR::default()
            .dpy(display_ptr)
            .window(window_id);
        let xlib = khr::xlib_surface::Instance::new(entry, instance);
        unsafe {
            xlib.create_xlib_surface(&create_info, None)
                .map_err(|e| VulkanError::Api(e.to_string()))
        }
    }

    #[cfg(target_os = "macos")]
    {
        let layer = match handle.native_ext {
            Some(SurfaceNativeExt::MetalLayer(ptr)) => ptr,
            _ => return Err(VulkanError::Surface("missing Metal layer".into())),
        };
        let create_info = vk::MetalSurfaceCreateInfoEXT::default().layer(layer);
        let metal = ash::ext::metal_surface::Instance::new(entry, instance);
        unsafe {
            metal
                .create_metal_surface(&create_info, None)
                .map_err(|e| VulkanError::Api(e.to_string()))
        }
    }
}
