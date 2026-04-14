//! Local tool registry with synchronous execution (editor cold path).

use std::collections::HashMap;

/// Declarative registration entry for a callable tool (MCP bridge omitted in this slice).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolDefinition {
    /// Stable tool identifier (e.g. `"place_mesh"`).
    pub id: String,
}

/// Runtime invocation with opaque argument payload (serialized at higher layers).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolInvocation {
    /// Target tool id.
    pub tool_id: String,
    /// Opaque argument blob for the handler (tests use sentinel values).
    pub args: String,
}

/// Successful tool execution output (minimal stub).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolResult {
    /// Human- or machine-readable payload returned to the assistant.
    pub output: String,
}

/// Tool dispatch failures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ToolError {
    /// Unknown tool id.
    NotFound,
    /// ACL denied execution (reserved for later wiring).
    Denied,
}

/// Access control snapshot passed into `execute` (stub).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ToolAccessControl;

/// In-memory registry backing synchronous `execute` (TC-15.9.2.1 / TC-15.9.2.2).
#[derive(Debug)]
pub struct ToolInterface {
    /// Registered tool ids mapped to canned success outputs for the stub implementation.
    outputs: HashMap<String, String>,
}

impl ToolInterface {
    /// Empty registry.
    pub fn new() -> Self {
        Self {
            outputs: HashMap::new(),
        }
    }

    /// Registers a tool locally (MCP publish omitted in this slice).
    pub fn register_tool(&mut self, definition: ToolDefinition) {
        self.outputs
            .insert(definition.id.clone(), "mesh_placed".to_string());
    }

    /// Dispatches a tool call synchronously.
    pub fn execute(
        &self,
        call: ToolInvocation,
        _acl: &ToolAccessControl,
    ) -> Result<ToolResult, ToolError> {
        let output = self
            .outputs
            .get(&call.tool_id)
            .ok_or(ToolError::NotFound)?
            .clone();
        let _ = call.args;
        Ok(ToolResult { output })
    }
}

impl Default for ToolInterface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.9.2.1 — tool registration enables dispatch.
    #[test]
    fn tc_15_9_2_1_tool_registration() {
        let mut tools = ToolInterface::new();
        tools.register_tool(ToolDefinition {
            id: "place_mesh".to_string(),
        });
        let result = tools.execute(
            ToolInvocation {
                tool_id: "place_mesh".to_string(),
                args: String::new(),
            },
            &ToolAccessControl,
        );
        assert_eq!(
            result,
            Ok(ToolResult {
                output: "mesh_placed".to_string(),
            })
        );
    }

    /// TC-15.9.2.2 — registered tool returns structured success.
    #[test]
    fn tc_15_9_2_2_tool_execution() {
        let mut tools = ToolInterface::new();
        tools.register_tool(ToolDefinition {
            id: "place_mesh".to_string(),
        });
        let result = tools.execute(
            ToolInvocation {
                tool_id: "place_mesh".to_string(),
                args: "{}".to_string(),
            },
            &ToolAccessControl,
        );
        assert_eq!(
            result,
            Ok(ToolResult {
                output: "mesh_placed".to_string(),
            })
        );
    }
}
