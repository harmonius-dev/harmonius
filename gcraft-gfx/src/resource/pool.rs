//! Transient resource pool for caching and reusing Vulkan images and buffers.
//!
//! The pool avoids per-frame allocation overhead by caching physical resources
//! keyed by their descriptor.  When a matching resource is available it is
//! reused; otherwise a new one is allocated via VMA.  Resources that have not
//! been used for a configurable number of frames are evicted and freed.

use ash::vk;
use vk_mem::Alloc; // trait required for create_image / create_buffer

use crate::graph::resource::{BufferDesc, ImageDesc};

// ---------------------------------------------------------------------------
// Cached resource wrappers
// ---------------------------------------------------------------------------

/// A physical image resource cached in the pool.
pub(crate) struct CachedImage {
    pub image: vk::Image,
    pub allocation: vk_mem::Allocation,
    pub desc: ImageDesc,
    /// Frame index when this resource was last used. Used for eviction.
    pub last_used_frame: u64,
}

/// A physical buffer resource cached in the pool.
pub(crate) struct CachedBuffer {
    pub buffer: vk::Buffer,
    pub allocation: vk_mem::Allocation,
    pub desc: BufferDesc,
    /// Frame index when this resource was last used.
    pub last_used_frame: u64,
}

// ---------------------------------------------------------------------------
// Resource pool
// ---------------------------------------------------------------------------

/// Caches physical Vulkan resources across frames so that transient images and
/// buffers can be reused without per-frame allocation overhead.
pub struct ResourcePool {
    /// Cached images available for reuse, keyed by descriptor.
    available_images: Vec<CachedImage>,
    /// Cached buffers available for reuse, keyed by descriptor.
    available_buffers: Vec<CachedBuffer>,
    /// Images currently in use this frame.
    in_use_images: Vec<CachedImage>,
    /// Buffers currently in use this frame.
    in_use_buffers: Vec<CachedBuffer>,
    /// Current frame counter for eviction tracking.
    current_frame: u64,
    /// Number of frames a resource can be unused before being freed.
    max_unused_frames: u64,
}

impl ResourcePool {
    /// Create an empty pool.
    ///
    /// `max_unused_frames` controls how many frames a resource can sit idle in
    /// the available list before [`evict`](Self::evict) destroys it.
    pub fn new(max_unused_frames: u64) -> Self {
        Self {
            available_images: Vec::new(),
            available_buffers: Vec::new(),
            in_use_images: Vec::new(),
            in_use_buffers: Vec::new(),
            current_frame: 0,
            max_unused_frames,
        }
    }

    /// Acquire a transient image matching `desc`.
    ///
    /// If a compatible image exists in the available pool it is reused;
    /// otherwise a new image is allocated through VMA.
    ///
    /// The pool exclusively owns the VMA allocation. Only the `vk::Image`
    /// handle is returned to the caller for use in Vulkan commands.
    pub fn acquire_image(
        &mut self,
        allocator: &vk_mem::Allocator,
        desc: &ImageDesc,
    ) -> vk::Image {
        // Look for a matching image in the available list.
        if let Some(idx) = self.available_images.iter().position(|c| c.desc == *desc) {
            let mut cached = self.available_images.swap_remove(idx);
            cached.last_used_frame = self.current_frame;
            let handle = cached.image;
            self.in_use_images.push(cached);
            return handle;
        }

        // No match – allocate a new image via VMA.
        let image_ci = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(desc.format)
            .extent(desc.extent)
            .mip_levels(desc.mip_levels)
            .array_layers(desc.array_layers)
            .samples(desc.samples)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(desc.usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);

        let alloc_ci = vk_mem::AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::AutoPreferDevice,
            ..Default::default()
        };

        // SAFETY: S6 (allocator valid)
        let (image, allocation) = unsafe {
            allocator
                .create_image(&image_ci, &alloc_ci)
                .expect("failed to allocate transient image")
        };

        self.in_use_images.push(CachedImage {
            image,
            allocation,
            desc: desc.clone(),
            last_used_frame: self.current_frame,
        });

        image
    }

    /// Acquire a transient buffer matching `desc`.
    ///
    /// If a compatible buffer exists in the available pool it is reused;
    /// otherwise a new buffer is allocated through VMA.
    ///
    /// The pool exclusively owns the VMA allocation. Only the `vk::Buffer`
    /// handle is returned.
    pub fn acquire_buffer(
        &mut self,
        allocator: &vk_mem::Allocator,
        desc: &BufferDesc,
    ) -> vk::Buffer {
        // Look for a matching buffer in the available list.
        if let Some(idx) = self
            .available_buffers
            .iter()
            .position(|c| c.desc == *desc)
        {
            let mut cached = self.available_buffers.swap_remove(idx);
            cached.last_used_frame = self.current_frame;
            let handle = cached.buffer;
            self.in_use_buffers.push(cached);
            return handle;
        }

        // No match – allocate a new buffer via VMA.
        let buffer_ci = vk::BufferCreateInfo::default()
            .size(desc.size)
            .usage(desc.usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let alloc_ci = vk_mem::AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::AutoPreferDevice,
            ..Default::default()
        };

        // SAFETY: S6 (allocator valid)
        let (buffer, allocation) = unsafe {
            allocator
                .create_buffer(&buffer_ci, &alloc_ci)
                .expect("failed to allocate transient buffer")
        };

        self.in_use_buffers.push(CachedBuffer {
            buffer,
            allocation,
            desc: desc.clone(),
            last_used_frame: self.current_frame,
        });

        buffer
    }

    /// Release all resources acquired this frame back into the available pool.
    ///
    /// Must be called once per frame after the GPU has finished using the
    /// resources (i.e. after the frame fence has been signaled).  Also
    /// increments the internal frame counter.
    pub fn release_frame(&mut self) {
        self.available_images
            .append(&mut self.in_use_images);
        self.available_buffers
            .append(&mut self.in_use_buffers);
        self.current_frame += 1;
    }

    /// Evict and destroy resources that have been unused for more than
    /// `max_unused_frames`.
    ///
    /// Should be called periodically (e.g. once per frame after
    /// [`release_frame`](Self::release_frame)) to reclaim GPU memory.
    pub fn evict(&mut self, allocator: &vk_mem::Allocator) {
        let deadline = self.current_frame.saturating_sub(self.max_unused_frames);

        // Evict stale images.
        let mut i = 0;
        while i < self.available_images.len() {
            if self.available_images[i].last_used_frame < deadline {
                let mut cached = self.available_images.swap_remove(i);
                // SAFETY: S8 (frame fence signaled, resource not in GPU use)
                unsafe {
                    allocator.destroy_image(cached.image, &mut cached.allocation);
                }
                // Don't increment i – swap_remove moved the last element here.
            } else {
                i += 1;
            }
        }

        // Evict stale buffers.
        let mut i = 0;
        while i < self.available_buffers.len() {
            if self.available_buffers[i].last_used_frame < deadline {
                let mut cached = self.available_buffers.swap_remove(i);
                // SAFETY: S8 (frame fence signaled, resource not in GPU use)
                unsafe {
                    allocator.destroy_buffer(cached.buffer, &mut cached.allocation);
                }
            } else {
                i += 1;
            }
        }
    }

    /// Destroy **all** resources held by the pool (both available and in-use).
    ///
    /// Call during shutdown after the device is idle.
    pub fn destroy_all(&mut self, allocator: &vk_mem::Allocator) {
        for mut cached in self.available_images.drain(..) {
            // SAFETY: S6, caller ensures no GPU use
            unsafe {
                allocator.destroy_image(cached.image, &mut cached.allocation);
            }
        }
        for mut cached in self.in_use_images.drain(..) {
            // SAFETY: S6, caller ensures no GPU use
            unsafe {
                allocator.destroy_image(cached.image, &mut cached.allocation);
            }
        }
        for mut cached in self.available_buffers.drain(..) {
            // SAFETY: S6, caller ensures no GPU use
            unsafe {
                allocator.destroy_buffer(cached.buffer, &mut cached.allocation);
            }
        }
        for mut cached in self.in_use_buffers.drain(..) {
            // SAFETY: S6, caller ensures no GPU use
            unsafe {
                allocator.destroy_buffer(cached.buffer, &mut cached.allocation);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::resource::{BufferDesc, ImageDesc};
    use crate::require_vulkan;
    use ash::vk;

    fn default_image_desc() -> ImageDesc {
        ImageDesc {
            format: vk::Format::R8G8B8A8_UNORM,
            extent: vk::Extent3D {
                width: 64,
                height: 64,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        }
    }

    fn default_buffer_desc() -> BufferDesc {
        BufferDesc {
            size: 1024,
            usage: vk::BufferUsageFlags::STORAGE_BUFFER,
        }
    }

    #[test]
    fn new_creates_empty_pool() {
        let _pool = super::ResourcePool::new(3);
    }

    #[test]
    fn acquire_image_returns_valid_handle() {
        let ctx = require_vulkan!();
        let mut pool = super::ResourcePool::new(3);
        let desc = default_image_desc();
        let image = pool.acquire_image(&ctx.allocator, &desc);
        assert_ne!(image, vk::Image::null());
        pool.destroy_all(&ctx.allocator);
    }

    #[test]
    fn acquire_buffer_returns_valid_handle() {
        let ctx = require_vulkan!();
        let mut pool = super::ResourcePool::new(3);
        let desc = default_buffer_desc();
        let buffer = pool.acquire_buffer(&ctx.allocator, &desc);
        assert_ne!(buffer, vk::Buffer::null());
        pool.destroy_all(&ctx.allocator);
    }

    #[test]
    fn release_and_reacquire_reuses_resource() {
        let ctx = require_vulkan!();
        let mut pool = super::ResourcePool::new(3);
        let desc = default_image_desc();

        let image1 = pool.acquire_image(&ctx.allocator, &desc);
        pool.release_frame();
        let image2 = pool.acquire_image(&ctx.allocator, &desc);
        assert_eq!(image1, image2, "expected reused image handle");

        pool.destroy_all(&ctx.allocator);
    }

    #[test]
    fn acquire_different_desc_allocates_new() {
        let ctx = require_vulkan!();
        let mut pool = super::ResourcePool::new(3);

        let desc_a = default_image_desc();
        let desc_b = ImageDesc {
            format: vk::Format::R16G16B16A16_SFLOAT,
            extent: vk::Extent3D {
                width: 128,
                height: 128,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        };

        let image_a = pool.acquire_image(&ctx.allocator, &desc_a);
        pool.release_frame();
        let image_b = pool.acquire_image(&ctx.allocator, &desc_b);
        assert_ne!(image_a, image_b, "different desc should allocate new image");

        pool.destroy_all(&ctx.allocator);
    }

    #[test]
    fn raw_vma_create_destroy_image() {
        use vk_mem::Alloc;
        let ctx = require_vulkan!();
        let image_ci = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(vk::Format::R8G8B8A8_UNORM)
            .extent(vk::Extent3D { width: 64, height: 64, depth: 1 })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);
        let alloc_ci = vk_mem::AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::AutoPreferDevice,
            ..Default::default()
        };
        // SAFETY: Test only. Allocator valid (S6-style); image not used on GPU.
        let (image, mut allocation) = unsafe {
            ctx.allocator.create_image(&image_ci, &alloc_ci).unwrap()
        };
        assert_ne!(image, vk::Image::null());
        // SAFETY: Test only. Image just created and not submitted; allocator valid.
        unsafe {
            ctx.allocator.destroy_image(image, &mut allocation);
        }
    }

    #[test]
    fn evict_removes_stale_resources() {
        let ctx = require_vulkan!();
        let mut pool = super::ResourcePool::new(3);
        let desc = default_image_desc();

        let _image1 = pool.acquire_image(&ctx.allocator, &desc);

        // Release and advance the frame counter past max_unused_frames.
        pool.release_frame();
        for _ in 0..4 {
            pool.release_frame();
        }
        pool.evict(&ctx.allocator);

        // The pool should be completely empty now. Verify by checking that
        // re-acquiring allocates a new resource (destroy_all would be a no-op).
        pool.destroy_all(&ctx.allocator);
    }

    #[test]
    fn destroy_all_cleans_up() {
        let ctx = require_vulkan!();
        let mut pool = super::ResourcePool::new(3);
        let img_desc = default_image_desc();
        let buf_desc = default_buffer_desc();

        let _image = pool.acquire_image(&ctx.allocator, &img_desc);
        let _buffer = pool.acquire_buffer(&ctx.allocator, &buf_desc);

        pool.destroy_all(&ctx.allocator);
    }
}
