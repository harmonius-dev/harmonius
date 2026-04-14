//! Binary search helpers for sorted `KeyEntry` slices.

use crate::{KeyEntry, LocalizedStringId};

#[cfg(test)]
thread_local! {
    static BINARY_SEARCH_CALLS: std::cell::Cell<u32> =
        const { std::cell::Cell::new(0) };
}

#[cfg(test)]
/// Resets the instrumentation counter used by `TC-10.1.9.5`.
pub fn reset_binary_search_call_count() {
    BINARY_SEARCH_CALLS.with(|c| c.set(0));
}

#[cfg(test)]
/// Returns the number of entry comparisons performed by the last searches on this thread.
pub fn binary_search_call_count() -> u32 {
    BINARY_SEARCH_CALLS.with(|c| c.get())
}

#[cfg(test)]
fn record_call() {
    BINARY_SEARCH_CALLS.with(|c| c.set(c.get().saturating_add(1)));
}

#[cfg(not(test))]
fn record_call() {}

/// Returns the index of `id` in `keys`, which must be sorted by `id` ascending.
///
/// Table loads reject archives whose ids are not strictly ascending.
#[must_use]
pub fn binary_search_entry(keys: &[KeyEntry], id: LocalizedStringId) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = keys.len();
    while lo < hi {
        record_call();
        let mid = lo + (hi - lo) / 2;
        match keys[mid].id.cmp(&id) {
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

/// Returns the UTF-8 substring of `blob` at `[offset, offset + length)` when in range.
#[must_use]
pub fn slice_str(blob: &str, offset: u32, length: u32) -> Option<&str> {
    let o = offset as usize;
    let l = length as usize;
    blob.get(o..o.checked_add(l)?)
}
