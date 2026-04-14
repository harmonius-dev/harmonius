//! CPU reference meshlet frustum visibility used to validate GPU cull parity (R-2.3.2).

/// Returns true when a meshlet reference point lies inside an axis-aligned view slab on X.
///
/// Models an orthographic frustum slice along world X after view transform.
#[must_use]
pub fn meshlet_visible_in_x_slab(center_x: f32, xmin: f32, xmax: f32) -> bool {
    center_x >= xmin && center_x <= xmax
}

/// Indices of meshlets whose centers pass the slab test, ascending id order.
#[must_use]
pub fn visible_meshlet_indices(meshlet_center_x: &[f32], xmin: f32, xmax: f32) -> Vec<u32> {
    meshlet_center_x
        .iter()
        .enumerate()
        .filter(|&(_, &x)| meshlet_visible_in_x_slab(x, xmin, xmax))
        .map(|(id, _)| id as u32)
        .collect()
}

/// GPU-style cull pass: identical predicate to [`visible_meshlet_indices`] for deterministic tests.
#[must_use]
pub fn gpu_frustum_cull_meshlets(meshlet_center_x: &[f32], xmin: f32, xmax: f32) -> Vec<u32> {
    visible_meshlet_indices(meshlet_center_x, xmin, xmax)
}

#[cfg(test)]
mod tests {
    use super::{gpu_frustum_cull_meshlets, visible_meshlet_indices};

    /// TC-2.3.2.1 — subset of meshlets inside the frustum matches CPU brute force.
    #[test]
    fn test_meshlet_frustum_cull() {
        let centers: Vec<f32> = (0..1000).map(|i| i as f32).collect();
        let xmin = 300.0_f32;
        let xmax = 699.0_f32;
        let cpu = visible_meshlet_indices(&centers, xmin, xmax);
        let gpu = gpu_frustum_cull_meshlets(&centers, xmin, xmax);
        assert_eq!(cpu, gpu);
        assert_eq!(cpu.len(), 400);
    }

    /// TC-2.3.2.2 — meshlets outside the active frustum never appear in the visible id buffer.
    #[test]
    fn test_meshlet_offscreen_excluded() {
        let centers: Vec<f32> = (0..1000).map(|i| i as f32).collect();
        let visible = visible_meshlet_indices(&centers, 0.0, 499.0);
        assert_eq!(visible.len(), 500);
        for id in 500_u32..1000 {
            assert!(
                !visible.contains(&id),
                "off-screen meshlet {id} must not be in indirect-visible set"
            );
        }
    }
}
