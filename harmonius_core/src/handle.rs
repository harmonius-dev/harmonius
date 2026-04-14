//! Generational [`Handle`] and dense [`HandleMap`] storage (F-1.7.4, F-1.7.5).

use core::marker::PhantomData;

/// Generational handle parameterized by the resource type `T`.
///
/// Two handles with the same index but different generations are not equal;
/// this detects use-after-free when slots are reused.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Handle<T: 'static> {
    /// Slot index inside the owning [`HandleMap`].
    pub index: u32,
    /// Generation counter for the slot at `index`.
    pub generation: u32,
    marker: PhantomData<fn() -> T>,
}

impl<T: 'static> core::fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Handle")
            .field("index", &self.index)
            .field("generation", &self.generation)
            .field("type", &core::any::type_name::<T>())
            .finish()
    }
}

impl<T: 'static> Handle<T> {
    /// Sentinel handle that never refers to live data.
    pub const NULL: Self = Self {
        index: u32::MAX,
        generation: u32::MAX,
        marker: PhantomData,
    };

    /// Returns `true` if this is [`Handle::NULL`].
    pub const fn is_null(&self) -> bool {
        self.index == u32::MAX && self.generation == u32::MAX
    }
}

struct Slot<T> {
    generation: u32,
    value: Option<T>,
}

/// Dense map keyed by [`Handle<T>`] with generational validation.
pub struct HandleMap<T> {
    slots: Vec<Slot<T>>,
    free: Vec<u32>,
    live: usize,
}

impl<T> core::fmt::Debug for HandleMap<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HandleMap")
            .field("len", &self.live)
            .field("capacity", &self.slots.len())
            .finish()
    }
}

impl<T> Default for HandleMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> HandleMap<T> {
    /// Creates an empty map.
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free: Vec::new(),
            live: 0,
        }
    }

    /// Creates an empty map with reserved capacity for at least `capacity` slots.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: Vec::with_capacity(capacity),
            free: Vec::with_capacity(capacity),
            live: 0,
        }
    }

    /// Returns the number of occupied slots.
    pub fn len(&self) -> usize {
        self.live
    }

    /// Returns `true` when no values are stored.
    pub fn is_empty(&self) -> bool {
        self.live == 0
    }

    /// Inserts `value` and returns a fresh [`Handle`].
    pub fn insert(&mut self, value: T) -> Handle<T> {
        let index = if let Some(i) = self.free.pop() {
            i
        } else {
            let i = u32::try_from(self.slots.len()).expect("slot index fits u32");
            self.slots.push(Slot {
                generation: 0,
                value: None,
            });
            i
        };

        let slot = &mut self.slots[index as usize];
        slot.value = Some(value);
        self.live += 1;

        Handle {
            index,
            generation: slot.generation,
            marker: PhantomData,
        }
    }

    /// Removes and returns the value for `handle` if it is still valid.
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        if handle.is_null() {
            return None;
        }

        let slot = self.slots.get_mut(handle.index as usize)?;
        if slot.generation != handle.generation || slot.value.is_none() {
            return None;
        }

        let value = slot.value.take()?;
        slot.generation = slot.generation.wrapping_add(1);
        self.free.push(handle.index);
        self.live -= 1;
        Some(value)
    }

    /// Borrows the value for `handle` when the generation matches.
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        if handle.is_null() {
            return None;
        }

        let slot = self.slots.get(handle.index as usize)?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.value.as_ref()
    }

    /// Mutably borrows the value for `handle` when the generation matches.
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        if handle.is_null() {
            return None;
        }

        let slot = self.slots.get_mut(handle.index as usize)?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.value.as_mut()
    }

    /// Iterates `(handle, value)` pairs for all occupied slots.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            slots: &self.slots,
            next: 0,
        }
    }

    /// Test-only: sets the generation counter for a free slot.
    ///
    /// # Panics
    ///
    /// Panics if the slot is still occupied or `index` is out of range.
    #[cfg(test)]
    pub fn test_set_slot_generation(&mut self, index: u32, generation: u32) {
        let slot = &mut self.slots[index as usize];
        assert!(
            slot.value.is_none(),
            "slot must be free before forcing generation"
        );
        slot.generation = generation;
    }
}

/// Iterator over live entries in a [`HandleMap`].
pub struct Iter<'a, T> {
    slots: &'a [Slot<T>],
    next: usize,
}

impl<'a, T> core::fmt::Debug for Iter<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Iter")
            .field(
                "remaining_slots",
                &(self.slots.len().saturating_sub(self.next)),
            )
            .finish()
    }
}

impl<'a, T: 'static> Iterator for Iter<'a, T> {
    type Item = (Handle<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.next < self.slots.len() {
            let i = self.next;
            self.next += 1;
            let slot = &self.slots[i];
            if let Some(value) = slot.value.as_ref() {
                let handle = Handle {
                    index: u32::try_from(i).ok()?,
                    generation: slot.generation,
                    marker: PhantomData,
                };
                return Some((handle, value));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::HandleMap;

    #[test]
    fn tc_1_7_4_1_handle_insert_get_remove() {
        let mut map = HandleMap::new();
        let h = map.insert(42);
        assert_eq!(h.index, 0);
        assert_eq!(h.generation, 0);
        assert_eq!(map.get(h), Some(&42));
        assert_eq!(map.remove(h), Some(42));
        assert_eq!(map.get(h), None);
    }

    #[test]
    fn tc_1_7_4_2_handle_generation_overflow_wraps() {
        let mut map = HandleMap::new();
        let h0 = map.insert(1_u32);
        map.remove(h0);
        map.test_set_slot_generation(0, u32::MAX);
        let h_max = map.insert(2_u32);
        assert_eq!(h_max.index, 0);
        assert_eq!(h_max.generation, u32::MAX);
        map.remove(h_max);
        let h_wrapped = map.insert(3_u32);
        assert_eq!(h_wrapped.index, 0);
        assert_eq!(h_wrapped.generation, 0);
        assert_eq!(map.get(h_wrapped), Some(&3));
    }

    #[test]
    fn tc_1_7_4_3_handle_iteration_skips_free() {
        let mut map = HandleMap::new();
        let mut handles = Vec::new();
        for v in 0..10 {
            handles.push(map.insert(v));
        }
        map.remove(handles[2]);
        map.remove(handles[5]);
        map.remove(handles[8]);
        assert_eq!(map.len(), 7);
        assert_eq!(map.iter().count(), 7);
    }
}
