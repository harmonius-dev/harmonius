//! Depth-of-field helpers (circle of confusion, bokeh sampling).

/// Thin-lens style depth-of-field parameters.
#[derive(Clone, Copy, Debug)]
pub struct DofParams {
    /// Focus plane distance in meters.
    pub focus_distance: f32,
    /// Aperture f-number (e.g. 2.0 for f/2).
    pub aperture: f32,
    /// Focal length in meters (design uses ~0.05 for 50 mm).
    pub focal_length: f32,
}

/// Supported bokeh aperture shapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BokehShape {
    /// Circular aperture kernel.
    Circle,
    /// Hexagonal aperture (unused in current tests).
    Hexagon,
}

/// Signed circle of confusion radius in sensor units (arbitrary scale).
pub fn circle_of_confusion(depth: f32, params: &DofParams) -> f32 {
    let denom = depth.max(1e-6);
    params.aperture * (depth - params.focus_distance) / denom * params.focal_length
}

/// Samples `BokehShape::Circle` at `sample_count` evenly spaced angles.
pub fn bokeh_circle_samples(sample_count: usize) -> Vec<(f32, f32)> {
    (0..sample_count)
        .map(|i| {
            let t = std::f32::consts::TAU * i as f32 / sample_count as f32;
            (t.cos(), t.sin())
        })
        .collect()
}
