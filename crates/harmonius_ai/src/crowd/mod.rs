//! Crowd-level flocking, flow fields, density, and LOD helpers.

pub mod density;
pub mod flocking;
pub mod flow_cache;
pub mod flow_field;
pub mod lod;

pub use density::{enforce_density, DensityCell, DensityGrid, OverflowAction};
pub use flocking::compute_flocking;
pub use flow_cache::{FlowFieldCache, FlowFieldKey};
pub use flow_field::{
    generate_flow_field, sample_flow_field, CostGrid, FlowField, UniformCostGrid,
};
pub use lod::{assign_lod_tiers, AiBudget, AiLodConfig, AiLodTier, LodAgent};
