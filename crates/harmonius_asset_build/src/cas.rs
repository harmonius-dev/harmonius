//! Content-addressed store fakes (IR-5.1.7).

use parking_lot::Mutex;
use std::collections::{HashMap, VecDeque};

/// Store error surfaced to callers (no panic).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CasError {
    /// BLAKE3 mismatch on read.
    HashMismatch,
    /// Key not present.
    Missing,
    /// Concurrent writers corrupted CAS state (tests only).
    Corruption,
}

/// Malformed CAS key (wrong length) — IR-5.1.7.N1.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MalformedCasKeyError {
    pub len: usize,
}

/// Minimal CAS interface for integration tests.
pub trait CasStore {
    /// Writes `bytes` under its BLAKE3 content key; returns the key.
    fn put(&mut self, bytes: &[u8]) -> Result<[u8; 32], CasError>;

    /// Reads bytes for `key`.
    fn get(&self, key: &[u8; 32]) -> Result<Vec<u8>, CasError>;
}

fn validate_key(key: &[u8]) -> Result<[u8; 32], MalformedCasKeyError> {
    key.try_into()
        .map_err(|_| MalformedCasKeyError { len: key.len() })
}

/// In-memory CAS keyed by 32-byte BLAKE3 hashes.
#[derive(Clone, Debug, Default)]
pub struct MemoryCas {
    map: HashMap<[u8; 32], Vec<u8>>,
}

impl MemoryCas {
    /// Number of objects stored.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true when no blobs are stored.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl CasStore for MemoryCas {
    fn put(&mut self, bytes: &[u8]) -> Result<[u8; 32], CasError> {
        let key = *blake3::hash(bytes).as_bytes();
        self.map.insert(key, bytes.to_vec());
        Ok(key)
    }

    fn get(&self, key: &[u8; 32]) -> Result<Vec<u8>, CasError> {
        self.map.get(key).cloned().ok_or(CasError::Missing)
    }
}

/// Thread-safe CAS with compare-on-read integrity checks.
#[derive(Debug)]
pub struct ConcurrentCas {
    inner: Mutex<HashMap<[u8; 32], Vec<u8>>>,
}

impl Default for ConcurrentCas {
    fn default() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }
}

impl ConcurrentCas {
    /// Creates an empty concurrent CAS.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts content; concurrent writers with identical content converge to one key.
    pub fn put(&self, bytes: &[u8]) -> Result<[u8; 32], CasError> {
        let key = *blake3::hash(bytes).as_bytes();
        let mut guard = self.inner.lock();
        if let Some(existing) = guard.get(&key) {
            if existing != bytes {
                return Err(CasError::Corruption);
            }
            return Ok(key);
        }
        guard.insert(key, bytes.to_vec());
        Ok(key)
    }

    /// Reads a blob; verifies BLAKE3 matches `key`.
    pub fn get(&self, key: &[u8; 32]) -> Result<Vec<u8>, CasError> {
        let guard = self.inner.lock();
        let bytes = guard.get(key).cloned().ok_or(CasError::Missing)?;
        if blake3::hash(&bytes).as_bytes() != key {
            return Err(CasError::HashMismatch);
        }
        Ok(bytes)
    }
}

/// LRU-backed CAS with a hard entry cap (IR-5.1.7.4).
#[derive(Debug)]
pub struct LruCas {
    max_entries: usize,
    map: HashMap<[u8; 32], Vec<u8>>,
    lru: VecDeque<[u8; 32]>,
}

impl LruCas {
    /// Creates a CAS that retains at most `max_entries` blobs.
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries,
            map: HashMap::new(),
            lru: VecDeque::new(),
        }
    }

    fn touch(&mut self, key: [u8; 32]) {
        self.lru.retain(|k| *k != key);
        self.lru.push_back(key);
    }

    /// Inserts `bytes`, evicting the least-recently-used entry when at capacity.
    pub fn put(&mut self, bytes: &[u8]) -> Result<[u8; 32], CasError> {
        let key = *blake3::hash(bytes).as_bytes();
        if let Some(slot) = self.map.get_mut(&key) {
            *slot = bytes.to_vec();
            self.touch(key);
            return Ok(key);
        }
        if self.map.len() >= self.max_entries {
            if let Some(evicted) = self.lru.pop_front() {
                self.map.remove(&evicted);
            }
        }
        self.map.insert(key, bytes.to_vec());
        self.touch(key);
        Ok(key)
    }

    /// Returns whether `key` is resident after optional eviction.
    pub fn contains_key(&self, key: &[u8; 32]) -> bool {
        self.map.contains_key(key)
    }
}

/// Validates a CAS key slice (IR-5.1.7.N1).
pub fn parse_cas_key(key: &[u8]) -> Result<[u8; 32], MalformedCasKeyError> {
    validate_key(key)
}
