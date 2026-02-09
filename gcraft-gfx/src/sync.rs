use ash::vk;

/// Manages N frames in flight (typically 2–3).
///
/// Owns per-frame fences and binary semaphores for swapchain acquire/present,
/// plus a single timeline semaphore for cross-queue GPU-GPU synchronization.
pub struct FrameSync {
    pub frames_in_flight: usize,
    pub frame_index: usize,
    pub fences: Vec<vk::Fence>,
    pub timeline_semaphore: vk::Semaphore,
    pub timeline_value: u64,
    pub image_available: Vec<vk::Semaphore>,
    pub render_finished: Vec<vk::Semaphore>,
}

impl FrameSync {
    /// Create all synchronization primitives for `frames_in_flight` frames.
    pub fn new(device: &ash::Device, frames_in_flight: usize) -> Self {
        let fence_info = vk::FenceCreateInfo {
            flags: vk::FenceCreateFlags::SIGNALED,
            ..Default::default()
        };

        let fences: Vec<vk::Fence> = (0..frames_in_flight)
            .map(|_| {
                // SAFETY: S6 (device valid)
                unsafe { device.create_fence(&fence_info, None) }.unwrap()
            })
            .collect();

        let mut timeline_type_info = vk::SemaphoreTypeCreateInfo {
            semaphore_type: vk::SemaphoreType::TIMELINE,
            initial_value: 0,
            ..Default::default()
        };

        let timeline_semaphore = {
            let info = vk::SemaphoreCreateInfo {
                p_next: &mut timeline_type_info as *mut _ as *mut std::ffi::c_void,
                ..Default::default()
            };
            // SAFETY: S6 (device valid)
            unsafe { device.create_semaphore(&info, None) }.unwrap()
        };

        let binary_info = vk::SemaphoreCreateInfo::default();

        let image_available: Vec<vk::Semaphore> = (0..frames_in_flight)
            .map(|_| {
                // SAFETY: S6 (device valid)
                unsafe { device.create_semaphore(&binary_info, None) }.unwrap()
            })
            .collect();

        let render_finished: Vec<vk::Semaphore> = (0..frames_in_flight)
            .map(|_| {
                // SAFETY: S6 (device valid)
                unsafe { device.create_semaphore(&binary_info, None) }.unwrap()
            })
            .collect();

        Self {
            frames_in_flight,
            frame_index: 0,
            fences,
            timeline_semaphore,
            timeline_value: 0,
            image_available,
            render_finished,
        }
    }

    /// Block until the fence for the current frame is signaled, then reset it.
    pub fn wait_for_frame(&self, device: &ash::Device) {
        let fence = self.fences[self.frame_index];
        // SAFETY: S8 (fence valid, frame finished)
        unsafe {
            device.wait_for_fences(&[fence], true, u64::MAX).unwrap();
            device.reset_fences(&[fence]).unwrap();
        }
    }

    /// Advance to the next frame slot (wraps around).
    pub fn advance_frame(&mut self) {
        self.frame_index = (self.frame_index + 1) % self.frames_in_flight;
    }

    /// Increment and return the next timeline semaphore value.
    pub fn next_timeline_value(&mut self) -> u64 {
        self.timeline_value += 1;
        self.timeline_value
    }

    /// Returns the fence for the current frame.
    pub fn current_fence(&self) -> vk::Fence {
        self.fences[self.frame_index]
    }

    /// Returns the image-available semaphore for the current frame.
    pub fn current_image_available(&self) -> vk::Semaphore {
        self.image_available[self.frame_index]
    }

    /// Returns the render-finished semaphore for the current frame.
    pub fn current_render_finished(&self) -> vk::Semaphore {
        self.render_finished[self.frame_index]
    }

    /// Destroy all synchronization primitives.
    ///
    /// The caller must ensure no frame is in flight when this is called.
    pub fn destroy(&self, device: &ash::Device) {
        for &fence in &self.fences {
            // SAFETY: S6 (device valid), resources not in use (caller ensures)
            unsafe { device.destroy_fence(fence, None) };
        }
        // SAFETY: S6 (device valid), resources not in use (caller ensures)
        unsafe { device.destroy_semaphore(self.timeline_semaphore, None) };
        for &sem in &self.image_available {
            // SAFETY: S6 (device valid), resources not in use (caller ensures)
            unsafe { device.destroy_semaphore(sem, None) };
        }
        for &sem in &self.render_finished {
            // SAFETY: S6 (device valid), resources not in use (caller ensures)
            unsafe { device.destroy_semaphore(sem, None) };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::require_vulkan;
    use ash::vk;

    #[test]
    fn new_creates_correct_number_of_fences_and_semaphores() {
        let ctx = require_vulkan!();
        let sync = super::FrameSync::new(&ctx.device, 2);

        assert_eq!(sync.fences.len(), 2);
        assert_eq!(sync.image_available.len(), 2);
        assert_eq!(sync.render_finished.len(), 2);
        assert_eq!(sync.frame_index, 0);
        assert_eq!(sync.timeline_value, 0);

        for &fence in &sync.fences {
            assert_ne!(fence, vk::Fence::null());
        }
        for &sem in &sync.image_available {
            assert_ne!(sem, vk::Semaphore::null());
        }
        for &sem in &sync.render_finished {
            assert_ne!(sem, vk::Semaphore::null());
        }
        assert_ne!(sync.timeline_semaphore, vk::Semaphore::null());

        sync.destroy(&ctx.device);
    }

    #[test]
    fn wait_for_frame_and_reset_does_not_deadlock() {
        let ctx = require_vulkan!();
        let sync = super::FrameSync::new(&ctx.device, 2);

        // Fences are created signaled, so waiting should return immediately.
        sync.wait_for_frame(&ctx.device);

        sync.destroy(&ctx.device);
    }

    #[test]
    fn advance_frame_wraps_around() {
        let ctx = require_vulkan!();
        let mut sync = super::FrameSync::new(&ctx.device, 2);

        assert_eq!(sync.frame_index, 0);
        sync.advance_frame();
        assert_eq!(sync.frame_index, 1);
        sync.advance_frame();
        assert_eq!(sync.frame_index, 0);
        sync.advance_frame();
        assert_eq!(sync.frame_index, 1);

        sync.destroy(&ctx.device);
    }

    #[test]
    fn next_timeline_value_increments() {
        let ctx = require_vulkan!();
        let mut sync = super::FrameSync::new(&ctx.device, 2);

        assert_eq!(sync.next_timeline_value(), 1);
        assert_eq!(sync.next_timeline_value(), 2);
        assert_eq!(sync.next_timeline_value(), 3);

        sync.destroy(&ctx.device);
    }

    #[test]
    fn current_accessors_return_correct_frame() {
        let ctx = require_vulkan!();
        let mut sync = super::FrameSync::new(&ctx.device, 3);

        // Frame 0
        assert_eq!(sync.current_fence(), sync.fences[0]);
        assert_eq!(sync.current_image_available(), sync.image_available[0]);
        assert_eq!(sync.current_render_finished(), sync.render_finished[0]);

        sync.advance_frame();

        // Frame 1
        assert_eq!(sync.current_fence(), sync.fences[1]);
        assert_eq!(sync.current_image_available(), sync.image_available[1]);
        assert_eq!(sync.current_render_finished(), sync.render_finished[1]);

        sync.advance_frame();

        // Frame 2
        assert_eq!(sync.current_fence(), sync.fences[2]);
        assert_eq!(sync.current_image_available(), sync.image_available[2]);
        assert_eq!(sync.current_render_finished(), sync.render_finished[2]);

        sync.destroy(&ctx.device);
    }

    #[test]
    fn destroy_cleans_up_without_errors() {
        let ctx = require_vulkan!();
        let sync = super::FrameSync::new(&ctx.device, 2);
        sync.destroy(&ctx.device);
    }
}
