//! Weather, puddles, snow deformation, lightning, wind-driven debris, and underwater helpers.

use super::budget::PlatformTier;

/// High-level weather kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeatherKind {
    /// Clear skies.
    Clear,
    /// Rain precipitation.
    Rain,
}

/// Runtime weather sample used for subsystem activation tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeatherState {
    /// Current kind.
    pub kind: WeatherKind,
    /// Intensity in \[0,1\].
    pub intensity: f32,
    /// Transition progress in \[0,1\] when lerping kinds.
    pub transition_progress: f32,
}

/// Advances a linear transition from `from` to `to` over `duration_secs` (**TC-11.4.1.1**).
pub fn step_weather_transition(
    mut state: WeatherState,
    target: WeatherKind,
    dt: f32,
    duration_secs: f32,
) -> WeatherState {
    if state.kind != target {
        state.transition_progress += dt / duration_secs.max(f32::EPSILON);
        if state.transition_progress >= 1.0 {
            state.kind = target;
            state.transition_progress = 1.0;
        }
        if target == WeatherKind::Rain {
            state.intensity = state.transition_progress.clamp(0.0, 1.0);
        }
    }
    state
}

/// Simple puddle depth bucket in arbitrary units.
#[derive(Clone, Copy, Debug, Default)]
pub struct PuddleSim {
    /// Current depth.
    pub depth: f32,
}

impl PuddleSim {
    /// Accumulates while `raining` at `rain_intensity`, drains otherwise (**TC-11.4.2.1**).
    pub fn step(&mut self, raining: bool, rain_intensity: f32, dt: f32) {
        if raining {
            self.depth += 0.2 * rain_intensity * dt;
        } else {
            self.depth = (self.depth - 0.15 * dt).max(0.0);
        }
    }
}

/// Surface material for wetness response (**TC-11.4.2.2**).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceMaterial {
    /// Stone surface.
    Stone,
    /// Metal surface.
    Metal,
    /// Wood surface.
    Wood,
}

/// Computes roughness override from wetness (**TC-11.4.2.2**).
pub fn wet_roughness_override(material: SurfaceMaterial, wetness: f32) -> f32 {
    let w = wetness.clamp(0.0, 1.0);
    match material {
        SurfaceMaterial::Stone => 1.0 - 0.8 * w,
        SurfaceMaterial::Metal => 1.0 - 0.95 * w,
        SurfaceMaterial::Wood => 1.0 - 0.7 * w,
    }
}

/// Snow trail deformation scalar that fades over time (**TC-11.4.3.1**).
pub fn snow_deformation_after_stamp(initial: f32, seconds: f32, fade_per_sec: f32) -> f32 {
    initial * (-fade_per_sec * seconds).exp()
}

/// Maximum lightning branch depth for a tier (**TC-11.4.5.1**).
pub fn lightning_max_branch_depth(tier: PlatformTier) -> u32 {
    match tier {
        PlatformTier::Mobile => 2,
        PlatformTier::Desktop => 4,
    }
}

/// Deterministic initial horizontal velocity for particle `index` (**TC-11.4.6.1**).
pub fn debris_initial_velocity_x(wind: [f32; 3], index: u32) -> f32 {
    let jitter = (((index.wrapping_mul(1103515245)) % 1000) as f32 / 1000.0) - 0.5;
    wind[0] + jitter
}

/// Applies simple exponential velocity damping (**TC-11.4.6.1** #2).
pub fn damp_velocity(mut v: [f32; 3], drag: f32, dt: f32) -> [f32; 3] {
    let k = (-drag * dt).exp();
    v[0] *= k;
    v[1] *= k;
    v[2] *= k;
    v
}

/// Underwater caustic scroll offset (**TC-11.4.7.1** #1).
pub fn caustic_scroll_offset(scroll_speed: f32, dt: f32, mut offset: f32) -> f32 {
    offset += scroll_speed * dt;
    offset
}

/// Depth fog attenuation along view distance (**TC-11.4.7.1** #2).
pub fn underwater_depth_fog_blue(distance_m: f32) -> f32 {
    (-0.1_f32 * distance_m).exp()
}

#[cfg(test)]
mod tests {
    use super::{
        PlatformTier, PuddleSim, SurfaceMaterial, WeatherKind, WeatherState,
        caustic_scroll_offset, debris_initial_velocity_x, damp_velocity,
        lightning_max_branch_depth, snow_deformation_after_stamp, step_weather_transition,
        underwater_depth_fog_blue, wet_roughness_override,
    };

    /// **TC-11.4.1.1** — clear to rain ramps intensity.
    #[test]
    fn tc_11_4_1_1_weather_state_transition() {
        let start = WeatherState {
            kind: WeatherKind::Clear,
            intensity: 0.0,
            transition_progress: 0.0,
        };
        let mid = step_weather_transition(start, WeatherKind::Rain, 0.5, 1.0);
        assert!(mid.intensity > 0.0);
        let end = step_weather_transition(mid, WeatherKind::Rain, 0.75, 1.0);
        assert_eq!(end.kind, WeatherKind::Rain);
        assert!((end.intensity - 1.0).abs() < 1e-3);
    }

    /// **TC-11.4.2.1** — accumulate during rain, drain after rain stops.
    #[test]
    fn tc_11_4_2_1_puddle_accumulate_drain() {
        let mut p = PuddleSim::default();
        for _ in 0..50 {
            p.step(true, 1.0, 0.1);
        }
        let peak = p.depth;
        assert!(peak > 0.0);
        for _ in 0..200 {
            p.step(false, 0.0, 0.1);
        }
        assert!(p.depth < 1e-3);
    }

    /// **TC-11.4.2.2** — wetness drives roughness overrides.
    #[test]
    fn tc_11_4_2_2_wet_surface_material() {
        assert!((wet_roughness_override(SurfaceMaterial::Stone, 1.0) - 0.2).abs() < 1e-5);
        assert!((wet_roughness_override(SurfaceMaterial::Metal, 1.0) - 0.05).abs() < 1e-5);
    }

    /// **TC-11.4.3.1** — deformation fades toward zero while snow continues.
    #[test]
    fn tc_11_4_3_1_snow_deformation_fade() {
        let d = snow_deformation_after_stamp(1.0, 10.0, 0.35);
        assert!(d < 0.05);
    }

    /// **TC-11.4.5.1** — branch depth caps per tier.
    #[test]
    fn tc_11_4_5_1_lightning_branch_depth() {
        assert_eq!(lightning_max_branch_depth(PlatformTier::Mobile), 2);
        assert_eq!(lightning_max_branch_depth(PlatformTier::Desktop), 4);
    }

    /// **TC-11.4.6.1** — wind sets initial vx band; drag damps toward zero.
    #[test]
    fn tc_11_4_6_1_wind_debris_velocity_from_wind_field() {
        let wind = [5.0_f32, 0.0, 0.0];
        for i in 0..100 {
            let vx = debris_initial_velocity_x(wind, i);
            assert!(vx >= 4.5 && vx <= 5.5);
        }
        let v = [10.0_f32, 0.0, 0.0];
        let v2 = damp_velocity(v, 2.0, 1.0);
        assert!(v2[0].abs() < 2.0);
    }

    /// **TC-11.4.7.1** — caustic scroll and depth fog attenuation.
    #[test]
    fn tc_11_4_7_1_underwater_caustic_pattern_update() {
        let off = caustic_scroll_offset(1.7, 1.0, 0.0);
        assert!((off - 1.7).abs() < 1e-5);
        let fog = underwater_depth_fog_blue(20.0);
        let expected = (-0.1_f32 * 20.0_f32).exp();
        assert!((fog - expected).abs() / expected < 0.01);
    }
}
