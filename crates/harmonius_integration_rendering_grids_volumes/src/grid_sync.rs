//! Worker ↔ render upload queue (`GpuGridSync`).

use crossbeam_channel::{bounded, Receiver, SendError, Sender, TrySendError};

use crate::types::{GridAssetHandle, GridUploadPriority, GridUploadStatus};
use crate::GpuTextureHandle;

/// One partial-grid upload command (CPU-owned tile list).
#[derive(Clone, Debug, PartialEq)]
pub struct GridUploadCommand {
    /// Source grid asset slot.
    pub handle: GridAssetHandle,
    /// Destination GPU texture slot.
    pub texture: GpuTextureHandle,
    /// Dirty tile indices in ascending order (see `DirtyRegionSet`).
    pub dirty_tiles: Vec<u32>,
    /// Scheduling priority.
    pub priority: GridUploadPriority,
}

/// Typed buffer filled when the render thread drains the MPSC channel.
#[derive(Debug, Default)]
pub struct GridUploadQueue {
    commands: Vec<GridUploadCommand>,
}

impl GridUploadQueue {
    /// Returns queued commands in drain order.
    #[must_use]
    pub fn as_slice(&self) -> &[GridUploadCommand] {
        &self.commands
    }

    /// Clears all drained commands.
    pub fn clear(&mut self) {
        self.commands.clear();
    }

    fn push(&mut self, cmd: GridUploadCommand) {
        self.commands.push(cmd);
    }
}

enum GridSyncEnd {
    Send(Sender<GridUploadCommand>),
    Recv(Receiver<GridUploadCommand>),
}

/// Design-facing facade for the worker→render `grid_upload_commands` channel.
///
/// Construct with [`Self::new_worker`] on the simulation side and
/// [`Self::new_render`] on the render thread. Wrong-side calls fail gracefully.
pub struct GpuGridSync {
    end: GridSyncEnd,
}

impl GpuGridSync {
    /// Channel capacity per integration design (128 commands).
    pub const CHANNEL_CAPACITY: usize = 128;

    /// Opens a bounded MPSC pair sharing the configured capacity.
    pub fn channel_pair() -> (Self, Self) {
        let (tx, rx) = bounded(Self::CHANNEL_CAPACITY);
        (Self::new_worker(tx), Self::new_render(rx))
    }

    /// Worker-side endpoint (producer).
    #[must_use]
    pub fn new_worker(sender: Sender<GridUploadCommand>) -> Self {
        Self {
            end: GridSyncEnd::Send(sender),
        }
    }

    /// Render-side endpoint (consumer).
    #[must_use]
    pub fn new_render(receiver: Receiver<GridUploadCommand>) -> Self {
        Self {
            end: GridSyncEnd::Recv(receiver),
        }
    }

    /// Enqueues a command from the worker. Returns [`GridUploadStatus::Failed`]
    /// when invoked on the render endpoint or when the channel is full.
    pub fn enqueue(&mut self, cmd: GridUploadCommand) -> GridUploadStatus {
        match &self.end {
            GridSyncEnd::Send(tx) => match tx.try_send(cmd) {
                Ok(()) => GridUploadStatus::Queued,
                Err(TrySendError::Full(_)) => GridUploadStatus::Failed,
                Err(TrySendError::Disconnected(_)) => GridUploadStatus::Failed,
            },
            GridSyncEnd::Recv(_) => GridUploadStatus::Failed,
        }
    }

    /// Blocking enqueue for tests / back-pressure experiments.
    pub fn enqueue_blocking(&mut self, cmd: GridUploadCommand) -> GridUploadStatus {
        match &self.end {
            GridSyncEnd::Send(tx) => match tx.send(cmd) {
                Ok(()) => GridUploadStatus::Queued,
                Err(SendError(_)) => GridUploadStatus::Failed,
            },
            GridSyncEnd::Recv(_) => GridUploadStatus::Failed,
        }
    }

    /// Drains every pending command into `queue` (render thread, Phase 7).
    pub fn drain_into(&self, queue: &mut GridUploadQueue) {
        let GridSyncEnd::Recv(rx) = &self.end else {
            return;
        };
        while let Ok(cmd) = rx.try_recv() {
            queue.push(cmd);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GpuGridSync, GridUploadCommand, GridUploadQueue};
    use crate::types::{GridAssetHandle, GridUploadPriority};
    use crate::GpuTextureHandle;

    /// TC-IR-3.3.X.1 — worker command reaches render-side drain.
    #[test]
    fn mpsc_worker_to_render_drain() {
        let (mut worker, render) = GpuGridSync::channel_pair();
        let cmd = GridUploadCommand {
            handle: GridAssetHandle {
                index: 0,
                generation: 0,
            },
            texture: GpuTextureHandle {
                index: 1,
                generation: 0,
            },
            dirty_tiles: vec![0, 1, 2],
            priority: GridUploadPriority::Normal,
        };
        assert_eq!(worker.enqueue(cmd.clone()), crate::GridUploadStatus::Queued);
        let mut q = GridUploadQueue::default();
        render.drain_into(&mut q);
        assert_eq!(q.as_slice(), &[cmd]);
    }
}
