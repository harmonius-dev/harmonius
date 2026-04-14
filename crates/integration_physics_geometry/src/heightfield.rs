//! Heightfield collider construction and validation.

use core::fmt;

use glam::Vec3;

use crate::{CollisionLayers, PhysicsMaterialHandle, TerrainTileSample};

/// Errors returned while constructing a [`HeightfieldCollider`] from geometry inputs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HeightfieldBuildError {
    /// The quantized height buffer size does not match `samples_x * samples_z`.
    HeightCountMismatch {
        /// Expected number of samples.
        expected: usize,
        /// Actual number of samples provided.
        actual: usize,
    },
    /// FM-5: `scale.* * samples_*` disagrees with the authored world extent.
    ScaleWorldSizeMismatch {
        /// Which axis failed validation (`"x"` or `"z"`).
        axis: &'static str,
    },
    /// The tile resolution was zero (invalid grid).
    ZeroResolution,
}

impl fmt::Display for HeightfieldBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HeightCountMismatch { expected, actual } => write!(
                f,
                "height sample count mismatch: expected {expected} samples, got {actual}"
            ),
            Self::ScaleWorldSizeMismatch { axis } => write!(
                f,
                "heightfield scale does not match tile world size on axis {axis}"
            ),
            Self::ZeroResolution => write!(f, "terrain tile resolution must be non-zero"),
        }
    }
}

impl std::error::Error for HeightfieldBuildError {}

/// Validates `scale.x * samples_x` and `scale.z * samples_z` against the tile world sizes (FM-5).
pub fn validate_heightfield_scale(
    world_size_x: f32,
    world_size_z: f32,
    samples_x: u32,
    samples_z: u32,
    scale: Vec3,
) -> Result<(), HeightfieldBuildError> {
    if samples_x == 0 || samples_z == 0 {
        return Err(HeightfieldBuildError::ZeroResolution);
    }
    const EPS: f32 = 1e-3;
    if (scale.x * samples_x as f32 - world_size_x).abs() > EPS {
        return Err(HeightfieldBuildError::ScaleWorldSizeMismatch { axis: "x" });
    }
    if (scale.z * samples_z as f32 - world_size_z).abs() > EPS {
        return Err(HeightfieldBuildError::ScaleWorldSizeMismatch { axis: "z" });
    }
    Ok(())
}

/// Builds a [`HeightfieldCollider`] from a sampled terrain tile (IR-3.8.2).
///
/// The resulting collider keeps the quantized payload verbatim and carries the optional
/// [`crate::TerrainHole`] without conversion (IR-3.8.4).
pub fn heightfield_collider_from_tile(
    tile: &TerrainTileSample,
    material: PhysicsMaterialHandle,
    layers: CollisionLayers,
) -> Result<HeightfieldCollider, HeightfieldBuildError> {
    if tile.resolution == 0 {
        return Err(HeightfieldBuildError::ZeroResolution);
    }
    let samples_x = tile.resolution;
    let samples_z = tile.resolution;
    let expected = samples_x as usize * samples_z as usize;
    if tile.heights.len() != expected {
        return Err(HeightfieldBuildError::HeightCountMismatch {
            expected,
            actual: tile.heights.len(),
        });
    }
    let scale = Vec3::new(
        tile.world_size_x / samples_x as f32,
        1.0,
        tile.world_size_z / samples_z as f32,
    );
    validate_heightfield_scale(
        tile.world_size_x,
        tile.world_size_z,
        samples_x,
        samples_z,
        scale,
    )?;
    Ok(HeightfieldCollider {
        samples_x,
        samples_z,
        quantized_heights: tile.heights.clone(),
        min_height: tile.min_height,
        max_height: tile.max_height,
        scale,
        hole_mask: tile.holes.clone(),
        material,
        layers,
    })
}

/// Same as [`heightfield_collider_from_tile`], but uses an explicit horizontal scale (FM-5 tests).
pub fn heightfield_collider_from_tile_with_scale(
    tile: &TerrainTileSample,
    scale: Vec3,
    material: PhysicsMaterialHandle,
    layers: CollisionLayers,
) -> Result<HeightfieldCollider, HeightfieldBuildError> {
    if tile.resolution == 0 {
        return Err(HeightfieldBuildError::ZeroResolution);
    }
    let samples_x = tile.resolution;
    let samples_z = tile.resolution;
    let expected = samples_x as usize * samples_z as usize;
    if tile.heights.len() != expected {
        return Err(HeightfieldBuildError::HeightCountMismatch {
            expected,
            actual: tile.heights.len(),
        });
    }
    validate_heightfield_scale(
        tile.world_size_x,
        tile.world_size_z,
        samples_x,
        samples_z,
        scale,
    )?;
    Ok(HeightfieldCollider {
        samples_x,
        samples_z,
        quantized_heights: tile.heights.clone(),
        min_height: tile.min_height,
        max_height: tile.max_height,
        scale,
        hole_mask: tile.holes.clone(),
        material,
        layers,
    })
}

/// Heightfield collider built from quantized terrain samples (IR-3.8.2 / IR-3.8.4).
#[derive(Clone, Debug, PartialEq)]
pub struct HeightfieldCollider {
    /// Number of height samples along +X.
    pub samples_x: u32,
    /// Number of height samples along +Z.
    pub samples_z: u32,
    /// Quantized `u16` heights from geometry (`TerrainTile.heights`).
    pub quantized_heights: Vec<u16>,
    /// Minimum world height corresponding to quantized `0`.
    pub min_height: f32,
    /// Maximum world height corresponding to quantized `65535`.
    pub max_height: f32,
    /// Horizontal scale for each sample step (world units per sample).
    pub scale: Vec3,
    /// Optional hole mask carried verbatim from geometry.
    pub hole_mask: Option<crate::TerrainHole>,
    /// Material handle resolved later by the physics runtime.
    pub material: PhysicsMaterialHandle,
    /// Collision layers copied onto the collider for filtering (IR-3.8.6).
    pub layers: CollisionLayers,
}

impl HeightfieldCollider {
    /// Dequantizes the height sample at column `ix`, row `iz`.
    #[must_use]
    pub fn sample_height(&self, ix: u32, iz: u32) -> f32 {
        let index = (iz * self.samples_x + ix) as usize;
        let sample = self.quantized_heights[index];
        self.min_height
            + (f32::from(sample) / 65535.0) * (self.max_height - self.min_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TerrainHole, TerrainTileSample};

    fn flat_tile(resolution: u32, height: u16) -> TerrainTileSample {
        let len = resolution as usize * resolution as usize;
        TerrainTileSample {
            heights: vec![height; len],
            resolution,
            min_height: 0.0,
            max_height: 1000.0,
            world_size_x: 1000.0,
            world_size_z: 1000.0,
            holes: None,
        }
    }

    #[test]
    fn tc_ir_3_8_2_3_u16_dequantization_precision() {
        let tile = flat_tile(2, 32768);
        let hf = heightfield_collider_from_tile(
            &tile,
            PhysicsMaterialHandle { id: 1 },
            CollisionLayers {
                mask: 1,
                filter: !0,
            },
        )
        .expect("build");
        let value = hf.sample_height(0, 0);
        assert!((value - 500.0).abs() < 0.02, "got {value}");
    }

    #[test]
    fn tc_ir_3_8_2_1_heightfield_peak_matches_tile() {
        let resolution = 5_u32;
        let mut heights = vec![0_u16; resolution as usize * resolution as usize];
        let center = resolution / 2;
        let idx = (center * resolution + center) as usize;
        heights[idx] = 65_535;
        let tile = TerrainTileSample {
            heights,
            resolution,
            min_height: 0.0,
            max_height: 250.0,
            world_size_x: 100.0,
            world_size_z: 100.0,
            holes: None,
        };
        let hf = heightfield_collider_from_tile(
            &tile,
            PhysicsMaterialHandle { id: 1 },
            CollisionLayers {
                mask: 1,
                filter: !0,
            },
        )
        .expect("build");
        let peak = hf.sample_height(center, center);
        assert!((peak - 250.0).abs() < 0.02);
    }

    #[test]
    fn tc_ir_3_8_e5_scale_mismatch_rejected() {
        let tile = flat_tile(4, 1000);
        let bad_scale = Vec3::new(10.0, 1.0, 10.0); // does not satisfy world_size / samples
        let err = heightfield_collider_from_tile_with_scale(
            &tile,
            bad_scale,
            PhysicsMaterialHandle { id: 1 },
            CollisionLayers {
                mask: 1,
                filter: !0,
            },
        )
        .expect_err("should fail");
        assert_eq!(
            err,
            HeightfieldBuildError::ScaleWorldSizeMismatch { axis: "x" }
        );
    }

    #[test]
    fn tc_ir_3_8_4_3_hole_mask_layout_match() {
        let resolution = 8_u32;
        let holes = TerrainHole {
            mask: vec![0xFF; 8],
            resolution,
        };
        let mut tile = flat_tile(resolution, 0);
        tile.holes = Some(holes.clone());
        let hf = heightfield_collider_from_tile(
            &tile,
            PhysicsMaterialHandle { id: 1 },
            CollisionLayers {
                mask: 1,
                filter: !0,
            },
        )
        .expect("build");
        assert_eq!(hf.hole_mask.as_ref(), Some(&holes));
        for row in 0..resolution {
            for col in 0..resolution {
                assert_eq!(
                    hf.hole_mask
                        .as_ref()
                        .expect("holes")
                        .cell_is_hole(row, col),
                    holes.cell_is_hole(row, col)
                );
            }
        }
    }
}
