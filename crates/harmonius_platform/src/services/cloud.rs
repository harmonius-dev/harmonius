//! Cloud storage stub (`R-14.5.5`).

use std::collections::HashMap;
use std::sync::Mutex;

/// Cloud key identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CloudKey(pub String);

/// Cloud quota or transport failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudError {
    /// Storage quota exceeded.
    QuotaExceeded {
        /// Bytes used.
        used: u64,
        /// Maximum bytes.
        max: u64,
    },
    /// Missing object key.
    KeyNotFound {
        /// Missing key.
        key: CloudKey,
    },
    /// Generic platform failure.
    Platform {
        /// Opaque code.
        code: i32,
    },
}

/// Conflict payload for merge UI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CloudConflict {
    /// Local bytes.
    pub local: Vec<u8>,
    /// Remote bytes.
    pub cloud: Vec<u8>,
}

/// In-memory cloud bucket for tests.
#[derive(Debug, Default)]
pub struct CloudStorageService {
    inner: Mutex<Inner>,
}

#[derive(Debug, Default)]
struct Inner {
    objects: HashMap<String, (Vec<u8>, u64)>,
    quota: Option<u64>,
    next_version: u64,
}

impl CloudStorageService {
    /// Creates an unbounded in-memory cloud.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an optional byte quota for uploads.
    pub fn set_quota(&self, max: Option<u64>) {
        let mut g = self.inner.lock().expect("cloud mutex poisoned");
        g.quota = max;
    }

    /// Uploads bytes under `key`.
    pub fn upload(&self, key: &CloudKey, data: &[u8]) -> Result<(), CloudError> {
        let mut g = self.inner.lock().expect("cloud mutex poisoned");
        if let Some(max) = g.quota {
            let used: u64 = g.objects.values().map(|(d, _)| d.len() as u64).sum();
            if used + data.len() as u64 > max {
                return Err(CloudError::QuotaExceeded {
                    used: used + data.len() as u64,
                    max,
                });
            }
        }
        g.next_version = g.next_version.saturating_add(1);
        let v = g.next_version;
        g.objects.insert(key.0.clone(), (data.into(), v));
        Ok(())
    }

    /// Downloads bytes previously uploaded.
    pub fn download(&self, key: &CloudKey) -> Result<Vec<u8>, CloudError> {
        let g = self.inner.lock().expect("cloud mutex poisoned");
        g.objects
            .get(&key.0)
            .map(|(d, _)| d.clone())
            .ok_or_else(|| CloudError::KeyNotFound { key: key.clone() })
    }

    /// Detects whether the cloud copy is newer than `old_timestamp`.
    pub fn check_conflict(
        &self,
        key: &CloudKey,
        local_data: &[u8],
        old_timestamp: u64,
    ) -> Result<CloudConflict, CloudError> {
        let g = self.inner.lock().expect("cloud mutex poisoned");
        let Some((cloud, ver)) = g.objects.get(&key.0) else {
            return Err(CloudError::KeyNotFound { key: key.clone() });
        };
        if *ver > old_timestamp {
            Ok(CloudConflict {
                local: local_data.into(),
                cloud: cloud.clone(),
            })
        } else {
            Err(CloudError::Platform { code: 0 })
        }
    }
}
