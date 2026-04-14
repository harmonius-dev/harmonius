//! Subtree reference validation between behavior-tree assets.

use std::collections::HashMap;

/// Load-time failure for invalid subtree wiring.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubtreeError {
    /// Directed cycle detected in subtree asset references.
    CircularReference,
}

/// Validates a directed graph of asset IDs. Each `edges[a]` lists assets referenced from `a`.
pub fn validate_subtree_graph(
    roots: &[u8],
    edges: &HashMap<u8, Vec<u8>>,
) -> Result<(), SubtreeError> {
    for &start in roots {
        let mut color = HashMap::<u8, u8>::new();
        if dfs_cycle(start, edges, &mut color) {
            return Err(SubtreeError::CircularReference);
        }
    }
    Ok(())
}

fn dfs_cycle(node: u8, edges: &HashMap<u8, Vec<u8>>, color: &mut HashMap<u8, u8>) -> bool {
    match color.get(&node).copied().unwrap_or(0) {
        2 => false,
        1 => true,
        _ => {
            color.insert(node, 1);
            if let Some(ch) = edges.get(&node) {
                for &c in ch {
                    if dfs_cycle(c, edges, color) {
                        return true;
                    }
                }
            }
            color.insert(node, 2);
            false
        }
    }
}
