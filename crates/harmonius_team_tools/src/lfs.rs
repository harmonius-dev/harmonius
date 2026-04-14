//! Git LFS tracking rules and lock coordination (**TC-15.10.2.1**–**TC-15.10.2.4**, R-15.10.2).
//!
//! Lock state is held in an in-process table so unit tests exercise real mutual exclusion without
//! mock HTTP clients; production wiring replaces the shared store with channel-backed I/O.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::git_status::VcError;

fn store_poisoned() -> VcError {
    VcError::Git {
        message: "LFS lock store poisoned".to_string(),
    }
}

fn now_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Byte quota reported by the LFS server (placeholder until remote wiring exists).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuotaInfo {
    /// Bytes of LFS storage used for the active remote scope.
    pub used_bytes: u64,
    /// Upper bound bytes for the same scope, when known.
    pub limit_bytes: u64,
}

/// One auto-tracking rule: match file extension and optional minimum size on disk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRule {
    /// Lowercase extension **without** a leading dot (for example `"mesh"`).
    pub extension: String,
    /// When `Some(n)`, the file must exist and `metadata().len() >= n` to match.
    pub min_bytes: Option<u64>,
}

/// Coordinates LFS path classification and per-path locks (see design `LfsManager`).
#[derive(Debug)]
pub struct LfsManager {
    owner: String,
    rules: Vec<TrackingRule>,
    store: Arc<Mutex<HashMap<PathBuf, LockInfo>>>,
}

impl LfsManager {
    /// Build a manager with tracking rules and acting owner name for lock records.
    pub fn new(rules: Vec<TrackingRule>, owner: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            rules,
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Second editor (or test) sharing the same lock table as [`Self::new`].
    pub fn with_same_store(&self, owner: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            rules: self.rules.clone(),
            store: Arc::clone(&self.store),
        }
    }

    /// Whether `path` matches any rule using on-disk size when `min_bytes` is set.
    ///
    /// Extension-only rules require the file to exist on disk so hypothetical paths do not read as
    /// tracked.
    pub fn is_lfs_tracked(&self, path: &Path) -> bool {
        let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
            return false;
        };
        let ext_lower = ext.to_lowercase();
        for rule in &self.rules {
            if rule.extension != ext_lower {
                continue;
            }
            return match rule.min_bytes {
                None => path.exists(),
                Some(min) => fs::metadata(path)
                    .map(|m| m.len() >= min)
                    .unwrap_or(false),
            };
        }
        false
    }

    /// Acquire an LFS lock for `path` or return [`VcError::LfsLockHeld`].
    pub fn lock(&self, path: &Path) -> Result<(), VcError> {
        let path_buf = path.to_path_buf();
        let mut guard = self.store.lock().map_err(|_| store_poisoned())?;
        match guard.get(&path_buf) {
            Some(existing) if existing.owner != self.owner => Err(VcError::LfsLockHeld {
                holder: existing.owner.clone(),
                path: path_buf,
            }),
            Some(_) => Ok(()),
            None => {
                guard.insert(
                    path_buf.clone(),
                    LockInfo {
                        locked_at: now_epoch_secs(),
                        owner: self.owner.clone(),
                        path: path_buf,
                    },
                );
                Ok(())
            }
        }
    }

    /// Release a lock held by this manager's owner.
    pub fn unlock(&self, path: &Path) -> Result<(), VcError> {
        let path_buf = path.to_path_buf();
        let mut guard = self.store.lock().map_err(|_| store_poisoned())?;
        match guard.remove(&path_buf) {
            Some(info) if info.owner == self.owner => Ok(()),
            Some(info) => {
                guard.insert(path_buf.clone(), info);
                Err(VcError::LfsLockMismatch { path: path_buf })
            }
            None => Err(VcError::LfsLockMismatch { path: path_buf }),
        }
    }

    /// List every active lock with holder metadata (stable sort by path).
    pub fn locks(&self) -> Result<Vec<LockInfo>, VcError> {
        let guard = self.store.lock().map_err(|_| store_poisoned())?;
        let mut rows: Vec<LockInfo> = guard.values().cloned().collect();
        rows.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(rows)
    }

    /// Storage quota (not yet backed by network; returns zeros).
    pub fn quota(&self) -> Result<QuotaInfo, VcError> {
        let _ = self;
        Ok(QuotaInfo {
            limit_bytes: 0,
            used_bytes: 0,
        })
    }
}

/// One row from the LFS locks API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LockInfo {
    /// Locked repository-relative path.
    pub path: PathBuf,
    /// Human-readable owner id string.
    pub owner: String,
    /// Seconds since Unix epoch when the lock was granted (test clock / wall time).
    pub locked_at: u64,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use tempfile::TempDir;

    use super::*;

    fn epoch_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_secs()
    }

    /// **TC-15.10.2.1** — LFS tracking rules match by extension and size.
    #[test]
    fn tc_15_10_2_1_tracking_rules_match_extension_and_size() {
        let dir = TempDir::new().expect("tempdir");
        let mesh = dir.path().join("hero.mesh");
        fs::write(&mesh, [0u8; 16]).expect("write mesh");

        let small_txt = dir.path().join("note.txt");
        fs::write(&small_txt, b"hi").expect("write small txt");

        let big_txt = dir.path().join("blob.txt");
        fs::write(&big_txt, vec![b'x'; 4096]).expect("write big txt");

        let rules = vec![
            TrackingRule {
                extension: "mesh".to_string(),
                min_bytes: None,
            },
            TrackingRule {
                extension: "txt".to_string(),
                min_bytes: Some(2048),
            },
        ];
        let mgr = LfsManager::new(rules, "alice");

        assert!(mgr.is_lfs_tracked(&mesh));
        assert!(!mgr.is_lfs_tracked(&small_txt));
        assert!(mgr.is_lfs_tracked(&big_txt));
        assert!(!mgr.is_lfs_tracked(
            dir.path().join("missing.mesh").as_path(),
        ));
    }

    /// **TC-15.10.2.2** — LFS lock acquire succeeds when the path is free.
    #[test]
    fn tc_15_10_2_2_lock_acquires_server_side_lock() {
        let mgr = LfsManager::new(Vec::new(), "alice");
        let path = Path::new("assets/level.bin");
        mgr.lock(path).expect("lock free path");
        let list = mgr.locks().expect("list");
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].path, path);
        assert_eq!(list[0].owner, "alice");
        assert!(list[0].locked_at <= epoch_secs());
    }

    /// **TC-15.10.2.3** — Unlock releases the lock on commit or discard.
    #[test]
    fn tc_15_10_2_3_unlock_releases_lock() {
        let mgr = LfsManager::new(Vec::new(), "alice");
        let path = Path::new("tex/wood.texture");
        mgr.lock(path).expect("lock");
        assert_eq!(mgr.locks().expect("list").len(), 1);
        mgr.unlock(path).expect("unlock");
        assert!(mgr.locks().expect("list").is_empty());
    }

    /// **TC-15.10.2.4** — Locks query returns all current locks with holders.
    #[test]
    fn tc_15_10_2_4_locks_query_returns_holders() {
        let alice = LfsManager::new(Vec::new(), "alice");
        let bob = alice.with_same_store("bob");

        alice.lock(Path::new("a.bin")).expect("a");
        bob.lock(Path::new("b.bin")).expect("b");

        let mut list = alice.locks().expect("list");
        list.sort_by(|x, y| x.path.cmp(&y.path));

        assert_eq!(list.len(), 2);
        assert_eq!(list[0].path, Path::new("a.bin"));
        assert_eq!(list[0].owner, "alice");
        assert_eq!(list[1].path, Path::new("b.bin"));
        assert_eq!(list[1].owner, "bob");
    }

    /// **TC-15.10.5.1** — LFS lock attempt on locked path returns lock holder.
    #[test]
    fn tc_15_10_5_1_lock_attempt_reports_holder() {
        use crate::git_status::VcError;

        let alice = LfsManager::new(Vec::new(), "alice");
        let bob = alice.with_same_store("bob");
        let path = Path::new("models/hero.bin");
        alice.lock(path).expect("alice locks");
        let err = bob.lock(path).expect_err("bob blocked");
        match err {
            VcError::LfsLockHeld { holder, path: p } => {
                assert_eq!(holder, "alice");
                assert_eq!(p, path);
            }
            other => panic!("unexpected error {other:?}"),
        }
    }
}
