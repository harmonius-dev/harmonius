//! Barrier batching and elision (GR-4.4, R-2.1.9).

/// Simplified resource state for barrier tests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceState {
    /// Initial / undefined.
    Undefined,
    /// Shader-readable.
    ShaderRead,
    /// Render target writable.
    RenderTarget,
}

/// Emits backend barriers only on real transitions.
#[derive(Debug, Default)]
pub struct BarrierOptimizer {
    resource_barrier_calls: u32,
}

impl BarrierOptimizer {
    /// Creates an optimizer with no prior state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests a transition to `to`; elides if already satisfied.
    pub fn request_transition(&mut self, from: ResourceState, to: ResourceState) {
        if from != to {
            self.resource_barrier_calls = self.resource_barrier_calls.saturating_add(1);
        }
    }

    /// Recorded backend `resource_barrier` calls.
    #[must_use]
    pub fn resource_barrier_calls(&self) -> u32 {
        self.resource_barrier_calls
    }
}

/// Collapses a chain of transitions when `to[n] == from[n+1]` (R-2.1.9).
#[must_use]
pub fn merge_barrier_chain(
    mut chain: Vec<(ResourceState, ResourceState)>,
) -> Vec<(ResourceState, ResourceState)> {
    if chain.is_empty() {
        return chain;
    }
    let mut out: Vec<(ResourceState, ResourceState)> = Vec::new();
    for (from, to) in chain.drain(..) {
        if from == to {
            continue;
        }
        if let Some(last) = out.last_mut() {
            if last.1 == from {
                last.1 = to;
                continue;
            }
        }
        out.push((from, to));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-2.1.4.1 / GR-4.4 — same-state barrier request is elided.
    #[test]
    fn test_barrier_elide_same_state() {
        let mut b = BarrierOptimizer::new();
        b.request_transition(ResourceState::ShaderRead, ResourceState::ShaderRead);
        assert_eq!(b.resource_barrier_calls(), 0);
    }

    /// TC-2.1.9.1 — chained compatible transitions merge.
    #[test]
    fn test_barrier_batching_merge() {
        let raw = vec![
            (ResourceState::Undefined, ResourceState::RenderTarget),
            (ResourceState::RenderTarget, ResourceState::ShaderRead),
        ];
        let merged = merge_barrier_chain(raw);
        assert_eq!(merged.len(), 1);
        assert_eq!(
            merged[0],
            (ResourceState::Undefined, ResourceState::ShaderRead)
        );
    }
}
