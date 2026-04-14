//! Typed visual overrides for socket attachments (IR-5.8.2).

use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// Marker type for mesh assets referenced by [`AssetHandle`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Mesh;

/// Marker type for material assets referenced by [`AssetHandle`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Material;

/// Strongly typed asset handle (no untyped ids at integration boundaries).
pub struct AssetHandle<T> {
    /// Stable asset identifier from the asset table.
    pub id: u32,
    marker: PhantomData<fn() -> T>,
}

impl<T> Clone for AssetHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for AssetHandle<T> {}

impl<T> std::fmt::Debug for AssetHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssetHandle")
            .field("id", &self.id)
            .finish()
    }
}

impl<T> PartialEq for AssetHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for AssetHandle<T> {}

impl<T> Hash for AssetHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> AssetHandle<T> {
    /// Constructs a handle from a raw asset id (tests and loaders).
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }
}

/// Optional mesh/material overrides applied when an item attaches to a socket.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisualOverride {
    /// Replacement mesh, if any.
    pub mesh_override: Option<AssetHandle<Mesh>>,
    /// Replacement material, if any.
    pub material_override: Option<AssetHandle<Material>>,
    /// When true, the base socket mesh should not draw.
    pub hide_socket_visual: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-5.8.2.U1 — mesh override field is typed `Option<AssetHandle<Mesh>>`.
    #[test]
    fn tc_ir_5_8_2_u1_typed_mesh_override_handle() {
        let vo = VisualOverride {
            mesh_override: Some(AssetHandle::<Mesh>::new(1)),
            material_override: None,
            hide_socket_visual: false,
        };
        assert_eq!(vo.mesh_override.unwrap().id, 1);
    }
}
