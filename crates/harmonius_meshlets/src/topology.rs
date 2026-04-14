use crate::BuildError;

/// Returns [`BuildError::InvalidTopology`] when any undirected edge is shared by more than two
/// triangles (non-manifold edge).
pub(crate) fn validate_manifold(indices: &[u32]) -> Result<(), BuildError> {
    if indices.len() % 3 != 0 {
        return Err(BuildError::InvalidTopology);
    }
    if indices.is_empty() {
        return Ok(());
    }

    let mut edge_counts: std::collections::HashMap<(u32, u32), u8> =
        std::collections::HashMap::new();
    for tri in indices.chunks_exact(3) {
        let a = tri[0];
        let b = tri[1];
        let c = tri[2];
        for (u, v) in [(a, b), (b, c), (c, a)] {
            let key = if u < v { (u, v) } else { (v, u) };
            let e = edge_counts.entry(key).or_insert(0);
            *e = e.saturating_add(1);
            if *e > 2 {
                return Err(BuildError::InvalidTopology);
            }
        }
    }
    Ok(())
}
