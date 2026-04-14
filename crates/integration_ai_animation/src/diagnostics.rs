/// Collects log lines produced during deterministic simulation steps (tests and tooling).
#[derive(Debug, Default, Clone)]
pub struct Diagnostics {
    /// Warning-level messages (FM-1, FM-2).
    pub warnings: Vec<String>,
    /// Error-level messages (FM-3).
    pub errors: Vec<String>,
    /// Debug-level messages (FM-4).
    pub debug: Vec<String>,
}

impl Diagnostics {
    /// Records a warning string.
    pub fn warn(&mut self, message: impl Into<String>) {
        self.warnings.push(message.into());
    }

    /// Records an error string.
    pub fn error(&mut self, message: impl Into<String>) {
        self.errors.push(message.into());
    }

    /// Records a debug string.
    pub fn debug(&mut self, message: impl Into<String>) {
        self.debug.push(message.into());
    }
}
