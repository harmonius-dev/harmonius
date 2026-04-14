//! Generational [`PipelineStateHandle`] arena owned by the render thread (IR-3.5.4).

/// Lightweight payload carried with a published pipeline state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PipelineStateDesc {
    /// Stable debug label (hash or small id).
    pub label: u32,
}

/// Generational handle into [`PipelineStateTable`].
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct PipelineStateHandle {
    /// Slot index in the arena.
    pub index: u32,
    /// Generation bumped on free to detect stale handles.
    pub generation: u32,
}

#[derive(Debug)]
struct Slot {
    generation: u32,
    alive: bool,
    desc: Option<PipelineStateDesc>,
}

/// Render-thread arena mapping [`PipelineStateHandle`] → [`PipelineStateDesc`].
#[derive(Debug, Default)]
pub struct PipelineStateTable {
    slots: Vec<Slot>,
    free: Vec<u32>,
}

impl PipelineStateTable {
    /// Creates an empty table.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Publishes `desc` and returns a fresh generational handle.
    pub fn publish(&mut self, desc: PipelineStateDesc) -> PipelineStateHandle {
        if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index as usize];
            slot.alive = true;
            slot.desc = Some(desc);
            return PipelineStateHandle {
                index,
                generation: slot.generation,
            };
        }
        let index = u32::try_from(self.slots.len()).expect("slot count fits u32");
        self.slots.push(Slot {
            generation: 0,
            alive: true,
            desc: Some(desc),
        });
        PipelineStateHandle {
            index,
            generation: 0,
        }
    }

    /// Resolves a handle when generation matches and the slot is alive.
    #[must_use]
    pub fn resolve(&self, handle: PipelineStateHandle) -> Option<&PipelineStateDesc> {
        let slot = self.slots.get(handle.index as usize)?;
        if !slot.alive || slot.generation != handle.generation {
            return None;
        }
        slot.desc.as_ref()
    }

    /// Frees a slot and invalidates prior handles sharing the old generation.
    pub fn free(&mut self, handle: PipelineStateHandle) {
        let Some(slot) = self.slots.get_mut(handle.index as usize) else {
            return;
        };
        if slot.generation != handle.generation || !slot.alive {
            return;
        }
        slot.alive = false;
        slot.desc = None;
        slot.generation = slot.generation.wrapping_add(1);
        self.free.push(handle.index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_5_4_u1_pipeline_handle_invalidates_after_free() {
        let mut t = PipelineStateTable::new();
        let h0 = t.publish(PipelineStateDesc { label: 1 });
        assert_eq!(t.resolve(h0), Some(&PipelineStateDesc { label: 1 }));
        t.free(h0);
        assert!(t.resolve(h0).is_none());
        let h1 = t.publish(PipelineStateDesc { label: 2 });
        assert_eq!(h1.index, h0.index);
        assert_ne!(h1.generation, h0.generation);
        assert!(t.resolve(h0).is_none());
        assert_eq!(t.resolve(h1), Some(&PipelineStateDesc { label: 2 }));
    }
}
