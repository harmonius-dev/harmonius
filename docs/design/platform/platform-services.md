# Platform Services Design

## Requirements Trace

> **Canonical sources:** [features/](../../features/), [requirements/](../../requirements/),
> [user-stories/](../../user-stories/).

### OS Integration (F-14.2 / R-14.2)

| Feature  | Requirement |
|----------|-------------|
| F-14.2.1 | R-14.2.1    |
| F-14.2.2 | R-14.2.2    |
| F-14.2.3 | R-14.2.3    |
| F-14.2.4 | R-14.2.4    |
| F-14.2.5 | R-14.2.5    |
| F-14.2.6 | R-14.2.6    |

1. **F-14.2.1** -- Clipboard read/write for text and images
2. **F-14.2.2** -- Native file/folder picker dialogs
3. **F-14.2.3** -- Toast notifications and system tray icons
4. **F-14.2.4** -- Drag-and-drop with MIME/extension validation
5. **F-14.2.5** -- Keyboard layout detection and dead keys
6. **F-14.2.6** -- IME integration for CJK text entry

### Crash Reporting and Diagnostics (F-14.4 / R-14.4)

| Feature  | Requirement |
|----------|-------------|
| F-14.4.1 | R-14.4.1    |
| F-14.4.2 | R-14.4.2    |
| F-14.4.3 | R-14.4.3    |
| F-14.4.4 | R-14.4.4    |
| F-14.4.5 | R-14.4.5    |
| F-14.4.6 | R-14.4.6    |

1. **F-14.4.1** -- Crash handler with out-of-process minidump
2. **F-14.4.2** -- Debug symbol upload and server symbolication
3. **F-14.4.3** -- Crash aggregation by stack signature
4. **F-14.4.4** -- Structured logging with async ring buffer
5. **F-14.4.5** -- Lock-free per-thread performance counters
6. **F-14.4.6** -- GPU crash breadcrumbs per render pass

### Platform Services (F-14.5 / R-14.5)

| Feature   | Requirement | User Story           |
|-----------|-------------|----------------------|
| F-14.5.1  | R-14.5.1    | US-14.5.1, 7, 8, 13 |
| F-14.5.2  | R-14.5.2    | US-14.5.2, 15       |
| F-14.5.3  | R-14.5.3    | US-14.5.3           |
| F-14.5.4  | R-14.5.4    | US-14.5.4, 16       |
| F-14.5.5  | R-14.5.5    | US-14.5.5, 12, 17   |
| F-14.5.6  | R-14.5.6    | US-14.5.6, 11       |
| F-14.5.7  | R-14.5.7    | US-14.5.10, 14      |

1. **F-14.5.1** -- Cross-platform achievements with deferred unlock
2. **F-14.5.2** -- Leaderboards with batching and rate-limit caching
3. **F-14.5.3** -- Rich presence throttled to 1 update / 15 s
4. **F-14.5.4** -- Platform voice/party bridge with Vivox fallback
5. **F-14.5.5** -- Cloud storage with conflict resolution dialog
6. **F-14.5.6** -- Entitlement/DLC/subscription verification
7. **F-14.5.7** -- Console certification compliance

### Storage (F-14.5.8--F-14.5.12)

| Feature   | Requirement | User Story           |
|-----------|-------------|----------------------|
| F-14.5.8  | R-14.5.8    | US-14.5.17, 18, 19  |
| F-14.5.9  | R-14.5.9    | US-14.5.20, 21      |
| F-14.5.10 | R-14.5.10   | US-14.5.22, 23      |
| F-14.5.11 | R-14.5.11   | US-14.5.24, 25, 26  |
| F-14.5.12 | R-14.5.12   | US-14.5.27, 28      |

1. **F-14.5.8** -- User prefs: TOML, atomic write, cloud sync
2. **F-14.5.9** -- Player cache: LRU, 10 GB default
3. **F-14.5.10** -- Developer cache: BLAKE3 keys, 3-tier
4. **F-14.5.11** -- PSO cache: GPU+driver versioned, < 1 ms
5. **F-14.5.12** -- Temp file manager: RAII, orphan cleanup

### Filesystem (F-14.6 / R-14.6)

| Feature  | Requirement |
|----------|-------------|
| F-14.6.1 | R-14.6.1    |
| F-14.6.2 | R-14.6.2    |
| F-14.6.3 | R-14.6.3    |
| F-14.6.4 | R-14.6.4    |
| F-14.6.5 | R-14.6.5    |
| F-14.6.6 | R-14.6.6    |
| F-14.6.7 | R-14.6.7    |

1. **F-14.6.1** -- Async file I/O via Tokio
2. **F-14.6.2** -- Async create/delete with batch unlink
3. **F-14.6.3** -- Async metadata and batch stat
4. **F-14.6.4** -- Directory enumeration with depth and glob
5. **F-14.6.5** -- File watching with debounce
6. **F-14.6.6** -- BLAKE3 content hash false-positive filter
7. **F-14.6.7** -- Canonical path resolution cross-platform

### SDK Integration (F-14.8 / R-14.8)

| Feature  | Requirement          | User Story          |
|----------|----------------------|---------------------|
| F-14.8.1 | R-14.8.1, R-14.8.2  | US-14.8.1, 2, 3    |
| F-14.8.2 | R-14.8.3, R-14.8.4  | US-14.8.8, 9, 12   |
| F-14.8.3 | R-14.8.5, R-14.8.6  | US-14.8.4, 5, 6, 7 |
| F-14.8.4 | R-14.8.7, R-14.8.8  | US-14.8.10, 11     |
| F-14.8.5 | R-14.8.9, R-14.8.10 | US-14.8.10         |

1. **F-14.8.1** -- Server-side console build service
2. **F-14.8.2** -- Proprietary SDK isolation
3. **F-14.8.3** -- Shared build server
4. **F-14.8.4** -- Remote console deployment
5. **F-14.8.5** -- Console build artifacts

## Overview

Consolidates OS integration, crash reporting, filesystem, platform services, storage, and SDK
integration. All share `harmonius_platform`, `Tokio runtime`, and overlapping F-14.5.x features. All
use static dispatch (`cfg`-gated). No `Arc`, `Rc`, `Cell`, `RefCell`.

- **OS integration** -- clipboard, file dialogs, notifications, drag-drop, keyboard, IME
- **Crash reporting** -- out-of-process handler, logging, perf counters, GPU breadcrumbs
- **Filesystem** -- async I/O via Tokio; no `std::fs` anywhere
- **Platform services** -- achievements, leaderboards, presence, cloud, entitlements, auth
- **Storage** -- preferences (TOML), player cache (LRU), dev cache (BLAKE3), PSO cache, temp files
- **SDK integration** -- IAP, subscriptions, matchmaking, anti-cheat, friends, mods, account linking

## Architecture

### Module Boundaries

```mermaid
graph TD
    subgraph "harmonius_platform::os"
        CB[Clipboard]
        FD[FileDialog]
        NT[Notifications]
        DD[DragDrop]
        KB[Keyboard]
        IM[IME]
    end

    subgraph "harmonius_platform::crash"
        CH[CrashHandler]
        SY[Symbols]
        LG[Logger]
        TL[Telemetry]
        GB[GpuBreadcrumbs]
    end

    subgraph "harmonius_platform::filesystem"
        FO[FileOps]
        MD[Metadata]
        EN[Enumerate]
        FW[FileWatcher]
        HS[ContentHasher]
        PA[PathResolver]
    end

    subgraph "harmonius_platform::services"
        PS[PlatformServices]
        ACH[AchievementService]
        LDR[LeaderboardService]
        RP[RichPresenceService]
        CS[CloudStorageService]
        ENT[EntitlementService]
    end

    subgraph "harmonius_platform::storage"
        PRF[PreferencesStore]
        PLC[PlayerCache]
        DVC[DeveloperCache]
        PSO[PsoCacheStore]
        TMP[TempFileManager]
    end

    subgraph "harmonius_platform::sdk"
        IAP[PurchaseApi]
        SUB[SubscriptionApi]
        MM[MatchmakingApi]
        AC[AntiCheatApi]
        FR[FriendsApi]
        MOD[ModApi]
    end

    subgraph "harmonius_core"
        IO[Tokio runtime]
        EV[EventDispatcher]
    end

    FO --> IO
    FW --> IO
    PLC --> IO
    DVC --> IO
    PSO --> IO
    TMP --> IO
    PRF --> CS
    CH --> LG
    FW --> HS

    PS --> ACH
    PS --> LDR
    PS --> RP
    PS --> CS
    PS --> ENT
```

### Core Data Structures

```mermaid
classDiagram
    class PlatformServices {
        +achievements AchievementService
        +leaderboards LeaderboardService
        +presence RichPresenceService
        +cloud CloudStorageService
        +entitlements EntitlementService
        +auth AuthenticationService
        +profile UserProfileService
        +init(reactor) Result
        +shutdown(reactor) Result
    }

    class AchievementService {
        -defs Vec~AchievementDef~
        -progress HashMap
        -deferred DeferredQueue
        +unlock(id, reactor) Result
        +increment(id, amount, reactor) Result
        +state(id) AchievementProgress
        +sync(reactor) Result
        +flush_deferred(reactor) Result
    }

    class AchievementDef {
        +id AchievementId
        +name StringKey
        +description StringKey
        +icon AssetId
        +hidden bool
        +platform_ids PlatformIdMap
    }

    class AchievementProgress {
        +id AchievementId
        +current u32
        +target u32
        +state UnlockState
    }

    class UnlockState {
        <<enumeration>>
        Locked
        Unlocked
        Pending
    }

    class LeaderboardService {
        -cache HashMap
        -cache_ttl_secs u32
        -pending_submissions Vec
        +submit(id, score, reactor) Result
        +query(id, scope, offset, count, reactor) Result
        +flush_pending(reactor) Result
    }

    class LeaderboardSort {
        <<enumeration>>
        Ascending
        Descending
    }

    class LeaderboardScope {
        <<enumeration>>
        Global
        FriendsOnly
        AroundPlayer
    }

    class LeaderboardRow {
        +rank u32
        +player_name String
        +score i64
        +player_id Option~String~
    }

    class RichPresenceService {
        -current Option~PresenceState~
        -last_update u64
        -throttle_interval_ms u64
        +update(state, reactor) Result
        +clear(reactor) Result
        +current() PresenceState
    }

    class PresenceState {
        +activity String
        +zone Option~String~
        +party_size Option~u32~
        +party_max Option~u32~
    }

    class CloudStorageService {
        -quota_bytes u64
        -used_bytes u64
        +upload(key, data, reactor) Result
        +download(key, reactor) Result
        +check_conflict(key, data, ts, reactor) Result
        +metadata(key, reactor) Result
    }

    class ConflictResult {
        <<enumeration>>
        NoConflict
        Conflict
    }

    class EntitlementService {
        -entitlements Vec~Entitlement~
        -last_check u64
        -poll_interval_secs u32
        +refresh(reactor) Result
        +is_owned(id) bool
        +is_subscription_active(id) bool
    }

    class EntitlementKind {
        <<enumeration>>
        BaseGame
        Expansion
        CosmeticDlc
        Subscription
    }

    class Entitlement {
        +id String
        +kind EntitlementKind
        +owned bool
        +expires Option~u64~
    }

    class AuthenticationService {
        -user Option~PlatformUser~
        -token Option~AuthToken~
        +authenticate(reactor) Result
        +current_user() PlatformUser
        +token(reactor) Result
    }

    class PlatformUser {
        +platform_id String
        +display_name String
        +avatar_url Option~String~
    }

    class AuthToken {
        +token String
        +expires_at u64
    }

    class UserProfileService {
        -local_profile Option~UserProfile~
        -friends_cache Vec~UserProfile~
        +fetch_local(reactor) Result
        +fetch_friends(reactor) Result
    }

    class UserProfile {
        +display_name String
        +avatar Option~AssetId~
        +online_status OnlineStatus
        +platform PlatformKind
    }

    class OnlineStatus {
        <<enumeration>>
        Online
        Away
        DoNotDisturb
        Offline
    }

    class PlatformKind {
        <<enumeration>>
        Steam
        Xbox
        PlayStation
        GameCenter
        Nintendo
        Apple
        GooglePlay
        Eos
    }

    class DeferredQueue {
        -pending Vec~DeferredEntry~
        -max_retries u32
        -base_backoff_ms u64
        +enqueue(item)
        +drain_ready(now) Vec
        +requeue(entry) bool
    }

    class DeferredEntry {
        +item T
        +enqueued_at u64
        +retry_count u32
        +next_retry_at u64
    }

    PlatformServices --> AchievementService
    PlatformServices --> LeaderboardService
    PlatformServices --> RichPresenceService
    PlatformServices --> CloudStorageService
    PlatformServices --> EntitlementService
    PlatformServices --> AuthenticationService
    PlatformServices --> UserProfileService
    AchievementService --> AchievementDef
    AchievementService --> AchievementProgress
    AchievementService --> DeferredQueue
    AchievementProgress --> UnlockState
    LeaderboardService --> LeaderboardRow
    LeaderboardService ..> LeaderboardSort
    LeaderboardService ..> LeaderboardScope
    RichPresenceService --> PresenceState
    CloudStorageService ..> ConflictResult
    EntitlementService --> Entitlement
    Entitlement --> EntitlementKind
    AuthenticationService --> PlatformUser
    AuthenticationService --> AuthToken
    UserProfileService --> UserProfile
    UserProfile --> OnlineStatus
    UserProfile --> PlatformKind
    DeferredQueue --> DeferredEntry
```

### OS, Crash, Filesystem, and Storage Types

```mermaid
classDiagram
    class Clipboard {
        +read_text() Result
        +write_text(text) Result
        +read_image() Result
        +write_image(image) Result
    }

    class ImageData {
        +width u32
        +height u32
        +pixels Vec~u8~
    }

    class FileDialog {
        +open_file(config) Result
        +save_file(config) Result
        +pick_folder(title, dir) Result
    }

    class Notifications {
        +show_notification(config) Result
        +create_tray_icon(tooltip, items) Result
    }

    class NotificationUrgency {
        <<enumeration>>
        Low
        Normal
        Critical
    }

    class DragDropHandler {
        +register(window, filter) Result
        +poll_event() Option~DragEvent~
        +respond(response)
    }

    class DragEvent {
        <<enumeration>>
        Enter
        Over
        Drop
        Leave
    }

    class DragResponse {
        <<enumeration>>
        Accept
        Reject
    }

    class Keyboard {
        +active_layout() KeyboardLayout
        +translate_key(scancode) DeadKeyResult
        +poll_layout_change() Option
    }

    class DeadKeyResult {
        <<enumeration>>
        Char
        Pending
        Composed
        Cancelled
    }

    class ImeHandler {
        +attach(window) Result
        +set_position(pos)
        +set_enabled(enabled)
        +poll_event() Option~ImeEvent~
    }

    class ImeEvent {
        <<enumeration>>
        Composition
        Commit
        CandidateList
        Cancel
    }

    class CrashHandler {
        +install(config) Result
        +set_metadata(key, value)
        +attach_gpu_breadcrumbs(bc)
        +pending_dumps() Vec
    }

    class Logger {
        +new(filter, sinks, cap) Logger
        +log(record)
        +flush()
        +set_filter(filter)
    }

    class Severity {
        <<enumeration>>
        Trace
        Debug
        Info
        Warn
        Error
        Fatal
    }

    class PerfCounters {
        +increment(name)
        +increment_by(name, amount)
        +gauge(name, value)
        +flush() Snapshot
    }

    class GpuBreadcrumbs {
        +new(device) Result
        +begin_pass(pass_id)
        +end_pass(pass_id)
        +read_last_completed() Option
    }

    class AsyncFile {
        +open(path, flags) Result
        +read(buf, offset) Result
        +write(data, offset) Result
        +flush() Result
        +close() Result
    }

    class OpenFlags {
        +read bool
        +write bool
        +create bool
        +truncate bool
        +append bool
    }

    class FileWatcher {
        +new(debounce_ms) Result
        +watch(path, recursive) Result
        +unwatch(id) Result
    }

    class FileEventKind {
        <<enumeration>>
        Created
        Modified
        Deleted
        Renamed
    }

    class ContentHasher {
        +hash_file(path) Result
        +has_content_changed(path, hash) Result
        +cache_hash(path, hash)
    }

    class CanonicalPath {
        +resolve(path) Result
        +as_str() str
        +file_name() Option
        +parent() Option
        +join(component) Result
    }

    class PreferencesStore {
        -values HashMap
        -dirty bool
        +load(path, cloud, reactor) Result
        +get(key) PrefValue
        +set(key, value)
        +save(cloud, reactor) Result
    }

    class PrefValue {
        <<enumeration>>
        Bool
        Int
        Float
        String
    }

    class PlayerCache {
        -root PathBuf
        -max_bytes u64
        +put(key, cat, data, reactor) Result
        +get(key, reactor) Result
        +evict_to_budget(reactor) Result
        +stats() CacheStats
    }

    class CacheCategory {
        <<enumeration>>
        AssetBundle
        DlcContent
        ModPackage
        StreamingData
        GenerationOutput
    }

    class DeveloperCache {
        -root PathBuf
        -shared_url Option~String~
        +lookup(hash, cat, reactor) Result
        +store(hash, cat, data, reactor) Result
        +hash(data) ContentHash
    }

    class CacheHitTier {
        <<enumeration>>
        Local
        SharedNetwork
        Miss
    }

    class PsoCacheStore {
        -gpu_driver GpuDriverKey
        -entries HashMap
        +load_all(reactor) Result
        +store(key, data, reactor) Result
        +get(key, reactor) Result
    }

    class TempFileManager {
        -root PathBuf
        -max_bytes u64
        +init(root, max, reactor) Result
        +allocate(name) Result
        +cleanup_orphans(reactor) Result
    }

    Clipboard --> ImageData
    Notifications --> NotificationUrgency
    DragDropHandler --> DragEvent
    DragDropHandler --> DragResponse
    Keyboard --> DeadKeyResult
    ImeHandler --> ImeEvent
    CrashHandler --> GpuBreadcrumbs
    CrashHandler --> Logger
    Logger --> Severity
    AsyncFile --> OpenFlags
    FileWatcher --> ContentHasher
    FileWatcher --> FileEventKind
    PreferencesStore --> PrefValue
    PlayerCache --> CacheCategory
    DeveloperCache ..> CacheHitTier
```

### SDK Integration Types

```mermaid
classDiagram
    class PurchaseApi {
        +query_products(ids) Result
        +initiate_purchase(id) Result
        +restore_purchases() Result
        +validate_receipt(receipt) Result
    }

    class PlatformReceipt {
        +transaction_id TransactionId
        +product_id ProductId
        +platform PlatformKind
        +receipt_data Vec~u8~
        +timestamp u64
    }

    class ValidationResult {
        +valid bool
        +transaction_id TransactionId
        +product_id ProductId
        +is_duplicate bool
    }

    class ReceiptValidator {
        +validate(receipt) Result
        +validate_with_retry(receipt, max) Result
    }

    class SubscriptionApi {
        +check_status(id) Result
        +manage_renewal(id, action) Result
        +handle_grace_period(id) Result
    }

    class SubState {
        <<enumeration>>
        Active
        GracePeriod
        BillingRetry
        Expired
        Cancelled
        Revoked
        Paused
    }

    class RenewalAction {
        <<enumeration>>
        EnableAutoRenew
        DisableAutoRenew
        UpgradeTier
        DowngradeTier
    }

    class MatchmakingApi {
        +create_lobby(config) Result
        +find_match(criteria) Result
        +join_lobby(id) Result
    }

    class VoiceChatApi {
        +start_session(channel) Result
        +stop_session() Result
        +mute_player(id, muted) Result
        +set_spatial_position(pos) Result
    }

    class AntiCheatApi {
        +initialize() Result
        +report_player(id, reason) Result
        +check_ban_status(id) Result
    }

    class FriendsApi {
        +list_friends() Result
        +send_invite(id, context) Result
        +set_rich_presence(state) Result
    }

    class ModApi {
        +upload(manifest) Result
        +download(id) Result
        +subscribe(id) Result
        +rate(id, score) Result
    }

    class AccountLinker {
        +link(engine_id, platform, token) Result
        +unlink(engine_id, platform) Result
        +list_linked(engine_id) Result
        +merge_entitlements(engine_id) Result
    }

    class LinkedAccount {
        +engine_account_id AccountId
        +platform PlatformKind
        +platform_user_id String
        +linked_at u64
    }

    PurchaseApi --> PlatformReceipt
    PurchaseApi --> ValidationResult
    PurchaseApi --> ReceiptValidator
    SubscriptionApi --> SubState
    SubscriptionApi --> RenewalAction
    AccountLinker --> LinkedAccount
```

## API Design

### OS Integration

#### Clipboard (F-14.2.1 / R-14.2.1)

```rust
/// RGBA image data for clipboard operations.
#[derive(Clone, Debug, Reflect)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

/// Platform clipboard access. All operations are
/// async. Uses cfg-gated platform backends.
#[derive(Reflect)]
pub struct Clipboard { /* platform fields */ }

impl Clipboard {
    pub fn new(window: &Window) -> Self;

    pub async fn read_text(
        &self,
    ) -> Result<Option<String>, OsError>;

    pub async fn write_text(
        &self,
        text: &str,
    ) -> Result<(), OsError>;

    pub async fn read_image(
        &self,
    ) -> Result<Option<ImageData>, OsError>;

    pub async fn write_image(
        &self,
        image: &ImageData,
    ) -> Result<(), OsError>;
}
```

#### File Dialogs (F-14.2.2 / R-14.2.2)

```rust
#[derive(Clone, Debug, Reflect)]
pub struct FileFilter {
    pub label: &'static str,
    pub extensions: &'static [&'static str],
}

#[derive(Clone, Debug, Reflect)]
pub struct FileDialogConfig {
    pub title: &'static str,
    pub initial_dir: Option<CanonicalPath>,
    pub filters: Vec<FileFilter>,
}

/// Dialogs run on a separate OS thread so the game
/// loop continues rendering (R-14.2.2).
#[derive(Reflect)]
pub struct FileDialog { /* platform fields */ }

impl FileDialog {
    pub fn new() -> Self;

    pub async fn open_file(
        &self,
        config: &FileDialogConfig,
    ) -> Result<Option<CanonicalPath>, OsError>;

    pub async fn save_file(
        &self,
        config: &FileDialogConfig,
    ) -> Result<Option<CanonicalPath>, OsError>;

    pub async fn pick_folder(
        &self,
        title: &str,
        initial_dir: Option<&CanonicalPath>,
    ) -> Result<Option<CanonicalPath>, OsError>;
}
```

#### Notifications (F-14.2.3 / R-14.2.3)

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum NotificationUrgency { Low, Normal, Critical }

#[derive(Clone, Debug, Reflect)]
pub struct NotificationConfig<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub urgency: NotificationUrgency,
    pub icon: Option<&'a str>,
}

#[derive(Clone, Debug, Reflect)]
pub struct TrayMenuItem {
    pub label: String,
    pub id: u32,
    pub enabled: bool,
}

#[derive(Reflect)]
pub struct Notifications { /* platform fields */ }

impl Notifications {
    pub fn new(window: &Window) -> Self;
    pub fn show_notification(
        &self, config: &NotificationConfig,
    ) -> Result<(), OsError>;
    pub fn create_tray_icon(
        &self, tooltip: &str,
        menu_items: &[TrayMenuItem],
    ) -> Result<TrayIconHandle, OsError>;
    pub fn remove_tray_icon(
        &self, handle: TrayIconHandle,
    ) -> Result<(), OsError>;
}
```

#### Drag and Drop (F-14.2.4 / R-14.2.4)

```rust
#[derive(Debug, Reflect)]
pub enum DragEvent {
    Enter {
        position: (f32, f32),
        mime_types: Vec<String>,
    },
    Over { position: (f32, f32) },
    Drop {
        position: (f32, f32),
        paths: Vec<CanonicalPath>,
        data: Option<Vec<u8>>,
    },
    Leave,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum DragResponse {
    Accept,
    Reject,
}

#[derive(Clone, Debug, Reflect)]
pub struct MimeFilter {
    pub mime_types: Vec<String>,
    pub extensions: Vec<String>,
}

#[derive(Reflect)]
pub struct DragDropHandler { /* platform fields */ }

impl DragDropHandler {
    pub fn register(
        window: &Window,
        filter: MimeFilter,
    ) -> Result<Self, OsError>;

    pub fn poll_event(&self) -> Option<DragEvent>;
    pub fn respond(&self, response: DragResponse);
    pub fn unregister(self) -> Result<(), OsError>;
}
```

#### Keyboard Layouts (F-14.2.5 / R-14.2.5)

```rust
#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub struct KeyboardLayout {
    pub name: String,
    pub id: KeyboardLayoutId,
}

#[derive(Debug, Reflect)]
pub enum DeadKeyResult {
    Char(char),
    Pending,
    Composed(char),
    Cancelled { dead_key: char, follow: char },
}

#[derive(Reflect)]
pub struct Keyboard { /* platform fields */ }

impl Keyboard {
    pub fn new() -> Self;
    pub fn active_layout(&self) -> KeyboardLayout;

    pub fn translate_key(
        &mut self,
        scancode: u32,
    ) -> DeadKeyResult;

    pub fn poll_layout_change(
        &mut self,
    ) -> Option<KeyboardLayout>;
}
```

#### IME (F-14.2.6 / R-14.2.6)

```rust
#[derive(Debug, Reflect)]
pub enum ImeEvent {
    Composition { text: String, cursor: usize },
    Commit { text: String },
    CandidateList {
        candidates: Vec<String>,
        selected: usize,
        page: usize,
        page_count: usize,
    },
    Cancel,
}

#[derive(Clone, Debug, Reflect)]
pub struct ImePosition {
    pub x: f32,
    pub y: f32,
    pub line_height: f32,
}

#[derive(Reflect)]
pub struct ImeHandler { /* platform fields */ }

impl ImeHandler {
    pub fn attach(
        window: &Window,
    ) -> Result<Self, OsError>;

    pub fn set_position(&self, pos: &ImePosition);
    pub fn set_enabled(&self, enabled: bool);
    pub fn poll_event(&self) -> Option<ImeEvent>;
    pub fn detach(self) -> Result<(), OsError>;
}
```

### Crash Reporting

#### Crash Handler (F-14.4.1 / R-14.4.1)

```rust
#[derive(Clone, Debug, Reflect)]
pub struct CrashHandlerConfig {
    pub crash_dir: CanonicalPath,
    pub oop_handler_path: CanonicalPath,
    pub max_retained_dumps: u32,
}

#[derive(Reflect)]
pub struct CrashHandler { /* platform fields */ }

impl CrashHandler {
    pub fn install(
        config: CrashHandlerConfig,
    ) -> Result<Self, CrashError>;

    pub fn set_metadata(&self, key: &str, value: &str);

    pub fn attach_gpu_breadcrumbs(
        &self,
        breadcrumbs: &GpuBreadcrumbs,
    );

    pub fn pending_dumps(
        &self,
    ) -> Vec<CanonicalPath>;

    pub async fn delete_dump(
        &self,
        path: &CanonicalPath,
    ) -> Result<(), CrashError>;
}
```

#### Symbol Upload (F-14.4.2 / R-14.4.2)

```rust
#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub enum SymbolFormat {
    Pdb { guid: String, age: u32 },
    Dsym { uuid: String },
    Dwarf { build_id: String },
}

#[derive(Reflect)]
pub struct SymbolUploader { /* ... */ }

impl SymbolUploader {
    pub fn new(endpoint: &str) -> Self;

    pub fn extract_build_id(
        binary_path: &CanonicalPath,
    ) -> Result<SymbolFormat, CrashError>;

    pub async fn upload_symbols(
        &self,
        binary_path: &CanonicalPath,
        symbol_path: &CanonicalPath,
    ) -> Result<(), CrashError>;
}
```

#### Structured Logging (F-14.4.4 / R-14.4.4)

```rust
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    PartialOrd, Ord, Reflect,
)]
pub enum Severity {
    Trace, Debug, Info, Warn, Error, Fatal,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct Channel(pub &'static str);

#[derive(Clone, Debug, Reflect)]
pub struct LogRecord<'a> {
    pub timestamp: u64,
    pub severity: Severity,
    pub channel: &'a Channel,
    pub message: &'a str,
    pub fields: &'a [(&'a str, &'a str)],
}

#[derive(Clone, Debug, Reflect)]
pub struct LogFilter {
    pub channel_levels: Vec<(Channel, Severity)>,
    pub default_level: Severity,
}

/// Log emission never blocks > 1 us (R-14.4.4).
/// Records go to a lock-free ring buffer.
#[derive(Reflect)]
pub struct Logger { /* ... */ }

impl Logger {
    pub fn new(
        filter: LogFilter,
        sinks: Vec<Box<dyn LogSink>>,
        ring_buffer_capacity: usize,
    ) -> Self;

    pub fn log(&self, record: &LogRecord);
    pub fn flush(&self);
    pub fn set_filter(&self, filter: LogFilter);
}
```

#### Performance Counters (F-14.4.5 / R-14.4.5)

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct CounterName(pub &'static str);

#[derive(Clone, Debug, Reflect)]
pub struct Snapshot {
    pub timestamp: u64,
    pub values: Vec<(CounterName, f64)>,
}

/// Lock-free per-thread counters.
/// Increment latency < 50 ns (R-14.4.5).
#[derive(Reflect)]
pub struct PerfCounters { /* ... */ }

impl PerfCounters {
    pub fn new() -> Self;
    pub fn increment(&self, name: &CounterName);
    pub fn increment_by(
        &self, name: &CounterName, amount: f64,
    );
    pub fn gauge(
        &self, name: &CounterName, value: f64,
    );
    pub fn flush(&self) -> Snapshot;
}
```

#### GPU Breadcrumbs (F-14.4.6 / R-14.4.6)

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub struct PassId(pub u32);

/// Writes incrementing markers into a GPU-visible
/// buffer per render pass. On device-lost, the last
/// marker identifies the faulting pass (R-14.4.6).
#[derive(Reflect)]
pub struct GpuBreadcrumbs { /* ... */ }

impl GpuBreadcrumbs {
    pub fn new(
        device: &GpuDevice,
    ) -> Result<Self, CrashError>;

    pub fn begin_pass(&self, pass_id: PassId);
    pub fn end_pass(&self, pass_id: PassId);

    pub fn read_last_completed(
        &self,
    ) -> Option<PassId>;

    pub fn serialize_for_crash_report(
        &self,
    ) -> Vec<u8>;
}
```

### Filesystem

#### Async File Operations (F-14.6.1 / R-14.6.1)

```rust
#[derive(Clone, Copy, Debug, Reflect)]
pub struct OpenFlags {
    pub read: bool,
    pub write: bool,
    pub create: bool,
    pub truncate: bool,
    pub append: bool,
}

impl OpenFlags {
    pub fn read_only() -> Self;
    pub fn write_only() -> Self;
    pub fn read_write() -> Self;
    pub fn create_new() -> Self;
}

/// No Rust stdlib file I/O (R-14.6.1). Backends:
/// Tokio (IOCP on Windows, kqueue on macOS,
/// epoll on Linux).
#[derive(Reflect)]
pub struct AsyncFile { /* platform fields */ }

impl AsyncFile {
    pub async fn open(
        path: &CanonicalPath, flags: OpenFlags,
    ) -> Result<Self, FsError>;

    pub async fn read(
        &self, buf: &mut [u8], offset: u64,
    ) -> Result<usize, FsError>;

    pub async fn read_to_end(
        &self,
    ) -> Result<Vec<u8>, FsError>;

    pub async fn write(
        &self, data: &[u8], offset: u64,
    ) -> Result<usize, FsError>;

    pub async fn flush(&self) -> Result<(), FsError>;
    pub async fn close(self) -> Result<(), FsError>;
}
```

#### File Create, Delete, Metadata (F-14.6.2--3)

```rust
pub async fn create_dir_all(
    path: &CanonicalPath,
) -> Result<(), FsError>;

pub async fn delete_file(
    path: &CanonicalPath,
) -> Result<(), FsError>;

pub async fn delete_batch(
    paths: &[CanonicalPath],
) -> Vec<Result<(), FsError>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum FileType { File, Directory, Symlink }

#[derive(Clone, Debug, Reflect)]
pub struct FileMetadata {
    pub file_type: FileType,
    pub size: u64,
    pub modified: u64,
    pub created: Option<u64>,
    pub read_only: bool,
}

pub async fn stat(
    path: &CanonicalPath,
) -> Result<FileMetadata, FsError>;

pub async fn stat_batch(
    paths: &[CanonicalPath],
) -> Vec<Result<FileMetadata, FsError>>;
```

#### Directory Enumeration (F-14.6.4 / R-14.6.4)

```rust
#[derive(Clone, Debug, Reflect)]
pub struct DirEntry {
    pub name: String,
    pub path: CanonicalPath,
    pub file_type: FileType,
    pub size: u64,
}

#[derive(Clone, Debug, Reflect)]
pub struct EnumerateOptions {
    pub max_depth: u32,
    pub glob: Option<String>,
}

pub async fn enumerate_dir(
    path: &CanonicalPath,
    options: &EnumerateOptions,
) -> Result<DirEntryStream, FsError>;
```

#### File Watcher (F-14.6.5 / R-14.6.5)

```rust
#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub enum FileEventKind {
    Created,
    Modified,
    Deleted,
    Renamed { from: CanonicalPath },
}

#[derive(Clone, Debug, Reflect)]
pub struct FileEvent {
    pub path: CanonicalPath,
    pub kind: FileEventKind,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect,
)]
pub struct WatchId(pub(crate) u32);

#[derive(Reflect)]
pub struct FileWatcher { /* ... */ }

impl FileWatcher {
    pub fn new(
        debounce_ms: u32,
    ) -> Result<Self, FsError>;

    pub fn watch(
        &mut self,
        path: &CanonicalPath,
        recursive: bool,
    ) -> Result<(WatchId, FileEventStream), FsError>;

    pub fn unwatch(
        &mut self, id: WatchId,
    ) -> Result<(), FsError>;
}
```

#### Content Hash (F-14.6.6 / R-14.6.6)

```rust
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect,
)]
pub struct Blake3Hash(pub [u8; 32]);

#[derive(Reflect)]
pub struct ContentHasher { /* ... */ }

impl ContentHasher {
    pub fn new() -> Self;

    pub async fn hash_file(
        &self, path: &CanonicalPath,
    ) -> Result<Blake3Hash, FsError>;

    pub async fn has_content_changed(
        &self,
        path: &CanonicalPath,
        old_hash: &Blake3Hash,
    ) -> Result<bool, FsError>;

    pub fn cache_hash(
        &mut self,
        path: CanonicalPath,
        hash: Blake3Hash,
    );
}
```

#### Canonical Path (F-14.6.7 / R-14.6.7)

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct CanonicalPath { /* ... */ }

impl CanonicalPath {
    pub fn resolve(
        path: &str,
    ) -> Result<Self, FsError>;
    pub fn as_str(&self) -> &str;
    pub fn file_name(&self) -> Option<&str>;
    pub fn parent(&self) -> Option<CanonicalPath>;
    pub fn join(
        &self, component: &str,
    ) -> Result<Self, FsError>;
    pub fn extension(&self) -> Option<&str>;
}
```

### Platform Services

#### Service Facade (F-14.5.1--7)

```rust
#[derive(Reflect)]
pub struct PlatformServices {
    pub achievements: AchievementService,
    pub leaderboards: LeaderboardService,
    pub presence: RichPresenceService,
    pub cloud: CloudStorageService,
    pub entitlements: EntitlementService,
    pub auth: AuthenticationService,
    pub profile: UserProfileService,
}

impl PlatformServices {
    pub async fn init(
        reactor: &Tokio runtime,
    ) -> Result<Self, PlatformError>;

    pub async fn shutdown(
        &mut self, reactor: &Tokio runtime,
    ) -> Result<(), PlatformError>;
}
```

#### Achievement Service (F-14.5.1 / R-14.5.1)

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct AchievementId(pub String);

#[derive(Clone, Debug, Reflect)]
pub struct AchievementDef {
    pub id: AchievementId,
    pub name: StringKey,
    pub description: StringKey,
    pub icon: AssetId,
    pub hidden: bool,
    pub platform_ids: PlatformIdMap,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum UnlockState { Locked, Unlocked, Pending }

#[derive(Clone, Debug, Reflect)]
pub struct AchievementProgress {
    pub id: AchievementId,
    pub current: u32,
    pub target: u32,
    pub state: UnlockState,
}

#[derive(Reflect)]
pub struct AchievementService {
    defs: Vec<AchievementDef>,
    progress: HashMap<
        AchievementId, AchievementProgress,
    >,
    deferred: DeferredQueue<AchievementId>,
}

impl AchievementService {
    pub async fn unlock(
        &mut self,
        id: &AchievementId,
        reactor: &Tokio runtime,
    ) -> Result<(), AchievementError>;

    pub async fn increment(
        &mut self,
        id: &AchievementId,
        amount: u32,
        reactor: &Tokio runtime,
    ) -> Result<(), AchievementError>;

    pub fn state(
        &self, id: &AchievementId,
    ) -> Option<&AchievementProgress>;

    pub async fn sync(
        &mut self, reactor: &Tokio runtime,
    ) -> Result<(), AchievementError>;

    pub async fn flush_deferred(
        &mut self, reactor: &Tokio runtime,
    ) -> Result<u32, AchievementError>;
}
```

#### Leaderboard Service (F-14.5.2 / R-14.5.2)

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct LeaderboardId(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum LeaderboardSort { Ascending, Descending }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum LeaderboardScope {
    Global, FriendsOnly, AroundPlayer,
}

#[derive(Clone, Debug, Reflect)]
pub struct LeaderboardRow {
    pub rank: u32,
    pub player_name: String,
    pub score: i64,
    pub player_id: Option<String>,
}

#[derive(Reflect)]
pub struct LeaderboardService {
    cache: HashMap<
        (LeaderboardId, LeaderboardScope),
        LeaderboardResult,
    >,
    cache_ttl_secs: u32,
    pending_submissions: Vec<(LeaderboardId, i64)>,
}

impl LeaderboardService {
    pub async fn submit(
        &mut self,
        id: &LeaderboardId,
        score: i64,
        reactor: &Tokio runtime,
    ) -> Result<(), LeaderboardError>;

    pub async fn query(
        &mut self,
        id: &LeaderboardId,
        scope: LeaderboardScope,
        offset: u32,
        count: u32,
        reactor: &Tokio runtime,
    ) -> Result<&LeaderboardResult, LeaderboardError>;

    pub async fn flush_pending(
        &mut self, reactor: &Tokio runtime,
    ) -> Result<u32, LeaderboardError>;
}
```

#### Rich Presence (F-14.5.3 / R-14.5.3)

```rust
#[derive(Clone, Debug, Reflect)]
pub struct PresenceState {
    pub activity: String,
    pub zone: Option<String>,
    pub party_size: Option<u32>,
    pub party_max: Option<u32>,
    pub details: Option<String>,
}

#[derive(Reflect)]
pub struct RichPresenceService {
    current: Option<PresenceState>,
    last_update: u64,
    throttle_interval_ms: u64,
}

impl RichPresenceService {
    pub async fn update(
        &mut self,
        state: PresenceState,
        reactor: &Tokio runtime,
    ) -> Result<(), PresenceError>;

    pub async fn clear(
        &mut self, reactor: &Tokio runtime,
    ) -> Result<(), PresenceError>;

    pub fn current(&self) -> Option<&PresenceState>;
}
```

#### Cloud Storage (F-14.5.5 / R-14.5.5)

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct CloudKey(pub String);

#[derive(Clone, Debug, Reflect)]
pub struct CloudMetadata {
    pub key: CloudKey,
    pub size_bytes: u64,
    pub timestamp: u64,
    pub checksum: u64,
}

#[derive(Clone, Debug, Reflect)]
pub enum ConflictResult {
    NoConflict(Vec<u8>),
    Conflict {
        local: Vec<u8>,
        local_timestamp: u64,
        cloud: Vec<u8>,
        cloud_timestamp: u64,
    },
}

#[derive(Reflect)]
pub struct CloudStorageService {
    quota_bytes: u64,
    used_bytes: u64,
}

impl CloudStorageService {
    pub async fn upload(
        &self, key: &CloudKey, data: &[u8],
        reactor: &Tokio runtime,
    ) -> Result<(), CloudError>;

    pub async fn download(
        &self, key: &CloudKey,
        reactor: &Tokio runtime,
    ) -> Result<Vec<u8>, CloudError>;

    pub async fn check_conflict(
        &self,
        key: &CloudKey,
        local_data: &[u8],
        local_timestamp: u64,
        reactor: &Tokio runtime,
    ) -> Result<ConflictResult, CloudError>;

    pub fn remaining_quota(&self) -> u64;
}
```

#### Entitlement Service (F-14.5.6 / R-14.5.6)

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum EntitlementKind {
    BaseGame, Expansion, CosmeticDlc, Subscription,
}

#[derive(Clone, Debug, Reflect)]
pub struct Entitlement {
    pub id: String,
    pub kind: EntitlementKind,
    pub owned: bool,
    pub expires: Option<u64>,
}

#[derive(Reflect)]
pub struct EntitlementService {
    entitlements: Vec<Entitlement>,
    last_check: u64,
    poll_interval_secs: u32,
}

impl EntitlementService {
    pub async fn refresh(
        &mut self, reactor: &Tokio runtime,
    ) -> Result<(), EntitlementError>;

    pub fn is_owned(&self, id: &str) -> bool;

    pub fn is_subscription_active(
        &self, id: &str,
    ) -> bool;
}
```

#### Deferred Queue

```rust
#[derive(Clone, Debug, Reflect)]
pub struct DeferredEntry<T> {
    pub item: T,
    pub enqueued_at: u64,
    pub retry_count: u32,
    pub next_retry_at: u64,
}

#[derive(Reflect)]
pub struct DeferredQueue<T> {
    pending: Vec<DeferredEntry<T>>,
    max_retries: u32,
    base_backoff_ms: u64,
}

impl<T: Clone> DeferredQueue<T> {
    pub fn new(
        max_retries: u32,
        base_backoff_ms: u64,
    ) -> Self;

    pub fn enqueue(&mut self, item: T);

    pub fn drain_ready(
        &mut self, now: u64,
    ) -> Vec<DeferredEntry<T>>;

    pub fn requeue(
        &mut self, entry: DeferredEntry<T>,
    ) -> bool;

    pub fn pending_count(&self) -> u32;
    pub fn is_empty(&self) -> bool;
}
```

### Storage

#### Preferences Store (F-14.5.8 / R-14.5.8)

```rust
#[derive(Clone, Debug, Reflect)]
pub enum PrefValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Clone, Debug, Reflect)]
pub struct PrefKey {
    pub key: &'static str,
    pub default: PrefValue,
}

#[derive(Reflect)]
pub struct PreferencesStore {
    values: HashMap<String, PrefValue>,
    dirty: bool,
    local_path: CanonicalPath,
    cloud_key: CloudKey,
}

impl PreferencesStore {
    pub async fn load(
        local_path: &CanonicalPath,
        cloud: &CloudStorageService,
        reactor: &Tokio runtime,
    ) -> Result<Self, PrefsError>;

    pub fn get(&self, key: &PrefKey) -> PrefValue;
    pub fn set(&mut self, key: &str, value: PrefValue);

    pub async fn save(
        &mut self,
        cloud: &CloudStorageService,
        reactor: &Tokio runtime,
    ) -> Result<(), PrefsError>;

    pub fn reset_to_defaults(
        &mut self, keys: &[PrefKey],
    );
    pub fn is_dirty(&self) -> bool;
}
```

#### Player Cache (F-14.5.9 / R-14.5.9)

```rust
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect,
)]
pub enum CacheCategory {
    AssetBundle,
    DlcContent,
    ModPackage,
    StreamingData,
    GenerationOutput,
}

#[derive(Clone, Debug, Reflect)]
pub struct CacheStats {
    pub total_bytes: u64,
    pub max_bytes: u64,
    pub per_category: HashMap<CacheCategory, u64>,
    pub entry_count: u32,
}

#[derive(Reflect)]
pub struct PlayerCache {
    root: CanonicalPath,
    entries: Vec<CacheEntry>,
    max_bytes: u64,
    total_bytes: u64,
}

impl PlayerCache {
    pub async fn put(
        &mut self, key: &str,
        category: CacheCategory, data: &[u8],
        reactor: &Tokio runtime,
    ) -> Result<(), CacheError>;

    pub async fn get(
        &mut self, key: &str,
        reactor: &Tokio runtime,
    ) -> Result<Option<Vec<u8>>, CacheError>;

    pub async fn evict_to_budget(
        &mut self, reactor: &Tokio runtime,
    ) -> Result<u32, CacheError>;

    pub fn stats(&self) -> CacheStats;
}
```

#### Developer Cache (F-14.5.10 / R-14.5.10)

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct ContentHash(pub [u8; 32]);

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect,
)]
pub enum DevCacheCategory {
    CompiledAsset,
    ShaderBytecode,
    LogicGraphBytecode,
    EditorThumbnail,
    HotReloadIntermediate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum CacheHitTier { Local, SharedNetwork, Miss }

#[derive(Reflect)]
pub struct DeveloperCache {
    root: CanonicalPath,
    shared_url: Option<String>,
}

impl DeveloperCache {
    pub async fn lookup(
        &self, hash: &ContentHash,
        category: DevCacheCategory,
        reactor: &Tokio runtime,
    ) -> Result<
        (CacheHitTier, Option<Vec<u8>>), CacheError,
    >;

    pub async fn store(
        &self, hash: &ContentHash,
        category: DevCacheCategory, data: &[u8],
        reactor: &Tokio runtime,
    ) -> Result<(), CacheError>;

    pub fn hash(data: &[u8]) -> ContentHash;
}
```

#### PSO Cache (F-14.5.11 / R-14.5.11)

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct GpuDriverKey {
    pub gpu_vendor_id: u32,
    pub gpu_device_id: u32,
    pub driver_version: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct PsoKey {
    pub shader_hash: ContentHash,
    pub render_state_hash: u64,
    pub vertex_layout_hash: u64,
    pub render_target_hash: u64,
}

#[derive(Reflect)]
pub struct PsoCacheStore {
    cache_dir: CanonicalPath,
    gpu_driver: GpuDriverKey,
    entries: HashMap<PsoKey, CanonicalPath>,
}

impl PsoCacheStore {
    pub async fn load_all(
        &mut self, reactor: &Tokio runtime,
    ) -> Result<u32, CacheError>;

    pub async fn store(
        &mut self, key: PsoKey, data: &[u8],
        reactor: &Tokio runtime,
    ) -> Result<(), CacheError>;

    /// Must complete in < 1 ms.
    pub async fn get(
        &self, key: &PsoKey,
        reactor: &Tokio runtime,
    ) -> Result<Option<Vec<u8>>, CacheError>;

    pub async fn invalidate_all(
        &mut self, reactor: &Tokio runtime,
    ) -> Result<u32, CacheError>;
}
```

#### Temp File Manager (F-14.5.12 / R-14.5.12)

```rust
/// RAII handle. File deleted on drop.
#[derive(Reflect)]
pub struct TempFileHandle {
    path: CanonicalPath,
}

#[derive(Reflect)]
pub struct TempFileManager {
    root: CanonicalPath,
    max_bytes: u64,
    total_bytes: u64,
}

impl TempFileManager {
    pub async fn init(
        root: CanonicalPath, max_bytes: u64,
        reactor: &Tokio runtime,
    ) -> Result<Self, TempError>;

    pub fn allocate(
        &mut self, name: &str,
    ) -> Result<TempFileHandle, TempError>;

    pub async fn cleanup_orphans(
        &mut self, reactor: &Tokio runtime,
    ) -> Result<u32, TempError>;
}
```

#### Platform Paths

```rust
#[derive(Reflect)]
pub struct PlatformPaths;

impl PlatformPaths {
    /// Windows: %LOCALAPPDATA%\Harmonius\{game}\
    /// macOS: ~/Library/Application Support/{game}/
    /// Linux: $XDG_DATA_HOME/{game}/
    pub fn preferences(game: &str) -> CanonicalPath;

    /// Windows: %LOCALAPPDATA%\...\Cache\
    /// macOS: ~/Library/Caches/{game}/
    /// Linux: $XDG_CACHE_HOME/{game}/
    pub fn player_cache(game: &str) -> CanonicalPath;

    /// Always: {project_root}/.harmonius/cache/
    pub fn developer_cache(
        root: &CanonicalPath,
    ) -> CanonicalPath;

    /// Windows: %TEMP%\Harmonius\{game}\
    /// macOS/Linux: /tmp/harmonius-{game}/
    pub fn temp(game: &str) -> CanonicalPath;
}
```

### SDK Integration

#### Purchase and Receipt Validation (F-14.8 / R-14.8)

```rust
#[derive(Clone, Debug, Reflect)]
pub struct PlatformReceipt {
    pub transaction_id: TransactionId,
    pub product_id: ProductId,
    pub platform: PlatformKind,
    pub receipt_data: Vec<u8>,
    pub timestamp: u64,
    pub signature: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Reflect)]
pub struct ValidationResult {
    pub valid: bool,
    pub transaction_id: TransactionId,
    pub product_id: ProductId,
    pub is_duplicate: bool,
    pub entitlement_granted: bool,
}

#[derive(Reflect)]
pub struct ReceiptValidator;

impl ReceiptValidator {
    pub async fn validate(
        &self, receipt: &PlatformReceipt,
    ) -> Result<ValidationResult, ValidationError>;

    pub async fn validate_with_retry(
        &self, receipt: &PlatformReceipt,
        max_retries: u32,
    ) -> Result<ValidationResult, ValidationError>;
}
```

#### Subscription Management

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum SubState {
    Active,
    GracePeriod,
    BillingRetry,
    Expired,
    Cancelled,
    Revoked,
    Paused,
}

#[derive(Clone, Debug, Reflect)]
pub struct SubStatus {
    pub active: bool,
    pub product_id: ProductId,
    pub state: SubState,
    pub renewal_date: Option<u64>,
    pub expiry_date: Option<u64>,
    pub grace_period_end: Option<u64>,
    pub last_verified_at: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum RenewalAction {
    EnableAutoRenew,
    DisableAutoRenew,
    UpgradeTier { new_product: ProductId },
    DowngradeTier { new_product: ProductId },
}
```

#### Cross-Platform Account Linking

```rust
#[derive(Clone, Debug, Reflect)]
pub struct LinkedAccount {
    pub engine_account_id: AccountId,
    pub platform: PlatformKind,
    pub platform_user_id: String,
    pub linked_at: u64,
}

#[derive(Clone, Debug, Reflect)]
pub struct EntitlementSet {
    pub purchases: Vec<ProductId>,
    pub subscriptions: Vec<SubStatus>,
    pub achievements: Vec<AchievementId>,
}

#[derive(Reflect)]
pub struct AccountLinker;

impl AccountLinker {
    pub async fn link(
        &self, engine_id: AccountId,
        platform: PlatformKind,
        platform_token: &[u8],
    ) -> Result<LinkedAccount, LinkError>;

    pub async fn unlink(
        &self, engine_id: AccountId,
        platform: PlatformKind,
    ) -> Result<(), LinkError>;

    pub async fn merge_entitlements(
        &self, engine_id: AccountId,
    ) -> Result<EntitlementSet, LinkError>;
}
```

### Error Types

```rust
#[derive(Debug, Reflect)]
pub enum OsError {
    Unsupported,
    Cancelled,
    Platform { code: i32, message: String },
    FormatMismatch,
    MimeRejected { mime_type: String },
}

#[derive(Debug, Reflect)]
pub enum CrashError {
    HandlerNotFound { path: String },
    HandlerLaunchFailed { code: i32 },
    UploadFailed { status: u16, message: String },
    BuildIdNotFound,
    GpuBufferAllocationFailed,
    Platform { code: i32, message: String },
}

#[derive(Debug, Reflect)]
pub enum FsError {
    NotFound { path: String },
    PermissionDenied { path: String },
    AlreadyExists { path: String },
    DirectoryNotEmpty { path: String },
    Cancelled,
    DeviceFull,
    SymlinkLoop { path: String },
    PathTooLong { path: String },
    Platform { code: i32, message: String },
}

#[derive(Debug, Reflect)]
pub enum PlatformError {
    NotInitialized,
    SdkError { platform: PlatformKind, code: i32 },
    NetworkUnavailable,
    AuthenticationFailed,
    Timeout,
}

#[derive(Debug, Reflect)]
pub enum AchievementError {
    NotFound { id: AchievementId },
    AlreadyUnlocked { id: AchievementId },
    Platform(PlatformError),
}

#[derive(Debug, Reflect)]
pub enum LeaderboardError {
    NotFound { id: LeaderboardId },
    RateLimited,
    Platform(PlatformError),
}

#[derive(Debug, Reflect)]
pub enum CloudError {
    QuotaExceeded { used: u64, max: u64 },
    KeyNotFound { key: CloudKey },
    Platform(PlatformError),
}

#[derive(Debug, Reflect)]
pub enum ValidationError {
    NetworkError,
    InvalidReceipt,
    ExpiredReceipt,
    PlatformError { code: i32 },
    Duplicate { original_txn: TransactionId },
}

#[derive(Debug, Reflect)]
pub enum CacheError {
    IoError(FsError),
    BudgetExceeded { used: u64, max: u64 },
    NetworkCacheUnavailable,
}

#[derive(Debug, Reflect)]
pub enum PrefsError {
    IoError(FsError),
    ParseError { line: u32, message: String },
    CloudError(CloudError),
}

#[derive(Debug, Reflect)]
pub enum TempError {
    IoError(FsError),
    BudgetExceeded { used: u64, max: u64 },
}
```

## Data Flow

### Async File Read

1. Caller calls `AsyncFile::read(buf, offset).await`
2. Submits to `Tokio runtime`; acquires `BufferSlot`, enqueues to platform backend
3. Future yields; kernel reads asynchronously
4. Poller thread wakes Future on completion
5. Worker resumes, returns `Ok(bytes_read)`

### Crash Handler

1. Process faults; OS delivers signal/exception
2. In-process handler notifies OOP monitor via pipe
3. OOP captures stack/registers, writes minidump + GPU breadcrumbs to crash directory
4. Next launch: scan, upload, symbolicate, cluster

### Achievement Unlock

```mermaid
sequenceDiagram
    participant G as Gameplay System
    participant ACH as AchievementService
    participant Q as DeferredQueue
    participant BE as Platform Backend
    participant IO as Tokio runtime

    G->>ACH: unlock(achievement_id)
    ACH->>ACH: map internal ID to platform ID
    ACH->>BE: submit_unlock(platform_id).await
    alt online
        BE-->>ACH: Ok
        ACH->>ACH: mark unlocked locally
    else offline
        BE-->>ACH: Err(Unavailable)
        ACH->>Q: enqueue(platform_id, timestamp)
    end

    Note over IO: next reactor poll
    IO->>Q: drain_pending()
    Q->>BE: retry_unlock(platform_id).await
    alt success
        BE-->>Q: Ok
        Q->>ACH: confirm(platform_id)
    else still unavailable
        BE-->>Q: Err
        Q->>Q: re-enqueue with backoff
    end
```

### Receipt Validation

```mermaid
sequenceDiagram
    participant P as Player
    participant CL as Client
    participant SDK as Platform Store SDK
    participant SRV as Game Server
    participant VAL as Receipt Validator
    participant PAPI as Platform API

    P->>CL: tap purchase button
    CL->>SDK: initiate_purchase(product_id)
    SDK-->>CL: PlatformReceipt

    CL->>SRV: submit_receipt(PlatformReceipt)
    SRV->>VAL: validate(receipt)
    VAL->>PAPI: verify receipt
    PAPI-->>VAL: validation response

    alt valid
        VAL-->>SRV: ValidationResult(valid=true)
        SRV->>SRV: grant entitlement
        SRV-->>CL: purchase_confirmed
    else invalid
        VAL-->>SRV: ValidationResult(valid=false)
        SRV-->>CL: purchase_failed(reason)
    end
```

### Startup Sequence

1. `PlatformServices::init()` authenticates via `AuthenticationService`
2. `AchievementService::sync()` fetches unlock state
3. `EntitlementService::refresh()` queries entitlements
4. `PreferencesStore::load()` reads local then cloud; shows conflict dialog if diverged
5. `PsoCacheStore::load_all()` pre-loads cached PSOs for current GPU/driver
6. `TempFileManager::init()` cleans up orphaned files
7. `AchievementService::flush_deferred()` retries queued unlocks from previous session

### Subscription State Machine

```mermaid
stateDiagram-v2
    [*] --> Active : purchase confirmed
    Active --> Active : auto-renew success
    Active --> GracePeriod : payment failed
    Active --> Cancelled : user cancels
    Active --> Paused : user pauses

    GracePeriod --> Active : payment recovered
    GracePeriod --> BillingRetry : grace expires
    BillingRetry --> Active : payment recovered
    BillingRetry --> Expired : max retries

    Cancelled --> Expired : billing period ends
    Cancelled --> Active : resubscribe

    Paused --> Active : user resumes
    Paused --> Expired : pause expires

    Expired --> Active : resubscribe
    Expired --> [*]
```

## Platform Considerations

### OS Integration APIs

| Feature   | Windows             | macOS              | Linux              |
|-----------|---------------------|--------------------|--------------------|
| Clipboard | `OpenClipboard`     | `NSPasteboard`     | X11/Wayland        |
| File dlg  | `IFileOpenDialog`   | `NSOpenPanel`      | portal D-Bus       |
| Notify    | `Shell_NotifyIcon`  | `UNNotificationCtr`| D-Bus Notifications|
| Drag-drop | `IDropTarget`       | `NSDraggingDest`   | XDND/Wayland DnD   |
| Keyboard  | `GetKeyboardLayout` | `TISCopy...Source` | `xkbcommon`        |
| IME       | TSF `ITfThreadMgr`  | `NSTextInputClient`| IBus/Fcitx D-Bus   |

### Crash Reporting APIs

| Feature    | Windows             | macOS              | Linux              |
|------------|---------------------|--------------------|--------------------|
| Handler    | SEH exception       | Mach exceptions    | SIGSEGV/SIGABRT    |
| Dump       | `MiniDumpWriteDump` | Core dump/custom   | ptrace core dump   |
| OOP comm   | Named pipe          | Mach ports         | Unix socket        |
| Build ID   | PE GUID + age       | `LC_UUID`          | `.note.gnu.build-id`|
| Symbols    | PDB files           | dSYM bundles       | DWARF              |
| Debug log  | `OutputDebugString` | `os_log`           | `sd_journal_sendv` |
| Trace      | ETW `EventWrite`    | `os_signpost`      | `perf_event_open`  |
| GPU crumbs | D3D12 DRED          | Shared `MTLBuffer` | Vulkan buf_marker  |

### Filesystem APIs

| Feature   | Windows             | macOS              | Linux              |
|-----------|---------------------|--------------------|--------------------|
| File open | `CreateFileW` Tokio | `tokio::fs::open`  | `tokio::fs::open`  |
| File I/O  | Tokio (IOCP)        | Tokio (kqueue)     | Tokio (epoll)      |
| Create/del| `CreateDirectoryW`  | `tokio::fs`        | `tokio::fs`        |
| Stat      | `GetFileInfoByHdlEx`| `tokio::fs`        | `tokio::fs`        |
| Dir enum  | `FindFirstFileExW`  | `tokio::fs`        | `tokio::fs`        |
| Watching  | `ReadDirChangesExW` | FSEvents/VNODE     | inotify + uring    |
| Path      | `GetFinalPathByHdlW`| `fcntl(F_GETPATH)` | `realpath`         |

### Platform SDK APIs

| Service      | Steam              | Xbox             | PSN          |
|--------------|--------------------|------------------|--------------|
| Achievements | `ISteamUserStats`  | `XblAchievement` | `NpTrophy`   |
| Leaderboards | `ISteamUserStats`  | `XblLeaderboard` | `NpScore`    |
| Presence     | `ISteamFriends`    | `XblPresence`    | `NpPresence` |
| Cloud Save   | `ISteamRemoteStor` | `XGameSave`      | `NpSaveData` |
| Entitlements | `ISteamApps`       | `XStore`         | `NpCommerce` |
| Purchases    | `ISteamMicroTxn`   | `XStore`         | `NpCommerce` |
| Matchmaking  | `ISteamMatchmaking`| SmartMatch       | `NpMatching2`|
| Anti-Cheat   | VAC                | TruePlay         | Custom       |

Also: Apple (StoreKit 2, GameCenter), Google (Play Billing 7), Nintendo (NEX), EOS (fallback).

### Rate Limits and Throttling

| Service       | Platform    | Limit         | Mitigation         |
|---------------|-------------|---------------|--------------------|
| Rich Presence | Steam       | ~15 s         | Throttle, latest   |
| Rich Presence | PlayStation | ~30 s         | Throttle, latest   |
| Leaderboards  | Xbox        | 30 req/min    | Cache, batch       |
| Leaderboards  | PlayStation | 10 req/min    | Cache, batch       |
| Cloud Storage | All         | Varies        | Debounce, batch    |

### Offline Graceful Degradation

| Service      | Offline Behavior                  |
|--------------|-----------------------------------|
| Achievements | Deferred queue; sync on reconnect |
| Leaderboards | Cache last-known; submit later    |
| Purchases    | Block (requires platform dialog)  |
| Subs         | Last-known status + grace window  |
| Cloud Save   | Local only; sync on reconnect     |
| Matchmaking  | LAN discovery fallback            |
| Voice Chat   | Disabled                          |
| Anti-Cheat   | Custom checks; skip platform      |
| Friends      | Cache last-known list             |
| Mods         | Use locally cached mods           |

### Console Certification Checklist

| Requirement          | PlayStation     | Xbox            |
|----------------------|-----------------|-----------------|
| Suspend/resume       | `LoadExec`      | `CoreApp` cycle |
| System UI overlay    | Must not block  | Must not block  |
| Controller disconnect| Prompt required | Prompt required |
| Safe-area rendering  | 90% inner zone  | Title-safe area |
| Trophy/achievement   | Mandatory       | Mandatory       |
| Memory pressure      | Release assets  | Release on time |

### Server-Side Proprietary Architecture

The engine is 100% open source. Console compilation, signing, and packaging runs on a shared build
server with console SDK licenses. Key decisions:

1. Zero proprietary code on client
2. One license per server, not per developer
3. Abstract trait boundary for console implementations
4. Content-hash artifact caching
5. Per-project isolation with access control

Build flow: Editor POSTs to build server REST API; worker compiles with console SDK, code-signs,
uploads to S3; editor downloads artifact and optionally deploys to dev kit via the same API with
WebSocket output relay.

### FFI Bridge Pattern

Apple uses swift-bridge for StoreKit 2 and GameCenter. Windows/Xbox uses `windows-rs` with COM
bindings. See [constraints.md](../constraints.md).

### Proposed Dependencies

| Crate          | Purpose                        |
|----------------|--------------------------------|
| `blake3`       | Content hashing (R-14.6.6)     |
| `windows-rs`   | Win32/COM/WinRT APIs           |
| `libc`         | POSIX/macOS/Linux syscalls     |
| `io-uring`     | Linux async file ops           |
| `xkbcommon`    | Linux keyboard layouts         |
| `steamworks`   | Steamworks SDK Rust bindings   |
| `swift-bridge` | Apple platform SDK bindings    |
| `toml`         | Preferences serialization      |

## Test Plan

Test cases are in the companion file
[platform-services-test-cases.md](platform-services-test-cases.md).

### Summary

| Category                  | Count | Coverage        |
|---------------------------|-------|-----------------|
| OS integration tests      | 16    | R-14.2.1--6     |
| Crash reporting tests     | 15    | R-14.4.1--6     |
| Filesystem tests          | 23    | R-14.6.1--7     |
| Achievement tests         | 5     | R-14.5.1        |
| Leaderboard tests         | 3     | R-14.5.2        |
| Rich presence tests       | 3     | R-14.5.3        |
| Cloud storage tests       | 4     | R-14.5.5        |
| Entitlement tests         | 2     | R-14.5.6        |
| Preferences tests         | 5     | R-14.5.8        |
| Player cache tests        | 4     | R-14.5.9        |
| Developer cache tests     | 4     | R-14.5.10       |
| PSO cache tests           | 3     | R-14.5.11       |
| Temp file tests           | 3     | R-14.5.12       |
| Platform path tests       | 3     | R-14.5.8        |
| IAP/receipt tests         | 10    | R-14.8.1--2     |
| Subscription tests        | 10    | R-14.5.6        |
| Matchmaking tests         | 8     | R-14.8.5        |
| Anti-cheat tests          | 8     | R-14.8.4        |
| Certification tests       | 8     | R-14.5.7        |
| Cross-platform tests      | 6     | R-14.8.6        |
| SDK isolation tests       | 10    | R-14.8.1--10    |
| **Total**                 | **153** |               |

### Benchmarks

| Benchmark                    | Target      | Source     |
|------------------------------|-------------|------------|
| Async read throughput        | >= 80% disk | US-14.6.11 |
| Async write throughput       | >= 80% disk | US-14.6.11 |
| Log emission latency         | < 1 us      | R-14.4.4   |
| Counter increment latency    | < 50 ns     | R-14.4.5   |
| Clipboard round-trip         | < 1 frame   | R-14.2.1   |
| File watcher event latency   | < 100 ms    | R-14.6.5   |
| Content hash throughput      | >= 2 GB/s   | R-14.6.6   |
| Achievement unlock (online)  | < 50 ms     | US-14.5.1  |
| Leaderboard query (cached)   | < 1 ms      | US-14.5.2  |
| Preferences save (atomic)    | < 50 ms     | R-14.5.8   |
| PSO cache get                | < 1 ms      | R-14.5.11  |
| BLAKE3 hash 1 MB             | < 500 us    | US-14.5.22 |
| Player cache eviction (100)  | < 100 ms    | US-14.5.21 |
| Temp file allocate + drop    | < 10 ms     | US-14.5.28 |
| Dev cache 3-tier lookup      | < 50 ms     | US-14.5.23 |

## Design Q and A

**Q1. Biggest constraint?** No-stdlib-file-I/O (R-14.6.1) forces all filesystem ops through async
backends. Trade-off: uniform API eliminates platform-conditional calling.

**Q2. Improvements?** Unified `SubscriptionEvent` enum for webhook handling. Vector clock for
preference conflicts instead of timestamps. Persist deferred queue to disk.

**Q3. Better approach?** Plugin architecture with dynamic backends conflicts with static dispatch.
`cfg`-gated has zero runtime overhead and matches SDK distribution model.

**Q4. Gaps?** Intra-app DnD for editor. Background PSO pre-compilation. Preferences schema
migration. Unified platform event normalization for overlay/guide events.

**Q5. Cohesion?** All I/O uses `Tokio runtime`. All FFI via C ABI bridges. BLAKE3 shared across file
watching and dev cache. PSO cache integrates with rendering pipeline.

## Open Questions

1. **OOP crash handler** -- separate binary or self-fork?
2. **Deferred queue persistence** -- persist to disk?
3. **File watcher** -- `notify` crate or custom?
4. **Console SDK access** -- feature-flagged crates or inline `cfg` modules?
5. **Cross-platform currency** -- per-platform or server-side wallet?
6. **PSO cache distribution** -- all GPU vendors or reference hardware only?
7. **Save data quotas** -- common denominator or per-platform limits?
8. **EOS as fallback** -- use for all platforms lacking native equivalents?
