//! UI subtree reload simulation.

/// One panel in a simple 3-panel layout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PanelState {
    /// Stable panel id (`A`, `B`, `C`).
    pub id: char,
    /// Widget identity token (changes on rebuild).
    pub widget_id: u32,
    /// Style fingerprint.
    pub style: u32,
}

/// Simple UI document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiDocument {
    /// Three panels.
    pub panels: [PanelState; 3],
}

impl UiDocument {
    /// Default A/B/C layout.
    pub fn default_three() -> Self {
        Self {
            panels: [
                PanelState {
                    id: 'A',
                    widget_id: 1,
                    style: 0,
                },
                PanelState {
                    id: 'B',
                    widget_id: 2,
                    style: 0,
                },
                PanelState {
                    id: 'C',
                    widget_id: 3,
                    style: 0,
                },
            ],
        }
    }
}

/// Bump style and widget id for one panel only.
pub fn reload_ui_panel_style(doc: &mut UiDocument, panel: char) {
    for p in &mut doc.panels {
        if p.id == panel {
            p.style = p.style.wrapping_add(1);
            p.widget_id = p.widget_id.wrapping_add(100);
        }
    }
}
