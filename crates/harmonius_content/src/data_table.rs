//! Row/column data tables stored as `.hat` text.

use std::collections::BTreeMap;

/// Sparse row storage: row id -> column -> cell string.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DataTable {
    /// Rows keyed by index.
    pub rows: BTreeMap<u32, BTreeMap<String, String>>,
}

impl DataTable {
    /// Parse simple `row,col=value` lines.
    pub fn parse_hat(text: &str) -> Self {
        let mut rows: BTreeMap<u32, BTreeMap<String, String>> = BTreeMap::new();
        for line in text.lines() {
            let Some((left, val)) = line.split_once('=') else {
                continue;
            };
            let Some((rs, cs)) = left.split_once(',') else {
                continue;
            };
            let r: u32 = rs.parse().unwrap_or(0);
            rows.entry(r)
                .or_default()
                .insert(cs.to_string(), val.to_string());
        }
        Self { rows }
    }

    /// Serialize to `.hat` text.
    pub fn to_hat(&self) -> String {
        let mut out = String::new();
        for (r, cols) in &self.rows {
            for (c, v) in cols {
                out.push_str(&format!("{r},{c}={v}\n"));
            }
        }
        out
    }
}

/// Edit one cell; returns new serialized table.
pub fn edit_data_table_cell(table: &DataTable, row: u32, col: &str, value: &str) -> String {
    let mut t = table.clone();
    t.rows.entry(row).or_default().insert(col.to_string(), value.to_string());
    t.to_hat()
}

/// Count changed cells between two tables.
pub fn structural_diff_table_cells(a: &DataTable, b: &DataTable) -> usize {
    let mut n = 0usize;
    for (r, cols_b) in &b.rows {
        let cols_a = a.rows.get(r);
        for (c, vb) in cols_b {
            let va = cols_a.and_then(|m| m.get(c));
            if va.map(String::as_str) != Some(vb.as_str()) {
                n += 1;
            }
        }
    }
    n
}
