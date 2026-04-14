//! Ragdoll blend weight scheduling and partial ragdoll masking.

use crate::math::Vec3;

/// Linear blend-in/out controller for ragdoll vs animation mix.
#[derive(Clone, Debug, Default)]
pub struct RagdollBlendController {
    /// Current blend toward ragdoll (1 = full ragdoll).
    pub blend_weight: f32,
    /// Seconds accumulated while blending in.
    pub blend_in_elapsed: f32,
    /// Seconds accumulated while recovering.
    pub recovery_elapsed: f32,
    /// Whether ragdoll influence is engaged.
    pub active: bool,
    /// Whether the controller is blending back to animation.
    pub recovering: bool,
}

impl RagdollBlendController {
    /// Advances time; when `active` and not `recovering`, ramps `blend_weight` to 1 over
    /// `blend_in_duration`. When `recovering`, ramps to 0 over `recovery_duration`.
    pub fn tick(
        &mut self,
        dt: f32,
        blend_in_duration: f32,
        recovery_duration: f32,
        just_activated: bool,
        just_recover: bool,
    ) {
        if just_activated {
            self.active = true;
            self.recovering = false;
            self.blend_in_elapsed = 0.0;
        }
        if just_recover {
            self.recovering = true;
            self.recovery_elapsed = 0.0;
        }
        if self.recovering {
            self.recovery_elapsed += dt;
            if recovery_duration <= 0.0 {
                self.blend_weight = 0.0;
                self.active = false;
                self.recovering = false;
            } else {
                self.blend_weight = 1.0 - (self.recovery_elapsed / recovery_duration).min(1.0);
                if self.blend_weight <= 0.0 {
                    self.active = false;
                    self.recovering = false;
                }
            }
        } else if self.active {
            self.blend_in_elapsed += dt;
            if blend_in_duration <= 0.0 {
                self.blend_weight = 1.0;
            } else {
                self.blend_weight = (self.blend_in_elapsed / blend_in_duration).min(1.0);
            }
        }
    }
}

/// Applies per-bone mask: `true` means ragdoll applies; `false` keeps animation pose.
pub fn blend_poses(anim: &[Vec3], rag: &[Vec3], mask: &[bool], global_weight: f32) -> Vec<Vec3> {
    anim.iter()
        .zip(rag.iter())
        .zip(mask.iter())
        .map(|((a, r), m)| {
            let w = if *m { global_weight } else { 0.0 };
            Vec3::lerp(*a, *r, w)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math;

    #[test]
    fn tc_9_3_4_1_blend_in() {
        let dt = 1.0 / 60.0;
        let mut c = RagdollBlendController::default();
        c.tick(dt, 0.5, 1.0, true, false);
        for _ in 0..29 {
            c.tick(dt, 0.5, 1.0, false, false);
        }
        assert!((c.blend_weight - 1.0).abs() < 1e-3);
        let mut c2 = RagdollBlendController::default();
        c2.tick(dt, 0.0, 1.0, true, false);
        assert!((c2.blend_weight - 1.0).abs() < 1e-3);
    }

    #[test]
    fn tc_9_3_4_2_recovery() {
        let dt = 1.0 / 60.0;
        let mut c = RagdollBlendController {
            blend_weight: 1.0,
            active: true,
            ..Default::default()
        };
        c.tick(dt, 0.5, 1.0, false, true);
        for _ in 0..59 {
            c.tick(dt, 0.5, 1.0, false, false);
        }
        assert!(c.blend_weight <= 1e-3);
        let mut c2 = RagdollBlendController {
            blend_weight: 1.0,
            active: true,
            ..Default::default()
        };
        c2.tick(dt, 0.5, 0.3, false, true);
        for _ in 0..17 {
            c2.tick(dt, 0.5, 0.3, false, false);
        }
        assert!(c2.blend_weight <= 1e-2);
    }

    #[test]
    fn tc_9_3_4_3_partial_mask() {
        let anim = vec![
            Vec3::ZERO,
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
        ];
        let rag = vec![
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(1.0, 2.0, 0.0),
        ];
        let mask = vec![false, true, true];
        let out = blend_poses(&anim, &rag, &mask, 1.0);
        assert!(math::approx_eq_vec3(out[0], anim[0], 1e-4));
        assert!(math::approx_eq_vec3(out[1], rag[1], 1e-4));
        assert!(math::approx_eq_vec3(out[2], rag[2], 1e-4));
    }

    #[test]
    fn tc_9_3_4_4_no_discontinuity() {
        let dt = 1.0 / 60.0;
        let mut c = RagdollBlendController::default();
        c.tick(dt, 0.5, 1.0, true, false);
        let mut prev = 0.0_f32;
        let mut max_delta = 0.0_f32;
        for _ in 0..60 {
            c.tick(dt, 0.5, 1.0, false, false);
            max_delta = max_delta.max((c.blend_weight - prev).abs());
            prev = c.blend_weight;
        }
        assert!(max_delta < 0.1);
    }
}
