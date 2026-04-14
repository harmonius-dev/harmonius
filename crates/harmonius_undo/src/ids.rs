//! Stable identifiers used by undo records.

/// Author of a command in a collaborative session.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UserId(pub u64);

/// Groups multiple editor commands into a single undo step.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TxId(pub u64);

/// Stable identifier for a command record.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CommandId(pub u64);
