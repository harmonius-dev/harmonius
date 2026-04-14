//! World anchor projection with optional depth visibility (CPU latch).

use crate::fallback::FallbackMetrics;
use crate::pick::PickCameraRig;
use crate::types::{CameraId, ProjectFlags, Vec2, Vec3, WorldProjectRequest, WorldProjectResult};

/// Camera matrices and viewport for projection + pick alignment.
#[derive(Clone, Debug, PartialEq)]
pub struct CameraFixture {
    /// Slot id.
    pub id: CameraId,
    /// Viewport width in pixels.
    pub viewport_width: f32,
    /// Viewport height in pixels.
    pub viewport_height: f32,
    /// Vertical field of view (radians).
    pub fov_y: f32,
    /// Near plane distance (positive).
    pub near: f32,
    /// Far plane distance (positive).
    pub far: f32,
    /// Pick rig matching this camera.
    pub pick_rig: PickCameraRig,
}

impl CameraFixture {
    /// Default harness camera: 800x600, 60 deg vertical FOV, identity pick rig.
    pub fn harness_default(id: CameraId) -> Self {
        Self {
            id,
            viewport_width: 800.0,
            viewport_height: 600.0,
            fov_y: std::f32::consts::FRAC_PI_3,
            near: 0.1,
            far: 200.0,
            pick_rig: PickCameraRig::standard_origin(),
        }
    }

    fn aspect(&self) -> f32 {
        self.viewport_width / self.viewport_height
    }

    /// Tangent of half the vertical field of view (matches pick rays).
    pub fn tan_half_fov_y(&self) -> f32 {
        (self.fov_y * 0.5).tan()
    }

    /// Projects `world` to screen pixels and a monotonic depth key in \[0, 1).
    pub fn project_world(&self, world: Vec3) -> (Vec2, f32) {
        let view_z = -world.z;
        let t = self.tan_half_fov_y();
        let a = self.aspect();
        let ndc_x = (world.x / view_z) / (t * a);
        let ndc_y = (world.y / view_z) / t;
        let sx = (ndc_x * 0.5 + 0.5) * self.viewport_width;
        let sy = (1.0 - (ndc_y * 0.5 + 0.5)) * self.viewport_height;
        let ndc_d = ((view_z - self.near) / (self.far - self.near)).clamp(0.0, 1.0);
        (Vec2 { x: sx, y: sy }, ndc_d)
    }

    fn ndc_of_screen(&self, screen: Vec2) -> Vec2 {
        let ndc_x = (screen.x / self.viewport_width) * 2.0 - 1.0;
        let ndc_y = (1.0 - screen.y / self.viewport_height) * 2.0 - 1.0;
        Vec2 { x: ndc_x, y: ndc_y }
    }

    fn clamp_screen(&self, mut s: Vec2) -> Vec2 {
        s.x = s.x.clamp(0.0, self.viewport_width);
        s.y = s.y.clamp(0.0, self.viewport_height);
        s
    }

    fn off_screen_ndc(&self, ndc: Vec2) -> bool {
        ndc.x < -1.0 || ndc.x > 1.0 || ndc.y < -1.0 || ndc.y > 1.0
    }
}

/// Registered cameras for a frame.
#[derive(Clone, Debug, Default)]
pub struct CameraTable {
    cams: Vec<CameraFixture>,
}

impl CameraTable {
    /// Creates a table with a single camera (common in tests).
    pub fn single(cam: CameraFixture) -> Self {
        Self { cams: vec![cam] }
    }

    /// Looks up a camera by id.
    pub fn get(&self, id: CameraId) -> Option<&CameraFixture> {
        self.cams.iter().find(|c| c.id == id)
    }
}

/// Latched depth readback (uniform depth for CI fixtures).
#[derive(Clone, Debug, PartialEq)]
pub struct DepthReadbackLatch {
    /// Frame index when `uniform_depth_ndc` was last committed.
    pub last_committed_frame: u64,
    /// Engine frame counter (may advance without a depth commit).
    pub current_frame: u64,
    /// Single depth value used for every screen sample in fixtures.
    pub uniform_depth_ndc: f32,
}

impl Default for DepthReadbackLatch {
    fn default() -> Self {
        Self {
            last_committed_frame: 0,
            current_frame: 0,
            uniform_depth_ndc: 0.5,
        }
    }
}

impl DepthReadbackLatch {
    /// Marks the logical frame boundary for staleness checks.
    pub fn set_current_frame(&mut self, frame: u64) {
        self.current_frame = frame;
    }

    /// Simulates render-thread depth commit.
    pub fn commit_frame(&mut self, frame: u64, uniform_depth_ndc: f32) {
        self.last_committed_frame = frame;
        self.uniform_depth_ndc = uniform_depth_ndc;
    }

    /// Samples latched depth at screen coordinates (fixture ignores x/y).
    pub fn sample_ndc_depth(&self, _sx: f32, _sy: f32) -> f32 {
        self.uniform_depth_ndc
    }
}

/// Batch world projection helper.
pub struct WorldProjectBatch;

impl WorldProjectBatch {
    /// Projects every request in input order.
    pub fn project_all(
        requests: &[WorldProjectRequest],
        table: &CameraTable,
        depth: &DepthReadbackLatch,
        metrics: &mut FallbackMetrics,
    ) -> Vec<WorldProjectResult> {
        requests
            .iter()
            .map(|r| project_one(r, table, depth, metrics))
            .collect()
    }
}

fn project_one(
    req: &WorldProjectRequest,
    table: &CameraTable,
    depth: &DepthReadbackLatch,
    metrics: &mut FallbackMetrics,
) -> WorldProjectResult {
    let Some(cam) = table.get(req.camera) else {
        metrics.fm6_unknown_camera = metrics.fm6_unknown_camera.saturating_add(1);
        return WorldProjectResult {
            screen_position: Vec2 { x: 0.0, y: 0.0 },
            ndc_depth: 0.0,
            visible: false,
        };
    };
    if depth.current_frame > depth.last_committed_frame + 1 {
        metrics.fm3_depth_stale = metrics.fm3_depth_stale.saturating_add(1);
    }
    let (mut screen, ndc_d) = cam.project_world(req.world_position);
    let ndc = cam.ndc_of_screen(screen);
    let mut visible = true;
    if cam.off_screen_ndc(ndc) {
        if req.flags.has(ProjectFlags::CLAMP_TO_SCREEN) {
            screen = cam.clamp_screen(screen);
        } else if !req.flags.has(ProjectFlags::ALLOW_OFF_SCREEN) {
            visible = false;
        }
    }
    if req.flags.has(ProjectFlags::NEED_VISIBILITY) {
        let latch = depth.sample_ndc_depth(screen.x, screen.y);
        if ndc_d > latch + 1e-4 {
            visible = false;
        }
    }
    WorldProjectResult {
        screen_position: screen,
        ndc_depth: ndc_d,
        visible,
    }
}
