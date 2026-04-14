//! Camera sequencer blending for cutscenes (`R-2.7.4`).

/// Minimal camera pose used by the sequencer tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraPose {
    /// Position in world meters.
    pub position: [f32; 3],
}

/// Keyframe payload referencing a camera slot index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CameraKeyframe {
    /// Index into the sequencer's camera list.
    pub camera_index: u8,
}

/// Runtime state for a simple A/B camera sequencer.
#[derive(Clone, Debug, PartialEq)]
pub struct CameraSequencerState {
    /// Camera entities in cutscene order.
    pub cameras: Vec<CameraPose>,
    /// Left keyframe index.
    pub index_a: u8,
    /// Right keyframe index.
    pub index_b: u8,
    /// Blend factor from A toward B in `[0,1]`.
    pub blend_t: f32,
}

/// Linearly interpolates two camera poses.
#[must_use]
pub fn blend_camera_pose(a: CameraPose, b: CameraPose, t: f32) -> CameraPose {
    let t = t.clamp(0.0, 1.0);
    CameraPose {
        position: [
            a.position[0] + (b.position[0] - a.position[0]) * t,
            a.position[1] + (b.position[1] - a.position[1]) * t,
            a.position[2] + (b.position[2] - a.position[2]) * t,
        ],
    }
}

impl CameraSequencerState {
    /// Returns the blended pose for the current `(index_a,index_b,blend_t)` triple.
    #[must_use]
    pub fn blended_pose(&self) -> Option<CameraPose> {
        let a = *self.cameras.get(usize::from(self.index_a))?;
        let b = *self.cameras.get(usize::from(self.index_b))?;
        Some(blend_camera_pose(a, b, self.blend_t))
    }
}

#[cfg(test)]
mod tests {
    use super::{blend_camera_pose, CameraPose, CameraSequencerState};

    /// TC-2.7.4.1 — two-camera blend at `t = 0.5` is the midpoint.
    #[test]
    fn test_sequencer_blend_ab() {
        let a = CameraPose {
            position: [0.0, 0.0, 0.0],
        };
        let b = CameraPose {
            position: [2.0, 0.0, 0.0],
        };
        let blended = blend_camera_pose(a, b, 0.5);
        assert!((blended.position[0] - 1.0).abs() < 1.0e-4);
    }

    /// TC-2.7.4.2 — three-slot sequencer resolves blended poses across keyframes.
    #[test]
    fn test_sequencer_cutscene_end_to_end() {
        let cameras = vec![
            CameraPose {
                position: [0.0, 0.0, 0.0],
            },
            CameraPose {
                position: [0.0, 2.0, 0.0],
            },
            CameraPose {
                position: [0.0, 0.0, 4.0],
            },
        ];
        let seq = CameraSequencerState {
            cameras: cameras.clone(),
            index_a: 0,
            index_b: 1,
            blend_t: 0.25,
        };
        let p0 = seq.blended_pose().expect("valid blend");
        assert!((p0.position[1] - 0.5).abs() < 1.0e-3);

        let seq = CameraSequencerState {
            cameras,
            index_a: 1,
            index_b: 2,
            blend_t: 0.5,
        };
        let p1 = seq.blended_pose().expect("valid blend");
        assert!((p1.position[2] - 2.0).abs() < 1.0e-3);
    }
}
