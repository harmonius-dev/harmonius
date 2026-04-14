//! Minimal editor + worker loop for CI integration tests.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::contracts::{
    AssetId, AssetImportProgress, AssetImportRequest, AssetImportResult, AssetReloadEvent,
    ImportBatchId, ImportKind, ImportOptions, ImportOutcome, ImportStage, ReloadScope,
    SourceChangedEvent,
};
use crate::dependency_graph::DependencyGraph;
use crate::fake_fs::{FakeFileSystem, IoError};
use crate::fake_importer::{FakeImportError, FakeImporter};
use crate::hot_reload::HotReloadBarrier;

/// Counters for fallback modes FM-1 … FM-7 (incremented by the harness).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FallbackCounters {
    pub fm1_source_read_error: u64,
    pub fm2_parse_error_keeps_previous: u64,
    pub fm3_dependency_cycle_detected: u64,
    pub fm4_hot_reload_barrier_timeout: u64,
    pub fm5_hot_reload_req_channel_backpressure: u64,
    pub fm6_progress_channel_drop_oldest: u64,
    pub fm7_partial_batch_failure: u64,
}

/// Headless editor slice: enqueue, worker drain, HUD banners, telemetry.
#[derive(Debug)]
pub struct HeadlessEditorHarness {
    asset_sequence: u64,
    pub banners: Vec<String>,
    cancelled: HashSet<u64>,
    pub counters: FallbackCounters,
    pub deps: DependencyGraph,
    pub fs: FakeFileSystem,
    pub hot_reload_barrier: HotReloadBarrier,
    pub importer: FakeImporter,
    last_hashes: HashMap<PathBuf, u64>,
    last_watcher_mtime_on_import: HashMap<PathBuf, SystemTime>,
    /// Play-in-editor mode: reload swaps go through the barrier with runtime scope.
    pub pie_mode: bool,
    progress_cap: usize,
    pub progress: Vec<AssetImportProgress>,
    pub registry: HashMap<PathBuf, AssetId>,
    pub reload_events: Vec<AssetReloadEvent>,
    request_sequence: u64,
    pub requests: Vec<AssetImportRequest>,
    pub results: Vec<AssetImportResult>,
    /// Monotonic revision per path for swap atomicity checks (IR-9.2.3.3).
    pub swap_generation: HashMap<PathBuf, u64>,
}

impl Default for HeadlessEditorHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadlessEditorHarness {
    /// Progress channel capacity (ImportProgressCh, FM-6).
    pub const DEFAULT_PROGRESS_CAP: usize = 256;

    pub fn new() -> Self {
        Self {
            asset_sequence: 1,
            banners: Vec::new(),
            cancelled: HashSet::new(),
            counters: FallbackCounters::default(),
            deps: DependencyGraph::default(),
            fs: FakeFileSystem::default(),
            hot_reload_barrier: HotReloadBarrier::new(),
            importer: FakeImporter::new(),
            last_hashes: HashMap::new(),
            last_watcher_mtime_on_import: HashMap::new(),
            pie_mode: false,
            progress_cap: Self::DEFAULT_PROGRESS_CAP,
            progress: Vec::new(),
            registry: HashMap::new(),
            reload_events: Vec::new(),
            request_sequence: 1,
            requests: Vec::new(),
            results: Vec::new(),
            swap_generation: HashMap::new(),
        }
    }

    /// Sets the bounded progress channel capacity (use a large value to disable drops in tests).
    pub fn set_progress_cap(&mut self, cap: usize) {
        self.progress_cap = cap;
    }

    /// Marks `request_id` as cancelled before the worker observes it (`ImportOutcome::Cancelled`).
    pub fn cancel_request(&mut self, request_id: u64) {
        self.cancelled.insert(request_id);
    }

    /// Simulates dropping a single file into the asset browser (IR-9.2.1).
    pub fn drop_file(&mut self, path: &str) -> u64 {
        self.enqueue_single(path, ImportBatchId(0), false)
    }

    fn enqueue_single(&mut self, path: &str, batch_id: ImportBatchId, only_if_stale: bool) -> u64 {
        let source_path = PathBuf::from(path);
        let kind = infer_kind(&source_path);
        let request_id = self.request_sequence;
        self.request_sequence += 1;
        self.requests.push(AssetImportRequest {
            batch_id,
            kind,
            only_if_stale,
            options: ImportOptions::default(),
            request_id,
            source_path,
            watcher_mtime: None,
        });
        request_id
    }

    /// Simulates recursive folder drop (IR-9.2.6) with a shared batch id.
    pub fn drop_folder(&mut self, dir: &str, count: usize, batch_id: ImportBatchId) -> Vec<u64> {
        let mut ids = Vec::new();
        for i in 0..count {
            let path = format!("{dir}/file_{i}.png");
            ids.push(self.enqueue_single(&path, batch_id, false));
        }
        ids
    }

    /// Handles `SourceChangedEvent` by enqueueing a stale-gated import (IR-9.2.5).
    pub fn on_source_changed(&mut self, event: SourceChangedEvent) -> u64 {
        let request_id = self.request_sequence;
        self.request_sequence += 1;
        let kind = infer_kind(&event.source_path);
        self.requests.push(AssetImportRequest {
            batch_id: ImportBatchId(0),
            kind,
            only_if_stale: true,
            options: ImportOptions::default(),
            request_id,
            source_path: event.source_path,
            watcher_mtime: Some(event.mtime),
        });
        request_id
    }

    /// Runs the synthetic import worker until the queue is empty.
    pub fn drain_worker(&mut self) {
        let results_start = self.results.len();
        while !self.requests.is_empty() {
            let batch = std::mem::take(&mut self.requests);
            for job in batch {
                let mut stack = Vec::new();
                self.run_one_recursive(job, &mut stack);
            }
        }
        self.finish_batch_telemetry(results_start);
    }

    fn finish_batch_telemetry(&mut self, results_start: usize) {
        let mut by_batch: HashMap<ImportBatchId, Vec<ImportOutcome>> = HashMap::new();
        for r in self.results[results_start..].iter() {
            if r.batch_id == ImportBatchId(0) {
                continue;
            }
            by_batch.entry(r.batch_id).or_default().push(r.outcome);
        }
        for outcomes in by_batch.values() {
            let any_ok = outcomes.contains(&ImportOutcome::Success);
            let any_fail = outcomes.contains(&ImportOutcome::Failed);
            if any_ok && any_fail {
                self.counters.fm7_partial_batch_failure += 1;
            }
        }
    }

    fn run_one_recursive(&mut self, job: AssetImportRequest, stack: &mut Vec<PathBuf>) {
        if self.cancelled.remove(&job.request_id) {
            self.results.push(AssetImportResult {
                asset_id: AssetId(0),
                batch_id: job.batch_id,
                duration_s: 0.0,
                outcome: ImportOutcome::Cancelled,
                request_id: job.request_id,
            });
            return;
        }
        if stack.contains(&job.source_path) {
            self.counters.fm3_dependency_cycle_detected += 1;
            self.banners.push(format!(
                "import aborted: dependency cycle ({})",
                job.source_path.display()
            ));
            return;
        }
        stack.push(job.source_path.clone());
        self.run_inner(job, stack);
        stack.pop();
    }

    fn run_inner(&mut self, job: AssetImportRequest, stack: &mut Vec<PathBuf>) {
        let start = std::time::Instant::now();
        self.emit_progress_stages(&job);

        let read = self.fs.read(job.source_path.as_path());
        let bytes = match read {
            Ok(b) => b,
            Err(IoError::PermissionDenied) => {
                self.counters.fm1_source_read_error += 1;
                self.results.push(AssetImportResult {
                    asset_id: AssetId(0),
                    batch_id: job.batch_id,
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
                    batch_id: job.batch_id,
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
                    let skip_by_mtime = match job.watcher_mtime {
                        None => true,
                        Some(w) => self
                            .last_watcher_mtime_on_import
                            .get(&job.source_path)
                            .map(|prev_t| w <= *prev_t)
                            .unwrap_or(true),
                    };
                    if skip_by_mtime {
                        if let Some(w) = job.watcher_mtime {
                            self.last_watcher_mtime_on_import
                                .insert(job.source_path.clone(), w);
                        }
                        self.results.push(AssetImportResult {
                            asset_id: self
                                .registry
                                .get(&job.source_path)
                                .copied()
                                .unwrap_or(AssetId(0)),
                            batch_id: job.batch_id,
                            duration_s: start.elapsed().as_secs_f64(),
                            outcome: ImportOutcome::AlreadyUpToDate,
                            request_id: job.request_id,
                        });
                        return;
                    }
                }
            }
        }

        let registry_before = self.registry.clone();
        let hash_before = self.last_hashes.get(&job.source_path).copied();

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
                    batch_id: job.batch_id,
                    duration_s: start.elapsed().as_secs_f64(),
                    outcome: ImportOutcome::Success,
                    request_id: job.request_id,
                });
                if let Some(w) = job.watcher_mtime {
                    self.last_watcher_mtime_on_import
                        .insert(job.source_path.clone(), w);
                }
                let scope = if self.pie_mode && job.kind == ImportKind::Texture {
                    ReloadScope::Both
                } else {
                    ReloadScope::EditorOnly
                };
                let path = job.source_path.clone();
                let swapped = self.hot_reload_barrier.try_swap(|| {
                    *self.swap_generation.entry(path.clone()).or_insert(0) += 1;
                    self.reload_events.push(AssetReloadEvent { asset_id, scope });
                });
                if !swapped {
                    self.registry = registry_before;
                    match hash_before {
                        Some(h) => {
                            self.last_hashes.insert(job.source_path.clone(), h);
                        }
                        None => {
                            self.last_hashes.remove(&job.source_path);
                        }
                    }
                    self.asset_sequence -= 1;
                    self.results.pop();
                    self.counters.fm4_hot_reload_barrier_timeout += 1;
                    self.results.push(AssetImportResult {
                        asset_id: AssetId(0),
                        batch_id: job.batch_id,
                        duration_s: start.elapsed().as_secs_f64(),
                        outcome: ImportOutcome::Failed,
                        request_id: job.request_id,
                    });
                    return;
                }

                let deps: Vec<PathBuf> = self.deps.dependents(&job.source_path).to_vec();
                for dep in deps {
                    let child = self.cascade_request(dep);
                    self.run_one_recursive(child, stack);
                }
            }
            Err(FakeImportError::Parse) => {
                self.counters.fm2_parse_error_keeps_previous += 1;
                self.results.push(AssetImportResult {
                    asset_id: self
                        .registry
                        .get(&job.source_path)
                        .copied()
                        .unwrap_or(AssetId(0)),
                    batch_id: job.batch_id,
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

    fn cascade_request(&mut self, source_path: PathBuf) -> AssetImportRequest {
        let kind = infer_kind(&source_path);
        let request_id = self.request_sequence;
        self.request_sequence += 1;
        AssetImportRequest {
            batch_id: ImportBatchId(0),
            kind,
            only_if_stale: false,
            options: ImportOptions::default(),
            request_id,
            source_path,
            watcher_mtime: None,
        }
    }

    fn push_progress(&mut self, tick: AssetImportProgress) {
        while self.progress.len() >= self.progress_cap {
            self.progress.remove(0);
            self.counters.fm6_progress_channel_drop_oldest += 1;
        }
        self.progress.push(tick);
    }

    fn emit_progress_stages(&mut self, job: &AssetImportRequest) {
        let linear = [
            ImportStage::Read,
            ImportStage::Parse,
            ImportStage::Process,
            ImportStage::Compress,
            ImportStage::Bake,
            ImportStage::Link,
            ImportStage::Upload,
        ];
        let total_steps: u8 = 9;
        let mut step: u8 = 0;
        let mut eta = 1.0_f64;
        for stage in linear {
            eta = (eta - 0.08).max(0.0);
            if stage == ImportStage::Bake {
                for (sub_i, sub_pct) in [(1_u8, 52_u8), (2, 68), (3, 84)] {
                    step = step.saturating_add(1);
                    self.push_progress(AssetImportProgress {
                        batch_id: job.batch_id,
                        eta_s: (eta - f64::from(sub_i) * 0.01).max(0.0),
                        percent: sub_pct,
                        request_id: job.request_id,
                        stage: ImportStage::Bake,
                    });
                }
                continue;
            }
            step = step.saturating_add(1);
            let percent = ((u16::from(step) * 100) / u16::from(total_steps)) as u8;
            self.push_progress(AssetImportProgress {
                batch_id: job.batch_id,
                eta_s: eta,
                percent,
                request_id: job.request_id,
                stage,
            });
        }
    }

    /// Emits many progress ticks for stress-testing the progress channel (TC-IR-9.2.4.N1).
    pub fn emit_progress_flood(&mut self, job: &AssetImportRequest, count: usize) {
        for i in 0..count {
            let percent = ((i + 1) * 100 / count.max(1)) as u8;
            self.push_progress(AssetImportProgress {
                batch_id: job.batch_id,
                eta_s: (1.0_f64 - (i as f64) * 0.001).max(0.0),
                percent,
                request_id: job.request_id,
                stage: ImportStage::Read,
            });
        }
    }

    /// Rolls up completed percent across all requests sharing a batch id (IR-9.2.6.2).
    pub fn batch_progress_percent(&self, batch_id: ImportBatchId) -> u8 {
        let ids: HashSet<u64> = self
            .results
            .iter()
            .filter(|r| r.batch_id == batch_id)
            .map(|r| r.request_id)
            .collect();
        if ids.is_empty() {
            return 0;
        }
        let mut total = 0_u32;
        let mut count = 0_u32;
        for rid in ids {
            let last_pct = self
                .progress
                .iter()
                .filter(|p| p.request_id == rid && p.batch_id == batch_id)
                .map(|p| p.percent as u32)
                .max()
                .unwrap_or(0);
            total += last_pct;
            count += 1;
        }
        (total / count.max(1)) as u8
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
    use crate::hot_reload::{HotReloadReqChannel, HotReloadResultChannel};
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
    fn tc_ir_9_2_1_n2_parse_error_keeps_previous_asset() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("good.png", vec![1, 2, 3]);
        harness.drop_file("good.png");
        harness.drain_worker();
        let first_id = *harness.registry.get(&PathBuf::from("good.png")).unwrap();
        harness.importer.fail_parse_for("good.png");
        harness.fs.insert("good.png", vec![9, 9, 9]);
        harness.drop_file("good.png");
        harness.drain_worker();
        assert_eq!(harness.counters.fm2_parse_error_keeps_previous, 1);
        assert_eq!(
            harness.registry.get(&PathBuf::from("good.png")),
            Some(&first_id)
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
                ImportStage::Compress,
                ImportStage::Bake,
                ImportStage::Bake,
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
    fn tc_ir_9_2_4_3_multi_tick_bake_progress_increases() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("a.png", vec![9]);
        harness.drop_file("a.png");
        harness.drain_worker();
        let bake: Vec<u8> = harness
            .progress
            .iter()
            .filter(|p| p.request_id == 1 && p.stage == ImportStage::Bake)
            .map(|p| p.percent)
            .collect();
        assert_eq!(bake, vec![52, 68, 84]);
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

    #[test]
    fn tc_ir_9_2_2_1_reimport_texture_cascades_to_material() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("tex.png", vec![1, 2]);
        harness.fs.insert("mat.mat", vec![3]);
        harness
            .deps
            .add_edge(PathBuf::from("mat.mat"), PathBuf::from("tex.png"));
        harness.drop_file("tex.png");
        harness.drain_worker();
        let mat_path = PathBuf::from("mat.mat");
        assert!(harness.registry.contains_key(&mat_path));
        assert!(harness.reload_events.len() >= 2);
    }

    #[test]
    fn tc_ir_9_2_2_2_removing_edge_stops_cascade() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("tex1.png", vec![1]);
        harness.fs.insert("tex2.png", vec![2]);
        harness.fs.insert("mat.mat", vec![3]);
        harness
            .deps
            .add_edge(PathBuf::from("mat.mat"), PathBuf::from("tex1.png"));
        harness
            .deps
            .add_edge(PathBuf::from("mat.mat"), PathBuf::from("tex2.png"));
        harness
            .deps
            .remove_edge(&PathBuf::from("mat.mat"), &PathBuf::from("tex1.png"));
        harness.drop_file("tex1.png");
        harness.drain_worker();
        assert!(!harness.registry.contains_key(&PathBuf::from("mat.mat")));
        harness.fs.insert("tex2.png", vec![7, 8]);
        harness.drop_file("tex2.png");
        harness.drain_worker();
        assert!(harness.registry.contains_key(&PathBuf::from("mat.mat")));
    }

    #[test]
    fn tc_ir_9_2_2_3_cycle_detected_increments_fm3() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("a.mat", vec![1]);
        harness.fs.insert("b.mat", vec![2]);
        harness
            .deps
            .add_edge(PathBuf::from("a.mat"), PathBuf::from("b.mat"));
        harness
            .deps
            .add_edge(PathBuf::from("b.mat"), PathBuf::from("a.mat"));
        harness.drop_file("a.mat");
        harness.drain_worker();
        assert!(harness.counters.fm3_dependency_cycle_detected >= 1);
        assert!(
            harness
                .banners
                .iter()
                .any(|b| b.contains("dependency cycle")),
            "expected FM-3 banner, got {:?}",
            harness.banners
        );
    }

    #[test]
    fn tc_ir_9_2_3_1_pie_texture_reload_uses_both_scope() {
        let mut harness = HeadlessEditorHarness::new();
        harness.pie_mode = true;
        harness.fs.insert("tex.png", vec![1, 2, 3]);
        harness.drop_file("tex.png");
        harness.drain_worker();
        let reload = harness.reload_events.last().expect("reload");
        assert_eq!(reload.scope, ReloadScope::Both);
        assert!(harness.hot_reload_barrier.park_cycles >= 1);
    }

    #[test]
    fn tc_ir_9_2_3_2_edit_mode_mesh_reload_editor_only() {
        let mut harness = HeadlessEditorHarness::new();
        harness.pie_mode = false;
        harness.fs.insert("m.glb", vec![1, 2, 3]);
        harness.drop_file("m.glb");
        harness.drain_worker();
        let reload = harness.reload_events.last().expect("reload");
        assert_eq!(reload.scope, ReloadScope::EditorOnly);
    }

    #[test]
    fn tc_ir_9_2_3_3_swap_updates_generation_atomically() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("tex.png", vec![1]);
        harness.drop_file("tex.png");
        harness.drain_worker();
        let g1 = *harness.swap_generation.get(&PathBuf::from("tex.png")).unwrap();
        harness.fs.insert("tex.png", vec![2, 3]);
        harness.drop_file("tex.png");
        harness.drain_worker();
        let g2 = *harness.swap_generation.get(&PathBuf::from("tex.png")).unwrap();
        assert_ne!(g1, g2);
    }

    #[test]
    fn tc_ir_9_2_3_n1_barrier_timeout_fm4() {
        let mut harness = HeadlessEditorHarness::new();
        harness.pie_mode = true;
        harness.hot_reload_barrier = HotReloadBarrier::always_times_out();
        harness.fs.insert("tex.png", vec![1]);
        harness.drop_file("tex.png");
        harness.drain_worker();
        assert_eq!(harness.counters.fm4_hot_reload_barrier_timeout, 1);
        assert!(harness.reload_events.is_empty());
        assert!(!harness.registry.contains_key(&PathBuf::from("tex.png")));
    }

    #[test]
    fn tc_ir_9_2_1_n3_ch20_backpressure_fm5() {
        let mut ch = HotReloadReqChannel::with_cap(16);
        for i in 0..32 {
            ch.enqueue_with_cooperative_drain(i);
        }
        assert_eq!(ch.fm5_backpressure_events, 16);
        assert_eq!(ch.len(), 16);
    }

    #[test]
    fn hot_reload_result_ch_roundtrip() {
        let mut ch = HotReloadResultChannel::default();
        ch.push_ok(7);
        assert_eq!(ch.pop().expect("pop").asset_index, 7);
        assert!(ch.pop().is_none());
    }

    #[test]
    fn cancel_request_emits_cancelled_outcome() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("z.png", vec![1]);
        let rid = harness.drop_file("z.png");
        harness.cancel_request(rid);
        harness.drain_worker();
        assert_eq!(
            harness.results.last().expect("result").outcome,
            ImportOutcome::Cancelled
        );
    }

    #[test]
    fn watcher_mtime_newer_forces_reimport_when_hash_matches() {
        let mut harness = HeadlessEditorHarness::new();
        harness.fs.insert("tex.png", vec![1, 2, 3]);
        harness.drop_file("tex.png");
        harness.drain_worker();
        harness.on_source_changed(SourceChangedEvent {
            mtime: UNIX_EPOCH + Duration::from_secs(3),
            source_path: PathBuf::from("tex.png"),
        });
        harness.drain_worker();
        harness.on_source_changed(SourceChangedEvent {
            mtime: UNIX_EPOCH + Duration::from_secs(10),
            source_path: PathBuf::from("tex.png"),
        });
        harness.drain_worker();
        let outcomes: Vec<ImportOutcome> = harness.results.iter().map(|r| r.outcome).collect();
        assert!(
            outcomes.contains(&ImportOutcome::AlreadyUpToDate),
            "{outcomes:?}"
        );
        assert!(outcomes.contains(&ImportOutcome::Success));
    }

    #[test]
    fn tc_ir_9_2_4_n1_progress_channel_drops_oldest_fm6() {
        let mut harness = HeadlessEditorHarness::new();
        harness.set_progress_cap(256);
        let job = AssetImportRequest {
            batch_id: ImportBatchId(0),
            kind: ImportKind::Texture,
            only_if_stale: false,
            options: ImportOptions::default(),
            request_id: 1,
            source_path: PathBuf::from("x.png"),
            watcher_mtime: None,
        };
        harness.emit_progress_flood(&job, 500);
        assert_eq!(harness.progress.len(), 256);
        assert_eq!(harness.counters.fm6_progress_channel_drop_oldest, 244);
    }

    #[test]
    fn tc_ir_9_2_6_1_folder_drop_batches_requests() {
        let mut harness = HeadlessEditorHarness::new();
        let batch = ImportBatchId(7);
        for i in 0..100 {
            harness
                .fs
                .insert(format!("drop/file_{i}.png"), vec![i as u8]);
        }
        harness.drop_folder("drop", 100, batch);
        assert_eq!(harness.requests.len(), 100);
        assert!(harness.requests.iter().all(|r| r.batch_id == batch));
        harness.drain_worker();
        assert_eq!(
            harness
                .results
                .iter()
                .filter(|r| r.batch_id == batch && r.outcome == ImportOutcome::Success)
                .count(),
            100
        );
    }

    #[test]
    fn tc_ir_9_2_6_2_batch_progress_rollup() {
        let mut harness = HeadlessEditorHarness::new();
        let batch = ImportBatchId(3);
        for i in 0..4 {
            harness.fs.insert(format!("b/file_{i}.png"), vec![1]);
        }
        harness.drop_folder("b", 4, batch);
        harness.drain_worker();
        let pct = harness.batch_progress_percent(batch);
        assert!(pct > 0);
    }

    #[test]
    fn tc_ir_9_2_6_n1_partial_batch_fm7() {
        let mut harness = HeadlessEditorHarness::new();
        let batch = ImportBatchId(9);
        for i in 0..100 {
            let path = format!("batch/file_{i}.png");
            if i == 42 {
                harness.fs.insert(&path, vec![0]);
                harness.importer.fail_parse_for(&path);
            } else {
                harness.fs.insert(&path, vec![1]);
            }
        }
        harness.drop_folder("batch", 100, batch);
        harness.drain_worker();
        let ok = harness
            .results
            .iter()
            .filter(|r| r.batch_id == batch && r.outcome == ImportOutcome::Success)
            .count();
        let fail = harness
            .results
            .iter()
            .filter(|r| r.batch_id == batch && r.outcome == ImportOutcome::Failed)
            .count();
        assert_eq!(ok, 99);
        assert_eq!(fail, 1);
        assert_eq!(harness.counters.fm7_partial_batch_failure, 1);
    }
}
