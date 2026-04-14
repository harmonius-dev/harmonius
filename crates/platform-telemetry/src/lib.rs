//! Harmonius platform telemetry reference client.
//!
//! Implements consent gating, bounded buffering with disk spill, batched uploads,
//! schema catalogs, and GDPR-oriented export/delete flows described in
//! `docs/design/platform/telemetry.md`.

#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![allow(missing_docs)]

mod macros;

pub mod backoff;
pub mod blob;
pub mod client;
pub mod consent;
pub mod counter;
pub mod error;
pub mod event;
pub mod ring_buffer;
pub mod schema;
pub mod types;
pub mod uploader;

pub use backoff::ExponentialBackoff;
pub use blob::BlobWriter;
pub use client::{DeleteReceipt, ExportBundle, TelemetryClient, TelemetryConfig};
pub use consent::{load_consent, save_consent, ConsentState};
pub use counter::CounterRegistry;
pub use error::{BufferError, CounterError, TelemetryError, UploadError};
pub use event::{Event, TaggedPayload};
pub use ring_buffer::{list_spill_files, read_disk_spill, EventRecord, RingBuffer};
pub use schema::{archive_event_wired, EventSchema, FieldDescriptor, FieldKind, SchemaCatalog};
pub use types::{AnonId, PiiClass, SchemaId, Scope};
pub use uploader::{DeleteBackend, MockUploader, NoopDeleteBackend, NoopUploader, Uploader};
