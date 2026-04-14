//! Stable identifiers shared by AI and simulation layers.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Opaque entity identifier used in logs and blackboards.
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
