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
}
