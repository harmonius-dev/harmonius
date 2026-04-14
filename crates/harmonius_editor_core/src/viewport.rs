//! Viewport configuration and picking rays (`TC-15.1.2.*`).

use glam::{Mat4, Vec2, Vec3};
use std::collections::HashMap;
use std::fmt;

/// Identifier for an editor viewport instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ViewportId(pub u32);

/// Named 3D ray for picking tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    /// Ray origin in world space.
    pub origin: Vec3,
    /// Unit direction in world space.
    pub direction: Vec3,
}

/// Supported editor camera projections.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CameraMode {
    /// Unconstrained fly camera.
    FreeFly,
    /// Preview from the active player camera.
    PlayerPreview,
    /// Orthographic top-down.
    OrthoTop,
    /// Orthographic front (+Z).
    OrthoFront,
    /// Orthographic right (+X).
    OrthoRight,
    /// Orbit around a pivot.
    Orbit,
    /// Dedicated 2D editing camera.
    Camera2D,
}

/// Per-viewport render and debug options.
#[derive(Clone, Debug, PartialEq)]
pub struct ViewportConfig {
    /// Human-readable label.
    pub name: String,
    /// Active camera mode.
    pub camera_mode: CameraMode,
    /// Full-frame pixel dimensions.
    pub size_px: [u32; 2],
    /// View-projection matrix for the current frame.
    pub view_proj: Mat4,
    /// Inverse view-projection for unprojection.
    pub inv_view_proj: Mat4,
}

/// Recoverable viewport errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ViewportError {
    /// Unknown viewport id.
    UnknownViewport,
}

impl fmt::Display for ViewportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViewportError::UnknownViewport => write!(f, "unknown viewport"),
        }
    }
}

impl std::error::Error for ViewportError {}

/// Owns editor viewports keyed by [`ViewportId`].
#[derive(Debug, Default)]
pub struct ViewportManager {
    map: HashMap<ViewportId, ViewportConfig>,
    next: u32,
}

impl ViewportManager {
    /// Creates an empty manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocates a viewport with `config`, returning its id.
    pub fn create(&mut self, mut config: ViewportConfig) -> Result<ViewportId, ViewportError> {
        let id = ViewportId(self.next);
        self.next = self.next.saturating_add(1);
        config.inv_view_proj = config.view_proj.inverse();
        self.map.insert(id, config);
        Ok(id)
    }

    /// Drops a viewport instance.
    pub fn destroy(&mut self, id: ViewportId) -> Result<(), ViewportError> {
        self.map.remove(&id).ok_or(ViewportError::UnknownViewport)?;
        Ok(())
    }

    /// Builds a world-space pick ray from normalized device coordinates.
    pub fn screen_to_ray(&self, id: ViewportId, pos_px: Vec2) -> Option<Ray> {
        let cfg = self.map.get(&id)?;
        let w = cfg.size_px[0] as f32;
        let h = cfg.size_px[1] as f32;
        if w <= 0.0 || h <= 0.0 {
            return None;
        }
        let ndc_x = (pos_px.x / w) * 2.0 - 1.0;
        let ndc_y = 1.0 - (pos_px.y / h) * 2.0;
        let clip_near = Vec3::new(ndc_x, ndc_y, 0.0);
        let clip_far = Vec3::new(ndc_x, ndc_y, 1.0);
        let world_near = cfg.inv_view_proj.project_point3(clip_near);
        let world_far = cfg.inv_view_proj.project_point3(clip_far);
        let dir = (world_far - world_near).normalize_or_zero();
        if dir.length_squared() == 0.0 {
            return None;
        }
        Some(Ray {
            origin: world_near,
            direction: dir,
        })
    }

    /// Borrows a configuration for tests.
    pub fn get(&self, id: ViewportId) -> Option<&ViewportConfig> {
        self.map.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity_config(name: &str) -> ViewportConfig {
        ViewportConfig {
            name: name.to_string(),
            camera_mode: CameraMode::FreeFly,
            size_px: [800, 600],
            view_proj: Mat4::IDENTITY,
            inv_view_proj: Mat4::IDENTITY,
        }
    }

    #[test]
    fn tc_15_1_2_1_viewport_create() {
        let mut mgr = ViewportManager::new();
        let id = mgr.create(identity_config("main")).unwrap();
        assert_eq!(mgr.get(id).unwrap().name, "main");
    }

    #[test]
    fn tc_15_1_2_2_viewport_camera_modes() {
        let mut cfg = identity_config("ortho");
        cfg.camera_mode = CameraMode::OrthoTop;
        let mut mgr = ViewportManager::new();
        let id = mgr.create(cfg).unwrap();
        assert_eq!(mgr.get(id).unwrap().camera_mode, CameraMode::OrthoTop);
    }

    #[test]
    fn tc_15_1_2_3_viewport_screen_to_ray() {
        let mut mgr = ViewportManager::new();
        let id = mgr.create(identity_config("pick")).unwrap();
        let ray = mgr.screen_to_ray(id, Vec2::new(400.0, 300.0)).expect("ray");
        assert!((ray.origin - Vec3::new(0.0, 0.0, 0.0)).length() < 1e-3);
        assert!((ray.direction - Vec3::new(0.0, 0.0, 1.0)).length() < 1e-3);
    }
}
