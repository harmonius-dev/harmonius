//! Frame and scoped bump allocators.

use super::tag::AllocationTag;
use super::tracker::MemoryTracker;
use std::alloc::Layout;

/// Configuration for [`FrameArena`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArenaConfig {
    /// Initial capacity in bytes.
    pub initial_capacity: usize,
    /// Maximum capacity after growth.
    pub max_capacity: usize,
    /// Default tag for allocations from this arena.
    pub tag: AllocationTag,
}

/// Bump allocator errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArenaError {
    /// Allocation does not fit in remaining capacity (and cannot grow further).
    OutOfMemory {
        /// Requested size in bytes.
        requested: usize,
        /// Bytes remaining before growth or cap.
        remaining: usize,
    },
}

/// Per-frame bump allocator backed by a growable byte buffer.
pub struct FrameArena {
    storage: Vec<u8>,
    watermark: usize,
    max_capacity: usize,
    tag: AllocationTag,
    tracker: Option<&'static MemoryTracker>,
    scope_marks: Vec<usize>,
}

impl FrameArena {
    /// Creates a new arena using `config`.
    #[must_use]
    pub fn new(config: ArenaConfig, tracker: Option<&'static MemoryTracker>) -> Self {
        let cap = config.initial_capacity.min(config.max_capacity);
        Self {
            storage: vec![0_u8; cap],
            watermark: 0,
            max_capacity: config.max_capacity,
            tag: config.tag,
            tracker,
            scope_marks: Vec::new(),
        }
    }

    /// Records the current watermark so a later [`FrameArena::pop_scope`] can restore it.
    pub fn push_scope(&mut self) {
        self.scope_marks.push(self.watermark);
    }

    /// Restores the watermark from the innermost [`FrameArena::push_scope`] mark.
    pub fn pop_scope(&mut self) {
        if let Some(w) = self.scope_marks.pop() {
            self.watermark = w;
        }
    }

    /// Bytes currently reserved.
    #[must_use]
    pub fn used(&self) -> usize {
        self.watermark
    }

    /// Bytes still available at the current capacity without growing.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.storage.len().saturating_sub(self.watermark)
    }

    /// Current backing capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.storage.len()
    }

    /// Resets the watermark to zero.
    pub fn reset(&mut self) {
        self.watermark = 0;
        self.scope_marks.clear();
    }

    /// Bump-allocates memory for `layout`.
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, ArenaError> {
        let align = layout.align().max(1);
        let size = layout.size();
        let start = align_up(self.watermark, align);
        let end = start.checked_add(size).ok_or(ArenaError::OutOfMemory {
            requested: size,
            remaining: self.remaining_at_offset(start),
        })?;
        if end > self.storage.len() {
            self.grow_to_fit(end)?;
        }
        let end = start.checked_add(size).expect("size");
        if end > self.storage.len() {
            return Err(ArenaError::OutOfMemory {
                requested: size,
                remaining: self.storage.len().saturating_sub(start),
            });
        }
        self.watermark = end;
        if let Some(t) = self.tracker {
            t.record_alloc(self.tag, size, "arena");
        }
        Ok(self.storage[start..end].as_mut_ptr())
    }

    /// Typed bump allocation.
    pub fn alloc_typed<T>(&mut self) -> Result<*mut T, ArenaError> {
        let layout = Layout::new::<T>();
        let p = self.alloc(layout)?;
        Ok(p.cast::<T>())
    }

    /// Opens a scoped sub-allocator that restores the watermark on drop.
    pub fn child_scope(&mut self, tag: AllocationTag) -> ScopedArena<'_> {
        self.scope_marks.push(self.watermark);
        let effective = self.tag.inherit(tag);
        ScopedArena {
            parent: self,
            tag: effective,
        }
    }

    fn remaining_at_offset(&self, start: usize) -> usize {
        self.storage.len().saturating_sub(start)
    }

    fn grow_to_fit(&mut self, need: usize) -> Result<(), ArenaError> {
        if need > self.max_capacity {
            return Err(ArenaError::OutOfMemory {
                requested: need.saturating_sub(self.storage.len()),
                remaining: self.max_capacity.saturating_sub(self.watermark),
            });
        }
        let mut new_cap = self.storage.len().max(1);
        while new_cap < need {
            new_cap = new_cap.checked_mul(2).ok_or(ArenaError::OutOfMemory {
                requested: need,
                remaining: self.max_capacity.saturating_sub(self.storage.len()),
            })?;
            if new_cap > self.max_capacity {
                new_cap = self.max_capacity;
                break;
            }
        }
        new_cap = new_cap.max(need).min(self.max_capacity);
        if new_cap <= self.storage.len() {
            return Ok(());
        }
        self.storage.resize(new_cap, 0_u8);
        Ok(())
    }
}

/// Temporary sub-arena sharing storage with a [`FrameArena`].
pub struct ScopedArena<'a> {
    parent: &'a mut FrameArena,
    tag: AllocationTag,
}

impl ScopedArena<'_> {
    /// Bytes reserved within this scope (since construction).
    #[must_use]
    pub fn used(&self) -> usize {
        let base = self.parent.scope_marks.last().copied().unwrap_or(0);
        self.parent.watermark.saturating_sub(base)
    }

    /// Remaining bytes in the parent after the current watermark.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.parent
            .storage
            .len()
            .saturating_sub(self.parent.watermark)
    }

    /// Bump-allocates memory for `layout` against the parent arena.
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, ArenaError> {
        let align = layout.align().max(1);
        let size = layout.size();
        let start = align_up(self.parent.watermark, align);
        let end = start.checked_add(size).ok_or(ArenaError::OutOfMemory {
            requested: size,
            remaining: self.parent.storage.len().saturating_sub(start),
        })?;
        if end > self.parent.storage.len() {
            self.parent.grow_to_fit(end)?;
        }
        if end > self.parent.storage.len() {
            return Err(ArenaError::OutOfMemory {
                requested: size,
                remaining: self.parent.storage.len().saturating_sub(start),
            });
        }
        self.parent.watermark = end;
        if let Some(t) = self.parent.tracker {
            t.record_alloc(self.tag, size, "scoped_arena");
        }
        Ok(self.parent.storage[start..end].as_mut_ptr())
    }

    /// Typed bump allocation within this scope.
    pub fn alloc_typed<T>(&mut self) -> Result<*mut T, ArenaError> {
        let layout = Layout::new::<T>();
        let p = self.alloc(layout)?;
        Ok(p.cast::<T>())
    }
}

impl Drop for ScopedArena<'_> {
    fn drop(&mut self) {
        if let Some(w) = self.parent.scope_marks.pop() {
            self.parent.watermark = w;
        }
    }
}

/// Owns one [`FrameArena`].
pub struct PerThreadArena {
    arenas: Vec<FrameArena>,
}

impl PerThreadArena {
    /// Creates `worker_count` arenas sharing `tracker`.
    #[must_use]
    pub fn new(
        worker_count: usize,
        config: ArenaConfig,
        tracker: Option<&'static MemoryTracker>,
    ) -> Self {
        let mut arenas = Vec::with_capacity(worker_count);
        for _ in 0..worker_count {
            arenas.push(FrameArena::new(config, tracker));
        }
        Self { arenas }
    }

    /// Borrows the arena for `worker_index`.
    #[must_use]
    pub fn get_mut(&mut self, worker_index: usize) -> Option<&mut FrameArena> {
        self.arenas.get_mut(worker_index)
    }

    /// Resets every arena watermark.
    pub fn reset_all(&mut self) {
        for a in &mut self.arenas {
            a.reset();
        }
    }
}

fn align_up(n: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two());
    (n + align - 1) & !(align - 1)
}
