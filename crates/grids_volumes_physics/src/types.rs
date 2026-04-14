//! ECS entity handle, voxel coordinates, and destruction request payloads.

use rkyv::{Archive, Deserialize, Serialize};

/// Stable entity identifier used in occupancy and hit results.
pub type Entity = u64;

/// Integer coordinates of a cell in a uniform tactical grid (XY; Z extruded at integration sites).
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[rkyv(derive(Debug, Eq, Hash, PartialEq), compare(PartialEq))]
pub struct CellCoord {
    /// X index in cell space.
    pub x: i32,
    /// Y index in cell space.
    pub y: i32,
}

/// Integer coordinates of a voxel inside a volume.
#[derive(Archive, Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[rkyv(derive(Debug, PartialEq), compare(PartialEq))]
pub struct VoxelCoord {
    /// X index in voxel space.
    pub x: i32,
    /// Y index in voxel space.
    pub y: i32,
    /// Z index in voxel space.
    pub z: i32,
}

/// Shape of a destruction region in voxel space.
#[derive(Archive, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[rkyv(derive(Debug, PartialEq), compare(PartialEq))]
pub enum DestructionPattern {
    /// Spherical blast clears voxels in radius.
    Sphere,
    /// Directional cone along impact normal; `half_angle` radians in `0..=PI/2`.
    Cone {
        /// Half-angle of the cone in radians.
        half_angle: f32,
    },
    /// Column collapse downward from impact.
    Column,
}

/// Destruction applied to a voxel volume after physics events are mapped to voxel space.
#[derive(Archive, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[rkyv(derive(Debug, PartialEq), compare(PartialEq))]
pub struct VoxelDestructionRequest {
    /// Volume entity owning the `VoxelVolume`.
    pub volume_entity: Entity,
    /// Voxel-space impact coordinate.
    pub impact_coord: VoxelCoord,
    /// Pattern-specific radius (e.g. sphere radius in voxels).
    pub radius: u32,
    /// Impact strength for gameplay systems.
    pub force: f32,
    /// Region shape.
    pub pattern: DestructionPattern,
}

/// Returns a distinct tag per [`DestructionPattern`] variant (exhaustiveness guard for tests).
#[must_use]
pub const fn destruction_pattern_tag(pattern: DestructionPattern) -> u8 {
    match pattern {
        DestructionPattern::Sphere => 0,
        DestructionPattern::Cone { .. } => 1,
        DestructionPattern::Column => 2,
    }
}

#[cfg(test)]
#[allow(missing_docs)]
mod tests {
    use super::*;
    use rkyv::rancor::Failure;

    #[test]
    fn tc_ir_3_10_u1_voxel_destruction_request_rkyv_round_trip() {
        let orig = VoxelDestructionRequest {
            volume_entity: 42,
            impact_coord: VoxelCoord { x: 8, y: 8, z: 8 },
            radius: 3,
            force: 1.0,
            pattern: DestructionPattern::Sphere,
        };
        let bytes = rkyv::to_bytes::<Failure>(&orig).expect("serialize");
        let deserialized = rkyv::from_bytes::<VoxelDestructionRequest, Failure>(bytes.as_slice())
            .expect("deserialize");
        assert_eq!(orig, deserialized);
    }

    #[test]
    fn tc_ir_3_10_u2_destruction_pattern_enum_complete() {
        assert_eq!(destruction_pattern_tag(DestructionPattern::Sphere), 0);
        assert_eq!(
            destruction_pattern_tag(DestructionPattern::Cone { half_angle: 0.5 }),
            1,
        );
        assert_eq!(destruction_pattern_tag(DestructionPattern::Column), 2);
    }
}
