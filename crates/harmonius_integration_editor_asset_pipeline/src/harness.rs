//! Minimal editor + worker loop for CI integration tests.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::contracts::{
    AssetId, AssetImportProgress, AssetImportRequest, AssetImportResult, AssetReloadEvent,
    ImportKind, ImportOptions, ImportOutcome, ImportStage, ReloadScope, SourceChangedEvent,
};
use crate::fake_fs::{FakeFileSystem, IoError};
use crate::fake_importer::{FakeImportError, FakeImporter};

/// Counters for fallback modes FM-1 … FM-7 (incremented by the harness).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FallbackCounters {
    pub fm1_source_read_error: u64,
    pub fm2_parse_error_keeps_previous: u64,
}

/// Headless editor slice: enqueue, worker drain, HUD banners, telemetry.
#[derive(Debug)]
pub struct HeadlessEditorHarness {
    asset_sequence: u64,
    pub banners: Vec<String>,
    pub counters: FallbackCounters,
    pub fs: FakeFileSystem,
    pub importer: FakeImporter,
    last_hashes: HashMap<PathBuf, u64>,
    pub progress: Vec<AssetImportProgress>,
    pub registry: HashMap<PathBuf, AssetId>,
    pub reload_events: Vec<AssetReloadEvent>,
    request_sequence: u64,
    pub requests: Vec<AssetImportRequest>,
    pub results: Vec<AssetImportResult>,
}

impl Default for HeadlessEditorHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadlessEditorHarness {
    pub fn new() -> Self {
        Self {
            asset_sequence: 1,
            banners: Vec::new(),
            counters: FallbackCounters::default(),
            fs: FakeFileSystem::default(),
            importer: FakeImporter::new(),
            last_hashes: HashMap::new(),
            progress: Vec::new(),
            registry: HashMap::new(),
            reload_events: Vec::new(),
            request_sequence: 1,
            requests: Vec::new(),
            results: Vec::new(),
        }
    }

    /// Simulates dropping a single file into the asset browser (IR-9.2.1).
    pub fn drop_file(&mut self, path: &str) -> u64 {
        let source_path = PathBuf::from(path);
        let kind = infer_kind(&source_path);
        let request_id = self.request_sequence;
        self.request_sequence += 1;
        self.requests.push(AssetImportRequest {
            batch_id: crate::contracts::ImportBatchId(0),
            kind,
            only_if_stale: false,
            options: ImportOptions::default(),
            request_id,
            source_path,
        });
        request_id
    }

    /// Handles `SourceChangedEvent` by enqueueing a stale-gated import (IR-9.2.5).
    pub fn on_source_changed(&mut self, event: SourceChangedEvent) -> u64 {
        let request_id = self.request_sequence;
        self.request_sequence += 1;
        let kind = infer_kind(&event.source_path);
        self.requests.push(AssetImportRequest {
            batch_id: crate::contracts::ImportBatchId(0),
            kind,
            only_if_stale: true,
            options: ImportOptions::default(),
            request_id,
            source_path: event.source_path,
        });
        request_id
    }

    /// Runs the synthetic import worker until the queue is empty.
    pub fn drain_worker(&mut self) {
        let queue = std::mem::take(&mut self.requests);
        for job in queue {
            self.run_one(job);
        }
    }

    fn run_one(&mut self, job: AssetImportRequest) {
        let start = std::time::Instant::now();
        self.emit_progress_stages(&job);

        let read = self.fs.read(job.source_path.as_path());
        let bytes = match read {
            Ok(b) => b,
            Err(IoError::PermissionDenied) => {
                self.counters.fm1_source_read_error += 1;
                self.results.push(AssetImportResult {
                    asset_id: AssetId(0),
                    duration_s: start.elapsed().as_secs_f64(),
                    outcome: ImportOutcome::Failed,
                    request_id: job.request_id,
                });
                self.banners
                    .push(format!("import failed: permission denied ({})", job.source_path.display()));
                return;
            }
            Err(IoError::NotFound) => {
                self.results.push(AssetImportResult {
                    asset_id: AssetId(0),
                    duration_s: start.elapsed().as_secs_f64(),
                    outcome: ImportOutcome::Failed,
                    request_id: job.request_id,
                });
                self.banners
                    .push(format!("import failed: source missing ({})", job.source_path.display()));
                return;
            }
        };

        let content_hash = hash_bytes(&bytes);
        if job.only_if_stale {
            if let Some(prev) = self.last_hashes.get(&job.source_path) {
                if *prev == content_hash {
                    self.results.push(AssetImportResult {
                        asset_id: self
                            .registry
                            .get(&job.source_path)
                            .copied()
                            .unwrap_or(AssetId(0)),
                        duration_s: start.elapsed().as_secs_f64(),
                        outcome: ImportOutcome::AlreadyUpToDate,
                        request_id: job.request_id,
                    });
                    return;
                }
            }
        }

        match self
            .importer
            .import(job.source_path.as_path(), job.kind, &job.options, &bytes)
        {
            Ok(_blob) => {
                let asset_id = AssetId(self.asset_sequence);
                self.asset_sequence += 1;
                self.registry.insert(job.source_path.clone(), asset_id);
                self.last_hashes.insert(job.source_path.clone(), content_hash);
                self.results.push(AssetImportResult {
                    asset_id,
                    duration_s: start.elapsed().as_secs_f64(),
                    outcome: ImportOutcome::Success,
                    request_id: job.request_id,
                });
                self.reload_events.push(AssetReloadEvent {
                    asset_id,
                    scope: ReloadScope::EditorOnly,
                });
            }
            Err(FakeImportError::Parse) => {
                self.counters.fm2_parse_error_keeps_previous += 1;
                self.results.push(AssetImportResult {
                    asset_id: self
                        .registry
                        .get(&job.source_path)
                        .copied()
                        .unwrap_or(AssetId(0)),
                    duration_s: start.elapsed().as_secs_f64(),
                    outcome: ImportOutcome::Failed,
                    request_id: job.request_id,
                });
                self.banners.push(format!(
                    "import failed: parse error ({})",
                    job.source_path.display()
                ));
            }
        }
    }

    fn emit_progress_stages(&mut self, job: &AssetImportRequest) {
        let stages = [
            ImportStage::Read,
            ImportStage::Parse,
            ImportStage::Process,
            ImportStage::Bake,
            ImportStage::Link,
            ImportStage::Upload,
        ];
        let mut eta = 1.0_f64;
        for (idx, stage) in stages.iter().enumerate() {
            let percent = (((idx + 1) * 100) / stages.len()) as u8;
            eta = (eta - 0.1).max(0.0);
            self.progress.push(AssetImportProgress {
                eta_s: eta,
                percent,
                request_id: job.request_id,
                stage: *stage,
            });
        }
    }
}

fn infer_kind(path: &Path) -> ImportKind {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "png" | "jpg" | "jpeg" | "tga" | "dds" => ImportKind::Texture,
        "glb" | "gltf" | "obj" | "fbx" => ImportKind::Mesh,
        "wav" | "flac" | "ogg" => ImportKind::Audio,
        "ttf" | "otf" => ImportKind::Font,
        "hlsl" | "shader" => ImportKind::Shader,
        "mat" => ImportKind::Material,
        "anim" => ImportKind::Animation,
        "scene" => ImportKind::Scene,
        _ => ImportKind::Texture,
    }
}

fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn tc_ir_9_2_1_1_file_drop_enqueues_import_request() {
        let mut harness = HeadlessEditorHarness::new();
        harness.drop_file("tex.png");
        assert_eq!(harness.requests.len(), 1);
        let req = &harness.requests[0];
        assert_eq!(req.kind, ImportKind::Texture);
        assert_eq!(req.source_path, PathBuf::from("tex.png"));
        assert!(!req.only_if_stale);
    }

    #[test]
    fn tc_ir_9_2_1_2_import_success_publishes_reload_event() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("tex.png", vec![0x89, 0x50]);
        harness.drop_file("tex.png");
        harness.drain_worker();
        let last = harness.results.last().expect("result");
        assert_eq!(last.outcome, ImportOutcome::Success);
        let reload = harness.reload_events.last().expect("reload");
        assert_eq!(reload.scope, ReloadScope::EditorOnly);
        assert_eq!(reload.asset_id, last.asset_id);
    }

    #[test]
    fn tc_ir_9_2_1_3_import_failure_publishes_banner() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("bad.png", vec![0]);
        harness.importer.fail_parse_for("bad.png");
        harness.drop_file("bad.png");
        harness.drain_worker();
        assert_eq!(
            harness.results.last().expect("result").outcome,
            ImportOutcome::Failed
        );
        assert!(
            harness.banners.iter().any(|b| b.contains("parse error")),
            "expected banner, got {:?}",
            harness.banners
        );
    }

    #[test]
    fn tc_ir_9_2_1_n1_source_read_error_increments_fm1() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("locked.png", vec![1, 2, 3]);
        harness.fs.fault_on_read("locked.png", IoError::PermissionDenied);
        harness.drop_file("locked.png");
        harness.drain_worker();
        assert_eq!(harness.counters.fm1_source_read_error, 1);
        assert_eq!(
            harness.results.last().expect("result").outcome,
            ImportOutcome::Failed
        );
    }

    #[test]
    fn tc_ir_9_2_4_1_progress_messages_delivered_in_order() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("a.png", vec![9]);
        harness.drop_file("a.png");
        harness.drain_worker();
        let ours: Vec<ImportStage> = harness
            .progress
            .iter()
            .filter(|p| p.request_id == 1)
            .map(|p| p.stage)
            .collect();
        assert_eq!(
            ours,
            vec![
                ImportStage::Read,
                ImportStage::Parse,
                ImportStage::Process,
                ImportStage::Bake,
                ImportStage::Link,
                ImportStage::Upload,
            ]
        );
    }

    #[test]
    fn tc_ir_9_2_4_2_eta_non_increasing() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("a.png", vec![9]);
        harness.drop_file("a.png");
        harness.drain_worker();
        let etas: Vec<f64> = harness
            .progress
            .iter()
            .filter(|p| p.request_id == 1)
            .map(|p| p.eta_s)
            .collect();
        for window in etas.windows(2) {
            assert!(window[0] + f64::EPSILON >= window[1]);
        }
    }

    #[test]
    fn tc_ir_9_2_5_1_source_change_enqueues_stale_gated_request() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("tex.png", vec![1]);
        harness.drop_file("tex.png");
        harness.drain_worker();
        let event = SourceChangedEvent {
            mtime: UNIX_EPOCH + Duration::from_secs(2),
            source_path: PathBuf::from("tex.png"),
        };
        harness.on_source_changed(event);
        assert_eq!(harness.requests.len(), 1);
        assert!(harness.requests[0].only_if_stale);
    }

    #[test]
    fn tc_ir_9_2_5_2_only_if_stale_skips_unchanged() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("tex.png", vec![1, 2, 3]);
        harness.drop_file("tex.png");
        harness.drain_worker();
        let rid = harness.on_source_changed(SourceChangedEvent {
            mtime: UNIX_EPOCH + Duration::from_secs(9),
            source_path: PathBuf::from("tex.png"),
        });
        harness.drain_worker();
        let last = harness
            .results
            .iter()
            .find(|r| r.request_id == rid)
            .expect("stale outcome");
        assert_eq!(last.outcome, ImportOutcome::AlreadyUpToDate);
    }
}
