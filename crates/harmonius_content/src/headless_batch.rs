//! Deterministic headless batch hashing.

use std::collections::BTreeMap;
use std::path::Path;

use crate::ContentHash;

/// Walk a fake tree of `n` assets with deterministic bytes; return id→hash map.
pub fn run_headless_batch_twice(
    n: usize,
) -> (BTreeMap<u32, ContentHash>, BTreeMap<u32, ContentHash>) {
    fn once(n: usize) -> BTreeMap<u32, ContentHash> {
        let mut m = BTreeMap::new();
        for i in 0..n as u32 {
            let path = Path::new("asset").join(format!("{i}.bin"));
            let bytes = format!("{:032x}", i).into_bytes();
            let _ = path;
            m.insert(i, ContentHash::from_data(&bytes));
        }
        m
    }
    (once(n), once(n))
}
