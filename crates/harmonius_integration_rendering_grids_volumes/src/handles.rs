//! Generational GPU texture handles resolved against a render-thread arena.

/// Generational handle into the render thread texture arena (`Copy`, no `Arc`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GpuTextureHandle {
    /// Slot index inside [`DeviceTextureArena`].
    pub index: u32,
    /// Generation counter bumped on slot reuse.
    pub generation: u32,
}

#[derive(Debug)]
struct Slot {
    generation: u32,
    alive: bool,
}

/// Minimal stand-in for `Arena<DeviceTexture>` on the render thread.
#[derive(Debug, Default)]
pub struct DeviceTextureArena {
    slots: Vec<Slot>,
    free: Vec<u32>,
}

impl DeviceTextureArena {
    /// Creates an empty arena.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocates a slot, reusing freed indices when possible.
    pub fn alloc(&mut self) -> GpuTextureHandle {
        if let Some(idx) = self.free.pop() {
            let i = usize::try_from(idx).expect("arena index fits usize");
            let slot = &mut self.slots[i];
            slot.alive = true;
            return GpuTextureHandle {
                index: idx,
                generation: slot.generation,
            };
        }
        let idx = u32::try_from(self.slots.len()).expect("arena size fits u32");
        self.slots.push(Slot {
            generation: 0,
            alive: true,
        });
        GpuTextureHandle {
            index: idx,
            generation: 0,
        }
    }

    /// Frees a slot; wrong generation is a no-op.
    pub fn free(&mut self, handle: GpuTextureHandle) {
        let Ok(i) = usize::try_from(handle.index) else {
            return;
        };
        let Some(slot) = self.slots.get_mut(i) else {
            return;
        };
        if !slot.alive || slot.generation != handle.generation {
            return;
        }
        slot.alive = false;
        slot.generation = slot.generation.saturating_add(1);
        self.free.push(handle.index);
    }

    /// Returns `Some(())` when the handle matches a live slot.
    #[must_use]
    pub fn resolve(&self, handle: GpuTextureHandle) -> Option<()> {
        let i = usize::try_from(handle.index).ok()?;
        let slot = self.slots.get(i)?;
        (slot.alive && slot.generation == handle.generation).then_some(())
    }
}

#[cfg(test)]
mod tests {
    use super::DeviceTextureArena;

    /// TC-IR-3.3.X.2 — stale handle returns `None` after free + reuse.
    #[test]
    fn stale_handle_resolves_none() {
        let mut arena = DeviceTextureArena::new();
        let h0 = arena.alloc();
        assert!(arena.resolve(h0).is_some());
        arena.free(h0);
        let h1 = arena.alloc();
        assert_eq!(h1.index, h0.index);
        assert_ne!(h1.generation, h0.generation);
        assert!(arena.resolve(h0).is_none());
        assert!(arena.resolve(h1).is_some());
    }

    #[test]
    fn double_free_ignored() {
        let mut arena = DeviceTextureArena::new();
        let h = arena.alloc();
        arena.free(h);
        arena.free(h);
        let h2 = arena.alloc();
        assert_eq!(h2.index, h.index);
    }

    #[test]
    fn wrong_generation_free_is_no_op() {
        let mut arena = DeviceTextureArena::new();
        let mut h = arena.alloc();
        h.generation = h.generation.wrapping_sub(1);
        arena.free(h);
        let h2 = arena.alloc();
        assert!(arena.resolve(h2).is_some());
    }
}
