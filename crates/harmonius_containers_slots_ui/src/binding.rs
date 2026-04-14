use smallvec::SmallVec;

use crate::entity::Entity;

/// One-way ECS binding from a container entity to slot widget entities.
///
/// Uses inline storage for up to 32 widgets; additional entries spill to the heap without panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryGridBinding {
    /// Container entity this grid displays.
    pub container_entity: Entity,
    slot_widgets: SmallVec<[Entity; 32]>,
    /// Grid width in cells (`u16` matches design / TC-IR-5.9.1.U2).
    pub grid_width: u16,
    /// Grid height in cells.
    pub grid_height: u16,
}

impl InventoryGridBinding {
    /// Creates an empty binding for `container` with the given grid dimensions.
    #[must_use]
    pub fn new(container_entity: Entity, grid_width: u16, grid_height: u16) -> Self {
        Self {
            container_entity,
            slot_widgets: SmallVec::new(),
            grid_width,
            grid_height,
        }
    }

    /// Current number of slot widget entities tracked by this binding.
    #[must_use]
    pub fn slot_widget_len(&self) -> usize {
        self.slot_widgets.len()
    }

    /// Appends a slot widget entity (heap spill is allowed after 32 entries).
    pub fn push_slot_widget(&mut self, widget: Entity) {
        self.slot_widgets.push(widget);
    }

    /// Read-only view of bound slot widget entities.
    #[must_use]
    pub fn slot_widgets(&self) -> &[Entity] {
        self.slot_widgets.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_5_9_1_u1_smallvec_spill_threshold() {
        let mut b = InventoryGridBinding::new(Entity(1), 8, 8);
        for i in 0..33 {
            b.push_slot_widget(Entity(i));
        }
        assert_eq!(b.slot_widget_len(), 33);
        assert_eq!(b.slot_widgets()[32], Entity(32));
    }

    #[test]
    fn tc_ir_5_9_1_u2_binding_width_height_u16_bounds() {
        let b = InventoryGridBinding::new(Entity(2), u16::MAX, 1);
        assert_eq!(b.grid_width, u16::MAX);
        assert_eq!(b.grid_height, 1);
        let cells = u32::from(b.grid_width) * u32::from(b.grid_height);
        assert_eq!(cells, u32::from(u16::MAX));
    }
}
