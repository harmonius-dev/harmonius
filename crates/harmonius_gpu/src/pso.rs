//! Pipeline state pre-validation (R-2.1.3).

/// Shader stage slot identifiers for signature checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShaderSlot {
    /// Location index.
    pub location: u32,
}

/// Minimal shader interface description.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShaderInterface {
    /// Outputs from vertex stage.
    pub outputs: Vec<ShaderSlot>,
}

/// Pair of shader interfaces under validation.
#[derive(Clone, Debug)]
pub struct PipelineDesc {
    /// Vertex stage outputs.
    pub vertex_out: ShaderInterface,
    /// Fragment stage expected inputs (locations consumed).
    pub fragment_in: ShaderInterface,
}

/// Failure modes for PSO construction prior to GPU object creation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PsoBuildError {
    /// Vertex outputs do not cover fragment inputs.
    SignatureMismatch,
}

/// Validates that every fragment input location exists in vertex outputs.
pub fn validate_pipeline_interfaces(desc: &PipelineDesc) -> Result<(), PsoBuildError> {
    for slot in &desc.fragment_in.outputs {
        if !desc
            .vertex_out
            .outputs
            .iter()
            .any(|o| o.location == slot.location)
        {
            return Err(PsoBuildError::SignatureMismatch);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-2.1.3.1 — mismatched VS/FS interface is rejected before GPU creation.
    #[test]
    fn test_pso_invalid_combination() {
        let desc = PipelineDesc {
            vertex_out: ShaderInterface {
                outputs: vec![ShaderSlot { location: 0 }],
            },
            fragment_in: ShaderInterface {
                outputs: vec![ShaderSlot { location: 1 }],
            },
        };
        assert_eq!(
            validate_pipeline_interfaces(&desc),
            Err(PsoBuildError::SignatureMismatch)
        );
    }
}
