//! Depth- and breadth-first hierarchy iterators.

#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};

use smallvec::SmallVec;

use super::Entity;
use super::World;

/// Maximum inline stack depth before spilling to the heap (`R-1.2.2a`).
pub const MAX_STACK_DEPTH: usize = 256;

/// Depth at which diagnostics increment (`R-1.2.2a`).
pub const DEPTH_WARNING_THRESHOLD: u32 = 128;

static DEPTH_WARNINGS: AtomicU64 = AtomicU64::new(0);
static HEAP_SPILLS: AtomicU64 = AtomicU64::new(0);

/// Returns how many depth warnings were emitted (tests reset this).
#[must_use]
pub fn depth_warning_count() -> u64 {
    DEPTH_WARNINGS.load(Ordering::Relaxed)
}

/// Resets [`depth_warning_count`] to zero.
pub fn reset_depth_warning_count() {
    DEPTH_WARNINGS.store(0, Ordering::Relaxed);
}

/// Returns how many times iterators spilled stacks to the heap.
#[must_use]
pub fn heap_spill_count() -> u64 {
    HEAP_SPILLS.load(Ordering::Relaxed)
}

/// Resets [`heap_spill_count`] to zero.
pub fn reset_heap_spill_count() {
    HEAP_SPILLS.store(0, Ordering::Relaxed);
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
enum SmallOrHeap {
    Inline(SmallVec<[(Entity, u32); MAX_STACK_DEPTH]>),
    Heap(Vec<(Entity, u32)>),
}

impl SmallOrHeap {
    fn new() -> Self {
        Self::Inline(SmallVec::new())
    }

    fn push(&mut self, value: (Entity, u32)) {
        match self {
            Self::Inline(vec) => {
                if vec.len() == MAX_STACK_DEPTH {
                    let mut promoted: Vec<(Entity, u32)> = vec.iter().copied().collect();
                    promoted.push(value);
                    HEAP_SPILLS.fetch_add(1, Ordering::Relaxed);
                    *self = Self::Heap(promoted);
                } else {
                    vec.push(value);
                }
            }
            Self::Heap(vec) => vec.push(value),
        }
    }

    fn pop(&mut self) -> Option<(Entity, u32)> {
        match self {
            Self::Inline(vec) => vec.pop(),
            Self::Heap(vec) => vec.pop(),
        }
    }

    fn pop_front(&mut self) -> Option<(Entity, u32)> {
        match self {
            Self::Inline(vec) if vec.is_empty() => None,
            Self::Inline(vec) => Some(vec.remove(0)),
            Self::Heap(vec) if vec.is_empty() => None,
            Self::Heap(vec) => Some(vec.remove(0)),
        }
    }

    fn push_back(&mut self, value: (Entity, u32)) {
        self.push(value);
    }

    fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut((Entity, u32)) -> bool,
    {
        match self {
            Self::Inline(vec) => vec.retain(|entry| keep(*entry)),
            Self::Heap(vec) => vec.retain(|entry| keep(*entry)),
        }
    }
}

/// Node yielded by traversal iterators.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HierarchyNode {
    /// Visited entity.
    pub entity: Entity,
    /// Depth from the iterator root.
    pub depth: u32,
}

/// Depth-first iterator over hierarchy edges stored on [`World`].
pub struct DepthFirstIterator<'w> {
    stack: SmallOrHeap,
    world: &'w World,
    last_yielded: Option<Entity>,
}

impl<'w> DepthFirstIterator<'w> {
    /// Creates a new iterator starting at `root`.
    #[must_use]
    pub fn new(world: &'w World, root: Entity, include_root: bool) -> Self {
        let mut stack = SmallOrHeap::new();
        if include_root {
            stack.push((root, 0));
        } else if let Some(children) = world.children(root) {
            for child in children.iter().rev() {
                stack.push((*child, 1));
            }
        }
        Self {
            stack,
            world,
            last_yielded: None,
        }
    }

    /// Skips descending into the last yielded node's subtree.
    pub fn skip_subtree(&mut self) {
        if let Some(anchor) = self.last_yielded {
            self.stack
                .retain(|(entity, _)| !Self::entity_is_under(self.world, entity, anchor));
        }
    }

    fn entity_is_under(world: &'w World, entity: Entity, ancestor: Entity) -> bool {
        let mut cursor = world.parent(entity);
        while let Some(parent) = cursor {
            if parent.entity == ancestor {
                return true;
            }
            cursor = world.parent(parent.entity);
        }
        false
    }

    fn note_depth(depth: u32) {
        if depth > DEPTH_WARNING_THRESHOLD {
            DEPTH_WARNINGS.fetch_add(1, Ordering::Relaxed);
        }
    }
}

impl<'w> Iterator for DepthFirstIterator<'w> {
    type Item = HierarchyNode;

    fn next(&mut self) -> Option<Self::Item> {
        let (entity, depth) = self.stack.pop()?;
        Self::note_depth(depth);
        self.last_yielded = Some(entity);
        if let Some(children) = self.world.children(entity) {
            for child in children.iter().rev() {
                self.stack.push((*child, depth.saturating_add(1)));
            }
        }
        Some(HierarchyNode { entity, depth })
    }
}

/// Breadth-first iterator over hierarchy edges stored on [`World`].
pub struct BreadthFirstIterator<'w> {
    queue: SmallOrHeap,
    world: &'w World,
    last_yielded: Option<Entity>,
}

impl<'w> BreadthFirstIterator<'w> {
    /// Creates a new iterator starting at `root`.
    #[must_use]
    pub fn new(world: &'w World, root: Entity, include_root: bool) -> Self {
        let mut queue = SmallOrHeap::new();
        if include_root {
            queue.push_back((root, 0));
        } else if let Some(children) = world.children(root) {
            for child in children.iter() {
                queue.push_back((*child, 1));
            }
        }
        Self {
            queue,
            world,
            last_yielded: None,
        }
    }

    /// Skips descending into the last yielded node's subtree.
    pub fn skip_subtree(&mut self) {
        if let Some(anchor) = self.last_yielded {
            self.queue.retain(|(entity, _)| {
                !DepthFirstIterator::entity_is_under(self.world, entity, anchor)
            });
        }
    }

    fn note_depth(depth: u32) {
        if depth > DEPTH_WARNING_THRESHOLD {
            DEPTH_WARNINGS.fetch_add(1, Ordering::Relaxed);
        }
    }
}

impl<'w> Iterator for BreadthFirstIterator<'w> {
    type Item = HierarchyNode;

    fn next(&mut self) -> Option<Self::Item> {
        let (entity, depth) = self.queue.pop_front()?;
        Self::note_depth(depth);
        self.last_yielded = Some(entity);
        if let Some(children) = self.world.children(entity) {
            for child in children.iter() {
                self.queue.push_back((*child, depth.saturating_add(1)));
            }
        }
        Some(HierarchyNode { entity, depth })
    }
}
