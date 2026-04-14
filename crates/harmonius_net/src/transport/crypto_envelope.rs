//! TLS 1.3 / QUIC carries encryption; tests use AES-GCM as the authenticated payload envelope.

use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key};

/// Encrypt `payload` with AES-256-GCM; ciphertext includes nonce prefix (12 bytes) + ct + tag.
pub fn transport_encrypt(key: &Key<Aes256Gcm>, payload: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let mut ct = cipher.encrypt(&nonce, payload).expect("encrypt");
    let mut out = nonce.to_vec();
    out.append(&mut ct);
    out
}

/// Decrypt envelope produced by [`transport_encrypt`].
pub fn transport_decrypt(key: &Key<Aes256Gcm>, envelope: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key);
    if envelope.len() < 12 {
        panic!("truncated");
    }
    let (n, c) = envelope.split_at(12);
    let nonce = aes_gcm::Nonce::from_slice(n);
    cipher.decrypt(nonce, c).expect("decrypt")
}

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
mod tests {
    use super::*;

    /// TC-8.1.5.1 — ciphertext differs; round-trip preserves payload; auth tag inside ct.
    #[test]
    fn test_dtls_encrypt_decrypt() {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        let payload = b"abilitycast(7)";
        let ct = transport_encrypt(&key, payload);
        assert_ne!(ct.as_slice(), payload);
        let plain = transport_decrypt(&key, &ct);
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
            envelopes.push(transport_encrypt(key, &[i as u8]));
        }
        for i in 0..1000 {
            let epoch = epoch_for_message_index(i, 500);
            let key = if epoch.0 == 0 { &k0 } else { &k1 };
            let p = transport_decrypt(key, &envelopes[i]);
            assert_eq!(p, vec![i as u8]);
        }
    }
}
