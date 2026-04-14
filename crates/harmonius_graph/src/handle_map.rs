use std::fmt;

/// Stable identifier for a node slot inside a [`super::HandleMap`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId {
    slot: u32,
    generation: u32,
}

impl NodeId {
    pub(crate) fn new(slot: u32, generation: u32) -> Self {
        Self { slot, generation }
    }

    /// Slot index inside the owning map (for deterministic ordering).
    #[must_use]
    pub fn slot(self) -> u32 {
        self.slot
    }

    /// Generation counter for stale-handle detection.
    #[must_use]
    pub fn generation(self) -> u32 {
        self.generation
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N{}:{}", self.slot, self.generation)
    }
}

/// Dense generational storage keyed by [`NodeId`].
#[derive(Debug, Clone)]
pub struct HandleMap<T> {
    slots: Vec<Option<(u32, T)>>,
    generations: Vec<u32>,
    free: Vec<u32>,
    len: usize,
}

impl<T> Default for HandleMap<T> {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            generations: Vec::new(),
            free: Vec::new(),
            len: 0,
        }
    }
}

impl<T> HandleMap<T> {
    /// Creates an empty map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of live entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` when no entries are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Inserts `value` and returns a fresh [`NodeId`].
    pub fn insert(&mut self, value: T) -> NodeId {
        if let Some(slot) = self.free.pop() {
            let generation = self.generations[slot as usize];
            self.slots[slot as usize] = Some((generation, value));
            self.len += 1;
            return NodeId::new(slot, generation);
        }
        let slot = u32::try_from(self.slots.len()).expect("slot index overflow");
        self.slots.push(Some((0, value)));
        self.generations.push(0);
        self.len += 1;
        NodeId::new(slot, 0)
    }

    /// Removes a node by id, returning its payload when the id is current.
    pub fn remove(&mut self, id: NodeId) -> Option<T> {
        let slot = id.slot as usize;
        let entry = self.slots.get_mut(slot)?;
        let (generation, _) = entry.as_ref()?;
        if *generation != id.generation {
            return None;
        }
        let (_, value) = entry.take()?;
        self.generations[slot] = self.generations[slot].wrapping_add(1);
        if self.generations[slot] == 0 {
            self.generations[slot] = 1;
        }
        self.free.push(id.slot);
        self.len -= 1;
        Some(value)
    }

    /// Borrows a live entry.
    #[must_use]
    pub fn get(&self, id: NodeId) -> Option<&T> {
        let slot = id.slot as usize;
        let (generation, value) = self.slots.get(slot)?.as_ref()?;
        (*generation == id.generation).then_some(value)
    }

    /// Mutable borrow of a live entry.
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut T> {
        let slot = id.slot as usize;
        let (generation, value) = self.slots.get_mut(slot)?.as_mut()?;
        (*generation == id.generation).then_some(value)
    }

    /// Iterator over all valid [`NodeId`] handles and their values.
    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &T)> + '_ {
        self.slots.iter().enumerate().filter_map(|(slot, cell)| {
            let (generation, value) = cell.as_ref()?;
            Some((NodeId::new(slot as u32, *generation), value))
        })
    }
}
