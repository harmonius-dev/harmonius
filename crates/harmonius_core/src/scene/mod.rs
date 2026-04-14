//! Scene hierarchy, transforms, propagation, and spatial integration hooks.

mod commands;
mod entity;
mod error;
mod hierarchy;
mod propagation;
mod resources;
mod scene_blob;
mod schedule;
mod transform;
mod transform2d;
mod traversal;
mod world;

pub use commands::{HierarchyCommand, HierarchyCommandBuffer};
pub use entity::Entity;
pub use error::SceneError;
pub use hierarchy::{Children, HierarchyEvent, OrphanPolicy, Parent};
pub use propagation::{propagate_transforms, ChangeTick};
pub use resources::{BvhResource, Resources, SpatialQueryBackend};
pub use scene_blob::{EntityMap, Scene, SceneInstanceId, SceneSpawner};
pub use schedule::{Schedule, SystemId};
pub use transform::{GlobalTransform, InterpolatedTransform, PreviousGlobalTransform, Transform};
pub use transform2d::{GlobalTransform2D, PreviousGlobalTransform2D, Transform2D};
pub use traversal::{BreadthFirstIterator, DepthFirstIterator, HierarchyNode};
pub use world::World;
