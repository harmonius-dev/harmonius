//! Shared governance error taxonomy.

/// Policy, provenance, audit, and review failures surfaced to editor systems.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GovernanceError {
    /// Enterprise policy bytes disagree with the attached signature (TC-15.7.5.1).
    InvalidSignature,
    /// Policy `version` is older than the currently applied document (TC-15.7.5.2).
    StalePolicy,
}
