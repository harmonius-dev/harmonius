//! Debug overlay contracts: colors, wireframe capture, contacts, triggers.

use crate::math::{Transform, Vec3};
use crate::model::{ColliderShape, CollisionLayers, ContactPoint, Entity};
use crate::world::World;

/// Wireframe payload for a single collider overlay.
#[derive(Clone, Debug, PartialEq)]
pub struct ColliderDebugData {
    /// Target entity.
    pub entity: Entity,
    /// Shape variant for primitive replay.
    pub shape: ColliderShape,
    /// Trigger volumes use the trigger color table branch.
    pub is_trigger: bool,
}

/// World-space contact sample for debug lines.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContactDebugData {
    /// World contact on A side.
    pub point_a: Vec3,
    /// World contact on B side.
    pub point_b: Vec3,
    /// Manifold normal in world space.
    pub normal: Vec3,
    /// Penetration depth.
    pub depth: f32,
}

/// Overlap event for trigger volumes (TC-IR-5.4.6.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TriggerEvent {
    /// First entity (often trigger owner).
    pub entity_a: Entity,
    /// Second entity (dynamic visitor).
    pub entity_b: Entity,
}

/// Recorded GPU-style primitive for CI (no GPU).
#[derive(Clone, Debug, PartialEq)]
pub enum DebugDrawPrimitive {
    /// Axis-aligned box wireframe in world space (half-extents, center).
    WireframeBox {
        /// Box center.
        center: Vec3,
        /// Half-extents.
        half_extents: Vec3,
        /// ARGB8888 color.
        color_rgba: u32,
    },
    /// Arrow from `origin` along `direction`, `length` world units.
    Arrow {
        /// Tail position.
        origin: Vec3,
        /// Direction (unit length in harness).
        direction: Vec3,
        /// Arrow length.
        length: f32,
        /// ARGB8888 color.
        color_rgba: u32,
    },
    /// Point marker in world space.
    Point {
        /// Marker center.
        position: Vec3,
        /// ARGB8888 color.
        color_rgba: u32,
    },
}

/// Captured debug draws for assertions (TC-IR-5.4.2.x / 4.x).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DebugWireframeCapture {
    /// Ordered primitives.
    pub primitives: Vec<DebugDrawPrimitive>,
}

/// Packs opaque + RGB bytes into ARGB8888 (TC-IR-5.4.U6 hex comparisons).
#[must_use]
pub const fn pack_argb(a: u8, r: u8, g: u8, b: u8) -> u32 {
    u32::from_be_bytes([a, r, g, b])
}

/// Debug color from editor/physics design table (TC-IR-5.4.U6).
///
/// Precedence: error → selected → trigger → sleeping → active.
#[must_use]
pub fn collider_debug_color_hex(
    is_sleeping: bool,
    is_trigger: bool,
    is_selected: bool,
    has_shape_error: bool,
) -> u32 {
    if has_shape_error {
        return pack_argb(0xFF, 0xFF, 0x00, 0x00);
    }
    if is_selected {
        return pack_argb(0xFF, 0x00, 0xFF, 0xFF);
    }
    if is_trigger {
        return pack_argb(0xFF, 0xFF, 0xFF, 0x00);
    }
    if is_sleeping {
        return pack_argb(0xFF, 0x80, 0x80, 0x80);
    }
    pack_argb(0xFF, 0x00, 0xFF, 0x00)
}

/// Converts `ContactPoint::local_a` to world using `transform` (TC-IR-5.4.U7).
#[must_use]
pub fn contact_point_world_a(point: &ContactPoint, transform: Transform) -> Vec3 {
    transform.transform_point(point.local_a)
}

/// Converts `ContactPoint::local_b` to world using `transform` (TC-IR-5.4.U7).
#[must_use]
pub fn contact_point_world_b(point: &ContactPoint, transform: Transform) -> Vec3 {
    transform.transform_point(point.local_b)
}

/// True when membership or mask is all-zero (TC-IR-5.4.N4).
#[must_use]
pub fn collision_layers_need_warning(layers: &CollisionLayers) -> bool {
    layers.membership == 0 || layers.mask == 0
}

/// Narrowphase proxy when convex hull is degenerate (TC-IR-5.4.N3).
#[derive(Clone, Debug, PartialEq)]
pub enum NarrowphaseProxy {
    /// Use the authored shape directly.
    Hull,
    /// Substitute AABB of hull vertices.
    AabbFallback,
}

/// Classifies convex hulls with fewer than four vertices as degenerate.
#[must_use]
pub fn narrowphase_proxy_for_shape(shape: &ColliderShape) -> NarrowphaseProxy {
    let ColliderShape::ConvexHull(h) = shape else {
        return NarrowphaseProxy::Hull;
    };
    if h.vertices.len() < 4 {
        return NarrowphaseProxy::AabbFallback;
    }
    NarrowphaseProxy::Hull
}

/// Logs degenerate hull diagnostics (TC-IR-5.4.N3).
pub fn log_narrowphase_proxy(world: &mut World, shape: &ColliderShape) {
    if matches!(
        narrowphase_proxy_for_shape(shape),
        NarrowphaseProxy::AabbFallback
    ) {
        world.log_diagnostic("degenerate convex hull: narrowphase uses AABB fallback");
    }
}

/// Caps visualization entries; physics solve is unaffected (TC-IR-5.4.N5).
#[must_use]
pub fn cap_contact_debug_entries<T: Clone>(contacts: &[T], max: usize) -> Vec<T> {
    contacts.iter().take(max).cloned().collect()
}

/// Returns true when the layer/mask pair can generate contacts (TC-IR-5.4.5.2).
#[must_use]
pub fn collision_layers_pair_generate_contacts(a: &CollisionLayers, b: &CollisionLayers) -> bool {
    (a.membership & b.mask) != 0 && (b.membership & a.mask) != 0
}

impl DebugWireframeCapture {
    /// Clears the capture buffer.
    pub fn clear(&mut self) {
        self.primitives.clear();
    }

    /// Records collider wireframe using the debug color table.
    pub fn record_collider(
        &mut self,
        data: &ColliderDebugData,
        center: Vec3,
        is_sleeping: bool,
        is_selected: bool,
        has_shape_error: bool,
    ) {
        let color =
            collider_debug_color_hex(is_sleeping, data.is_trigger, is_selected, has_shape_error);
        match &data.shape {
            ColliderShape::Box { half_extents } => {
                self.primitives.push(DebugDrawPrimitive::WireframeBox {
                    center,
                    half_extents: *half_extents,
                    color_rgba: color,
                })
            }
            ColliderShape::Sphere { radius } => {
                let h = Vec3 {
                    x: *radius,
                    y: *radius,
                    z: *radius,
                };
                self.primitives.push(DebugDrawPrimitive::WireframeBox {
                    center,
                    half_extents: h,
                    color_rgba: color,
                });
            }
            ColliderShape::Capsule {
                half_height,
                radius,
            } => {
                let h = Vec3 {
                    x: *radius,
                    y: *half_height + *radius,
                    z: *radius,
                };
                self.primitives.push(DebugDrawPrimitive::WireframeBox {
                    center,
                    half_extents: h,
                    color_rgba: color,
                });
            }
            ColliderShape::ConvexHull(_)
            | ColliderShape::TriMesh(_)
            | ColliderShape::Heightfield => {}
        }
    }

    /// Records compound children at local offsets (world `entity_origin`).
    pub fn record_compound_children(
        &mut self,
        entity: Entity,
        children: &[(ColliderShape, Vec3, bool)],
        entity_origin: Vec3,
        is_sleeping: bool,
        is_selected: bool,
    ) {
        for (shape, local_offset, is_trigger) in children {
            let data = ColliderDebugData {
                entity,
                shape: shape.clone(),
                is_trigger: *is_trigger,
            };
            let center = Vec3 {
                x: entity_origin.x + local_offset.x,
                y: entity_origin.y + local_offset.y,
                z: entity_origin.z + local_offset.z,
            };
            self.record_collider(&data, center, is_sleeping, is_selected, false);
        }
    }

    /// Records contact markers and normal arrow (TC-IR-5.4.4.x).
    pub fn record_contact_manifold(
        &mut self,
        normal: Vec3,
        contacts: &[ContactDebugData],
        point_color: u32,
        arrow_color: u32,
    ) {
        for c in contacts {
            self.primitives.push(DebugDrawPrimitive::Point {
                position: c.point_a,
                color_rgba: point_color,
            });
            self.primitives.push(DebugDrawPrimitive::Point {
                position: c.point_b,
                color_rgba: point_color,
            });
            let dir_len = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
            let direction = if dir_len > f32::EPSILON {
                Vec3 {
                    x: normal.x / dir_len,
                    y: normal.y / dir_len,
                    z: normal.z / dir_len,
                }
            } else {
                Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                }
            };
            self.primitives.push(DebugDrawPrimitive::Arrow {
                origin: c.point_a,
                direction,
                length: 0.25,
                color_rgba: arrow_color,
            });
        }
    }
}
