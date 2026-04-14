//! Minimal `TableRegistry` and codegen-shaped row structs for integration tests.

use crate::types::{ColumnId, RowId, RowRef, TableId};

/// ECS binding from an entity to a concrete table row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DatabaseRow {
    /// Owning table id.
    pub table: TableId,
    /// Owning row id.
    pub row: RowId,
}

/// Example `materials` row used by IR-2.9.x tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialsRow {
    /// Material mass used by FK tests.
    pub weight: f32,
    /// Optional next hop in a FK chain.
    pub origin: RowRef,
}

/// Example `weapons` row used by IR-2.9.x tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeaponsRow {
    /// Base damage column.
    pub base_dmg: f32,
    /// Bonus damage column.
    pub bonus: f32,
    /// Foreign key into `materials`.
    pub material: RowRef,
}

/// Immutable table snapshot used by formulas and logic graphs.
#[derive(Clone, Debug, PartialEq)]
pub struct TableRegistry {
    /// Weapons rows indexed by `RowId.0`.
    pub weapons: Vec<WeaponsRow>,
    /// Materials rows indexed by `RowId.0`.
    pub materials: Vec<MaterialsRow>,
}

impl TableRegistry {
    /// Returns `weapons[row]` when in range.
    pub fn weapons_row(&self, row: RowId) -> Option<&WeaponsRow> {
        self.weapons.get(row.0 as usize)
    }

    /// Returns `materials[row]` when in range.
    pub fn materials_row(&self, row: RowId) -> Option<&MaterialsRow> {
        self.materials.get(row.0 as usize)
    }

    /// Resolves a `RowRef` into a `MaterialsRow` when the ref targets materials.
    pub fn resolve_material(&self, reference: RowRef) -> Option<&MaterialsRow> {
        if reference.table_id != TableId(1) {
            return None;
        }
        self.materials_row(reference.row_id)
    }

    /// Resolves a `RowRef` into a `WeaponsRow` when the ref targets weapons.
    pub fn resolve_weapon(&self, reference: RowRef) -> Option<&WeaponsRow> {
        if reference.table_id != TableId(0) {
            return None;
        }
        self.weapons_row(reference.row_id)
    }

    /// Generic FK resolve used by formula codegen tests.
    pub fn resolve_foreign_key(&self, reference: RowRef) -> ForeignRow<'_> {
        if let Some(row) = self.resolve_weapon(reference) {
            ForeignRow::Weapon(row)
        } else if let Some(row) = self.resolve_material(reference) {
            ForeignRow::Material(row)
        } else {
            ForeignRow::Missing
        }
    }

    /// Codegen-shaped accessor used by table-lookup tests for `weapons.damage`.
    pub fn weapons_damage_via_lookup(
        &self,
        table_id: TableId,
        column_id: ColumnId,
        row: RowId,
    ) -> Option<f32> {
        if table_id != TableId(0) || column_id != ColumnId(0) {
            return None;
        }
        let weapon = self.weapons_row(row)?;
        Some(weapon.base_dmg + weapon.bonus)
    }

    /// Reads `weapons` through a `DatabaseRow` entity binding.
    pub fn weapons_row_via_database_row(&self, binding: DatabaseRow) -> Option<&WeaponsRow> {
        if binding.table != TableId(0) {
            return None;
        }
        self.weapons_row(binding.row)
    }
}

/// Resolved foreign row for integration tests (no `dyn`, no `Value`).
#[derive(Clone, Copy, Debug)]
pub enum ForeignRow<'a> {
    /// Resolved `WeaponsRow`.
    Weapon(&'a WeaponsRow),
    /// Resolved `MaterialsRow`.
    Material(&'a MaterialsRow),
    /// Dangling reference.
    Missing,
}
