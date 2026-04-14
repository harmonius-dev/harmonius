#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Editor selection primitives: [`SelectionState`], marquee and lasso helpers, and
//! [`SelectionChanged`] notifications.
//!
//! This crate implements the data structures described in `docs/design/tools/selection-model.md`
//! without coupling to a full ECS host so unit tests can run in isolation.
//!
//! Editor-only request types (`PickRequest`, marquee/lasso payloads) stay in the host until the
//! tools stack wires transport; this crate focuses on deterministic selection math and state.

mod aggregate;
mod events;
mod ray;
mod screen_selection;
mod selectable;
mod selection_state;
mod types;

pub use aggregate::aggregate_affine_for_selection;
pub use events::SelectionChanged;
pub use ray::{
    nearest_subobject_along_ray, ray_triangle_intersect, raycast_spheres, Ray3, SubObjectRayHit,
};
pub use screen_selection::{lasso_select_by_centroid, marquee_select, IntersectMode, ScreenRect};
pub use selectable::{filter_pickable_entities, resolve_pick_entity, Selectable};
pub use selection_state::{SelectionSnapshot, SelectionState};
pub use types::{EditorWorldId, EntityRef, SubObjectElement, SubObjectKind};
