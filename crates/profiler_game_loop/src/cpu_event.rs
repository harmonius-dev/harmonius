/// One CPU timing interval recorded by [`crate::CpuScopeGuard`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CpuEvent {
    /// Timestamp at scope entry.
    pub begin_tsc: u64,
    /// Timestamp at scope exit.
    pub end_tsc: u64,
    /// Stable identifier for the originating thread.
    pub thread_id: u64,
    /// FNV-1a 32-bit hash of the zone label.
    pub zone_name_hash: u32,
    /// Nesting depth at emission time.
    pub depth: u16,
    /// Bit flags (for example non-monotonic clamp).
    pub flags: u16,
}

impl CpuEvent {
    /// Bit set when `end_tsc < begin_tsc` was clamped.
    pub const FLAG_NON_MONOTONIC: u16 = 1;

    /// Sentinel for an incomplete event still on the pending stack.
    pub const INCOMPLETE: u64 = 0;

    /// Builds an incomplete event (end timestamp not yet known).
    pub const fn pending(begin_tsc: u64, thread_id: u64, zone_name_hash: u32, depth: u16) -> Self {
        Self {
            begin_tsc,
            end_tsc: Self::INCOMPLETE,
            thread_id,
            zone_name_hash,
            depth,
            flags: 0,
        }
    }

    /// Returns `true` when the event is still pending completion.
    pub const fn is_pending(self) -> bool {
        self.end_tsc == Self::INCOMPLETE
    }
}
