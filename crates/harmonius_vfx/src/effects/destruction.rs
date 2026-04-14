//! Destruction-adjacent color, growth, ordering, shockwave, and fire spread helpers.

use super::environment::SurfaceMaterial;

/// RGB color in linear space.
pub type Rgb = [f32; 3];

/// Dust tint for a destroyed surface (**TC-11.5.2.1**).
pub fn dust_color_for_material(material: SurfaceMaterial) -> Rgb {
    match material {
        SurfaceMaterial::Stone => [0.6, 0.6, 0.6],
        SurfaceMaterial::Metal => [0.6, 0.6, 0.6],
        SurfaceMaterial::Wood => [0.5, 0.35, 0.2],
    }
}

/// Spark color along normalized lifetime `t` in \[0,1\] (**TC-11.5.3.1**).
pub fn spark_color(t: f32) -> Rgb {
    let t = t.clamp(0.0, 1.0);
    if t <= 0.5 {
        let u = t / 0.5;
        [1.0, 0.5 + 0.5 * u, 0.0]
    } else {
        let u = (t - 0.5) / 0.5;
        let c = 0.2 * (1.0 - u);
        [c, 0.1 * (1.0 - u), 0.0]
    }
}

/// Crack radius growth for one timestep (**TC-11.5.4.1**).
pub fn crack_radius_delta(accumulated_damage: f32, growth_speed: f32, dt: f32) -> f32 {
    (accumulated_damage / 100.0) * growth_speed * dt
}

/// Priority ordering: scorch above transient decals (**TC-11.5.5.1**).
pub fn scorch_renders_above_transient(scorch_priority: u8, transient_priority: u8) -> bool {
    scorch_priority > transient_priority
}

/// Shockwave radius after elapsed time (**TC-11.5.6.1**).
pub fn shockwave_radius(speed: f32, elapsed_secs: f32) -> f32 {
    speed * elapsed_secs
}

/// Fuel cell for simple fire spread tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuelCell {
    /// Stone blocks spread.
    Stone,
    /// Unburned wood.
    Wood,
    /// Burning wood.
    Burning,
}

/// 1D fire spread: wood adjacent to burning becomes burning; stone unchanged (**TC-11.5.7.1**).
pub fn fire_spread_step_1d(cells: &[FuelCell]) -> Vec<FuelCell> {
    let n = cells.len();
    let mut out = cells.to_vec();
    for i in 0..n {
        if cells[i] != FuelCell::Wood {
            continue;
        }
        let left = i > 0 && cells[i - 1] == FuelCell::Burning;
        let right = i + 1 < n && cells[i + 1] == FuelCell::Burning;
        if left || right {
            out[i] = FuelCell::Burning;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        FuelCell, crack_radius_delta, dust_color_for_material, fire_spread_step_1d,
        scorch_renders_above_transient, shockwave_radius, spark_color,
    };
    use crate::effects::environment::SurfaceMaterial;

    /// **TC-11.5.2.1** — dust color follows destroyed material.
    #[test]
    fn tc_11_5_2_1_dust_color_by_material() {
        let stone = dust_color_for_material(SurfaceMaterial::Stone);
        assert!((stone[0] - 0.6).abs() < 1e-5);
        let wood = dust_color_for_material(SurfaceMaterial::Wood);
        assert!((wood[0] - 0.5).abs() < 1e-5);
        assert!((wood[1] - 0.35).abs() < 1e-5);
        assert!((wood[2] - 0.2).abs() < 1e-5);
    }

    /// **TC-11.5.3.1** — spark color fades along lifetime.
    #[test]
    fn tc_11_5_3_1_spark_color_fade() {
        let a = spark_color(0.0);
        assert!((a[0] - 1.0).abs() < 1e-5);
        let b = spark_color(0.5);
        assert!(b[0] > 0.9 && b[1] > 0.4);
        let c = spark_color(1.0);
        assert!(c[0] < 0.3 && c[1] < 0.2);
    }

    /// **TC-11.5.4.1** — crack growth scales with damage and dt.
    #[test]
    fn tc_11_5_4_1_crack_growth_rate() {
        let d = crack_radius_delta(50.0, 1.0, 1.0);
        assert!((d - 0.5).abs() < 1e-5);
    }

    /// **TC-11.5.5.1** — scorch draws above transient decals.
    #[test]
    fn tc_11_5_5_1_scorch_persistence() {
        assert!(scorch_renders_above_transient(6, 2));
    }

    /// **TC-11.5.6.1** — shockwave radius grows with speed * time.
    #[test]
    fn tc_11_5_6_1_shockwave_expansion() {
        let r = shockwave_radius(50.0, 1.0);
        assert!((r - 50.0).abs() < 1e-3);
    }

    /// **TC-11.5.7.1** — fire spreads along wood but never converts stone.
    #[test]
    fn tc_11_5_7_1_fire_material_blocking() {
        let cells = [
            FuelCell::Burning,
            FuelCell::Wood,
            FuelCell::Stone,
            FuelCell::Wood,
        ];
        let after = fire_spread_step_1d(&cells);
        assert_eq!(after[2], FuelCell::Stone);
        assert_eq!(after[1], FuelCell::Burning);
        assert_eq!(after[3], FuelCell::Wood);
    }
}
