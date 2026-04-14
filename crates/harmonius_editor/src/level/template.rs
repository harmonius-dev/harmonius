//! Entity template hierarchy and propagation helpers for `PLAN-tools-level-world`.
//!
//! Covered test IDs: `TC-15.2.2.1`, `TC-15.2.2.2` in
//! `docs/design/tools/level-world-test-cases.md`.

use std::collections::BTreeMap;

/// Identifier for one logical entity inside a [`TemplateAsset`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EntityTemplateId(pub u32);

/// One node in a template hierarchy (`docs/design/tools/level-world.md`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityTemplate {
    /// Stable id within the asset.
    pub id: EntityTemplateId,
    /// Parent in the template graph, [`None`] for the root.
    pub parent: Option<EntityTemplateId>,
    /// Child ids; iteration order is sorted by id for determinism.
    pub children: Vec<EntityTemplateId>,
}

/// Serialized template graph (subset used by the editor crate).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemplateAsset {
    /// All entity definitions; ids must be unique.
    pub entities: Vec<EntityTemplate>,
}

/// One spawned row after [`instantiate_template_hierarchy`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpawnedEntity {
    /// Template-space id copied from [`EntityTemplate`].
    pub template_id: EntityTemplateId,
    /// Dense runtime id assigned in deterministic DFS order starting at `0`.
    pub instance_id: u32,
    /// Parent runtime id, [`None`] for the root instance.
    pub parent_instance: Option<u32>,
}

/// [`TemplateAsset`] is inconsistent or not a single-root tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstantiateError {
    /// Two or more [`EntityTemplate`] rows share the same [`EntityTemplateId`].
    DuplicateTemplateId,
    /// A [`EntityTemplate::parent`] or child reference points to a missing id.
    BrokenReference,
    /// Zero or more than one root (`parent == None`).
    RootCountInvalid,
    /// [`EntityTemplate::parent`] does not list this node as a child (edge mismatch).
    ParentChildMismatch,
}

/// Builds `id -> template` with deterministic key order.
fn index_entities(
    entities: &[EntityTemplate],
) -> Result<BTreeMap<EntityTemplateId, &EntityTemplate>, InstantiateError> {
    let mut map = BTreeMap::new();
    for e in entities {
        if map.insert(e.id, e).is_some() {
            return Err(InstantiateError::DuplicateTemplateId);
        }
    }
    Ok(map)
}

fn parent_child_consistent(
    by_id: &BTreeMap<EntityTemplateId, &EntityTemplate>,
) -> Result<(), InstantiateError> {
    for (&id, node) in by_id {
        if let Some(pid) = node.parent {
            let parent = by_id.get(&pid).ok_or(InstantiateError::BrokenReference)?;
            if !parent.children.contains(&id) {
                return Err(InstantiateError::ParentChildMismatch);
            }
        }
        for &cid in &node.children {
            let child = by_id.get(&cid).ok_or(InstantiateError::BrokenReference)?;
            if child.parent != Some(id) {
                return Err(InstantiateError::ParentChildMismatch);
            }
        }
    }
    Ok(())
}

fn single_root(by_id: &BTreeMap<EntityTemplateId, &EntityTemplate>) -> Result<EntityTemplateId, InstantiateError> {
    let mut roots = by_id
        .values()
        .filter(|n| n.parent.is_none())
        .map(|n| n.id);
    let root = roots.next().ok_or(InstantiateError::RootCountInvalid)?;
    if roots.next().is_some() {
        return Err(InstantiateError::RootCountInvalid);
    }
    Ok(root)
}

fn visit_depth_first_sorted(
    id: EntityTemplateId,
    parent_instance: Option<u32>,
    by_id: &BTreeMap<EntityTemplateId, &EntityTemplate>,
    next_instance: &mut u32,
    out: &mut Vec<SpawnedEntity>,
) {
    let instance_id = *next_instance;
    *next_instance += 1;
    out.push(SpawnedEntity {
        template_id: id,
        instance_id,
        parent_instance,
    });
    let node = by_id.get(&id).expect("visit only known ids");
    let mut children: Vec<EntityTemplateId> = node.children.clone();
    children.sort();
    for cid in children {
        visit_depth_first_sorted(cid, Some(instance_id), by_id, next_instance, out);
    }
}

/// Spawns one runtime row per template entity in deterministic DFS order.
///
/// Validates a single-root tree with consistent parent/child edges (`level-world.md`).
pub fn instantiate_template_hierarchy(
    asset: &TemplateAsset,
) -> Result<Vec<SpawnedEntity>, InstantiateError> {
    if asset.entities.is_empty() {
        return Err(InstantiateError::RootCountInvalid);
    }
    let by_id = index_entities(&asset.entities)?;
    parent_child_consistent(&by_id)?;
    let root = single_root(&by_id)?;
    let mut out = Vec::with_capacity(asset.entities.len());
    let mut next = 0_u32;
    visit_depth_first_sorted(root, None, &by_id, &mut next, &mut out);
    if out.len() != asset.entities.len() {
        return Err(InstantiateError::BrokenReference);
    }
    Ok(out)
}

/// Root entity hp on the **source** template (numeric stand-in for component data).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceTemplateHp {
    /// Which template entity carries `hp` on the source asset.
    pub entity: EntityTemplateId,
    /// Current hit points on the source definition.
    pub hp: i32,
}

/// Per-instance hp override for [`resolve_instance_hp`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstanceHp {
    /// Template entity this row tracks (same id as on [`SourceTemplateHp`] for the tests).
    pub entity: EntityTemplateId,
    /// When [`None`], instance tracks [`SourceTemplateHp::hp`].
    pub hp_override: Option<i32>,
}

/// Effective hp: override wins; otherwise the live source value (`TC-15.2.2.2`).
#[must_use]
pub fn resolve_instance_hp(source: &SourceTemplateHp, inst: &InstanceHp) -> i32 {
    if inst.entity != source.entity {
        return source.hp;
    }
    inst.hp_override.unwrap_or(source.hp)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.2.2.1 — Instantiate nested template (`level-world-test-cases.md`).
    #[test]
    fn tc_15_2_2_1_entity_template_instantiate_three_levels() {
        let root = EntityTemplateId(0);
        let child = EntityTemplateId(1);
        let grand = EntityTemplateId(2);
        let asset = TemplateAsset {
            entities: vec![
                EntityTemplate {
                    id: root,
                    parent: None,
                    children: vec![child],
                },
                EntityTemplate {
                    id: child,
                    parent: Some(root),
                    children: vec![grand],
                },
                EntityTemplate {
                    id: grand,
                    parent: Some(child),
                    children: vec![],
                },
            ],
        };
        let got = instantiate_template_hierarchy(&asset).expect("valid tree");
        assert_eq!(got.len(), 3);
        assert_eq!(got[0].template_id, root);
        assert_eq!(got[0].instance_id, 0);
        assert_eq!(got[0].parent_instance, None);
        assert_eq!(got[1].template_id, child);
        assert_eq!(got[1].instance_id, 1);
        assert_eq!(got[1].parent_instance, Some(0));
        assert_eq!(got[2].template_id, grand);
        assert_eq!(got[2].instance_id, 2);
        assert_eq!(got[2].parent_instance, Some(1));
    }

    /// TC-15.2.2.2 — Source change vs overrides (`level-world-test-cases.md`).
    #[test]
    fn tc_15_2_2_2_entity_template_propagation_respects_overrides() {
        let e0 = EntityTemplateId(0);
        let mut source = SourceTemplateHp { entity: e0, hp: 100 };
        let no_override = InstanceHp {
            entity: e0,
            hp_override: None,
        };
        assert_eq!(resolve_instance_hp(&source, &no_override), 100);
        source.hp = 200;
        assert_eq!(resolve_instance_hp(&source, &no_override), 200);

        let with_override = InstanceHp {
            entity: e0,
            hp_override: Some(50),
        };
        source.hp = 200;
        assert_eq!(resolve_instance_hp(&source, &with_override), 50);
    }
}
