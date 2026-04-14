//! Replication: deltas, schema negotiation, AOI, priority, dormancy.

pub mod delta;
pub mod dormancy;
pub mod interest;
pub mod owner;
pub mod priority;
pub mod schema;

pub use delta::{delta_encode_component20, Component20Fields};
pub use dormancy::{DormancyTracker, EntityId};
pub use interest::{filter_aoi_radius_m, EntityPos};
pub use owner::owner_only_field;
pub use priority::{schedule_replication, EntityRep, PriorityClass};
pub use schema::{decode_with_optional_field, SchemaRev};
