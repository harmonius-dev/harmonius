use crate::entity::Entity;
use crate::types::TransferResult;

/// `dest_slot == u16::MAX` selects auto-place (matches `TransferRequest` design).
pub const DEST_SLOT_AUTO: u16 = u16::MAX;

/// Returns `true` when the destination slot requests automatic placement.
#[must_use]
pub const fn is_auto_dest_slot(dest_slot: u16) -> bool {
    dest_slot == DEST_SLOT_AUTO
}

/// Drag-drop operation initiated by UI, resolved by `TransferSystem` in the game world.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UiTransferRequest {
    /// Source container entity.
    pub source_container: Entity,
    /// Slot index (`u16` matches `ContainerSlot.slot_index`).
    pub source_slot: u16,
    /// Destination container entity.
    pub dest_container: Entity,
    /// Destination slot; [`DEST_SLOT_AUTO`] requests auto placement.
    pub dest_slot: u16,
    /// Item quantity; `0` means entire stack when `split` is `false`.
    pub quantity: u32,
    /// `true` when the request originates from a stack-split modal.
    pub split: bool,
}

/// UI-visible feedback pairing the originating request with a [`TransferResult`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UiTransferFeedback {
    /// Original UI request.
    pub request: UiTransferRequest,
    /// Engine validation / apply outcome.
    pub result: TransferResult,
}

/// Resolves `quantity` per TC-IR-5.9.3.U1 and split semantics (TC-IR-5.9.4.*).
#[must_use]
pub fn resolve_transfer_quantity(quantity: u32, stack_qty: u32, split: bool) -> u32 {
    if stack_qty == 0 {
        return 0;
    }
    if split {
        return crate::split::clamp_split_quantity(quantity, stack_qty);
    }
    if quantity == 0 {
        stack_qty
    } else {
        quantity.min(stack_qty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_5_9_3_u1_quantity_zero_full_stack() {
        let q = resolve_transfer_quantity(0, 7, false);
        assert_eq!(q, 7);
    }

    #[test]
    fn tc_ir_5_9_3_u2_auto_dest_slot_constant() {
        assert!(is_auto_dest_slot(DEST_SLOT_AUTO));
        assert!(!is_auto_dest_slot(0));
    }
}
