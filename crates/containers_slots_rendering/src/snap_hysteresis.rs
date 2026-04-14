//! Snap preview radius hysteresis (IR-5.8.5, FM-4).

/// Returns whether a snap preview should be active after evaluating cursor distance to the target.
///
/// Uses Schmitt-style hysteresis: activation when `distance <= snap_radius`, deactivation once active
/// only when `distance > snap_radius * 1.1`.
#[must_use]
pub fn snap_preview_is_active(was_active: bool, distance: f32, snap_radius: f32) -> bool {
    if was_active {
        distance <= snap_radius * 1.1
    } else {
        distance <= snap_radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-5.8.5.U1 — activation at exactly `snap_radius` when inactive.
    #[test]
    fn tc_ir_5_8_5_u1_activate_at_radius_when_inactive() {
        let r = 2.0_f32;
        assert!(snap_preview_is_active(false, r, r));
    }

    /// TC-IR-5.8.5.U2 — hysteresis band while active; deactivation beyond `1.1 * r`.
    #[test]
    fn tc_ir_5_8_5_u2_hysteresis_deactivation_threshold() {
        let r = 2.0_f32;
        assert!(snap_preview_is_active(true, r * 1.05, r));
        assert!(!snap_preview_is_active(true, r * 1.11, r));
    }

    /// TC-IR-5.8.5.N1 — boundary band does not oscillate for a stable cursor.
    #[test]
    fn tc_ir_5_8_5_n1_stable_near_outer_band() {
        let r = 1.0_f32;
        let d = r * 1.02;
        // Begin inside the preview so `1.02r` sits in the hysteresis hold band (between `r` and
        // `1.1r`), then hold position — state must not flip when distance is constant.
        let mut active = true;
        for _ in 0..10 {
            active = snap_preview_is_active(active, d, r);
        }
        assert!(active);
    }
}
