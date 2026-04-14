//! MPSC bridge between network receive (phase 2) and save execution (phase 8).

use crossbeam_channel::{Receiver, Sender, TrySendError};

use crate::wire::{ConnectionId, SaveError, SaveRequestRpc};

/// Maximum queued save RPC envelopes (`TC-IR-4.6.N1`, design channel table).
pub const SAVE_RPC_BRIDGE_CAPACITY: usize = 64;

/// Messages crossing the network → save boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SaveRpcMsg {
    /// Authoritative save work requested by `who` with payload `rpc`.
    Request {
        /// Connection that issued the RPC.
        who: ConnectionId,
        /// Decoded request body.
        rpc: SaveRequestRpc,
    },
    /// Client reported readiness for `checkpoint_id`.
    CheckpointReady {
        /// Connection that acknowledged.
        who: ConnectionId,
        /// Checkpoint identifier from `CheckpointPrepareRpc`.
        checkpoint_id: u64,
    },
}

/// Bounded MPSC queue from RPC dispatch to `SaveManager`.
#[derive(Debug)]
pub struct SaveRpcBridge {
    tx: Sender<SaveRpcMsg>,
    rx: Receiver<SaveRpcMsg>,
}

impl SaveRpcBridge {
    /// Builds a bridge with the Harmonius-standard capacity of 64 messages.
    #[must_use]
    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::bounded(SAVE_RPC_BRIDGE_CAPACITY);
        Self { tx, rx }
    }

    /// Clones the producer handle for additional dispatch threads.
    #[must_use]
    pub fn sender(&self) -> Sender<SaveRpcMsg> {
        self.tx.clone()
    }

    /// Non-blocking enqueue used from the network thread (`TC-IR-4.6.N1`).
    ///
    /// A full queue surfaces [`SaveError::Io`] per integration failure matrix F2.
    pub fn try_push(&self, msg: SaveRpcMsg) -> Result<(), SaveError> {
        self.tx.try_send(msg).map_err(|e| match e {
            TrySendError::Full(_) | TrySendError::Disconnected(_) => SaveError::Io,
        })
    }

    /// Drains every currently queued message in FIFO order for the consumer.
    #[must_use]
    pub fn drain(&self) -> Vec<SaveRpcMsg> {
        let mut out = Vec::new();
        while let Ok(msg) = self.rx.try_recv() {
            out.push(msg);
        }
        out
    }
}

impl Default for SaveRpcBridge {
    fn default() -> Self {
        Self::new()
    }
}
