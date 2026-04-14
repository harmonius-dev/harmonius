//! Headless harness for editor ↔ asset pipeline integration scenarios described in
//! `docs/design/integration/editor-asset-pipeline.md` and exercised by
//! `docs/design/integration/editor-asset-pipeline-test-cases.md`.

#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod contracts;
mod fake_fs;
mod fake_importer;
mod harness;

pub use contracts::{
    AssetId, AssetImportProgress, AssetImportRequest, AssetImportResult, AssetReloadEvent,
    CompressionMode, ImportBatchId, ImportKind, ImportOptions, ImportOutcome, ImportStage,
    ReloadScope, SourceChangedEvent,
};
pub use fake_fs::{FakeFileSystem, IoError};
pub use fake_importer::{FakeImportError, FakeImporter};
pub use harness::{FallbackCounters, HeadlessEditorHarness};
