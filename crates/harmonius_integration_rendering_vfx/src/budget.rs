//! Global effect budget spawn scaling (IR-3.7.7 failure path).

use crate::types::EffectBudget;

/// Applies one budget tick when live particles cross the configured cap.
///
/// TC-IR-3.7.7.F1 — over-cap live counts ease `spawn_rate_scale` down; recovery eases it up.
pub fn tick_spawn_rate_scale(mut budget: EffectBudget, live_particles: u32) -> EffectBudget {
    if live_particles > budget.max_live_particles {
        budget.spawn_rate_scale = (budget.spawn_rate_scale * 0.85_f32).max(0.1);
    } else {
        budget.spawn_rate_scale = (budget.spawn_rate_scale + 0.05_f32).min(1.0);
    }
    budget
}
