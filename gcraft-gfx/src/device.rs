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
