//! Achievement unlock staging (`R-14.5.1`).

use std::collections::HashMap;
use std::sync::Mutex;

use super::PlatformError;

/// Achievement identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AchievementId(pub String);

/// Progress snapshot for UI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AchievementProgress {
    /// Achievement id.
    pub id: AchievementId,
    /// Current counter value.
    pub current: u32,
    /// Target counter value.
    pub target: u32,
    /// High-level unlock state.
    pub state: UnlockState,
}

/// Unlock lifecycle state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnlockState {
    /// Not yet completed.
    Locked,
    /// Submitted to platform, awaiting ack.
    Pending,
    /// Confirmed unlocked.
    Unlocked,
}

/// Achievement-specific failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AchievementError {
    /// Unknown achievement id.
    NotFound {
        /// Missing id.
        id: AchievementId,
    },
    /// Duplicate unlock request.
    AlreadyUnlocked {
        /// Conflicting id.
        id: AchievementId,
    },
    /// Wrapped platform error.
    Platform(PlatformError),
}

#[derive(Debug)]
struct Entry {
    target: u32,
    current: u32,
    state: UnlockState,
}

/// Tracks local achievement progress and deferred unlocks.
#[derive(Debug)]
pub struct AchievementService {
    entries: Mutex<HashMap<String, Entry>>,
    deferred: Mutex<Vec<AchievementId>>,
    online: Mutex<bool>,
}

impl AchievementService {
    /// Creates an empty service (starts online).
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
            deferred: Mutex::new(Vec::new()),
            online: Mutex::new(true),
        }
    }

    /// Registers an achievement with a progress target.
    pub fn register(&self, id: &AchievementId, target: u32) {
        let mut g = self.entries.lock().expect("ach mutex poisoned");
        g.entry(id.0.clone()).or_insert(Entry {
            target,
            current: 0,
            state: UnlockState::Locked,
        });
    }

    /// Requests an unlock (may defer when offline).
    pub fn unlock(&self, id: &AchievementId) -> Result<(), AchievementError> {
        let mut g = self.entries.lock().expect("ach mutex poisoned");
        let e = g.get_mut(&id.0).ok_or_else(|| AchievementError::NotFound {
            id: id.clone(),
        })?;
        match e.state {
            UnlockState::Unlocked => Err(AchievementError::AlreadyUnlocked { id: id.clone() }),
            UnlockState::Pending => Ok(()),
            UnlockState::Locked => {
                e.state = UnlockState::Pending;
                if !*self.online.lock().expect("ach mutex poisoned") {
                    self.deferred
                        .lock()
                        .expect("ach mutex poisoned")
                        .push(id.clone());
                }
                Ok(())
            }
        }
    }

    /// Adds progress toward `id`.
    pub fn increment(&self, id: &AchievementId, amount: u32) -> Result<(), AchievementError> {
        let mut g = self.entries.lock().expect("ach mutex poisoned");
        let e = g.get_mut(&id.0).ok_or_else(|| AchievementError::NotFound {
            id: id.clone(),
        })?;
        e.current = e.current.saturating_add(amount);
        if e.current >= e.target && matches!(e.state, UnlockState::Locked) {
            e.state = UnlockState::Pending;
        }
        Ok(())
    }

    /// Returns progress for `id`.
    pub fn state(&self, id: &AchievementId) -> AchievementProgress {
        let g = self.entries.lock().expect("ach mutex poisoned");
        let e = g.get(&id.0).unwrap();
        AchievementProgress {
            id: id.clone(),
            current: e.current,
            target: e.target,
            state: e.state.clone(),
        }
    }

    /// Simulates reconnecting to platform services.
    pub fn set_online(&self, online: bool) {
        *self.online.lock().expect("ach mutex poisoned") = online;
    }

    /// Flushes deferred unlock attempts.
    pub fn flush_deferred(&self) -> Result<u32, AchievementError> {
        if !*self.online.lock().expect("ach mutex poisoned") {
            return Ok(0);
        }
        self.deferred.lock().expect("ach mutex poisoned").clear();
        let mut g = self.entries.lock().expect("ach mutex poisoned");
        let mut count = 0u32;
        for e in g.values_mut() {
            if matches!(e.state, UnlockState::Pending) {
                e.state = UnlockState::Unlocked;
                count += 1;
            }
        }
        Ok(count)
    }
}

impl Default for AchievementService {
    fn default() -> Self {
        Self::new()
    }
}
