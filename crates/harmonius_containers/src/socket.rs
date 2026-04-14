//! Typed sockets with tag compatibility and occupant tracking.
//!
//! Offsets use [`glam::Vec3`]; gameplay binding is 2D with `z = 0` per subsystem constraints.

use glam::Vec3;

use crate::entity::Entity;
use crate::tags::TagSet;
use crate::transfer::TransferError;

/// A single attachment socket on an item or character.
#[derive(Clone, Debug, PartialEq)]
pub struct Socket {
    /// Tags required on attachments (all must be present).
    pub required_tags: TagSet,
    /// Currently attached entity, if any.
    pub occupant: Option<Entity>,
    /// Local-space offset persisted across attach cycles.
    pub transform_offset: Vec3,
}

impl Socket {
    /// Creates a socket with required tags and offset.
    #[must_use]
    pub fn new(required_tags: TagSet, transform_offset: Vec3) -> Self {
        Self {
            required_tags,
            occupant: None,
            transform_offset,
        }
    }

    /// Attaches `item` when its tags satisfy `required_tags`.
    pub fn attach(&mut self, item: Entity, item_tags: &TagSet) -> Result<(), TransferError> {
        if self.occupant.is_some() {
            return Err(TransferError::SocketOccupied);
        }
        let required: Vec<String> = self.required_tags.to_vec();
        if !item_tags.contains_all(&required) {
            return Err(TransferError::TagMismatch {
                required: required.clone(),
                provided: item_tags.to_vec(),
            });
        }
        self.occupant = Some(item);
        Ok(())
    }

    /// Clears the occupant, if present.
    pub fn detach(&mut self) -> Option<Entity> {
        self.occupant.take()
    }
}
