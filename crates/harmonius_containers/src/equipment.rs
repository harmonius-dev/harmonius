//! Typed equipment slots with tag constraints.

use crate::entity::Entity;
use crate::tags::TagSet;
use crate::transfer::{SlotConstraintMismatch, TransferError};

/// A single equipment slot with allowed tags.
#[derive(Clone, Debug, PartialEq)]
pub struct EquipmentSlot {
    /// Tags accepted by this slot (all must be present on the item).
    pub allowed_tags: TagSet,
    /// Equipped entity, if any.
    pub occupant: Option<Entity>,
}

impl EquipmentSlot {
    /// Creates an empty slot with allowed tags.
    #[must_use]
    pub fn new(allowed_tags: TagSet) -> Self {
        Self {
            allowed_tags,
            occupant: None,
        }
    }
}

/// Character equipment layout used by inventory tests.
#[derive(Clone, Debug, PartialEq)]
pub struct EquipmentSlots {
    /// Helmet slot.
    pub helmet: EquipmentSlot,
    /// Chest slot.
    pub chest: EquipmentSlot,
    /// Legs slot.
    pub legs: EquipmentSlot,
    /// Weapon slot.
    pub weapon: EquipmentSlot,
}

/// Attempts to equip `item` into `slot`, enforcing tag compatibility.
pub fn try_equip(
    slot: &mut EquipmentSlot,
    slot_name: &str,
    item: Entity,
    item_tags: &TagSet,
) -> Result<(), TransferError> {
    if slot.occupant.is_some() {
        return Err(TransferError::EquipmentOccupied {
            slot: slot_name.to_string(),
        });
    }
    let allowed = slot.allowed_tags.to_vec();
    if !item_tags.contains_all(&allowed) {
        return Err(TransferError::SlotConstraintMismatch(Box::new(
            SlotConstraintMismatch {
                slot: slot_name.to_string(),
                allowed: slot.allowed_tags.clone(),
                provided: item_tags.clone(),
            },
        )));
    }
    slot.occupant = Some(item);
    Ok(())
}
