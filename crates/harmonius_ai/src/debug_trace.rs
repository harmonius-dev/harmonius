//! Deterministic trace log for behavior-tree debugging.

/// One trace line recorded during a tick.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceEntry {
    /// Human-readable node label (matches design overlay expectations).
    pub label: String,
    /// Final status after this visit.
    pub status: crate::bt::NodeStatus,
}

/// Append-only trace for a single tick or session.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TraceLog {
    entries: Vec<TraceEntry>,
}

impl TraceLog {
    /// Builds an empty trace buffer.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Records a node visit (appends).
    pub fn push(&mut self, label: impl Into<String>, status: crate::bt::NodeStatus) {
        self.entries.push(TraceEntry {
            label: label.into(),
            status,
        });
    }

    /// Prepends a node visit (used so composite parents appear before children in the log).
    pub fn push_front(&mut self, label: impl Into<String>, status: crate::bt::NodeStatus) {
        self.entries.insert(
            0,
            TraceEntry {
                label: label.into(),
                status,
            },
        );
    }

    /// Ordered trace lines.
    pub fn entries(&self) -> &[TraceEntry] {
        &self.entries
    }

    /// Clears all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
