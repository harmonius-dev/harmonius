//! Idle-time contextual reminders for long-running tool sessions.

/// Emits a single reminder after the idle threshold on the paint tool (TC-15.9.5.1).
pub fn contextual_action_reminder(tool: &str, idle_secs: u64) -> Option<&'static str> {
    if tool == "paint" && idle_secs >= 60 {
        return Some("try symmetry mode");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.9.5.1 — long idle on the paint tool surfaces a symmetry hint once.
    #[test]
    fn tc_15_9_5_1_contextual_action_reminder() {
        let reminder = contextual_action_reminder("paint", 60);
        assert_eq!(reminder, Some("try symmetry mode"));
    }
}
