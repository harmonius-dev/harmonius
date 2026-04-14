//! Processing pipeline with cycle detection.

use std::collections::{HashMap, HashSet};

use crate::AssetId;

/// Pipeline failure modes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PipelineError {
    /// Directed cycle in dependency edges.
    DependencyCycle {
        /// Asset ids on the cycle (including repeated start).
        path: Vec<AssetId>,
    },
}

/// Run pipeline on edges `edges[i] = (from, to)` meaning `from` depends on `to`.
pub fn run_pipeline(edges: &[(AssetId, AssetId)]) -> Result<(), PipelineError> {
    let mut adj: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut nodes = HashSet::new();
    for (a, b) in edges {
        adj.entry(a.0).or_default().push(b.0);
        nodes.insert(a.0);
        nodes.insert(b.0);
    }
    let mut state: HashMap<u64, u8> = HashMap::new();
    let mut stack: Vec<u64> = Vec::new();
    for &s in &nodes {
        if state.get(&s).copied().unwrap_or(0) == 0 {
            if let Err(path) = visit(s, &adj, &mut state, &mut stack) {
                return Err(PipelineError::DependencyCycle { path });
            }
        }
    }
    Ok(())
}

fn visit(
    u: u64,
    adj: &HashMap<u64, Vec<u64>>,
    state: &mut HashMap<u64, u8>,
    stack: &mut Vec<u64>,
) -> Result<(), Vec<AssetId>> {
    match state.get(&u).copied().unwrap_or(0) {
        2 => return Ok(()),
        1 => {
            let pos = stack.iter().position(|&x| x == u).unwrap();
            let mut cyc: Vec<AssetId> = stack[pos..].iter().map(|&x| AssetId(x)).collect();
            cyc.push(AssetId(u));
            return Err(cyc);
        }
        _ => {}
    }
    state.insert(u, 1);
    stack.push(u);
    for &v in adj.get(&u).into_iter().flatten() {
        visit(v, adj, state, stack)?;
    }
    state.insert(u, 2);
    stack.pop();
    Ok(())
}
