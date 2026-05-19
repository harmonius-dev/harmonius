//! GPU runtime services (memory, state, barriers) — bootstrap stubs.

#![deny(clippy::all)]

pub mod barrier;
pub mod memory;
pub mod state;

#[cfg(test)]
mod tests {
    #[test]
    fn test_crate_links() {
        assert!(true);
    }
}
