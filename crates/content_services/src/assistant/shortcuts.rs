//! Shortcut recommendation heuristics from recent editor actions.

/// Surfaces a shortcut hint when the user intent matches a known pattern (TC-15.9.4.1).
pub fn recommend_shortcut(context: &str, last_actions: &[&str]) -> Option<String> {
    if context.to_lowercase().contains("select all") && last_actions.len() >= 3 {
        return Some("Try Ctrl+A / Cmd+A to select all entities".to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.9.4.1 — repetitive selection context yields a shortcut suggestion.
    #[test]
    fn tc_15_9_4_1_shortcut_recommendations() {
        let actions = ["pick", "pick", "pick", "pick", "pick"];
        let refs: Vec<&str> = actions.iter().copied().collect();
        let hint = recommend_shortcut("select all vertices in mesh", &refs);
        assert!(hint.is_some());
        let text = hint.unwrap();
        assert!(text.contains("Ctrl+A") || text.contains("Cmd+A"));
    }
}
