//! Bundle packing from a cooked manifest (IR-5.1.4).

use blake3::Hasher;

use crate::types::{CookedManifest, TargetPlatform};

/// One packed blob inside a bundle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BundleEntry {
    /// CAS key for this segment.
    pub cas_key: [u8; 32],
    pub bytes: Vec<u8>,
}

/// One bundle partition under a size limit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BundleShard {
    /// Ordered entries in this shard (stable cook order).
    pub entries: Vec<BundleEntry>,
    /// BLAKE3 over concatenated `cas_key || len || bytes` in order.
    pub blake3: [u8; 32],
}

/// Full bundle set for a cook.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BundleSet {
    pub platform: TargetPlatform,
    pub shards: Vec<BundleShard>,
}

fn hash_shard(entries: &[BundleEntry]) -> [u8; 32] {
    let mut h = Hasher::new();
    for e in entries {
        h.update(&e.cas_key);
        h.update(&e.bytes.len().to_le_bytes());
        h.update(&e.bytes);
    }
    *h.finalize().as_bytes()
}

/// Splits `manifest` blobs (via `resolve`) into shards each at most `max_shard_bytes`.
pub fn build_bundles(
    manifest: &CookedManifest,
    max_shard_bytes: u64,
    mut resolve: impl FnMut(&[u8; 32]) -> Option<Vec<u8>>,
) -> Option<BundleSet> {
    let mut shards = Vec::new();
    let mut current: Vec<BundleEntry> = Vec::new();
    let mut current_size: u64 = 0;
    for entry in &manifest.entries {
        let bytes = resolve(&entry.cas_key)?;
        let entry_size = 32u64 + 8 + bytes.len() as u64;
        if entry_size > max_shard_bytes {
            return None;
        }
        if !current.is_empty() && current_size + entry_size > max_shard_bytes {
            let h = hash_shard(&current);
            shards.push(BundleShard {
                entries: std::mem::take(&mut current),
                blake3: h,
            });
            current_size = 0;
        }
        current_size += entry_size;
        current.push(BundleEntry {
            cas_key: entry.cas_key,
            bytes,
        });
    }
    if !current.is_empty() {
        let h = hash_shard(&current);
        shards.push(BundleShard {
            entries: current,
            blake3: h,
        });
    }
    Some(BundleSet {
        platform: manifest.platform,
        shards,
    })
}
