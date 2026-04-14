/// One of the eight composition primitives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveKind {
    /// Directed graph primitive.
    DirectedGraph,
    /// Data table primitive.
    DataTable,
    /// Attributes / effects primitive.
    Attributes,
    /// Containers / slots primitive.
    Containers,
    /// Grid / volume primitive.
    Grid,
    /// Spatial awareness primitive.
    Awareness,
    /// Timeline primitive.
    Timeline,
    /// Event log primitive.
    EventLog,
}
