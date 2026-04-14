use crate::types::DebugLine;

/// Owned per-frame buffer of debug lines with overflow accounting.
#[derive(Clone, Debug, PartialEq)]
pub struct DebugDrawBuffer {
    pub lines: Vec<DebugLine>,
    pub capacity: u32,
    pub dropped: u32,
}

impl DebugDrawBuffer {
    pub fn new(capacity: u32) -> Self {
        Self {
            lines: Vec::new(),
            capacity,
            dropped: 0,
        }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.dropped = 0;
    }

    /// Pushes a line when under `max_lines`; otherwise increments `dropped`.
    pub fn push_line(&mut self, max_lines: u32, line: DebugLine) {
        if self.lines.len() >= max_lines as usize {
            self.dropped = self.dropped.saturating_add(1);
            return;
        }
        self.lines.push(line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LinearColor;
    use glam::Vec3;

    #[test]
    fn tc_ir_3_4_n1_line_budget_overflow() {
        let mut buf = DebugDrawBuffer::new(16);
        let line = DebugLine {
            start: Vec3::ZERO,
            end: Vec3::X,
            color: LinearColor::new(1.0, 1.0, 1.0, 1.0),
        };
        for _ in 0..4 {
            buf.push_line(3, line);
        }
        assert_eq!(buf.lines.len(), 3);
        assert_eq!(buf.dropped, 1);
    }

    #[test]
    fn clear_resets_dropped_and_lines() {
        let mut buf = DebugDrawBuffer::new(16);
        let line = DebugLine {
            start: Vec3::ZERO,
            end: Vec3::X,
            color: LinearColor::new(1.0, 1.0, 1.0, 1.0),
        };
        buf.push_line(1, line);
        assert_eq!(buf.dropped, 0);
        buf.push_line(1, line);
        assert_eq!(buf.dropped, 1);
        buf.clear();
        assert!(buf.lines.is_empty());
        assert_eq!(buf.dropped, 0);
    }
}
