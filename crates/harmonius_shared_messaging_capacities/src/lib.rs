//! Canonical cross-thread channel capacities from
//! `docs/design/integration/shared-messaging-capacities.md`.
//!
//! Capacities mirror the normative `CH-*` rows; some rows are tuned beyond the
//! naive `MaxProd × Burst × Margin` product (see rationale notes in that doc).
//!
//! Reference worker count for `N` in MaxProd columns.
#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

/// Reference worker count (`N_WORKERS`) used when design table rows use `N` for
/// `MaxProducersPerFrame`.
pub const REFERENCE_N_WORKERS: u32 = 8;

/// Channel buffer layout from the integration design.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ChannelKind {
    /// Double buffer (two slots, overwrite semantics).
    DoubleBuf,
    /// Bounded multi-producer, single-consumer queue.
    Mpsc,
    /// Triple buffer (three slots, overwrite semantics).
    TripleBuf,
}

/// Overflow policy names from the integration design.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum OverflowPolicy {
    /// Block producers until consumers make space.
    BackPressure,
    /// Merge consecutive updates targeting the same key.
    CoalesceParams,
    /// Drop the oldest item and continue.
    DropOldest,
    /// Replace the destination slot without retaining history.
    Overwrite,
}

/// Stable `CH-*` identifiers from the shared messaging capacities table.
///
/// Discriminants match the design row index (`Ch01` == `CH-1`, value `1`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum MessageChannelId {
    Ch01 = 1,
    Ch02,
    Ch03,
    Ch04,
    Ch05,
    Ch06,
    Ch07,
    Ch08,
    Ch09,
    Ch10,
    Ch11,
    Ch12,
    Ch13,
    Ch14,
    Ch15,
    Ch16,
    Ch17,
    Ch18,
    Ch19,
    Ch20,
    Ch21,
    Ch22,
    Ch23,
    Ch24,
    Ch25,
    Ch26,
    Ch27,
    Ch28,
    Ch29,
    Ch30,
    Ch31,
    Ch32,
}

impl MessageChannelId {
    /// Returns `Some(id)` for `CH-1` … `CH-32` (`raw` 1..=32).
    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            1 => Some(Self::Ch01),
            2 => Some(Self::Ch02),
            3 => Some(Self::Ch03),
            4 => Some(Self::Ch04),
            5 => Some(Self::Ch05),
            6 => Some(Self::Ch06),
            7 => Some(Self::Ch07),
            8 => Some(Self::Ch08),
            9 => Some(Self::Ch09),
            10 => Some(Self::Ch10),
            11 => Some(Self::Ch11),
            12 => Some(Self::Ch12),
            13 => Some(Self::Ch13),
            14 => Some(Self::Ch14),
            15 => Some(Self::Ch15),
            16 => Some(Self::Ch16),
            17 => Some(Self::Ch17),
            18 => Some(Self::Ch18),
            19 => Some(Self::Ch19),
            20 => Some(Self::Ch20),
            21 => Some(Self::Ch21),
            22 => Some(Self::Ch22),
            23 => Some(Self::Ch23),
            24 => Some(Self::Ch24),
            25 => Some(Self::Ch25),
            26 => Some(Self::Ch26),
            27 => Some(Self::Ch27),
            28 => Some(Self::Ch28),
            29 => Some(Self::Ch29),
            30 => Some(Self::Ch30),
            31 => Some(Self::Ch31),
            32 => Some(Self::Ch32),
            _ => None,
        }
    }

    /// All rows in the canonical table, `CH-1` first.
    pub const fn all() -> [Self; 32] {
        [
            Self::Ch01,
            Self::Ch02,
            Self::Ch03,
            Self::Ch04,
            Self::Ch05,
            Self::Ch06,
            Self::Ch07,
            Self::Ch08,
            Self::Ch09,
            Self::Ch10,
            Self::Ch11,
            Self::Ch12,
            Self::Ch13,
            Self::Ch14,
            Self::Ch15,
            Self::Ch16,
            Self::Ch17,
            Self::Ch18,
            Self::Ch19,
            Self::Ch20,
            Self::Ch21,
            Self::Ch22,
            Self::Ch23,
            Self::Ch24,
            Self::Ch25,
            Self::Ch26,
            Self::Ch27,
            Self::Ch28,
            Self::Ch29,
            Self::Ch30,
            Self::Ch31,
            Self::Ch32,
        ]
    }
}

/// Documented capacity for `id` (slots in the bounded queue or buffer).
pub const fn canonical_capacity(id: MessageChannelId) -> usize {
    match id {
        MessageChannelId::Ch01 => 1024,
        MessageChannelId::Ch02 => 4096,
        MessageChannelId::Ch03 => 4096,
        MessageChannelId::Ch04 => 1024,
        MessageChannelId::Ch05 => 64,
        MessageChannelId::Ch06 => 3,
        MessageChannelId::Ch07 => 256,
        MessageChannelId::Ch08 => 256,
        MessageChannelId::Ch09 => 16,
        MessageChannelId::Ch10 => 64,
        MessageChannelId::Ch11 => 128,
        MessageChannelId::Ch12 => 128,
        MessageChannelId::Ch13 => 2,
        MessageChannelId::Ch14 => 256,
        MessageChannelId::Ch15 => 128,
        MessageChannelId::Ch16 => 1024,
        MessageChannelId::Ch17 => 64,
        MessageChannelId::Ch18 => 256,
        MessageChannelId::Ch19 => 256,
        MessageChannelId::Ch20 => 16,
        MessageChannelId::Ch21 => 16,
        MessageChannelId::Ch22 => 256,
        MessageChannelId::Ch23 => 2048,
        MessageChannelId::Ch24 => 32,
        MessageChannelId::Ch25 => 256,
        MessageChannelId::Ch26 => 256,
        MessageChannelId::Ch27 => 8,
        MessageChannelId::Ch28 => 256,
        MessageChannelId::Ch29 => 16,
        MessageChannelId::Ch30 => 512,
        MessageChannelId::Ch31 => 256,
        MessageChannelId::Ch32 => 3,
    }
}

/// Documented buffer kind for `id`.
pub const fn channel_kind(id: MessageChannelId) -> ChannelKind {
    match id {
        MessageChannelId::Ch06 | MessageChannelId::Ch32 => ChannelKind::TripleBuf,
        MessageChannelId::Ch13 => ChannelKind::DoubleBuf,
        _ => ChannelKind::Mpsc,
    }
}

/// Documented overflow policy for `id`.
pub const fn overflow_policy(id: MessageChannelId) -> OverflowPolicy {
    match id {
        MessageChannelId::Ch01
        | MessageChannelId::Ch02
        | MessageChannelId::Ch07
        | MessageChannelId::Ch08
        | MessageChannelId::Ch12
        | MessageChannelId::Ch14
        | MessageChannelId::Ch16
        | MessageChannelId::Ch23
        | MessageChannelId::Ch24
        | MessageChannelId::Ch25
        | MessageChannelId::Ch26
        | MessageChannelId::Ch27
        | MessageChannelId::Ch28
        | MessageChannelId::Ch30
        | MessageChannelId::Ch31 => OverflowPolicy::DropOldest,
        MessageChannelId::Ch03 | MessageChannelId::Ch18 => OverflowPolicy::CoalesceParams,
        MessageChannelId::Ch04
        | MessageChannelId::Ch05
        | MessageChannelId::Ch09
        | MessageChannelId::Ch10
        | MessageChannelId::Ch11
        | MessageChannelId::Ch15
        | MessageChannelId::Ch17
        | MessageChannelId::Ch19
        | MessageChannelId::Ch20
        | MessageChannelId::Ch21
        | MessageChannelId::Ch22
        | MessageChannelId::Ch29 => OverflowPolicy::BackPressure,
        MessageChannelId::Ch06 | MessageChannelId::Ch13 | MessageChannelId::Ch32 => {
            OverflowPolicy::Overwrite
        }
    }
}

/// MPSC sizing helper: `ceil(max_prod × burst × margin)` then next power of two.
///
/// `margin` is `margin_num / margin_den` (for example `3 / 2` == 1.5).
pub const fn mpsc_capacity_pow2(
    max_prod: u32,
    burst: u32,
    margin_num: u32,
    margin_den: u32,
) -> usize {
    assert!(margin_den != 0);
    let prod = (max_prod as u128)
        .saturating_mul(burst as u128)
        .saturating_mul(margin_num as u128);
    let ceil = prod.div_ceil(margin_den as u128);
    let ceil_u = if ceil > usize::MAX as u128 {
        usize::MAX
    } else {
        ceil as usize
    };
    if ceil_u <= 1 {
        return if ceil_u == 0 { 1 } else { ceil_u };
    }
    ceil_u.next_power_of_two()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_capacities_match_shared_messaging_design_table() {
        let expected: [(MessageChannelId, usize); 32] = [
            (MessageChannelId::Ch01, 1024),
            (MessageChannelId::Ch02, 4096),
            (MessageChannelId::Ch03, 4096),
            (MessageChannelId::Ch04, 1024),
            (MessageChannelId::Ch05, 64),
            (MessageChannelId::Ch06, 3),
            (MessageChannelId::Ch07, 256),
            (MessageChannelId::Ch08, 256),
            (MessageChannelId::Ch09, 16),
            (MessageChannelId::Ch10, 64),
            (MessageChannelId::Ch11, 128),
            (MessageChannelId::Ch12, 128),
            (MessageChannelId::Ch13, 2),
            (MessageChannelId::Ch14, 256),
            (MessageChannelId::Ch15, 128),
            (MessageChannelId::Ch16, 1024),
            (MessageChannelId::Ch17, 64),
            (MessageChannelId::Ch18, 256),
            (MessageChannelId::Ch19, 256),
            (MessageChannelId::Ch20, 16),
            (MessageChannelId::Ch21, 16),
            (MessageChannelId::Ch22, 256),
            (MessageChannelId::Ch23, 2048),
            (MessageChannelId::Ch24, 32),
            (MessageChannelId::Ch25, 256),
            (MessageChannelId::Ch26, 256),
            (MessageChannelId::Ch27, 8),
            (MessageChannelId::Ch28, 256),
            (MessageChannelId::Ch29, 16),
            (MessageChannelId::Ch30, 512),
            (MessageChannelId::Ch31, 256),
            (MessageChannelId::Ch32, 3),
        ];
        for (id, cap) in expected {
            assert_eq!(canonical_capacity(id), cap, "capacity mismatch for {id:?}");
        }
    }

    #[test]
    fn mpsc_capacity_pow2_applies_ceil_then_next_power_of_two() {
        assert_eq!(mpsc_capacity_pow2(1, 200, 3, 1), 1024);
        assert_eq!(mpsc_capacity_pow2(1, 7, 1, 1), 8);
        assert_eq!(mpsc_capacity_pow2(1, 1, 3, 2), 2);
        assert_eq!(mpsc_capacity_pow2(1, 1, 1, 1), 1);
    }

    #[test]
    fn naive_formula_aligns_with_table_for_select_rows() {
        let n = REFERENCE_N_WORKERS;
        assert_eq!(
            mpsc_capacity_pow2(n, 16, 2, 1),
            canonical_capacity(MessageChannelId::Ch14)
        );
        assert_eq!(
            mpsc_capacity_pow2(n, 16, 2, 1),
            canonical_capacity(MessageChannelId::Ch26)
        );
        assert_eq!(
            mpsc_capacity_pow2(n, 32, 2, 1),
            canonical_capacity(MessageChannelId::Ch30)
        );
    }

    #[test]
    fn documented_overrides_differ_from_naive_formula() {
        assert_ne!(
            mpsc_capacity_pow2(REFERENCE_N_WORKERS, 8, 4, 1),
            canonical_capacity(MessageChannelId::Ch03)
        );
        assert_ne!(
            mpsc_capacity_pow2(1, 8, 4, 1),
            canonical_capacity(MessageChannelId::Ch07)
        );
        assert_ne!(
            mpsc_capacity_pow2(REFERENCE_N_WORKERS, 16, 2, 1),
            canonical_capacity(MessageChannelId::Ch12)
        );
        assert_ne!(
            mpsc_capacity_pow2(1, 32, 4, 1),
            canonical_capacity(MessageChannelId::Ch16)
        );
        assert_ne!(
            mpsc_capacity_pow2(1, 500, 4, 1),
            canonical_capacity(MessageChannelId::Ch02)
        );
    }

    #[test]
    fn from_raw_round_trips_discriminants() {
        for raw in 1_u8..=32 {
            let id = MessageChannelId::from_raw(raw).expect("in range");
            assert_eq!(raw, id as u8);
        }
        assert!(MessageChannelId::from_raw(0).is_none());
        assert!(MessageChannelId::from_raw(33).is_none());
    }
}
