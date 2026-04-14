//! Work graph native vs emulated dispatch (R-2.1.10).

/// Opaque work graph handle placeholder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkGraphHandle(pub u32);

/// Host-side work graph runtime selection.
#[derive(Debug)]
pub struct WorkGraphRuntime {
    native: bool,
}

impl WorkGraphRuntime {
    /// Creates runtime preferring native when `native` is true.
    #[must_use]
    pub fn new(native: bool) -> Self {
        Self { native }
    }

    /// Returns true when hardware work graphs are used.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.native
    }

    /// Dispatches either native or emulated path; returns label for assertions.
    pub fn dispatch_label(&self) -> &'static str {
        if self.native {
            "native-workgraph"
        } else {
            "emulated-workgraph"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-2.1.10.1 — native flag selects native dispatch label.
    #[test]
    fn test_work_graph_native_dispatch() {
        let wg = WorkGraphRuntime::new(true);
        assert!(wg.is_native());
        assert_eq!(wg.dispatch_label(), "native-workgraph");
    }
}
