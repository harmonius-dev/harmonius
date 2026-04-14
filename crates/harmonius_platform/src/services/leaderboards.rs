//! Leaderboard submissions and cache (`R-14.5.2`).

use std::collections::HashMap;
use std::sync::Mutex;

use super::PlatformError;

/// Leaderboard identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LeaderboardId(pub String);

/// Sort direction for rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeaderboardSort {
    /// Ascending scores.
    Ascending,
    /// Descending scores.
    Descending,
}

/// Query scope for leaderboard reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeaderboardScope {
    /// Global board.
    Global,
    /// Friends-only slice (stub).
    FriendsOnly,
    /// Rows around the local player (stub).
    AroundPlayer,
}

/// One visible leaderboard row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeaderboardRow {
    /// 1-based rank.
    pub rank: u32,
    /// Display name.
    pub player_name: String,
    /// Numeric score.
    pub score: i64,
    /// Optional opaque player id.
    pub player_id: Option<String>,
}

/// Cached query response.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeaderboardResult {
    /// Rows returned for the query.
    pub rows: Vec<LeaderboardRow>,
}

/// Leaderboard-specific failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeaderboardError {
    /// Unknown leaderboard id.
    NotFound {
        /// Missing id.
        id: LeaderboardId,
    },
    /// Client-side rate limiting.
    RateLimited,
    /// Wrapped platform error.
    Platform(PlatformError),
}

/// Tracks pending score uploads and cached query results.
#[derive(Debug)]
pub struct LeaderboardService {
    pending: Mutex<Vec<(LeaderboardId, i64)>>,
    cache: Mutex<HashMap<String, LeaderboardResult>>,
}

impl LeaderboardService {
    /// Creates an empty service.
    pub fn new() -> Self {
        Self {
            pending: Mutex::new(Vec::new()),
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Seeds cached rows for `id` (test helper).
    pub fn seed_cache(&self, id: &LeaderboardId, rows: Vec<LeaderboardRow>) {
        self.cache
            .lock()
            .expect("lb mutex poisoned")
            .insert(id.0.clone(), LeaderboardResult { rows });
    }

    /// Queues a score submission.
    pub fn submit(&self, id: &LeaderboardId, score: i64) -> Result<(), LeaderboardError> {
        self.pending
            .lock()
            .expect("lb mutex poisoned")
            .push((id.clone(), score));
        Ok(())
    }

    /// Reads cached rows when present.
    pub fn query(
        &self,
        id: &LeaderboardId,
        _scope: LeaderboardScope,
        _offset: u32,
        _count: u32,
    ) -> Option<LeaderboardResult> {
        self.cache
            .lock()
            .expect("lb mutex poisoned")
            .get(&id.0)
            .cloned()
    }

    /// Flushes queued submissions.
    pub fn flush_pending(&self) -> Result<u32, LeaderboardError> {
        let mut p = self.pending.lock().expect("lb mutex poisoned");
        let n = p.len() as u32;
        p.clear();
        Ok(n)
    }

    /// Returns queued submission count.
    pub fn pending_len(&self) -> usize {
        self.pending.lock().expect("lb mutex poisoned").len()
    }
}

impl Default for LeaderboardService {
    fn default() -> Self {
        Self::new()
    }
}
