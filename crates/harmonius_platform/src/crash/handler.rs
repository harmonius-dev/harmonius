//! In-process crash handler metadata (`R-14.4.1`).

use std::collections::HashMap;
use std::sync::Mutex;

use crate::filesystem::CanonicalPath;

/// Configuration for installing a crash handler.
#[derive(Clone, Debug)]
pub struct CrashHandlerConfig {
    /// Directory receiving crash artifacts.
    pub crash_dir: CanonicalPath,
    /// Path to the out-of-process minidump helper binary.
    pub oop_handler: CanonicalPath,
}

/// Captures metadata appended to crash bundles.
pub struct CrashHandler {
    meta: Mutex<HashMap<String, String>>,
    #[allow(dead_code)]
    config: CrashHandlerConfig,
}

impl CrashHandler {
    /// Installs handler state (stub: no OS signals hooked in unit tests).
    pub fn install(config: CrashHandlerConfig) -> Result<Self, super::CrashError> {
        Ok(Self {
            meta: Mutex::new(HashMap::new()),
            config,
        })
    }

    /// Stores a key/value pair shipped with the next crash report.
    pub fn set_metadata(&self, key: &str, value: &str) {
        let mut g = self.meta.lock().expect("crash meta mutex poisoned");
        g.insert(key.into(), value.into());
    }

    /// Returns the captured metadata map.
    pub fn metadata_snapshot(&self) -> HashMap<String, String> {
        self.meta.lock().expect("crash meta mutex poisoned").clone()
    }
}
