/// Stable asset identifier (design: stable across peers).
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
#[rkyv(derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd))]
pub struct AssetId(pub u64);
