//! IR-2.5.3 climb detection via a vertical capsule-style sweep surrogate.

use glam::Vec3;

use crate::geometry::AxisAlignedBox;

/// Climb sweep query issued by AI motion planning.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClimbSweepQuery {
    pub origin: Vec3,
    pub direction: Vec3,
    pub capsule_radius: f32,
    pub max_height: f32,
}

/// Result of a climb sweep against simplified ledge geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClimbSweepResult {
    pub ledge_top: Option<Vec3>,
    pub climb_height: f32,
}

/// Axis-aligned ledge used by the reference harness (thin slab).
#[derive(Clone, Debug)]
pub struct ClimbScene {
    pub wall_x: f32,
    pub ledge: AxisAlignedBox,
}

impl ClimbScene {
    /// Sweeps `query` against `self`, returning the first ledge top within `max_height`.
    #[must_use]
    pub fn sweep(&self, query: ClimbSweepQuery) -> ClimbSweepResult {
        if query.direction.length_squared() < 1e-12 {
            return ClimbSweepResult {
                ledge_top: None,
                climb_height: 0.0,
            };
        }

        let dir = query.direction.normalize();
        let steps = 256;
        for i in 1..=steps {
            let t = (i as f32 / steps as f32) * query.max_height;
            let p = query.origin + dir * t;
            let dx = (p.x - self.wall_x).abs();
            if dx > query.capsule_radius {
                continue;
            }

            if self.ledge_contains(p) {
                let top = Vec3::new(p.x, self.ledge.max.y, p.z);
                let climb_height = top.y - query.origin.y;
                return ClimbSweepResult {
                    ledge_top: Some(top),
                    climb_height,
                };
            }
        }

        ClimbSweepResult {
            ledge_top: None,
            climb_height: 0.0,
        }
    }

    fn ledge_contains(&self, p: Vec3) -> bool {
        p.x >= self.ledge.min.x - 1e-3
            && p.x <= self.ledge.max.x + 1e-3
            && p.z >= self.ledge.min.z - 1e-3
            && p.z <= self.ledge.max.z + 1e-3
            && p.y + 1e-3 >= self.ledge.min.y
            && p.y - 1e-3 <= self.ledge.max.y
    }
}

/// Curved-wall fixture: vertical cylinder `x^2 + z^2 = r^2` with an outward ledge tangent in +X.
#[derive(Clone, Copy, Debug)]
pub struct CylinderClimbFixture {
    pub radius: f32,
    pub ledge_height: f32,
}

impl CylinderClimbFixture {
    /// Returns the analytical tangent ledge top for an agent outside the cylinder on +X.
    #[must_use]
    pub fn tangent_ledge_top(&self, agent_xz: Vec3) -> Vec3 {
        let r = self.radius;
        let x0 = agent_xz.x;
        let z0 = agent_xz.z;
        let len = (x0 * x0 + z0 * z0).sqrt();
        let scale = r / len;
        Vec3::new(x0 * scale, self.ledge_height, z0 * scale)
    }
}
