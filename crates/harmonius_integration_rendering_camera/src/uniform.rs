//! [`crate::types::ViewUniform`] construction from a [`crate::types::RenderView`].

use glam::Vec3;

use crate::types::{RenderView, ViewUniform};

/// Builds a GPU uniform block from a render view.
#[must_use]
pub fn view_uniform_from_render_view(rv: &RenderView) -> ViewUniform {
    let world_from_view = rv.view_matrix.inverse();
    let camera_position = world_from_view.transform_point3(Vec3::ZERO);
    ViewUniform {
        view: rv.view_matrix,
        projection: rv.projection,
        view_projection: rv.projection * rv.view_matrix,
        camera_position: camera_position.extend(1.0),
    }
}
