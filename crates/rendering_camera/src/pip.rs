//! Picture-in-picture viewport ordering (`R-2.7.3`).

/// Normalized viewport rectangle in `[0,1]` UV space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PipRect {
    /// Left edge in normalized horizontal space.
    pub x: f32,
    /// Top edge in normalized vertical space.
    pub y: f32,
    /// Width in normalized horizontal space.
    pub w: f32,
    /// Height in normalized vertical space.
    pub h: f32,
}

/// A lightweight PiP camera description used for ordering tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PipCamera {
    /// Stable identifier for assertions.
    pub id: u32,
    /// Higher priority cameras are composited **after** lower priority cameras.
    pub priority: i16,
    /// Viewport footprint.
    pub rect: PipRect,
}

/// Returns camera indices sorted for composite: low priority first, high priority last (on top).
#[must_use]
pub fn composite_order(cameras: &[PipCamera]) -> Vec<u32> {
    let mut ordered: Vec<PipCamera> = cameras.to_vec();
    ordered.sort_by_key(|c| c.priority);
    ordered.into_iter().map(|c| c.id).collect()
}

#[cfg(test)]
mod tests {
    use super::{composite_order, PipCamera, PipRect};

    /// TC-2.7.3.1 — two cameras produce a deterministic composite sequence.
    #[test]
    fn test_pip_two_viewports_composite() {
        let main = PipCamera {
            id: 1,
            priority: 0,
            rect: PipRect {
                x: 0.0,
                y: 0.0,
                w: 1.0,
                h: 1.0,
            },
        };
        let pip = PipCamera {
            id: 2,
            priority: 10,
            rect: PipRect {
                x: 0.7,
                y: 0.7,
                w: 0.25,
                h: 0.25,
            },
        };
        let order = composite_order(&[pip, main]);
        assert_eq!(order, vec![1, 2]);
    }

    /// TC-2.7.3.2 — four cameras are all present in the composite ordering.
    #[test]
    fn test_pip_four_viewports_composite() {
        let cams: Vec<PipCamera> = (0..4)
            .map(|idx| PipCamera {
                id: idx,
                priority: idx as i16,
                rect: PipRect {
                    x: 0.0,
                    y: 0.0,
                    w: 0.25,
                    h: 0.25,
                },
            })
            .collect();
        let order = composite_order(&cams);
        assert_eq!(order, vec![0, 1, 2, 3]);
    }

    /// TC-2.7.3.3 — higher priority overlays lower.
    #[test]
    fn test_pip_priority_order_respected() {
        let low = PipCamera {
            id: 10,
            priority: -5,
            rect: PipRect {
                x: 0.0,
                y: 0.0,
                w: 1.0,
                h: 1.0,
            },
        };
        let high = PipCamera {
            id: 11,
            priority: 50,
            rect: PipRect {
                x: 0.2,
                y: 0.2,
                w: 0.6,
                h: 0.6,
            },
        };
        let order = composite_order(&[high, low]);
        assert_eq!(order, vec![10, 11]);
    }
}
