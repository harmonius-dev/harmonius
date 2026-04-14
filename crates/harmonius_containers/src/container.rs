//! Linear list containers with stacking and metadata used by sorting.

use crate::entity::Entity;
use crate::sort::{Rarity, SortCriterion, SortOrder};
use crate::tags::TagSet;
use crate::transfer::TransferError;

/// Authoring-time limits for a linear container.
#[derive(Clone, Debug)]
pub struct ContainerDef {
    /// Maximum occupied slots.
    pub capacity: u16,
    /// Maximum total weight in kilograms.
    pub weight_limit: f32,
    /// Optional tag gate applied on insert (empty means no gate).
    pub required_tags: TagSet,
}

/// Runtime slot contents for list-mode containers.
#[derive(Clone, Debug, PartialEq)]
pub enum SlotEntry {
    /// Slot is empty.
    Empty,
    /// Single non-stackable occupant.
    Item {
        /// Item entity id.
        entity: Entity,
        /// Total weight contribution.
        weight: f32,
        /// Display name for lexicographic sorts.
        name: String,
        /// Rarity tier for rarity sorts.
        rarity: Rarity,
        /// Tags for validation and sockets.
        tags: TagSet,
    },
    /// Stackable occupant.
    Stack {
        /// Stack kind id (item archetype).
        kind: Entity,
        /// Stack height.
        count: u32,
        /// Per-type stack cap.
        max_stack: u32,
        /// Weight per stacked unit.
        per_unit_weight: f32,
        /// Display name for lexicographic sorts.
        name: String,
        /// Rarity tier for rarity sorts.
        rarity: Rarity,
        /// Tags for validation and sockets.
        tags: TagSet,
    },
}

/// Linear container with fixed slot capacity and optional weight limit.
#[derive(Clone, Debug)]
pub struct Container {
    def: ContainerDef,
    slots: Vec<SlotEntry>,
    current_weight: f32,
}

impl Container {
    /// Creates an empty container from a definition.
    #[must_use]
    pub fn new(def: ContainerDef) -> Self {
        let len = usize::from(def.capacity);
        Self {
            def,
            slots: vec![SlotEntry::Empty; len],
            current_weight: 0.0,
        }
    }

    /// Number of occupied slots (stacks count as one slot).
    #[must_use]
    pub fn len(&self) -> usize {
        self.slots
            .iter()
            .filter(|slot| !matches!(slot, SlotEntry::Empty))
            .count()
    }

    /// Returns true when no slot is occupied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Maximum slot count from the definition.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    /// Current aggregate weight.
    #[must_use]
    pub fn total_weight(&self) -> f32 {
        self.current_weight
    }

    /// Slot slice for tests and tooling (mutable).
    #[must_use]
    pub fn slots(&self) -> &[SlotEntry] {
        &self.slots
    }

    /// Mutable slot slice for sort helpers.
    pub fn slots_mut(&mut self) -> &mut [SlotEntry] {
        &mut self.slots
    }

    /// Definition reference.
    #[must_use]
    pub fn def(&self) -> &ContainerDef {
        &self.def
    }

    /// Inserts using stack merge rules and capacity checks.
    pub fn insert(&mut self, stack: ItemStack) -> Result<(), TransferError> {
        if !self.def.required_tags.0.is_empty() {
            let req: Vec<String> = self.def.required_tags.to_vec();
            let prov = stack.tags().to_vec();
            if !stack.tags().contains_all(&req) {
                return Err(TransferError::TagMismatch {
                    required: req,
                    provided: prov,
                });
            }
        }

        match stack {
            ItemStack::Single {
                entity,
                weight,
                name,
                rarity,
                tags,
            } => self.insert_single(entity, weight, name, rarity, tags),
            ItemStack::Stack {
                kind,
                count,
                max_stack,
                per_unit_weight,
                name,
                rarity,
                tags,
            } => self.insert_stack(kind, count, max_stack, per_unit_weight, name, rarity, tags),
        }
    }

    fn insert_single(
        &mut self,
        entity: Entity,
        weight: f32,
        name: String,
        rarity: Rarity,
        tags: TagSet,
    ) -> Result<(), TransferError> {
        if self.len() >= self.capacity() {
            return Err(TransferError::ContainerFull);
        }
        let attempted = self.current_weight + weight;
        if attempted > self.def.weight_limit {
            return Err(TransferError::OverWeight {
                limit: self.def.weight_limit,
                attempted,
            });
        }
        let idx = self
            .slots
            .iter()
            .position(|slot| matches!(slot, SlotEntry::Empty))
            .ok_or(TransferError::ContainerFull)?;
        self.slots[idx] = SlotEntry::Item {
            entity,
            weight,
            name,
            rarity,
            tags,
        };
        self.current_weight = attempted;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_stack(
        &mut self,
        kind: Entity,
        mut count: u32,
        max_stack: u32,
        per_unit_weight: f32,
        name: String,
        rarity: Rarity,
        tags: TagSet,
    ) -> Result<(), TransferError> {
        if count == 0 {
            return Ok(());
        }

        if let Some(idx) = self
            .slots
            .iter()
            .position(|slot| matches!(slot, SlotEntry::Stack { kind: k, .. } if *k == kind))
        {
            let (existing, cap) = match &self.slots[idx] {
                SlotEntry::Stack {
                    count: existing,
                    max_stack: cap,
                    ..
                } => (*existing, *cap),
                _ => (0, 0),
            };

            let room = cap.saturating_sub(existing);
            let merge = room.min(count);
            if merge > 0 {
                let added_weight = merge as f32 * per_unit_weight;
                let attempted = self.current_weight + added_weight;
                if attempted > self.def.weight_limit {
                    return Err(TransferError::OverWeight {
                        limit: self.def.weight_limit,
                        attempted,
                    });
                }
                self.current_weight = attempted;
                if let SlotEntry::Stack { count: c, .. } = &mut self.slots[idx] {
                    *c += merge;
                }
                count -= merge;
            }
        }

        while count > 0 {
            if self.len() >= self.capacity() {
                return Err(TransferError::ContainerFull);
            }
            let chunk = count.min(max_stack);
            let added_weight = chunk as f32 * per_unit_weight;
            let attempted = self.current_weight + added_weight;
            if attempted > self.def.weight_limit {
                return Err(TransferError::OverWeight {
                    limit: self.def.weight_limit,
                    attempted,
                });
            }
            let idx = self
                .slots
                .iter()
                .position(|slot| matches!(slot, SlotEntry::Empty))
                .ok_or(TransferError::ContainerFull)?;
            self.slots[idx] = SlotEntry::Stack {
                kind,
                count: chunk,
                max_stack,
                per_unit_weight,
                name: name.clone(),
                rarity,
                tags: tags.clone(),
            };
            self.current_weight = attempted;
            count -= chunk;
        }
        Ok(())
    }

    /// Removes a specific entity from its slot (single items only for tests).
    pub fn remove_entity(&mut self, entity: Entity) -> Result<SlotEntry, TransferError> {
        let idx = self
            .slots
            .iter()
            .position(|slot| matches!(slot, SlotEntry::Item { entity: e, .. } if *e == entity))
            .ok_or(TransferError::ItemNotFound)?;
        let removed = std::mem::replace(&mut self.slots[idx], SlotEntry::Empty);
        if let SlotEntry::Item { weight, .. } = removed {
            self.current_weight -= weight;
        }
        Ok(removed)
    }

    /// Clears a stack slot by kind id (test helper).
    pub fn remove_stack_kind(&mut self, kind: Entity) -> Result<(), TransferError> {
        let idx = self
            .slots
            .iter()
            .position(|slot| matches!(slot, SlotEntry::Stack { kind: k, .. } if *k == kind))
            .ok_or(TransferError::ItemNotFound)?;
        if let SlotEntry::Stack {
            count,
            per_unit_weight,
            ..
        } = self.slots[idx]
        {
            self.current_weight -= count as f32 * per_unit_weight;
        }
        self.slots[idx] = SlotEntry::Empty;
        Ok(())
    }
}

/// Payload accepted by `Container::insert`.
#[derive(Clone, Debug)]
pub enum ItemStack {
    /// Non-stackable item occupying one slot.
    Single {
        /// Item entity id.
        entity: Entity,
        /// Weight in kilograms.
        weight: f32,
        /// Display name.
        name: String,
        /// Rarity tier.
        rarity: Rarity,
        /// Tags carried by the item.
        tags: TagSet,
    },
    /// Stackable insertion (may merge and spill into new slots).
    Stack {
        /// Stack kind id.
        kind: Entity,
        /// Requested stack count.
        count: u32,
        /// Maximum stack height for this kind.
        max_stack: u32,
        /// Weight per unit.
        per_unit_weight: f32,
        /// Display name.
        name: String,
        /// Rarity tier.
        rarity: Rarity,
        /// Tags carried by the stack.
        tags: TagSet,
    },
}

impl ItemStack {
    fn tags(&self) -> &TagSet {
        match self {
            ItemStack::Single { tags, .. } | ItemStack::Stack { tags, .. } => tags,
        }
    }
}

/// Sorts occupied slots in-place without spawning/despawning entities.
pub fn sort_container(slots: &mut [SlotEntry], criterion: SortCriterion, order: SortOrder) {
    let mut occupied: Vec<(usize, SlotEntry)> = slots
        .iter_mut()
        .enumerate()
        .filter_map(|(idx, slot)| {
            if matches!(slot, SlotEntry::Empty) {
                None
            } else {
                Some((idx, std::mem::replace(slot, SlotEntry::Empty)))
            }
        })
        .collect();

    let mut target_indices: Vec<usize> = occupied.iter().map(|(idx, _)| *idx).collect();
    target_indices.sort_unstable();

    occupied.sort_by(|(ia, a), (ib, b)| {
        let cmp = match criterion {
            SortCriterion::Weight => {
                let wa = slot_weight(a);
                let wb = slot_weight(b);
                wa.partial_cmp(&wb).unwrap_or(std::cmp::Ordering::Equal)
            }
            SortCriterion::Name => {
                let na = slot_name(a);
                let nb = slot_name(b);
                na.cmp(nb)
            }
            SortCriterion::Rarity => {
                let ra = slot_rarity(a);
                let rb = slot_rarity(b);
                ra.cmp(&rb)
            }
        };
        let cmp = match order {
            SortOrder::Asc => cmp,
            SortOrder::Desc => cmp.reverse(),
        };
        cmp.then_with(|| ia.cmp(ib))
    });

    let sorted_entries: Vec<SlotEntry> = occupied.into_iter().map(|(_, slot)| slot).collect();
    for (dest_idx, slot) in target_indices.into_iter().zip(sorted_entries) {
        slots[dest_idx] = slot;
    }
}

fn slot_weight(slot: &SlotEntry) -> f32 {
    match slot {
        SlotEntry::Empty => 0.0,
        SlotEntry::Item { weight, .. } => *weight,
        SlotEntry::Stack {
            count,
            per_unit_weight,
            ..
        } => *count as f32 * *per_unit_weight,
    }
}

fn slot_name(slot: &SlotEntry) -> &str {
    match slot {
        SlotEntry::Empty => "",
        SlotEntry::Item { name, .. } | SlotEntry::Stack { name, .. } => name.as_str(),
    }
}

fn slot_rarity(slot: &SlotEntry) -> Rarity {
    match slot {
        SlotEntry::Empty => Rarity::Common,
        SlotEntry::Item { rarity, .. } | SlotEntry::Stack { rarity, .. } => *rarity,
    }
}
