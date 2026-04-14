//! Client prediction, input buffering, interpolation, extrapolation.

pub mod extrapolation;
pub mod input_buffer;
pub mod interpolation;
pub mod predictor;

pub use extrapolation::{extrapolate_position, smooth_correct};
pub use input_buffer::InputRing;
pub use interpolation::interpolate_snapshots;
pub use predictor::{replay_unacked, Correction, PredictedState};
