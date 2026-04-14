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
