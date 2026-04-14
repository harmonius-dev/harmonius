/// Clamps a UI split quantity to `[1, source_stack_qty]` per IR-5.9.4 / TC-IR-5.9.4.*.
///
/// When `source_stack_qty == 0`, returns `0` (nothing to split).
#[must_use]
pub fn clamp_split_quantity(requested: u32, source_stack_qty: u32) -> u32 {
    if source_stack_qty == 0 {
        return 0;
    }
    let lower = 1;
    let upper = source_stack_qty;
    let normalized = if requested == 0 { lower } else { requested };
    normalized.clamp(lower, upper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_5_9_4_u1_split_clamp_lower_bound() {
        assert_eq!(clamp_split_quantity(0, 10), 1);
    }

    #[test]
    fn tc_ir_5_9_4_u2_split_clamp_upper_bound() {
        assert_eq!(clamp_split_quantity(11, 10), 10);
    }
}
