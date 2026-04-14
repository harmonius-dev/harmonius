//! Minimal ECS-like world backing scene transform tests.

use glam::{Mat3, Mat4, Vec3};

use super::commands::{HierarchyCommand, HierarchyCommandBuffer};
use super::entity::Entity;
use super::error::SceneError;
use super::hierarchy::{Children, HierarchyEvent, Parent};
use super::resources::Resources;
use super::transform::{GlobalTransform, PreviousGlobalTransform, Transform};
use super::transform2d::{GlobalTransform2D, PreviousGlobalTransform2D, Transform2D};
#[derive(Debug)]
struct Slot {
    generation: u32,
    alive: bool,
}

/// Minimal runtime world for transform and hierarchy experiments.
#[derive(Debug)]
pub struct World {
    slots: Vec<Slot>,
    free: Vec<u32>,
    transforms: Vec<Option<Transform>>,
    globals: Vec<Option<GlobalTransform>>,
    prevs: Vec<Option<PreviousGlobalTransform>>,
    transforms2d: Vec<Option<Transform2D>>,
    globals2d: Vec<Option<GlobalTransform2D>>,
    prevs2d: Vec<Option<PreviousGlobalTransform2D>>,
    parents: Vec<Option<Parent>>,
    children: Vec<Option<Children>>,
    transform_dirty: Vec<bool>,
    transform2d_dirty: Vec<bool>,
    events: Vec<HierarchyEvent>,
    commands: HierarchyCommandBuffer,
    propagation_epoch: u64,
    /// Shared engine resources (currently only BVH).
    pub resources: Resources,
}

impl World {
    /// Creates an empty world.
    #[must_use]
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free: Vec::new(),
            transforms: Vec::new(),
            globals: Vec::new(),
            prevs: Vec::new(),
            transforms2d: Vec::new(),
            globals2d: Vec::new(),
            prevs2d: Vec::new(),
            parents: Vec::new(),
            children: Vec::new(),
            transform_dirty: Vec::new(),
            transform2d_dirty: Vec::new(),
            events: Vec::new(),
            commands: HierarchyCommandBuffer::new(),
            propagation_epoch: 0,
            resources: Resources::default(),
        }
    }

    /// Registers the shared BVH resource (at most one).
    pub fn register_bvh(&mut self, bvh: super::resources::BvhResource) -> Result<(), SceneError> {
        if self.resources.bvh.is_some() {
            return Err(SceneError::DuplicateBvh);
        }
        self.resources.bvh = Some(bvh);
        Ok(())
    }

    /// Queries spatial backends (tests swap in fakes via trait objects).
    pub fn spatial_query_aabb<'a>(
        &'a self,
        backend: &'a mut dyn super::resources::SpatialQueryBackend,
        min: Vec3,
        max: Vec3,
    ) -> &'a [Entity] {
        backend.query_aabb(min, max)
    }

    /// Drains hierarchy events emitted by the last command flush.
    pub fn take_hierarchy_events(&mut self) -> Vec<HierarchyEvent> {
        std::mem::take(&mut self.events)
    }

    /// Mutable access to the deferred hierarchy command buffer.
    pub fn commands_mut(&mut self) -> &mut HierarchyCommandBuffer {
        &mut self.commands
    }

    /// Spawns a 3D transform bundle and returns its entity.
    pub fn spawn_transform(&mut self, transform: Transform) -> Entity {
        let index = self.alloc_slot();
        self.transforms[index] = Some(transform);
        let matrix = transform.local_matrix();
        self.globals[index] = Some(GlobalTransform::from_matrix(matrix));
        self.prevs[index] = Some(PreviousGlobalTransform::IDENTITY);
        self.transform_dirty[index] = false;
        Entity::new(index as u32, self.slots[index].generation)
    }

    /// Spawns a 2D transform bundle and returns its entity.
    pub fn spawn_transform2d(&mut self, transform: Transform2D) -> Entity {
        let index = self.alloc_slot();
        self.transforms2d[index] = Some(transform);
        let matrix = transform.local_matrix();
        self.globals2d[index] = Some(GlobalTransform2D::from_matrix(matrix));
        self.prevs2d[index] = Some(PreviousGlobalTransform2D::IDENTITY);
        self.transform2d_dirty[index] = false;
        Entity::new(index as u32, self.slots[index].generation)
    }

    /// Returns `true` when the entity handle matches a live row.
    #[must_use]
    pub fn is_alive(&self, entity: Entity) -> bool {
        let idx = entity.index() as usize;
        if idx >= self.slots.len() {
            return false;
        }
        let slot = &self.slots[idx];
        slot.alive && slot.generation == entity.generation()
    }

    /// Borrows [`Children`] for `entity`.
    #[must_use]
    pub fn children(&self, entity: Entity) -> Option<&Children> {
        if !self.is_alive(entity) {
            return None;
        }
        self.children
            .get(entity.index() as usize)
            .and_then(|c| c.as_ref())
    }

    /// Borrows [`Parent`] for `entity`.
    #[must_use]
    pub fn parent(&self, entity: Entity) -> Option<Parent> {
        if !self.is_alive(entity) {
            return None;
        }
        self.parents.get(entity.index() as usize).and_then(|p| *p)
    }

    /// Borrows a local [`Transform`].
    #[must_use]
    pub fn transform(&self, entity: Entity) -> Option<&Transform> {
        if !self.is_alive(entity) {
            return None;
        }
        self.transforms
            .get(entity.index() as usize)
            .and_then(|t| t.as_ref())
    }

    /// Mutably borrows a local [`Transform`], marking it dirty.
    pub fn transform_mut(&mut self, entity: Entity) -> Option<&mut Transform> {
        if !self.is_alive(entity) {
            return None;
        }
        let idx = entity.index() as usize;
        self.transform_dirty[idx] = true;
        self.transforms.get_mut(idx).and_then(|t| t.as_mut())
    }

    /// Borrows a [`GlobalTransform`].
    #[must_use]
    pub fn global_transform(&self, entity: Entity) -> Option<&GlobalTransform> {
        if !self.is_alive(entity) {
            return None;
        }
        self.globals
            .get(entity.index() as usize)
            .and_then(|g| g.as_ref())
    }

    /// Borrows a local [`Transform2D`].
    #[must_use]
    pub fn transform2d(&self, entity: Entity) -> Option<&Transform2D> {
        if !self.is_alive(entity) {
            return None;
        }
        self.transforms2d
            .get(entity.index() as usize)
            .and_then(|t| t.as_ref())
    }

    /// Mutably borrows a local [`Transform2D`], marking it dirty.
    pub fn transform2d_mut(&mut self, entity: Entity) -> Option<&mut Transform2D> {
        if !self.is_alive(entity) {
            return None;
        }
        let idx = entity.index() as usize;
        self.transform2d_dirty[idx] = true;
        self.transforms2d.get_mut(idx).and_then(|t| t.as_mut())
    }

    /// Borrows a [`GlobalTransform2D`].
    #[must_use]
    pub fn global_transform2d(&self, entity: Entity) -> Option<&GlobalTransform2D> {
        if !self.is_alive(entity) {
            return None;
        }
        self.globals2d
            .get(entity.index() as usize)
            .and_then(|g| g.as_ref())
    }

    /// Flushes deferred hierarchy commands.
    pub fn flush_hierarchy_commands(&mut self) -> Result<(), SceneError> {
        let commands: Vec<HierarchyCommand> = self.commands.drain().collect();
        if commands.is_empty() {
            return Ok(());
        }
        for command in commands {
            match command {
                HierarchyCommand::SetParent { child, parent } => {
                    self.apply_set_parent(child, parent)?;
                }
                HierarchyCommand::RemoveParent { child } => {
                    self.apply_remove_parent(child)?;
                }
                HierarchyCommand::DespawnRecursive { root } => {
                    self.apply_despawn_recursive(root)?;
                }
                HierarchyCommand::DespawnOrphaning { root } => {
                    self.apply_despawn_orphaning(root)?;
                }
            }
        }
        self.mark_all_transforms_dirty();
        Ok(())
    }

    fn mark_all_transforms_dirty(&mut self) {
        for index in 0..self.slots.len() {
            if !self.slots[index].alive {
                continue;
            }
            if self.transforms[index].is_some() {
                self.transform_dirty[index] = true;
            }
            if self.transforms2d[index].is_some() {
                self.transform2d_dirty[index] = true;
            }
        }
    }

    /// Runs transform propagation for all entities.
    pub fn run_propagation(&mut self) {
        self.propagation_epoch = self.propagation_epoch.saturating_add(1);
        self.run_propagation_3d();
        self.run_propagation_2d();
        self.clear_transform_dirty_flags();
    }

    fn clear_transform_dirty_flags(&mut self) {
        for flag in &mut self.transform_dirty {
            *flag = false;
        }
        for flag in &mut self.transform2d_dirty {
            *flag = false;
        }
    }

    fn run_propagation_3d(&mut self) {
        let refresh = self.compute_refresh_mask_3d();
        self.propagate_roots_serial_3d(&refresh);
    }

    fn propagate_roots_serial_3d(&mut self, refresh: &[bool]) {
        let roots = self.collect_roots_3d();
        for root in roots {
            self.propagate_subtree_3d(root, refresh);
        }
    }

    fn run_propagation_2d(&mut self) {
        let roots = self.collect_roots_2d();
        let refresh = self.compute_refresh_mask_2d();
        for root in roots {
            self.propagate_subtree_2d(root, &refresh);
        }
    }

    fn collect_roots_3d(&self) -> Vec<Entity> {
        let mut roots = Vec::new();
        for index in 0..self.slots.len() {
            let entity = Entity::new(index as u32, self.slots[index].generation);
            if !self.slots[index].alive {
                continue;
            }
            if self.transforms[index].is_none() {
                continue;
            }
            if self.parents[index].is_some() {
                continue;
            }
            roots.push(entity);
        }
        roots
    }

    fn collect_roots_2d(&self) -> Vec<Entity> {
        let mut roots = Vec::new();
        for index in 0..self.slots.len() {
            let entity = Entity::new(index as u32, self.slots[index].generation);
            if !self.slots[index].alive {
                continue;
            }
            if self.transforms2d[index].is_none() {
                continue;
            }
            if self.parents[index].is_some() {
                continue;
            }
            roots.push(entity);
        }
        roots
    }

    fn compute_refresh_mask_3d(&self) -> Vec<bool> {
        let mut mask = vec![false; self.slots.len()];
        for (index, slot) in self.slots.iter().enumerate() {
            if !slot.alive || self.transforms[index].is_none() {
                continue;
            }
            let entity = Entity::new(index as u32, slot.generation);
            mask[index] = self.transform_dirty[index]
                || self.has_dirty_ancestor(entity, &self.transform_dirty);
        }
        mask
    }

    fn compute_refresh_mask_2d(&self) -> Vec<bool> {
        let mut mask = vec![false; self.slots.len()];
        for (index, slot) in self.slots.iter().enumerate() {
            if !slot.alive || self.transforms2d[index].is_none() {
                continue;
            }
            let entity = Entity::new(index as u32, slot.generation);
            mask[index] = self.transform2d_dirty[index]
                || self.has_dirty_ancestor(entity, &self.transform2d_dirty);
        }
        mask
    }

    fn has_dirty_ancestor(&self, entity: Entity, dirty_table: &[bool]) -> bool {
        let mut cursor = self.parent(entity);
        while let Some(parent) = cursor {
            if dirty_table[parent.entity.index() as usize] {
                return true;
            }
            cursor = self.parent(parent.entity);
        }
        false
    }

    fn propagate_subtree_3d(&mut self, root: Entity, refresh: &[bool]) {
        let mut stack = vec![root];
        while let Some(entity) = stack.pop() {
            let idx = entity.index() as usize;
            if !refresh[idx] {
                if let Some(children) = self.children(entity).cloned() {
                    for child in children.iter().rev() {
                        stack.push(*child);
                    }
                }
                continue;
            }
            let parent_global = if let Some(parent) = self.parents[idx] {
                self.globals[parent.entity.index() as usize]
                    .expect("parent global transform missing")
                    .matrix
            } else {
                Mat4::IDENTITY
            };
            let local = self.transforms[idx]
                .expect("local transform missing")
                .local_matrix();
            let prev = self.globals[idx].expect("global transform missing").matrix;
            if let Some(previous) = self.prevs[idx].as_mut() {
                previous.matrix = prev;
            }
            let new_global = parent_global * local;
            let epoch = self.propagation_epoch;
            if let Some(global) = self.globals[idx].as_mut() {
                global.matrix = new_global;
                global.last_write_epoch = epoch;
            }
            if let Some(children) = self.children(entity).cloned() {
                for child in children.iter().rev() {
                    stack.push(*child);
                }
            }
        }
    }

    fn propagate_subtree_2d(&mut self, root: Entity, refresh: &[bool]) {
        let mut stack = vec![root];
        while let Some(entity) = stack.pop() {
            let idx = entity.index() as usize;
            if !refresh[idx] {
                if let Some(children) = self.children(entity).cloned() {
                    for child in children.iter().rev() {
                        stack.push(*child);
                    }
                }
                continue;
            }
            let parent_global = if let Some(parent) = self.parents[idx] {
                self.globals2d[parent.entity.index() as usize]
                    .expect("parent global transform2d missing")
                    .matrix
            } else {
                Mat3::IDENTITY
            };
            let local = self.transforms2d[idx]
                .expect("local transform2d missing")
                .local_matrix();
            let prev = self.globals2d[idx]
                .expect("global transform2d missing")
                .matrix;
            if let Some(previous) = self.prevs2d[idx].as_mut() {
                previous.matrix = prev;
            }
            let new_global = parent_global * local;
            let epoch = self.propagation_epoch;
            if let Some(global) = self.globals2d[idx].as_mut() {
                global.matrix = new_global;
                global.last_write_epoch = epoch;
            }
            if let Some(children) = self.children(entity).cloned() {
                for child in children.iter().rev() {
                    stack.push(*child);
                }
            }
        }
    }

    fn alloc_slot(&mut self) -> usize {
        if let Some(index) = self.free.pop() {
            let idx = index as usize;
            self.slots[idx].alive = true;
            self.ensure_component_capacity(idx);
            idx
        } else {
            let idx = self.slots.len();
            self.slots.push(Slot {
                generation: 0,
                alive: true,
            });
            self.push_empty_components();
            idx
        }
    }

    fn ensure_component_capacity(&mut self, idx: usize) {
        while self.transforms.len() <= idx {
            self.push_empty_components();
        }
    }

    fn push_empty_components(&mut self) {
        self.transforms.push(None);
        self.globals.push(None);
        self.prevs.push(None);
        self.transforms2d.push(None);
        self.globals2d.push(None);
        self.prevs2d.push(None);
        self.parents.push(None);
        self.children.push(None);
        self.transform_dirty.push(false);
        self.transform2d_dirty.push(false);
    }

    fn apply_set_parent(&mut self, child: Entity, parent: Entity) -> Result<(), SceneError> {
        if !self.is_alive(child) || !self.is_alive(parent) {
            return Err(SceneError::StaleEntity);
        }
        if child == parent {
            return Err(SceneError::HierarchyCycle);
        }
        if self.would_create_cycle(child, parent) {
            return Err(SceneError::HierarchyCycle);
        }
        let child_idx = child.index() as usize;
        if let Some(old_parent) = self.parents[child_idx] {
            self.remove_child_link(old_parent.entity, child);
            self.events.push(HierarchyEvent::ChildMoved {
                child,
                old_parent: old_parent.entity,
                new_parent: parent,
            });
        } else {
            self.events
                .push(HierarchyEvent::ChildAdded { child, parent });
        }
        self.parents[child_idx] = Some(Parent { entity: parent });
        self.insert_child_link(parent, child);
        Ok(())
    }

    fn apply_remove_parent(&mut self, child: Entity) -> Result<(), SceneError> {
        if !self.is_alive(child) {
            return Err(SceneError::StaleEntity);
        }
        let child_idx = child.index() as usize;
        let Some(old_parent) = self.parents[child_idx] else {
            return Ok(());
        };
        self.remove_child_link(old_parent.entity, child);
        self.parents[child_idx] = None;
        self.events.push(HierarchyEvent::ChildRemoved {
            child,
            old_parent: old_parent.entity,
        });
        Ok(())
    }

    fn apply_despawn_recursive(&mut self, root: Entity) -> Result<(), SceneError> {
        if !self.is_alive(root) {
            return Err(SceneError::StaleEntity);
        }
        let order = self.collect_postorder(root);
        for entity in order {
            self.despawn_single(entity);
        }
        Ok(())
    }

    fn apply_despawn_orphaning(&mut self, root: Entity) -> Result<(), SceneError> {
        if !self.is_alive(root) {
            return Err(SceneError::StaleEntity);
        }
        let children = self.children(root).cloned().unwrap_or_default();
        for child in children.iter() {
            self.remove_child_link(root, *child);
            let child_idx = child.index() as usize;
            self.parents[child_idx] = None;
        }
        self.despawn_single(root);
        Ok(())
    }

    fn collect_postorder(&self, root: Entity) -> Vec<Entity> {
        let mut stack = vec![(root, false)];
        let mut out = Vec::new();
        while let Some((node, expanded)) = stack.pop() {
            if expanded {
                out.push(node);
                continue;
            }
            stack.push((node, true));
            if let Some(children) = self.children(node) {
                for child in children.iter().rev() {
                    stack.push((*child, false));
                }
            }
        }
        out
    }

    fn despawn_single(&mut self, entity: Entity) {
        let idx = entity.index() as usize;
        if !self.slots[idx].alive {
            return;
        }
        if let Some(parent) = self.parents[idx] {
            self.remove_child_link(parent.entity, entity);
        }
        self.parents[idx] = None;
        self.children[idx] = None;
        self.transforms[idx] = None;
        self.globals[idx] = None;
        self.prevs[idx] = None;
        self.transforms2d[idx] = None;
        self.globals2d[idx] = None;
        self.prevs2d[idx] = None;
        self.transform_dirty[idx] = false;
        self.transform2d_dirty[idx] = false;
        self.slots[idx].alive = false;
        self.slots[idx].generation = self.slots[idx].generation.wrapping_add(1);
        self.free.push(entity.index());
    }

    fn insert_child_link(&mut self, parent: Entity, child: Entity) {
        let parent_idx = parent.index() as usize;
        if self.children[parent_idx].is_none() {
            self.children[parent_idx] = Some(Children::new());
        }
        if let Some(children) = self.children[parent_idx].as_mut() {
            if !children.contains(child) {
                children.entities.push(child);
            }
        }
    }

    fn remove_child_link(&mut self, parent: Entity, child: Entity) {
        let parent_idx = parent.index() as usize;
        if let Some(children) = self.children[parent_idx].as_mut() {
            children.entities.retain(|entity| *entity != child);
            if children.is_empty() {
                self.children[parent_idx] = None;
            }
        }
    }

    fn would_create_cycle(&self, child: Entity, parent: Entity) -> bool {
        let mut cursor = Some(parent);
        let mut guard = 0usize;
        while let Some(entity) = cursor {
            if entity == child {
                return true;
            }
            cursor = self
                .parents
                .get(entity.index() as usize)
                .and_then(|p| p.map(|parent| parent.entity));
            guard = guard.saturating_add(1);
            if guard > self.slots.len() {
                break;
            }
        }
        false
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use glam::{Vec2, Vec3};

    use super::super::propagation::propagate_transforms;
    use super::super::resources::BvhResource;
    use super::super::scene_blob::{Scene, SceneSpawner};
    use super::super::schedule::{Schedule, SystemId};
    use super::super::traversal::{
        depth_warning_count, heap_spill_count, reset_depth_warning_count, reset_heap_spill_count,
        BreadthFirstIterator, DepthFirstIterator,
    };
    use super::*;

    /// Serializes tests that read/write global traversal diagnostics (`DEPTH_WARNINGS`,
    /// `HEAP_SPILLS`) so parallel `cargo test` runs stay deterministic.
    static TRAVERSAL_DIAG_TESTS_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn tc_1_2_4_4_propagation_root_only() {
        let mut world = World::new();
        let root = world.spawn_transform(Transform::from_translation(Vec3::X));
        propagate_transforms(&mut world);
        let global = world.global_transform(root).expect("global transform");
        assert_eq!(
            global.matrix,
            Transform::from_translation(Vec3::X).local_matrix()
        );
    }

    #[test]
    fn tc_1_2_4_5_propagation_two_levels() {
        let mut world = World::new();
        let parent = world.spawn_transform(Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)));
        let child = world.spawn_transform(Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)));
        world.commands_mut().set_parent(child, parent);
        world.flush_hierarchy_commands().unwrap();
        propagate_transforms(&mut world);
        let child_global = world.global_transform(child).expect("child global");
        let expected = world
            .global_transform(parent)
            .expect("parent global")
            .matrix
            * Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)).local_matrix();
        let diff = child_global.matrix - expected;
        assert!(diff.to_cols_array().iter().all(|value| value.abs() < 1e-4));
        assert!((child_global.translation() - Vec3::new(10.0, 5.0, 0.0)).length() < 1e-3);
    }

    #[test]
    fn tc_1_2_4_6_propagation_deep_chain() {
        let mut world = World::new();
        let mut last = world.spawn_transform(Transform::IDENTITY);
        for _ in 0..50 {
            let next = world.spawn_transform(Transform::from_translation(Vec3::X));
            world.commands_mut().set_parent(next, last);
            last = next;
        }
        world.flush_hierarchy_commands().unwrap();
        propagate_transforms(&mut world);
        let leaf_global = world.global_transform(last).expect("leaf global");
        assert!((leaf_global.translation() - Vec3::new(50.0, 0.0, 0.0)).length() < 1e-3);
    }

    #[test]
    fn tc_1_2_4_7_propagation_no_stack_overflow() {
        let mut world = World::new();
        let mut last = world.spawn_transform(Transform::IDENTITY);
        for _ in 0..999 {
            let next = world.spawn_transform(Transform::from_translation(Vec3::X));
            world.commands_mut().set_parent(next, last);
            last = next;
        }
        world.flush_hierarchy_commands().unwrap();
        propagate_transforms(&mut world);
        assert!(world.global_transform(last).is_some());
    }

    #[test]
    fn tc_1_2_5_1_dirty_tracking_unchanged() {
        let mut world = World::new();
        let entity = world.spawn_transform(Transform::IDENTITY);
        propagate_transforms(&mut world);
        let epoch_after_first = world
            .global_transform(entity)
            .expect("global transform")
            .last_write_epoch();
        propagate_transforms(&mut world);
        let epoch_after_second = world
            .global_transform(entity)
            .expect("global transform")
            .last_write_epoch();
        assert_eq!(epoch_after_second, epoch_after_first);
    }

    #[test]
    fn tc_1_2_5_4_dirty_no_false_marks() {
        let mut world = World::new();
        let entity = world.spawn_transform(Transform::IDENTITY);
        let _ = world.transform(entity);
        assert!(!world.transform_dirty[entity.index() as usize]);
    }

    #[test]
    fn tc_1_2_5_2_dirty_leaf_only() {
        let mut world = World::new();
        let root = world.spawn_transform(Transform::IDENTITY);
        let mut parent = root;
        for _ in 0..4 {
            let child = world.spawn_transform(Transform::IDENTITY);
            world.commands_mut().set_parent(child, parent);
            parent = child;
        }
        world.flush_hierarchy_commands().unwrap();
        propagate_transforms(&mut world);
        let leaf = parent;
        let root_epoch_before = world.global_transform(root).unwrap().last_write_epoch();
        let leaf_epoch_before = world.global_transform(leaf).unwrap().last_write_epoch();
        if let Some(transform) = world.transform_mut(leaf) {
            transform.translation = Vec3::new(0.0, 1.0, 0.0);
        }
        propagate_transforms(&mut world);
        let root_epoch_after = world.global_transform(root).unwrap().last_write_epoch();
        let leaf_epoch_after = world.global_transform(leaf).unwrap().last_write_epoch();
        assert!(leaf_epoch_after > leaf_epoch_before);
        assert_eq!(root_epoch_after, root_epoch_before);
    }

    #[test]
    fn tc_1_2_5_3_dirty_root_propagates() {
        let mut world = World::new();
        let root = world.spawn_transform(Transform::IDENTITY);
        let mut parent = root;
        for _ in 0..4 {
            let child = world.spawn_transform(Transform::IDENTITY);
            world.commands_mut().set_parent(child, parent);
            parent = child;
        }
        world.flush_hierarchy_commands().unwrap();
        propagate_transforms(&mut world);
        let epoch_before = world.global_transform(parent).unwrap().last_write_epoch();
        if let Some(transform) = world.transform_mut(root) {
            transform.translation = Vec3::new(2.0, 0.0, 0.0);
        }
        propagate_transforms(&mut world);
        let epoch_after = world.global_transform(parent).unwrap().last_write_epoch();
        assert!(epoch_after > epoch_before);
    }

    #[test]
    fn tc_1_2_1_1_hierarchy_single_parent() {
        let mut world = World::new();
        let a = world.spawn_transform(Transform::IDENTITY);
        let b = world.spawn_transform(Transform::IDENTITY);
        let child = world.spawn_transform(Transform::IDENTITY);
        world.commands_mut().set_parent(child, a);
        world.flush_hierarchy_commands().unwrap();
        assert_eq!(world.parent(child), Some(Parent { entity: a }));
        world.commands_mut().set_parent(child, b);
        world.flush_hierarchy_commands().unwrap();
        assert_eq!(world.parent(child), Some(Parent { entity: b }));
        let a_children = world.children(a).cloned().unwrap_or_default();
        assert!(!a_children.contains(child));
    }

    #[test]
    fn tc_1_2_1_2_children_ordering() {
        let mut world = World::new();
        let parent = world.spawn_transform(Transform::IDENTITY);
        let c1 = world.spawn_transform(Transform::IDENTITY);
        let c2 = world.spawn_transform(Transform::IDENTITY);
        let c3 = world.spawn_transform(Transform::IDENTITY);
        world.commands_mut().set_parent(c1, parent);
        world.commands_mut().set_parent(c2, parent);
        world.commands_mut().set_parent(c3, parent);
        world.flush_hierarchy_commands().unwrap();
        let order: Vec<Entity> = world
            .children(parent)
            .expect("children")
            .iter()
            .copied()
            .collect();
        assert_eq!(order, vec![c1, c2, c3]);
        world.commands_mut().remove_parent(c2);
        world.flush_hierarchy_commands().unwrap();
        let order: Vec<Entity> = world
            .children(parent)
            .expect("children")
            .iter()
            .copied()
            .collect();
        assert_eq!(order, vec![c1, c3]);
        let c4 = world.spawn_transform(Transform::IDENTITY);
        world.commands_mut().set_parent(c4, parent);
        world.flush_hierarchy_commands().unwrap();
        let order: Vec<Entity> = world
            .children(parent)
            .expect("children")
            .iter()
            .copied()
            .collect();
        assert_eq!(order, vec![c1, c3, c4]);
    }

    #[test]
    fn tc_1_2_1_3_set_parent_command() {
        let mut world = World::new();
        let parent = world.spawn_transform(Transform::IDENTITY);
        let child = world.spawn_transform(Transform::IDENTITY);
        world.commands_mut().set_parent(child, parent);
        world.flush_hierarchy_commands().unwrap();
        assert_eq!(world.parent(child), Some(Parent { entity: parent }));
        assert!(world.children(parent).expect("children").contains(child));
        let events = world.take_hierarchy_events();
        assert!(events
            .iter()
            .any(|event| matches!(event, HierarchyEvent::ChildAdded { .. })));
    }

    #[test]
    fn tc_1_2_1_4_remove_parent_command() {
        let mut world = World::new();
        let parent = world.spawn_transform(Transform::IDENTITY);
        let child = world.spawn_transform(Transform::IDENTITY);
        world.commands_mut().set_parent(child, parent);
        world.flush_hierarchy_commands().unwrap();
        world.commands_mut().remove_parent(child);
        world.flush_hierarchy_commands().unwrap();
        assert_eq!(world.parent(child), None);
        assert!(world
            .children(parent)
            .map(|children| !children.contains(child))
            .unwrap_or(true));
        let events = world.take_hierarchy_events();
        assert!(events
            .iter()
            .any(|event| matches!(event, HierarchyEvent::ChildRemoved { .. })));
    }

    #[test]
    fn tc_1_2_1_5_reparent_child() {
        let mut world = World::new();
        let a = world.spawn_transform(Transform::IDENTITY);
        let b = world.spawn_transform(Transform::IDENTITY);
        let child = world.spawn_transform(Transform::IDENTITY);
        world.commands_mut().set_parent(child, a);
        world.flush_hierarchy_commands().unwrap();
        world.commands_mut().set_parent(child, b);
        world.flush_hierarchy_commands().unwrap();
        assert!(world
            .children(a)
            .map(|children| !children.contains(child))
            .unwrap_or(true));
        assert!(world
            .children(b)
            .is_some_and(|children| children.contains(child)));
    }

    #[test]
    fn tc_1_2_3_1_cascade_despawn() {
        let mut world = World::new();
        let root = world.spawn_transform(Transform::IDENTITY);
        let mid = world.spawn_transform(Transform::IDENTITY);
        let leaf = world.spawn_transform(Transform::IDENTITY);
        world.commands_mut().set_parent(mid, root);
        world.commands_mut().set_parent(leaf, mid);
        world.flush_hierarchy_commands().unwrap();
        world.commands_mut().despawn_recursive(root);
        world.flush_hierarchy_commands().unwrap();
        assert!(!world.is_alive(root));
        assert!(!world.is_alive(mid));
        assert!(!world.is_alive(leaf));
    }

    #[test]
    fn tc_1_2_3_2_orphan_on_delete() {
        let mut world = World::new();
        let parent = world.spawn_transform(Transform::IDENTITY);
        let c1 = world.spawn_transform(Transform::IDENTITY);
        let c2 = world.spawn_transform(Transform::IDENTITY);
        world.commands_mut().set_parent(c1, parent);
        world.commands_mut().set_parent(c2, parent);
        world.flush_hierarchy_commands().unwrap();
        world.commands_mut().despawn_orphaning(parent);
        world.flush_hierarchy_commands().unwrap();
        assert!(!world.is_alive(parent));
        assert!(world.is_alive(c1));
        assert!(world.is_alive(c2));
        assert_eq!(world.parent(c1), None);
        assert_eq!(world.parent(c2), None);
    }

    #[test]
    fn tc_1_2_3_3_no_orphaned_entities() {
        let mut world = World::new();
        let mut parent = world.spawn_transform(Transform::IDENTITY);
        for _ in 0..4 {
            let child = world.spawn_transform(Transform::IDENTITY);
            world.commands_mut().set_parent(child, parent);
            parent = child;
        }
        world.flush_hierarchy_commands().unwrap();
        let root = {
            let mut cursor = parent;
            while let Some(p) = world.parent(cursor) {
                cursor = p.entity;
            }
            cursor
        };
        world.commands_mut().despawn_recursive(root);
        world.flush_hierarchy_commands().unwrap();
        assert!(!world.is_alive(root));
        assert!(!world.is_alive(parent));
    }

    #[test]
    fn tc_1_2_2_1_dfs_order() {
        let mut world = World::new();
        let r = world.spawn_transform(Transform::IDENTITY);
        let a = world.spawn_transform(Transform::IDENTITY);
        let b = world.spawn_transform(Transform::IDENTITY);
        let c = world.spawn_transform(Transform::IDENTITY);
        let d = world.spawn_transform(Transform::IDENTITY);
        let e = world.spawn_transform(Transform::IDENTITY);
        let f = world.spawn_transform(Transform::IDENTITY);
        for (child, parent) in [(a, r), (b, r), (c, r), (d, a), (e, a), (f, b)] {
            world.commands_mut().set_parent(child, parent);
        }
        world.flush_hierarchy_commands().unwrap();
        let order: Vec<Entity> = DepthFirstIterator::new(&world, r, true)
            .map(|node| node.entity)
            .collect();
        assert_eq!(order, vec![r, a, d, e, b, f, c]);
    }

    #[test]
    fn tc_1_2_2_2_bfs_order() {
        let mut world = World::new();
        let r = world.spawn_transform(Transform::IDENTITY);
        let a = world.spawn_transform(Transform::IDENTITY);
        let b = world.spawn_transform(Transform::IDENTITY);
        let c = world.spawn_transform(Transform::IDENTITY);
        let d = world.spawn_transform(Transform::IDENTITY);
        let e = world.spawn_transform(Transform::IDENTITY);
        let f = world.spawn_transform(Transform::IDENTITY);
        for (child, parent) in [(a, r), (b, r), (c, r), (d, a), (e, a), (f, b)] {
            world.commands_mut().set_parent(child, parent);
        }
        world.flush_hierarchy_commands().unwrap();
        let order: Vec<Entity> = BreadthFirstIterator::new(&world, r, true)
            .map(|node| node.entity)
            .collect();
        assert_eq!(order, vec![r, a, b, c, d, e, f]);
    }

    #[test]
    fn tc_1_2_2_3_skip_subtree() {
        let mut world = World::new();
        let r = world.spawn_transform(Transform::IDENTITY);
        let a = world.spawn_transform(Transform::IDENTITY);
        let b = world.spawn_transform(Transform::IDENTITY);
        let d = world.spawn_transform(Transform::IDENTITY);
        let e = world.spawn_transform(Transform::IDENTITY);
        for (child, parent) in [(a, r), (b, r), (d, a), (e, a)] {
            world.commands_mut().set_parent(child, parent);
        }
        world.flush_hierarchy_commands().unwrap();
        let mut iter = DepthFirstIterator::new(&world, r, true);
        let mut visited = Vec::new();
        while let Some(node) = iter.next() {
            visited.push(node.entity);
            if node.entity == a {
                iter.skip_subtree();
            }
        }
        assert_eq!(visited, vec![r, a, b]);
    }

    #[test]
    fn tc_1_2_2_4_early_termination() {
        let mut world = World::new();
        let root = world.spawn_transform(Transform::IDENTITY);
        let mut parent = root;
        for _ in 0..9 {
            let child = world.spawn_transform(Transform::IDENTITY);
            world.commands_mut().set_parent(child, parent);
            parent = child;
        }
        world.flush_hierarchy_commands().unwrap();
        let mut iter = DepthFirstIterator::new(&world, root, true);
        let mut count = 0usize;
        while iter.next().is_some() {
            count += 1;
            if count == 3 {
                break;
            }
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn tc_1_2_2_7_depth_warning() {
        let _guard = TRAVERSAL_DIAG_TESTS_MUTEX
            .lock()
            .expect("traversal diagnostic test mutex poisoned");
        reset_depth_warning_count();
        let mut world = World::new();
        let mut parent = world.spawn_transform(Transform::IDENTITY);
        for _ in 0..129 {
            let child = world.spawn_transform(Transform::IDENTITY);
            world.commands_mut().set_parent(child, parent);
            parent = child;
        }
        world.flush_hierarchy_commands().unwrap();
        let root = {
            let mut cursor = parent;
            while let Some(p) = world.parent(cursor) {
                cursor = p.entity;
            }
            cursor
        };
        for _ in DepthFirstIterator::new(&world, root, true) {}
        assert!(depth_warning_count() > 0);
    }

    #[test]
    fn tc_1_2_2_5_traversal_256_wide_no_heap_spill() {
        let _guard = TRAVERSAL_DIAG_TESTS_MUTEX
            .lock()
            .expect("traversal diagnostic test mutex poisoned");
        reset_heap_spill_count();
        let mut world = World::new();
        let root = world.spawn_transform(Transform::IDENTITY);
        // Stay just under the inline stack capacity to avoid the manual spill path.
        for _ in 0..255 {
            let child = world.spawn_transform(Transform::IDENTITY);
            world.commands_mut().set_parent(child, root);
        }
        world.flush_hierarchy_commands().unwrap();
        for _ in DepthFirstIterator::new(&world, root, true) {}
        assert_eq!(heap_spill_count(), 0);
    }

    #[test]
    fn tc_1_2_2_6_traversal_257_wide_heap_fallback() {
        let _guard = TRAVERSAL_DIAG_TESTS_MUTEX
            .lock()
            .expect("traversal diagnostic test mutex poisoned");
        reset_heap_spill_count();
        let mut world = World::new();
        let root = world.spawn_transform(Transform::IDENTITY);
        for _ in 0..257 {
            let child = world.spawn_transform(Transform::IDENTITY);
            world.commands_mut().set_parent(child, root);
        }
        world.flush_hierarchy_commands().unwrap();
        for _ in DepthFirstIterator::new(&world, root, true) {}
        assert!(heap_spill_count() > 0);
    }

    #[test]
    fn tc_1_2_1_6_scene_roundtrip() {
        let scene = Scene::from_records(vec![
            (Transform::IDENTITY, None),
            (Transform::from_translation(Vec3::X), Some(0)),
        ]);
        let bytes = scene.to_json_bytes().unwrap();
        let roundtrip = Scene::from_json_bytes(&bytes).unwrap();
        assert_eq!(scene, roundtrip);
    }

    #[test]
    fn tc_1_2_1_7_scene_entity_remap() {
        let mut world = World::new();
        world.spawn_transform(Transform::IDENTITY); // occupy index 0
        let scene = Scene::from_records(vec![(Transform::from_translation(Vec3::Y), None)]);
        let mut spawner = SceneSpawner::default();
        let map = spawner.spawn(&scene, &mut world).unwrap();
        let spawned = map.get(0).expect("spawned entity");
        assert!(world.is_alive(spawned));
    }

    #[test]
    fn tc_1_2_1_8_scene_spawn_as_child() {
        let mut world = World::new();
        let parent = world.spawn_transform(Transform::IDENTITY);
        let scene = Scene::from_records(vec![(Transform::from_translation(Vec3::X), None)]);
        let mut spawner = SceneSpawner::default();
        let map = spawner.spawn_as_child(&scene, &mut world, parent).unwrap();
        let child = map.get(0).expect("spawned child");
        assert_eq!(world.parent(child), Some(Parent { entity: parent }));
    }

    #[test]
    fn tc_1_2_1_9_scene_cyclic_detection() {
        let scene = Scene::from_records(vec![
            (Transform::IDENTITY, Some(1)),
            (Transform::IDENTITY, Some(0)),
        ]);
        assert_eq!(scene.validate(), Err(SceneError::CyclicHierarchy));
    }

    #[test]
    fn tc_1_1_28_1_schedule_ordering() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        for _ in 0..100 {
            schedule
                .run_frame(
                    &mut world,
                    &[SystemId::HierarchyApply, SystemId::PropagateTransforms],
                )
                .unwrap();
            assert_eq!(
                schedule.last_run_order,
                vec![SystemId::HierarchyApply, SystemId::PropagateTransforms]
            );
        }
    }

    #[test]
    fn tc_1_9_1_1_single_bvh_resource() {
        let mut world = World::new();
        world.register_bvh(BvhResource::default()).unwrap();
        assert!(world.resources.bvh.is_some());
        assert_eq!(
            world.register_bvh(BvhResource::default()),
            Err(SceneError::DuplicateBvh)
        );
    }

    struct FakeSpatial {
        calls: usize,
    }

    impl super::super::resources::SpatialQueryBackend for FakeSpatial {
        fn query_aabb(&mut self, _min: Vec3, _max: Vec3) -> &[Entity] {
            self.calls += 1;
            &[]
        }
    }

    #[test]
    fn tc_1_2_7_1_spatial_query_backend_swap() {
        let world = World::new();
        let mut fake = FakeSpatial { calls: 0 };
        let _hits = world.spatial_query_aabb(&mut fake, Vec3::ZERO, Vec3::ONE);
        assert_eq!(fake.calls, 1);
    }

    #[test]
    fn tc_2d_propagation_two_levels() {
        let mut world = World::new();
        let parent = world.spawn_transform2d(Transform2D::from_position(Vec2::new(4.0, 0.0)));
        let child = world.spawn_transform2d(Transform2D::from_position(Vec2::new(0.0, 3.0)));
        world.commands_mut().set_parent(child, parent);
        world.flush_hierarchy_commands().unwrap();
        propagate_transforms(&mut world);
        let child_global = world.global_transform2d(child).expect("child global");
        let expected = world
            .global_transform2d(parent)
            .expect("parent global")
            .matrix
            * Transform2D::from_position(Vec2::new(0.0, 3.0)).local_matrix();
        let diff = child_global.matrix - expected;
        assert!(diff.to_cols_array().iter().all(|value| value.abs() < 1e-4));
    }

    #[test]
    fn tc_2d_3d_coexistence() {
        let mut world = World::new();
        let e3 = world.spawn_transform(Transform::from_translation(Vec3::Z));
        let e2 = world.spawn_transform2d(Transform2D::from_position(Vec2::ONE));
        propagate_transforms(&mut world);
        assert!(world.global_transform(e3).is_some());
        assert!(world.global_transform2d(e2).is_some());
    }
}
