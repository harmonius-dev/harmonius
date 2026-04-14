//! Editor panel visibility bookkeeping for LLM-driven UI actions.

/// Records which logical panel is focused after scripted opens.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EditorPanelState {
    focused: Option<String>,
    opened: Vec<String>,
}

impl EditorPanelState {
    /// Empty desktop state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks `panel` as the focused surface (TC-15.9.3.1).
    pub fn open_panel(&mut self, panel: &str) {
        self.opened.push(panel.to_string());
        self.focused = Some(panel.to_string());
    }

    /// Returns the focused panel id when any panel is active.
    pub fn focused_panel(&self) -> Option<&str> {
        self.focused.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.9.3.1 — opening a panel records focus for the node graph surface.
    #[test]
    fn tc_15_9_3_1_visual_tool_palette_access() {
        let mut state = EditorPanelState::new();
        state.open_panel("NodeGraph");
        assert_eq!(state.focused_panel(), Some("NodeGraph"));
    }
}
