//! Storage helpers: preferences, caches, temp files (`R-14.5.8`–`R-14.5.12`).

mod developer_cache;
mod error;
mod player_cache;
mod prefs;
mod pso_cache;
mod temp_file;

pub use developer_cache::{
    CacheHitTier, ContentHash, DevCacheCategory, DeveloperCache,
};
pub use error::PrefsError;
pub use player_cache::{CacheCategory, CacheStats, PlayerCache};
pub use prefs::{NoCloud, PlatformPaths, PrefKey, PrefValue, PreferencesStore};
pub use pso_cache::{GpuDriverKey, PsoCacheStore};
pub use temp_file::{TempError, TempFileHandle, TempFileManager};
