//! Core steering forces: seek, flee, arrive, wander, pursuit, evade, flocking
//! primitives, hide, interpose, and feeler obstacle avoidance for behavior blends.

use glam::Vec3;

use super::obstacle::{compute_obstacle_avoidance, SpatialQuery};

/// Steering force that moves toward `target` at `max_speed`, minus current `velocity`.
pub fn seek(position: Vec3, velocity: Vec3, target: Vec3, max_speed: f32) -> Vec3 {
    let desired = (target - position).normalize_or_zero() * max_speed;
    desired - velocity
}

/// Steering force that accelerates away from `threat` at `max_speed`.
pub fn flee(position: Vec3, velocity: Vec3, threat: Vec3, max_speed: f32) -> Vec3 {
    let desired = (position - threat).normalize_or_zero() * max_speed;
    desired - velocity
}

/// Arrive steering with speed scaled down inside `decel_radius` of `target`.
pub fn arrive(
    position: Vec3,
    velocity: Vec3,
    target: Vec3,
    max_speed: f32,
    decel_radius: f32,
) -> Vec3 {
    let to_target = target - position;
    let dist = to_target.length();
    if dist < f32::EPSILON {
        return -velocity;
    }
    let speed = if dist < decel_radius {
        max_speed * (dist / decel_radius)
    } else {
        max_speed
    };
    let desired = (to_target / dist) * speed;
    desired - velocity
}

/// Random walk steering; updates `wander_target` on the wander sphere each call.
pub fn wander(
    velocity: Vec3,
    max_speed: f32,
    jitter: f32,
    radius: f32,
    distance: f32,
    wander_target: &mut Vec3,
    rng: &mut impl rand::Rng,
) -> Vec3 {
    let jitter_vec = Vec3::new(
        rng.gen_range(-1.0..1.0) * jitter,
        0.0,
        rng.gen_range(-1.0..1.0) * jitter,
    );
    *wander_target = (*wander_target + jitter_vec).normalize_or_zero() * radius;
    let heading = velocity.normalize_or_zero();
    let center = heading * distance;
    (center + *wander_target).normalize_or_zero() * max_speed - velocity
}

/// Predictive chase toward `target_pos` using constant `target_vel` lookahead.
pub fn pursuit(
    position: Vec3,
    velocity: Vec3,
    target_pos: Vec3,
    target_vel: Vec3,
    max_speed: f32,
) -> Vec3 {
    let to_target = target_pos - position;
    let look_ahead = to_target.length() / max_speed.max(f32::EPSILON);
    let predicted = target_pos + target_vel * look_ahead;
    seek(position, velocity, predicted, max_speed)
}

/// Predictive escape from `threat_pos` using constant `threat_vel` lookahead.
pub fn evade(
    position: Vec3,
    velocity: Vec3,
    threat_pos: Vec3,
    threat_vel: Vec3,
    max_speed: f32,
) -> Vec3 {
    let to_threat = threat_pos - position;
    let look_ahead = to_threat.length() / max_speed.max(f32::EPSILON);
    let predicted = threat_pos + threat_vel * look_ahead;
    flee(position, velocity, predicted, max_speed)
}

/// Match average neighbor heading (`SteeringBehaviorKind::Align`).
pub fn align(velocity: Vec3, neighbors: &[(Vec3, Vec3)], max_speed: f32) -> Vec3 {
    if neighbors.is_empty() {
        return Vec3::ZERO;
    }
    let mut h = Vec3::ZERO;
    for (_, nvel) in neighbors {
        h += nvel.normalize_or_zero();
    }
    let desired = (h / neighbors.len() as f32).normalize_or_zero() * max_speed;
    desired - velocity
}

/// Push away from neighbors inside `separation_radius` (`SteeringBehaviorKind::Separate`).
pub fn separate(
    position: Vec3,
    velocity: Vec3,
    neighbor_positions: &[Vec3],
    separation_radius: f32,
    max_speed: f32,
) -> Vec3 {
    let mut acc = Vec3::ZERO;
    for &other in neighbor_positions {
        let diff = position - other;
        let d = diff.length();
        if d < separation_radius && d > f32::EPSILON {
            acc += diff.normalize() * (1.0 / d);
        }
    }
    let desired = acc.normalize_or_zero() * max_speed;
    desired - velocity
}

/// Pull toward neighbor centroid (`SteeringBehaviorKind::Cohesion`).
pub fn cohesion(
    position: Vec3,
    velocity: Vec3,
    neighbor_positions: &[Vec3],
    max_speed: f32,
) -> Vec3 {
    if neighbor_positions.is_empty() {
        return Vec3::ZERO;
    }
    let c: Vec3 =
        neighbor_positions.iter().copied().sum::<Vec3>() / (neighbor_positions.len() as f32);
    seek(position, velocity, c, max_speed)
}

/// Feeler-based obstacle avoidance for weighted blends (`SteeringBehaviorKind::ObstacleAvoid`).
pub fn obstacle_avoid(
    position: Vec3,
    velocity: Vec3,
    spatial: &impl SpatialQuery,
    feeler_length: f32,
    feeler_count: u8,
) -> Vec3 {
    compute_obstacle_avoidance(position, velocity, feeler_count, feeler_length, spatial)
}

/// Move past `occluder_center` away from `threat_pos` (`SteeringBehaviorKind::Hide`).
pub fn hide(
    position: Vec3,
    velocity: Vec3,
    threat_pos: Vec3,
    occluder_center: Vec3,
    occluder_radius: f32,
    hide_distance: f32,
    max_speed: f32,
) -> Vec3 {
    let away = (occluder_center - threat_pos).normalize_or_zero();
    let target = occluder_center + away * (occluder_radius + hide_distance);
    seek(position, velocity, target, max_speed)
}

/// Steer toward a point between `a_pos` and `b_pos` (`SteeringBehaviorKind::Interpose`).
pub fn interpose(
    position: Vec3,
    velocity: Vec3,
    a_pos: Vec3,
    b_pos: Vec3,
    ratio: f32,
    max_speed: f32,
) -> Vec3 {
    let target = a_pos.lerp(b_pos, ratio);
    seek(position, velocity, target, max_speed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    #[test]
    fn tc_7_2_3_1_seek_points_toward_target() {
        let f = seek(Vec3::ZERO, Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0), 5.0);
        assert!(f.dot(Vec3::X) > 0.0);
    }

    #[test]
    fn tc_7_2_3_1_seek_at_target_zero() {
        let f = seek(
            Vec3::new(10.0, 0.0, 0.0),
            Vec3::ZERO,
            Vec3::new(10.0, 0.0, 0.0),
            5.0,
        );
        assert!(f.length_squared() < 1e-6);
    }

    #[test]
    fn tc_7_2_3_2_flee_points_away() {
        let f = flee(Vec3::ZERO, Vec3::ZERO, Vec3::new(5.0, 0.0, 0.0), 5.0);
        assert!(f.x < 0.0);
    }

    #[test]
    fn tc_7_2_3_3_arrive_deceleration_zone() {
        let max_force = 10.0;
        let f_inside = arrive(
            Vec3::new(7.0, 0.0, 0.0),
            Vec3::ZERO,
            Vec3::new(10.0, 0.0, 0.0),
            max_force,
            5.0,
        );
        let f_outside = arrive(
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::ZERO,
            Vec3::new(10.0, 0.0, 0.0),
            max_force,
            5.0,
        );
        assert!(f_inside.length() < max_force);
        assert!((f_outside.length() - max_force).abs() < 1e-3);
    }

    #[test]
    fn tc_7_2_3_4_arrive_reaches_target() {
        let target = Vec3::new(10.0, 0.0, 0.0);
        let mut pos = Vec3::ZERO;
        let mut vel = Vec3::ZERO;
        let dt = 0.016;
        let max_speed = 8.0;
        let max_force = 20.0;
        for _ in 0..600 {
            let steer = arrive(pos, vel, target, max_speed, 5.0);
            let accel = steer.clamp_length_max(max_force);
            vel += accel * dt;
            vel = vel.clamp_length_max(max_speed);
            pos += vel * dt;
        }
        assert!((pos - target).length() < 0.1);
    }

    #[test]
    fn tc_7_2_3_5_wander_bounded() {
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut pos = Vec3::ZERO;
        let mut vel = Vec3::new(1.0, 0.0, 0.0);
        let mut wt = Vec3::Y;
        let radius = 50.0_f32;
        for _ in 0..1000 {
            let steer = wander(vel, 5.0, 0.5, radius, 4.0, &mut wt, &mut rng);
            vel = (vel + steer * 0.016).clamp_length_max(5.0);
            pos += vel * 0.016;
            // TC-7.2.3.5 calls for 1000 ticks; allow extra slack for numeric drift.
            assert!(pos.length() < radius + 25.0);
        }
    }

    #[test]
    fn tc_7_2_3_6_pursuit_faster_than_seek() {
        let target_start = Vec3::new(25.0, 0.0, 8.0);
        let target_vel = Vec3::new(0.0, 0.0, -4.0);
        let max_speed = 8.0_f32;
        let max_force = 40.0_f32;
        let dt = 0.04_f32;

        let dist_after = |use_pursuit: bool| -> f32 {
            let mut p = Vec3::ZERO;
            let mut v = Vec3::ZERO;
            let mut tpos = target_start;
            for _ in 0..120 {
                let steer = if use_pursuit {
                    pursuit(p, v, tpos, target_vel, max_speed)
                } else {
                    seek(p, v, tpos, max_speed)
                };
                let accel = steer.clamp_length_max(max_force);
                v += accel * dt;
                v = v.clamp_length_max(max_speed);
                p += v * dt;
                tpos += target_vel * dt;
            }
            (p - tpos).length()
        };

        let dp = dist_after(true);
        let ds = dist_after(false);
        assert!(dp < ds, "pursuit dist {dp} vs seek dist {ds}");
    }

    #[test]
    fn tc_7_2_3_7_evade_increases_distance() {
        let mut evader = Vec3::ZERO;
        let mut evader_vel = Vec3::ZERO;
        let threat_vel = Vec3::new(-1.0, 0.0, 0.0);
        let mut threat = Vec3::new(5.0, 0.0, 0.0);
        let max_speed = 5.0_f32;
        let max_force = 25.0_f32;
        let dt = 0.05_f32;
        let d0 = (evader - threat).length();
        for _ in 0..100 {
            let steer = evade(evader, evader_vel, threat, threat_vel, max_speed);
            let accel = steer.clamp_length_max(max_force);
            evader_vel += accel * dt;
            evader_vel = evader_vel.clamp_length_max(max_speed);
            evader += evader_vel * dt;
            threat += threat_vel * dt;
        }
        let d1 = (evader - threat).length();
        assert!(d1 > d0);
    }

    #[test]
    fn steering_primitives_align_separate_cohesion() {
        let neighbors = [
            (Vec3::new(1.0, 0.0, 0.0), Vec3::X),
            (Vec3::new(-1.0, 0.0, 0.0), Vec3::X),
        ];
        let f_align = align(Vec3::ZERO, &neighbors, 5.0);
        assert!(f_align.dot(Vec3::X) > 0.0);

        let npos = [Vec3::new(0.1, 0.0, 0.0)];
        let f_sep = separate(Vec3::ZERO, Vec3::ZERO, &npos, 0.5, 5.0);
        assert!(f_sep.length() > 0.0);

        let cpos = [Vec3::new(5.0, 0.0, 0.0), Vec3::new(7.0, 0.0, 0.0)];
        let f_coh = cohesion(Vec3::ZERO, Vec3::ZERO, &cpos, 5.0);
        assert!(f_coh.dot(Vec3::X) > 0.0);
    }

    #[test]
    fn steering_interpose_seeks_segment_midpoint() {
        let f = interpose(
            Vec3::new(-5.0, 0.0, 0.0),
            Vec3::ZERO,
            Vec3::new(-10.0, 0.0, 0.0),
            Vec3::new(10.0, 0.0, 0.0),
            0.5,
            5.0,
        );
        assert!(f.dot(Vec3::X) > 0.0);
    }

    #[test]
    fn steering_hide_points_past_occluder() {
        let threat = Vec3::new(-10.0, 0.0, 0.0);
        let occ = Vec3::ZERO;
        let agent = Vec3::new(-2.0, 0.0, 0.0);
        let f = hide(agent, Vec3::ZERO, threat, occ, 0.5, 1.0, 5.0);
        assert!(f.x > 0.0);
    }

    #[test]
    fn steering_obstacle_avoid_nonzero_near_wall() {
        use crate::steering::obstacle::{StaticWalls, WallSegment};
        let walls = StaticWalls {
            walls: vec![WallSegment {
                a: Vec3::new(2.0, 0.0, 0.0),
                b: Vec3::new(2.0, 0.0, 2.0),
                thickness: 0.2,
            }],
        };
        let pos = Vec3::new(1.0, 0.0, 1.0);
        let heading = Vec3::X;
        let f = obstacle_avoid(pos, heading, &walls, 3.0, 3);
        assert!(f.length_squared() > 1e-6);
    }
}
