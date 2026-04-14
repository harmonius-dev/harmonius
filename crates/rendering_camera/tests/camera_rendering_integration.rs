use std::cell::Cell;

use rendering_camera::{
    boom_tip, BokehShape, DofSettings, ExposureMode, ExposureSettings, HistogramSample,
    SphereCastQuery, SphereCastResult, SpringArm, SpringArmWorld,
};

struct ToggleObstructionWorld {
    frames: Cell<u32>,
}

impl SpringArmWorld for ToggleObstructionWorld {
    fn sphere_cast(&self, _query: SphereCastQuery) -> Option<SphereCastResult> {
        let frame = self.frames.get();
        self.frames.set(frame + 1);
        if frame < 90 {
            Some(SphereCastResult { distance: 2.2 })
        } else {
            None
        }
    }
}

/// TC-2.7.1.5 — deterministic scripted sphere cast; no physics engine in this crate yet.
#[test]
fn test_spring_arm_with_scripted_obstruction_world() {
    let world = ToggleObstructionWorld {
        frames: Cell::new(0),
    };
    let mut arm = SpringArm {
        desired_length: 3.0,
        current_length: 3.0,
        probe_radius: 0.12,
        retract_rate: 12.0,
        restore_rate: 4.0,
        layer_mask: 0xFFFF,
    };
    let forward = [1.0_f32, 0.0, 0.0];
    for _ in 0..240 {
        let _tip = boom_tip([0.0, 0.0, 0.0], forward, arm.current_length);
        let query = SphereCastQuery {
            origin: [0.0, 0.0, 0.0],
            direction: forward,
            radius: arm.probe_radius,
            layer_mask: arm.layer_mask,
        };
        let hit = world.sphere_cast(query).map(|h| h.distance);
        arm.step_toward_target(hit, 1.0 / 60.0, 0.05);
        assert!(
            arm.current_length > 0.4,
            "boom never collapses to the pivot"
        );
    }
    assert!((arm.current_length - arm.desired_length).abs() < 0.15);
}

/// TC-2.7.6.4 — pass gating: sane DoF settings validate before a GPU DoF pass exists.
#[test]
fn test_dof_pass_settings_validate_for_pass() {
    let settings = DofSettings {
        enabled: true,
        focus_distance_m: 2.5,
        focal_length_mm: 50.0,
        aperture_fstop: 2.0,
        bokeh_shape: BokehShape::Circle,
    };
    assert!(settings.validate_for_pass().is_ok());
}

/// TC-2.7.7.5 — brightness swings adapt EV across a scripted sequence.
#[test]
fn test_auto_exposure_adapts_over_sequence() {
    let s = ExposureSettings {
        mode: ExposureMode::AutomaticHistogram,
        manual_ev100: 0.0,
        ev_bias: 0.0,
        adaptation_speed: 3.0,
        min_ev: -6.0,
        max_ev: 6.0,
    };
    let mut ev = 0.0_f32;
    let bright = HistogramSample { mean_linear: 0.72 };
    let dark = HistogramSample { mean_linear: 0.045 };
    for _ in 0..90 {
        ev = s.adapt_histogram(ev, bright, 1.0 / 60.0);
    }
    let bright_ev = ev;
    for _ in 0..120 {
        ev = s.adapt_histogram(ev, dark, 1.0 / 60.0);
    }
    let dark_ev = ev;
    for _ in 0..120 {
        ev = s.adapt_histogram(ev, bright, 1.0 / 60.0);
    }
    let recovered_ev = ev;
    assert!(dark_ev < bright_ev);
    assert!(recovered_ev > dark_ev);
}
