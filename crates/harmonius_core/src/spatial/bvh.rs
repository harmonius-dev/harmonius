use std::cmp::Ordering;

use glam::Vec3;

use super::aabb::Aabb;
use super::entity::Entity;
use super::error::SpatialError;
use super::handle::BvhHandle;
use super::layers::SpatialLayerMask;

const LEAF_BIT: u32 = 1 << 31;
const INVALID: u32 = u32::MAX;

/// Build-time and runtime tuning for [`BvhIndex`].
#[derive(Clone, Debug)]
pub struct BvhConfig {
    /// Relative traversal cost weight for SAH.
    pub traversal_cost: f32,
    /// Number of SAH bins per axis.
    pub sah_bins: u32,
    /// Extra margin applied to fattened leaf bounds.
    pub fat_aabb_margin: f32,
    /// Quality ratio that triggers a background rebuild in the full engine.
    pub rebuild_quality_threshold: f32,
    /// Leaf count at which SAH stops subdividing.
    pub leaf_threshold: u32,
}

impl Default for BvhConfig {
    fn default() -> Self {
        Self {
            traversal_cost: 1.0,
            sah_bins: 12,
            fat_aabb_margin: 0.1,
            rebuild_quality_threshold: 2.0,
            leaf_threshold: 8,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct BvhNode {
    pub min: Vec3,
    pub max: Vec3,
    pub left: u32,
    pub right: u32,
}

impl BvhNode {
    fn aabb(self) -> Aabb {
        Aabb::new(self.min, self.max)
    }

    fn set_aabb(&mut self, aabb: Aabb) {
        self.min = aabb.min;
        self.max = aabb.max;
    }

    fn is_leaf(left: u32) -> bool {
        (left & LEAF_BIT) != 0
    }

    fn leaf_index(left: u32) -> u32 {
        left & !LEAF_BIT
    }

    fn encode_leaf(leaf: u32) -> u32 {
        LEAF_BIT | leaf
    }
}

pub(crate) struct LeafEntry {
    pub entity: Entity,
    pub aabb: Aabb,
    pub layers: SpatialLayerMask,
    pub fat_aabb: Aabb,
    pub node_index: u32,
}

struct HandleSlot {
    generation: u32,
    leaf_index: u32,
}

/// Bounding volume hierarchy index shared across gameplay queries.
pub struct BvhIndex {
    nodes: Vec<BvhNode>,
    parents: Vec<u32>,
    leaves: Vec<LeafEntry>,
    handles: Vec<HandleSlot>,
    free_handles: Vec<u32>,
    root: u32,
    pub(crate) config: BvhConfig,
    sah_quality: f32,
    entity_count: u32,
    #[cfg(test)]
    pub(crate) test_rebuild_calls: u32,
    #[cfg(test)]
    pub(crate) test_propagation_steps: u32,
}

impl BvhIndex {
    /// Creates an empty BVH with the supplied configuration.
    #[must_use]
    pub fn new(config: BvhConfig) -> Self {
        Self {
            nodes: Vec::new(),
            parents: Vec::new(),
            leaves: Vec::new(),
            handles: Vec::new(),
            free_handles: Vec::new(),
            root: INVALID,
            config,
            sah_quality: 1.0,
            entity_count: 0,
            #[cfg(test)]
            test_rebuild_calls: 0,
            #[cfg(test)]
            test_propagation_steps: 0,
        }
    }

    /// Inserts a leaf and returns its stable handle.
    pub fn insert(&mut self, entity: Entity, aabb: Aabb, layers: SpatialLayerMask) -> BvhHandle {
        let fat = aabb.expanded(self.config.fat_aabb_margin);
        let leaf_index = self.leaves.len() as u32;
        self.leaves.push(LeafEntry {
            entity,
            aabb,
            layers,
            fat_aabb: fat,
            node_index: INVALID,
        });
        let handle = self.alloc_handle(leaf_index);
        self.entity_count = self.entity_count.saturating_add(1);
        self.rebuild_full();
        handle
    }

    /// Batch-inserts many leaves, rebuilding once at the end.
    pub fn batch_insert(&mut self, entries: &[(Entity, Aabb, SpatialLayerMask)]) -> Vec<BvhHandle> {
        let mut out = Vec::with_capacity(entries.len());
        for (entity, aabb, layers) in entries {
            let fat = aabb.expanded(self.config.fat_aabb_margin);
            let leaf_index = self.leaves.len() as u32;
            self.leaves.push(LeafEntry {
                entity: *entity,
                aabb: *aabb,
                layers: *layers,
                fat_aabb: fat,
                node_index: INVALID,
            });
            out.push(self.alloc_handle(leaf_index));
            self.entity_count = self.entity_count.saturating_add(1);
        }
        self.rebuild_full();
        out
    }

    /// Removes a leaf by handle.
    pub fn remove(&mut self, handle: BvhHandle) -> Result<(), SpatialError> {
        let slot = self
            .handles
            .get(handle.index() as usize)
            .ok_or(SpatialError::StaleHandle { handle })?;
        if slot.generation != handle.generation() {
            return Err(SpatialError::StaleHandle { handle });
        }
        let leaf_index = slot.leaf_index as usize;
        if leaf_index >= self.leaves.len() {
            return Err(SpatialError::StaleHandle { handle });
        }

        let last = self.leaves.len() - 1;
        if leaf_index != last {
            self.leaves.swap(leaf_index, last);
            for hs in &mut self.handles {
                if hs.leaf_index as usize == last {
                    hs.leaf_index = leaf_index as u32;
                    break;
                }
            }
        }
        self.leaves.pop();
        self.free_handles.push(handle.index());
        self.handles[handle.index() as usize] = HandleSlot {
            generation: handle.generation().saturating_add(1),
            leaf_index: INVALID,
        };
        self.entity_count = self.entity_count.saturating_sub(1);
        self.rebuild_full();
        Ok(())
    }

    /// Updates a leaf's bounds and optional layer mask.
    pub fn update(
        &mut self,
        handle: BvhHandle,
        new_aabb: Aabb,
        new_layers: SpatialLayerMask,
    ) -> Result<(), SpatialError> {
        let slot = self
            .handles
            .get(handle.index() as usize)
            .ok_or(SpatialError::StaleHandle { handle })?;
        if slot.generation != handle.generation() {
            return Err(SpatialError::StaleHandle { handle });
        }
        let leaf_index = slot.leaf_index as usize;
        let node_index = self
            .leaves
            .get(leaf_index)
            .ok_or(SpatialError::StaleHandle { handle })?
            .node_index;
        let fits_fat = self
            .leaves
            .get(leaf_index)
            .ok_or(SpatialError::StaleHandle { handle })?
            .fat_aabb
            .contains_aabb(&new_aabb);

        if fits_fat {
            {
                let leaf = &mut self.leaves[leaf_index];
                leaf.aabb = new_aabb;
                leaf.layers = new_layers;
            }
            self.refresh_leaf_node_bounds(leaf_index);
            self.propagate_from_parent(node_index);
            Ok(())
        } else {
            {
                let leaf = &mut self.leaves[leaf_index];
                leaf.aabb = new_aabb;
                leaf.layers = new_layers;
                leaf.fat_aabb = new_aabb.expanded(self.config.fat_aabb_margin);
            }
            self.rebuild_full();
            Ok(())
        }
    }

    /// Applies many positional updates, refitting in place when fat bounds allow.
    pub fn refit_moved(&mut self, updates: &[(BvhHandle, Aabb)]) {
        for (handle, new_aabb) in updates {
            let _ = self.refit_one(*handle, *new_aabb);
        }
    }

    fn refit_one(&mut self, handle: BvhHandle, new_aabb: Aabb) -> Result<(), SpatialError> {
        let slot = self
            .handles
            .get(handle.index() as usize)
            .ok_or(SpatialError::StaleHandle { handle })?;
        if slot.generation != handle.generation() {
            return Err(SpatialError::StaleHandle { handle });
        }
        let leaf_index = slot.leaf_index as usize;
        let layers = self
            .leaves
            .get(leaf_index)
            .ok_or(SpatialError::StaleHandle { handle })?
            .layers;

        if self.leaf_fits_fat(leaf_index, &new_aabb) {
            let node_index = self.leaves[leaf_index].node_index;
            {
                let leaf = &mut self.leaves[leaf_index];
                leaf.aabb = new_aabb;
            }
            self.refresh_leaf_node_bounds(leaf_index);
            self.propagate_from_parent(node_index);
            Ok(())
        } else {
            self.update(handle, new_aabb, layers)
        }
    }

    fn leaf_fits_fat(&self, leaf_index: usize, new_aabb: &Aabb) -> bool {
        self.leaves
            .get(leaf_index)
            .map(|leaf| leaf.fat_aabb.contains_aabb(new_aabb))
            .unwrap_or(false)
    }

    fn refresh_leaf_node_bounds(&mut self, leaf_index: usize) {
        let node_index = self.leaves[leaf_index].node_index;
        if node_index == INVALID {
            return;
        }
        let aabb = self.leaves[leaf_index].aabb;
        self.nodes[node_index as usize].set_aabb(aabb);
    }

    /// Rebuilds the entire hierarchy from the current leaf set.
    pub fn rebuild_full(&mut self) {
        #[cfg(test)]
        {
            self.test_rebuild_calls = self.test_rebuild_calls.saturating_add(1);
        }
        self.nodes.clear();
        self.parents.clear();
        if self.leaves.is_empty() {
            self.root = INVALID;
            self.sah_quality = 1.0;
            return;
        }

        let indices: Vec<u32> = (0..self.leaves.len() as u32).collect();
        let root = self.build_subtree(&indices);
        self.root = root;
        if self.root != INVALID {
            self.parents[self.root as usize] = INVALID;
        }
        self.repair_leaf_node_links();
        self.sah_quality = self.compute_sah_quality();
    }

    /// Current SAH quality metric (`1.0` is optimal for this builder).
    #[must_use]
    pub fn sah_quality(&self) -> f32 {
        self.sah_quality
    }

    /// Total node count including leaves.
    #[must_use]
    pub fn node_count(&self) -> u32 {
        self.nodes.len() as u32
    }

    /// Number of indexed entities.
    #[must_use]
    pub fn entity_count(&self) -> u32 {
        self.entity_count
    }

    /// Traverses leaves whose AABB overlaps `query` and match `layer_mask`.
    pub(crate) fn for_each_overlapping_leaf(
        &self,
        query: &Aabb,
        layer_mask: SpatialLayerMask,
        mut f: impl FnMut(&LeafEntry) -> bool,
    ) {
        if self.root == INVALID {
            return;
        }
        let mut stack: Vec<u32> = vec![self.root];
        while let Some(node_index) = stack.pop() {
            let node = self.nodes[node_index as usize];
            if !node.aabb().intersects(query) {
                continue;
            }
            if BvhNode::is_leaf(node.left) {
                let li = BvhNode::leaf_index(node.left) as usize;
                if let Some(leaf) = self.leaves.get(li) {
                    if leaf.layers.contains(layer_mask) && leaf.aabb.intersects(query) && !f(leaf) {
                        return;
                    }
                }
            } else {
                let left = node.left;
                let right = node.right;
                if right != INVALID {
                    stack.push(right);
                }
                if left != INVALID {
                    stack.push(left);
                }
            }
        }
    }

    /// Ray traversal in world space.
    pub(crate) fn traverse_ray(
        &self,
        origin: Vec3,
        direction: Vec3,
        t_max: f32,
        layer_mask: SpatialLayerMask,
        mut visitor: impl FnMut(&LeafEntry, f32) -> bool,
    ) {
        if self.root == INVALID {
            return;
        }
        let inv_dir = Vec3::new(
            safe_inv(direction.x),
            safe_inv(direction.y),
            safe_inv(direction.z),
        );
        let mut stack: Vec<u32> = vec![self.root];
        while let Some(node_index) = stack.pop() {
            let node = self.nodes[node_index as usize];
            if node.aabb().intersects_ray(origin, inv_dir, t_max).is_none() {
                continue;
            }
            if BvhNode::is_leaf(node.left) {
                let li = BvhNode::leaf_index(node.left) as usize;
                if let Some(leaf) = self.leaves.get(li) {
                    if leaf.layers.contains(layer_mask) {
                        if let Some(t) = leaf.aabb.intersects_ray(origin, inv_dir, t_max) {
                            if !visitor(leaf, t) {
                                return;
                            }
                        }
                    }
                }
            } else {
                let left = node.left;
                let right = node.right;
                if right != INVALID {
                    stack.push(right);
                }
                if left != INVALID {
                    stack.push(left);
                }
            }
        }
    }

    fn alloc_handle(&mut self, leaf_index: u32) -> BvhHandle {
        if let Some(slot_index) = self.free_handles.pop() {
            let gen = self.handles[slot_index as usize].generation;
            self.handles[slot_index as usize] = HandleSlot {
                generation: gen,
                leaf_index,
            };
            BvhHandle::new(slot_index, gen)
        } else {
            let slot_index = self.handles.len() as u32;
            self.handles.push(HandleSlot {
                generation: 0,
                leaf_index,
            });
            BvhHandle::new(slot_index, 0)
        }
    }

    fn propagate_from_parent(&mut self, mut node_index: u32) {
        while node_index != INVALID {
            #[cfg(test)]
            {
                self.test_propagation_steps = self.test_propagation_steps.saturating_add(1);
            }
            let parent = self
                .parents
                .get(node_index as usize)
                .copied()
                .unwrap_or(INVALID);
            if parent == INVALID {
                break;
            }
            let p_left = self.nodes[parent as usize].left;
            let p_right = self.nodes[parent as usize].right;
            let new_aabb = merge_children_aabb(self, p_left, p_right);
            self.nodes[parent as usize].set_aabb(new_aabb);
            node_index = parent;
        }
    }

    fn repair_leaf_node_links(&mut self) {
        for leaf in &mut self.leaves {
            leaf.node_index = INVALID;
        }
        if self.root == INVALID {
            return;
        }
        let mut stack = vec![self.root];
        while let Some(node_index) = stack.pop() {
            let node = self.nodes[node_index as usize];
            if BvhNode::is_leaf(node.left) {
                let li = BvhNode::leaf_index(node.left) as usize;
                if let Some(leaf) = self.leaves.get_mut(li) {
                    leaf.node_index = node_index;
                }
            } else {
                let left = node.left;
                let right = node.right;
                if right != INVALID {
                    stack.push(right);
                }
                if left != INVALID {
                    stack.push(left);
                }
            }
        }
    }

    fn compute_sah_quality(&self) -> f32 {
        if self.leaves.is_empty() {
            return 1.0;
        }
        let built = self.total_sah_cost();
        // Reference cost: naive one-root box cost.
        let root_only = self
            .leaves
            .iter()
            .fold(self.leaves[0].aabb, |acc, l| acc.merged(&l.aabb))
            .surface_area()
            * self.leaves.len() as f32;
        if root_only <= f32::EPSILON {
            1.0
        } else {
            (built / root_only).max(1.0e-4)
        }
    }

    fn total_sah_cost(&self) -> f32 {
        let mut sum = 0.0_f32;
        for (i, node) in self.nodes.iter().enumerate() {
            if BvhNode::is_leaf(node.left) {
                continue;
            }
            let a = node.aabb().surface_area();
            let lc = subtree_leaf_count(self, i as u32);
            sum += a * lc as f32;
        }
        sum
    }

    fn build_subtree(&mut self, leaf_indices: &[u32]) -> u32 {
        debug_assert!(!leaf_indices.is_empty());
        if leaf_indices.len() == 1 {
            let li = leaf_indices[0];
            let aabb = self.leaves[li as usize].aabb;
            let node_index = self.push_node(BvhNode {
                min: aabb.min,
                max: aabb.max,
                left: BvhNode::encode_leaf(li),
                right: INVALID,
            });
            return node_index;
        }

        if leaf_indices.len() <= self.config.leaf_threshold as usize {
            return self.build_leaf_cluster(leaf_indices);
        }

        let (left_set, right_set) = partition_median(self, leaf_indices);
        if left_set.is_empty() || right_set.is_empty() {
            return self.build_leaf_cluster(leaf_indices);
        }
        let left_child = self.build_subtree(&left_set);
        let right_child = self.build_subtree(&right_set);
        let bounds = merge_node_bounds(self, left_child, right_child);
        let node_index = self.push_node(BvhNode {
            min: bounds.min,
            max: bounds.max,
            left: left_child,
            right: right_child,
        });
        self.parents[left_child as usize] = node_index;
        self.parents[right_child as usize] = node_index;
        node_index
    }

    fn build_leaf_cluster(&mut self, leaf_indices: &[u32]) -> u32 {
        let mut nodes: Vec<u32> = Vec::new();
        for &li in leaf_indices {
            let aabb = self.leaves[li as usize].aabb;
            let ni = self.push_node(BvhNode {
                min: aabb.min,
                max: aabb.max,
                left: BvhNode::encode_leaf(li),
                right: INVALID,
            });
            nodes.push(ni);
        }
        while nodes.len() > 1 {
            let a = nodes.pop().unwrap();
            let b = nodes.pop().unwrap();
            let bounds = merge_node_bounds(self, a, b);
            let parent = self.push_node(BvhNode {
                min: bounds.min,
                max: bounds.max,
                left: a,
                right: b,
            });
            self.parents[a as usize] = parent;
            self.parents[b as usize] = parent;
            nodes.push(parent);
        }
        nodes[0]
    }

    fn push_node(&mut self, node: BvhNode) -> u32 {
        let index = self.nodes.len() as u32;
        self.nodes.push(node);
        self.parents.push(INVALID);
        index
    }

    /// Upper bound on bytes used by nodes, parents, leaves, and handles for `entity_count`.
    #[must_use]
    pub fn memory_upper_bound_bytes(&self) -> usize {
        let n = self.entity_count as usize;
        let nodes = std::mem::size_of::<BvhNode>() * n.saturating_mul(2).saturating_sub(1).max(1);
        let parents = std::mem::size_of::<u32>() * n.saturating_mul(2).saturating_sub(1).max(1);
        let leaves = std::mem::size_of::<LeafEntry>() * n;
        let handles = std::mem::size_of::<HandleSlot>() * self.handles.len().max(n);
        nodes + parents + leaves + handles
    }
}

pub(crate) fn safe_inv(axis: f32) -> f32 {
    if axis.abs() < 1e-8 {
        1e8 * if axis >= 0.0 { 1.0 } else { -1.0 }
    } else {
        1.0 / axis
    }
}

fn partition_median(bvh: &BvhIndex, leaf_indices: &[u32]) -> (Vec<u32>, Vec<u32>) {
    let mut cmin = Vec3::splat(f32::INFINITY);
    let mut cmax = Vec3::splat(f32::NEG_INFINITY);
    for &li in leaf_indices {
        let c = bvh.leaves[li as usize].aabb.center();
        cmin = cmin.min(c);
        cmax = cmax.max(c);
    }
    let extent = cmax - cmin;
    let mut best_axis = 0usize;
    let mut best_extent = extent.x;
    if extent.y > best_extent {
        best_extent = extent.y;
        best_axis = 1;
    }
    if extent.z > best_extent {
        best_axis = 2;
    }

    let mut sorted: Vec<u32> = leaf_indices.to_vec();
    sorted.sort_by(|&a, &b| {
        let ca = bvh.leaves[a as usize].aabb.center()[best_axis];
        let cb = bvh.leaves[b as usize].aabb.center()[best_axis];
        ca.partial_cmp(&cb).unwrap_or(Ordering::Equal)
    });
    let mid = (sorted.len() / 2).max(1);
    let left = sorted[..mid].to_vec();
    let right = sorted[mid..].to_vec();
    (left, right)
}

fn merge_children_aabb(bvh: &BvhIndex, left: u32, right: u32) -> Aabb {
    let mut acc = node_aabb(bvh, left);
    if right != INVALID {
        acc = acc.merged(&node_aabb(bvh, right));
    }
    acc
}

fn merge_node_bounds(bvh: &BvhIndex, left: u32, right: u32) -> Aabb {
    merge_children_aabb(bvh, left, right)
}

fn node_aabb(bvh: &BvhIndex, node_index: u32) -> Aabb {
    let node = bvh.nodes[node_index as usize];
    if BvhNode::is_leaf(node.left) {
        let li = BvhNode::leaf_index(node.left) as usize;
        bvh.leaves[li].aabb
    } else {
        node.aabb()
    }
}

fn subtree_leaf_count(bvh: &BvhIndex, node_index: u32) -> u32 {
    let mut count = 0u32;
    let mut stack = vec![node_index];
    while let Some(ni) = stack.pop() {
        let node = bvh.nodes[ni as usize];
        if BvhNode::is_leaf(node.left) {
            count += 1;
        } else {
            if node.right != INVALID {
                stack.push(node.right);
            }
            stack.push(node.left);
        }
    }
    count
}
