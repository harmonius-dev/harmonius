#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! GPU-facing render camera helpers: cine lens math, spring arms, exposure, DoF, distortion,
//! layer masks, PiP ordering, and camera sequencing.
//!
//! This crate implements deterministic, testable **pure functions** and small state machines that
//! mirror the Harmonius `camera-rendering` design. ECS wiring lives in higher layers.

pub mod cine_lens;
pub mod distortion;
pub mod dof;
pub mod exposure;
pub mod layer_mask;
pub mod pip;
pub mod sequencer;
pub mod spring_arm;

pub use cine_lens::CineLens;
pub use distortion::DistortionSettings;
pub use dof::{circle_of_confusion_m, BokehShape, DofSettings};
pub use exposure::{ExposureMode, ExposureSettings, HistogramSample};
pub use layer_mask::RenderLayerMask;
pub use pip::{composite_order, PipCamera, PipRect};
pub use sequencer::{blend_camera_pose, CameraKeyframe, CameraPose, CameraSequencerState};
pub use spring_arm::{boom_tip, SphereCastQuery, SphereCastResult, SpringArm, SpringArmWorld};
