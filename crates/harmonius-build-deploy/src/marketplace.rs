//! Asset marketplace catalog, dependencies, reviews, and publisher flows (R-15.17.*).

use std::collections::{BTreeMap, BTreeSet};

/// Asset listing row for catalog search (TC-15.17.1.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AssetListing {
    /// Stable asset identifier.
    pub id: u64,
    /// Human-readable name.
    pub name: String,
}

/// Performs a case-insensitive substring search (TC-15.17.1.1).
#[must_use]
pub(crate) fn catalog_search(listings: &[AssetListing], needle: &str) -> Vec<u64> {
    let n = needle.to_ascii_lowercase();
    listings
        .iter()
        .filter(|l| l.name.to_ascii_lowercase().contains(&n))
        .map(|l| l.id)
        .collect()
}

/// Resolves dependencies in topological order (TC-15.17.2.1).
#[must_use]
pub(crate) fn resolve_dependencies(edges: &[(u64, u64)]) -> Option<Vec<u64>> {
    let mut nodes = BTreeSet::new();
    for (a, b) in edges {
        nodes.insert(*a);
        nodes.insert(*b);
    }
    let mut indeg: BTreeMap<u64, u32> = nodes.iter().map(|n| (*n, 0)).collect();
    for (_a, b) in edges {
        *indeg.entry(*b).or_insert(0) += 1;
    }
    let mut remaining: BTreeSet<u64> = nodes.clone();
    let mut out = Vec::new();
    while !remaining.is_empty() {
        let mut ready: Vec<u64> = remaining
            .iter()
            .copied()
            .filter(|n| indeg.get(n).copied().unwrap_or(0) == 0)
            .collect();
        if ready.is_empty() {
            return None;
        }
        ready.sort_unstable();
        let pick = ready[0];
        remaining.remove(&pick);
        out.push(pick);
        for (a, b) in edges {
            if *a == pick {
                let e = indeg.get_mut(b).expect("node exists");
                *e = e.saturating_sub(1);
            }
        }
    }
    Some(out)
}

/// Review record (TC-15.17.3.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Review {
    /// Star rating 1-5.
    pub stars: u8,
    /// Free-form text.
    pub text: String,
}

/// In-memory review CRUD (TC-15.17.3.1).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ReviewStore {
    rows: BTreeMap<u64, Review>,
    next_id: u64,
}

impl ReviewStore {
    /// Creates an empty review store.
    #[must_use]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Inserts a review returning its id.
    pub(crate) fn create(&mut self, review: Review) -> u64 {
        self.next_id = self.next_id.saturating_add(1);
        let id = self.next_id;
        self.rows.insert(id, review);
        id
    }

    /// Reads a review by id.
    #[must_use]
    pub(crate) fn read(&self, id: u64) -> Option<&Review> {
        self.rows.get(&id)
    }

    /// Updates a review by id.
    pub(crate) fn update(&mut self, id: u64, review: Review) -> bool {
        self.rows
            .get_mut(&id)
            .map(|row| {
                *row = review;
                true
            })
            .unwrap_or(false)
    }

    /// Deletes a review by id.
    pub(crate) fn delete(&mut self, id: u64) -> bool {
        self.rows.remove(&id).is_some()
    }
}

/// Compat test trigger flag (TC-15.17.5.1).
#[must_use]
pub(crate) fn compat_test_should_run(changed_api: bool) -> bool {
    changed_api
}

/// Publisher dashboard counters (TC-15.17.4.1).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct PublisherMetrics {
    /// Total downloads.
    pub downloads: u64,
    /// Monthly revenue cents.
    pub revenue_cents: u64,
}

impl PublisherMetrics {
    /// Records a download event.
    pub(crate) fn record_download(&mut self) {
        self.downloads = self.downloads.saturating_add(1);
    }
}

/// Splits revenue by basis points shares that sum to 10_000 (TC-15.17.6.1).
#[must_use]
pub(crate) fn revenue_share_cents(
    total: u64,
    publisher_bps: u64,
    platform_bps: u64,
) -> Option<(u64, u64)> {
    if publisher_bps.saturating_add(platform_bps) != 10_000 {
        return None;
    }
    let p = total.saturating_mul(publisher_bps) / 10_000;
    let pl = total.saturating_mul(platform_bps) / 10_000;
    Some((p, pl))
}

/// Validates asset type manifest entries (TC-15.17.7.1).
#[must_use]
pub(crate) fn validate_asset_types(types: &[&str]) -> bool {
    !types.is_empty() && types.iter().all(|t| !t.is_empty())
}

#[cfg(test)]
mod tests {
    use super::{
        AssetListing, PublisherMetrics, Review, ReviewStore, catalog_search, compat_test_should_run,
        resolve_dependencies, revenue_share_cents, validate_asset_types,
    };

    /// TC-15.17.1.1 — Asset catalog search (R-15.17.1).
    #[test]
    fn tc_15_17_1_1_catalog_search() {
        let rows = vec![
            AssetListing {
                id: 1,
                name: "Wood Crate".into(),
            },
            AssetListing {
                id: 2,
                name: "Metal Beam".into(),
            },
        ];
        assert_eq!(catalog_search(&rows, "wood"), vec![1]);
    }

    /// TC-15.17.2.1 — Dependency resolution (R-15.17.2).
    #[test]
    fn tc_15_17_2_1_dep_resolution() {
        let edges = vec![(1, 2), (2, 3)];
        let order = resolve_dependencies(&edges).expect("acyclic");
        assert!(order.contains(&1));
        assert!(order.contains(&3));
    }

    /// TC-15.17.3.1 — Review CRUD (R-15.17.3).
    #[test]
    fn tc_15_17_3_1_review_crud() {
        let mut s = ReviewStore::new();
        let id = s.create(Review {
            stars: 5,
            text: "great".into(),
        });
        assert_eq!(s.read(id).unwrap().stars, 5);
        assert!(s.update(
            id,
            Review {
                stars: 4,
                text: "ok".into(),
            }
        ));
        assert!(s.delete(id));
        assert!(s.read(id).is_none());
    }

    /// TC-15.17.5.1 — Compat test trigger (R-15.17.5).
    #[test]
    fn tc_15_17_5_1_compat_trigger() {
        assert!(compat_test_should_run(true));
        assert!(!compat_test_should_run(false));
    }

    /// TC-15.17.4.1 — Publisher dashboard metrics (R-15.17.4).
    #[test]
    fn tc_15_17_4_1_publisher_metrics() {
        let mut m = PublisherMetrics::default();
        m.record_download();
        assert_eq!(m.downloads, 1);
    }

    /// TC-15.17.6.1 — Revenue share calculation (R-15.17.6).
    #[test]
    fn tc_15_17_6_1_revenue_share() {
        assert_eq!(revenue_share_cents(1000, 7000, 3000), Some((700, 300)));
        assert_eq!(revenue_share_cents(1000, 7000, 2000), None);
    }

    /// TC-15.17.7.1 — Asset type manifest validation (R-15.17.7).
    #[test]
    fn tc_15_17_7_1_asset_types() {
        assert!(validate_asset_types(&["tex", "mesh"]));
        assert!(!validate_asset_types(&[]));
    }
}
