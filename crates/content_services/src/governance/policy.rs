//! Signed enterprise policy documents with monotonic version enforcement.

use blake3::Hasher;

use super::error::GovernanceError;

/// Serialized policy bytes plus an integrity digest checked before toggle updates.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyDocument {
    /// Monotonic generation counter across the organization.
    pub version: u64,
    /// Wire payload: `[generative_on, assistance_on]` for the stub codec.
    pub payload: Vec<u8>,
    /// BLAKE3-256 digest over `secret || version_le || payload`.
    pub signature: [u8; 32],
}

/// Computes the policy signature used by `PolicyDocument::sign`.
pub fn policy_signature(secret: &[u8; 8], version: u64, payload: &[u8]) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(secret);
    hasher.update(&version.to_le_bytes());
    hasher.update(payload);
    *hasher.finalize().as_bytes()
}

impl PolicyDocument {
    /// Builds a policy with a correct signature for the given secret and flags.
    pub fn sign(secret: &[u8; 8], version: u64, generative: bool, assistance: bool) -> Self {
        let payload = vec![u8::from(generative), u8::from(assistance)];
        let signature = policy_signature(secret, version, &payload);
        Self {
            version,
            payload,
            signature,
        }
    }

    /// Returns `InvalidSignature` when bytes disagree with the digest.
    pub fn verify(&self, secret: &[u8; 8]) -> Result<(), GovernanceError> {
        let expected = policy_signature(secret, self.version, &self.payload);
        if expected != self.signature {
            return Err(GovernanceError::InvalidSignature);
        }
        Ok(())
    }
}
