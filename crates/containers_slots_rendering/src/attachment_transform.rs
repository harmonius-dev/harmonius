//! Socket offset transform composition for attached items (IR-5.8.1).

use glam::{Mat4, Quat, Vec3};

/// World-space transform matrix for an entity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlobalTransform(pub Mat4);

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
    let offset = Mat4::from_rotation_translation(
        socket_def.rotation_offset,
        socket_def.transform_offset,
    );
    GlobalTransform(parent_gt.0 * offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-5.8.1.U1 — compute_attachment_transform.
    #[test]
    fn tc_ir_5_8_1_u1_offset_translation_composes_with_parent() {
        let parent = GlobalTransform(Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0)));
        let socket = SocketDefinition {
            transform_offset: Vec3::new(0.0, 1.0, 0.0),
            rotation_offset: Quat::IDENTITY,
        };
        let child = compute_attachment_transform(&parent, &socket);
        let out_t = child.0.transform_point3(Vec3::ZERO);
        assert!((out_t.x - 1.0).abs() < 1e-5);
        assert!((out_t.y - 3.0).abs() < 1e-5);
        assert!((out_t.z - 3.0).abs() < 1e-5);
    }

    /// TC-IR-5.8.1.U2 — identity socket offset yields parent transform.
    #[test]
    fn tc_ir_5_8_1_u2_identity_socket_offset_matches_parent() {
        let parent = GlobalTransform(Mat4::from_axis_angle(Vec3::Y, 0.7).mul_mat4(&Mat4::from_translation(
            Vec3::new(2.0, -1.0, 4.0),
        )));
        let socket = SocketDefinition {
            transform_offset: Vec3::ZERO,
            rotation_offset: Quat::IDENTITY,
        };
        let child = compute_attachment_transform(&parent, &socket);
        assert_eq!(child.0, parent.0);
    }

    /// TC-IR-5.8.1.N1 — zero/default offsets behave as identity (FM-1).
    #[test]
    fn tc_ir_5_8_1_n1_zero_socket_fields_use_identity_offset() {
        let parent = GlobalTransform(Mat4::from_translation(Vec3::new(5.0, 6.0, 7.0)));
        let socket = SocketDefinition {
            transform_offset: Vec3::ZERO,
            rotation_offset: Quat::IDENTITY,
        };
        let child = compute_attachment_transform(&parent, &socket);
        assert_eq!(child.0, parent.0);
    }
}
