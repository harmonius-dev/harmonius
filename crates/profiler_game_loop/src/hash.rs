/// FNV-1a 32-bit hash for static profiling zone labels.
#[must_use]
pub fn fnv1a(data: &str) -> u32 {
    let mut hash: u32 = 0x811c_9dc5;
    for byte in data.as_bytes() {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

/// Extends an FNV-1a hash with raw bytes (e.g. a custom phase id payload).
#[must_use]
pub fn fnv1a_continue(mut hash: u32, data: &[u8]) -> u32 {
    for byte in data {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}
