//! Canonical BLAKE3 keys for the shared compiled-asset cache (R-15.11.1).

use blake3::Hasher;

/// Inputs that participate in a shared cache entry key.
///
/// All slices are hashed in a length-prefixed, field-tagged layout so adjacent fields cannot
/// alias one another.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CacheKeyInputs<'a> {
    /// Semver or build-id string for the toolchain that produced the artifact.
    pub tool_version: &'a str,
    /// Source bytes (file contents, serialized graph, etc.).
    pub source: &'a [u8],
    /// Serialized settings / cook flags / platform knobs.
    pub settings: &'a [u8],
}

/// Computes the 256-bit BLAKE3 digest used as a shared cache key (TC-15.11.1.1, TC-15.11.1.2).
#[must_use]
pub fn compute_shared_cache_key(inputs: &CacheKeyInputs<'_>) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(b"HARMONIUS_SHARED_CACHE_KEY_V1\0");
    hash_labeled_slice(
        &mut hasher,
        b"tool_version\0",
        inputs.tool_version.as_bytes(),
    );
    hash_labeled_slice(&mut hasher, b"source\0", inputs.source);
    hash_labeled_slice(&mut hasher, b"settings\0", inputs.settings);
    *hasher.finalize().as_bytes()
}

fn hash_labeled_slice(hasher: &mut Hasher, label: &[u8], payload: &[u8]) {
    hasher.update(label);
    let len = u64::try_from(payload.len()).expect("slice length fits u64");
    hasher.update(&len.to_le_bytes());
    hasher.update(payload);
}

#[cfg(test)]
mod tests {
    use super::{compute_shared_cache_key, CacheKeyInputs};

    /// TC-15.11.1.1 — Cache key compute (R-15.11.1).
    #[test]
    fn tc_15_11_1_1_cache_key_compute_matches_golden_vector() {
        let inputs = CacheKeyInputs {
            tool_version: "harmonius-tools/0.0.0",
            source: b"texture.png",
            settings: b"platform=macos;compress=bc7",
        };
        let key = compute_shared_cache_key(&inputs);
        assert_eq!(
            key,
            [
                0xf5, 0xec, 0x29, 0x39, 0x63, 0xda, 0x66, 0x2a, 0x27, 0x81, 0x90, 0x8d, 0x97, 0xa8,
                0xb0, 0x81, 0x44, 0xc6, 0x9d, 0x84, 0xa9, 0xd6, 0x98, 0x77, 0x3c, 0x05, 0x0c, 0x1b,
                0x02, 0x4f, 0xa7, 0x4c
            ],
            "golden vector locks the canonical serialization; update only if the format changes"
        );
    }

    /// TC-15.11.1.2 — Cache key determinism (R-15.11.1).
    #[test]
    fn tc_15_11_1_2_cache_key_determinism() {
        let inputs = CacheKeyInputs {
            tool_version: "1",
            source: b"x",
            settings: b"y",
        };
        assert_eq!(
            compute_shared_cache_key(&inputs),
            compute_shared_cache_key(&inputs)
        );
    }
}
