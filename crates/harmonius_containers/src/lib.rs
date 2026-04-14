//! Bounded containers, grid occupancy, stacking, nesting, sorting, sockets, and transfer validation.
//!
//! Test names map to `TC-*` identifiers in `docs/design/data-systems/containers-slots-test-cases.md`.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod container;
pub mod drag_drop;
pub mod entity;
pub mod equipment;
pub mod grid;
pub mod nesting;
pub mod socket;
pub mod sort;
pub mod stats;
pub mod tags;
pub mod transfer;
pub mod validate;

pub use container::{Container, ContainerDef, ItemStack, SlotEntry, sort_container};
pub use drag_drop::{DragDropRequest, ItemTransferred, process_drag_drop};
pub use entity::Entity;
pub use equipment::{EquipmentSlot, EquipmentSlots, try_equip};
pub use grid::GridContainer;
pub use nesting::{compute_depth, validate_circular, validate_nesting_insert};
pub use socket::Socket;
pub use sort::{Rarity, SortCriterion, SortOrder};
pub use stats::{
    AttributeSet, StatModifier, apply_socket_modifiers, clear_socket_modifiers, read_strength,
};
pub use tags::TagSet;
pub use transfer::{SlotConstraintMismatch, TransferError};
pub use validate::validate_transfer;
