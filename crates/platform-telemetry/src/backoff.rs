//! Exponential backoff for upload retries (R-14.5.4).

/// Simple bounded exponential backoff tracker in milliseconds.
#[derive(Debug, Clone)]
pub struct ExponentialBackoff {
    current_ms: u64,
    factor: u64,
    max_ms: u64,
}

impl ExponentialBackoff {
    /// Create backoff starting at `initial_ms`, doubling each step up to `max_ms`.
    pub fn new(initial_ms: u64, max_ms: u64) -> Self {
        Self {
            current_ms: initial_ms,
            factor: 2,
            max_ms,
        }
    }

    /// Next delay to wait before a retry.
    pub fn next_delay_ms(&mut self) -> u64 {
        let out = self.current_ms;
        self.current_ms = (self.current_ms.saturating_mul(self.factor)).min(self.max_ms);
        out
    }

    /// Reset to the initial window after a successful upload.
    pub fn reset(&mut self, initial_ms: u64) {
        self.current_ms = initial_ms;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_uses_exponential_backoff() {
        let mut b = ExponentialBackoff::new(100, 500);
        assert_eq!(b.next_delay_ms(), 100);
        assert_eq!(b.next_delay_ms(), 200);
        assert_eq!(b.next_delay_ms(), 400);
        assert_eq!(b.next_delay_ms(), 500);
        assert_eq!(b.next_delay_ms(), 500);
    }
}
