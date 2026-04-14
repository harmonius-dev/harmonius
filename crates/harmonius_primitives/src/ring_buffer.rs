//! Fixed-capacity ring buffer.

use core::mem::MaybeUninit;

/// Fixed-capacity inline ring buffer.
#[derive(Debug)]
pub struct RingBuffer<T, const N: usize> {
    buf: [MaybeUninit<T>; N],
    head: u32,
    tail: u32,
    len: u32,
}

impl<T, const N: usize> RingBuffer<T, N> {
    /// Creates an empty buffer.
    #[must_use]
    #[allow(clippy::new_without_default)] // `N == 0` is rejected at runtime; `Default` would share the same panic.
    pub fn new() -> Self {
        assert!(N > 0, "RingBuffer capacity must be non-zero");
        Self {
            buf: core::array::from_fn(|_| MaybeUninit::uninit()),
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    /// Pushes `value` when capacity remains.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if usize::try_from(self.len).ok() == Some(N) {
            return Err(value);
        }

        let tail = self.tail as usize;
        self.buf[tail].write(value);
        self.tail = ((self.tail as usize + 1) % N) as u32;
        self.len += 1;
        Ok(())
    }

    /// Pops the oldest value, if any.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let head = self.head as usize;
        // SAFETY: `len > 0` guarantees `buf[head]` is initialized.
        let value = unsafe { self.buf[head].assume_init_read() };
        self.head = ((head + 1) % N) as u32;
        self.len -= 1;
        Some(value)
    }

    /// Returns the number of stored elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns `true` when no elements are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns `true` when no additional elements can be stored.
    #[must_use]
    pub fn is_full(&self) -> bool {
        usize::try_from(self.len).ok() == Some(N)
    }
}

impl<T, const N: usize> Drop for RingBuffer<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
