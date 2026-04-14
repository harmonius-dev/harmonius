//! Transfer and attach error taxonomy aligned with design test cases.

use crate::tags::TagSet;

/// Slot constraint failure for equipment-style sockets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlotConstraintMismatch {
    /// Human-readable slot name (for example `"helmet"`).
    pub slot: String,
    /// Tags accepted by the slot.
    pub allowed: TagSet,
    /// Tags carried by the item.
    pub provided: TagSet,
}

/// Errors surfaced by validators and mutating transfer helpers.
#[derive(Clone, Debug, PartialEq)]
pub enum TransferError {
    /// No free linear slot is available.
    ContainerFull,
    /// Weight capacity would be exceeded.
    OverWeight {
        /// Declared limit in kilograms.
        limit: f32,
        /// Post-transfer projected total weight.
        attempted: f32,
    },
    /// Tag gates rejected the item.
    TagMismatch {
        /// Required tags (container or socket).
        required: Vec<String>,
        /// Tags present on the item.
        provided: Vec<String>,
    },
    /// Grid placement could not find a non-overlapping region.
    NoFreeRegion,
    /// Nested container depth would exceed the authored maximum.
    NestingTooDeep {
        /// Maximum allowed depth.
        max: u8,
        /// Depth after the attempted transfer.
        attempted: u8,
    },
    /// Would place an ancestor container inside its descendant.
    CircularNesting,
    /// Equipment slot tags did not accept the item.
    SlotConstraintMismatch(Box<SlotConstraintMismatch>),
    /// Requested entity was not present in the container.
    ItemNotFound,
}
