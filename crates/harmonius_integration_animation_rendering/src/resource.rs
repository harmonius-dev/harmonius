//! Minimal resource manager validating generational handles.

use crate::handle::GpuBuffer;
use crate::handle::Handle;
use core::fmt;
use std::error::Error;

/// Errors returned when resolving GPU resources.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceError {
    /// Handle generation did not match the live slot.
    StaleHandle,
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceError::StaleHandle => write!(f, "stale GPU handle"),
        }
    }
}

impl Error for ResourceError {}

/// Slot storage for [`Handle<GpuBuffer>`] validation tests.
#[derive(Clone, Debug, Default)]
pub struct GpuBufferTable {
    generations: Vec<u32>,
}

impl GpuBufferTable {
    /// Allocates a new buffer slot and returns its handle.
    pub fn allocate(&mut self) -> Handle<GpuBuffer> {
        let index = self.generations.len() as u32;
        self.generations.push(1);
        Handle::from_raw(index, 1)
    }

    /// Frees a slot, bumping generation so older handles become stale.
    pub fn free(&mut self, handle: Handle<GpuBuffer>) -> Result<(), ResourceError> {
        let idx = handle.index() as usize;
        let g = self
            .generations
            .get_mut(idx)
            .ok_or(ResourceError::StaleHandle)?;
        if *g != handle.generation() {
            return Err(ResourceError::StaleHandle);
        }
        *g = g.saturating_add(1);
        Ok(())
    }

    /// Validates a handle against the live generation.
    pub fn validate(&self, handle: Handle<GpuBuffer>) -> Result<(), ResourceError> {
        let idx = handle.index() as usize;
        let live = self
            .generations
            .get(idx)
            .copied()
            .ok_or(ResourceError::StaleHandle)?;
        if live != handle.generation() {
            return Err(ResourceError::StaleHandle);
        }
        Ok(())
    }
}
