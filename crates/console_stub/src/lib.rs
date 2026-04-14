//! Desktop / editor stub for [`console_sdk::ConsoleSdk`].

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use console_sdk::{
    AccountHandle, Bytes, CertHandle, ConsoleConfig, ConsoleInitError, ConsoleSdk, ConsoleUser,
    ConsoleUserId, OnlineError, OnlineHandle, PresenceString, SaveSlot, SessionDesc, SessionId,
    SignInError, SlotId, StorageError, StorageHandle, TrophyError, TrophyHandle, TrophyId,
};
use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Default)]
struct StubInner {
    storage: Vec<(SlotId, Vec<u8>)>,
    next_session: u64,
    trophies: HashSet<TrophyId>,
    rating_lock: bool,
    suspend_count: u32,
    resume_count: u32,
    controller_disconnect_count: u32,
    input_pause_side_effect: bool,
    user_switch_cb: Option<Box<dyn Fn(ConsoleUser) + Send + Sync>>,
}

/// No-op / in-memory fake backend for non-console targets.
pub struct StubConsoleSdk {
    inner: Mutex<StubInner>,
}

impl std::fmt::Debug for StubConsoleSdk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StubConsoleSdk").finish_non_exhaustive()
    }
}

impl StubConsoleSdk {
    fn with_inner<T>(&self, f: impl FnOnce(&mut StubInner) -> T) -> T {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        f(&mut guard)
    }

    /// Test helper: suspend notification count.
    #[must_use]
    pub fn suspend_count(&self) -> u32 {
        self.with_inner(|inner| inner.suspend_count)
    }

    /// Test helper: resume notification count.
    #[must_use]
    pub fn resume_count(&self) -> u32 {
        self.with_inner(|inner| inner.resume_count)
    }

    /// Test helper: controller disconnect notification count.
    #[must_use]
    pub fn controller_disconnect_count(&self) -> u32 {
        self.with_inner(|inner| inner.controller_disconnect_count)
    }

    /// Test helper: whether rating lock is enabled.
    #[must_use]
    pub fn rating_lock_enabled(&self) -> bool {
        self.with_inner(|inner| inner.rating_lock)
    }

    /// Test helper: whether disconnect path toggled the input-pause side-effect flag.
    #[must_use]
    pub fn input_pause_after_disconnect(&self) -> bool {
        self.with_inner(|inner| inner.input_pause_side_effect)
    }
}

impl ConsoleSdk for StubConsoleSdk {
    type Account = StubConsoleSdk;
    type Cert = StubConsoleSdk;
    type Online = StubConsoleSdk;
    type Storage = StubConsoleSdk;
    type Trophies = StubConsoleSdk;

    fn init(_config: ConsoleConfig) -> Result<Self, ConsoleInitError> {
        Ok(Self {
            inner: Mutex::new(StubInner::default()),
        })
    }

    fn shutdown(self) {
        drop(self);
    }

    fn account(&self) -> &Self::Account {
        self
    }

    fn storage(&self) -> &Self::Storage {
        self
    }

    fn trophies(&self) -> &Self::Trophies {
        self
    }

    fn online(&self) -> &Self::Online {
        self
    }

    fn cert(&self) -> &Self::Cert {
        self
    }
}

impl AccountHandle for StubConsoleSdk {
    fn current_user(&self) -> Option<ConsoleUser> {
        None
    }

    fn sign_in(&self, required: bool) -> Result<ConsoleUser, SignInError> {
        if required {
            return Err(SignInError::Cancelled);
        }
        Ok(ConsoleUser {
            id: ConsoleUserId(0xDEAD_BEEF_0000_0001),
            display_name: "stub-user".to_string(),
            age_restricted: false,
        })
    }

    fn on_switch(&self, cb: Box<dyn Fn(ConsoleUser) + Send + Sync>) {
        self.with_inner(|inner| {
            inner.user_switch_cb = Some(cb);
        });
    }
}

impl StorageHandle for StubConsoleSdk {
    fn save_slots(&self) -> Vec<SaveSlot> {
        self.with_inner(|inner| {
            inner
                .storage
                .iter()
                .map(|(id, bytes)| SaveSlot {
                    id: *id,
                    size: bytes.len() as u64,
                    last_modified: 1,
                })
                .collect()
        })
    }

    fn write_slot(&self, slot: SlotId, bytes: &[u8]) -> Result<(), StorageError> {
        self.with_inner(|inner| {
            if let Some(existing) = inner.storage.iter_mut().find(|(s, _)| *s == slot) {
                existing.1 = bytes.to_vec();
            } else {
                inner.storage.push((slot, bytes.to_vec()));
            }
            Ok(())
        })
    }

    fn read_slot(&self, slot: SlotId) -> Result<Bytes, StorageError> {
        self.with_inner(|inner| {
            inner
                .storage
                .iter()
                .find(|(s, _)| *s == slot)
                .map(|(_, b)| b.clone())
                .ok_or(StorageError::MissingSlot)
        })
    }

    fn backup(&self, _slot: SlotId) -> Result<(), StorageError> {
        Ok(())
    }
}

impl TrophyHandle for StubConsoleSdk {
    fn unlock(&self, id: TrophyId) -> Result<(), TrophyError> {
        self.with_inner(|inner| {
            inner.trophies.insert(id);
            Ok(())
        })
    }

    fn progress(&self, id: TrophyId, _value: u32) -> Result<(), TrophyError> {
        if self.with_inner(|inner| inner.trophies.contains(&id)) {
            Ok(())
        } else {
            Err(TrophyError::UnknownTrophy)
        }
    }
}

impl OnlineHandle for StubConsoleSdk {
    fn session_create(&self, _desc: SessionDesc) -> Result<SessionId, OnlineError> {
        self.with_inner(|inner| {
            if inner.rating_lock {
                return Err(OnlineError::RatingLocked);
            }
            let id = SessionId(inner.next_session);
            inner.next_session = inner.next_session.saturating_add(1);
            Ok(id)
        })
    }

    fn session_join(&self, _id: SessionId) -> Result<(), OnlineError> {
        Ok(())
    }

    fn presence(&self, _set: PresenceString) {}
}

impl CertHandle for StubConsoleSdk {
    fn enforce_rating_lock(&self, enabled: bool) {
        self.with_inner(|inner| {
            inner.rating_lock = enabled;
        });
    }

    fn handle_suspend(&self) {
        self.with_inner(|inner| {
            inner.suspend_count = inner.suspend_count.saturating_add(1);
        });
    }

    fn handle_resume(&self) {
        self.with_inner(|inner| {
            inner.resume_count = inner.resume_count.saturating_add(1);
        });
    }

    fn handle_controller_disconnect(&self) {
        self.with_inner(|inner| {
            inner.controller_disconnect_count = inner.controller_disconnect_count.saturating_add(1);
            inner.input_pause_side_effect = true;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub_init_returns_ok() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        drop(sdk);
    }

    #[test]
    fn test_stub_account_current_user_none() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        assert!(sdk.account().current_user().is_none());
    }

    #[test]
    fn test_stub_account_sign_in_returns_fake() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        let user = sdk.account().sign_in(false).expect("sign-in");
        assert_eq!(user.id, ConsoleUserId(0xDEAD_BEEF_0000_0001));
    }

    #[test]
    fn test_stub_storage_list_empty() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        assert!(sdk.storage().save_slots().is_empty());
    }

    #[test]
    fn test_stub_storage_write_and_read_roundtrip() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        let slot = SlotId(0);
        sdk.storage().write_slot(slot, b"hello").expect("write ok");
        let bytes = sdk.storage().read_slot(slot).expect("read ok");
        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn test_stub_trophies_unlock_ok() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        let id = TrophyId(7);
        sdk.trophies().unlock(id).expect("unlock");
        sdk.trophies().progress(id, 3).expect("progress");
    }

    #[test]
    fn test_stub_online_session_create_ok() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        let sid = sdk
            .online()
            .session_create(SessionDesc {
                label: "lobby".to_string(),
            })
            .expect("session");
        assert_eq!(sid, SessionId(0));
    }

    #[test]
    fn test_stub_cert_all_methods_no_panic() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        sdk.cert().enforce_rating_lock(true);
        sdk.cert().handle_suspend();
        sdk.cert().handle_resume();
        sdk.cert().handle_controller_disconnect();
    }

    #[test]
    fn test_cert_handle_suspend_idempotent() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        sdk.cert().handle_suspend();
        sdk.cert().handle_suspend();
        assert_eq!(sdk.suspend_count(), 2);
    }

    #[test]
    fn test_cert_handle_resume_idempotent() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        sdk.cert().handle_resume();
        sdk.cert().handle_resume();
        assert_eq!(sdk.resume_count(), 2);
    }

    #[test]
    fn test_cert_handle_controller_disconnect() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        sdk.cert().handle_controller_disconnect();
        assert_eq!(sdk.controller_disconnect_count(), 1);
        assert!(sdk.input_pause_after_disconnect());
    }

    #[test]
    fn test_cert_rating_lock_toggles() {
        let sdk = StubConsoleSdk::init(ConsoleConfig).expect("init ok");
        sdk.cert().enforce_rating_lock(true);
        assert!(sdk.rating_lock_enabled());
        assert!(sdk.online().session_create(SessionDesc::default()).is_err());
        sdk.cert().enforce_rating_lock(false);
        assert!(!sdk.rating_lock_enabled());
        assert!(sdk.online().session_create(SessionDesc::default()).is_ok());
    }
}
