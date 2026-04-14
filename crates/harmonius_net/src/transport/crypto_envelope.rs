//! TLS 1.3 inside QUIC owns production confidentiality. AES-GCM helpers here are **test-only**
//! doubles for TC-8.1.5.x; they are not a second wire encrypt path.

/// Key epoch for rotation tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyEpoch(pub u32);

/// Returns key epoch for linear message index `idx` with rotation after `rotate_at`.
pub fn epoch_for_message_index(idx: usize, rotate_at: usize) -> KeyEpoch {
    if idx >= rotate_at {
        KeyEpoch(1)
    } else {
        KeyEpoch(0)
    }
}

#[cfg(test)]
mod test_crypto {
    use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
    use aes_gcm::{Aes256Gcm, Key};

    use super::super::NetworkError;

    /// Encrypt `payload` with AES-256-GCM (tests only).
    pub fn test_transport_encrypt(
        key: &Key<Aes256Gcm>,
        payload: &[u8],
    ) -> Result<Vec<u8>, NetworkError> {
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let mut ct = cipher.encrypt(&nonce, payload).map_err(|_| {
            NetworkError::InvalidPacket {
                detail: "encrypt".into(),
            }
        })?;
        let mut out = nonce.to_vec();
        out.append(&mut ct);
        Ok(out)
    }

    /// Decrypt envelope from [`test_transport_encrypt`] (tests only).
    pub fn test_transport_decrypt(
        key: &Key<Aes256Gcm>,
        envelope: &[u8],
    ) -> Result<Vec<u8>, NetworkError> {
        if envelope.len() < 12 {
            return Err(NetworkError::InvalidPacket {
                detail: "truncated nonce".into(),
            });
        }
        let cipher = Aes256Gcm::new(key);
        let (n, c) = envelope.split_at(12);
        let nonce = aes_gcm::Nonce::from_slice(n);
        cipher.decrypt(nonce, c).map_err(|_| NetworkError::InvalidPacket {
            detail: "decrypt auth".into(),
        })
    }
}

#[cfg(test)]
pub use test_crypto::{test_transport_decrypt, test_transport_encrypt};

#[cfg(test)]
mod tests {
    use aes_gcm::aead::{KeyInit, OsRng};
    use aes_gcm::Aes256Gcm;

    use super::*;

    /// TC-8.1.5.1 — ciphertext differs; round-trip preserves payload; auth tag inside ct.
    #[test]
    fn test_dtls_encrypt_decrypt() {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        let payload = b"abilitycast(7)";
        let ct = test_transport_encrypt(&key, payload).expect("encrypt");
        assert_ne!(ct.as_slice(), payload);
        let plain = test_transport_decrypt(&key, &ct).expect("decrypt");
        assert_eq!(plain.as_slice(), payload);
    }

    /// TC-8.1.5.2 — rotate key mid-stream; all messages decrypt under correct epoch key.
    #[test]
    fn test_dtls_key_rotation() {
        let k0 = Aes256Gcm::generate_key(&mut OsRng);
        let k1 = Aes256Gcm::generate_key(&mut OsRng);
        let mut envelopes = Vec::new();
        for i in 0..1000 {
            let epoch = epoch_for_message_index(i, 500);
            let key = if epoch.0 == 0 { &k0 } else { &k1 };
            envelopes.push(test_transport_encrypt(key, &[i as u8]).expect("enc"));
        }
        for i in 0..1000 {
            let epoch = epoch_for_message_index(i, 500);
            let key = if epoch.0 == 0 { &k0 } else { &k1 };
            let p = test_transport_decrypt(key, &envelopes[i]).expect("dec");
            assert_eq!(p, vec![i as u8]);
        }
    }
}
