#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Headless editor ↔ physics integration harness.
//!
//! Covers undoable commands, debug visualization contracts, and deterministic
//! physics preview aligned with `docs/design/integration/editor-physics.md`.

pub mod command;
pub mod debug;
pub mod gizmo;
pub mod math;
pub mod model;
pub mod preview;
pub mod trigger;
pub mod undo_stack;
pub mod world;

pub use command::{
    ColliderEditCommand, CommandError, CompoundChildEditCommand, EditorCommand,
    MaterialAssignCommand,
};
pub use debug::{
    cap_contact_debug_entries, collider_debug_color_hex, collision_layers_need_warning,
    collision_layers_pair_generate_contacts, contact_point_world_a, contact_point_world_b,
    log_narrowphase_proxy, narrowphase_proxy_for_shape, pack_argb, ColliderDebugData,
    ContactDebugData, DebugDrawPrimitive, DebugWireframeCapture, NarrowphaseProxy, TriggerEvent,
};
pub use gizmo::{
    apply_box_half_extent_delta_x, apply_capsule_half_height_delta, apply_sphere_radius_delta,
};
pub use math::{Mat4, Quat, Transform, Vec3};
pub use model::{
    Collider, ColliderShape, CollisionLayers, CompoundChild, CompoundCollider, ContactPoint,
    ConvexHull, Entity, PhysicsMaterialHandle, RigidBody, SleepTimer, Sleeping, TriMesh,
};
pub use preview::{PhysicsPreview, PreviewBody};
pub use trigger::trigger_event_if_overlapping;
pub use undo_stack::UndoStack;
pub use world::World;
