//! Shared group centroid, heading, and cohesion-style corrections.

use glam::Vec3;
use smallvec::SmallVec;

use super::formation::SteeringAgentId;

/// Identifier for a steering group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GroupId(pub u32);

/// Aggregated state for a small group of agents.
#[derive(Clone, Debug, PartialEq)]
pub struct GroupState {
    /// Average member position.
    pub centroid: Vec3,
    /// Average forward direction.
    pub avg_heading: Vec3,
    /// Scale for cohesion pull toward `centroid`.
    pub cohesion_strength: f32,
    /// Scale for alignment with `avg_heading`.
    pub alignment_strength: f32,
    /// Member entities (subset of the simulation).
    pub members: SmallVec<[SteeringAgentId; 16]>,
}

fn truncate(v: Vec3, max_len: f32) -> Vec3 {
    let len = v.length();
    if len > max_len && len > f32::EPSILON {
        v * (max_len / len)
    } else {
        v
    }
}

/// Recompute `centroid` and `avg_heading` from member transforms.
pub fn update_group_state(state: &mut GroupState, positions: &[(SteeringAgentId, Vec3, Vec3)]) {
    let mut c = Vec3::ZERO;
    let mut h = Vec3::ZERO;
    let mut matched = 0_u32;
    for &member in &state.members {
        if let Some((_, pos, heading)) = positions.iter().find(|(e, _, _)| *e == member) {
            c += *pos;
            h += *heading;
            matched += 1;
        }
    }
    if matched == 0 {
        return;
    }
    let count = matched as f32;
    state.centroid = c / count;
    state.avg_heading = (h / count).normalize_or_zero();
}

/// Cohesion + alignment correction blended toward `max_speed`.
pub fn group_corrections(
    position: Vec3,
    velocity: Vec3,
    state: &GroupState,
    max_speed: f32,
) -> Vec3 {
    let cohesion_dir = (state.centroid - position).normalize_or_zero();
    let alignment_dir = state.avg_heading;
    let correction =
        cohesion_dir * state.cohesion_strength + alignment_dir * state.alignment_strength;
    truncate(correction * max_speed - velocity, max_speed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_7_2_6_1_group_cohesion() {
        let mut members: SmallVec<[SteeringAgentId; 16]> = SmallVec::new();
        for i in 0..8 {
            members.push(SteeringAgentId(i));
        }
        let mut state = GroupState {
            centroid: Vec3::ZERO,
            avg_heading: Vec3::X,
            cohesion_strength: 0.4,
            alignment_strength: 0.1,
            members,
        };
        let mut positions: Vec<(SteeringAgentId, Vec3, Vec3)> = (0..8)
            .map(|i| {
                let a = i as f32 * 0.5;
                (
                    SteeringAgentId(i),
                    Vec3::new(a.cos(), 0.0, a.sin()) * 2.0,
                    Vec3::X,
                )
            })
            .collect();
        let initial_radius = group_radius(&positions);
        for _ in 0..200 {
            update_group_state(&mut state, &positions);
            for entry in &mut positions {
                let (e, p, h) = *entry;
                let corr = group_corrections(p, Vec3::ZERO, &state, 3.0);
                let new_p = p + corr * 0.05;
                *entry = (e, new_p, h);
            }
        }
        let final_radius = group_radius(&positions);
        assert!(final_radius <= initial_radius * 1.5 + 0.5);
    }

    fn group_radius(positions: &[(SteeringAgentId, Vec3, Vec3)]) -> f32 {
        let mut c = Vec3::ZERO;
        for (_, p, _) in positions {
            c += *p;
        }
        c /= positions.len() as f32;
        positions
            .iter()
            .map(|(_, p, _)| p.distance(c))
            .fold(0.0_f32, f32::max)
    }

    #[test]
    fn tc_7_2_6_2_group_velocity_convergence() {
        let mut members: SmallVec<[SteeringAgentId; 16]> = SmallVec::new();
        for i in 0..8 {
            members.push(SteeringAgentId(i));
        }
        let shared_dir = Vec3::X;
        let mut state = GroupState {
            centroid: Vec3::ZERO,
            avg_heading: shared_dir,
            cohesion_strength: 0.2,
            alignment_strength: 1.2,
            members,
        };
        let mut vels: Vec<Vec3> = (0..8)
            .map(|i| {
                let a = i as f32 * 1.7;
                Vec3::new(a.sin(), 0.0, a.cos()) * 2.0
            })
            .collect();
        let shared_speed = 3.0_f32;
        for _ in 0..800 {
            let mut positions: Vec<(SteeringAgentId, Vec3, Vec3)> = (0..8_usize)
                .map(|i| {
                    let ring = Vec3::new((i as f32) * 0.2, 0.0, 0.0);
                    (SteeringAgentId(i as u32), ring, vels[i].normalize_or_zero())
                })
                .collect();
            update_group_state(&mut state, &positions);
            state.avg_heading = shared_dir;
            for (i, slot) in positions.iter_mut().enumerate() {
                let p = slot.1;
                let corr = group_corrections(p, vels[i], &state, shared_speed);
                vels[i] = (vels[i] + corr * 0.1).clamp_length_max(shared_speed);
            }
        }
        let shared = shared_dir * shared_speed;
        for v in &vels {
            assert!(
                v.normalize_or_zero().dot(shared.normalize_or_zero()) > 0.9,
                "vel {v:?} not aligned with {shared:?}"
            );
        }
    }
}
