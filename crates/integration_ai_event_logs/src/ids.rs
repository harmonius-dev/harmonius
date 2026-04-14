//! Stable identifiers shared by AI and simulation layers.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Codegen event kind discriminator for [`crate::event_log::EventLogQuery`].
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    RkyvDeserialize,
    RkyvSerialize,
)]
#[archive_attr(derive(
    bytecheck::CheckBytes,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
))]
pub struct EventTypeId(pub u32);

/// Opaque entity identifier used in logs and blackboards.
///
/// Prefer [`std::collections::BTreeMap`] keyed by `Entity` when iteration order must match
/// deterministic simulation replay (see integration test cases for blackboard paths).
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    RkyvDeserialize,
    RkyvSerialize,
)]
#[archive_attr(derive(bytecheck::CheckBytes, Debug, Eq, Hash, Ord, PartialEq, PartialOrd))]
pub struct Entity(pub u64);

/// Codegen action discriminator carried on decision entries.
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    RkyvDeserialize,
    RkyvSerialize,
)]
#[archive_attr(derive(bytecheck::CheckBytes, Debug, Eq, Hash, Ord, PartialEq, PartialOrd))]
pub struct ActionId(pub u32);
