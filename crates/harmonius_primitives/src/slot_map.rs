//! Dense storage with a sparse generation-aware index table.

use crate::handle::Handle;
use std::vec::Vec;

/// Densely packed storage keyed by [`Handle`], optimized for fast iteration over live values.
#[derive(Debug)]
pub struct SlotMap<T> {
    dense: Vec<T>,
    /// Maps sparse slot index → dense index (`u32::MAX` when free).
    sparse: Vec<u32>,
    /// Maps dense index → sparse slot index.
    reverse: Vec<u32>,
    free: Vec<u32>,
    generations: Vec<u32>,
}

impl<T> Default for SlotMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SlotMap<T> {
    /// Creates an empty slot map.
    #[must_use]
    pub fn new() -> Self {
        Self {
            dense: Vec::new(),
            sparse: Vec::new(),
            reverse: Vec::new(),
            free: Vec::new(),
            generations: Vec::new(),
        }
    }

    /// Inserts `value` and returns a fresh [`Handle`].
    pub fn insert(&mut self, value: T) -> Handle<T> {
        let index = if let Some(i) = self.free.pop() {
            i
        } else {
            let i = u32::try_from(self.sparse.len()).expect("SlotMap capacity overflow");
            self.sparse.push(u32::MAX);
            self.generations.push(0);
            i
        };

        let dense_index = self.dense.len();
        self.dense.push(value);
        debug_assert_eq!(self.reverse.len(), dense_index);
        self.reverse.push(index);

        self.sparse[index as usize] = u32::try_from(dense_index).expect("SlotMap dense overflow");

        Handle {
            index,
            generation: self.generations[index as usize],
            _marker: core::marker::PhantomData,
        }
    }

    /// Removes `handle`'s value when the generation matches.
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        let dense_pos = *self.sparse.get(handle.index as usize)?;
        if dense_pos == u32::MAX {
            return None;
        }
        if self.generations.get(handle.index as usize)? != &handle.generation {
            return None;
        }

        let dense_pos = usize::try_from(dense_pos).ok()?;
        let last = self.dense.len().saturating_sub(1);

        let removed = if dense_pos == last {
            let value = self.dense.pop()?;
            self.reverse.pop();
            value
        } else {
            let value = self.dense.swap_remove(dense_pos);
            let moved_dense_old = last;
            let moved_dense_new = dense_pos;

            let moved_sparse = self.reverse[moved_dense_old];
            self.sparse[moved_sparse as usize] = u32::try_from(moved_dense_new).ok()?;
            self.reverse[moved_dense_new] = moved_sparse;
            self.reverse.pop();
            value
        };

        self.sparse[handle.index as usize] = u32::MAX;
        self.generations[handle.index as usize] =
            self.generations[handle.index as usize].wrapping_add(1);
        self.free.push(handle.index);
        Some(removed)
    }

    /// Returns the dense slice of live values.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.dense
    }
}
