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
