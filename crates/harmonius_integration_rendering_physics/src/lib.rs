//! Rendering ↔ physics integration helpers: debug draw buffers, interpolation alpha, and
//! wireframe emitters that match `docs/design/integration/rendering-physics.md`.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(missing_docs)]

mod alpha;
mod buffer;
mod bvh_debug;
mod channel;
mod config;
mod contacts;
mod debug_system;
mod ray_debug;
mod types;
mod wireframe;

pub use alpha::{interp_alpha_from_fixed_accumulator, lerp_transform_translation};
pub use buffer::DebugDrawBuffer;
pub use bvh_debug::{bvh_debug_lines, BvhNode};
pub use channel::{BoundedRenderFrameSender, RenderFrame};
pub use config::PhysicsDebugConfig;
pub use contacts::{contact_debug_lines, ContactManifold, ContactPoint};
pub use debug_system::{run_physics_debug_system, PhysicsDebugWorldSnapshot};
pub use ray_debug::{ray_debug_lines, shape_cast_debug_lines, RayCastResult, ShapeCastResult};
pub use types::{
    ColliderShape, DebugLine, InterpAlpha, InterpolatedTransform, LinearColor, RigidBodyType,
    Transform,
};
pub use wireframe::{
    capsule_wireframe_lines, collider_wireframe_lines, convex_hull_wireframe_lines,
    heightfield_wireframe_lines, triangle_mesh_wireframe_lines,
};
