//! Display identifiers for multi-monitor window placement.

/// Unique identifier for a connected display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DisplayId(pub u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_14_1_3_1_display_id_equality() {
        assert_eq!(DisplayId(1), DisplayId(1));
        assert_ne!(DisplayId(1), DisplayId(2));
    }
}
