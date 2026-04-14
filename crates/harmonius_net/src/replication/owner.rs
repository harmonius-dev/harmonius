//! Owner-only visibility for replicated properties.

/// Returns `Some(value)` for the owning client, always `None` for observers.
pub fn owner_only_field<T: Clone>(is_owner: bool, value: &T) -> Option<T> {
    if is_owner {
        Some(value.clone())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.2.4.1 — non-owners do not observe owner-only fields.
    #[test]
    fn test_owner_only_property() {
        let inv = vec![1u8, 2, 3];
        assert_eq!(owner_only_field(true, &inv), Some(vec![1, 2, 3]));
        assert_eq!(owner_only_field(false, &inv), None);
    }
}
