//! Dense connection identifiers for voice demuxing.

/// Session-scoped connection handle used on the wire and in the audio thread.
#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize,
)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, Hash, PartialEq))]
#[repr(transparent)]
pub struct ConnectionId(pub u16);

impl ConnectionId {
    /// Dense slot index for `Vec`-backed per-connection voice state.
    #[must_use]
    pub const fn slot(self) -> usize {
        self.0 as usize
    }
}
