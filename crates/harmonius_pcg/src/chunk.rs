//! Deterministic chunk generation and simple LRU-style memory accounting.

use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use twox_hash::XxHash64;

/// Fingerprint of chunk contents from `(seed, cx, cz)`.
pub fn chunk_digest(seed: u64, cx: i32, cz: i32) -> u64 {
    let mut h = XxHash64::with_seed(seed);
    cx.hash(&mut h);
    cz.hash(&mut h);
    h.finish()
}

/// LRU-ish cache with byte budget (each chunk costs `chunk_cost_bytes`).
#[derive(Debug)]
pub struct ChunkMemoryCache {
    budget_bytes: usize,
    chunk_cost_bytes: usize,
    order: VecDeque<(i32, i32)>,
    map: HashMap<(i32, i32), u64>,
}

impl ChunkMemoryCache {
    /// New cache with `budget_bytes` and fixed per-chunk cost.
    pub fn new(budget_bytes: usize, chunk_cost_bytes: usize) -> Self {
        Self {
            budget_bytes,
            chunk_cost_bytes,
            order: VecDeque::new(),
            map: HashMap::new(),
        }
    }

    /// Resident bytes (approximate).
    pub fn used_bytes(&self) -> usize {
        self.map.len() * self.chunk_cost_bytes
    }

    /// Inserts or refreshes a chunk digest, evicting LRU until under budget.
    pub fn touch(&mut self, coord: (i32, i32), digest: u64) {
        if self.map.remove(&coord).is_some() {
            self.order.retain(|&c| c != coord);
        }
        self.map.insert(coord, digest);
        self.order.push_back(coord);
        while self.used_bytes() > self.budget_bytes {
            if let Some(old) = self.order.pop_front() {
                self.map.remove(&old);
            } else {
                break;
            }
        }
    }
}
