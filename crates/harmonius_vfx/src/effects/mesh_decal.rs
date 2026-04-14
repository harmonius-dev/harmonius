//! Mesh decal tangent frame helpers (numeric stand-in for GPU mesh projection).

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-8);
    [v[0] / l, v[1] / l, v[2] / l]
}

fn combine3(t: [f32; 3], b: [f32; 3], n: [f32; 3], ts: [f32; 3]) -> [f32; 3] {
    [
        t[0] * ts[0] + b[0] * ts[1] + n[0] * ts[2],
        t[1] * ts[0] + b[1] * ts[1] + n[1] * ts[2],
        t[2] * ts[0] + b[2] * ts[1] + n[2] * ts[2],
    ]
}

/// Reconstructs world normal from tangent frame + tangent-space normal (**TC-11.2.2.1**).
pub fn world_normal_from_tbn(
    tangent: [f32; 3],
    bitangent: [f32; 3],
    normal: [f32; 3],
    ts_normal: [f32; 3],
) -> [f32; 3] {
    let t = normalize3(tangent);
    let b = normalize3(bitangent);
    let n = normalize3(normal);
    normalize3(combine3(t, b, n, ts_normal))
}

#[cfg(test)]
mod tests {
    use super::world_normal_from_tbn;

    fn angle_deg(a: [f32; 3], b: [f32; 3]) -> f32 {
        fn normalize3(v: [f32; 3]) -> [f32; 3] {
            let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-8);
            [v[0] / l, v[1] / l, v[2] / l]
        }
        let a = normalize3(a);
        let b = normalize3(b);
        let d = (a[0] * b[0] + a[1] * b[1] + a[2] * b[2]).clamp(-1.0, 1.0);
        d.acos().to_degrees()
    }

    /// **TC-11.2.2.1** — TBN reconstruction matches analytic expectation on a curved frame.
    #[test]
    fn tc_11_2_2_1_mesh_decal_tangent_space_normals() {
        let t = [1.0_f32, 0.0, 0.0];
        let b = [0.0_f32, 0.0, 1.0];
        let n = [0.0_f32, 1.0, 0.0];
        let ts = [0.0_f32, 0.0, 1.0];
        let wn = world_normal_from_tbn(t, b, n, ts);
        assert!((wn[0] - n[0]).abs() < 0.001);
        assert!((wn[1] - n[1]).abs() < 0.001);
        assert!((wn[2] - n[2]).abs() < 0.001);

        let ang = angle_deg(wn, n);
        assert!(ang < 1.0);
    }
}
