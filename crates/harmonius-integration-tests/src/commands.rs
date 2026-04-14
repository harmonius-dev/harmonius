//! Deferred command segments and merge ordering.

use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicU64, Ordering};

use smallvec::SmallVec;

use crate::world::{Component, Entity, World};

static NEXT_ENTITY: AtomicU64 = AtomicU64::new(1);

fn alloc_entity() -> Entity {
    Entity(NEXT_ENTITY.fetch_add(1, Ordering::SeqCst))
}

/// One deferred operation in a `CommandSegment`.
#[derive(Debug)]
pub enum Command {
    /// Insert or replace a component value.
    Insert {
        /// Target entity.
        entity: Entity,
        /// Component type id.
        ty: TypeId,
        /// Boxed component value.
        value: Box<dyn Any + Send>,
    },
    /// Remove a component type from an entity.
    Remove {
        /// Target entity.
        entity: Entity,
        /// Component type id.
        ty: TypeId,
    },
    /// Despawn an entity and strip components.
    Despawn {
        /// Target entity.
        entity: Entity,
    },
    /// Spawn a new entity with optional components (applied at flush).
    SpawnBundle {
        /// Components to attach after entity id allocation.
        bundle: Vec<(TypeId, Box<dyn Any + Send>)>,
    },
}

/// Builder returned from `ExecutionContext::spawn`.
pub struct EntityCommands<'a> {
    segment: &'a mut CommandSegment,
    pending: Vec<(TypeId, Box<dyn Any + Send>)>,
}

impl EntityCommands<'_> {
    /// Attach a component to the spawned entity (deferred).
    pub fn insert<T: Component>(&mut self, value: T) -> &mut Self {
        self.pending.push((TypeId::of::<T>(), Box::new(value)));
        self
    }
}

impl Drop for EntityCommands<'_> {
    fn drop(&mut self) {
        if self.pending.is_empty() {
            return;
        }
        let bundle = std::mem::take(&mut self.pending);
        let _ = self.segment.push_op(Command::SpawnBundle { bundle });
    }
}

/// Per-thread deferred command buffer with inline capacity.
#[derive(Debug)]
pub struct CommandSegment {
    /// Inline ops (hot path).
    pub(crate) ops: SmallVec<[Command; 32]>,
    /// Max inline ops before spill list (test hook).
    max_inline: usize,
    /// Max ops total including spill list (test hook).
    max_total: usize,
    /// Spilled ops when inline capacity exceeded.
    pub(crate) spill: Vec<Command>,
    /// Monotonic op index for diagnostics.
    pub op_index: u64,
}

impl Default for CommandSegment {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandSegment {
    /// Default test limits (32 inline, 1024 total).
    pub fn new() -> Self {
        Self::with_limits(32, 1024)
    }

    /// Segment with explicit overflow limits for negative tests.
    pub fn with_limits(max_inline: usize, max_total: usize) -> Self {
        Self {
            max_inline,
            max_total,
            op_index: 0,
            ops: SmallVec::new(),
            spill: Vec::new(),
        }
    }

    fn total_len(&self) -> usize {
        self.ops.len() + self.spill.len()
    }

    /// Queue insert; returns `false` on overflow (no partial enqueue).
    pub fn insert<T: Component>(&mut self, entity: Entity, value: T) -> bool {
        self.push_op(Command::Insert {
            entity,
            ty: TypeId::of::<T>(),
            value: Box::new(value),
        })
    }

    /// Queue remove component.
    pub fn remove<T: Component>(&mut self, entity: Entity) -> bool {
        self.push_op(Command::Remove {
            entity,
            ty: TypeId::of::<T>(),
        })
    }

    /// Queue despawn.
    pub fn despawn(&mut self, entity: Entity) -> bool {
        self.push_op(Command::Despawn { entity })
    }

    /// Begin spawn command.
    pub fn spawn(&mut self) -> EntityCommands<'_> {
        EntityCommands {
            segment: self,
            pending: Vec::new(),
        }
    }

    pub(crate) fn push_op(&mut self, op: Command) -> bool {
        if self.total_len() >= self.max_total {
            return false;
        }
        if self.ops.len() < self.max_inline {
            self.ops.push(op);
        } else {
            self.spill.push(op);
        }
        self.op_index = self.op_index.wrapping_add(1);
        true
    }

    /// Inline and total op limits (for parallel batch segments).
    pub fn limits(&self) -> (usize, usize) {
        (self.max_inline, self.max_total)
    }
}

/// Owning writer that fans out per-worker `CommandSegment` handles.
#[derive(Debug)]
pub struct ParallelCommandWriter {
    segments: Vec<CommandSegment>,
}

impl ParallelCommandWriter {
    /// Number of worker command segments.
    pub fn workers(&self) -> usize {
        self.segments.len()
    }

    /// Allocate `workers` empty segments.
    pub fn new(workers: usize) -> Self {
        Self::with_limits(workers, 32, 1024)
    }

    /// Custom limits per segment (integration tests).
    pub fn with_limits(workers: usize, inline: usize, total: usize) -> Self {
        let mut v = Vec::with_capacity(workers);
        for _ in 0..workers {
            v.push(CommandSegment::with_limits(inline, total));
        }
        Self { segments: v }
    }

    /// Borrow segment `idx` for mutation.
    pub fn writer(&mut self, idx: usize) -> &mut CommandSegment {
        &mut self.segments[idx]
    }

    /// Drain another segment into `slot` (used after parallel batch).
    pub fn absorb_worker_segment(&mut self, slot: usize, mut src: CommandSegment) {
        let dst = &mut self.segments[slot];
        for op in src.ops.drain(..) {
            let _ = dst.push_op(op);
        }
        for op in src.spill.drain(..) {
            let _ = dst.push_op(op);
        }
    }

    /// Merge segments into `buffer` in ascending worker index order.
    pub fn merge_into(&mut self, buffer: &mut CommandBuffer) {
        buffer.ops.clear();
        for seg in &mut self.segments {
            for op in seg.ops.drain(..) {
                buffer.ops.push(op);
            }
            for op in seg.spill.drain(..) {
                buffer.ops.push(op);
            }
        }
    }
}

/// Merged command stream for one sync point.
#[derive(Debug, Default)]
pub struct CommandBuffer {
    pub(crate) ops: Vec<Command>,
}

impl CommandBuffer {
    /// Drain queued ops from `segment` into this buffer (preserves order).
    pub fn extend_from_segment(&mut self, segment: &mut CommandSegment) {
        for op in segment.ops.drain(..) {
            self.ops.push(op);
        }
        for op in segment.spill.drain(..) {
            self.ops.push(op);
        }
    }

    /// Apply all deferred ops to `world` in queue order.
    pub fn flush(&mut self, world: &mut World) {
        for op in self.ops.drain(..) {
            match op {
                Command::Insert { entity, ty, value } => {
                    let _ = world.insert_dyn(entity, ty, value);
                }
                Command::Remove { entity, ty } => {
                    world.remove_dyn(entity, ty);
                }
                Command::Despawn { entity } => {
                    world.despawn(entity);
                }
                Command::SpawnBundle { bundle } => {
                    let e = alloc_entity();
                    world.reserve_entity(e);
                    for (ty, val) in bundle {
                        let _ = world.insert_dyn(e, ty, val);
                    }
                }
            }
        }
    }
}
