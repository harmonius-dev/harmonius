//! Aggregate transform helpers for gizmo placement.

use glam::{Affine3A, Quat, Vec3};

use crate::selection_state::SelectionState;
use crate::types::EntityRef;

/// Computes an aggregate [`Affine3A`] for the current entity selection.
///
/// A single selection returns the resolved transform unchanged. Multiple selections average
/// translations while keeping the first resolved rotation.
pub fn aggregate_affine_for_selection(
    selection: &SelectionState,
    mut resolve: impl FnMut(EntityRef) -> Option<Affine3A>,
) -> Option<Affine3A> {
    let entities = selection.entities();
    if entities.is_empty() {
        return None;
    }
    if entities.len() == 1 {
        return resolve(entities[0]);
    }
    let mut sum = Vec3::ZERO;
    let mut count = 0u32;
    let mut first_rotation = None::<Quat>;
    for &entity in entities {
        if let Some(affine) = resolve(entity) {
            sum += Vec3::from(affine.translation);
            count += 1;
            if first_rotation.is_none() {
                first_rotation = Some(Quat::from_mat3(&glam::Mat3::from_mat4(glam::Mat4::from(
                    affine,
                ))));
            }
        }
    }
    if count == 0 {
        return None;
    }
    let translation = sum / count as f32;
    let rotation = first_rotation.unwrap_or(Quat::IDENTITY);
    Some(Affine3A::from_rotation_translation(rotation, translation))
}

#[cfg(test)]
mod tests {
    use glam::{Affine3A, Vec3};

    use super::*;
    use crate::types::EntityRef;

    fn e(id: u32) -> EntityRef {
        EntityRef(id)
    }

    #[test]
    fn test_aggregate_transform_single() {
        let mut sel = SelectionState::default();
        assert!(sel.add(e(1)));
        let t = Affine3A::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let agg = aggregate_affine_for_selection(
            &sel,
            |entity| {
                if entity == e(1) {
                    Some(t)
                } else {
                    None
                }
            },
        )
        .expect("aggregate");
        assert_eq!(Vec3::from(agg.translation), Vec3::from(t.translation));
    }

    #[test]
    fn test_aggregate_transform_average() {
        let mut sel = SelectionState::default();
        assert!(sel.add(e(1)));
        assert!(sel.add(e(2)));
        let agg = aggregate_affine_for_selection(&sel, |entity| {
            if entity == e(1) {
                Some(Affine3A::from_translation(Vec3::new(0.0, 0.0, 0.0)))
            } else if entity == e(2) {
                Some(Affine3A::from_translation(Vec3::new(2.0, 0.0, 0.0)))
            } else {
                None
            }
        })
        .expect("aggregate");
        assert_eq!(Vec3::from(agg.translation), Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_aggregate_transform_empty() {
        let sel = SelectionState::default();
        assert!(aggregate_affine_for_selection(&sel, |_| None).is_none());
    }
}
