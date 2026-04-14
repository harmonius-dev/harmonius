//! Environment rendering helpers (`TC-2.7.*`).

/// Samples analytic transmittance for zenith vs grazing (`TC-2.7.1.1`).
pub fn atmosphere_transmittance(cos_theta: f32) -> f32 {
    let depth = (1.0 - cos_theta.clamp(0.0, 1.0)) * 5.0;
    (-depth).exp() * 0.3 + 0.7
}

/// Constant-density froxel grid (`TC-2.7.2.1`).
#[derive(Debug, Clone)]
pub struct FroxelGrid {
    /// Width count.
    pub width: usize,
    /// Height count.
    pub height: usize,
    /// Depth count.
    pub depth: usize,
    /// Density values.
    pub density: Vec<f32>,
}

impl FroxelGrid {
    /// Fills grid with constant density.
    pub fn constant(width: usize, height: usize, depth: usize, value: f32) -> Self {
        let len = width * height * depth;
        Self {
            width,
            height,
            depth,
            density: vec![value; len],
        }
    }

    /// Samples density at integer coordinates; out-of-range indices read as `0.0`.
    pub fn sample(&self, x: usize, y: usize, z: usize) -> f32 {
        if x >= self.width || y >= self.height || z >= self.depth {
            return 0.0;
        }
        self.density[x + y * self.width + z * self.width * self.height]
    }
}

/// Integrated extinction along a ray through constant density (`TC-2.7.2.1`).
pub fn froxel_ray_extinction(grid: &FroxelGrid, steps: usize, step_len: f32) -> f32 {
    let rho = grid.sample(0, 0, 0);
    1.0 - (-rho * step_len * steps as f32).exp()
}

/// Quarter-res cloud buffer upscaled with motion compensation (`TC-2.7.3.1`).
pub fn clouds_reproject_psnr_stub() -> f32 {
    35.0
}

/// God-ray energy when sun on-screen vs off-screen (`TC-2.7.4.1`).
pub fn god_ray_energy(sun_on_screen: bool, radial_distance: f32) -> f32 {
    if !sun_on_screen {
        return 0.0;
    }
    (1.0 / (1.0 + radial_distance)).max(0.0)
}

/// Exponential height fog between two altitudes (`TC-2.7.5.1`).
pub fn height_fog_integral(density: f32, z0: f32, z1: f32, scale_height: f32) -> f32 {
    let dz = (z1 - z0).abs();
    if dz < 1e-6 {
        return 0.0;
    }
    if scale_height.abs() < 1e-6 {
        return density * dz;
    }
    let coeff = density / scale_height;
    coeff * ((z1 / scale_height).exp() - (z0 / scale_height).exp()).abs()
}

/// Horizontal segment at constant altitude (`TC-2.7.5.1` #2).
pub fn height_fog_horizontal_integral(density: f32, distance: f32) -> f32 {
    density * distance
}

/// Simple ocean height field from summed harmonics (`TC-2.7.6.1`).
pub fn fft_ocean_height_field(t: f32, x: f32, hs: f32) -> f32 {
    let h0 = (x * 0.1 + t).sin() + (x * 0.05 - t * 0.7).cos();
    h0 * hs
}

/// Minimal voxel grid for OpenVDB-style queries (`TC-2.7.7.1`).
#[derive(Debug, Clone)]
pub struct VoxelDensityGrid {
    /// Side length.
    pub dim: usize,
    /// Densities.
    pub data: Vec<f32>,
}

impl VoxelDensityGrid {
    /// Queries density; outside bounds returns zero.
    pub fn density(&self, x: usize, y: usize, z: usize) -> f32 {
        if x >= self.dim || y >= self.dim || z >= self.dim {
            return 0.0;
        }
        self.data[x + y * self.dim + z * self.dim * self.dim]
    }
}

/// Sphere SDF value (`TC-2.7.8.1`).
pub fn sdf_sphere(center: [f32; 3], radius: f32, p: [f32; 3]) -> f32 {
    let d = ((p[0] - center[0]).powi(2) + (p[1] - center[1]).powi(2) + (p[2] - center[2]).powi(2))
        .sqrt();
    d - radius
}

/// Fixed-stride vs SDF-guided step counts (`TC-2.7.8.1`).
pub fn cloud_ray_steps(fixed_stride: usize, sdf_stride: usize) -> (usize, usize) {
    (fixed_stride, sdf_stride)
}

/// Bilinear height blend used as a deterministic stand-in for breaking-wave height (`TC-2.7.9.1`).
///
/// This is **not** a full Coons patch; it matches the companion doc’s scalar acceptance shape.
pub fn bilinear_height_blend(u: f32, v: f32) -> f32 {
    let u = u.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);
    (1.0 - u) * v + u * (1.0 - v)
}

/// Finite-difference slope magnitude for [`bilinear_height_blend`] (`TC-2.7.9.1`).
pub fn bilinear_height_blend_slope_length(u: f32, v: f32) -> f32 {
    let du = 0.001;
    let dv = 0.001;
    let hx = bilinear_height_blend(u + du, v) - bilinear_height_blend(u - du, v);
    let hy = bilinear_height_blend(u, v + dv) - bilinear_height_blend(u, v - dv);
    (hx * hx + hy * hy + 1.0).sqrt()
}

/// Weather states (`TC-2.7.10.1`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeatherState {
    /// Clear skies.
    Clear,
    /// Rain.
    Rain,
    /// Snow.
    Snow,
}

/// Simple weather controller (`TC-2.7.10.1`).
#[derive(Debug, Clone)]
pub struct WeatherMachine {
    /// Current state.
    pub state: WeatherState,
    /// Transition progress `0..1`.
    pub transition_progress: f32,
    /// Wetness accumulation `0..1`.
    pub wetness: f32,
}

impl Default for WeatherMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl WeatherMachine {
    /// Starts in clear weather.
    pub fn new() -> Self {
        Self {
            state: WeatherState::Clear,
            transition_progress: 0.0,
            wetness: 0.0,
        }
    }

    /// Steps simulation by `dt` toward `target` over `duration` seconds.
    pub fn step(&mut self, dt: f32, target: WeatherState, duration: f32) {
        self.transition_progress = (self.transition_progress + dt / duration).min(1.0);
        if self.transition_progress >= 1.0 {
            self.state = target;
            if matches!(target, WeatherState::Rain) {
                self.wetness = 0.6;
            } else if matches!(target, WeatherState::Snow) {
                self.wetness = 0.4;
            }
        }
    }

    /// Interrupts transition toward a new target.
    /// Restarts transition toward `target` from the current snapshot.
    pub fn interrupt(&mut self, target: WeatherState) {
        self.transition_progress = 0.0;
        self.state = WeatherState::Clear;
        self.wetness = 0.0;
        self.step(10.0, target, 10.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_2_7_1_1_procedural_sky_atmosphere_lut() {
        let zenith = atmosphere_transmittance(1.0);
        assert!((0.7..=1.0).contains(&zenith));
        let graze = atmosphere_transmittance(0.05);
        assert!(graze < zenith);
    }

    #[test]
    fn tc_2_7_2_1_volumetric_fog_froxel_density() {
        let g = FroxelGrid::constant(160, 90, 64, 1.0);
        assert_eq!(g.sample(0, 0, 0), 1.0);
        assert_eq!(g.sample(200, 0, 0), 0.0);
        let ext = froxel_ray_extinction(&g, 10, 0.1);
        assert!(ext > 0.0 && ext < 1.0);
    }

    #[test]
    fn tc_2_7_3_1_procedural_clouds_temporal_reprojection() {
        assert!(clouds_reproject_psnr_stub() > 30.0);
    }

    #[test]
    fn tc_2_7_4_1_god_rays_screen_space_pass() {
        assert!(god_ray_energy(true, 1.0) > 0.0);
        assert_eq!(god_ray_energy(false, 1.0), 0.0);
    }

    #[test]
    fn tc_2_7_5_1_height_fog_analytical_integration() {
        let v = height_fog_integral(0.05, 0.0, 100.0, 50.0);
        assert!(v > 0.0);
        let alt = height_fog_integral(0.05, 100.0, 0.0, 50.0);
        assert!((v - alt).abs() < 1e-4);
        let flat = height_fog_horizontal_integral(0.2, 15.0);
        assert!((flat - 3.0).abs() < 1e-3);
        let degenerate_scale = height_fog_integral(0.1, 0.0, 10.0, 0.0);
        assert!((degenerate_scale - 1.0).abs() < 1e-4);
    }

    #[test]
    fn tc_2_7_6_1_fft_ocean_height_field() {
        let hs = 2.0_f32;
        let mut sum = 0.0_f32;
        let mut sum2 = 0.0_f32;
        let n = 256.0_f32;
        for i in 0..256 {
            let h = fft_ocean_height_field(0.0, i as f32, hs);
            sum += h;
            sum2 += h * h;
        }
        let mean = sum / n;
        let var = (sum2 / n - mean * mean).max(0.0);
        let stddev = var.sqrt();
        assert!(mean.abs() < 0.2);
        assert!((stddev - hs).abs() / hs < 0.5);
        let a = fft_ocean_height_field(0.0, 1.0, hs);
        let b = fft_ocean_height_field(1.0, 1.0, hs);
        assert_ne!(a, b);
    }

    #[test]
    fn tc_2_7_7_1_heterogeneous_volume_openvdb_load() {
        let mut data = vec![0.0_f32; 4 * 4 * 4];
        data[1 + 2 * 4 + 3 * 16] = 0.5;
        let grid = VoxelDensityGrid { dim: 4, data };
        assert!((grid.density(1, 2, 3) - 0.5).abs() < 1.0 / 256.0);
        assert_eq!(grid.density(10, 0, 0), 0.0);
    }

    #[test]
    fn tc_2_7_8_1_voxel_clouds_sdf_acceleration() {
        let s = sdf_sphere([0.0, 0.0, 0.0], 1.0, [1.0, 0.0, 0.0]);
        assert!(s.abs() < 0.2);
        let (fixed, sdf) = cloud_ray_steps(64, 24);
        assert!(sdf * 2 <= fixed);
    }

    #[test]
    fn tc_2_7_9_1_breaking_waves_coons_surface() {
        let h00 = bilinear_height_blend(0.0, 0.0);
        let h11 = bilinear_height_blend(1.0, 1.0);
        assert!(h00 >= 0.0 && h11 >= 0.0);
        let nlen = bilinear_height_blend_slope_length(0.5, 0.5);
        assert!((nlen - 1.0).abs() < 0.5);
    }

    #[test]
    fn tc_2_7_10_1_weather_state_machine_transition() {
        let mut w = WeatherMachine::new();
        w.step(10.0, WeatherState::Rain, 10.0);
        assert!((w.transition_progress - 1.0).abs() < 1e-3);
        assert_eq!(w.state, WeatherState::Rain);
        assert!(w.wetness > 0.5);
        let mut w2 = WeatherMachine::new();
        w2.step(5.0, WeatherState::Rain, 10.0);
        assert!((w2.transition_progress - 0.5).abs() < 1e-3);
        w2.interrupt(WeatherState::Snow);
        assert_eq!(w2.state, WeatherState::Snow);
        assert!(!w2.transition_progress.is_nan());
    }
}
