//! Upload transport hook (R-14.5.4).

use crate::error::UploadError;
use crate::ring_buffer::EventRecord;

/// Transport for batched telemetry rows.
pub trait Uploader {
    /// Upload a single batch; failures must leave remote state unchanged for safe retry.
    fn send_batch(&mut self, batch: &[EventRecord]) -> Result<(), UploadError>;
}

/// Default uploader that discards batches (offline / tests).
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopUploader;

impl Uploader for NoopUploader {
    fn send_batch(&mut self, _batch: &[EventRecord]) -> Result<(), UploadError> {
        Ok(())
    }
}

/// Deterministic uploader that fails the first N attempts to exercise retry paths.
#[derive(Debug, Default)]
pub struct MockUploader {
    /// Remaining simulated transport failures.
    pub fails_remaining: usize,
    /// Observed batches for assertions.
    pub batches: Vec<Vec<EventRecord>>,
}

impl Uploader for MockUploader {
    fn send_batch(&mut self, batch: &[EventRecord]) -> Result<(), UploadError> {
        if self.fails_remaining > 0 {
            self.fails_remaining -= 1;
            return Err(UploadError("simulated 5xx".into()));
        }
        self.batches.push(batch.to_vec());
        Ok(())
    }
}

/// Remote delete contract used by [`crate::TelemetryClient::delete`].
pub trait DeleteBackend {
    /// Ask the backend to erase remote copies for the anonymous id.
    fn delete_remote(
        &mut self,
        id: &crate::types::AnonId,
    ) -> Result<(), crate::error::TelemetryError>;
}

/// Successful no-op delete backend.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopDeleteBackend;

impl DeleteBackend for NoopDeleteBackend {
    fn delete_remote(
        &mut self,
        _id: &crate::types::AnonId,
    ) -> Result<(), crate::error::TelemetryError> {
        Ok(())
    }
}
