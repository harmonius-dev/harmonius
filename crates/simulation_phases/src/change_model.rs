//! Minimal change-visibility rules at simulation set boundaries.

use crate::SimSet;

/// Whether a reader in `reader` should observe primitive writes performed in `writer` during the
/// same simulation tick.
#[must_use]
pub fn change_visible_across_sets(writer: SimSet, reader: SimSet) -> bool {
    writer.ordinal() < reader.ordinal()
}

/// Whether a follow-up system in the same set should treat the write as a fresh external change.
///
/// Harmonius guarantees downstream `Changed<T>` semantics only across set boundaries; systems
/// scheduled after a producer in the same set do not see that write as a `Changed` edge for this
/// primitive.
#[must_use]
pub fn change_visible_same_set_followup(_writer: SimSet, _reader: SimSet) -> bool {
    false
}
