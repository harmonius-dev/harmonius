//! Containers/slots ↔ UI integration: transient binding types and pure bridge helpers.
//!
//! Coverage is aligned with `docs/design/integration/containers-slots-ui-test-cases.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod binding;
mod bridge;
mod entity;
mod filter_sort;
mod split;
mod transfer;
mod types;

pub use binding::InventoryGridBinding;
pub use bridge::EventBridge;
pub use entity::Entity;
pub use filter_sort::{
    FilterCriteria, SlotItem, TagId, TagSet, filter_slots, sort_slots_stable_by_name,
};
pub use split::clamp_split_quantity;
pub use transfer::{
    DEST_SLOT_AUTO, UiTransferFeedback, UiTransferRequest, is_auto_dest_slot,
    resolve_transfer_quantity,
};
pub use types::{AssetHandle, SortCriteria, StringId, TransferResult, UiIcon};
