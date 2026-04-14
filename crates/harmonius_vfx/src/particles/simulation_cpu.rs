//! Deterministic CPU stepping for individual simulation modules (`TC-11.1.2.*` subset).

use glam::{Vec3, Vec4};

/// Applies euler gravity integration: `velocity + gravity * dt`.
#[must_use]
pub fn apply_gravity(velocity: Vec3, gravity: Vec3, dt: f32) -> Vec3 {
    velocity + gravity * dt
}

/// Applies linear drag: `velocity * (1.0 - drag * dt)` (clamped so velocity does not flip sign
/// unexpectedly for extreme coefficients).
#[must_use]
pub fn apply_drag(velocity: Vec3, drag: f32, dt: f32) -> Vec3 {
    let factor = (1.0 - drag * dt).max(0.0);
    velocity * factor
}

/// Analytical divergence-free field used for CPU-side curl-noise stand-in (`TC-11.1.2.2`).
#[must_use]
pub fn curl_noise_velocity_sample(position: Vec3, amplitude: f32) -> Vec3 {
    amplitude * Vec3::new(position.y.cos(), -position.x.cos(), 0.0)
}

/// Central-difference divergence estimate for a vector field at `p` (should be ~0 for
/// [`curl_noise_velocity_sample`]).
#[must_use]
pub fn divergence_estimate_3d(field: impl Fn(Vec3) -> Vec3, p: Vec3, h: f32) -> f32 {
    let hx = Vec3::new(h, 0.0, 0.0);
    let hy = Vec3::new(0.0, h, 0.0);
    let hz = Vec3::new(0.0, 0.0, h);
    let dfx = (field(p + hx) - field(p - hx)).x / (2.0 * h);
    let dfy = (field(p + hy) - field(p - hy)).y / (2.0 * h);
    let dfz = (field(p + hz) - field(p - hz)).z / (2.0 * h);
    dfx + dfy + dfz
}

/// Linear RGBA gradient between `a` (t=0) and `b` (t=1).
#[must_use]
pub fn sample_color_over_life_linear(a: Vec4, b: Vec4, age: f32, lifetime: f32) -> Vec4 {
    let t = if lifetime > 0.0 {
        (age / lifetime).clamp(0.0, 1.0)
    } else {
        0.0
    };
    a.lerp(b, t)
}

/// Linear size curve between start and end over normalized lifetime.
#[must_use]
pub fn sample_size_over_life(start: f32, end: f32, age: f32, lifetime: f32) -> f32 {
    let t = if lifetime > 0.0 {
        (age / lifetime).clamp(0.0, 1.0)
    } else {
        0.0
    };
    start + (end - start) * t
}

/// Reflects `velocity` about `plane_normal` (unit) and scales by `restitution`.
#[must_use]
pub fn depth_buffer_bounce_velocity(velocity: Vec3, plane_normal: Vec3, restitution: f32) -> Vec3 {
    let n = plane_normal.normalize_or_zero();
    let vn = velocity.dot(n);
    if vn >= 0.0 {
        return velocity;
    }
    velocity - (1.0 + restitution) * vn * n
}

/// If `sdf_enabled` is false (mobile), returns `velocity` unchanged (`TC-11.1.2.7`).
#[must_use]
pub fn sdf_collision_velocity(
    velocity: Vec3,
    sdf_gradient: Vec3,
    restitution: f32,
    sdf_enabled: bool,
) -> Vec3 {
    if !sdf_enabled {
        return velocity;
    }
    let n = sdf_gradient.normalize_or_zero();
    let vn = velocity.dot(n);
    if vn >= 0.0 {
        return velocity;
    }
    velocity - (1.0 + restitution) * vn * n
}

/// Descriptor for fused simulation: module count does not change dispatch count (`TC-11.1.2.8`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FusedSimDescriptor {
    /// Number of fused modules in the shader.
    pub module_count: usize,
    /// Always one fused compute dispatch per emitter in this design.
    pub dispatch_count: u32,
}

impl FusedSimDescriptor {
    /// Builds a descriptor for `module_count` fused modules.
    #[must_use]
    pub fn new(module_count: usize) -> Self {
        Self {
            module_count,
            dispatch_count: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.2.1` — gravity module sample.
    #[test]
    fn tc_11_1_2_1_gravity_module() {
        let v = Vec3::ZERO;
        let g = Vec3::new(0.0, -9.81, 0.0);
        let dt = 1.0 / 60.0;
        let out = apply_gravity(v, g, dt);
        let expected = -0.1635;
        assert!(
            (out.y - expected).abs() < 1e-4,
            "got {} expected {}",
            out.y,
            expected
        );
        assert_eq!(out.x, 0.0);
        assert_eq!(out.z, 0.0);
    }

    /// `TC-11.1.2.3` — drag module sample.
    #[test]
    fn tc_11_1_2_3_drag_module() {
        let v = Vec3::splat(10.0);
        let out = apply_drag(v, 0.5, 1.0);
        assert_approx_vec3(out, Vec3::splat(5.0), 1e-5);
    }

    fn assert_approx_vec3(a: Vec3, b: Vec3, eps: f32) {
        assert!(
            (a - b).length() <= eps,
            "expected {a:?} ~= {b:?} (eps {eps})"
        );
    }

    /// `TC-11.1.2.2` — curl-like field displaces velocity and is near divergence-free.
    #[test]
    fn tc_11_1_2_2_curl_noise_module() {
        let p = Vec3::new(0.2, -0.4, 0.1);
        let v0 = Vec3::ZERO;
        let dv = curl_noise_velocity_sample(p, 1.0);
        assert!(dv.length() > 1e-3);
        let v1 = v0 + dv;
        assert_ne!(v0, v1);
        let div = divergence_estimate_3d(|q| curl_noise_velocity_sample(q, 1.0), p, 1e-3);
        assert!(div.abs() < 5e-2, "divergence {div}");
    }

    /// `TC-11.1.2.4` — color over life at t=0.5 between red and blue.
    #[test]
    fn tc_11_1_2_4_color_over_life() {
        let red = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let blue = Vec4::new(0.0, 0.0, 1.0, 1.0);
        let c = sample_color_over_life_linear(red, blue, 0.5, 1.0);
        assert!((c.x - 0.5).abs() < 1e-4 && (c.z - 0.5).abs() < 1e-4);
    }

    /// `TC-11.1.2.5` — size curve start 1 end 0 at birth and death.
    #[test]
    fn tc_11_1_2_5_size_over_life() {
        assert!((sample_size_over_life(1.0, 0.0, 0.0, 1.0) - 1.0).abs() < 1e-5);
        assert!((sample_size_over_life(1.0, 0.0, 1.0, 1.0) - 0.0).abs() < 1e-5);
    }

    /// `TC-11.1.2.6` — depth buffer collision reflects with restitution 0.5.
    #[test]
    fn tc_11_1_2_6_depth_buffer_collision() {
        let v = Vec3::new(0.0, -1.0, 0.0);
        let n = Vec3::Y;
        let out = depth_buffer_bounce_velocity(v, n, 0.5);
        assert_approx_vec3(out, Vec3::new(0.0, 0.5, 0.0), 1e-4);
    }

    /// `TC-11.1.2.7` — SDF collision disabled on mobile-style flag.
    #[test]
    fn tc_11_1_2_7_sdf_collision_platform() {
        let v = Vec3::new(0.0, -1.0, 0.0);
        let g = Vec3::Y;
        let pass_through = sdf_collision_velocity(v, g, 0.5, false);
        assert_eq!(pass_through, v);
        let bounce = sdf_collision_velocity(v, g, 0.5, true);
        assert!(bounce.y > 0.0);
    }

    /// `TC-11.1.2.8` — multiple modules still one dispatch.
    #[test]
    fn tc_11_1_2_8_fused_module_dispatch() {
        let d = FusedSimDescriptor::new(3);
        assert_eq!(d.module_count, 3);
        assert_eq!(d.dispatch_count, 1);
    }
}
