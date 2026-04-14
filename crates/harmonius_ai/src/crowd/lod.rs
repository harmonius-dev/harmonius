//! Distance-based AI LOD tier assignment with a global high-LOD cap.

use glam::Vec3;

/// Simulation fidelity bucket for an AI agent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AiLodTier {
    /// Full steering and perception.
    High,
    /// Reduced-rate update cadence (not modeled numerically here).
    Mid,
    /// Flow-field-driven cheap pathing.
    Low,
}

/// Designer-tunable LOD radii and caps.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiLodConfig {
    /// Distance threshold for high LOD (m).
    pub high_radius: f32,
    /// Outer distance threshold for mid LOD (m).
    pub mid_radius: f32,
    /// Max agents allowed at high LOD for this archetype.
    pub high_max_agents: u32,
    /// Mid-LOD agents run every N simulation ticks (`2` = every other tick).
    pub mid_tick_divisor: u8,
    /// When true, always assign high LOD.
    pub force_high_lod: bool,
}

/// Global AI microseconds budget (design `AiBudget`); tier fields reserved for future schedulers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AiBudget {
    /// Total AI microseconds per frame.
    pub total_us: u32,
    /// Budget allocated to high-LOD agents.
    pub high_lod_us: u32,
    /// Budget allocated to mid-LOD agents.
    pub mid_lod_us: u32,
    /// Budget allocated to low-LOD agents.
    pub low_lod_us: u32,
    /// Budget consumed this frame (reset by `assign_lod_tiers`).
    pub used_us: u32,
}

/// Agent inputs for `assign_lod_tiers`.
#[derive(Clone, Debug, PartialEq)]
pub struct LodAgent {
    /// Agent world position.
    pub position: Vec3,
    /// LOD tuning for this agent.
    pub config: AiLodConfig,
    /// Tier written by `assign_lod_tiers`.
    pub tier: AiLodTier,
}

/// Assign [`AiLodTier`] using nearest-player distance, optional global high cap, and force-high.
pub fn assign_lod_tiers(
    agents: &mut [LodAgent],
    player_positions: &[Vec3],
    budget: &mut AiBudget,
    global_high_lod_cap: Option<u32>,
) {
    let mut high_count: u32 = 0;
    let global_cap = global_high_lod_cap.unwrap_or(u32::MAX);

    for agent in agents.iter_mut() {
        if agent.config.force_high_lod {
            agent.tier = AiLodTier::High;
            high_count += 1;
            continue;
        }

        let min_dist = player_positions
            .iter()
            .map(|p| p.distance(agent.position))
            .fold(f32::MAX, f32::min);

        let archetype_cap = agent.config.high_max_agents.min(global_cap);

        agent.tier = if min_dist <= agent.config.high_radius && high_count < archetype_cap {
            high_count += 1;
            AiLodTier::High
        } else if min_dist <= agent.config.mid_radius {
            AiLodTier::Mid
        } else {
            AiLodTier::Low
        };
    }

    budget.used_us = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_7_7_5_1_lod_assignment() {
        let cam = Vec3::ZERO;
        let mut a = LodAgent {
            position: Vec3::new(10.0, 0.0, 0.0),
            config: AiLodConfig {
                high_radius: 20.0,
                mid_radius: 40.0,
                high_max_agents: 100,
                mid_tick_divisor: 2,
                force_high_lod: false,
            },
            tier: AiLodTier::Low,
        };
        assign_lod_tiers(
            std::slice::from_mut(&mut a),
            &[cam],
            &mut AiBudget::default(),
            None,
        );
        assert_eq!(a.tier, AiLodTier::High);

        let mut b = LodAgent {
            position: Vec3::new(50.0, 0.0, 0.0),
            config: AiLodConfig {
                high_radius: 20.0,
                mid_radius: 40.0,
                high_max_agents: 100,
                mid_tick_divisor: 2,
                force_high_lod: false,
            },
            tier: AiLodTier::High,
        };
        assign_lod_tiers(
            std::slice::from_mut(&mut b),
            &[cam],
            &mut AiBudget::default(),
            None,
        );
        assert_eq!(b.tier, AiLodTier::Low);

        let mut c = LodAgent {
            position: Vec3::new(30.0, 0.0, 0.0),
            config: AiLodConfig {
                high_radius: 20.0,
                mid_radius: 40.0,
                high_max_agents: 100,
                mid_tick_divisor: 2,
                force_high_lod: false,
            },
            tier: AiLodTier::Low,
        };
        assign_lod_tiers(
            std::slice::from_mut(&mut c),
            &[cam],
            &mut AiBudget::default(),
            None,
        );
        assert_eq!(c.tier, AiLodTier::Mid);
    }

    #[test]
    fn tc_7_7_5_2_lod_force_high() {
        let mut a = LodAgent {
            position: Vec3::new(100.0, 0.0, 0.0),
            config: AiLodConfig {
                high_radius: 20.0,
                mid_radius: 40.0,
                high_max_agents: 10,
                mid_tick_divisor: 2,
                force_high_lod: true,
            },
            tier: AiLodTier::Low,
        };
        assign_lod_tiers(
            std::slice::from_mut(&mut a),
            &[Vec3::ZERO],
            &mut AiBudget::default(),
            None,
        );
        assert_eq!(a.tier, AiLodTier::High);
    }

    #[test]
    fn tc_7_7_5_3_lod_budget_cap() {
        let cam = Vec3::ZERO;
        let mut agents: Vec<LodAgent> = (0..100)
            .map(|i| LodAgent {
                position: Vec3::new(i as f32 * 0.05, 0.0, 0.0),
                config: AiLodConfig {
                    high_radius: 100.0,
                    mid_radius: 200.0,
                    high_max_agents: 1000,
                    mid_tick_divisor: 2,
                    force_high_lod: false,
                },
                tier: AiLodTier::Low,
            })
            .collect();
        assign_lod_tiers(&mut agents, &[cam], &mut AiBudget::default(), Some(50));
        let highs = agents.iter().filter(|a| a.tier == AiLodTier::High).count();
        assert_eq!(highs, 50);
    }
}
