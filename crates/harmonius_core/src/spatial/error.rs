use super::entity::Entity;
use super::grid::GridCoord;
use super::handle::BvhHandle;
use glam::UVec2;

/// Recoverable spatial index errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpatialError {
    /// Handle generation did not match the slot.
    StaleHandle {
        /// Stale handle value.
        handle: BvhHandle,
    },
    /// Entity was not present in the structure.
    EntityNotFound {
        /// Missing entity id.
        entity: Entity,
    },
    /// Grid coordinate was outside configured dimensions.
    OutOfBounds {
        /// Coordinate that failed validation.
        coord: GridCoord,
        /// Grid extent in cells.
        dimensions: UVec2,
    },
}
