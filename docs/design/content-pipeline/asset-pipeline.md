# Asset Pipeline Design

## Requirements Trace

> **Canonical sources:** Features, requirements, and user stories are in
> [features/](../../features/), [requirements/](../../requirements/), and
> [user-stories/](../../user-stories/).

### Asset Import (F-12.1)

| Feature  | Requirement |
|----------|-------------|
| F-12.1.1 | R-12.1.1    |
| F-12.1.2 | R-12.1.2    |
| F-12.1.3 | R-12.1.3    |
| F-12.1.4 | R-12.1.4    |
| F-12.1.5 | R-12.1.5    |

1. **F-12.1.1** -- Geometry import (glTF 2.0, Alembic) with BLAKE3 and CAS
2. **F-12.1.2** -- Texture import (PNG, EXR, KTX2)
3. **F-12.1.3** -- Audio import (WAV, FLAC)
4. **F-12.1.4** -- Validation with path, offset, fix suggestions
5. **F-12.1.5** -- Batch import with progress and rollback

### Asset Database (F-12.3)

| Feature   | Requirement |
|-----------|-------------|
| F-12.3.1  | R-12.3.1    |
| F-12.3.2  | R-12.3.2    |
| F-12.3.3  | R-12.3.3    |
| F-12.3.4  | R-12.3.4    |
| F-12.3.5  | R-12.3.5    |
| F-12.3.6  | R-12.3.6    |
| F-12.3.10 | R-12.3.10   |

1. **F-12.3.1** -- CAS keyed by BLAKE3 hash
2. **F-12.3.2** -- Persistent metadata (IDs, paths, hashes, deps)
3. **F-12.3.3** -- Hash-based import caching
4. **F-12.3.4** -- Incremental builds via dependency invalidation
5. **F-12.3.5** -- Full-text and tag-based faceted search
6. **F-12.3.6** -- Async thumbnail generation during import
7. **F-12.3.10** -- Asset versioning with hash-based restore

### Hot Reload (F-12.4)

| Feature  | Requirement |
|----------|-------------|
| F-12.4.1 | R-12.4.1    |
| F-12.4.2 | R-12.4.2    |
| F-12.4.3 | R-12.4.3    |
| F-12.4.4 | R-12.4.4    |
| F-12.4.5 | R-12.4.5    |
| F-12.4.6 | R-12.4.6    |
| F-12.4.7 | R-12.4.7    |

1. **F-12.4.1** -- File watcher with debounce and deduplication
2. **F-12.4.2** -- Asset hot reload with atomic pointer swap
3. **F-12.4.3** -- Shader hot reload with error overlay
4. **F-12.4.4** -- Logic graph hot reload with state preservation
5. **F-12.4.5** -- UI hot reload preserving scroll/focus/animation
6. **F-12.4.6** -- Partial re-import of modified sub-assets
7. **F-12.4.7** -- Bidirectional editor-runtime sync channel

### Asset Versioning (F-12.7)

| Feature  | Requirement |
|----------|-------------|
| F-12.7.1 | R-12.7.1    |
| F-12.7.2 | R-12.7.2    |
| F-12.7.3 | R-12.7.3    |
| F-12.7.4 | R-12.7.4    |
| F-12.7.5 | R-12.7.5    |
| F-12.7.6 | R-12.7.6    |
| F-12.7.7 | R-12.7.7    |
| F-12.7.8 | R-12.7.8    |

1. **F-12.7.1** -- Universal binary asset format: mmap, O(1) access
2. **F-12.7.2** -- Compressed bundles: LZ4 runtime, Zstd dist
3. **F-12.7.3** -- Structural asset diffing by type
4. **F-12.7.4** -- Three-way merge with Git custom merge driver
5. **F-12.7.5** -- Auto conflict resolution: LWW, union, ordered
6. **F-12.7.6** -- Spreadsheet-style data table editor
7. **F-12.7.7** -- Universal visual asset inspector
8. **F-12.7.8** -- Git LFS with custom merge driver

### Cross-Cutting Dependencies

| Dependency | Source | Consumed API |
|------------|--------|--------------|
| Platform I/O | F-14.3.5 | `io_uring` / IOCP / GCD `dispatch_io` |
| ThreadPool | F-14.3.1 | `spawn`, `scope` |
| FileWatcher | F-14.6.5 | `FileWatcher`, `FileEventStream` |
| ContentHasher | F-14.6.6 | `ContentHasher`, `Blake3Hash` |
| ECS | F-1.1.1 | Component storage for asset handles |
| Codegen | F-15.8.1 | Codegen'd type descriptors in middleman `.dylib` |

## Overview

The asset pipeline covers all stages of content ingest: source file import, database registration,
file watching, hot reload, binary asset format, and version control integration. It ingests standard
interchange formats:

- **Geometry** -- glTF 2.0 (meshes, scenes, skeletal animation), Alembic (vertex cache animation,
  particles)
- **Textures** -- PNG, EXR, KTX2
- **Audio** -- WAV, FLAC

All imported content flows through: source detection, format decoding, validation, BLAKE3 hashing,
CAS storage, and metadata registration.

**Async scope.** Game-side paths (asset handles, residency manager, hot reload swap, and every API
the engine or editor calls at runtime) are fully synchronous: no `async`, no `await`, no `Future`.
Asset-import CLI tools (`harmonius-import`, `harmonius-cook`) run as separate processes and *may*
use a Rust async runtime internally, but they never link against the engine process. Runtime batch
imports parallelize across the custom job system via `scope()` / `par_iter`, submitting file I/O to
the main thread through crossbeam-channel and reading completions at frame boundaries. Hot-reload
schema migration follows
[../core-runtime/hot-reload-protocol.md](../core-runtime/hot-reload-protocol.md).

Key design goals:

- **Deterministic builds.** Identical inputs always produce identical outputs with identical hashes.
- **Incremental imports.** Only changed sources are re-imported. Cache key = hash(source + settings
  - tool version).
- **Sub-second hot reload.** File watcher detects changes, pipeline re-imports, and atomically swaps
  runtime representations without restarting.
- **Standard interchange formats.** glTF 2.0, Alembic, PNG, EXR, KTX2, WAV, FLAC as import sources.

## Architecture

### Module Boundaries

```mermaid
graph TD
    subgraph harmonius_content::import
        IM[ImportCoordinator]
        GI[GltfImporter]
        ABI[AlembicImporter]
        TI[TextureImporter]
        AI[AudioImporter]
        VL[ImportValidator]
        PR[ProgressTracker]
    end

    subgraph harmonius_content::database
        CAS[ContentAddressableStore]
        META[MetadataStore]
        DEP[DependencyGraph]
        IDX[SearchIndex]
        CACHE[ImportCache]
        VER[VersionStore]
    end

    subgraph harmonius_content::hot_reload
        AW[AssetWatcher]
        CD[ChangeDetector]
        RC[ReloadCoordinator]
        SW[SwapScheduler]
        SR[ShaderReloader]
        LR[LogicGraphReloader]
        UR[UiReloader]
        ES[EditorSync]
    end

    subgraph harmonius_asset_version
        AF[AssetFormat]
        BW[BundleWriter/Reader]
        SDIFF[StructuralDiff]
        MERGE[ThreeWayMerge]
        PATCH[BinaryDiffPatch]
    end

    subgraph harmonius_platform
        IOB[IoBridge]
        TP[ThreadPool]
        FW[FileWatcher]
    end

    IM --> GI
    IM --> ABI
    IM --> TI
    IM --> AI
    IM --> VL
    IM --> CAS
    IM --> META
    IM --> DEP
    IM --> CACHE
    IM --> VER
    META --> IDX

    AW --> FW
    AW --> CD
    CD --> RC
    RC --> IM
    RC --> SW
    RC --> SR
    RC --> LR
    RC --> UR
    ES --> RC

    AF --> SDIFF
    SDIFF --> MERGE
    MERGE --> PATCH

    IM -.-> TK
    IM -.-> TP
    CAS -.-> TK
    BW -.-> TK
```

### File Layout

```text
harmonius_content/
├── import/
│   ├── coordinator.rs   # ImportCoordinator
│   ├── registry.rs      # ImporterRegistry
│   ├── gltf.rs          # GltfImporter
│   ├── alembic.rs       # AlembicImporter
│   ├── texture.rs       # TextureImporter
│   ├── audio.rs         # AudioImporter
│   ├── validator.rs     # ImportValidator
│   └── progress.rs      # ProgressTracker
├── database/
│   ├── cas.rs           # ContentAddressableStore
│   ├── metadata.rs      # MetadataStore
│   ├── deps.rs          # DependencyGraph
│   ├── search.rs        # SearchIndex
│   ├── cache.rs         # ImportCache, CacheKey
│   ├── version.rs       # VersionStore
│   └── id.rs            # AssetId, ContentHash
└── hot_reload/
    ├── watcher.rs        # AssetWatcher
    ├── detector.rs       # ChangeDetector
    ├── coordinator.rs    # ReloadCoordinator
    ├── swap.rs           # SwapScheduler
    ├── shader.rs         # ShaderReloader
    ├── logic_graph.rs    # LogicGraphReloader
    ├── ui.rs             # UiReloader
    ├── editor_sync.rs    # EditorSync
    └── error.rs          # HotReloadError

harmonius_asset_version/
├── format/
│   ├── header.rs         # AssetHeader, TOC
│   ├── reader.rs         # AssetReader (mmap)
│   └── writer.rs         # AssetWriter
├── bundle/
│   ├── writer.rs         # BundleWriter
│   └── reader.rs         # BundleReader
├── diff/
│   └── structural.rs     # StructuralDiff
├── merge/
│   ├── three_way.rs      # ThreeWayMerge
│   └── git_driver.rs     # Git merge driver
└── patch/
    └── binary_diff.rs    # BinaryDiffEncoder
```

### Import Pipeline Flow

```mermaid
flowchart LR
    SRC[Source File] --> DET{Format?}

    DET -->|glTF 2.0| GI[GltfImporter]
    DET -->|Alembic| ABI[AlembicImporter]
    DET -->|PNG/EXR/KTX2| TI[TextureImporter]
    DET -->|WAV/FLAC| AI[AudioImporter]

    GI --> VAL[Validate]
    ABI --> VAL
    TI --> VAL
    AI --> VAL

    VAL --> HASH[BLAKE3 Hash]
    HASH --> DEDUP{CAS?}
    DEDUP -->|hit| SKIP[Skip Store]
    DEDUP -->|miss| STORE[Write CAS]
    SKIP --> REG[Register Metadata]
    STORE --> REG
    REG --> DEPS[Update Dep Graph]
```

### Hot Reload Pipeline

```mermaid
sequenceDiagram
    participant FS as Filesystem
    participant FW as FileWatcher
    participant AW as AssetWatcher
    participant CD as ChangeDetector
    participant RC as ReloadCoordinator
    participant IM as Importer
    participant PR as Processor
    participant SS as SwapScheduler
    participant RT as Runtime

    FS->>FW: File change notification
    FW->>AW: FileEvent queue (drained per frame)
    AW->>CD: Batch of FileEvents
    CD->>CD: BLAKE3 hash comparison
    CD->>CD: Resolve dependents
    CD->>RC: ReloadRequest

    RC->>IM: Re-import source
    IM-->>RC: ImportedAsset
    RC->>PR: Reprocess asset
    PR-->>RC: ProcessedAsset

    RC->>SS: Schedule swap
    Note over SS: Wait for frame boundary
    SS->>RT: Atomic swap at sync point
```

### Asset Handle Indirection

```mermaid
graph LR
    subgraph User Code
        H["AssetHandle(T)"]
    end
    subgraph Handle Table
        S[Slot: gen + ptr]
    end
    subgraph Frame N
        A1[Asset Data v1]
    end
    subgraph Frame N+1
        A2[Asset Data v2]
    end

    H -->|index + gen| S
    S -->|ptr before swap| A1
    S -.->|ptr after swap| A2
```

### Core Data Structures

```mermaid
classDiagram
    class AssetId {
        +inner u64
        +new() AssetId
    }
    class ContentHash {
        +bytes [u8; 32]
        +from_data(data) ContentHash
        +hex() String
        +prefix() [u8; 2]
    }
    class AssetType {
        <<enumeration>>
        Mesh
        Skeleton
        Animation
        Texture
        Material
        Audio
        Scene
        ShaderGraph
        LogicGraph
        UiLayout
    }
    class AssetMetadata {
        +asset_id AssetId
        +content_hash ContentHash
        +source_path PathBuf
        +import_settings ImportSettings
        +dependencies Vec~AssetId~
        +asset_type AssetType
        +byte_size u64
        +version u32
    }
    class ImportCoordinator {
        -registry ImporterRegistry
        -cas ContentAddressableStore
        -metadata MetadataStore
        -cache ImportCache
        +import_file(path, settings) RequestId
        +batch_import(entries) BatchImportHandle
        +reimport(id) RequestId
        +invalidate(changed) Result
    }
    class ContentAddressableStore {
        -root_path PathBuf
        +store(hash, data) RequestId
        +load(hash) RequestId
        +exists(hash) bool
        +gc(referenced) Result
    }
    class MetadataStore {
        -entries SortedVecMap
        +get(id) Option~AssetMetadata~
        +put(id, meta) Result
        +query(filter) Vec~AssetMetadata~
        +transaction() MetadataTransaction
    }
    class DependencyGraph {
        -forward HashMap
        -reverse HashMap
        +add_dependency(from, to)
        +dependents_of(id) Vec~AssetId~
        +transitive_dependents(id) Vec~AssetId~
        +topological_order() Result
    }
    class ImportCache {
        +lookup(key) Option~CacheEntry~
        +insert(key, entry) Result
        +invalidate(key) Result
    }
    class Importer {
        <<trait>>
        +extensions() Vec~str~
        +import(source, settings) Result
    }

    ImportCoordinator --> ContentAddressableStore
    ImportCoordinator --> MetadataStore
    ImportCoordinator --> DependencyGraph
    ImportCoordinator --> ImportCache
    MetadataStore --> AssetMetadata
    AssetMetadata --> AssetId
    AssetMetadata --> ContentHash
    AssetMetadata --> AssetType
```

### Hot Reload Data Structures

```mermaid
classDiagram
    class AssetWatcher {
        -file_watcher FileWatcher
        -hasher ContentHasher
        +start(config, db) Result
        +poll_changes() Vec~FileEvent~
        +stop()
    }
    class ChangeDetector {
        -asset_db AssetDatabase
        +detect(changes) Vec~ReloadTask~
        +resolve_dependents(id) Vec~AssetId~
    }
    class ReloadCoordinator {
        -importer AssetImporter
        -swap_scheduler SwapScheduler
        +submit(requests) Vec~RequestId~
        +poll_completed() Vec~ReloadTask~
    }
    class SwapScheduler {
        -pending Vec~PendingSwap~
        -retirement Vec~RetiredAsset~
        +schedule(swap)
        +apply_pending_swaps(table) u32
        +retire_old_assets(fence)
    }
    class HandleTable {
        -slots Vec~HandleSlot~
        +allocate(ptr, size) u32
        +swap_ptr(index, ptr, size) ptr
        +resolve(index, gen) Option~ptr~
        +retire(index)
    }
    class AssetHandle~T~ {
        -index u32
        -generation u32
        +get(table) Option~T~
        +is_valid(table) bool
    }
    class SwapStrategy {
        <<enumeration>>
        AtomicPointer
        DescriptorHeap
        PipelineState
        BytecodePatch
        SubtreeRebuild
    }
    class ReloadKind {
        <<enumeration>>
        Asset
        Shader
        LogicGraph
        Ui
    }

    AssetWatcher --> ChangeDetector
    ChangeDetector --> ReloadCoordinator
    ChangeDetector --> ReloadKind
    ReloadCoordinator --> SwapScheduler
    SwapScheduler --> HandleTable
    SwapScheduler --> SwapStrategy
    HandleTable --> AssetHandle~T~
```

### Versioning Structures

```mermaid
classDiagram
    class AssetHeader {
        +magic [u8; 4]
        +format_version u32
        +asset_type_id u64
        +content_hash [u8; 32]
        +toc_offset u64
        +toc_count u32
        +total_size u64
    }
    class AssetReader {
        -data bytes
        -header AssetHeader
        +from_bytes(data) Result
        +section(name) Result
        +verify_integrity() Result
    }
    class AssetWriter {
        -asset_type_id u64
        +add_section(name, type, data, comp) Self
        +build() Result
        +write_to(path) RequestId
    }
    class StructuralDiff {
        +diff(old, new, type_id) DiffResult
    }
    class ThreeWayMerge {
        +merge(ancestor, ours, theirs) MergeResult
    }
    class BinaryDiffEncoder {
        +encode(old, new) DiffPatch
    }

    AssetReader --> AssetHeader
    AssetWriter --> AssetHeader
    StructuralDiff --> ThreeWayMerge
    ThreeWayMerge --> BinaryDiffEncoder
```

## API Design

### Identity Types

```rust
/// Stable unique identifier for an asset.
/// Persists across reimports, renames, and moves.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub struct AssetId(pub u64);

/// BLAKE3 content hash (32 bytes). Used as the
/// CAS address and for integrity verification.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub struct ContentHash {
    pub bytes: [u8; 32],
}

impl ContentHash {
    pub fn from_data(data: &[u8]) -> Self;
    /// Submit a hash request over a file and return
    /// an id. Poll via `IoBridge::take_result`.
    pub fn from_file(
        path: &CanonicalPath,
        bridge: &mut IoBridge,
    ) -> Result<IoRequestId, IoError>;
    pub fn hex(&self) -> String;
    pub fn prefix(&self) -> [u8; 2];
}

/// Type discriminant for imported assets.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub enum AssetType {
    Mesh,
    Skeleton,
    Animation,
    Texture,
    Material,
    Audio,
    Scene,
    EntityTemplate,
    ShaderGraph,
    LogicGraph,
    UiLayout,
}
```

### Asset Metadata and Import Settings

```rust
/// Per-asset metadata in the MetadataStore.
#[derive(Clone, Debug, Reflect)]
pub struct AssetMetadata {
    pub asset_id: AssetId,
    pub content_hash: ContentHash,
    pub source_path: PathBuf,
    pub import_settings: ImportSettings,
    pub dependencies: Vec<AssetId>,
    pub dependents: Vec<AssetId>,
    pub tags: Vec<String>,
    pub asset_type: AssetType,
    pub byte_size: u64,
    pub created_at: u64,
    pub modified_at: u64,
    pub version: u32,
}

/// Format-specific import settings. Hashed as
/// part of the cache key.
#[derive(Clone, Debug, Reflect)]
pub enum ImportSettings {
    Gltf(GltfImportSettings),
    Alembic(AlembicImportSettings),
    Texture(TextureImportSettings),
    Audio(AudioImportSettings),
}

#[derive(Clone, Debug, Reflect)]
pub struct TextureImportSettings {
    pub compression: TextureCompression,
    pub generate_mips: bool,
    pub color_space: ColorSpace,
    pub max_dimension: u32,
}

#[derive(Clone, Debug, Reflect)]
pub struct AudioImportSettings {
    pub encoding: AudioEncoding,
    pub sample_rate: u32,
    pub channels: ChannelMode,
}
```

### Content-Addressable Store

```rust
pub struct ContentAddressableStore {
    root_path: PathBuf,
}

impl ContentAddressableStore {
    pub fn new(root_path: PathBuf) -> Self;

    /// Submit a store request; returns an id.
    /// Completion arrives via IoBridge at a later
    /// frame boundary.
    pub fn store(
        &mut self,
        hash: ContentHash,
        data: &[u8],
        bridge: &mut IoBridge,
    ) -> IoRequestId;

    pub fn load(
        &mut self,
        hash: ContentHash,
        bridge: &mut IoBridge,
    ) -> IoRequestId;

    /// Synchronous existence check against the
    /// in-memory manifest (no I/O).
    pub fn exists(&self, hash: ContentHash) -> bool;

    pub fn gc(
        &mut self,
        referenced: &HashSet<ContentHash>,
        bridge: &mut IoBridge,
    ) -> IoRequestId;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum StoreResult {
    Written,
    Deduplicated,
}
```

### Metadata Store

```rust
/// Synchronous metadata store. Reads/writes hit
/// an in-memory SortedVecMap; durability is a
/// journal file written by the IoBridge.
pub struct MetadataStore {
    entries: SortedVecMap<AssetId, AssetMetadata>,
    journal_path: PathBuf,
    dirty: bool,
}

impl MetadataStore {
    pub fn open(
        db_path: PathBuf,
        bridge: &mut IoBridge,
    ) -> Result<Self, MetadataError>;
    pub fn get(
        &self,
        id: AssetId,
    ) -> Option<&AssetMetadata>;
    pub fn put(
        &mut self,
        id: AssetId,
        metadata: AssetMetadata,
    );
    pub fn remove(&mut self, id: AssetId) -> bool;
    pub fn query(
        &self,
        filter: &SearchFilter,
    ) -> Vec<AssetId>;
    /// Begin atomic transaction. Drop = rollback.
    pub fn transaction(
        &mut self,
    ) -> MetadataTransaction;
    /// Submit a journal flush; completion via
    /// IoBridge at a later frame boundary.
    pub fn flush(
        &mut self,
        bridge: &mut IoBridge,
    ) -> IoRequestId;
}
```

### Importer Trait and Coordinator

```rust
pub trait Importer: Send + Sync {
    fn extensions(&self) -> &[&str];
    fn asset_types(&self) -> &[AssetType];
    /// Synchronous import. Long-running importers run
    /// on the worker pool via `ImportCoordinator::submit`.
    fn import(
        &self,
        source: &SourceFile,
        settings: &ImportSettings,
    ) -> Result<ImportOutput, ImportError>;
}

pub struct ImportCoordinator {
    registry: ImporterRegistry,
    cas: ContentAddressableStore,
    metadata: MetadataStore,
    dep_graph: DependencyGraph,
    cache: ImportCache,
    pool: ThreadPool,
    version_store: VersionStore,
}

impl ImportCoordinator {
    /// Queue a single-file import on the worker pool.
    /// Returns an id; poll via `take_result`.
    pub fn import_file(
        &mut self,
        path: PathBuf,
        settings: ImportSettings,
    ) -> ImportRequestId;
    /// Queue a batch of imports. Returns a handle
    /// that reports per-entry completion.
    pub fn batch_import(
        &mut self,
        entries: Vec<ImportEntry>,
    ) -> BatchImportHandle;
    pub fn reimport(
        &mut self,
        id: AssetId,
    ) -> ImportRequestId;
    pub fn invalidate(
        &mut self,
        changed: &[AssetId],
    ) -> Vec<AssetId>;
    /// Drain completed import jobs at the start of
    /// a frame. Called from the main thread.
    pub fn take_result(
        &mut self,
        id: ImportRequestId,
    ) -> Option<Result<ImportResult, ImportError>>;
}
```

### Asset Watcher and Change Detector

```rust
#[derive(Clone, Debug, Reflect)]
pub struct AssetWatcherConfig {
    pub watch_dirs: Vec<CanonicalPath>,
    pub debounce_ms: u32,
    pub max_batch_size: u32,
    pub batch_window_ms: u32,
}

pub struct AssetWatcher { /* ... */ }

impl AssetWatcher {
    pub fn start(
        config: AssetWatcherConfig,
        asset_db: &AssetDatabase,
    ) -> Result<Self, HotReloadError>;
    /// Called each frame on the main thread.
    /// Drains pending OS file events.
    pub fn poll_changes(
        &mut self,
    ) -> Vec<AssetChange>;
    pub fn stop(&mut self);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum ReloadKind {
    Asset,
    Shader,
    LogicGraph,
    Ui,
}

#[derive(Clone, Debug)]
pub struct ReloadRequest {
    pub primary: AssetId,
    pub new_hash: Blake3Hash,
    pub old_hash: Blake3Hash,
    pub dependents: Vec<AssetId>,
    pub kind: ReloadKind,
    pub partial: bool,
    pub sub_asset_indices: Vec<u32>,
}

pub struct ChangeDetector { /* ... */ }

impl ChangeDetector {
    pub fn detect(
        &mut self,
        changes: &[AssetChange],
    ) -> Vec<ReloadRequest>;
    pub fn resolve_dependents(
        &self,
        asset_id: AssetId,
    ) -> Vec<AssetId>;
}
```

### Swap Scheduler and Handle Table

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum SwapStrategy {
    AtomicPointer,
    DescriptorHeap,
    PipelineState,
    BytecodePatch,
    SubtreeRebuild,
}

pub struct SwapScheduler { /* ... */ }

impl SwapScheduler {
    pub fn schedule(&mut self, swap: PendingSwap);
    pub fn apply_pending_swaps(
        &mut self,
        handle_table: &mut HandleTable,
    ) -> u32;
    pub fn retire_old_assets(
        &mut self,
        completed_fence: u64,
    );
}

pub struct HandleTable { /* ... */ }

impl HandleTable {
    pub fn allocate(
        &mut self,
        ptr: *const u8,
        data_size: usize,
    ) -> (u32, u32);
    pub fn swap_ptr(
        &mut self,
        index: u32,
        new_ptr: *const u8,
        new_size: usize,
    ) -> (*const u8, usize);
    pub fn resolve(
        &self,
        index: u32,
        generation: u32,
    ) -> Option<*const u8>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AssetHandle<T> {
    index: u32,
    generation: u32,
    _marker: core::marker::PhantomData<T>,
}
```

### Universal Binary Asset Format (F-12.7.1)

```rust
pub const ASSET_MAGIC: [u8; 4] = *b"HAST";
pub const FORMAT_VERSION: u32 = 1;

#[repr(C)]
#[derive(Clone, Debug, Reflect)]
pub struct AssetHeader {
    pub magic: [u8; 4],
    pub format_version: u32,
    pub asset_type_id: u64,
    pub schema_version: SchemaVersion,
    pub content_hash: [u8; 32],
    pub toc_offset: u64,
    pub toc_count: u32,
    pub flags: AssetFlags,
    pub total_size: u64,
}

/// Memory-mapped asset reader with O(1) section
/// access.
pub struct AssetReader<'a> {
    data: &'a [u8],
    header: &'a AssetHeader,
    toc: &'a [SectionDescriptor],
}

impl<'a> AssetReader<'a> {
    pub fn from_bytes(
        data: &'a [u8],
    ) -> Result<Self, AssetError>;
    pub fn section(
        &self,
        name: &str,
    ) -> Result<&'a [u8], AssetError>;
    pub fn verify_integrity(
        &self,
    ) -> Result<(), AssetError>;
}

/// Builds a new asset file with sections.
pub struct AssetWriter {
    asset_type_id: u64,
    schema_version: SchemaVersion,
    sections: Vec<WriterSection>,
}

impl AssetWriter {
    pub fn add_section(
        &mut self,
        name: &str,
        section_type: SectionType,
        data: Vec<u8>,
        compression: Compression,
    ) -> &mut Self;
    pub fn build(
        self,
    ) -> Result<Vec<u8>, AssetError>;
    /// Submit the built bytes for write via the
    /// IoBridge. Returns an id; completion is polled
    /// from `IoBridge::take_result`.
    pub fn write_to(
        self,
        path: &std::path::Path,
        bridge: &mut IoBridge,
    ) -> IoRequestId;
}
```

### Material Mapping (F-12.6.25)

```rust
/// Source format for material translation.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum MaterialSource {
    GltfPbr,
    GltfSpecGloss,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum ValueTransform {
    Identity,
    Invert,
    SrgbToLinear,
    LinearToSrgb,
}

pub struct MaterialMapper {
    rules: Vec<MaterialMappingRule>,
    fallbacks: HashMap<HarTextureSlot, [f32; 4]>,
}

impl MaterialMapper {
    pub fn load_rules(
        &mut self,
        data: &[u8],
    ) -> Result<(), MaterialMapError>;
    pub fn translate(
        &self,
        source_fmt: MaterialSource,
        source: &ImportedMaterial,
    ) -> Result<
        (HarMaterialDesc, Vec<MaterialWarning>),
        MaterialMapError,
    >;
}
```

### Three-Way Merge (F-12.7.4)

```rust
pub enum MergeResult {
    Success { merged: Vec<u8> },
    AutoResolved {
        merged: Vec<u8>,
        resolutions: Vec<AutoResolution>,
    },
    Conflict {
        partial: Vec<u8>,
        conflicts: Vec<MergeConflict>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum ResolutionStrategy {
    LastWriterWins,
    Union,
    DeterministicOrder,
}

pub struct ThreeWayMerge {
    diff: StructuralDiff,
    strategies: HashMap<u64, MergeStrategy>,
}

impl ThreeWayMerge {
    pub fn merge(
        &self,
        ancestor: &AssetReader<'_>,
        ours: &AssetReader<'_>,
        theirs: &AssetReader<'_>,
    ) -> Result<MergeResult, MergeError>;
}

/// Git custom merge driver entry point.
pub fn git_merge_driver(
    ancestor_path: &Path,
    ours_path: &Path,
    theirs_path: &Path,
    output_path: &Path,
) -> Result<i32, MergeError>;
```

### Error Types

```rust
pub enum ImportError {
    FileNotFound { path: PathBuf },
    UnsupportedFormat { extension: String },
    ValidationFailed {
        diagnostics: Vec<ValidationDiagnostic>,
    },
    CorruptFile { path: PathBuf, message: String },
    HashMismatch {
        expected: ContentHash,
        actual: ContentHash,
    },
    Io(IoError),
    Cancelled,
}

pub enum HotReloadError {
    Fs(FsError),
    AssetNotFound { asset_id: AssetId },
    ImportFailed { asset_id: AssetId, message: String },
    ProcessFailed { asset_id: AssetId, message: String },
    ShaderCompileFailed {
        errors: Vec<ShaderCompileError>,
    },
    GraphCompileFailed {
        asset_id: AssetId,
        message: String,
    },
    SyncError { message: String },
    WatchDirNotFound { path: String },
    CapacityExceeded { current: u32, max: u32 },
}
```

## Data Flow

### Single File Import Lifecycle

1. Detect format from file extension
2. Submit source-file read via `IoBridge` and wait one frame for completion (sync call)
3. Check import cache (BLAKE3(source + settings + version))
4. On cache miss: run format-specific importer on worker pool via `scope()`
5. BLAKE3 hash artifact, store in CAS (deduplicates)
6. Register metadata in `MetadataStore`
7. Update `DependencyGraph` edges
8. Record `VersionEntry` for history
9. Insert cache entry for future hits

### Asset Dependency Graph Invalidation

When a source file changes, cascading invalidation walks the reverse dependency graph. Every
dependent asset is marked dirty and queued for reprocessing even if it was not touched directly.

```mermaid
flowchart TD
    S[source.png modified] --> H[recompute BLAKE3]
    H --> C{hash == cached?}
    C -->|yes| D1[no-op]
    C -->|no| U[update CAS entry]
    U --> M[invalidate MetadataStore]
    M --> DG[walk DependencyGraph reverse edges]
    DG --> L[build ordered dirty list]
    L --> R[reprocess each dependent]
    R --> UD[update dependents]
    UD --> DG2{any new dependents?}
    DG2 -->|yes| DG
    DG2 -->|no| F[fire ReloadRequest per asset]
```

```rust
/// Cascade dependency invalidation. Pure function:
/// given a root set of changed assets and the
/// reverse edges of the dependency graph, returns
/// the fully closed transitive set in topological
/// order, ready to hand to the reload coordinator.
///
/// Runs on the main thread; caller already owns
/// &mut MetadataStore and &DependencyGraph.
pub fn cascade_invalidation(
    roots: &[AssetId],
    graph: &DependencyGraph,
) -> Vec<AssetId> {
    let mut visited = HashSet::new();
    let mut order = Vec::new();
    let mut stack: Vec<AssetId> = roots.iter().copied().collect();
    while let Some(id) = stack.pop() {
        if !visited.insert(id) {
            continue;
        }
        order.push(id);
        for &dependent in graph.dependents_of(id) {
            if !visited.contains(&dependent) {
                stack.push(dependent);
            }
        }
    }
    // Topologically sort the dirty subset so
    // downstream assets reprocess after upstream.
    graph.topological_sort(&mut order);
    order
}
```

The coordinator then submits each dirty asset's reprocessing as a worker job. Hot-reload swap
follows [../core-runtime/hot-reload-protocol.md](../core-runtime/hot-reload-protocol.md).

### Batch Import with Cancellation

1. Begin `MetadataTransaction`
2. Fan out imports across `ThreadPool::scope`
3. Each task checks `CancellationToken` before work
4. On completion: commit transaction atomically
5. On cancel: drop transaction (automatic rollback)

### Hot Reload End-to-End

1. `FileWatcher` delivers raw filesystem event
2. `AssetWatcher` batches (50 ms window), deduplicates, maps paths to `AssetId`
3. `ChangeDetector` BLAKE3-filters false positives, walks dependency graph for transitive
   dependents, classifies by `ReloadKind`
4. `ReloadCoordinator` spawns async re-import/reprocess
5. Type-specific reloaders handle domain logic:
   - **Shader**: recompile permutations, build new PSOs
   - **Logic graph**: check variable layout compatibility
   - **UI**: capture state snapshot, rebuild, restore
6. `SwapScheduler` enqueues `PendingSwap`
7. At frame boundary: `apply_pending_swaps()` atomically replaces pointers in `HandleTable`
8. Old data retired after GPU fence completes

## Platform Considerations

### Async I/O

| Platform | I/O Backend | Usage |
|----------|-------------|-------|
| Windows | Tokio (IOCP) | Overlapped reads/writes |
| macOS | Tokio (kqueue) | Async file ops |
| Linux | Tokio (epoll) | Async file ops |

### File Watcher Backends

| Platform | API |
|----------|-----|
| Windows | `ReadDirectoryChangesExW` |
| macOS | `FSEvents` |
| Linux | `inotify_add_watch` |

### Swap Strategy per Asset Type

| Asset Type | Strategy | Latency |
|------------|----------|---------|
| Texture | DescriptorHeap | < 2 s |
| Mesh | AtomicPointer | < 3 s |
| Material | AtomicPointer | < 2 s |
| Shader | PipelineState | < 5 s |
| Logic graph | BytecodePatch | < 500 ms |
| UI layout | SubtreeRebuild | < 500 ms |
| Audio | AtomicPointer | < 1 s |

### Supported Import Formats

| Category | Format | Extensions |
|----------|--------|------------|
| Geometry | glTF 2.0 | `.gltf`, `.glb` |
| Geometry | Alembic | `.abc` |
| Texture | PNG | `.png` |
| Texture | EXR | `.exr` |
| Texture | KTX2 | `.ktx2` |
| Audio | WAV | `.wav` |
| Audio | FLAC | `.flac` |

### DCC-to-Format Export Guidance

| DCC Tool | Geometry | Textures | Animation |
|----------|----------|----------|-----------|
| Blender | glTF 2.0 | PNG/EXR | glTF 2.0 |
| Maya | glTF 2.0 | PNG/EXR | glTF 2.0 / Alembic |
| Houdini | Alembic | EXR | Alembic |
| 3ds Max | glTF 2.0 | PNG/EXR | glTF 2.0 |
| ZBrush | glTF 2.0 | PNG/EXR | n/a |
| Substance | n/a | PNG/EXR | n/a |
| Photoshop | n/a | PNG/EXR | n/a |

1. **Blender** -- native glTF 2.0 exporter; use for meshes, skeletal animation, and full scenes
2. **Maya** -- glTF via Autodesk plugin for rigged meshes; Alembic for vertex cache animation
3. **Houdini** -- Alembic for particles and vertex cache animation; EXR for baked textures
4. **3ds Max** -- Babylon.js glTF exporter plugin
5. **ZBrush** -- export high-poly as glTF via decimation or retopology
6. **Substance** -- export texture maps as PNG or EXR (linear color space preferred)
7. **Photoshop** -- export textures as PNG (sRGB) or EXR (linear HDR)

### CAS Storage Paths

| Platform | Path |
|----------|------|
| Windows | `%APPDATA%\Harmonius\cas\` |
| macOS | `~/Library/App Support/Harmonius/cas/` |
| Linux | `~/.local/share/harmonius/cas/` |

### Proposed Dependencies

| Crate | Purpose |
|-------|---------|
| `blake3` | Content hashing (SIMD) |
| `claxon` | FLAC decoding |
| `exr` | EXR image decoding |
| `gltf` | glTF 2.0 parsing |
| `hound` | WAV decoding |
| `image` | PNG/texture decoding |
| `ktx2` | KTX2 texture decoding |
| `tokio` | Async runtime and I/O |
| `uuid` | Asset ID generation |

## Test Plan

Test cases are defined inline below.

### Unit Tests

| Test | Req |
|------|-----|
| `test_content_hash_deterministic` | R-12.3.1 |
| `test_cas_store_and_load` | R-12.3.1 |
| `test_cas_deduplication` | R-12.3.1 |
| `test_cas_gc_removes_unreferenced` | R-12.3.1 |
| `test_metadata_put_get` | R-12.3.2 |
| `test_metadata_transaction_commit` | R-12.1.5 |
| `test_metadata_transaction_rollback` | R-12.1.5 |
| `test_dep_graph_transitive` | R-12.3.4 |
| `test_dep_graph_cycle_detect` | R-12.2.8 |
| `test_cache_hit_skips_import` | R-12.3.3 |
| `test_validate_gltf_header` | R-12.1.4 |
| `test_search_by_text` | R-12.3.5 |
| `test_version_record_and_history` | R-12.3.10 |
| `test_importer_registry_find` | US-12.1.7 |
| `test_change_detector_filters_false_positive` | R-12.4.1 |
| `test_change_detector_resolves_dependents` | R-12.4.2 |
| `test_handle_table_swap_preserves_handle` | R-12.4.2 |
| `test_swap_scheduler_applies_at_boundary` | US-12.4.9 |
| `test_shader_reloader_error_preserves_old` | R-12.4.3 |
| `test_logic_graph_compatible_layout` | R-12.4.4 |
| `test_ui_reloader_preserves_scroll` | R-12.4.5 |
| `test_debounce_coalesces_rapid_events` | R-12.4.1 |
| `test_editor_sync_property_roundtrip` | R-12.4.7 |
| `test_asset_header_roundtrip` | R-12.7.1 |
| `test_section_o1_access` | R-12.7.1 |
| `test_material_mapper_gltf_pbr` | R-12.6.25 |
| `test_merge_non_overlapping` | R-12.7.4 |
| `test_merge_conflict_detected` | R-12.7.4 |
| `test_binary_diff_apply_roundtrip` | R-12.7.2 |

### Integration Tests

| Test | Req |
|------|-----|
| `test_import_gltf_end_to_end` | R-12.1.1 |
| `test_import_alembic_end_to_end` | R-12.1.1 |
| `test_import_png_end_to_end` | R-12.1.2 |
| `test_import_exr_end_to_end` | R-12.1.2 |
| `test_import_ktx2_end_to_end` | R-12.1.2 |
| `test_import_wav_end_to_end` | R-12.1.3 |
| `test_import_flac_end_to_end` | R-12.1.3 |
| `test_batch_import_100_assets` | R-12.1.5 |
| `test_batch_cancel_rollback` | R-12.1.5 |
| `test_incremental_reimport` | R-12.3.4 |
| `test_texture_hot_reload_e2e` | R-12.4.2 |
| `test_shader_hot_reload_valid` | R-12.4.3 |
| `test_logic_graph_reload_500ms` | R-12.4.4 |
| `test_hot_reload_no_memory_leak` | US-12.4.10 |
| `test_headless_batch_identical` | R-12.6.26 |
| `test_full_merge_workflow` | R-12.7.4 |

### Benchmarks

| Benchmark | Target |
|-----------|--------|
| BLAKE3 hash 1 GB | < 1.5 s single core |
| CAS store 10 MB blob | < 20 ms |
| Import 100 glTF assets | < 10 s (8 cores) |
| Cache lookup | < 0.1 ms |
| Full-text search (1M entries) | < 100 ms |
| Texture hot reload latency | < 2 s |
| Shader hot reload latency | < 5 s |
| Logic graph hot reload | < 500 ms |
| Handle table resolve | < 10 ns |
| File change detection | < 500 ms |
| glTF parse 100k verts | < 10 ms |
| Asset mmap section access | < 1 us |
| Three-way merge (clean) | < 200 ms |

## Open Questions

1. **Metadata persistence format.** Custom binary with WAL, SQLite via `rusqlite`, or per-asset RON
   files? Impacts search at scale.
2. **CAS blob compression.** Compress in CAS (LZ4/Zstd) or raw? Streaming subsystem compresses at
   archive level, so CAS compression may be redundant.
3. **Asset ID stability across branches.** Independent UUID generation creates duplicates on merge.
   Deterministic IDs from source path break on rename.
4. **Descriptor set update atomicity.** Vulkan descriptor set updates require double-buffering or
   descriptor indexing for texture hot reload.
5. **Structural diff granularity.** Per-vertex mesh diff (precise but slow) vs summary statistics
   (fast but coarse)?
6. **Alembic large cache handling.** Alembic files can be very large (multi-GB). Stream-parse or
   require pre-split sub-files?

## Review Feedback

### RF-1: Replace all Tokio with platform-native I/O + job system

The entire design is built on Tokio (7+ references). Replace with:

- Platform-native I/O: io_uring (Linux), IOCP (Windows), GCD dispatch_io (Apple)
- Main thread submits I/O, polls completions, posts as jobs via crossbeam-channel
- Import functions become synchronous, run on job system workers
- `CAS.store()`, `CAS.load()`, `MetadataStore` methods use platform I/O layer
- Remove `tokio` from proposed dependencies and cross-cutting deps table

All `async fn` in the design must be rethought as synchronous functions that submit I/O requests and
receive completions via channels.

### RF-2: Remove AsyncRwLock

`MetadataStore` and `ImportCoordinator` use `AsyncRwLock`. This violates "no custom async
primitives" and "no shared mutable state." Replace with:

- Channel-serialized access: single worker owns the data, processes requests from a
  crossbeam-channel
- Or make them single-owner resources accessed within job system scope

### RF-3: Remove all Reflect derives

16 structs derive `Reflect`. Zero reflection is the constraint. Remove all `#[derive(Reflect)]`. If
type metadata is needed for the editor, use static codegen (generated TypeDescriptor, serde
derives).

### RF-4: Add GPU DMA asset loading

Add a GPU asset loading path that bypasses CPU memory entirely:

- Windows: Vulkan staging buffers with GPU decompression via compute queue
- Apple: Vulkan staging buffers (VkQueue transfer) for disk-to-GPU DMA
- Linux: io_uring to CPU staging buffer, then Vulkan upload

Add `GpuAssetLoader` with platform backends. Textures and meshes destined for GPU should skip CPU
processing when possible. The asset state machine should include a `GpuUploading` state for assets
in the DMA pipeline.

### RF-5: Add asset state machine

Add per-handle state tracking:

```rust
pub enum AssetState {
    Queued,
    Loading,
    BytesReady,
    Processing,
    GpuUploading,
    Ready,
    Failed(AssetError),
}
```

Store in the handle table. ECS systems query state via `asset_server.state(handle)`.

### RF-6: Add synchronous AssetServer

Add the user-facing synchronous API per constraints:

```rust
pub struct AssetServer { /* ... */ }

impl AssetServer {
    /// Load an asset. Returns handle immediately.
    /// I/O happens asynchronously on main thread.
    pub fn load<T: Asset>(
        &self, path: &str,
    ) -> AssetHandle<T>;

    /// Query asset state.
    pub fn state<T: Asset>(
        &self, handle: AssetHandle<T>,
    ) -> AssetState;

    /// Check if ready.
    pub fn is_ready<T: Asset>(
        &self, handle: AssetHandle<T>,
    ) -> bool;
}
```

`AssetServer` is an ECS resource (`Res<AssetServer>`). Systems call `load()` synchronously; the I/O
is dispatched to the main thread internally.

### RF-7: Add resource residency manager

Add streaming and memory management:

- Memory budget tracking per asset type (textures, meshes, audio)
- LRU eviction when budget is exceeded
- Priority-based unloading (distance from camera, last access time)
- Integration with spatial queries for distance-based streaming
- Prefetching based on camera velocity and predicted position

This is critical for open-world games. The residency manager owns the decision of which assets are
loaded and which are evicted.

### RF-8: Use rkyv for baked assets

Add `rkyv` to proposed dependencies. Use rkyv for typed section data within the binary asset format:

- Baked assets use `#[derive(Archive, rkyv::Serialize, rkyv::Deserialize)]`
- mmap access without deserialization (zero-copy)
- Throughput well above the 500 MB/s target

The custom `AssetHeader` / `SectionDescriptor` format can remain as the container envelope. rkyv
handles the typed payload within each section.

### RF-9: Create companion test cases file

Extract the inline test tables into `docs/design/content-pipeline/asset-pipeline-test-cases.md` with
full TC-X.Y.Z.N IDs, explicit inputs, and expected outputs per design rules.

### RF-10: Fix async fn in Importer trait

`trait Importer` uses `async fn import()`. With Tokio removed, importers become synchronous
functions that run on job system workers. I/O within an importer is submitted to the main thread and
completions are received via channel. Replace `async fn` with synchronous `fn`.

### RF-11: No async/await anywhere

Remove ALL `async fn`, `.await`, and `Future` types from the design body. All code is synchronous.
I/O is submitted to the main thread via channels and completions arrive as jobs. This applies to
`CAS.store()`, `CAS.load()`, `AssetReader`, `Importer::import()`, and all filesystem operations.

### RF-12: Residency manager with usage-based eviction

The residency manager must track three signals for eviction decisions:

1. **Reference count** — never evict assets currently in use by any system (active Handle
   references)
2. **Last access time** — LRU eviction for assets not accessed in N frames
3. **Predictive priority** — assets likely to be needed soon get higher priority. Prediction uses:
   - Player position + velocity → next terrain tiles, next streaming cells
   - Camera frustum direction → assets in predicted view
   - Navigation path → assets along the planned route
   - Spatial index query → nearby assets ranked by distance

Eviction policy:

```rust
pub struct ResidencyManager {
    budget_per_type: HashMap<AssetType, usize>,
    entries: Vec<ResidencyEntry>,
}

pub struct ResidencyEntry {
    handle: ErasedHandle,
    asset_type: AssetType,
    size_bytes: usize,
    ref_count: u32,
    last_access_frame: u64,
    priority: f32,
}

impl ResidencyManager {
    /// Called when memory pressure exceeds budget.
    /// Evicts lowest-priority unreferenced assets.
    pub fn evict_to_budget(&mut self) -> Vec<ErasedHandle>;

    /// Update priorities based on camera position,
    /// velocity, and spatial queries.
    pub fn update_priorities(
        &mut self,
        camera_pos: Vec3,
        camera_vel: Vec3,
        spatial: &SpatialIndex,
    );

    /// Prefetch assets predicted to be needed soon.
    pub fn prefetch(
        &mut self,
        predicted: &[AssetId],
        asset_server: &AssetServer,
    );
}
```

The eviction loop: never evict `ref_count > 0`, then sort remaining by `priority * recency_weight`,
evict lowest until within budget. Run once per frame after all systems have released handles.

Scene transitions trigger explicit bulk unloading. When switching scenes:

1. Diff old scene asset set vs new scene asset set
2. Assets in old but not new → mark for unload (drop ref count)
3. Assets in both → keep (no reload needed)
4. Assets in new but not old → queue for load

```rust
impl ResidencyManager {
    /// Explicit scene transition. Unloads assets
    /// exclusive to the old scene, keeps shared,
    /// queues new.
    pub fn transition_scene(
        &mut self,
        old_assets: &HashSet<AssetId>,
        new_assets: &HashSet<AssetId>,
        asset_server: &AssetServer,
    );
}
```

This avoids the LRU eviction delay for scene changes — assets from the old scene are unloaded
immediately rather than waiting for memory pressure. Shared assets (UI, player model, global audio)
survive the transition without reload.

**LOD-aware eviction.** When an entity moves closer to the camera, it transitions from a low-LOD
mesh to a high-LOD mesh. The low-LOD asset is no longer needed and should be unloaded. Conversely,
moving away transitions high→low and the high-LOD can be evicted:

- Each LOD level is a separate asset with its own Handle
- The LOD system updates which Handle is active per entity
- Inactive LOD handles drop their ref count → eligible for eviction
- The residency manager sees `ref_count == 0` and evicts on next pass

**OS memory pressure events.** The platform layer (main thread) receives low-memory warnings from
the OS:

- iOS: `didReceiveMemoryWarning` / `os_proc_available_memory`
- Android: `onTrimMemory` / `ComponentCallbacks2`
- Windows: `CreateMemoryResourceNotification`
- Linux: cgroup memory pressure notifications

On receiving a memory pressure event, the residency manager enters emergency eviction mode:

```rust
impl ResidencyManager {
    /// Emergency eviction triggered by OS memory
    /// pressure. Unloads all unreferenced assets
    /// below the given priority threshold.
    pub fn emergency_evict(
        &mut self,
        priority_threshold: f32,
    ) -> usize; // bytes freed
}
```

Emergency eviction unloads everything with `ref_count == 0` below the priority threshold, regardless
of LRU age. If still insufficient, it can downgrade LOD levels (swap high-LOD for low-LOD on active
entities) to free memory while keeping content visible.
