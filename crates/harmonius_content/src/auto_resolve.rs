//! Auto-merge strategies for scalar and set conflicts.

/// How a field was resolved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResolutionStrategy {
    /// Last writer wins by timestamp.
    LastWriterWins,
    /// Set union.
    Union,
}

/// One resolved field record.
#[derive(Clone, Debug, PartialEq)]
pub struct AutoResolution {
    /// Strategy used.
    pub strategy: ResolutionStrategy,
    /// Field label.
    pub field: &'static str,
}

/// Auto-resolved payload.
#[derive(Clone, Debug, PartialEq)]
pub struct AutoResolved {
    /// Winning roughness.
    pub roughness: f32,
    /// Resolution log.
    pub resolutions: Vec<AutoResolution>,
    /// Merged tags (sorted, unique).
    pub tags: Vec<String>,
}

/// LWW on `roughness` using timestamps `t_ours`, `t_theirs`.
pub fn auto_resolve_lww(roughness_ours: f32, t_ours: u64, roughness_theirs: f32, t_theirs: u64) -> AutoResolved {
    let (roughness, strat) = if t_theirs >= t_ours {
        (roughness_theirs, ResolutionStrategy::LastWriterWins)
    } else {
        (roughness_ours, ResolutionStrategy::LastWriterWins)
    };
    AutoResolved {
        roughness,
        resolutions: vec![AutoResolution {
            strategy: strat,
            field: "roughness",
        }],
        tags: vec![],
    }
}

/// Union merge on string tags.
pub fn auto_resolve_union_tags(ancestor: &[String], ours: &[String], theirs: &[String]) -> AutoResolved {
    let mut set: std::collections::BTreeSet<String> = ancestor.iter().cloned().collect();
    for t in ours.iter().chain(theirs.iter()) {
        set.insert(t.clone());
    }
    AutoResolved {
        roughness: 0.0,
        resolutions: vec![AutoResolution {
            strategy: ResolutionStrategy::Union,
            field: "tags",
        }],
        tags: set.into_iter().collect(),
    }
}
