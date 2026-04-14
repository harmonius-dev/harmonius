use crate::types::LinearColor;

/// Runtime configuration for physics debug drawing (not `#[cfg]` gated).
#[derive(Clone, Debug, PartialEq)]
pub struct PhysicsDebugConfig {
    pub draw_colliders: bool,
    pub draw_contacts: bool,
    pub draw_bvh: bool,
    pub draw_raycasts: bool,
    pub draw_shapecasts: bool,
    pub bvh_max_depth: u32,
    pub max_debug_lines: u32,
    pub collider_color_static: LinearColor,
    pub collider_color_dynamic: LinearColor,
    pub collider_color_kinematic: LinearColor,
    pub bvh_color_leaf: LinearColor,
    pub bvh_color_internal: LinearColor,
    pub contact_normal_scale: f32,
    pub contact_point_dot_radius: f32,
}

impl PhysicsDebugConfig {
    /// Returns `true` when any visualization category is enabled.
    pub fn any_draw_enabled(&self) -> bool {
        self.draw_colliders
            || self.draw_contacts
            || self.draw_bvh
            || self.draw_raycasts
            || self.draw_shapecasts
    }

    /// Default colors match the integration design examples.
    pub fn default_colors() -> Self {
        Self {
            draw_colliders: false,
            draw_contacts: false,
            draw_bvh: false,
            draw_raycasts: false,
            draw_shapecasts: false,
            bvh_max_depth: 8,
            max_debug_lines: 1_000_000,
            collider_color_static: LinearColor::new(0.5, 0.5, 0.5, 1.0),
            collider_color_dynamic: LinearColor::new(1.0, 0.2, 0.2, 1.0),
            collider_color_kinematic: LinearColor::new(0.2, 0.6, 1.0, 1.0),
            bvh_color_leaf: LinearColor::new(0.2, 1.0, 0.2, 1.0),
            bvh_color_internal: LinearColor::new(1.0, 1.0, 0.2, 1.0),
            contact_normal_scale: 1.0,
            contact_point_dot_radius: 0.05,
        }
    }
}

/// Maps rigid body type to the configured collider color.
pub fn collider_color_for_body_type(
    cfg: &PhysicsDebugConfig,
    body: crate::types::RigidBodyType,
) -> LinearColor {
    match body {
        crate::types::RigidBodyType::Static => cfg.collider_color_static,
        crate::types::RigidBodyType::Dynamic => cfg.collider_color_dynamic,
        crate::types::RigidBodyType::Kinematic => cfg.collider_color_kinematic,
    }
}
