//! IR-2.5.4 physics-aware avoidance neighbor enumeration.

use glam::Vec3;
use smallvec::SmallVec;

use crate::metrics::FallbackMetrics;
use crate::types::Entity;

/// Query for nearby dynamic rigid bodies used by ORCA/RVO steering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AvoidanceQuery {
    pub center: Vec3,
    pub radius: f32,
    pub self_entity: Entity,
}

/// Neighbor state exported to steering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NeighborState {
    pub position: Vec3,
    pub velocity: Vec3,
    pub radius: f32,
    pub entity: Entity,
}

/// Bounded neighbor buffer (`FM-6` applies when more than 16 candidates exist).
#[derive(Clone, Debug, PartialEq)]
pub struct AvoidanceResult {
    pub neighbors: SmallVec<[NeighborState; 16]>,
}

/// Rigid body classification for avoidance filtering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RigidBodyKind {
    /// Dynamic body with linear velocity used by avoidance.
    Dynamic {
        /// Linear velocity in world units per second.
        linvel: Vec3,
    },
    /// Static geometry ignored by avoidance enumeration.
    Static,
}

/// Minimal rigid body record carried in the test scene.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicsBody {
    pub entity: Entity,
    pub position: Vec3,
    pub radius: f32,
    pub body: RigidBodyKind,
}

/// Enumerates neighbors for `query`, applying `FM-6` truncation deterministically.
#[must_use]
pub fn enumerate_avoidance(
    bodies: &[PhysicsBody],
    query: AvoidanceQuery,
    metrics: &mut FallbackMetrics,
) -> AvoidanceResult {
    let mut candidates: SmallVec<[(Entity, f32, NeighborState); 64]> = SmallVec::new();

    for body in bodies {
        if body.entity == query.self_entity {
            continue;
        }
        let RigidBodyKind::Dynamic { linvel } = body.body else {
            continue;
        };

        let delta = body.position - query.center;
        let dist = delta.length();
        if dist > query.radius {
            continue;
        }

        candidates.push((
            body.entity,
            dist,
            NeighborState {
                position: body.position,
                velocity: linvel,
                radius: body.radius,
                entity: body.entity,
            },
        ));
    }

    candidates.sort_by(|a, b| a.1.total_cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

    if candidates.len() > 16 {
        let excess = candidates.len() - 16;
        metrics.fm6_avoidance_truncation += excess as u64;
        candidates.truncate(16);
    }

    AvoidanceResult {
        neighbors: candidates.iter().map(|(_, _, n)| *n).collect(),
    }
}
