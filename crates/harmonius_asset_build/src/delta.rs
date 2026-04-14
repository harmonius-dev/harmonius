//! BLAKE3-aware delta bundles between cooked manifests (IR-5.1.6).

use crate::merkle::{cooked_manifest_merkle_root, verify_cooked_manifest_root};
use crate::types::CookedManifest;

/// Delta patcher errors (no panic paths).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeltaError {
    /// `blake3_root` does not match entry CAS keys.
    MerkleRootMismatch,
    /// Source manifest v1 failed integrity checks.
    CorruptV1,
    /// Source manifest v2 failed integrity checks.
    CorruptV2,
    /// CAS lookup failed for a required key.
    MissingBlob,
}

/// Wire payload for a delta patch (simplified deterministic encoding).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeltaPatch {
    /// Serialized changed blobs: `(cas_key, bytes)` pairs in stable key order.
    pub payload: Vec<u8>,
}

/// Builds a delta patch from `v1` → `v2` using CAS lookups.
///
/// Identical manifests yield an empty payload (TC-IR-5.1.6.2).
pub fn build_delta_patch(
    v1: &CookedManifest,
    v2: &CookedManifest,
    resolve: impl Fn(&[u8; 32]) -> Option<Vec<u8>>,
) -> Result<DeltaPatch, DeltaError> {
    if !verify_cooked_manifest_root(v2) {
        return Err(DeltaError::MerkleRootMismatch);
    }
    if !verify_cooked_manifest_root(v1) {
        return Err(DeltaError::CorruptV1);
    }
    let mut changed: Vec<([u8; 32], Vec<u8>)> = Vec::new();
    for e2 in &v2.entries {
        let prev = v1.entries.iter().find(|e| e.asset_id == e2.asset_id);
        let need = match prev {
            None => true,
            Some(e1) => e1.cas_key != e2.cas_key,
        };
        if need {
            let bytes = resolve(&e2.cas_key).ok_or(DeltaError::MissingBlob)?;
            if blake3::hash(&bytes).as_bytes() != &e2.cas_key {
                return Err(DeltaError::CorruptV2);
            }
            changed.push((e2.cas_key, bytes));
        }
    }
    changed.sort_by_key(|(k, _)| *k);
    let mut payload = Vec::new();
    for (k, b) in changed {
        payload.extend_from_slice(&k);
        let len = b.len() as u64;
        payload.extend_from_slice(&len.to_le_bytes());
        payload.extend_from_slice(&b);
    }
    Ok(DeltaPatch { payload })
}

/// Returns true when `v2` Merkle root is wrong vs recomputed leaves.
pub fn detect_tampered_root(manifest: &CookedManifest) -> bool {
    !verify_cooked_manifest_root(manifest)
}

/// Returns patch payload size for statistics (TC-IR-5.1.6.1 / .2).
pub fn full_v2_wire_size(
    v2: &CookedManifest,
    resolve: impl Fn(&[u8; 32]) -> Option<Vec<u8>>,
) -> Result<usize, DeltaError> {
    if !verify_cooked_manifest_root(v2) {
        return Err(DeltaError::MerkleRootMismatch);
    }
    let mut total = 0usize;
    for e in &v2.entries {
        let b = resolve(&e.cas_key).ok_or(DeltaError::MissingBlob)?;
        total += 32 + 8 + b.len();
    }
    Ok(total)
}

/// Recomputes the Merkle root for `entries` (test helper).
pub fn merkle_over_entries(entries: &[[u8; 32]]) -> [u8; 32] {
    cooked_manifest_merkle_root(entries)
}
