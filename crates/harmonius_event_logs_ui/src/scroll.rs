//! Scroll view model for combat log auto-scroll (`IR-2.10.6`).

use smol_str::SmolStr;

use crate::types::RichText;

/// Epsilon for “user is scrolled to bottom” detection in logical pixels.
pub const AUTO_SCROLL_EPSILON: f32 = 1.0;

/// Vertical log scroll region with resolved [`RichText`] children.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollView {
    /// Vertical scroll offset (positive moves content up).
    pub scroll_offset_y: f32,
    /// Viewport height in logical pixels.
    pub viewport_height: f32,
    /// Total content height in logical pixels.
    pub content_height: f32,
    /// Rendered lines from oldest at the front to newest at the back.
    pub lines: Vec<RichText>,
}

impl ScrollView {
    /// Creates an empty scroll view with the given viewport height.
    pub fn new(viewport_height: f32) -> Self {
        Self {
            scroll_offset_y: 0.0,
            viewport_height,
            content_height: 0.0,
            lines: Vec::new(),
        }
    }

    /// Maximum scroll offset (non-negative).
    pub fn scroll_max(&self) -> f32 {
        (self.content_height - self.viewport_height).max(0.0)
    }

    /// Returns true when the viewport is anchored within [`AUTO_SCROLL_EPSILON`] of the bottom.
    pub fn is_scrolled_to_bottom(&self) -> bool {
        let smax = self.scroll_max();
        self.scroll_offset_y + AUTO_SCROLL_EPSILON >= smax
    }

    /// Recomputes content height from line count using a fixed line height model.
    pub fn set_line_metrics(&mut self, line_count: usize, line_height: f32) {
        self.content_height = line_count as f32 * line_height;
    }

    /// Appends a resolved line and optionally pins scroll to the newest row.
    pub fn append_line(&mut self, line: RichText, line_height: f32, auto_follow: bool) {
        self.lines.push(line);
        self.set_line_metrics(self.lines.len(), line_height);
        if auto_follow {
            self.scroll_offset_y = self.scroll_max();
        }
    }

    /// Clears structural children (used when the backing log entity disappears).
    pub fn clear_lines(&mut self) {
        self.lines.clear();
        self.content_height = 0.0;
        self.scroll_offset_y = 0.0;
    }

    /// Simulates “user scrolled up” by pinning offset away from the bottom.
    pub fn scroll_up(&mut self, delta: f32) {
        self.scroll_offset_y = (self.scroll_offset_y - delta).clamp(0.0, self.scroll_max());
    }

    /// Builds a single debug line for negative tests (no `SmolStr` field on [`CombatLogLine`]).
    pub fn placeholder_no_entries_text() -> SmolStr {
        SmolStr::new_inline("no entries")
    }
}
