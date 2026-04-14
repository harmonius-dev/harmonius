//! SDF cell validation for upload (`SdfClampMode`).

use crate::types::SdfClampMode;

/// Non-finite SDF sample rejected under [`SdfClampMode::RejectNaN`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SdfSampleRejected;

/// Normalizes one SDF distance sample for GPU upload.
///
/// Returns `Ok(value)` when the sample is accepted, or [`Err(SdfSampleRejected)`] when
/// [`SdfClampMode::RejectNaN`] refuses a NaN or non-finite input.
pub fn clamp_sdf_cell(
    distance: f32,
    max_distance: f32,
    mode: SdfClampMode,
) -> Result<f32, SdfSampleRejected> {
    if distance.is_nan() {
        return match mode {
            SdfClampMode::ClampToMaxDistance => Ok(max_distance),
            SdfClampMode::RejectNaN => Err(SdfSampleRejected),
        };
    }
    if !distance.is_finite() {
        return match mode {
            SdfClampMode::ClampToMaxDistance => Ok(max_distance),
            SdfClampMode::RejectNaN => Err(SdfSampleRejected),
        };
    }
    Ok(distance.clamp(-max_distance, max_distance))
}

/// Editor-style validation: rejects any non-finite distance (including NaN).
pub fn reject_nan_sdf_cell(distance: f32) -> Result<f32, SdfSampleRejected> {
    if distance.is_nan() || !distance.is_finite() {
        return Err(SdfSampleRejected);
    }
    Ok(distance)
}

#[cfg(test)]
mod tests {
    use super::{clamp_sdf_cell, reject_nan_sdf_cell, SdfSampleRejected};
    use crate::types::SdfClampMode;

    /// TC-IR-3.3.N.5 — NaN clamped to max distance in clamp mode.
    #[test]
    fn nan_clamped_to_max_distance() {
        let out = clamp_sdf_cell(f32::NAN, 10.0, SdfClampMode::ClampToMaxDistance).unwrap();
        assert_eq!(out, 10.0);
    }

    /// TC-IR-3.3.N.6 — NaN rejected in editor mode.
    #[test]
    fn nan_rejected_in_editor_mode() {
        assert_eq!(reject_nan_sdf_cell(f32::NAN), Err(SdfSampleRejected));
        assert_eq!(
            clamp_sdf_cell(f32::NAN, 1.0, SdfClampMode::RejectNaN),
            Err(SdfSampleRejected)
        );
    }

    #[test]
    fn finite_passthrough() {
        assert_eq!(
            clamp_sdf_cell(3.5, 10.0, SdfClampMode::ClampToMaxDistance).unwrap(),
            3.5
        );
    }
}
