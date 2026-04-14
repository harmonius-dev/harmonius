//! Visual editor models: graph IR, validation, widgets, and compile stubs.
//!
//! Implements traceable behavior for `PLAN-tools-visual-editors` test cases.

#![forbid(unsafe_code)]

pub mod animation;
pub mod compiler;
pub mod diff;
pub mod graph_ir;
pub mod inference;
pub mod lowering;
pub mod material;
pub mod material_graph;
pub mod optimize;
pub mod refactor;
pub mod shader;
pub mod specialized;
pub mod validate;
pub mod widgets;
