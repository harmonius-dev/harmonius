use crate::types::StringId;

/// Tag identifier used by [`TagSet`] and equipment compatibility checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TagId(pub u32);

/// Up to 64 simultaneous tags represented as a bit set (deterministic, allocation-free).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TagSet(pub u64);

impl TagSet {
    /// Returns whether `tag` is present in this set.
    #[must_use]
    pub const fn contains(self, tag: TagId) -> bool {
        let bit = 1_u64 << (tag.0 & 63);
        (self.0 & bit) != 0
    }

    /// Inserts `tag` into the set.
    pub const fn with(self, tag: TagId) -> Self {
        let bit = 1_u64 << (tag.0 & 63);
        Self(self.0 | bit)
    }
}

/// Minimal slot row used by filter and sort unit tests (no ECS).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotItem {
    /// Stable row id for stable-sort assertions.
    pub stable_id: u32,
    /// Display name used for [`SortCriteria::Name`].
    pub name: &'static str,
    /// Tags carried by the hypothetical item in this slot.
    pub tags: TagSet,
}

/// UI-side filter criteria (never mutates the authoritative container).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FilterCriteria {
    /// Show every non-empty slot row.
    None,
    /// Require intersection with this tag set.
    ByTag(TagSet),
    /// Reserved for substring filters keyed by [`StringId`].
    ByName(StringId),
    /// Designer extension id.
    Custom(u32),
}

/// Returns visibility mask aligned with `slots` (true == visible).
#[must_use]
pub fn filter_slots(slots: &[SlotItem], criteria: &FilterCriteria) -> Vec<bool> {
    match criteria {
        FilterCriteria::None => vec![true; slots.len()],
        FilterCriteria::ByTag(required) => {
            slots.iter().map(|s| (s.tags.0 & required.0) != 0).collect()
        }
        FilterCriteria::ByName(_) | FilterCriteria::Custom(_) => vec![true; slots.len()],
    }
}

/// Stable lexicographic sort by `name`, preserving relative order for equal names (TC-IR-5.9.7.U3).
pub fn sort_slots_stable_by_name(slots: &mut [SlotItem]) {
    slots.sort_by(|a, b| {
        a.name
            .cmp(b.name)
            .then_with(|| a.stable_id.cmp(&b.stable_id))
    });
}

/// When UTF-8 decoding fails, returns an empty pattern and sets `warned` once.
///
/// This is a UI-side normalization seam (TC-IR-5.9.7.FB2); higher layers wire logging.
#[must_use]
#[allow(dead_code)] // Used by integration tests and future `FilterCriteria::ByName` plumbing.
pub fn normalized_filter_subpattern<'a>(bytes: &'a [u8], warned: &mut bool) -> &'a str {
    match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => {
            if !*warned {
                *warned = true;
            }
            ""
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WEAPON: TagId = TagId(0);
    const ARMOR: TagId = TagId(1);

    #[test]
    fn tc_ir_5_9_7_u1_filter_none_passes_all() {
        let slots: Vec<SlotItem> = (0..20)
            .map(|i| SlotItem {
                stable_id: i,
                name: "x",
                tags: TagSet::default(),
            })
            .collect();
        let vis = filter_slots(&slots, &FilterCriteria::None);
        assert_eq!(vis.len(), 20);
        assert!(vis.iter().all(|&v| v));
    }

    #[test]
    fn tc_ir_5_9_7_u2_filter_by_tag_predicate() {
        let weapon_only = TagSet::default().with(WEAPON);
        let slots = vec![
            SlotItem {
                stable_id: 0,
                name: "sword",
                tags: TagSet::default().with(WEAPON),
            },
            SlotItem {
                stable_id: 1,
                name: "helmet",
                tags: TagSet::default().with(ARMOR),
            },
        ];
        let vis = filter_slots(&slots, &FilterCriteria::ByTag(weapon_only));
        assert!(vis[0]);
        assert!(!vis[1]);
    }

    #[test]
    fn tc_ir_5_9_7_u3_sort_stable_ordering() {
        let mut slots = vec![
            SlotItem {
                stable_id: 10,
                name: "b",
                tags: TagSet::default(),
            },
            SlotItem {
                stable_id: 11,
                name: "a",
                tags: TagSet::default(),
            },
            SlotItem {
                stable_id: 12,
                name: "b",
                tags: TagSet::default(),
            },
        ];
        sort_slots_stable_by_name(&mut slots);
        assert_eq!(slots[0].stable_id, 11);
        assert_eq!(slots[1].stable_id, 10);
        assert_eq!(slots[2].stable_id, 12);
    }

    #[test]
    fn tc_ir_5_9_7_fb2_invalid_utf8_filter_string() {
        let mut warned = false;
        let pat = normalized_filter_subpattern(&[0xFF, 0xFE], &mut warned);
        assert_eq!(pat, "");
        assert!(warned);
    }
}
