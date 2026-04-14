//! Shared container and utility primitives for Harmonius core runtime.
//!
//! This crate implements the types described in `docs/design/core-runtime/primitives.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
// `RingBuffer` uses `MaybeUninit` and manual drop logic as specified by the design.
#![allow(unsafe_code)]

mod aliases;
mod budget_allocator;
mod deterministic_rng;
mod dirty_region_set;
mod dispatch_table;
mod generational_index;
mod handle;
mod handle_map;
mod ring_buffer;
mod slot_map;
mod sorted_vec_map;

pub use aliases::{FixedBitSet, SmallVec};
pub use budget_allocator::BudgetAllocator;
pub use deterministic_rng::DeterministicRng;
pub use dirty_region_set::{DirtyRegionSet, Region};
pub use dispatch_table::DispatchTable;
pub use generational_index::GenerationalIndex;
pub use handle::Handle;
pub use handle_map::HandleMap;
pub use ring_buffer::RingBuffer;
pub use slot_map::SlotMap;
pub use sorted_vec_map::SortedVecMap;
