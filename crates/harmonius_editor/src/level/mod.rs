//! Level-authoring modules (placement, snapping, templates, …).
pub mod snap;

pub use snap::{snap_grid, snap_position, SnapMode, Vec3};
