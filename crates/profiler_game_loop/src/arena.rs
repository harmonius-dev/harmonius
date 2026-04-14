/// Bump allocator backing hot-path CPU event aggregation.
#[derive(Debug)]
pub struct FrameArena {
    storage: Vec<u8>,
    cursor: usize,
    capacity: usize,
}

impl FrameArena {
    /// Creates an arena with the given usable byte capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            storage: vec![0; capacity],
            cursor: 0,
            capacity,
        }
    }

    /// Resets the bump cursor for a new frame.
    pub fn reset(&mut self) {
        self.cursor = 0;
    }

    /// Returns remaining usable bytes.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.capacity.saturating_sub(self.cursor)
    }

    /// Allocates a zeroed slice of `len` `T` items or returns `None` on overflow.
    pub fn alloc_slice<T: Copy>(&mut self, len: usize) -> Option<&mut [T]> {
        let size = core::mem::size_of::<T>().checked_mul(len)?;
        let align = core::mem::align_of::<T>();
        let aligned = align_up(self.cursor, align)?;
        let end = aligned.checked_add(size)?;
        if end > self.capacity {
            return None;
        }
        let ptr = self.storage[aligned..end].as_mut_ptr().cast::<T>();
        // SAFETY: `storage` is `len * size` bytes of zeroed memory with `T` alignment.
        unsafe {
            core::ptr::write_bytes(ptr, 0, len);
        }
        self.cursor = end;
        // SAFETY: region is valid for `len` elements of `T` and uniquely borrowed.
        Some(unsafe { core::slice::from_raw_parts_mut(ptr, len) })
    }
}

fn align_up(value: usize, align: usize) -> Option<usize> {
    if align == 0 {
        return Some(value);
    }
    let mis = value % align;
    if mis == 0 {
        Some(value)
    } else {
        value.checked_add(align - mis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_5_6_4_u2_reset_returns_slice_at_base() {
        let mut arena = FrameArena::with_capacity(4096);
        let first = arena.alloc_slice::<u32>(4).expect("alloc");
        first.fill(1);
        arena.reset();
        assert_eq!(arena.remaining(), 4096);
        let second = arena.alloc_slice::<u32>(4).expect("alloc after reset");
        assert!(second.iter().all(|v| *v == 0));
    }
}
