//! Wire types shared between the editor shell and the asset import worker.

use std::path::PathBuf;
use std::time::SystemTime;

/// Stable asset identifier assigned after a successful import.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AssetId(pub u64);

/// Groups folder-drop imports for rollup progress (IR-9.2.6).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ImportBatchId(pub u32);

/// High-level asset category for routing importers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImportKind {
    Animation,
    Audio,
    Font,
    Material,
    Mesh,
    Scene,
    Shader,
    Texture,
}

/// Texture compression mode placeholder (design contract surface).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompressionMode {
    None,
    Bc7,
}

/// Per-import tuning flags carried with the request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportOptions {
    pub compression: CompressionMode,
    pub force_reimport: bool,
    pub generate_mips: bool,
}

impl Default for ImportOptions {
    fn default() -> Self {
        Self {
            compression: CompressionMode::None,
            force_reimport: false,
            generate_mips: false,
        }
    }
}

/// Editor → worker import job envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetImportRequest {
    pub batch_id: ImportBatchId,
    pub kind: ImportKind,
    pub only_if_stale: bool,
    pub options: ImportOptions,
    pub request_id: u64,
    pub source_path: PathBuf,
    /// When set, `only_if_stale` compares this against the last recorded watcher time (IR-9.2.5).
    pub watcher_mtime: Option<SystemTime>,
}

/// Terminal status for an import attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImportOutcome {
    AlreadyUpToDate,
    Cancelled,
    Failed,
    Success,
}

/// Worker → editor completion payload.
#[derive(Clone, Debug, PartialEq)]
pub struct AssetImportResult {
    pub asset_id: AssetId,
    pub batch_id: ImportBatchId,
    pub duration_s: f64,
    pub outcome: ImportOutcome,
    pub request_id: u64,
}

/// Fine-grained import stage for HUD progress (IR-9.2.4).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImportStage {
    Bake,
    Compress,
    Link,
    Parse,
    Process,
    Read,
    Upload,
}

/// Progress tick emitted while an import runs.
#[derive(Clone, Debug, PartialEq)]
pub struct AssetImportProgress {
    pub batch_id: ImportBatchId,
    pub eta_s: f64,
    pub percent: u8,
    pub request_id: u64,
    pub stage: ImportStage,
}

/// Which worlds should observe a reload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReloadScope {
    Both,
    EditorOnly,
    RuntimeOnly,
}

/// Signals that baked assets were swapped and views should refresh.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetReloadEvent {
    pub asset_id: AssetId,
    pub scope: ReloadScope,
}

/// File watcher notification forwarded into the editor (IR-9.2.5).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceChangedEvent {
    pub mtime: SystemTime,
    pub source_path: PathBuf,
}
