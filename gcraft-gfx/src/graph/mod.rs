pub mod resource;
pub mod pass;
pub mod compile;
pub mod barrier;
pub mod execute;

use resource::{
    BufferDesc, BufferHandle, ConditionFlag, DrawSlot, ImageDesc, ImageHandle, SubresourceRange,
    BufferInfo, FlagInfo, ImageInfo, ResourceInfo, ResourceKind,
};
use pass::{PassBuilder, PassNode, QueueType, RecordedCommand};
use crate::command::batch::{self, Batchable};

// ---------------------------------------------------------------------------
// RenderGraph
// ---------------------------------------------------------------------------

/// The central render graph builder.
///
/// Resources and passes are declared through this struct.  After construction,
/// the graph is compiled (in [`compile`]) into a linear schedule with
/// automatic barrier insertion.
pub struct RenderGraph {
    generation: u32,
    images: Vec<ImageInfo>,
    buffers: Vec<BufferInfo>,
    passes: Vec<PassNode>,
    flags: Vec<FlagInfo>,
    draw_slot_counter: u16,
    /// Expanded commands for each draw slot, indexed by slot_index.
    draw_slot_commands: Vec<Vec<RecordedCommand>>,
}

impl RenderGraph {
    /// Create a new, empty render graph with generation 0.
    pub fn new() -> Self {
        Self {
            generation: 0,
            images: Vec::new(),
            buffers: Vec::new(),
            passes: Vec::new(),
            flags: Vec::new(),
            draw_slot_counter: 0,
            draw_slot_commands: Vec::new(),
        }
    }

    // -- Resource creation (internal) ----------------------------------------

    fn create_image_inner(
        &mut self,
        name: &'static str,
        desc: ImageDesc,
        kind: ResourceKind,
    ) -> ImageHandle {
        assert!(
            desc.extent.width > 0 && desc.extent.height > 0 && desc.extent.depth > 0,
            "create_image `{name}`: extent must not have zero dimensions",
        );
        assert!(
            desc.mip_levels > 0,
            "create_image `{name}`: mip_levels must be > 0",
        );
        assert!(
            desc.array_layers > 0,
            "create_image `{name}`: array_layers must be > 0",
        );
        assert!(
            !desc.usage.is_empty(),
            "create_image `{name}`: usage must not be empty",
        );

        let index = self.images.len() as u16;
        self.images.push(ImageInfo {
            desc,
            resource: ResourceInfo {
                name,
                kind,
                current_version: 0,
                subresource: None,
                parent_index: None,
            },
        });

        ImageHandle {
            index,
            version: 0,
            generation: self.generation,
        }
    }

    fn create_buffer_inner(
        &mut self,
        name: &'static str,
        desc: BufferDesc,
        kind: ResourceKind,
    ) -> BufferHandle {
        assert!(
            desc.size > 0,
            "create_buffer `{name}`: size must be > 0",
        );
        assert!(
            !desc.usage.is_empty(),
            "create_buffer `{name}`: usage must not be empty",
        );

        let index = self.buffers.len() as u16;
        self.buffers.push(BufferInfo {
            desc,
            resource: ResourceInfo {
                name,
                kind,
                current_version: 0,
                subresource: None,
                parent_index: None,
            },
        });

        BufferHandle {
            index,
            version: 0,
            generation: self.generation,
        }
    }

    // -- Resource creation (public) ------------------------------------------

    /// Declare a transient image resource.
    ///
    /// # Panics
    ///
    /// Panics if `desc` has a zero-dimension extent, zero mip levels, zero
    /// array layers, or empty usage flags.
    pub fn create_image(&mut self, name: &'static str, desc: ImageDesc) -> ImageHandle {
        self.create_image_inner(name, desc, ResourceKind::Transient)
    }

    /// Declare a transient buffer resource.
    ///
    /// # Panics
    ///
    /// Panics if `desc` has zero size or empty usage flags.
    pub fn create_buffer(&mut self, name: &'static str, desc: BufferDesc) -> BufferHandle {
        self.create_buffer_inner(name, desc, ResourceKind::Transient)
    }

    /// Import an externally-owned image into the graph for barrier tracking.
    ///
    /// Validation is identical to [`create_image`](Self::create_image).
    pub fn import_image(&mut self, name: &'static str, desc: ImageDesc) -> ImageHandle {
        self.create_image_inner(name, desc, ResourceKind::Imported)
    }

    /// Import an externally-owned buffer into the graph for barrier tracking.
    ///
    /// Validation is identical to [`create_buffer`](Self::create_buffer).
    pub fn import_buffer(&mut self, name: &'static str, desc: BufferDesc) -> BufferHandle {
        self.create_buffer_inner(name, desc, ResourceKind::Imported)
    }

    // -- Sub-resource views --------------------------------------------------

    /// Create a view that references a single mip level of an existing image.
    ///
    /// The returned handle shares the parent's descriptor and version but
    /// carries a [`SubresourceRange`] narrowed to the requested mip level.
    ///
    /// # Panics
    ///
    /// Panics if the handle generation does not match the graph, or if
    /// `mip_level >= desc.mip_levels`.
    pub fn image_mip_view(&mut self, handle: ImageHandle, mip_level: u32) -> ImageHandle {
        assert_eq!(
            handle.generation, self.generation,
            "image_mip_view: handle generation {} does not match graph generation {}",
            handle.generation, self.generation,
        );

        let parent = &self.images[handle.index as usize];
        assert!(
            mip_level < parent.desc.mip_levels,
            "image_mip_view: mip_level {mip_level} >= mip_levels {}",
            parent.desc.mip_levels,
        );

        // Extract values before mutably borrowing self.images.
        let desc = parent.desc.clone();
        let name = parent.resource.name;
        let kind = parent.resource.kind.clone();
        let array_layers = desc.array_layers;

        let index = self.images.len() as u16;
        self.images.push(ImageInfo {
            desc,
            resource: ResourceInfo {
                name,
                kind,
                current_version: 0,
                subresource: Some(SubresourceRange {
                    base_mip_level: mip_level,
                    mip_count: 1,
                    base_array_layer: 0,
                    layer_count: array_layers,
                }),
                parent_index: Some(handle.index),
            },
        });

        ImageHandle {
            index,
            version: handle.version,
            generation: self.generation,
        }
    }

    /// Create a view that references a single array layer of an existing image.
    ///
    /// The returned handle shares the parent's descriptor and version but
    /// carries a [`SubresourceRange`] narrowed to the requested array layer.
    ///
    /// # Panics
    ///
    /// Panics if the handle generation does not match the graph, or if
    /// `array_layer >= desc.array_layers`.
    pub fn image_layer_view(&mut self, handle: ImageHandle, array_layer: u32) -> ImageHandle {
        assert_eq!(
            handle.generation, self.generation,
            "image_layer_view: handle generation {} does not match graph generation {}",
            handle.generation, self.generation,
        );

        let parent = &self.images[handle.index as usize];
        assert!(
            array_layer < parent.desc.array_layers,
            "image_layer_view: array_layer {array_layer} >= array_layers {}",
            parent.desc.array_layers,
        );

        // Extract values before mutably borrowing self.images.
        let desc = parent.desc.clone();
        let name = parent.resource.name;
        let kind = parent.resource.kind.clone();
        let mip_levels = desc.mip_levels;

        let index = self.images.len() as u16;
        self.images.push(ImageInfo {
            desc,
            resource: ResourceInfo {
                name,
                kind,
                current_version: 0,
                subresource: Some(SubresourceRange {
                    base_mip_level: 0,
                    mip_count: mip_levels,
                    base_array_layer: array_layer,
                    layer_count: 1,
                }),
                parent_index: Some(handle.index),
            },
        });

        ImageHandle {
            index,
            version: handle.version,
            generation: self.generation,
        }
    }

    // -- Pass creation -------------------------------------------------------

    fn add_pass(&mut self, name: &'static str, queue: QueueType) -> PassBuilder<'_> {
        PassBuilder::new(
            &mut self.passes,
            self.generation,
            &self.images,
            &self.buffers,
            &mut self.draw_slot_counter,
            name,
            queue,
        )
    }

    /// Add a graphics render pass.
    pub fn add_graphics_pass(&mut self, name: &'static str) -> PassBuilder<'_> {
        self.add_pass(name, QueueType::Graphics)
    }

    /// Add a compute pass that runs on the graphics queue.
    pub fn add_compute_pass(&mut self, name: &'static str) -> PassBuilder<'_> {
        self.add_pass(name, QueueType::Graphics)
    }

    /// Add a compute pass that runs on a dedicated async-compute queue.
    pub fn add_async_compute_pass(&mut self, name: &'static str) -> PassBuilder<'_> {
        self.add_pass(name, QueueType::AsyncCompute)
    }

    /// Add a transfer pass that runs on a dedicated transfer queue.
    pub fn add_transfer_pass(&mut self, name: &'static str) -> PassBuilder<'_> {
        self.add_pass(name, QueueType::Transfer)
    }

    // -- Condition flags -----------------------------------------------------

    /// Define a boolean condition flag with an initial value.
    ///
    /// Flags can be toggled with [`set_flag`](Self::set_flag) and queried with
    /// [`flag_value`](Self::flag_value).  Passes can be conditionally enabled
    /// or disabled based on flag state.
    pub fn define_flag(&mut self, name: &'static str, initial: bool) -> ConditionFlag {
        let index = self.flags.len() as u16;
        self.flags.push(FlagInfo { name, value: initial });
        ConditionFlag {
            index,
            generation: self.generation,
        }
    }

    /// Set the value of a condition flag.
    ///
    /// # Panics
    ///
    /// Panics if the flag generation does not match the graph.
    pub fn set_flag(&mut self, flag: ConditionFlag, value: bool) {
        assert_eq!(
            flag.generation, self.generation,
            "set_flag: flag generation {} does not match graph generation {}",
            flag.generation, self.generation,
        );
        self.flags[flag.index as usize].value = value;
    }

    /// Query the current value of a condition flag.
    ///
    /// # Panics
    ///
    /// Panics if the flag generation does not match the graph.
    pub fn flag_value(&self, flag: ConditionFlag) -> bool {
        assert_eq!(
            flag.generation, self.generation,
            "flag_value: flag generation {} does not match graph generation {}",
            flag.generation, self.generation,
        );
        self.flags[flag.index as usize].value
    }

    // -- Draw slot filling ---------------------------------------------------

    /// Fill an opaque draw slot with batched commands.
    ///
    /// Sorts `commands` by batch key for maximum GPU state batching, then
    /// expands them into [`RecordedCommand`]s stored in the draw slot.
    ///
    /// # Panics
    ///
    /// Panics if the slot generation does not match the graph, or if the slot
    /// has already been filled.
    pub fn fill_opaque<T: Batchable>(&mut self, slot: DrawSlot, commands: &mut [T]) {
        assert_eq!(
            slot.generation, self.generation,
            "fill_opaque: slot generation {} does not match graph generation {}",
            slot.generation, self.generation,
        );
        let idx = slot.slot_index as usize;
        if self.draw_slot_commands.len() <= idx {
            self.draw_slot_commands.resize_with(idx + 1, Vec::new);
        }
        assert!(
            self.draw_slot_commands[idx].is_empty(),
            "fill_opaque: draw slot {} already filled",
            slot.slot_index,
        );
        self.draw_slot_commands[idx] = batch::fill_opaque(commands);
    }

    /// Fill a transparent draw slot with batched commands.
    ///
    /// Sorts `commands` by distance (back-to-front) for correct alpha
    /// blending, then expands them into [`RecordedCommand`]s stored in the
    /// draw slot.
    ///
    /// # Panics
    ///
    /// Panics if the slot generation does not match the graph, or if the slot
    /// has already been filled.
    pub fn fill_transparent<T: Batchable>(&mut self, slot: DrawSlot, commands: &mut [T]) {
        assert_eq!(
            slot.generation, self.generation,
            "fill_transparent: slot generation {} does not match graph generation {}",
            slot.generation, self.generation,
        );
        let idx = slot.slot_index as usize;
        if self.draw_slot_commands.len() <= idx {
            self.draw_slot_commands.resize_with(idx + 1, Vec::new);
        }
        assert!(
            self.draw_slot_commands[idx].is_empty(),
            "fill_transparent: draw slot {} already filled",
            slot.slot_index,
        );
        self.draw_slot_commands[idx] = batch::fill_transparent(commands);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use ash::vk;

    // -- 1. new() creates empty graph ----------------------------------------

    #[test]
    fn new_creates_empty_graph() {
        let graph = RenderGraph::new();
        assert_eq!(graph.generation, 0);
        assert!(graph.images.is_empty());
        assert!(graph.buffers.is_empty());
        assert!(graph.passes.is_empty());
        assert!(graph.flags.is_empty());
        assert_eq!(graph.draw_slot_counter, 0);
        assert!(graph.draw_slot_commands.is_empty());
    }

    // -- 2. create_image returns valid handle --------------------------------

    #[test]
    fn create_image_returns_valid_handle() {
        let mut graph = RenderGraph::new();
        let handle = graph.create_image("color", ImageDesc::default());

        assert_eq!(handle.index, 0);
        assert_eq!(handle.version, 0);
        assert_eq!(handle.generation, 0);
        assert_eq!(graph.images.len(), 1);
        assert_eq!(graph.images[0].resource.name, "color");
        assert!(matches!(graph.images[0].resource.kind, ResourceKind::Transient));
    }

    // -- 3. create_image panics on zero extent -------------------------------

    #[test]
    #[should_panic(expected = "zero dimensions")]
    fn create_image_panics_on_zero_extent() {
        let mut graph = RenderGraph::new();
        graph.create_image(
            "bad",
            ImageDesc {
                extent: vk::Extent3D { width: 0, height: 1, depth: 1 },
                ..Default::default()
            },
        );
    }

    // -- 4. create_buffer returns valid handle --------------------------------

    #[test]
    fn create_buffer_returns_valid_handle() {
        let mut graph = RenderGraph::new();
        let handle = graph.create_buffer(
            "data",
            BufferDesc {
                size: 4096,
                usage: vk::BufferUsageFlags::STORAGE_BUFFER,
            },
        );

        assert_eq!(handle.index, 0);
        assert_eq!(handle.version, 0);
        assert_eq!(handle.generation, 0);
        assert_eq!(graph.buffers.len(), 1);
        assert_eq!(graph.buffers[0].resource.name, "data");
    }

    // -- 5. create_buffer panics on zero size --------------------------------

    #[test]
    #[should_panic(expected = "size must be > 0")]
    fn create_buffer_panics_on_zero_size() {
        let mut graph = RenderGraph::new();
        graph.create_buffer(
            "bad",
            BufferDesc {
                size: 0,
                usage: vk::BufferUsageFlags::STORAGE_BUFFER,
            },
        );
    }

    // -- 6. image_mip_view creates correct subresource -----------------------

    #[test]
    fn image_mip_view_creates_correct_subresource() {
        let mut graph = RenderGraph::new();
        let handle = graph.create_image(
            "color",
            ImageDesc {
                mip_levels: 4,
                array_layers: 6,
                ..Default::default()
            },
        );

        let mip_handle = graph.image_mip_view(handle, 2);

        assert_eq!(mip_handle.index, 1);
        assert_eq!(mip_handle.version, 0);
        assert_eq!(mip_handle.generation, 0);

        let info = &graph.images[mip_handle.index as usize];
        let sub = info.resource.subresource.as_ref().unwrap();
        assert_eq!(sub.base_mip_level, 2);
        assert_eq!(sub.mip_count, 1);
        assert_eq!(sub.base_array_layer, 0);
        assert_eq!(sub.layer_count, 6);
        assert_eq!(info.resource.parent_index, Some(0));
    }

    // -- 7. image_mip_view panics on out-of-range mip level -----------------

    #[test]
    #[should_panic(expected = "mip_level")]
    fn image_mip_view_panics_on_invalid_mip() {
        let mut graph = RenderGraph::new();
        let handle = graph.create_image(
            "color",
            ImageDesc {
                mip_levels: 4,
                ..Default::default()
            },
        );
        graph.image_mip_view(handle, 4);
    }

    // -- 8. image_layer_view creates correct subresource ---------------------

    #[test]
    fn image_layer_view_creates_correct_subresource() {
        let mut graph = RenderGraph::new();
        let handle = graph.create_image(
            "cubemap",
            ImageDesc {
                mip_levels: 3,
                array_layers: 6,
                ..Default::default()
            },
        );

        let layer_handle = graph.image_layer_view(handle, 4);

        assert_eq!(layer_handle.index, 1);
        assert_eq!(layer_handle.version, 0);

        let info = &graph.images[layer_handle.index as usize];
        let sub = info.resource.subresource.as_ref().unwrap();
        assert_eq!(sub.base_mip_level, 0);
        assert_eq!(sub.mip_count, 3);
        assert_eq!(sub.base_array_layer, 4);
        assert_eq!(sub.layer_count, 1);
        assert_eq!(info.resource.parent_index, Some(0));
    }

    // -- 9. define_flag + set_flag + flag_value ------------------------------

    #[test]
    fn define_flag_set_flag_flag_value() {
        let mut graph = RenderGraph::new();
        let flag = graph.define_flag("debug_vis", false);
        assert!(!graph.flag_value(flag));

        graph.set_flag(flag, true);
        assert!(graph.flag_value(flag));

        graph.set_flag(flag, false);
        assert!(!graph.flag_value(flag));
    }

    // -- 10. add_graphics_pass + build --------------------------------------

    #[test]
    fn add_graphics_pass_and_build() {
        let mut graph = RenderGraph::new();
        let img = graph.create_image("color", ImageDesc::default());

        graph
            .add_graphics_pass("forward")
            .sample_image(img, vk::PipelineStageFlags2::FRAGMENT_SHADER)
            .build();

        assert_eq!(graph.passes.len(), 1);
        assert_eq!(graph.passes[0].name, "forward");
        assert_eq!(graph.passes[0].queue, QueueType::Graphics);
        assert_eq!(graph.passes[0].accesses.len(), 1);
    }

    // -- 11. import_image marks resource as Imported -------------------------

    #[test]
    fn import_image_marks_as_imported() {
        let mut graph = RenderGraph::new();
        let handle = graph.import_image("swapchain", ImageDesc::default());

        assert!(matches!(
            graph.images[handle.index as usize].resource.kind,
            ResourceKind::Imported,
        ));
    }

    // -- 12. multiple images get sequential indices --------------------------

    #[test]
    fn multiple_images_get_sequential_indices() {
        let mut graph = RenderGraph::new();
        let a = graph.create_image("a", ImageDesc::default());
        let b = graph.create_image("b", ImageDesc::default());
        let c = graph.create_image("c", ImageDesc::default());

        assert_eq!(a.index, 0);
        assert_eq!(b.index, 1);
        assert_eq!(c.index, 2);
        assert_eq!(graph.images.len(), 3);
    }
}
