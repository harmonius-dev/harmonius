//! Wire-shaped types shared with transport (rkyv archived).

use crate::ids::{ConnectionId, SequenceTick};

/// Marker component for replicated entities (wire view).
#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, PartialEq))]
pub struct Replicated {
    /// Replication priority (higher = more bandwidth).
    pub priority: u8,
    /// Replication condition.
    pub condition: ReplicationCondition,
    /// Authority owner (server or specific client).
    pub authority: Authority,
}

/// Opaque filter ID for codegen-resolved custom replication rules.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, Hash, PartialEq))]
#[repr(transparent)]
pub struct ReplicationFilterId(pub u32);

/// Replication gating condition.
#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, PartialEq))]
pub enum ReplicationCondition {
    /// Always replicate when changed.
    Always,
    /// Only replicate to the owning client.
    OwnerOnly,
    /// Only replicate on initial spawn.
    InitialOnly,
    /// Replicate only while visible to a client camera.
    Visible,
    /// Replicate only while inside a relevancy trigger volume.
    InVolume,
    /// Custom condition resolved via static function-pointer table.
    Custom(ReplicationFilterId),
}

/// Authority holder for a replicated entity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, PartialEq))]
pub enum Authority {
    /// Server authoritative.
    Server,
    /// Client authoritative for this entity.
    Client(ConnectionId),
}

/// Per-client baseline snapshot carried on the wire.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, PartialEq))]
pub struct Baseline {
    /// Owning connection.
    pub connection: ConnectionId,
    /// Last acknowledged tick for this baseline.
    pub tick: SequenceTick,
    /// Dense baseline bytes parallel to chunk layout.
    pub state: Vec<u8>,
}

/// One differing byte run inside a delta payload.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, PartialEq))]
pub struct DeltaRun {
    /// Byte offset in the dense chunk layout.
    pub offset: u32,
    /// New live bytes for this run.
    pub bytes: Vec<u8>,
}

/// Wire packet for a delta update.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, PartialEq))]
pub struct DeltaPayload {
    /// Tick this delta applies to.
    pub tick: SequenceTick,
    /// Encoded runs (dense-array XOR diff + RLE of non-equal bytes).
    pub runs: Vec<DeltaRun>,
}

/// ACK forwarded from the main thread to the replication worker.
#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug, Eq, PartialEq))]
pub struct AckMessage {
    /// Client that acknowledged.
    pub client: ConnectionId,
    /// Acknowledged tick.
    pub tick: SequenceTick,
}
