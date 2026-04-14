//! Mobile tier-1 visibility buffer packing (IR-3.2.10).

/// Packs a 20-bit instance id and 12-bit triangle id for the mobile V-buffer.
///
/// # Panics
///
/// Debug builds panic when `instance_id` or `triangle_id` exceed their bit widths.
#[must_use]
pub fn pack_vbuffer_mobile(instance_id: u32, triangle_id: u32) -> u32 {
    debug_assert!(instance_id < (1 << 20));
    debug_assert!(triangle_id < (1 << 12));
    (instance_id & 0x000F_FFFF) | ((triangle_id & 0x0FFF) << 20)
}

/// Unpacks [`pack_vbuffer_mobile`].
#[must_use]
pub fn unpack_vbuffer_mobile(packed: u32) -> (u32, u32) {
    let instance_id = packed & 0x000F_FFFF;
    let triangle_id = (packed >> 20) & 0x0FFF;
    (instance_id, triangle_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_2_10_2_roundtrip() {
        let cases = [
            (0u32, 0u32),
            (1, 1),
            (999_999, 100),
            ((1 << 20) - 1, (1 << 12) - 1),
        ];
        for &(inst, tri) in &cases {
            let p = pack_vbuffer_mobile(inst, tri);
            assert_eq!(unpack_vbuffer_mobile(p), (inst, tri));
        }
    }
}
