//! Socket offset transform composition for attached items (IR-5.8.1).

use glam::{Mat4, Quat, Vec3};

/// World-space transform matrix for an entity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlobalTransform {
    /// Column-major world matrix (`parent * offset` composition uses this field).
    pub matrix: Mat4,
}

/// Authoritative socket pose in parent space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SocketDefinition {
    /// Translation from parent origin to socket origin.
    pub transform_offset: Vec3,
    /// Rotation from parent basis to socket basis.
    pub rotation_offset: Quat,
}

/// Computes the attached entity world transform from the parent transform and socket definition.
///
/// `offset` is `T * R` in column-vector convention (`parent * offset`).
#[must_use]
pub fn compute_attachment_transform(
    parent_gt: &GlobalTransform,
    socket_def: &SocketDefinition,
) -> GlobalTransform {
    let offset =
        Mat4::from_rotation_translation(socket_def.rotation_offset, socket_def.transform_offset);
    GlobalTransform {
        matrix: parent_gt.matrix * offset,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-5.8.1.U1 — compute_attachment_transform.
    #[test]
    fn tc_ir_5_8_1_u1_offset_translation_composes_with_parent() {
        let parent = GlobalTransform {
            matrix: Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0)),
        };
        let socket = SocketDefinition {
            transform_offset: Vec3::new(0.0, 1.0, 0.0),
            rotation_offset: Quat::IDENTITY,
        };
        let child = compute_attachment_transform(&parent, &socket);
        let out_t = child.matrix.transform_point3(Vec3::ZERO);
        assert!((out_t.x - 1.0).abs() < 1e-5);
        assert!((out_t.y - 3.0).abs() < 1e-5);
        assert!((out_t.z - 3.0).abs() < 1e-5);
    }

    /// TC-IR-5.8.1.U1 — linear part (rotation) composes, not only translation of origin.
    #[test]
    fn tc_ir_5_8_1_u1_rotation_preserved_under_composition() {
        let parent = GlobalTransform {
            matrix: Mat4::from_axis_angle(Vec3::Y, 0.25)
                * Mat4::from_translation(Vec3::new(2.0, 0.0, 1.0)),
        };
        let socket = SocketDefinition {
            transform_offset: Vec3::new(0.5, 0.0, 0.0),
            rotation_offset: Quat::from_rotation_x(0.9),
        };
        let offset =
            Mat4::from_rotation_translation(socket.rotation_offset, socket.transform_offset);
        let expected = parent.matrix * offset;
        let child = compute_attachment_transform(&parent, &socket);
        assert_mat4_close(child.matrix, expected);
    }

    /// TC-IR-5.8.1.U2 — identity socket offset yields parent transform.
    #[test]
    fn tc_ir_5_8_1_u2_identity_socket_offset_matches_parent() {
        let parent = GlobalTransform {
            matrix: Mat4::from_axis_angle(Vec3::Y, 0.7)
                .mul_mat4(&Mat4::from_translation(Vec3::new(2.0, -1.0, 4.0))),
        };
        let socket = SocketDefinition {
            transform_offset: Vec3::ZERO,
            rotation_offset: Quat::IDENTITY,
        };
        let child = compute_attachment_transform(&parent, &socket);
        assert_eq!(child.matrix, parent.matrix);
    }

    /// TC-IR-5.8.1.N1 — zero/default offsets behave as identity (FM-1).
    #[test]
    fn tc_ir_5_8_1_n1_zero_socket_fields_use_identity_offset() {
        let parent = GlobalTransform {
            matrix: Mat4::from_translation(Vec3::new(5.0, 6.0, 7.0)),
        };
        let socket = SocketDefinition {
            transform_offset: Vec3::ZERO,
            rotation_offset: Quat::IDENTITY,
        };
        let child = compute_attachment_transform(&parent, &socket);
        assert_eq!(child.matrix, parent.matrix);
    }

    fn assert_mat4_close(a: Mat4, b: Mat4) {
        const EPS: f32 = 1e-4;
        for c in 0..4 {
            for r in 0..4 {
                assert!((a.col(c)[r] - b.col(c)[r]).abs() < EPS);
            }
        }
    }
}
