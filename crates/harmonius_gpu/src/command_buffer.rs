//! Recorded command buffer operations for multi-queue submission tests (R-2.1.2).

/// Kind of operation recorded on a fake command buffer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CmdKind {
    /// Draw call placeholder.
    Draw,
    /// Compute dispatch.
    Dispatch {
        /// X groups.
        x: u32,
        /// Y groups.
        y: u32,
        /// Z groups.
        z: u32,
    },
    /// Buffer copy.
    CopyBuffer,
}

/// Fake command buffer recording draw, dispatch, and copy in order.
#[derive(Debug, Default)]
pub struct FakeCommandBuffer {
    ops: Vec<CmdKind>,
}

impl FakeCommandBuffer {
    /// Empty buffer.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a draw.
    pub fn draw(&mut self) {
        self.ops.push(CmdKind::Draw);
    }

    /// Records a compute dispatch.
    pub fn dispatch(&mut self, x: u32, y: u32, z: u32) {
        self.ops.push(CmdKind::Dispatch { x, y, z });
    }

    /// Records a copy.
    pub fn copy_buffer(&mut self) {
        self.ops.push(CmdKind::CopyBuffer);
    }

    /// Recorded operations in submission order.
    #[must_use]
    pub fn ops(&self) -> &[CmdKind] {
        &self.ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-2.1.2.3 — one buffer records graphics, compute, and copy in order.
    #[test]
    fn test_cmd_buf_graphics_compute_copy() {
        let mut cb = FakeCommandBuffer::new();
        cb.draw();
        cb.dispatch(4, 4, 1);
        cb.copy_buffer();
        assert_eq!(
            cb.ops(),
            &[
                CmdKind::Draw,
                CmdKind::Dispatch { x: 4, y: 4, z: 1 },
                CmdKind::CopyBuffer
            ]
        );
    }
}
