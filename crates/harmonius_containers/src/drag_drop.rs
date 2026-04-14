//! UI drag-drop routing through the same validator used by gameplay transfers.

use crate::container::{Container, ItemStack, SlotEntry};
use crate::entity::Entity;
use crate::transfer::TransferError;
use crate::validate::validate_transfer;

/// UI-originated drag-drop request between two linear containers.
pub struct DragDropRequest<'a> {
    /// Source container holding `item`.
    pub source: &'a mut Container,
    /// Destination container.
    pub target: &'a mut Container,
    /// Item entity being moved.
    pub item: Entity,
}

/// Successful transfer notification (exactly one per successful drag-drop).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemTransferred {
    /// Moved item entity.
    pub item: Entity,
}

/// Validates and applies a drag-drop move atomically.
pub fn process_drag_drop(req: DragDropRequest<'_>) -> Result<ItemTransferred, TransferError> {
    let DragDropRequest {
        source,
        target,
        item,
    } = req;

    let slot_entry = source
        .slots()
        .iter()
        .find(|slot| matches!(slot, SlotEntry::Item { entity: e, .. } if *e == item))
        .ok_or(TransferError::ItemNotFound)?;

    let (entity, weight, name, rarity, tags) = match slot_entry {
        SlotEntry::Item {
            entity,
            weight,
            name,
            rarity,
            tags,
        } => (*entity, *weight, name.clone(), *rarity, tags.clone()),
        _ => return Err(TransferError::ItemNotFound),
    };

    validate_transfer(target, weight, &tags)?;
    source.remove_entity(item)?;
    target.insert(ItemStack::Single {
        entity,
        weight,
        name,
        rarity,
        tags,
    })?;

    Ok(ItemTransferred { item })
}
