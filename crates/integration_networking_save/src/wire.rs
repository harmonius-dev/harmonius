//! rkyv wire types for save-related RPCs (IR-4.6.2, IR-4.6.5).

use rkyv::{Archive, Deserialize, Serialize};

/// Opaque client / peer handle carried on the wire with RPC payloads.
#[derive(
    Archive, Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[archive(compare(PartialEq))]
pub struct ConnectionId(pub u64);

/// Identifies a save slot in RPC payloads and metadata.
#[derive(
    Archive, Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[archive(compare(PartialEq))]
pub struct SlotId(pub u32);

/// Monotonic schema version for save payloads.
#[derive(
    Archive, Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[archive(compare(PartialEq))]
pub struct SchemaVersion(pub u32);

/// Slot metadata carried on successful save RPC results.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[archive(compare(PartialEq))]
pub struct SaveSlotMeta {
    /// Slot this metadata describes.
    pub slot_id: SlotId,
    /// Wall-clock seconds since Unix epoch (design contract).
    pub timestamp: u64,
    /// Blake3-style content digest (32 bytes).
    pub content_hash: [u8; 32],
    /// Engine save schema revision.
    pub schema_version: SchemaVersion,
}

/// High-level save trigger classification.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[archive(compare(PartialEq))]
pub enum SaveType {
    /// Player-invoked save from UI.
    Manual,
    /// Fast slot rotation save.
    Quicksave,
    /// System-triggered periodic save.
    Autosave,
    /// Multiplayer coordinated barrier save.
    Checkpoint,
}

/// Errors surfaced across save RPC and I/O boundaries.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[archive(compare(PartialEq))]
pub enum SaveError {
    /// Local I/O failure, channel full, or job-system failure.
    Io,
    /// Authenticated but not permitted to save.
    PermissionDenied,
    /// Device or quota exhaustion.
    OutOfSpace,
    /// Detected corruption while reading or writing.
    Corruption,
    /// Cloud path unavailable (full channel, offline, and similar).
    CloudUnavailable,
}

/// Server-authored outcome for a client `SaveRequestRpc`.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[archive(compare(PartialEq))]
pub enum SaveRpcResult {
    /// Save finished and produced fresh metadata.
    Success {
        /// Serialized slot header returned to the client.
        meta: SaveSlotMeta,
    },
    /// Save failed after permission checks passed.
    Failed {
        /// Failure reason for UI / telemetry.
        reason: SaveError,
    },
    /// Request rejected before attempting serialization.
    PermissionDenied,
}

/// Client-to-server request to perform a save for `slot_id`.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[archive(compare(PartialEq))]
pub struct SaveRequestRpc {
    /// Target slot for this save attempt.
    pub slot_id: SlotId,
    /// Kind of save the server should schedule.
    pub save_type: SaveType,
}

/// Server-to-client response after processing a save request.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[archive(compare(PartialEq))]
pub struct SaveResponseRpc {
    /// Slot this response refers to.
    pub slot_id: SlotId,
    /// Outcome of the save attempt.
    pub result: SaveRpcResult,
}

/// Announces a multiplayer checkpoint and lists required ACK peers.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[archive(compare(PartialEq))]
pub struct CheckpointPrepareRpc {
    /// Monotonic checkpoint identifier chosen by the server.
    pub checkpoint_id: u64,
    /// Peers that must ACK before the save may proceed.
    pub participants: Vec<ConnectionId>,
}

/// Client ACK that it reached the checkpoint barrier.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[archive(compare(PartialEq))]
pub struct CheckpointReadyRpc {
    /// Matches `CheckpointPrepareRpc::checkpoint_id`.
    pub checkpoint_id: u64,
}

/// Reason a checkpoint sequence was aborted server-side.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[archive(compare(PartialEq))]
pub enum CheckpointAbortReason {
    /// Participant set changed or a peer dropped mid-checkpoint.
    PeerDisconnect,
    /// Wall-clock timeout waiting for all ACKs.
    Timeout,
    /// Operator or gameplay cancelled the checkpoint.
    ServerCancelled,
}

/// Server broadcast when a checkpoint cannot complete.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[archive(compare(PartialEq))]
pub struct CheckpointAbortRpc {
    /// Matches the in-flight checkpoint id.
    pub checkpoint_id: u64,
    /// Why the checkpoint was torn down.
    pub reason: CheckpointAbortReason,
}
