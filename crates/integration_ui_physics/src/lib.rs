//! UI ↔ physics integration contracts (CPU fixtures).
//!
//! Implements the data flow in `docs/design/integration/ui-physics.md` for CI tests.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod fallback;
mod pick;
mod project;
mod reticle;
mod tooltip;
mod types;

pub use fallback::FallbackMetrics;
pub use pick::{
    resolve_pick_through_channel, resolve_world_pick, PhysicsPickScene, PickBody, PickCameraRig,
    Ray, UiPickChannel,
};
pub use project::{CameraFixture, CameraTable, DepthReadbackLatch, WorldProjectBatch};
pub use reticle::{compute_reticle_snap, AimableTarget};
pub use tooltip::{TooltipComponent, TooltipRenderEntry, TooltipUiState};
pub use types::{
    CameraId, CollisionMask, Entity, LocalizedStringId, ProjectFlags, ReticleSnap, Vec2, Vec3,
    WorldPickRequest, WorldPickResult, WorldProjectRequest, WorldProjectResult,
};
pub use types::{L_DEFAULT, L_ENEMY};
