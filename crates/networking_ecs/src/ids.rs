/// Opaque stable identifier for a replicated entity.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd))]
#[repr(transparent)]
pub struct Entity(pub u32);

/// Logical transport connection (client or uplink).
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd))]
#[repr(transparent)]
pub struct ConnectionId(pub u32);

/// Monotonic simulation tick used for replication ordering and baselines.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd))]
#[repr(transparent)]
pub struct SequenceTick(pub u32);
