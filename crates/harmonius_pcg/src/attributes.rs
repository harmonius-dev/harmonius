//! Columnar attribute storage on point streams.

use std::collections::HashMap;

/// Simple column store keyed by attribute name.
#[derive(Debug, Default, Clone)]
pub struct AttributeStore {
    columns: HashMap<String, Vec<u32>>,
}

impl AttributeStore {
    /// New empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces a `u32` column.
    pub fn insert_u32_column(&mut self, name: &str, values: Vec<u32>) {
        self.columns.insert(name.to_string(), values);
    }

    /// Reads a `u32` column.
    pub fn get_u32_column(&self, name: &str) -> Option<&[u32]> {
        self.columns.get(name).map(|v| v.as_slice())
    }

    /// Partitions row indices by discrete `key` column values.
    pub fn partition_by_u32(&self, key: &str) -> HashMap<u32, Vec<usize>> {
        let mut out: HashMap<u32, Vec<usize>> = HashMap::new();
        if let Some(col) = self.get_u32_column(key) {
            for (idx, v) in col.iter().enumerate() {
                out.entry(*v).or_default().push(idx);
            }
        }
        out
    }
}
