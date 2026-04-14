//! Ed25519 signature verification and coarse trust levels.

use crate::manifest::manifest_signing_digest;
use crate::types::{Blake3Hash, PublisherId};
use ed25519_dalek::{Signature as DalekSignature, Verifier, VerifyingKey};
use ed25519_dalek::{Signer, SigningKey};
use std::collections::HashMap;

/// Wire-format signature attached to a package (publisher key + detached sig).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature {
    /// Declared publisher ed25519 public key (32 bytes).
    pub publisher_key: [u8; 32],
    /// Detached signature over the manifest digest (64 bytes).
    pub sig: [u8; 64],
}

/// Trust tier assigned after cryptographic and policy checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrustLevel {
    /// First-party Harmonius publisher key.
    Official,
    /// Known marketplace publisher with valid signature.
    Verified,
    /// Registered community publisher.
    Community,
    /// Unknown publisher or local-only policy bucket.
    Unsigned,
}

/// Errors from trust evaluation.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum TrustError {
    /// Signature bytes did not verify for the given manifest bytes.
    #[error("invalid signature")]
    InvalidSignature,
}

/// Registered publisher identity and expected signing key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PublisherEntry {
    verifying_key: [u8; 32],
    level: TrustLevel,
}

/// Publisher registry used to classify keys after successful verification.
#[derive(Clone, Debug, Default)]
pub struct TrustStore {
    /// Known publishers and their registered ed25519 verifying keys.
    publishers: HashMap<PublisherId, PublisherEntry>,
    /// Baked-in official verifying keys (first-party builds).
    official_keys: Vec<[u8; 32]>,
}

impl TrustStore {
    /// Create an empty trust store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a publisher id, its expected ed25519 verifying key, and trust level.
    ///
    /// After [`Self::verify_with_author`], a manifest is promoted to `level` only when the
    /// detached signature was made with `verifying_key` and `author` matches `id`.
    pub fn register_publisher(
        &mut self,
        id: PublisherId,
        verifying_key: [u8; 32],
        level: TrustLevel,
    ) {
        self.publishers.insert(
            id,
            PublisherEntry {
                verifying_key,
                level,
            },
        );
    }

    /// Mark a raw public key as official (Harmonius team).
    pub fn add_official_key(&mut self, key: [u8; 32]) {
        self.official_keys.push(key);
    }

    /// Verify detached signature over `blake3(manifest_bytes)` and classify trust.
    pub fn verify(&self, manifest_bytes: &[u8], sig: &Signature) -> Result<TrustLevel, TrustError> {
        let digest = manifest_signing_digest(manifest_bytes);
        self.verify_digest(&digest, sig)
    }

    /// Verify a signature against an explicit manifest digest.
    pub fn verify_digest(
        &self,
        digest: &Blake3Hash,
        sig: &Signature,
    ) -> Result<TrustLevel, TrustError> {
        let vk = VerifyingKey::from_bytes(&sig.publisher_key)
            .map_err(|_| TrustError::InvalidSignature)?;
        let dalek_sig =
            DalekSignature::from_slice(&sig.sig).map_err(|_| TrustError::InvalidSignature)?;
        vk.verify(digest.0.as_ref(), &dalek_sig)
            .map_err(|_| TrustError::InvalidSignature)?;
        if self.official_keys.iter().any(|k| k == &sig.publisher_key) {
            return Ok(TrustLevel::Official);
        }
        // Unknown key: still cryptographically valid -> Unsigned per TC-15.17.8.3.
        Ok(TrustLevel::Unsigned)
    }

    /// Like [`Self::verify`], but if `author` matches a registered publisher **and**
    /// `sig.publisher_key` equals that publisher's registered key, return the registered level.
    pub fn verify_with_author(
        &self,
        manifest_bytes: &[u8],
        author: &PublisherId,
        sig: &Signature,
    ) -> Result<TrustLevel, TrustError> {
        let base = self.verify(manifest_bytes, sig)?;
        if matches!(base, TrustLevel::Official) {
            return Ok(TrustLevel::Official);
        }
        if let Some(entry) = self.publishers.get(author)
            && sig.publisher_key == entry.verifying_key
        {
            return Ok(entry.level);
        }
        Ok(base)
    }
}

/// Sign `blake3(manifest_bytes)` with a publisher signing key (tests and publishing tools).
pub fn sign_manifest(manifest_bytes: &[u8], signing_key: &SigningKey) -> Signature {
    let digest = blake3::hash(manifest_bytes);
    let sig = signing_key.sign(digest.as_bytes());
    Signature {
        publisher_key: signing_key.verifying_key().to_bytes(),
        sig: sig.to_bytes(),
    }
}
