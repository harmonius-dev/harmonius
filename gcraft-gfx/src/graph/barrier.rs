//! Barrier deduction for automatic Vulkan synchronization.
//!
//! Given the topologically sorted list of pass accesses, this module deduces
//! the minimal set of pipeline barriers (using Vulkan 1.3 synchronization2)
//! required to satisfy all memory and execution dependencies.

use ash::vk;

use crate::graph::pass::ResourceAccess;
use crate::graph::resource::SubresourceRange;

// ---------------------------------------------------------------------------
// BarrierInfo
// ---------------------------------------------------------------------------

/// Describes a single memory barrier to insert between two passes.
#[derive(Clone, Debug)]
pub(crate) struct BarrierInfo {
    /// Resource index this barrier applies to.
    pub resource_index: u16,
    /// Whether this is an image (true) or buffer (false) barrier.
    pub is_image: bool,
    /// Source pipeline stage.
    pub src_stage: vk::PipelineStageFlags2,
    /// Source access mask.
    pub src_access: vk::AccessFlags2,
    /// Destination pipeline stage.
    pub dst_stage: vk::PipelineStageFlags2,
    /// Destination access mask.
    pub dst_access: vk::AccessFlags2,
    /// Old image layout (only meaningful for images).
    pub old_layout: vk::ImageLayout,
    /// New image layout (only meaningful for images).
    pub new_layout: vk::ImageLayout,
    /// Subresource range (only meaningful for images).
    pub subresource: Option<SubresourceRange>,
    /// Source queue family index for queue ownership transfer (VK_QUEUE_FAMILY_IGNORED if not needed).
    pub src_queue_family: u32,
    /// Destination queue family index for queue ownership transfer.
    pub dst_queue_family: u32,
}

// ---------------------------------------------------------------------------
// QueueOwnershipTransfer
// ---------------------------------------------------------------------------

/// A pair of barriers for cross-queue resource transfer.
#[derive(Clone, Debug)]
pub(crate) struct QueueOwnershipTransfer {
    /// Release barrier (executed on the source queue).
    pub release: BarrierInfo,
    /// Acquire barrier (executed on the destination queue).
    pub acquire: BarrierInfo,
}

// ---------------------------------------------------------------------------
// PassBarriers
// ---------------------------------------------------------------------------

/// All barriers to be executed before a given pass.
#[derive(Clone, Debug, Default)]
pub(crate) struct PassBarriers {
    /// Barriers to execute before this pass on the same queue.
    pub barriers: Vec<BarrierInfo>,
    /// Queue ownership transfers involving this pass.
    pub transfers: Vec<QueueOwnershipTransfer>,
}

// ---------------------------------------------------------------------------
// Barrier deduction
// ---------------------------------------------------------------------------

/// Determine whether a barrier is needed between two consecutive accesses to
/// the same resource (ordered by topological sort) and, if so, return the
/// minimal barrier.
///
/// Returns `None` when no hazard exists — specifically, read-after-read with
/// the same image layout requires no synchronization.
pub(crate) fn deduce_barrier(
    prev: &ResourceAccess,
    next: &ResourceAccess,
    is_image: bool,
    subresource: Option<SubresourceRange>,
) -> Option<BarrierInfo> {
    // Read-after-read with the same layout needs no barrier.
    let prev_read_only = prev.version_write.is_none();
    let next_read_only = next.version_write.is_none();
    if prev_read_only && next_read_only && prev.layout == next.layout {
        return None;
    }

    Some(BarrierInfo {
        resource_index: prev.resource_index,
        is_image,
        src_stage: prev.stage,
        src_access: prev.access,
        dst_stage: next.stage,
        dst_access: next.access,
        old_layout: prev.layout,
        new_layout: next.layout,
        subresource,
        src_queue_family: vk::QUEUE_FAMILY_IGNORED,
        dst_queue_family: vk::QUEUE_FAMILY_IGNORED,
    })
}

/// Produce a release/acquire barrier pair for a cross-queue ownership transfer.
///
/// The release barrier is executed on the source queue to make writes visible,
/// and the acquire barrier is executed on the destination queue to make the
/// resource available for the next access.
pub(crate) fn deduce_queue_transfer(
    prev: &ResourceAccess,
    next: &ResourceAccess,
    is_image: bool,
    subresource: Option<SubresourceRange>,
    src_queue_family: u32,
    dst_queue_family: u32,
) -> QueueOwnershipTransfer {
    // Release barrier on source queue: drain source stages/access, but leave
    // destination stages/access empty — the acquire side handles those.
    let release = BarrierInfo {
        resource_index: prev.resource_index,
        is_image,
        src_stage: prev.stage,
        src_access: prev.access,
        dst_stage: vk::PipelineStageFlags2::NONE,
        dst_access: vk::AccessFlags2::NONE,
        old_layout: prev.layout,
        new_layout: next.layout,
        subresource,
        src_queue_family,
        dst_queue_family,
    };

    // Acquire barrier on destination queue: source stages/access are empty
    // because the release side already handled the flush.
    let acquire = BarrierInfo {
        resource_index: next.resource_index,
        is_image,
        src_stage: vk::PipelineStageFlags2::NONE,
        src_access: vk::AccessFlags2::NONE,
        dst_stage: next.stage,
        dst_access: next.access,
        old_layout: prev.layout,
        new_layout: next.layout,
        subresource,
        src_queue_family,
        dst_queue_family,
    };

    QueueOwnershipTransfer { release, acquire }
}

/// Emit a global memory barrier for transient resource memory aliasing.
///
/// When two transient resources share the same backing memory (aliasing), an
/// execution barrier is required between the last use of one and the first use
/// of the other to avoid undefined behaviour.  This barrier uses
/// `ALL_COMMANDS` stages with no access flags — a pure execution dependency.
pub(crate) fn aliasing_barrier(resource_index: u16) -> BarrierInfo {
    BarrierInfo {
        resource_index,
        is_image: false, // memory barrier, not image/buffer specific
        src_stage: vk::PipelineStageFlags2::ALL_COMMANDS,
        src_access: vk::AccessFlags2::NONE,
        dst_stage: vk::PipelineStageFlags2::ALL_COMMANDS,
        dst_access: vk::AccessFlags2::NONE,
        old_layout: vk::ImageLayout::UNDEFINED,
        new_layout: vk::ImageLayout::UNDEFINED,
        subresource: None,
        src_queue_family: vk::QUEUE_FAMILY_IGNORED,
        dst_queue_family: vk::QUEUE_FAMILY_IGNORED,
    }
}

/// Merge compatible barriers. Barriers between the same two passes targeting
/// different resources are batched into a single `vkCmdPipelineBarrier2` call.
/// This function doesn't change the barrier semantics, just organizes them
/// for efficient submission.
pub(crate) fn merge_barriers(barriers: &mut Vec<BarrierInfo>) {
    // Stable sort by resource_index so image barriers for the same resource
    // are adjacent (makes building VkDependencyInfo easier later).
    barriers.sort_by_key(|b| b.resource_index);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn read_access(index: u16, stage: vk::PipelineStageFlags2, layout: vk::ImageLayout) -> ResourceAccess {
        ResourceAccess {
            resource_index: index,
            version_read: Some(0),
            version_write: None,
            stage,
            access: vk::AccessFlags2::SHADER_SAMPLED_READ,
            layout,
            condition: None,
        }
    }

    fn write_access(index: u16, stage: vk::PipelineStageFlags2, layout: vk::ImageLayout) -> ResourceAccess {
        ResourceAccess {
            resource_index: index,
            version_read: Some(0),
            version_write: Some(1),
            stage,
            access: vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
            layout,
            condition: None,
        }
    }

    #[test]
    fn read_after_read_same_layout_no_barrier() {
        let a = read_access(0, vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let b = read_access(0, vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        assert!(deduce_barrier(&a, &b, true, None).is_none());
    }

    #[test]
    fn read_after_read_different_layout_needs_barrier() {
        let a = read_access(0, vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let b = read_access(0, vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::ImageLayout::GENERAL);
        let barrier = deduce_barrier(&a, &b, true, None);
        assert!(barrier.is_some());
        let b = barrier.unwrap();
        assert_eq!(b.old_layout, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        assert_eq!(b.new_layout, vk::ImageLayout::GENERAL);
    }

    #[test]
    fn write_after_read_needs_barrier() {
        let a = read_access(0, vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let b = write_access(0, vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let barrier = deduce_barrier(&a, &b, true, None);
        assert!(barrier.is_some());
        let b = barrier.unwrap();
        assert_eq!(b.src_stage, vk::PipelineStageFlags2::FRAGMENT_SHADER);
        assert_eq!(b.dst_stage, vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT);
    }

    #[test]
    fn read_after_write_needs_barrier() {
        let a = write_access(0, vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let b = read_access(0, vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let barrier = deduce_barrier(&a, &b, true, None);
        assert!(barrier.is_some());
    }

    #[test]
    fn deduce_barrier_sets_queue_family_ignored() {
        let a = write_access(0, vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let b = read_access(0, vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let barrier = deduce_barrier(&a, &b, true, None).unwrap();
        assert_eq!(barrier.src_queue_family, vk::QUEUE_FAMILY_IGNORED);
        assert_eq!(barrier.dst_queue_family, vk::QUEUE_FAMILY_IGNORED);
    }

    #[test]
    fn queue_transfer_release_has_no_dst_stage() {
        let a = write_access(0, vk::PipelineStageFlags2::COMPUTE_SHADER, vk::ImageLayout::GENERAL);
        let b = read_access(0, vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let transfer = deduce_queue_transfer(&a, &b, true, None, 0, 1);
        assert_eq!(transfer.release.src_stage, vk::PipelineStageFlags2::COMPUTE_SHADER);
        assert_eq!(transfer.release.dst_stage, vk::PipelineStageFlags2::NONE);
        assert_eq!(transfer.release.dst_access, vk::AccessFlags2::NONE);
    }

    #[test]
    fn queue_transfer_acquire_has_no_src_stage() {
        let a = write_access(0, vk::PipelineStageFlags2::COMPUTE_SHADER, vk::ImageLayout::GENERAL);
        let b = read_access(0, vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let transfer = deduce_queue_transfer(&a, &b, true, None, 0, 1);
        assert_eq!(transfer.acquire.src_stage, vk::PipelineStageFlags2::NONE);
        assert_eq!(transfer.acquire.src_access, vk::AccessFlags2::NONE);
        assert_eq!(transfer.acquire.dst_stage, vk::PipelineStageFlags2::FRAGMENT_SHADER);
    }

    #[test]
    fn queue_transfer_preserves_queue_families() {
        let a = write_access(0, vk::PipelineStageFlags2::COMPUTE_SHADER, vk::ImageLayout::GENERAL);
        let b = read_access(0, vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let transfer = deduce_queue_transfer(&a, &b, true, None, 2, 5);
        assert_eq!(transfer.release.src_queue_family, 2);
        assert_eq!(transfer.release.dst_queue_family, 5);
        assert_eq!(transfer.acquire.src_queue_family, 2);
        assert_eq!(transfer.acquire.dst_queue_family, 5);
    }

    #[test]
    fn aliasing_barrier_uses_all_commands() {
        let b = aliasing_barrier(42);
        assert_eq!(b.resource_index, 42);
        assert_eq!(b.src_stage, vk::PipelineStageFlags2::ALL_COMMANDS);
        assert_eq!(b.dst_stage, vk::PipelineStageFlags2::ALL_COMMANDS);
        assert_eq!(b.src_access, vk::AccessFlags2::NONE);
        assert_eq!(b.dst_access, vk::AccessFlags2::NONE);
        assert_eq!(b.old_layout, vk::ImageLayout::UNDEFINED);
        assert_eq!(b.new_layout, vk::ImageLayout::UNDEFINED);
        assert!(!b.is_image);
    }

    #[test]
    fn merge_barriers_sorts_by_resource_index() {
        let mut barriers = vec![
            aliasing_barrier(3),
            aliasing_barrier(1),
            aliasing_barrier(2),
            aliasing_barrier(1),
        ];
        merge_barriers(&mut barriers);
        let indices: Vec<u16> = barriers.iter().map(|b| b.resource_index).collect();
        assert_eq!(indices, vec![1, 1, 2, 3]);
    }

    #[test]
    fn buffer_barrier_has_undefined_layouts() {
        let a = ResourceAccess {
            resource_index: 0,
            version_read: Some(0),
            version_write: Some(1),
            stage: vk::PipelineStageFlags2::COMPUTE_SHADER,
            access: vk::AccessFlags2::SHADER_STORAGE_WRITE,
            layout: vk::ImageLayout::UNDEFINED,
            condition: None,
        };
        let b = ResourceAccess {
            resource_index: 0,
            version_read: Some(1),
            version_write: None,
            stage: vk::PipelineStageFlags2::COMPUTE_SHADER,
            access: vk::AccessFlags2::SHADER_STORAGE_READ,
            layout: vk::ImageLayout::UNDEFINED,
            condition: None,
        };
        let barrier = deduce_barrier(&a, &b, false, None).unwrap();
        assert!(!barrier.is_image);
        assert_eq!(barrier.old_layout, vk::ImageLayout::UNDEFINED);
        assert_eq!(barrier.new_layout, vk::ImageLayout::UNDEFINED);
    }

    #[test]
    fn deduce_barrier_propagates_subresource() {
        let a = write_access(0, vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let b = read_access(0, vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let sub = SubresourceRange {
            base_mip_level: 1,
            mip_count: 3,
            base_array_layer: 0,
            layer_count: 6,
        };
        let barrier = deduce_barrier(&a, &b, true, Some(sub)).unwrap();
        let s = barrier.subresource.unwrap();
        assert_eq!(s.base_mip_level, 1);
        assert_eq!(s.mip_count, 3);
        assert_eq!(s.base_array_layer, 0);
        assert_eq!(s.layer_count, 6);
    }
}
