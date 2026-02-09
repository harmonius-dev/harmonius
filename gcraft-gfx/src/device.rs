/// Queue type for pass scheduling.
///
/// Duplicated from `graph::pass` to avoid circular deps.
/// The canonical definition is in `graph::pass`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QueueType {
    Graphics,
    AsyncCompute,
    Transfer,
}

/// A Vulkan queue paired with its family index.
#[derive(Clone, Copy, Debug)]
pub struct QueueHandle {
    pub queue: ash::vk::Queue,
    pub family_index: u32,
}

/// Wraps the Vulkan device, queue handles, and VMA allocator.
///
/// The render graph receives a `&DeviceContext` for execution.
/// The device context is NOT created by the graph — it is provided externally.
/// The graph only borrows it during execution.
///
/// The caller is responsible for managing the device lifetime; no `Drop` is
/// implemented here.
pub struct DeviceContext {
    pub instance: ash::Instance,
    pub physical_device: ash::vk::PhysicalDevice,
    pub device: ash::Device,
    pub allocator: vk_mem::Allocator,
    pub graphics_queue: QueueHandle,
    /// Dedicated / async compute queue, if available.
    pub compute_queue: Option<QueueHandle>,
    /// Dedicated transfer queue, if available.
    pub transfer_queue: Option<QueueHandle>,
}

impl DeviceContext {
    /// Create a new `DeviceContext` from all required components.
    pub fn new(
        instance: ash::Instance,
        physical_device: ash::vk::PhysicalDevice,
        device: ash::Device,
        allocator: vk_mem::Allocator,
        graphics_queue: QueueHandle,
        compute_queue: Option<QueueHandle>,
        transfer_queue: Option<QueueHandle>,
    ) -> Self {
        Self {
            instance,
            physical_device,
            device,
            allocator,
            graphics_queue,
            compute_queue,
            transfer_queue,
        }
    }

    /// Returns the appropriate queue for a given [`QueueType`].
    ///
    /// When a dedicated compute or transfer queue is not available, the
    /// graphics queue is used as a fallback.
    pub fn queue_for(&self, queue_type: QueueType) -> &QueueHandle {
        match queue_type {
            QueueType::Graphics => &self.graphics_queue,
            QueueType::AsyncCompute => {
                self.compute_queue.as_ref().unwrap_or(&self.graphics_queue)
            }
            QueueType::Transfer => {
                self.transfer_queue.as_ref().unwrap_or(&self.graphics_queue)
            }
        }
    }

    /// Returns `true` if a dedicated async-compute queue is available.
    pub fn has_dedicated_compute(&self) -> bool {
        self.compute_queue.is_some()
    }

    /// Returns `true` if a dedicated transfer queue is available.
    pub fn has_dedicated_transfer(&self) -> bool {
        self.transfer_queue.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ash::vk::{self, Handle};
    use std::mem::ManuallyDrop;

    // ---- helpers ----

    /// Create a [`QueueHandle`] from a synthetic raw handle.
    fn fake_queue(family: u32, raw: u64) -> QueueHandle {
        QueueHandle {
            queue: vk::Queue::from_raw(raw),
            family_index: family,
        }
    }

    /// Build a [`DeviceContext`] from the test Vulkan context with the given
    /// optional compute / transfer queues.
    ///
    /// The returned value is wrapped in [`ManuallyDrop`] so the bitwise-copied
    /// allocator is never dropped (the [`TestVulkanContext`] owns the real one).
    fn make_dc(
        ctx: &crate::test_utils::TestVulkanContext,
        compute_queue: Option<QueueHandle>,
        transfer_queue: Option<QueueHandle>,
    ) -> ManuallyDrop<DeviceContext> {
        let graphics_queue = QueueHandle {
            queue: ctx.graphics_queue,
            family_index: ctx.graphics_family,
        };
        // SAFETY: Allocator is bitwise-copied once; DeviceContext is wrapped in ManuallyDrop
        // so the allocator is never dropped by DeviceContext (no double-free). Drop order
        // in tests is under test control; production DeviceContext is not built this way.
        let allocator = unsafe { std::ptr::read(&*ctx.allocator) };
        ManuallyDrop::new(DeviceContext {
            instance: ctx.instance.clone(),
            physical_device: ctx.physical_device,
            device: ctx.device.clone(),
            allocator,
            graphics_queue,
            compute_queue,
            transfer_queue,
        })
    }

    // ---- queue_for ----

    #[test]
    fn queue_for_returns_graphics_queue() {
        let ctx = crate::require_vulkan!();
        let dc = make_dc(&ctx, None, None);
        let q = dc.queue_for(QueueType::Graphics);
        assert_eq!(q.family_index, ctx.graphics_family);
    }

    #[test]
    fn queue_for_falls_back_to_graphics_when_no_compute() {
        let ctx = crate::require_vulkan!();
        let dc = make_dc(&ctx, None, None);
        let q = dc.queue_for(QueueType::AsyncCompute);
        assert_eq!(q.family_index, ctx.graphics_family);
    }

    #[test]
    fn queue_for_returns_dedicated_compute() {
        let ctx = crate::require_vulkan!();
        let compute = fake_queue(42, 0xC0);
        let dc = make_dc(&ctx, Some(compute), None);
        let q = dc.queue_for(QueueType::AsyncCompute);
        assert_eq!(q.family_index, 42);
    }

    #[test]
    fn queue_for_falls_back_to_graphics_when_no_transfer() {
        let ctx = crate::require_vulkan!();
        let dc = make_dc(&ctx, None, None);
        let q = dc.queue_for(QueueType::Transfer);
        assert_eq!(q.family_index, ctx.graphics_family);
    }

    #[test]
    fn queue_for_returns_dedicated_transfer() {
        let ctx = crate::require_vulkan!();
        let transfer = fake_queue(99, 0xD0);
        let dc = make_dc(&ctx, None, Some(transfer));
        let q = dc.queue_for(QueueType::Transfer);
        assert_eq!(q.family_index, 99);
    }

    // ---- has_dedicated_compute ----

    #[test]
    fn has_dedicated_compute_false_when_none() {
        let ctx = crate::require_vulkan!();
        let dc = make_dc(&ctx, None, None);
        assert!(!dc.has_dedicated_compute());
    }

    #[test]
    fn has_dedicated_compute_true_when_present() {
        let ctx = crate::require_vulkan!();
        let dc = make_dc(&ctx, Some(fake_queue(42, 0xC0)), None);
        assert!(dc.has_dedicated_compute());
    }

    // ---- has_dedicated_transfer ----

    #[test]
    fn has_dedicated_transfer_false_when_none() {
        let ctx = crate::require_vulkan!();
        let dc = make_dc(&ctx, None, None);
        assert!(!dc.has_dedicated_transfer());
    }

    #[test]
    fn has_dedicated_transfer_true_when_present() {
        let ctx = crate::require_vulkan!();
        let dc = make_dc(&ctx, None, Some(fake_queue(99, 0xD0)));
        assert!(dc.has_dedicated_transfer());
    }
}
