//! Console SDK trait surface (public repo, NDA-free).

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod nda;

pub use nda::{forbidden_substrings, scan_source_for_console_nda_markers, ForbiddenMarkerKind};

use std::fmt;

/// Raw save payload returned by the stub or platform storage layer.
pub type Bytes = Vec<u8>;

/// Configuration passed to [`ConsoleSdk::init`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConsoleConfig;

/// Stable identifier for a save slot.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SlotId(pub u32);

/// Trophy or achievement identifier (opaque in the public repo).
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrophyId(pub u32);

/// Network session identifier (opaque in the public repo).
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionId(pub u64);

/// Minimal session creation parameters for the public trait surface.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionDesc {
    /// Human-readable label for debugging (not sent to platform services in the stub).
    pub label: String,
}

/// Presence string wrapper so engine code never passes raw unvalidated strings.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PresenceString(pub String);

/// Console-local user record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsoleUser {
    /// Stable user id for tests and stub behavior.
    pub id: ConsoleUserId,
    /// Display name placeholder.
    pub display_name: String,
    /// Age-restricted flag for cert flows.
    pub age_restricted: bool,
}

/// Opaque user id for the public API.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConsoleUserId(pub u64);

/// Save slot metadata for UI and validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveSlot {
    /// Slot id.
    pub id: SlotId,
    /// Declared maximum size in bytes.
    pub size: u64,
    /// Last modified timestamp (stub: monotonic counter).
    pub last_modified: u64,
}

/// Errors from [`ConsoleSdk::init`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConsoleInitError {
    /// Initialization failed for a platform-specific reason.
    InitFailed(&'static str),
}

impl fmt::Display for ConsoleInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsoleInitError::InitFailed(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ConsoleInitError {}

/// Sign-in failures for [`AccountHandle::sign_in`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SignInError {
    /// User declined or sign-in unavailable.
    Cancelled,
}

impl fmt::Display for SignInError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignInError::Cancelled => write!(f, "sign-in cancelled"),
        }
    }
}

impl std::error::Error for SignInError {}

/// Storage errors for [`StorageHandle`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StorageError {
    /// Slot not found.
    MissingSlot,
    /// I/O or platform error (stub uses this for invalid args only).
    IoFailed(&'static str),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::MissingSlot => write!(f, "missing save slot"),
            StorageError::IoFailed(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for StorageError {}

/// Trophy API errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrophyError {
    /// Unknown trophy id for the stub implementation.
    UnknownTrophy,
}

impl fmt::Display for TrophyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrophyError::UnknownTrophy => write!(f, "unknown trophy id"),
        }
    }
}

impl std::error::Error for TrophyError {}

/// Online session errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OnlineError {
    /// Online blocked by parental rating lock.
    RatingLocked,
    /// Generic failure for stub-only paths.
    Failed(&'static str),
}

impl fmt::Display for OnlineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OnlineError::RatingLocked => write!(f, "online blocked by rating lock"),
            OnlineError::Failed(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for OnlineError {}

/// Abstract console SDK (implemented by stub and private fork crates).
pub trait ConsoleSdk: Send + Sync {
    /// Account subsystem handle.
    type Account: AccountHandle;
    /// Save storage handle.
    type Storage: StorageHandle;
    /// Trophies / achievements handle.
    type Trophies: TrophyHandle;
    /// Online multiplayer handle.
    type Online: OnlineHandle;
    /// Certification-required hooks.
    type Cert: CertHandle;

    /// Initializes the active backend.
    fn init(config: ConsoleConfig) -> Result<Self, ConsoleInitError>
    where
        Self: Sized;

    /// Shuts down the backend.
    fn shutdown(self);

    /// Account handle.
    fn account(&self) -> &Self::Account;

    /// Storage handle.
    fn storage(&self) -> &Self::Storage;

    /// Trophies handle.
    fn trophies(&self) -> &Self::Trophies;

    /// Online handle.
    fn online(&self) -> &Self::Online;

    /// Certification hooks handle.
    fn cert(&self) -> &Self::Cert;
}

/// Account operations required by certification (sign-in gating, user switches).
pub trait AccountHandle: Send + Sync {
    /// Returns the signed-in user, if any.
    fn current_user(&self) -> Option<ConsoleUser>;

    /// Signs in (stub returns a deterministic fake user when `required` is false).
    fn sign_in(&self, required: bool) -> Result<ConsoleUser, SignInError>;

    /// Registers a callback when the active user changes.
    fn on_switch(&self, cb: Box<dyn Fn(ConsoleUser) + Send + Sync>);
}

/// Save data IO surface.
pub trait StorageHandle: Send + Sync {
    /// Lists known save slots.
    fn save_slots(&self) -> Vec<SaveSlot>;

    /// Writes bytes to a slot.
    fn write_slot(&self, slot: SlotId, bytes: &[u8]) -> Result<(), StorageError>;

    /// Reads bytes from a slot.
    fn read_slot(&self, slot: SlotId) -> Result<Bytes, StorageError>;

    /// Requests a backup for a slot (stub is a no-op Ok).
    fn backup(&self, slot: SlotId) -> Result<(), StorageError>;
}

/// Trophies / achievements surface.
pub trait TrophyHandle: Send + Sync {
    /// Unlocks a trophy / achievement.
    fn unlock(&self, id: TrophyId) -> Result<(), TrophyError>;

    /// Sets incremental progress for a trophy / achievement.
    fn progress(&self, id: TrophyId, value: u32) -> Result<(), TrophyError>;
}

/// Online session and presence surface.
pub trait OnlineHandle: Send + Sync {
    /// Creates a new online session.
    fn session_create(&self, desc: SessionDesc) -> Result<SessionId, OnlineError>;

    /// Joins an existing session.
    fn session_join(&self, id: SessionId) -> Result<(), OnlineError>;

    /// Updates presence string (validated in private forks).
    fn presence(&self, set: PresenceString);
}

/// Certification hooks (suspend/resume, controller disconnect, rating lock).
pub trait CertHandle: Send + Sync {
    /// Enables or disables parental rating lock enforcement for online features.
    fn enforce_rating_lock(&self, enabled: bool);

    /// Suspend notification (must be safe to call repeatedly).
    fn handle_suspend(&self);

    /// Resume notification (must be safe to call repeatedly).
    fn handle_resume(&self);

    /// Controller disconnect notification.
    fn handle_controller_disconnect(&self);
}
