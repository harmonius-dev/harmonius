//! Hair level-of-detail selection from camera distance.

/// Active hair representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HairLod {
    /// Full strand simulation.
    Strand,
    /// Clustered strands.
    Cluster,
    /// Shell impostor.
    Shell,
}

/// Picks LOD from camera distance in meters.
pub fn hair_lod(distance_m: f32, strand_count: u32) -> (HairLod, u32) {
    if distance_m < 5.0 {
        (HairLod::Strand, strand_count)
    } else if distance_m < 45.0 {
        (HairLod::Cluster, strand_count / 4)
    } else {
        (HairLod::Shell, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_5_4_1_hair_lod() {
        let n = 256;
        let (l0, c0) = hair_lod(2.0, n);
        assert_eq!(l0, HairLod::Strand);
        assert_eq!(c0, n);
        let (l1, c1) = hair_lod(10.0, n);
        assert_eq!(l1, HairLod::Cluster);
        assert_eq!(c1, n / 4);
        let (l2, c2) = hair_lod(50.0, n);
        assert_eq!(l2, HairLod::Shell);
        assert_eq!(c2, 1);
    }
}
