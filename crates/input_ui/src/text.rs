//! Text input routing and IME composition policies.

/// Policy for committing IME composition on focus loss.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FocusCommitPolicy {
    /// Commit composition text when focus leaves the widget.
    CommitOnBlur,
    /// Discard composition text when focus leaves the widget.
    CancelOnBlur,
}

/// Minimal text field model for routing tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextInput {
    /// Visible text buffer.
    pub text: String,
    /// Pending IME composition buffer.
    pub composition: String,
    /// Focus policy for composition.
    pub policy: FocusCommitPolicy,
}

impl TextInput {
    /// Creates an empty text input.
    pub fn new(policy: FocusCommitPolicy) -> Self {
        Self {
            text: String::new(),
            composition: String::new(),
            policy,
        }
    }

    /// Inserts a typed character while focused.
    pub fn insert_char(&mut self, ch: char) {
        self.text.push(ch);
    }

    /// Begins IME composition with the provided prefix.
    pub fn begin_composition(&mut self, prefix: &str) {
        self.composition.clear();
        self.composition.push_str(prefix);
    }

    /// Applies focus loss semantics for composition.
    pub fn on_blur(&mut self) {
        match self.policy {
            FocusCommitPolicy::CommitOnBlur => {
                self.text.push_str(&self.composition);
                self.composition.clear();
            }
            FocusCommitPolicy::CancelOnBlur => {
                self.composition.clear();
            }
        }
    }
}

/// Returns true when gameplay should observe a `W` / `w` key as `MoveForward`.
///
/// When a `TextInput` is focused and the active UI context consumes keyboard input, gameplay must
/// not observe the key.
pub fn gameplay_observes_move_forward_key(
    key: char,
    text_focused: bool,
    keyboard_consumed: bool,
) -> bool {
    if !matches!(key, 'w' | 'W') {
        return true;
    }
    !(text_focused && keyboard_consumed)
}
