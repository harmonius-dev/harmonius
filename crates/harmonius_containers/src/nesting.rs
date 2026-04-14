//! Container nesting depth and cycle detection.
//!
//! For container moves, call [`validate_circular`](crate::nesting::validate_circular) in addition
//! to [`validate_nesting_insert`](crate::nesting::validate_nesting_insert) so cycles are ruled
//! out before depth checks alone.

use std::collections::HashMap;

use crate::entity::Entity;
use crate::transfer::TransferError;

/// Computes the 1-based nesting depth of `entity` using immediate container parents.
#[must_use]
pub fn compute_depth(entity: Entity, parent: &HashMap<Entity, Entity>) -> u8 {
    let mut depth: u8 = 1;
    let mut current = entity;
    let mut guard = 0u16;
    while guard < 4096 {
        guard += 1;
        if let Some(next) = parent.get(&current).copied() {
            depth = depth.saturating_add(1);
            current = next;
        } else {
            break;
        }
    }
    depth
}

/// Validates that inserting `inner` into `target` does not exceed `max_depth` for the moved subtree.
pub fn validate_nesting_insert(
    inner: Entity,
    target: Entity,
    parent: &HashMap<Entity, Entity>,
    max_depth: u8,
) -> Result<(), TransferError> {
    if inner == target {
        return Err(TransferError::CircularNesting);
    }

    let subtree_height = max_subtree_depth(inner, parent);
    let target_depth = compute_depth(target, parent);
    let attempted = target_depth.saturating_add(subtree_height);
    if attempted > max_depth {
        return Err(TransferError::NestingTooDeep {
            max: max_depth,
            attempted,
        });
    }
    Ok(())
}

/// Rejects moves that would place an ancestor into its descendant.
pub fn validate_circular(
    inner: Entity,
    target: Entity,
    parent: &HashMap<Entity, Entity>,
) -> Result<(), TransferError> {
    if inner == target {
        return Err(TransferError::CircularNesting);
    }
    let mut walk = Some(target);
    let mut guard = 0u16;
    while guard < 4096 {
        guard += 1;
        match walk {
            None => return Ok(()),
            Some(node) if node == inner => return Err(TransferError::CircularNesting),
            Some(node) => walk = parent.get(&node).copied(),
        }
    }
    Ok(())
}

fn max_subtree_depth(root: Entity, parent: &HashMap<Entity, Entity>) -> u8 {
    let mut max_depth: u8 = 1;
    for (&child, &p) in parent {
        if p != root {
            continue;
        }
        let d = max_subtree_depth(child, parent).saturating_add(1);
        max_depth = max_depth.max(d);
    }
    max_depth
}
