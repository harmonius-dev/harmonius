//! Allocation tags and subsystem identifiers.

/// Numeric subsystem identifier for budgets and profiling.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SubsystemId(pub u16);

/// Tag attached to allocations for accounting and profiling.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AllocationTag {
    /// Owning subsystem.
    pub subsystem: SubsystemId,
    /// Optional human-readable label.
    pub label: Option<&'static str>,
}

impl AllocationTag {
    /// Effective tag for accounting when `other` omits a label.
    #[must_use]
    pub fn inherit(self, other: Self) -> Self {
        if other.label.is_some() {
            other
        } else {
            Self {
                subsystem: other.subsystem,
                label: self.label,
            }
        }
    }
}
