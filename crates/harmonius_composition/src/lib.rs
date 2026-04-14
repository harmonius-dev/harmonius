//! Composition layer primitives: [`DefinitionAsset`], recipes, deterministic stepping, snapshots.
//!
//! Implements design `docs/design/data-systems/composition.md` (R-16.5.x).
//!
//! Cross-primitive wiring is expressed as synchronous helper callbacks plus an optional
//! [`CompositionEventQueue`](events::CompositionEventQueue) resource until engine-wide Bevy event
//! routing lands for this subsystem.

#![deny(clippy::all)]
#![deny(unsafe_code)]
// Rustdoc on every `pub use` re-export is deferred until this crate lands in the full workspace.
#![allow(missing_docs)]
#![allow(dead_code)]

mod asset_id;
mod bind_error;
mod components;
mod definition_asset;
mod definitions;
mod determinism;
mod events;
mod phase;
mod primitive_kind;
mod recipe;
mod recipes;
mod snapshot;

pub use asset_id::AssetId;
pub use bind_error::BindError;
pub use components::{
    AbilityCastTarget, AbilityState, AttributeSet, Container, ContainerLayout, CraftingState,
    Destination, DirectedGraphInstance, Effect, InventoryState, ItemId, Meter, QuestState,
    ScheduleState, StealthState,
};
pub use definition_asset::DefinitionAsset;
pub use definitions::{
    AttributeSetDefinition, ContainerDefinition, DirectedGraphDefinition, MeterDefinition,
};
pub use determinism::{peer_state_hash, step_frame, DeterministicRng, FrameInput};
pub use events::{
    CellChanged, CompositionEvent, CompositionEventQueue, EntryAppended, KeyframeFired,
    SlotChanged, ThresholdCrossed,
};
pub use phase::FramePhase;
pub use primitive_kind::PrimitiveKind;
pub use recipe::{CompositionRecipe, DefinitionRef, RecipeContext, RecipeError, RecipeHandle};
pub use recipes::{
    apply_quest_reward, on_cell_changed_apply_effect, on_entry_appended_advance_graph,
    on_keyframe_fire_log, on_slot_changed_aggregate, AbilityRecipe, CraftingRecipe,
    InventoryRecipe, QuestRecipe, ScheduleRecipe, StealthRecipe,
};
pub use snapshot::{
    load_crafting_snapshot, load_inventory_snapshot, load_snapshot, save_crafting_snapshot,
    save_inventory_snapshot, save_snapshot, CompositionSnapshot, CraftingProgressSnapshot,
    InventoryEffectSnapshot, SnapshotError,
};
