//! Flat stat modifiers applied while items are socketed.
//!
//! `StatModifier` is a minimal numeric stand-in; authored stat ids and modifier ops from the
//! subsystem design integrate in later plans.

/// Authoring-time base attributes plus runtime socket bonuses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttributeSet {
    /// Base strength before socket modifiers.
    pub strength_base: i32,
    /// Flat additive strength from socketed items.
    pub strength_bonus: i32,
}

/// Simple flat-add modifier keyed by stat name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatModifier {
    /// Stat identifier (for example `"strength"`).
    pub stat: &'static str,
    /// Flat amount to add while active.
    pub flat_add: i32,
}

/// Reads effective strength including socket bonuses.
#[must_use]
pub fn read_strength(attrs: &AttributeSet) -> i32 {
    attrs.strength_base + attrs.strength_bonus
}

/// Applies gem modifiers to the parent attribute set.
pub fn apply_socket_modifiers(attrs: &mut AttributeSet, mods: &[StatModifier]) {
    for modifier in mods {
        if modifier.stat == "strength" {
            attrs.strength_bonus += modifier.flat_add;
        }
    }
}

/// Removes previously applied gem modifiers.
pub fn clear_socket_modifiers(attrs: &mut AttributeSet, mods: &[StatModifier]) {
    for modifier in mods {
        if modifier.stat == "strength" {
            attrs.strength_bonus -= modifier.flat_add;
        }
    }
}
