//! Non-blocking cloud I/O submission envelope (IR-4.6.3).

use std::sync::Arc;

use crate::wire::{SaveError, SlotId};

/// Bounded work item handed to the main-thread I/O worker (design IR-4.6.3).
///
/// The discriminant plus inline payload must stay within the 32-byte budget from
/// `TC-IR-4.6.U7` on supported 64-bit targets.
#[derive(Debug)]
pub enum CloudIoRequest {
    /// Query remote metadata for `slot`.
    Query {
        /// Slot to query.
        slot: SlotId,
    },
    /// Upload compressed save bytes for `slot`.
    Upload {
        /// Slot being uploaded.
        slot: SlotId,
        /// Immutable payload shared with the worker thread.
        bytes: Arc<[u8]>,
    },
    /// Download remote bytes for `slot`.
    Download {
        /// Slot to pull down.
        slot: SlotId,
    },
}

/// Capacity for the `CloudIoRequest` MPSC queue (design + `TC-IR-4.6.N2`).
pub const CLOUD_IO_REQUEST_CAPACITY: usize = 256;

/// Submit-side handle for cloud I/O work (design `CloudSyncAdapter` channel).
pub type CloudIoSender = crossbeam_channel::Sender<CloudIoRequest>;

/// Consumer handle polled on the main thread.
pub type CloudIoReceiver = crossbeam_channel::Receiver<CloudIoRequest>;

/// Opens a bounded `CloudIoRequest` channel with production capacity.
#[must_use]
pub fn cloud_io_channel() -> (CloudIoSender, CloudIoReceiver) {
    crossbeam_channel::bounded(CLOUD_IO_REQUEST_CAPACITY)
}

/// Submits `request` without blocking; maps a full queue to [`SaveError::CloudUnavailable`].
pub fn try_submit_cloud_io(
    sender: &CloudIoSender,
    request: CloudIoRequest,
) -> Result<(), SaveError> {
    sender
        .try_send(request)
        .map_err(|_| SaveError::CloudUnavailable)
}
