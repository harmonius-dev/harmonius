use crate::uniform::UniformGrid;

/// Simple statistics for propagation passes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropagationStats {
    /// Number of cells visited.
    pub cells_visited: u32,
}

/// Four-neighbor diffusion step used by integration tests (IR-2.3.6.7).
///
/// Reads the current `back` buffer, writes an updated field back into `back`.
pub fn propagate_influence_2d_four_way(grid: &mut UniformGrid<f32>) -> PropagationStats {
    let width = grid.width() as usize;
    let height = grid.height() as usize;
    let src: Vec<f32> = grid.back_slice().to_vec();
    let mut dst = src.clone();
    let mut visited = 0_u32;

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let mut sum = src[idx];
            let mut count = 1_u32;
            if x > 0 {
                sum += src[idx - 1];
                count += 1;
            }
            if x + 1 < width {
                sum += src[idx + 1];
                count += 1;
            }
            if y > 0 {
                sum += src[idx - width];
                count += 1;
            }
            if y + 1 < height {
                sum += src[idx + width];
                count += 1;
            }
            dst[idx] = sum / (count as f32);
            visited += 1;
        }
    }

    grid.back_mut().copy_from_slice(&dst);
    PropagationStats {
        cells_visited: visited,
    }
}
