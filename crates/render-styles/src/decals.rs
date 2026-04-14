//! Decal atlas and blending (`TC-11.2.*`).

/// G-buffer channels touched by a deferred decal (`TC-11.2.1.1`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GbufferSample {
    /// World normal x.
    pub nx: f32,
    /// World normal y.
    pub ny: f32,
    /// World normal z.
    pub nz: f32,
    /// Base color r.
    pub cr: f32,
    /// Base color g.
    pub cg: f32,
    /// Base color b.
    pub cb: f32,
}

/// Applies decal normal override inside axis bounds (`TC-11.2.1.1`).
pub fn apply_deferred_decal(
    base: GbufferSample,
    inside: bool,
    decal_normal: [f32; 3],
) -> GbufferSample {
    if !inside {
        return base;
    }
    GbufferSample {
        nx: decal_normal[0],
        ny: decal_normal[1],
        nz: decal_normal[2],
        cr: base.cr,
        cg: base.cg,
        cb: base.cb,
    }
}

/// Multiply blend for base color (`TC-11.2.1.1`).
pub fn multiply_blend(base: f32, factor: f32) -> f32 {
    base * factor
}

/// Transforms tangent-space decal normal with TBN (`TC-11.2.2.1`).
pub fn transform_decal_normal_ts(tbn: [[f32; 3]; 3], n_ts: [f32; 3]) -> [f32; 3] {
    let mut out = [0.0_f32; 3];
    for i in 0..3 {
        out[i] = tbn[i][0] * n_ts[0] + tbn[i][1] * n_ts[1] + tbn[i][2] * n_ts[2];
    }
    out
}

/// LRU decal atlas (`TC-11.2.3.1`).
#[derive(Debug, Clone)]
pub struct DecalAtlas {
    /// Slot ids from MRU front to LRU back.
    pub slots: Vec<u32>,
    /// Capacity.
    pub capacity: usize,
}

impl DecalAtlas {
    /// Creates atlas with fixed capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            slots: Vec::new(),
            capacity,
        }
    }

    /// Inserts decal id, evicting LRU if needed.
    pub fn insert(&mut self, id: u32) -> Option<u32> {
        self.slots.retain(|&x| x != id);
        self.slots.insert(0, id);
        if self.slots.len() > self.capacity {
            let evicted = self.slots.pop();
            return evicted;
        }
        None
    }

    /// Marks id as most-recently-used.
    pub fn touch(&mut self, id: u32) {
        self.slots.retain(|&x| x != id);
        self.slots.insert(0, id);
    }
}

/// Blends two decal layers by priority (`TC-11.2.4.1`).
pub fn decal_priority_color(low: f32, high: f32, low_pri: u8, high_pri: u8) -> f32 {
    if high_pri > low_pri {
        high
    } else {
        low
    }
}

/// Fade alpha over lifetime (`TC-11.2.4.1`).
pub fn decal_fade_alpha(t: f32, lifetime: f32) -> f32 {
    if t >= lifetime {
        0.0
    } else {
        1.0 - t / lifetime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_11_2_1_1_deferred_decal_gbuffer_modification() {
        let base = GbufferSample {
            nx: 0.0,
            ny: 1.0,
            nz: 0.0,
            cr: 0.5,
            cg: 0.5,
            cb: 0.5,
        };
        let out = apply_deferred_decal(base, true, [0.0, 0.0, 1.0]);
        assert!((out.nz - 1.0).abs() < 1e-4);
        let out2 = apply_deferred_decal(base, false, [0.0, 0.0, 1.0]);
        assert_eq!(out2.ny, base.ny);
        assert!((multiply_blend(0.5, 0.5) - 0.25).abs() < 1e-4);
    }

    #[test]
    fn tc_11_2_2_1_mesh_decal_tangent_space_normal() {
        let tbn = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let n = transform_decal_normal_ts(tbn, [0.0, 0.0, 1.0]);
        assert!((n[2] - 1.0).abs() < 1e-4);
    }

    #[test]
    fn tc_11_2_3_1_decal_atlas_lru_eviction() {
        let mut atlas = DecalAtlas::new(256);
        for i in 0..256 {
            atlas.insert(i);
        }
        let evicted = atlas.insert(256).expect("evict");
        assert_eq!(evicted, 0);
        atlas.touch(0);
        let evicted2 = atlas.insert(257).expect("evict2");
        assert_eq!(evicted2, 1);
    }

    #[test]
    fn tc_11_2_4_1_decal_priority_layering() {
        let c = decal_priority_color(0.2, 0.8, 1, 5);
        assert!((c - 0.8).abs() < 1e-4);
        assert!((decal_fade_alpha(2.0, 2.0) - 0.0).abs() < 1e-4);
        assert!(decal_fade_alpha(1.0, 2.0) > 0.0);
    }
}
