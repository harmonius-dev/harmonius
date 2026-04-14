//! Optimal Reciprocal Collision Avoidance (ORCA) in the XZ plane (`glam::Vec2`).
//!
//! Each `solve_orca` call allocates neighbor agent storage; callers should reuse buffers in hot
//! paths once ECS integration lands.

use std::borrow::Cow;

use dodgy_2d::{Agent as DodgyAgent, AvoidanceOptions, Vec2 as DodgyVec2};
use glam::Vec2;

fn to_dodgy(v: Vec2) -> DodgyVec2 {
    DodgyVec2::new(v.x, v.y)
}

fn from_dodgy(v: DodgyVec2) -> Vec2 {
    Vec2::new(v.x, v.y)
}

/// Fixed timestep used internally for dodgy avoidance (seconds).
const ORCA_DT: f32 = 1.0 / 60.0;

/// Compute an ORCA-safe planar velocity close to `preferred_vel`.
pub fn solve_orca(
    agent_pos: Vec2,
    agent_vel: Vec2,
    agent_radius: f32,
    preferred_vel: Vec2,
    neighbors: &[(Vec2, Vec2, f32)],
    time_horizon: f32,
    max_speed: f32,
) -> Vec2 {
    let agent = DodgyAgent {
        position: to_dodgy(agent_pos),
        velocity: to_dodgy(agent_vel),
        radius: agent_radius,
        avoidance_responsibility: 0.5,
    };

    let neighbour_agents: Vec<Cow<'_, DodgyAgent>> = neighbors
        .iter()
        .map(|&(p, v, r)| {
            Cow::Owned(DodgyAgent {
                position: to_dodgy(p),
                velocity: to_dodgy(v),
                radius: r,
                avoidance_responsibility: 0.5,
            })
        })
        .collect();

    let opts = AvoidanceOptions {
        obstacle_margin: 0.0,
        time_horizon,
        obstacle_time_horizon: time_horizon,
    };

    let out = agent.compute_avoiding_velocity(
        &neighbour_agents,
        &[],
        to_dodgy(preferred_vel),
        max_speed,
        ORCA_DT,
        &opts,
    );
    from_dodgy(out).clamp_length_max(max_speed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_7_2_1_1_orca_no_overlap() {
        let radius = 0.3_f32;
        let mut pos: Vec<Vec2> = Vec::new();
        let mut vel: Vec<Vec2> = Vec::new();
        for i in 0..200 {
            let gx = (i % 20) as f32;
            let gz = (i / 20) as f32;
            pos.push(Vec2::new(gx * 0.5, gz * 0.5));
            vel.push(Vec2::new(0.01, 0.02));
        }
        let th = 2.0_f32;
        let max_speed = 1.5_f32;
        for _ in 0..100 {
            let snapshot_pos = pos.clone();
            let snapshot_vel = vel.clone();
            for i in 0..pos.len() {
                let mut nbs = Vec::new();
                for j in 0..pos.len() {
                    if i == j {
                        continue;
                    }
                    let d = snapshot_pos[i].distance(snapshot_pos[j]);
                    if d < 2.5 * radius {
                        nbs.push((snapshot_pos[j], snapshot_vel[j], radius));
                    }
                }
                let pref = snapshot_vel[i];
                vel[i] = solve_orca(
                    snapshot_pos[i],
                    snapshot_vel[i],
                    radius,
                    pref,
                    &nbs,
                    th,
                    max_speed,
                );
            }
            for i in 0..pos.len() {
                pos[i] += vel[i] * 0.05;
            }
        }
        for i in 0..pos.len() {
            for j in i + 1..pos.len() {
                let d = pos[i].distance(pos[j]);
                assert!(d >= 2.0 * radius - 0.02, "pair {i},{j} dist {d}");
            }
        }
    }

    #[test]
    fn tc_7_2_1_2_orca_passage() {
        let radius = 0.25_f32;
        let mut pos: Vec<Vec2> = Vec::new();
        let mut vel: Vec<Vec2> = Vec::new();
        let goal_left = Vec2::new(-5.0, 0.0);
        let goal_right = Vec2::new(5.0, 0.0);
        for i in 0..10 {
            pos.push(Vec2::new(-3.0, -0.4 + i as f32 * 0.08));
            vel.push(Vec2::new(-0.8, 0.0));
        }
        for i in 0..10 {
            pos.push(Vec2::new(3.0, -0.4 + i as f32 * 0.08));
            vel.push(Vec2::new(0.8, 0.0));
        }
        let th = 2.5_f32;
        let max_speed = 1.2_f32;
        for tick in 0..2500 {
            let snapshot_pos = pos.clone();
            let snapshot_vel = vel.clone();
            for i in 0..pos.len() {
                let goal = if i < 10 { goal_left } else { goal_right };
                let to_goal = (goal - snapshot_pos[i]).normalize_or_zero() * max_speed;
                let mut nbs = Vec::new();
                for j in 0..pos.len() {
                    if i == j {
                        continue;
                    }
                    if snapshot_pos[i].distance(snapshot_pos[j]) < 4.0 {
                        nbs.push((snapshot_pos[j], snapshot_vel[j], radius));
                    }
                }
                vel[i] = solve_orca(
                    snapshot_pos[i],
                    snapshot_vel[i],
                    radius,
                    to_goal,
                    &nbs,
                    th,
                    max_speed,
                );
            }
            let mut reached = 0_usize;
            for i in 0..pos.len() {
                pos[i] += vel[i] * 0.05;
                let goal = if i < 10 { goal_left } else { goal_right };
                if pos[i].distance(goal) < 1.2 {
                    reached += 1;
                }
            }
            if tick == 2499 {
                assert_eq!(reached, 20, "not all agents reached goals");
            }
        }
    }
}
