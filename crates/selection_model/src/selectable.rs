//! `Selectable` metadata and pick redirection rules.

use crate::types::EntityRef;

/// Marks an entity as participating in editor picking and optional grouping.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Selectable {
    /// Optional selection group root; picks on this entity redirect to the root.
    pub group: Option<EntityRef>,
    /// When `false`, picking must ignore this entity.
    pub pickable: bool,
}

/// Returns the entity that should receive selection after applying group rules.
pub fn resolve_pick_entity(entity: EntityRef, selectable: &Selectable) -> EntityRef {
    if let Some(root) = selectable.group {
        root
    } else {
        entity
    }
}

/// Drops hit candidates that must not participate in picking (`pickable == false`).
///
/// Call after spatial queries so non-pickable occluders never reach selection (`TC-15.1.4.2.1`).
pub fn filter_pickable_entities(
    candidates: &[(EntityRef, Selectable)],
) -> impl Iterator<Item = EntityRef> + '_ {
    candidates
        .iter()
        .filter(|(_, sel)| sel.pickable)
        .map(|(entity, _)| *entity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selectable_pickable_flag() {
        let hidden = Selectable {
            group: None,
            pickable: false,
        };
        assert!(!hidden.pickable);
    }

    #[test]
    fn test_selectable_group_redirect() {
        let root = EntityRef(1);
        let child = EntityRef(2);
        let child_sel = Selectable {
            group: Some(root),
            pickable: true,
        };
        assert_eq!(resolve_pick_entity(child, &child_sel), root);
    }

    #[test]
    fn test_filter_pickable_drops_non_pickable_hits() {
        let candidates = [
            (
                EntityRef(1),
                Selectable {
                    group: None,
                    pickable: true,
                },
            ),
            (
                EntityRef(2),
                Selectable {
                    group: None,
                    pickable: false,
                },
            ),
            (
                EntityRef(3),
                Selectable {
                    group: None,
                    pickable: true,
                },
            ),
        ];
        let picked: Vec<EntityRef> = filter_pickable_entities(&candidates).collect();
        assert_eq!(picked, vec![EntityRef(1), EntityRef(3)]);
    }
}
