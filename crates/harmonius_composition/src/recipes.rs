//! Concrete [`CompositionRecipe`](super::CompositionRecipe) implementations (R-16.5.2).

use bevy_ecs::prelude::*;
use smallvec::smallvec;

use crate::components::{
    AbilityState, AttributeSet, CraftingState, Destination, DirectedGraphInstance, Effect,
    InventoryState, QuestState, ScheduleState, StealthState,
};
use crate::definition_asset::DefinitionAsset;
use crate::definitions::{
    AttributeSetDefinition, ContainerDefinition, DirectedGraphDefinition, MeterDefinition,
};
use crate::events::{CompositionEvent, CompositionEventQueue, EntryAppended};
use crate::primitive_kind::PrimitiveKind;
use crate::recipe::{CompositionRecipe, DefinitionRef, RecipeContext, RecipeError, RecipeHandle};

/// Quest recipe: graph + attribute set + quest state (R-16.5.2).
pub struct QuestRecipe {
    /// Graph definition asset.
    pub graph: DirectedGraphDefinition,
    /// Buff attribute set applied while quest active.
    pub buff_attrs: AttributeSetDefinition,
}

impl CompositionRecipe for QuestRecipe {
    fn name(&self) -> &'static str {
        "quest"
    }

    fn primitives(&self) -> &'static [PrimitiveKind] {
        const P: &[PrimitiveKind] = &[PrimitiveKind::DirectedGraph, PrimitiveKind::Attributes];
        P
    }

    fn install(
        &self,
        world: &mut World,
        root: Entity,
        _ctx: &RecipeContext,
    ) -> Result<RecipeHandle, RecipeError> {
        let gh = self.graph.mint_handle(world);
        self.graph.bind(world, root, gh)?;
        let ah = self.buff_attrs.mint_handle(world);
        self.buff_attrs.bind(world, root, ah)?;
        world.entity_mut(root).insert(QuestState {
            active_objective: 0,
            objectives: 3,
        });
        Ok(RecipeHandle {
            root,
            definitions: smallvec![
                DefinitionRef::DirectedGraph(self.graph.id()),
                DefinitionRef::AttributeSet(self.buff_attrs.id()),
            ],
        })
    }

    fn uninstall(&self, world: &mut World, handle: &RecipeHandle) -> Result<(), RecipeError> {
        world.entity_mut(handle.root).remove::<QuestState>();
        // Reverse bind order: attribute set then graph.
        self.buff_attrs.unbind(world, handle.root)?;
        self.graph.unbind(world, handle.root)?;
        Ok(())
    }
}

/// Ability recipe: graph + meter + ability state.
pub struct AbilityRecipe {
    /// Activation graph.
    pub graph: DirectedGraphDefinition,
    /// Mana pool meter.
    pub mana: MeterDefinition,
}

impl CompositionRecipe for AbilityRecipe {
    fn name(&self) -> &'static str {
        "ability"
    }

    fn primitives(&self) -> &'static [PrimitiveKind] {
        const P: &[PrimitiveKind] = &[PrimitiveKind::DirectedGraph, PrimitiveKind::Attributes];
        P
    }

    fn install(
        &self,
        world: &mut World,
        root: Entity,
        _ctx: &RecipeContext,
    ) -> Result<RecipeHandle, RecipeError> {
        let gh = self.graph.mint_handle(world);
        self.graph.bind(world, root, gh)?;
        let mh = self.mana.mint_handle(world);
        self.mana.bind(world, root, mh)?;
        world.entity_mut(root).insert(AbilityState { mana: 100.0 });
        Ok(RecipeHandle {
            root,
            definitions: smallvec![
                DefinitionRef::DirectedGraph(self.graph.id()),
                DefinitionRef::Meter(self.mana.id()),
            ],
        })
    }

    fn uninstall(&self, world: &mut World, handle: &RecipeHandle) -> Result<(), RecipeError> {
        world.entity_mut(handle.root).remove::<AbilityState>();
        self.mana.unbind(world, handle.root)?;
        self.graph.unbind(world, handle.root)?;
        Ok(())
    }
}

/// Inventory recipe: container + attribute set + inventory state.
pub struct InventoryRecipe {
    /// Item grid container.
    pub container: ContainerDefinition,
    /// Stats.
    pub stats: AttributeSetDefinition,
}

impl CompositionRecipe for InventoryRecipe {
    fn name(&self) -> &'static str {
        "inventory"
    }

    fn primitives(&self) -> &'static [PrimitiveKind] {
        const P: &[PrimitiveKind] = &[PrimitiveKind::Containers, PrimitiveKind::Attributes];
        P
    }

    fn install(
        &self,
        world: &mut World,
        root: Entity,
        _ctx: &RecipeContext,
    ) -> Result<RecipeHandle, RecipeError> {
        let ch = self.container.mint_handle(world);
        self.container.bind(world, root, ch)?;
        let sh = self.stats.mint_handle(world);
        self.stats.bind(world, root, sh)?;
        world
            .entity_mut(root)
            .insert(InventoryState { attack: 10.0 });
        Ok(RecipeHandle {
            root,
            definitions: smallvec![
                DefinitionRef::Container(self.container.id()),
                DefinitionRef::AttributeSet(self.stats.id()),
            ],
        })
    }

    fn uninstall(&self, world: &mut World, handle: &RecipeHandle) -> Result<(), RecipeError> {
        world.entity_mut(handle.root).remove::<InventoryState>();
        self.stats.unbind(world, handle.root)?;
        self.container.unbind(world, handle.root)?;
        Ok(())
    }
}

/// Crafting recipe: graph + container + crafting state + timeline placeholder.
pub struct CraftingRecipe {
    pub graph: DirectedGraphDefinition,
    pub container: ContainerDefinition,
}

impl CompositionRecipe for CraftingRecipe {
    fn name(&self) -> &'static str {
        "crafting"
    }

    fn primitives(&self) -> &'static [PrimitiveKind] {
        const P: &[PrimitiveKind] = &[
            PrimitiveKind::DirectedGraph,
            PrimitiveKind::Containers,
            PrimitiveKind::Timeline,
        ];
        P
    }

    fn install(
        &self,
        world: &mut World,
        root: Entity,
        _ctx: &RecipeContext,
    ) -> Result<RecipeHandle, RecipeError> {
        let gh = self.graph.mint_handle(world);
        self.graph.bind(world, root, gh)?;
        let ch = self.container.mint_handle(world);
        self.container.bind(world, root, ch)?;
        world.entity_mut(root).insert(CraftingState {
            herbs: 2,
            vials: 1,
            potions: 0,
        });
        world.entity_mut(root).insert(ScheduleState {
            elapsed: 0,
            fire_at: 540,
            fired: false,
        });
        Ok(RecipeHandle {
            root,
            definitions: smallvec![
                DefinitionRef::DirectedGraph(self.graph.id()),
                DefinitionRef::Container(self.container.id()),
            ],
        })
    }

    fn uninstall(&self, world: &mut World, handle: &RecipeHandle) -> Result<(), RecipeError> {
        world.entity_mut(handle.root).remove::<ScheduleState>();
        world.entity_mut(handle.root).remove::<CraftingState>();
        self.container.unbind(world, handle.root)?;
        self.graph.unbind(world, handle.root)?;
        Ok(())
    }
}

/// Stealth recipe: attribute + stealth state (grid / awareness represented minimally).
pub struct StealthRecipe {
    pub attrs: AttributeSetDefinition,
}

impl CompositionRecipe for StealthRecipe {
    fn name(&self) -> &'static str {
        "stealth"
    }

    fn primitives(&self) -> &'static [PrimitiveKind] {
        const P: &[PrimitiveKind] = &[PrimitiveKind::Attributes];
        P
    }

    fn install(
        &self,
        world: &mut World,
        root: Entity,
        _ctx: &RecipeContext,
    ) -> Result<RecipeHandle, RecipeError> {
        let h = self.attrs.mint_handle(world);
        self.attrs.bind(world, root, h)?;
        world
            .entity_mut(root)
            .insert(StealthState { detected: false });
        Ok(RecipeHandle {
            root,
            definitions: smallvec![DefinitionRef::AttributeSet(self.attrs.id())],
        })
    }

    fn uninstall(&self, world: &mut World, handle: &RecipeHandle) -> Result<(), RecipeError> {
        world.entity_mut(handle.root).remove::<StealthState>();
        self.attrs.unbind(world, handle.root)?;
        Ok(())
    }
}

/// NPC schedule recipe: timeline state + destination placeholder.
pub struct ScheduleRecipe {
    pub role_table: AttributeSetDefinition,
}

impl CompositionRecipe for ScheduleRecipe {
    fn name(&self) -> &'static str {
        "schedule"
    }

    fn primitives(&self) -> &'static [PrimitiveKind] {
        const P: &[PrimitiveKind] = &[PrimitiveKind::Timeline, PrimitiveKind::Attributes];
        P
    }

    fn install(
        &self,
        world: &mut World,
        root: Entity,
        ctx: &RecipeContext,
    ) -> Result<RecipeHandle, RecipeError> {
        let h = self.role_table.mint_handle(world);
        self.role_table.bind(world, root, h)?;
        let target = world.spawn_empty().id();
        world.entity_mut(root).insert(ScheduleState {
            elapsed: ctx.tick,
            fire_at: 540,
            fired: false,
        });
        world.entity_mut(root).insert(Destination { target });
        Ok(RecipeHandle {
            root,
            definitions: smallvec![DefinitionRef::AttributeSet(self.role_table.id())],
        })
    }

    fn uninstall(&self, world: &mut World, handle: &RecipeHandle) -> Result<(), RecipeError> {
        if let Some(dest) = world
            .entity(handle.root)
            .get::<Destination>()
            .map(|d| d.target)
        {
            let _ = world.despawn(dest);
        }
        world.entity_mut(handle.root).remove::<Destination>();
        world.entity_mut(handle.root).remove::<ScheduleState>();
        self.role_table.unbind(world, handle.root)?;
        Ok(())
    }
}

/// Apply reward effect when quest reaches final objective (integration helper).
pub fn apply_quest_reward(world: &mut World, quest: Entity) {
    let (active, max) = world
        .entity(quest)
        .get::<QuestState>()
        .map(|q| (q.active_objective, q.objectives))
        .unwrap_or((0, 1));
    if active + 1 == max {
        world.entity_mut(quest).insert(Effect {
            name: "quest_reward".into(),
            magnitude: 5.0,
        });
    }
}

/// Wire [`crate::events::EntryAppended`] to advance a graph when label matches `"kill"`.
pub fn on_entry_appended_advance_graph(world: &mut World, evt: &crate::events::EntryAppended) {
    if evt.label != "kill" {
        return;
    }
    if let Some(mut g) = world
        .entity_mut(evt.target)
        .get_mut::<DirectedGraphInstance>()
    {
        g.current_node = (g.current_node + 1).min(g.nodes.saturating_sub(1));
    }
    if let Some(mut q) = world.entity_mut(evt.target).get_mut::<QuestState>() {
        q.active_objective = (q.active_objective + 1).min(q.objectives.saturating_sub(1));
    }
}

/// Propagate slot change into attribute aggregation (R-16.5.3).
pub fn on_slot_changed_aggregate(world: &mut World, evt: &crate::events::SlotChanged) {
    if let Some(mut attrs) = world.entity_mut(evt.owner).get_mut::<AttributeSet>() {
        let bonus = if evt.slot == 0 { 5.0 } else { 0.0 };
        attrs.values.entry("attack".into()).or_insert(10.0);
        if let Some(v) = attrs.values.get_mut("attack") {
            *v += bonus;
        }
    }
}

/// Grid cell change crosses threshold -> apply effect (R-16.5.3).
pub fn on_cell_changed_apply_effect(world: &mut World, evt: &crate::events::CellChanged) {
    if evt.value >= 1.0 {
        world.entity_mut(evt.owner).insert(Effect {
            name: "grid_pulse".into(),
            magnitude: evt.value,
        });
    }
}

/// Timeline keyframe fires -> enqueue logical event log entry (R-16.5.3).
///
/// When a [`CompositionEventQueue`](crate::events::CompositionEventQueue) resource is present,
/// appends [`CompositionEvent::EntryAppended`](crate::events::CompositionEvent::EntryAppended) so
/// tests and future ingest phases observe the same bus shape.
pub fn on_keyframe_fire_log(world: &mut World, evt: &crate::events::KeyframeFired) {
    if let Some(mut q) = world.get_resource_mut::<CompositionEventQueue>() {
        q.send(CompositionEvent::EntryAppended(EntryAppended {
            target: evt.owner,
            label: "keyframe",
        }));
    }
}
