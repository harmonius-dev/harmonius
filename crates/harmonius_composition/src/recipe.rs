//! [`CompositionRecipe`] trait and [`RecipeHandle`] (R-16.5.2, R-16.5.6).

use bevy_ecs::prelude::*;
use smallvec::SmallVec;

use crate::asset_id::AssetId;
use crate::bind_error::BindError;
use crate::primitive_kind::PrimitiveKind;

/// Reference to a definition asset installed by a recipe.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DefinitionRef {
    /// Meter definition asset.
    Meter(AssetId),
    /// Attribute set definition asset.
    AttributeSet(AssetId),
    /// Container definition asset.
    Container(AssetId),
    /// Directed graph definition asset.
    DirectedGraph(AssetId),
}

/// Handle returned after a successful recipe [`CompositionRecipe::install`](CompositionRecipe::install).
#[derive(Debug)]
pub struct RecipeHandle {
    /// Root entity for the recipe instance.
    pub root: Entity,
    /// Definitions bound in install order (uninstall reverses).
    pub definitions: SmallVec<[DefinitionRef; 8]>,
}

/// Minimal context passed into recipe installation.
#[derive(Clone, Copy, Debug)]
pub struct RecipeContext {
    /// Current simulation tick (deterministic scheduling).
    pub tick: u64,
}

/// Errors surfaced by recipe installation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecipeError {
    /// Underlying bind failure.
    Bind(BindError),
}

impl From<BindError> for RecipeError {
    fn from(value: BindError) -> Self {
        RecipeError::Bind(value)
    }
}

/// A gameplay recipe composing multiple primitives (R-16.5.2).
pub trait CompositionRecipe {
    /// Human-readable recipe name.
    fn name(&self) -> &'static str;

    /// Primitives participating in this recipe.
    fn primitives(&self) -> &'static [PrimitiveKind];

    /// Install the recipe under `root`.
    fn install(
        &self,
        world: &mut World,
        root: Entity,
        ctx: &RecipeContext,
    ) -> Result<RecipeHandle, RecipeError>;

    /// Uninstall in reverse bind order (R-16.5.6).
    fn uninstall(&self, world: &mut World, handle: &RecipeHandle) -> Result<(), RecipeError>;
}
