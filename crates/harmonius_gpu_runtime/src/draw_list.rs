//! Minimal draw list representation used for per-view profiling statistics.

/// One instanced draw command in a view-local list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrawCommand {
    /// Number of triangles submitted by this command.
    pub triangles: u32,
}

/// High-level render graph phase bucket for attribution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RenderPhase {
    /// Shadow map generation.
    Shadow,
    /// Depth prepass.
    Depth,
    /// Deferred G-buffer fill.
    GBuffer,
    /// Lighting passes.
    Light,
    /// Transparent geometry.
    Transparent,
    /// Post processing stack.
    PostProcess,
    /// UI overlay.
    Ui,
    /// Swapchain presentation preparation.
    Present,
}

/// Per-view draw list built on worker threads during snapshotting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrawList {
    /// Stable view identifier (camera / split-screen slot).
    pub view_id: u32,
    /// Render graph phase for this list.
    pub phase: RenderPhase,
    /// Ordered draw commands for this view.
    pub commands: Vec<DrawCommand>,
}

impl DrawList {
    /// Number of draw commands in this list.
    #[must_use]
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Returns `true` when the draw list contains no commands.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Total triangle count across all commands.
    #[must_use]
    pub fn triangle_count(&self) -> u32 {
        self.commands.iter().map(|c| c.triangles).sum()
    }

    /// Snapshot of counts used for profiler ingestion.
    #[must_use]
    pub fn stats(&self) -> DrawListStats {
        DrawListStats {
            view_id: self.view_id,
            phase: self.phase,
            draw_calls: self.len() as u32,
            triangles: self.triangle_count(),
        }
    }
}

/// Per-view counts attached to a [`GpuFrameCapture`](crate::GpuFrameCapture).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrawListStats {
    /// View identifier matching [`DrawList::view_id`](DrawList::view_id).
    pub view_id: u32,
    /// Phase attribution for this view.
    pub phase: RenderPhase,
    /// Number of draw calls in the view.
    pub draw_calls: u32,
    /// Total triangles submitted for the view.
    pub triangles: u32,
}

#[cfg(test)]
mod tests {
    use super::{DrawCommand, DrawList, RenderPhase};

    #[test]
    fn tc_ir_5_7_2_u1_draw_list_stats_sums() {
        let list = DrawList {
            view_id: 7,
            phase: RenderPhase::GBuffer,
            commands: vec![DrawCommand { triangles: 30 }; 10],
        };
        let stats = list.stats();
        assert_eq!(stats.draw_calls, 10);
        assert_eq!(stats.triangles, 300);
        assert_eq!(stats.view_id, 7);
        assert_eq!(stats.phase, RenderPhase::GBuffer);
    }
}
