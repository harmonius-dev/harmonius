//! Telemetry client orchestrating consent, buffering, and uploads.

use std::path::{Path, PathBuf};

use crate::backoff::ExponentialBackoff;
use crate::blob::BlobWriter;
use crate::consent::ConsentState;
use crate::error::{BufferError, TelemetryError};
use crate::event::Event;
use crate::ring_buffer::{list_spill_files, read_disk_spill, EventRecord, RingBuffer};
use crate::schema::SchemaCatalog;
use crate::types::{AnonId, Scope};
use crate::uploader::{DeleteBackend, NoopDeleteBackend, NoopUploader, Uploader};

/// Bundle returned by [`TelemetryClient::export_local`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportBundle {
    /// All known rows for the active anonymous id.
    pub events: Vec<EventRecord>,
    /// Anonymous id at export time.
    pub anonymous_id: AnonId,
    /// Export timestamp in milliseconds (tests may use `0`).
    pub exported_at_ms: u64,
}

/// Receipt returned after a successful delete round-trip.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeleteReceipt {
    /// Anonymous id that was erased remotely.
    pub anonymous_id: AnonId,
    /// Local wall clock when delete completed.
    pub deleted_at_ms: u64,
    /// Whether the backend acknowledged the delete.
    pub server_ack: bool,
}

/// Client configuration for disk layout and capacity.
#[derive(Clone, Debug)]
pub struct TelemetryConfig {
    /// Maximum in-memory records before hard `Err`.
    pub memory_cap: usize,
    /// Directory for spill files and replayed batches.
    pub buffer_dir: PathBuf,
}

impl TelemetryConfig {
    /// Sensible defaults for unit tests (small memory, tempdir caller supplies path).
    pub fn for_tests(buffer_dir: PathBuf) -> Self {
        Self {
            memory_cap: 16,
            buffer_dir,
        }
    }
}

/// Primary telemetry façade (design `TelemetryClient`).
pub struct TelemetryClient<U: Uploader = NoopUploader, D: DeleteBackend = NoopDeleteBackend> {
    state: ConsentState,
    memory: RingBuffer,
    schemas: SchemaCatalog,
    config: TelemetryConfig,
    uploader: U,
    delete_backend: D,
    backoff: ExponentialBackoff,
}

impl<U: Uploader, D: DeleteBackend> TelemetryClient<U, D> {
    /// Construct a client, loading any spill files present under `config.buffer_dir`.
    pub fn new(
        state: ConsentState,
        schemas: SchemaCatalog,
        config: TelemetryConfig,
        uploader: U,
        delete_backend: D,
    ) -> Result<Self, TelemetryError> {
        let spill_threshold = (config.memory_cap * 75).div_ceil(100).max(1);
        let mut memory = RingBuffer::new(config.memory_cap, spill_threshold);
        Self::replay_disk_spills(&config.buffer_dir, &mut memory)?;
        Ok(Self {
            state,
            memory,
            schemas,
            config,
            uploader,
            delete_backend,
            backoff: ExponentialBackoff::new(100, 800),
        })
    }

    fn replay_disk_spills(dir: &Path, memory: &mut RingBuffer) -> Result<(), TelemetryError> {
        if !dir.exists() {
            return Ok(());
        }
        for file in list_spill_files(dir)? {
            let rows = read_disk_spill(&file)?;
            for row in rows {
                memory
                    .push(row)
                    .map_err(|_| TelemetryError::Config("replay overflow"))?;
            }
        }
        Ok(())
    }

    /// Current consent snapshot.
    pub fn consent(&self) -> ConsentState {
        self.state.clone()
    }

    /// Update consent flags; marks the first-run prompt as completed.
    pub fn set_consent(&mut self, scope: Scope, enabled: bool) {
        self.state.first_run_ack = true;
        match scope {
            Scope::Engine => self.state.engine_scope = enabled,
            Scope::GameLogic => self.state.game_scope = enabled,
        }
    }

    /// Record an event if consent allows the scope.
    pub fn record<E: Event>(&mut self, event: &E) {
        if !self.state.consent_for(E::SCOPE) {
            return;
        }
        let mut writer = BlobWriter::with_capacity(64);
        event.archive(&mut writer);
        let record = EventRecord {
            schema_id: E::SCHEMA_ID,
            timestamp_ms: 0,
            scope: E::SCOPE,
            payload: writer.finish(),
        };
        let _ = self.memory.push(record);
    }

    /// Force disk spill of the entire in-memory queue.
    pub fn flush_memory_to_disk(&mut self, seq: u64) -> Result<(), TelemetryError> {
        let path = self.config.buffer_dir.join(format!("{seq}.jsonl"));
        self.memory.flush_to_disk(&path)
    }

    /// Drain exactly `n` records and upload them, restoring the buffer on transport failure.
    pub fn send_batch_exact(&mut self, n: usize) -> Result<(), TelemetryError> {
        let batch = self.memory.try_drain_exact(n).map_err(|e| {
            TelemetryError::Config(match e {
                BufferError::NotEnoughRecords => "not enough records for exact batch",
                BufferError::Full => "unexpected full buffer while draining",
            })
        })?;
        match self.uploader.send_batch(&batch) {
            Ok(()) => {
                self.backoff.reset(100);
                Ok(())
            }
            Err(err) => {
                for row in batch.into_iter().rev() {
                    self.memory.push_front(row).map_err(|_| {
                        TelemetryError::Config("failed to restore batch after upload error")
                    })?;
                }
                let _ = self.backoff.next_delay_ms();
                Err(TelemetryError::Upload(err.to_string()))
            }
        }
    }

    /// Local export merging memory and on-disk spill files.
    pub fn export_local(&self) -> Result<ExportBundle, TelemetryError> {
        let mut events = self.memory.snapshot();
        if self.config.buffer_dir.exists() {
            for file in list_spill_files(&self.config.buffer_dir)? {
                let mut rows = read_disk_spill(&file)?;
                events.append(&mut rows);
            }
        }
        Ok(ExportBundle {
            anonymous_id: self.state.anonymous_id,
            exported_at_ms: 0,
            events,
        })
    }

    /// Delete remote data, wipe local buffers, and rotate the anonymous id.
    pub fn delete(&mut self) -> Result<DeleteReceipt, TelemetryError> {
        let old = self.state.anonymous_id;
        self.delete_backend.delete_remote(&old)?;
        self.memory.clear();
        if self.config.buffer_dir.exists() {
            for file in list_spill_files(&self.config.buffer_dir)? {
                std::fs::remove_file(&file).map_err(|e| TelemetryError::Io(e.to_string()))?;
            }
        }
        self.state.anonymous_id = AnonId::random();
        Ok(DeleteReceipt {
            anonymous_id: old,
            deleted_at_ms: 0,
            server_ack: true,
        })
    }

    /// Access the schema catalog for diagnostics.
    pub fn schemas(&self) -> &SchemaCatalog {
        &self.schemas
    }

    /// Test hook: memory length.
    pub fn memory_len(&self) -> usize {
        self.memory.len()
    }

    /// Test hook: spill counter on the ring buffer.
    pub fn spill_count(&self) -> u32 {
        self.memory.spill_count()
    }

    /// Test hook: expose uploader mutably.
    pub fn uploader_mut(&mut self) -> &mut U {
        &mut self.uploader
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list_spill_files;
    use crate::schema::{EventSchema, SchemaCatalog};
    use crate::types::SchemaId;
    use crate::uploader::MockUploader;
    use tempfile::tempdir;

    struct EnginePing;
    impl Event for EnginePing {
        const SCHEMA_ID: u32 = 1;
        const SCOPE: Scope = Scope::Engine;
        fn archive(&self, out: &mut BlobWriter) {
            out.write_all(b"e");
        }
    }

    struct GamePing;
    impl Event for GamePing {
        const SCHEMA_ID: u32 = 2;
        const SCOPE: Scope = Scope::GameLogic;
        fn archive(&self, out: &mut BlobWriter) {
            out.write_all(b"g");
        }
    }

    fn catalog() -> SchemaCatalog {
        SchemaCatalog::from_schemas(vec![
            EventSchema {
                id: SchemaId(1),
                name: "EnginePing".into(),
                scope: Scope::Engine,
                fields: vec![],
            },
            EventSchema {
                id: SchemaId(2),
                name: "GamePing".into(),
                scope: Scope::GameLogic,
                fields: vec![],
            },
        ])
        .unwrap()
    }

    #[test]
    fn test_first_run_prompt_once_per_install() {
        let dir = tempdir().unwrap();
        let mut client = TelemetryClient::new(
            ConsentState::fresh(),
            catalog(),
            TelemetryConfig::for_tests(dir.path().to_path_buf()),
            NoopUploader,
            NoopDeleteBackend,
        )
        .unwrap();
        assert!(client.consent().is_first_run());
        client.set_consent(Scope::Engine, true);
        assert!(!client.consent().is_first_run());
    }

    #[test]
    fn test_engine_scope_drops_game_events() {
        let dir = tempdir().unwrap();
        let mut client = TelemetryClient::new(
            ConsentState::fresh(),
            catalog(),
            TelemetryConfig::for_tests(dir.path().to_path_buf()),
            NoopUploader,
            NoopDeleteBackend,
        )
        .unwrap();
        client.set_consent(Scope::Engine, true);
        client.record(&GamePing);
        client.record(&EnginePing);
        assert_eq!(client.memory_len(), 1);
    }

    #[test]
    fn test_game_scope_drops_engine_events() {
        let dir = tempdir().unwrap();
        let mut client = TelemetryClient::new(
            ConsentState::fresh(),
            catalog(),
            TelemetryConfig::for_tests(dir.path().to_path_buf()),
            NoopUploader,
            NoopDeleteBackend,
        )
        .unwrap();
        client.set_consent(Scope::GameLogic, true);
        client.record(&EnginePing);
        client.record(&GamePing);
        assert_eq!(client.memory_len(), 1);
    }

    #[test]
    fn test_both_scopes_records_all() {
        let dir = tempdir().unwrap();
        let mut client = TelemetryClient::new(
            ConsentState::fresh(),
            catalog(),
            TelemetryConfig::for_tests(dir.path().to_path_buf()),
            NoopUploader,
            NoopDeleteBackend,
        )
        .unwrap();
        client.set_consent(Scope::Engine, true);
        client.set_consent(Scope::GameLogic, true);
        client.record(&EnginePing);
        client.record(&GamePing);
        assert_eq!(client.memory_len(), 2);
    }

    #[test]
    fn test_neither_scope_records_none() {
        let dir = tempdir().unwrap();
        let mut client = TelemetryClient::new(
            ConsentState::fresh(),
            catalog(),
            TelemetryConfig::for_tests(dir.path().to_path_buf()),
            NoopUploader,
            NoopDeleteBackend,
        )
        .unwrap();
        client.record(&EnginePing);
        client.record(&GamePing);
        assert_eq!(client.memory_len(), 0);
    }

    #[test]
    fn test_batch_drains_buffer_atomically() {
        let dir = tempdir().unwrap();
        let mut client = TelemetryClient::new(
            ConsentState::fresh(),
            catalog(),
            TelemetryConfig::for_tests(dir.path().to_path_buf()),
            NoopUploader,
            NoopDeleteBackend,
        )
        .unwrap();
        client.set_consent(Scope::Engine, true);
        client.record(&EnginePing);
        client.record(&EnginePing);
        assert!(client.send_batch_exact(3).is_err());
        assert_eq!(client.memory_len(), 2);
        client.send_batch_exact(2).unwrap();
        assert_eq!(client.memory_len(), 0);
    }

    #[test]
    fn test_failed_upload_retains_batch() {
        let dir = tempdir().unwrap();
        let mut client = TelemetryClient::new(
            ConsentState::fresh(),
            catalog(),
            TelemetryConfig::for_tests(dir.path().to_path_buf()),
            MockUploader {
                fails_remaining: 1,
                batches: Vec::new(),
            },
            NoopDeleteBackend,
        )
        .unwrap();
        client.set_consent(Scope::Engine, true);
        client.record(&EnginePing);
        assert!(client.send_batch_exact(1).is_err());
        assert_eq!(client.memory_len(), 1);
        assert!(client.send_batch_exact(1).is_ok());
        assert_eq!(client.memory_len(), 0);
    }

    #[test]
    fn test_export_bundle_contains_records() {
        let dir = tempdir().unwrap();
        let mut client = TelemetryClient::new(
            ConsentState::fresh(),
            catalog(),
            TelemetryConfig::for_tests(dir.path().to_path_buf()),
            NoopUploader,
            NoopDeleteBackend,
        )
        .unwrap();
        client.set_consent(Scope::Engine, true);
        client.record(&EnginePing);
        client.record(&EnginePing);
        let bundle = client.export_local().unwrap();
        assert_eq!(bundle.events.len(), 2);
        assert_eq!(bundle.anonymous_id, client.consent().anonymous_id);
    }

    #[test]
    fn test_delete_clears_local_buffer() {
        let dir = tempdir().unwrap();
        let mut client = TelemetryClient::new(
            ConsentState::fresh(),
            catalog(),
            TelemetryConfig::for_tests(dir.path().to_path_buf()),
            NoopUploader,
            NoopDeleteBackend,
        )
        .unwrap();
        client.set_consent(Scope::Engine, true);
        client.record(&EnginePing);
        client.flush_memory_to_disk(1).unwrap();
        assert!(!list_spill_files(&client.config.buffer_dir)
            .unwrap()
            .is_empty());
        client.delete().unwrap();
        assert_eq!(client.memory_len(), 0);
        assert!(list_spill_files(&client.config.buffer_dir)
            .unwrap()
            .is_empty());
    }

    #[test]
    fn test_delete_rotates_anonymous_id() {
        let dir = tempdir().unwrap();
        let mut client = TelemetryClient::new(
            ConsentState::fresh(),
            catalog(),
            TelemetryConfig::for_tests(dir.path().to_path_buf()),
            NoopUploader,
            NoopDeleteBackend,
        )
        .unwrap();
        let before = client.consent().anonymous_id;
        client.delete().unwrap();
        let after = client.consent().anonymous_id;
        assert_ne!(before, after);
    }
}
