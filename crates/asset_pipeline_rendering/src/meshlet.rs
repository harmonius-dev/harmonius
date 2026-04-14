//! Meshlet sizing helpers (IR-5.2.5).

/// Maximum vertices per meshlet (`meshoptimizer` cluster sizing).
pub const MAX_MESHLET_VERTICES: u32 = 64;

/// Maximum triangles per meshlet (design + `meshoptimizer` cap).
pub const MAX_MESHLET_TRIANGLES: u32 = 124;

/// Returns the number of meshlet clusters required for the given geometry counts.
///
/// This is the conservative upper bound `max(ceil(V/64), ceil(T/124))` from the integration design.
#[must_use]
pub fn meshlet_cluster_count(vertex_count: u32, triangle_count: u32) -> u32 {
    let v_clusters = vertex_count.div_ceil(MAX_MESHLET_VERTICES);
    let t_clusters = triangle_count.div_ceil(MAX_MESHLET_TRIANGLES);
    v_clusters.max(t_clusters)
}

/// Half-angle of a tight normal cone for a perfectly flat surface (all normals parallel).
#[must_use]
pub fn normal_cone_half_angle_flat(normals: &[[f32; 3]]) -> f32 {
    if normals.is_empty() {
        return f32::NAN;
    }
    let n0 = normals[0];
    const EPS: f32 = 1.0e-5;
    for n in normals.iter().skip(1) {
        let d = (n[0] - n0[0]).abs() + (n[1] - n0[1]).abs() + (n[2] - n0[2]).abs();
        if d > EPS {
            return std::f32::consts::PI;
        }
    }
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_5_2_5_u1_meshlet_cluster_limit_1024() {
        assert_eq!(meshlet_cluster_count(1024, 1024), 16);
        assert_eq!(1024_u32.div_ceil(MAX_MESHLET_VERTICES), 16);
        assert_eq!(1024_u32.div_ceil(MAX_MESHLET_TRIANGLES), 9);
    }

    #[test]
    fn tc_ir_5_2_5_u2_normal_cone_flat_quad() {
        let n = [0.0_f32, 0.0, 1.0];
        let normals = [n, n, n, n];
        assert_eq!(normal_cone_half_angle_flat(&normals), 0.0);
    }
}
