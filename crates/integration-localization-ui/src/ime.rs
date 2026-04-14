//! IME composition model for `TextInput` integration tests.

/// IME lifecycle kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImeEventKind {
    /// Composition session started.
    Start,
    /// Composition text updated.
    Update,
    /// Composition committed into the widget.
    Commit,
    /// Composition cancelled.
    Cancel,
}

/// IME event payload (subset of platform IME fields used by tests).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImeEvent {
    /// Event kind.
    pub kind: ImeEventKind,
    /// Composition or commit text.
    pub text: String,
    /// Selection start (unused in minimal model).
    pub selection_start: u32,
    /// Selection end (unused in minimal model).
    pub selection_end: u32,
}

/// Minimal `TextInput` state machine for composition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextInputState {
    /// Committed text.
    pub text: String,
    /// Active composition buffer.
    pub composition: Option<String>,
    /// Whether composition should render with underline styling.
    pub composition_underline: bool,
}

impl TextInputState {
    /// Empty input.
    #[must_use]
    pub fn new() -> Self {
        Self {
            text: String::new(),
            composition: None,
            composition_underline: false,
        }
    }

    /// Applies `event` while `focused` is true for the widget, bumping `fm4` on discarded commits.
    pub fn apply_ime(&mut self, event: &ImeEvent, focused: bool, fm4: &mut u32) {
        match event.kind {
            ImeEventKind::Start => {
                if focused {
                    self.composition = Some(event.text.clone());
                    self.composition_underline = true;
                }
            }
            ImeEventKind::Update => {
                if focused {
                    self.composition = Some(event.text.clone());
                    self.composition_underline = true;
                }
            }
            ImeEventKind::Commit => {
                if !focused {
                    *fm4 += 1;
                    return;
                }
                self.text.push_str(&event.text);
                self.composition = None;
                self.composition_underline = false;
            }
            ImeEventKind::Cancel => {
                self.composition = None;
                self.composition_underline = false;
            }
        }
    }
}

impl Default for TextInputState {
    fn default() -> Self {
        Self::new()
    }
}
