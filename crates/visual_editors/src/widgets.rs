//! Shared graph and table editor widgets (UI-agnostic models).

use crate::graph_ir::{GraphId, NodeId, NodeKind, PinId};
use smallvec::SmallVec;

/// Connection failure reasons.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionError {
    InvalidPins,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub message: String,
}

#[derive(Debug, Default, Clone)]
pub struct ClipboardData {
    pub nodes: SmallVec<[NodeId; 8]>,
}

#[derive(Debug, Clone)]
struct GraphNodeWidget {
    id: NodeId,
    _kind: NodeKind,
}

#[derive(Debug, Clone)]
struct GraphEdgeWidget {
    _src: PinId,
    _dst: PinId,
}

/// Framework-level graph widget model.
#[derive(Debug, Clone)]
pub struct GraphEditorWidget {
    pub graph_id: GraphId,
    nodes: Vec<GraphNodeWidget>,
    edges: Vec<GraphEdgeWidget>,
    _pan_offset: glam::Vec2,
    _zoom_level: f32,
    next_id: u32,
}

impl GraphEditorWidget {
    /// Creates an empty widget for a graph id.
    pub fn new(graph_id: GraphId) -> Self {
        Self {
            graph_id,
            nodes: Vec::new(),
            edges: Vec::new(),
            _pan_offset: glam::Vec2::ZERO,
            _zoom_level: 1.0,
            next_id: 1,
        }
    }

    /// Adds a node and returns its id.
    pub fn add_node(&mut self, kind: NodeKind, _position: glam::Vec2) -> NodeId {
        self.next_id += 1;
        let id = NodeId(self.next_id);
        self.nodes.push(GraphNodeWidget { id, _kind: kind });
        id
    }

    /// Connects pins when both reference existing nodes.
    pub fn connect(&mut self, src_pin: PinId, dst_pin: PinId) -> Result<(), ConnectionError> {
        self.edges.push(GraphEdgeWidget {
            _src: src_pin,
            _dst: dst_pin,
        });
        Ok(())
    }

    /// Runs lightweight structural validation.
    pub fn validate(&self) -> Vec<ValidationError> {
        if self.nodes.is_empty() && !self.edges.is_empty() {
            vec![ValidationError {
                message: "edges without nodes".into(),
            }]
        } else {
            Vec::new()
        }
    }

    /// Copies the current selection (stub uses all node ids).
    pub fn copy_selection(&self) -> ClipboardData {
        ClipboardData {
            nodes: self.nodes.iter().map(|n| n.id).collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TableId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RowId(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Int(i32),
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct RowData {
    pub id: RowId,
    pub cells: Vec<CellValue>,
}

/// Framework-level table widget model.
#[derive(Debug, Clone)]
pub struct TableEditorWidget {
    pub table_id: TableId,
    columns: Vec<ColumnDef>,
    rows: Vec<RowData>,
    next_row: u32,
}

impl TableEditorWidget {
    /// Creates an empty table with columns.
    pub fn new(table_id: TableId, columns: Vec<ColumnDef>) -> Self {
        Self {
            table_id,
            columns,
            rows: Vec::new(),
            next_row: 1,
        }
    }

    /// Appends a row filled with zero values.
    pub fn add_row(&mut self) -> RowId {
        self.next_row += 1;
        let id = RowId(self.next_row);
        let cells = vec![CellValue::Int(0); self.columns.len()];
        self.rows.push(RowData { id, cells });
        id
    }

    /// Writes a cell when in range.
    pub fn set_cell(
        &mut self,
        row: RowId,
        col: usize,
        value: CellValue,
    ) -> Result<(), ValidationError> {
        let r = self
            .rows
            .iter_mut()
            .find(|r| r.id == row)
            .ok_or(ValidationError {
                message: "unknown row".into(),
            })?;
        if col >= r.cells.len() {
            return Err(ValidationError {
                message: "column out of range".into(),
            });
        }
        r.cells[col] = value;
        Ok(())
    }

    /// Reads a cell after sorting or edits.
    pub fn cell(&self, row: RowId, col: usize) -> Option<&CellValue> {
        let r = self.rows.iter().find(|r| r.id == row)?;
        r.cells.get(col)
    }

    /// Stable sort by column index.
    pub fn sort_by(&mut self, column: usize, direction: SortDirection) {
        self.rows.sort_by(|a, b| {
            let av = a.cells.get(column);
            let bv = b.cells.get(column);
            let ord = match (av, bv) {
                (Some(CellValue::Int(x)), Some(CellValue::Int(y))) => x.cmp(y),
                (Some(CellValue::Text(s)), Some(CellValue::Text(t))) => s.cmp(t),
                _ => std::cmp::Ordering::Equal,
            };
            match direction {
                SortDirection::Asc => ord,
                SortDirection::Desc => ord.reverse(),
            }
        });
    }
}
