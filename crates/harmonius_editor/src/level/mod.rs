//! Level-authoring modules (placement, snapping, templates, …).
pub mod snap;
pub mod template;
pub mod template_override;

pub use snap::{
    rotate_vector_by_unit_quat, rotation_align_positive_y_to_surface_normal, snap_grid,
    snap_position, snap_position_to_nearest_vertex, Quat, SnapMode, Vec3,
};
pub use template::{
    instantiate_template_hierarchy, resolve_instance_hp, EntityTemplate, EntityTemplateId,
    InstanceHp, InstantiateError, SourceTemplateHp, SpawnedEntity, TemplateAsset,
};
pub use template_override::{
    apply_override_to_source, effective_i32, ApplyToSourceError, PropertyOverrideMap,
};
