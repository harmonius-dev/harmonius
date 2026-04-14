//! `StreamRequestTable` — mip/LOD streaming request arena (IR-5.2.7).

/// Generational handle into `StreamRequestTable`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct StreamHandle {
    pub index: u32,
    pub generation: u32,
}

/// I/O request state.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum StreamRequestState {
    Pending = 0,
    Ready = 1,
    Failed = 2,
}

/// Streaming I/O request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamRequest {
    pub state: StreamRequestState,
    pub priority: u8,
    pub request_id: u64,
    pub gpu_offset: u64,
    pub retries: u8,
}

/// Typed arena of streaming requests.
#[derive(Debug, Default)]
pub struct StreamRequestTable {
    slots: Vec<StreamRequest>,
    generations: Vec<u32>,
    next_request_id: u64,
}

impl StreamRequestTable {
    /// Creates an empty table.
    #[must_use]
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            generations: Vec::new(),
            next_request_id: 1,
        }
    }

    /// Submits a new pending request; returns a generational handle.
    pub fn submit(&mut self, priority: u8) -> StreamHandle {
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1);
        let request = StreamRequest {
            state: StreamRequestState::Pending,
            priority,
            request_id,
            gpu_offset: 0,
            retries: 0,
        };
        let index = self.slots.len() as u32;
        let generation = 0;
        self.slots.push(request);
        self.generations.push(generation);
        StreamHandle { index, generation }
    }

    /// Returns the current state for `handle`.
    #[must_use]
    pub fn poll(&self, handle: StreamHandle) -> StreamRequestState {
        let idx = handle.index as usize;
        match (self.slots.get(idx), self.generations.get(idx)) {
            (Some(request), Some(gen)) if *gen == handle.generation => request.state,
            _ => StreamRequestState::Failed,
        }
    }

    /// Marks a request ready after platform I/O completes.
    pub fn mark_ready(&mut self, handle: StreamHandle, gpu_offset: u64) {
        let idx = handle.index as usize;
        if let (Some(request), Some(gen)) = (self.slots.get_mut(idx), self.generations.get(idx)) {
            if *gen == handle.generation {
                request.state = StreamRequestState::Ready;
                request.gpu_offset = gpu_offset;
            }
        }
    }

    /// Marks a request failed after retries are exhausted.
    pub fn mark_failed(&mut self, handle: StreamHandle) {
        let idx = handle.index as usize;
        if let (Some(request), Some(gen)) = (self.slots.get_mut(idx), self.generations.get(idx)) {
            if *gen == handle.generation {
                request.state = StreamRequestState::Failed;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_5_2_7_u1_submit_returns_generation_zero() {
        let mut table = StreamRequestTable::new();
        let h = table.submit(5);
        assert_eq!(h.generation, 0);
        assert_eq!(table.poll(h), StreamRequestState::Pending);
    }

    #[test]
    fn tc_ir_5_2_7_u2_mark_ready_then_poll_is_ready() {
        let mut table = StreamRequestTable::new();
        let h = table.submit(1);
        table.mark_ready(h, 4096);
        assert_eq!(table.poll(h), StreamRequestState::Ready);
        assert_eq!(table.slots[h.index as usize].gpu_offset, 4096);
    }
}
