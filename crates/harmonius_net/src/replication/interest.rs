//! Area-of-interest filtering.

/// Test entity id + planar distance in meters from origin.
#[derive(Clone, Copy, Debug)]
pub struct EntityPos {
    /// Id.
    pub id: u32,
    /// Distance from listener along one axis (test uses 10..1000 step 10).
    pub distance_m: f32,
}

/// Returns entity ids within `radius_m` of origin (inclusive).
pub fn filter_aoi_radius_m(entities: &[EntityPos], radius_m: f32) -> Vec<u32> {
    entities
        .iter()
        .filter(|e| e.distance_m <= radius_m)
        .map(|e| e.id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.2.3.1 — only entities within 50 m.
    #[test]
    fn test_aoi_radius_filter() {
        let ents: Vec<EntityPos> = (1..=100)
            .map(|i| EntityPos {
                id: i,
                distance_m: (i * 10) as f32,
            })
            .collect();
        let in_aoi = filter_aoi_radius_m(&ents, 50.0);
        assert_eq!(in_aoi, vec![1, 2, 3, 4, 5]);
    }
}
