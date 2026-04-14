//! Bake-time validation helpers (cycles, types, sandbox).

use std::collections::VecDeque;

use crate::types::{BakeError, ColumnType, FormulaId};

/// Detects dependency cycles between formulas using Kahn topological ordering.
///
/// Each `(consumer, depends_on)` tuple states that `consumer` must be evaluated after
/// `depends_on`.
pub fn detect_formula_dependency_cycle(
    formula_count: u32,
    deps: &[(FormulaId, FormulaId)],
) -> Result<(), BakeError> {
    if formula_count == 0 {
        return Ok(());
    }
    let n = formula_count as usize;
    let mut indegree = vec![0u32; n];
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (consumer, depends_on) in deps {
        let c = consumer.0 as usize;
        let d = depends_on.0 as usize;
        if c >= n || d >= n {
            return Err(BakeError::UnresolvedSymbol);
        }
        adj[d].push(c);
        indegree[c] += 1;
    }
    let mut queue = VecDeque::new();
    for (idx, degree) in indegree.iter().enumerate().take(n) {
        if *degree == 0 {
            queue.push_back(idx);
        }
    }
    let mut visited = 0u32;
    while let Some(node) = queue.pop_front() {
        visited += 1;
        for &next in &adj[node] {
            indegree[next] -= 1;
            if indegree[next] == 0 {
                queue.push_back(next);
            }
        }
    }
    if visited == formula_count {
        Ok(())
    } else {
        Err(BakeError::CycleDetected)
    }
}

/// Validates that a monomorphized output type matches the authored column schema.
pub fn validate_formula_output_type(
    column: ColumnType,
    output: ColumnType,
) -> Result<(), BakeError> {
    match (column, output) {
        (ColumnType::F32, ColumnType::F32) | (ColumnType::I32, ColumnType::I32) => Ok(()),
        _ => Err(BakeError::TypeMismatch),
    }
}

/// Rejects authored Rust sources that violate the scripting sandbox.
pub fn sandbox_check_rust_source(source: &str) -> Result<(), BakeError> {
    if source.contains("unsafe") {
        return Err(BakeError::SandboxViolation);
    }
    Ok(())
}
