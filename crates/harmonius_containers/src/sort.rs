//! In-place container sorting helpers.

/// Item rarity used by `SortCriterion::Rarity`.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Rarity {
    /// Lowest rarity tier.
    Common,
    /// Mid rarity tier.
    Rare,
    /// Highest rarity tier.
    Legendary,
}

/// Sort axis requested by gameplay or UI.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortCriterion {
    /// Lexicographic name ordering.
    Name,
    /// Total weight ordering.
    Weight,
    /// Rarity tier ordering.
    Rarity,
}

/// Ascending or descending comparator direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortOrder {
    /// Smallest / weakest first.
    Asc,
    /// Largest / strongest first.
    Desc,
}
