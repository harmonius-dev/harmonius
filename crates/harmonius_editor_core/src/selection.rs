//! Selection sets and modifiers (`TC-15.1.4.*`).

use smallvec::SmallVec;
use std::collections::HashSet;
use std::fmt;

/// Stable entity identifier for editor selection tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity(pub u64);

/// Stable asset identifier for editor selection tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AssetId(pub u64);

/// Sub-element targeted on a mesh or skeleton.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SubObjectElement {
    /// Mesh vertex index.
    Vertex(u32),
    /// Mesh edge index.
    Edge(u32),
    /// Mesh face index.
    Face(u32),
    /// Skeleton bone index.
    Bone(u32),
}

/// Anything the editor can select.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Selectable {
    /// ECS entity selection.
    Entity(Entity),
    /// Content database asset selection.
    Asset(AssetId),
    /// Component sub-object selection.
    SubObject {
        /// Owning entity.
        entity: Entity,
        /// Vertex, edge, face, or bone handle.
        element: SubObjectElement,
    },
}

/// Modifier keys applied to the next selection operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectionModifier {
    /// Replace the active selection.
    Replace,
    /// Union with the active selection.
    Add,
    /// Remove hits from the active selection.
    Subtract,
    /// Toggle membership for each hit.
    Toggle,
}

/// Named saved selection set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionSet {
    /// User-visible label.
    pub name: String,
    /// Members captured when the set was saved.
    pub items: Vec<Selectable>,
}

/// Recoverable selection failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectionError {
    /// Unknown named selection set.
    UnknownSet,
}

impl fmt::Display for SelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectionError::UnknownSet => write!(f, "unknown selection set"),
        }
    }
}

impl std::error::Error for SelectionError {}

/// Current editor selection and saved sets.
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    items: SmallVec<[Selectable; 8]>,
    saved_sets: Vec<SelectionSet>,
}

impl SelectionState {
    /// Empty selection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies `modifier` to merge `items` into the active selection.
    pub fn select(&mut self, items: &[Selectable], modifier: SelectionModifier) {
        match modifier {
            SelectionModifier::Replace => {
                self.items.clear();
                self.items.extend_from_slice(items);
            }
            SelectionModifier::Add => {
                for it in items {
                    if !self.items.contains(it) {
                        self.items.push(*it);
                    }
                }
            }
            SelectionModifier::Subtract => {
                for it in items {
                    self.items.retain(|x| *x != *it);
                }
            }
            SelectionModifier::Toggle => {
                for it in items {
                    if let Some(pos) = self.items.iter().position(|x| *x == *it) {
                        self.items.remove(pos);
                    } else {
                        self.items.push(*it);
                    }
                }
            }
        }
    }

    /// Clears the active selection without touching saved sets.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns unique entities referenced by the active selection.
    pub fn entities(&self) -> Vec<Entity> {
        let mut out = HashSet::new();
        for s in &self.items {
            match s {
                Selectable::Entity(e) => {
                    out.insert(*e);
                }
                Selectable::SubObject { entity, .. } => {
                    out.insert(*entity);
                }
                Selectable::Asset(_) => {}
            }
        }
        out.into_iter().collect()
    }

    /// Saves the current selection under `name`.
    pub fn save_set(&mut self, name: String) {
        self.saved_sets.push(SelectionSet {
            name,
            items: self.items.to_vec(),
        });
    }

    /// Replaces the active selection from a saved set.
    pub fn restore_set(&mut self, name: &str) -> Result<(), SelectionError> {
        let set = self
            .saved_sets
            .iter()
            .find(|s| s.name == name)
            .ok_or(SelectionError::UnknownSet)?;
        self.items.clear();
        self.items.extend_from_slice(&set.items);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_15_1_4_1_click_select_entity() {
        let mut s = SelectionState::new();
        let e = Entity(9);
        s.select(&[Selectable::Entity(e)], SelectionModifier::Replace);
        assert_eq!(s.entities(), vec![e]);
    }

    #[test]
    fn tc_15_1_4_2_marquee_select() {
        let mut s = SelectionState::new();
        let a = Entity(1);
        let b = Entity(2);
        s.select(
            &[Selectable::Entity(a), Selectable::Entity(b)],
            SelectionModifier::Replace,
        );
        let mut ents = s.entities();
        ents.sort_by_key(|e| e.0);
        assert_eq!(ents, vec![a, b]);
    }

    #[test]
    fn tc_15_1_4_3_selection_modifiers() {
        let mut s = SelectionState::new();
        let a = Entity(1);
        let b = Entity(2);
        s.select(&[Selectable::Entity(a)], SelectionModifier::Replace);
        s.select(&[Selectable::Entity(b)], SelectionModifier::Add);
        s.select(&[Selectable::Entity(a)], SelectionModifier::Subtract);
        assert_eq!(s.entities(), vec![b]);
        s.select(&[Selectable::Entity(a)], SelectionModifier::Toggle);
        s.select(&[Selectable::Entity(b)], SelectionModifier::Toggle);
        let mut ents = s.entities();
        ents.sort_by_key(|e| e.0);
        assert_eq!(ents, vec![a]);
    }

    #[test]
    fn tc_15_1_4_4_named_selection_sets() {
        let mut s = SelectionState::new();
        let a = Entity(5);
        s.select(&[Selectable::Entity(a)], SelectionModifier::Replace);
        s.save_set("heroes".to_string());
        s.clear();
        assert!(s.entities().is_empty());
        s.restore_set("heroes").unwrap();
        assert_eq!(s.entities(), vec![a]);
    }
}
