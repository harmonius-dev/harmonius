//! VisualBinding transform write semantics (IR-5.8.1, FM-6).

use glam::Mat4;

use crate::GlobalTransform;

/// Prior-tick world transform used for render interpolation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreviousGlobalTransform {
    /// Column-major prior world matrix.
    pub matrix: Mat4,
}

/// Writes the new world transform while capturing the prior value for interpolation.
///
/// When `existing_current` is `None` (first frame / missing component), the previous matrix is
/// initialized to the newly computed transform so `lerp(prev, current, α)` does not jitter (FM-6).
#[must_use]
pub fn visual_binding_write_transform(
    existing_current: Option<GlobalTransform>,
    newly_computed: GlobalTransform,
) -> (GlobalTransform, PreviousGlobalTransform) {
    let prev_mat = existing_current
        .map(|g| g.matrix)
        .unwrap_or(newly_computed.matrix);
    (newly_computed, PreviousGlobalTransform { matrix: prev_mat })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-5.8.1.U3 — PreviousGlobalTransform captures old current on subsequent ticks.
    #[test]
    fn tc_ir_5_8_1_u3_previous_matches_prior_current() {
        let a = GlobalTransform {
            matrix: Mat4::from_translation(glam::Vec3::new(1.0, 0.0, 0.0)),
        };
        let b = GlobalTransform {
            matrix: Mat4::from_translation(glam::Vec3::new(3.0, 0.0, 0.0)),
        };

        let (first_gt, first_prev) = visual_binding_write_transform(None, a);
        assert_eq!(first_gt, a);
        assert_eq!(first_prev.matrix, a.matrix);

        let (second_gt, second_prev) = visual_binding_write_transform(Some(first_gt), b);
        assert_eq!(second_gt, b);
        assert_eq!(second_prev.matrix, a.matrix);
    }

    /// TC-IR-5.8.1.N3 — first tick initializes previous equal to new transform.
    #[test]
    fn tc_ir_5_8_1_n3_first_frame_prev_equals_current() {
        let computed = GlobalTransform {
            matrix: Mat4::from_translation(glam::Vec3::new(10.0, 2.0, -3.0)),
        };
        let (gt, prev) = visual_binding_write_transform(None, computed);
        assert_eq!(gt.matrix, prev.matrix);
    }
}
