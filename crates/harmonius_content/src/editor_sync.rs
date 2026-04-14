//! Bidirectional editor/runtime sync (test harness).

use std::collections::VecDeque;

/// Editor → runtime messages.
#[derive(Clone, Debug, PartialEq)]
pub enum EditorMessage {
    /// Set scalar material parameter.
    SetMaterialParam {
        /// Material slot id.
        material: u32,
        /// Parameter name.
        param: String,
        /// RGBA linear.
        value: [f32; 4],
    },
}

/// Runtime material table (single material for tests).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MaterialRuntime {
    /// Albedo RGBA for material 0 tests.
    pub albedo: [f32; 4],
}

/// Queued sync channel.
#[derive(Debug, Default)]
pub struct EditorSync {
    q: VecDeque<EditorMessage>,
}

impl EditorSync {
    /// New empty sync.
    pub fn new() -> Self {
        Self::default()
    }

    /// Editor pushes a message.
    pub fn send_editor(&mut self, m: EditorMessage) {
        self.q.push_back(m);
    }

    /// Runtime drain (one tick).
    pub fn poll(&mut self, rt: &mut MaterialRuntime) {
        while let Some(m) = self.q.pop_front() {
            let EditorMessage::SetMaterialParam { param, value, .. } = m;
            if param == "albedo" {
                rt.albedo = value;
            }
        }
    }
}
