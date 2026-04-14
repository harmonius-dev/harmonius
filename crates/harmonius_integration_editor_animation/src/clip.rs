//! Animation clip handles and slot validation.

use crate::handle::Handle;

/// Marker type for [`Handle`] referring to animation clip data.
#[derive(Debug, Eq, PartialEq)]
pub enum AnimationClip {}

/// Tracks generations for clip slots (in-memory only; not serialized).
#[derive(Debug, Default)]
pub struct AnimationClipTable {
    generations: Vec<u32>,
}

impl AnimationClipTable {
    /// Allocates a new clip slot and returns its handle.
    pub fn allocate(&mut self) -> Handle<AnimationClip> {
        let index = u32::try_from(self.generations.len()).expect("clip table size fits u32");
        self.generations.push(1);
        Handle::from_raw_parts(index, 1)
    }

    /// Frees a slot, bumping generation so old handles become stale.
    pub fn free(&mut self, handle: Handle<AnimationClip>) {
        let i = handle.index() as usize;
        if let Some(generation) = self.generations.get_mut(i) {
            *generation = generation.saturating_add(1);
        }
    }

    /// Returns `true` when the handle matches a live slot.
    #[must_use]
    pub fn is_valid(&self, handle: Handle<AnimationClip>) -> bool {
        let index = handle.index() as usize;
        let generation = handle.generation();
        self
            .generations
            .get(index)
            .is_some_and(|g| *g == generation && *g > 0)
    }
}
