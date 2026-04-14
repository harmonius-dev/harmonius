//! Console certification checklist (`R-14.5.7`).

/// One failed checklist row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertFailure {
    /// Stable error code string.
    pub code: String,
    /// Human-readable detail.
    pub message: String,
}

/// Certification outcome for a cooked build.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertificationReport {
    /// Failures (empty means pass).
    pub failures: Vec<CertFailure>,
}

/// Declares required certification artifacts for a build.
#[derive(Clone, Debug, Default)]
pub struct BuildCertInputs {
    /// Required metadata keys that must be present.
    pub required_keys: Vec<String>,
    /// Provided key/value declarations from the build.
    pub declarations: Vec<(String, String)>,
}

/// Evaluates first-party certification requirements.
#[derive(Debug, Default)]
pub struct CertificationValidator;

impl CertificationValidator {
    /// Validates `inputs`, producing a [`CertificationReport`].
    pub fn validate(&self, inputs: &BuildCertInputs) -> CertificationReport {
        let mut failures = Vec::new();
        for key in &inputs.required_keys {
            if !inputs
                .declarations
                .iter()
                .any(|(k, _)| k == key)
            {
                failures.push(CertFailure {
                    code: "MISSING_DECLARATION".into(),
                    message: format!("missing required item: {key}"),
                });
            }
        }
        CertificationReport { failures }
    }
}
