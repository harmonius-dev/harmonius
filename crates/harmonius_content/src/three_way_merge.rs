//! Toy three-way merge for disjoint node additions.

/// Merge outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MergeResult {
    /// Clean merge.
    Success {
        /// Merged payload.
        merged: Vec<u8>,
    },
}

/// Merge logic graphs encoded as comma-separated node lists after `nodes:` prefix.
pub fn three_way_merge_disjoint_graphs(
    ancestor: &[u8],
    ours: &[u8],
    theirs: &[u8],
) -> MergeResult {
    fn nodes(line: &[u8]) -> Vec<String> {
        let s = std::str::from_utf8(line).unwrap_or("");
        s.trim_start_matches("nodes:")
            .split(',')
            .filter(|x| !x.is_empty())
            .map(std::string::ToString::to_string)
            .collect()
    }
    let mut a = nodes(ancestor);
    let bo = nodes(ours);
    let bt = nodes(theirs);
    for n in bo {
        if !a.contains(&n) {
            a.push(n);
        }
    }
    for n in bt {
        if !a.contains(&n) {
            a.push(n);
        }
    }
    let body = format!("nodes:{}", a.join(","));
    MergeResult::Success {
        merged: body.into_bytes(),
    }
}
