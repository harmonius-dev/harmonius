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
    /// Item entity being moved, or stack [`Entity`](crate::entity::Entity) id when moving a stack.
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

    if source
        .slots()
        .iter()
        .any(|slot| matches!(slot, SlotEntry::Item { entity: e, .. } if *e == item))
    {
        return transfer_single_item(source, target, item);
    }

    if source
        .slots()
        .iter()
        .any(|slot| matches!(slot, SlotEntry::Stack { kind: k, .. } if *k == item))
    {
        return transfer_stack_by_kind(source, target, item);
    }

    Err(TransferError::ItemNotFound)
}

fn transfer_single_item(
    source: &mut Container,
    target: &mut Container,
    item: Entity,
) -> Result<ItemTransferred, TransferError> {
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

fn transfer_stack_by_kind(
    source: &mut Container,
    target: &mut Container,
    kind: Entity,
) -> Result<ItemTransferred, TransferError> {
    let slot_entry = source
        .slots()
        .iter()
        .find(|slot| matches!(slot, SlotEntry::Stack { kind: k, .. } if *k == kind))
        .ok_or(TransferError::ItemNotFound)?;

    let (count, max_stack, per_unit_weight, name, rarity, tags) = match slot_entry {
        SlotEntry::Stack {
            count,
            max_stack,
            per_unit_weight,
            name,
            rarity,
            tags,
            ..
        } => (
            *count,
            *max_stack,
            *per_unit_weight,
            name.clone(),
            *rarity,
            tags.clone(),
        ),
        _ => return Err(TransferError::ItemNotFound),
    };

    let weight = count as f32 * per_unit_weight;
    validate_transfer(target, weight, &tags)?;
    source.remove_stack_kind(kind)?;
    target.insert(ItemStack::Stack {
        kind,
        count,
        max_stack,
        per_unit_weight,
        name,
        rarity,
        tags,
    })?;

    Ok(ItemTransferred { item: kind })
}
