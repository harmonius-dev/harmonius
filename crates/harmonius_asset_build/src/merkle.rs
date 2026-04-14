//! BLAKE3 Merkle root over `CookedEntry::cas_key` leaves (IR-5.1.6).

use blake3::Hasher;

use crate::types::CookedManifest;

/// Computes the manifest Merkle root from entry CAS keys (does not read `manifest.blake3_root`).
pub fn cooked_manifest_merkle_root(entries: &[[u8; 32]]) -> [u8; 32] {
    if entries.is_empty() {
        return *blake3::hash(b"harmonius:cooked_manifest:empty").as_bytes();
    }
    let mut level: Vec<[u8; 32]> = entries.to_vec();
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut i = 0usize;
        while i < level.len() {
            if i + 1 < level.len() {
                let mut h = Hasher::new();
                h.update(&level[i]);
                h.update(&level[i + 1]);
                next.push(*h.finalize().as_bytes());
                i += 2;
            } else {
                next.push(level[i]);
                i += 1;
            }
        }
        level = next;
    }
    level[0]
}

/// Recomputes the Merkle root and compares it to `manifest.blake3_root`.
pub fn verify_cooked_manifest_root(manifest: &CookedManifest) -> bool {
    let leaves: Vec<[u8; 32]> = manifest.entries.iter().map(|e| e.cas_key).collect();
    let expected = cooked_manifest_merkle_root(&leaves);
    expected == manifest.blake3_root
}
