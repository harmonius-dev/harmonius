use std::time::Instant;

static ANCHOR: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

/// Returns a monotonic timestamp suitable for ordering CPU events.
#[must_use]
pub fn monotonic_stamp() -> u64 {
    let anchor = *ANCHOR.get_or_init(Instant::now);
    anchor.elapsed().as_nanos() as u64
}
