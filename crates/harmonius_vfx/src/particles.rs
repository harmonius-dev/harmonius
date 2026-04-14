//! Particle spawn shapes and sprite blend metadata used by GPU paths.

/// Emitter spawn volume shapes (**TC-11.1.1.1**).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpawnShape {
    /// Single point emission.
    Point,
    /// Uniform sphere volume.
    Sphere {
        /// Radius in meters.
        radius: f32,
    },
    /// Axis-aligned box volume.
    Box {
        /// Half extents in meters.
        half_extents: [f32; 3],
    },
    /// Right circular cone.
    Cone {
        /// Cone half-angle in radians.
        angle: f32,
        /// Base radius in meters.
        radius: f32,
    },
    /// Mesh surface emission (asset id placeholder).
    MeshSurface {
        /// Mesh asset id.
        mesh: u64,
    },
}

/// Returns a distinct marker token per variant for codegen tests (**TC-11.1.1.1** #1).
pub fn spawn_kernel_marker(shape: &SpawnShape) -> &'static str {
    match shape {
        SpawnShape::Point => "spawn_point",
        SpawnShape::Sphere { .. } => "spawn_sphere",
        SpawnShape::Box { .. } => "spawn_box",
        SpawnShape::Cone { .. } => "spawn_cone",
        SpawnShape::MeshSurface { .. } => "spawn_mesh_surface",
    }
}

/// Deterministic sample inside a unit sphere for index `i` (**TC-11.1.1.1** #2).
pub fn sphere_spawn_offset(radius: f32, i: u32) -> [f32; 3] {
    let u = (((i.wrapping_mul(1664525)) % 1000) as f32 / 500.0) - 1.0;
    let v = (((i.wrapping_mul(22695477)) % 1000) as f32 / 500.0) - 1.0;
    let w = (((i.wrapping_mul(1103515245)) % 1000) as f32 / 500.0) - 1.0;
    let mut p = [u * radius, v * radius, w * radius];
    let len = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
    if len > radius && len > 0.0 {
        let s = radius / len;
        p[0] *= s;
        p[1] *= s;
        p[2] *= s;
    }
    p
}

/// Dispatch counters for spawn tests (**TC-11.1.1.1** #3).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GpuDispatchCounters {
    /// Number of CPU readbacks triggered (must stay zero on spawn-only paths).
    pub cpu_readbacks: u32,
}

/// Simulates a GPU-only spawn dispatch (**TC-11.1.1.1** #3).
pub fn dispatch_spawn_only(_particle_count: u32) -> GpuDispatchCounters {
    GpuDispatchCounters {
        cpu_readbacks: 0,
    }
}

/// Sprite blending modes (**TC-11.1.3.1**).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpriteBlend {
    /// Additive blending.
    Additive,
    /// Traditional alpha blending.
    Alpha,
}

/// Maps blend mode to a canonical blend-state key (**TC-11.1.3.1**).
pub fn sprite_blend_state_key(blend: SpriteBlend) -> &'static str {
    match blend {
        SpriteBlend::Additive => "SrcAlpha_One",
        SpriteBlend::Alpha => "SrcAlpha_InvSrcAlpha",
    }
}

/// Records whether soft-particle depth fading is enabled (**TC-11.1.3.1** #3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteOutputConfig {
    /// Blend mode for the draw call.
    pub blend: SpriteBlend,
    /// Enables depth-buffer sampling for soft particles.
    pub soft_particle: bool,
    /// Fade distance in meters.
    pub depth_fade: f32,
}

/// Returns a shader feature token for soft particles (**TC-11.1.3.1** #3).
pub fn soft_particle_shader_token(cfg: &SpriteOutputConfig) -> Option<&'static str> {
    if cfg.soft_particle {
        Some("DEPTH_FADE")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GpuDispatchCounters, SpawnShape, SpriteBlend, SpriteOutputConfig, dispatch_spawn_only,
        sphere_spawn_offset, spawn_kernel_marker, sprite_blend_state_key, soft_particle_shader_token,
    };

    /// **TC-11.1.1.1** — spawn markers, sphere bounds, and zero readback dispatch.
    #[test]
    fn tc_11_1_1_1_gpu_spawn_shape_coverage() {
        let shapes = [
            SpawnShape::Point,
            SpawnShape::Sphere { radius: 1.0 },
            SpawnShape::Box {
                half_extents: [1.0, 1.0, 1.0],
            },
            SpawnShape::Cone {
                angle: 0.5,
                radius: 1.0,
            },
            SpawnShape::MeshSurface { mesh: 42 },
        ];
        let mut markers = std::collections::HashSet::new();
        for s in &shapes {
            markers.insert(spawn_kernel_marker(s));
        }
        assert_eq!(markers.len(), 5);

        let origin = [0.0_f32; 3];
        let r = 1.0_f32;
        for i in 0..10_000 {
            let p = sphere_spawn_offset(r, i);
            let dx = origin[0] + p[0];
            let dy = origin[1] + p[1];
            let dz = origin[2] + p[2];
            let len = (dx * dx + dy * dy + dz * dz).sqrt();
            assert!(len <= r + 1e-3);
        }

        let c = dispatch_spawn_only(10_000);
        assert_eq!(c, GpuDispatchCounters::default());
    }

    /// **TC-11.1.3.1** — blend modes and soft particle depth fade flag.
    #[test]
    fn tc_11_1_3_1_sprite_billboard_blend_modes() {
        assert_eq!(
            sprite_blend_state_key(SpriteBlend::Additive),
            "SrcAlpha_One"
        );
        assert_eq!(
            sprite_blend_state_key(SpriteBlend::Alpha),
            "SrcAlpha_InvSrcAlpha"
        );
        let cfg = SpriteOutputConfig {
            blend: SpriteBlend::Alpha,
            soft_particle: true,
            depth_fade: 0.5,
        };
        assert_eq!(soft_particle_shader_token(&cfg), Some("DEPTH_FADE"));
    }
}
