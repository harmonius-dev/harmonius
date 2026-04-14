//! Generational handles, handle maps, and dense slot maps.

mod handle;
mod handle_map;
mod slot_map;

pub use handle::Handle;
pub use handle_map::{HandleMap, HandleMapError};
pub use slot_map::{SlotMap, SlotMapError};
