//! Compression, encryption, checksumming, and atomic slot writes (R-13.3.6).

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::path::Path;

use aes_gcm::aead::Aead;
use aes_gcm::aead::KeyInit;
use aes_gcm::Aes256Gcm;
use aes_gcm::Nonce;
use crc32fast::Hasher as Crc32;

use crate::error::LoadError;
use crate::error::SaveError;
use crate::types::CompressionMode;
use crate::types::EncryptionConfig;
use crate::types::IoPriority;
use crate::types::KeySource;
use crate::types::SchemaVersion;
use crate::types::SlotId;
use crate::vfs::VirtualFileSystem;

const MAGIC: &[u8; 4] = b"HARM";
const HEADER_VERSION: u8 = 1;

const COMP_NONE: u8 = 0;
const COMP_LZ4: u8 = 1;
const COMP_ZSTD: u8 = 2;

/// Save pipeline: CRC-32 over plaintext, compress, encrypt, atomic write (design ordering).
#[derive(Clone, Debug)]
pub struct SavePipeline {
    compression: CompressionMode,
    encryption: EncryptionConfig,
    current_schema: SchemaVersion,
    /// Fixed AES-256 key when [`KeySource::FixedKey`] is active (tests / offline tooling).
    fixed_key: Option<[u8; 32]>,
}

impl SavePipeline {
    /// New pipeline with explicit engine schema and optional fixed key for tests.
    pub fn new(
        compression: CompressionMode,
        encryption: EncryptionConfig,
        current_schema: SchemaVersion,
        fixed_key: Option<[u8; 32]>,
    ) -> Self {
        Self {
            compression,
            encryption,
            current_schema,
            fixed_key,
        }
    }

    fn resolve_key(&self) -> Result<[u8; 32], SaveError> {
        match self.encryption.key_source {
            KeySource::FixedKey => self.fixed_key.ok_or(SaveError::EncryptionKeyUnavailable),
            KeySource::PlatformKeystore | KeySource::HardwareBound => {
                Err(SaveError::EncryptionKeyUnavailable)
            }
        }
    }

    fn compress(&self, plain: &[u8]) -> Result<Vec<u8>, SaveError> {
        match self.compression {
            CompressionMode::None => Ok(plain.to_vec()),
            CompressionMode::Lz4 => Ok(lz4_flex::block::compress_prepend_size(plain)),
            CompressionMode::Zstd { level } => {
                zstd::encode_all(plain, level).map_err(|e| SaveError::SerializationFailed {
                    entity: 0,
                    type_hash: 0,
                    detail: e.to_string(),
                })
            }
        }
    }

    fn decompress(
        &self,
        mode: u8,
        data: &[u8],
        uncompressed_size: u64,
    ) -> Result<Vec<u8>, LoadError> {
        match mode {
            COMP_NONE => Ok(data.to_vec()),
            COMP_LZ4 => lz4_flex::block::decompress_size_prepended(data).map_err(|e| {
                LoadError::DeserializationFailed {
                    detail: e.to_string(),
                }
            }),
            COMP_ZSTD => zstd::decode_all(data).map_err(|e| LoadError::DeserializationFailed {
                detail: e.to_string(),
            }),
            _ => Err(LoadError::InvalidHeader),
        }
        .and_then(|v| {
            if v.len() as u64 != uncompressed_size {
                return Err(LoadError::DeserializationFailed {
                    detail: format!(
                        "decompressed length {} does not match header {}",
                        v.len(),
                        uncompressed_size
                    ),
                });
            }
            Ok(v)
        })
    }

    /// Serialize, protect, and atomically write the save archive for `slot`.
    pub fn write(
        &self,
        _slot: SlotId,
        data: &[u8],
        _priority: IoPriority,
        vfs: &dyn VirtualFileSystem,
        save_path: &Path,
    ) -> Result<(), SaveError> {
        let key = self.resolve_key()?;
        let mut crc = Crc32::new();
        crc.update(data);
        let crc_val = crc.finalize();
        let compressed = self.compress(data)?;
        let comp_tag = match self.compression {
            CompressionMode::None => COMP_NONE,
            CompressionMode::Lz4 => COMP_LZ4,
            CompressionMode::Zstd { .. } => COMP_ZSTD,
        };
        let cipher =
            Aes256Gcm::new_from_slice(&key).map_err(|_| SaveError::EncryptionKeyUnavailable)?;
        let nonce_bytes: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, compressed.as_ref())
            .map_err(|_| SaveError::EncryptionKeyUnavailable)?;
        let mut out = Vec::new();
        out.extend_from_slice(MAGIC);
        out.push(HEADER_VERSION);
        out.extend_from_slice(&self.current_schema.major.to_le_bytes());
        out.extend_from_slice(&self.current_schema.minor.to_le_bytes());
        out.extend_from_slice(&self.current_schema.patch.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.push(comp_tag);
        out.extend_from_slice(&(data.len() as u64).to_le_bytes());
        out.extend_from_slice(&crc_val.to_le_bytes());
        out.extend_from_slice(&nonce_bytes);
        out.extend_from_slice(&ciphertext);
        vfs.write_atomic(save_path, &out)
            .map_err(SaveError::IoFailed)?;
        Ok(())
    }

    /// Read and verify a save archive for `slot`.
    pub fn read(
        &self,
        _slot: SlotId,
        vfs: &dyn VirtualFileSystem,
        save_path: &Path,
    ) -> Result<(SchemaVersion, Vec<u8>), LoadError> {
        let key = self.resolve_key_read()?;
        let blob = vfs.read_file(save_path).map_err(LoadError::IoFailed)?;
        if blob.len() < 4 + 1 + 6 + 4 + 1 + 8 + 4 + 12 {
            return Err(LoadError::InvalidHeader);
        }
        let mut i = 0usize;
        if &blob[i..i + 4] != MAGIC {
            return Err(LoadError::InvalidHeader);
        }
        i += 4;
        if blob[i] != HEADER_VERSION {
            return Err(LoadError::InvalidHeader);
        }
        i += 1;
        let saved = SchemaVersion {
            major: u16::from_le_bytes(
                blob[i..i + 2]
                    .try_into()
                    .map_err(|_| LoadError::InvalidHeader)?,
            ),
            minor: u16::from_le_bytes(
                blob[i + 2..i + 4]
                    .try_into()
                    .map_err(|_| LoadError::InvalidHeader)?,
            ),
            patch: u16::from_le_bytes(
                blob[i + 4..i + 6]
                    .try_into()
                    .map_err(|_| LoadError::InvalidHeader)?,
            ),
        };
        i += 6;
        if saved > self.current_schema {
            return Err(LoadError::ForwardCompatError {
                saved,
                current: self.current_schema,
            });
        }
        i += 4;
        let comp_tag = blob[i];
        i += 1;
        let uncompressed_size = u64::from_le_bytes(
            blob[i..i + 8]
                .try_into()
                .map_err(|_| LoadError::InvalidHeader)?,
        );
        i += 8;
        let expected_crc = u32::from_le_bytes(
            blob[i..i + 4]
                .try_into()
                .map_err(|_| LoadError::InvalidHeader)?,
        );
        i += 4;
        let nonce_bytes: [u8; 12] = blob[i..i + 12]
            .try_into()
            .map_err(|_| LoadError::InvalidHeader)?;
        i += 12;
        let ciphertext = &blob[i..];
        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| LoadError::DecryptionFailed)?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let compressed = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| LoadError::DecryptionFailed)?;
        let plain = self.decompress(comp_tag, &compressed, uncompressed_size)?;
        let mut crc = Crc32::new();
        crc.update(&plain);
        let actual = crc.finalize();
        if actual != expected_crc {
            return Err(LoadError::ChecksumMismatch {
                expected: expected_crc,
                actual,
            });
        }
        Ok((saved, plain))
    }

    fn resolve_key_read(&self) -> Result<[u8; 32], LoadError> {
        match self.encryption.key_source {
            KeySource::FixedKey => self.fixed_key.ok_or(LoadError::DecryptionFailed),
            KeySource::PlatformKeystore | KeySource::HardwareBound => {
                Err(LoadError::DecryptionFailed)
            }
        }
    }
}

/// Deterministic scheduler used to validate priority semantics (TC-13.3.6.3).
#[derive(Debug, Default)]
pub struct SaveIoScheduler {
    heap: BinaryHeap<ScheduledWrite>,
    seq: u64,
}

#[derive(Debug, Eq, PartialEq)]
struct ScheduledWrite {
    priority: IoPriority,
    seq: u64,
    slot: SlotId,
}

impl Ord for ScheduledWrite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| other.seq.cmp(&self.seq))
    }
}

impl PartialOrd for ScheduledWrite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SaveIoScheduler {
    /// Enqueue a write request.
    pub fn submit(&mut self, priority: IoPriority, slot: SlotId) {
        self.seq = self.seq.wrapping_add(1);
        self.heap.push(ScheduledWrite {
            priority,
            seq: self.seq,
            slot,
        });
    }

    /// Next slot to service (highest priority first, FIFO within priority).
    pub fn pop_next(&mut self) -> Option<SlotId> {
        self.heap.pop().map(|j| j.slot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vfs::StdVirtualFileSystem;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn pipe() -> SavePipeline {
        SavePipeline::new(
            CompressionMode::Lz4,
            EncryptionConfig {
                key_source: KeySource::FixedKey,
            },
            SchemaVersion {
                major: 1,
                minor: 0,
                patch: 0,
            },
            Some([7u8; 32]),
        )
    }

    /// TC-13.3.6.1 LZ4 + AES-256-GCM roundtrip.
    #[test]
    fn tc_13_3_6_1_compress_encrypt_roundtrip() {
        let dir = tempdir().unwrap();
        let vfs = StdVirtualFileSystem::new(dir.path().to_path_buf());
        let p = pipe();
        let payload = vec![0xABu8; 1024 * 1024];
        let path = PathBuf::from("t.save");
        p.write(SlotId(1), &payload, IoPriority::Normal, &vfs, &path)
            .unwrap();
        let disk = vfs.read_file(&path).unwrap();
        assert!(disk.len() < payload.len() + 64);
        let (_, got) = p.read(SlotId(1), &vfs, &path).unwrap();
        assert_eq!(got, payload);
    }

    /// TC-13.3.6.2 Atomic rename leaves no temp file (StdVirtualFileSystem).
    #[test]
    fn tc_13_3_6_2_atomic_rename_no_temp() {
        let dir = tempdir().unwrap();
        let vfs = StdVirtualFileSystem::new(dir.path().to_path_buf());
        let p = pipe();
        let path = PathBuf::from("slot.save");
        p.write(SlotId(1), b"x", IoPriority::Normal, &vfs, &path)
            .unwrap();
        let names = vfs.read_dir_names(Path::new(".")).unwrap();
        assert!(!names.iter().any(|n| n.ends_with(".tmp")));
        assert!(names.iter().any(|n| n == "slot.save"));
    }

    /// TC-13.3.6.3 Critical before Background.
    #[test]
    fn tc_13_3_6_3_priority_ordering() {
        let mut q = SaveIoScheduler::default();
        q.submit(IoPriority::Background, SlotId(1));
        q.submit(IoPriority::Critical, SlotId(2));
        assert_eq!(q.pop_next(), Some(SlotId(2)));
        assert_eq!(q.pop_next(), Some(SlotId(1)));
    }

    /// TC-13.3.6.4 LZ4 vs Zstd size tradeoff on 1 MiB pattern.
    #[test]
    fn tc_13_3_6_4_lz4_vs_zstd() {
        let data: Vec<u8> = (0..(1024 * 1024)).map(|i| (i % 251) as u8).collect();
        let c_lz4 = lz4_flex::block::compress_prepend_size(&data);
        let c_z = zstd::encode_all(data.as_slice(), 3).unwrap();
        assert!(
            c_z.len() < c_lz4.len(),
            "zstd should compress better on this pattern"
        );
    }

    /// TC-13.3.6.5 Wrong key yields decryption failure.
    #[test]
    fn tc_13_3_6_5_wrong_key() {
        let dir = tempdir().unwrap();
        let vfs = StdVirtualFileSystem::new(dir.path().to_path_buf());
        let writer = SavePipeline::new(
            CompressionMode::None,
            EncryptionConfig {
                key_source: KeySource::FixedKey,
            },
            SchemaVersion {
                major: 1,
                minor: 0,
                patch: 0,
            },
            Some([1u8; 32]),
        );
        let path = PathBuf::from("x.save");
        writer
            .write(SlotId(1), b"hi", IoPriority::Normal, &vfs, &path)
            .unwrap();
        let reader = SavePipeline::new(
            CompressionMode::None,
            EncryptionConfig {
                key_source: KeySource::FixedKey,
            },
            SchemaVersion {
                major: 1,
                minor: 0,
                patch: 0,
            },
            Some([2u8; 32]),
        );
        let err = reader.read(SlotId(1), &vfs, &path).expect_err("wrong key");
        assert!(matches!(err, LoadError::DecryptionFailed));
    }

    /// TC-13.3.2.9 Forward compatibility error when save schema is newer than engine.
    #[test]
    fn tc_13_3_2_9_forward_compat_error() {
        let dir = tempdir().unwrap();
        let vfs = StdVirtualFileSystem::new(dir.path().to_path_buf());
        let old_engine = SchemaVersion {
            major: 1,
            minor: 0,
            patch: 0,
        };
        let new_save = SchemaVersion {
            major: 9,
            minor: 0,
            patch: 0,
        };
        let writer = SavePipeline::new(
            CompressionMode::None,
            EncryptionConfig {
                key_source: KeySource::FixedKey,
            },
            new_save,
            Some([5u8; 32]),
        );
        let path = PathBuf::from("f.save");
        writer
            .write(SlotId(1), b"x", IoPriority::Normal, &vfs, &path)
            .unwrap();
        let reader = SavePipeline::new(
            CompressionMode::None,
            EncryptionConfig {
                key_source: KeySource::FixedKey,
            },
            old_engine,
            Some([5u8; 32]),
        );
        let err = reader
            .read(SlotId(1), &vfs, &path)
            .expect_err("forward compat");
        assert!(matches!(err, LoadError::ForwardCompatError { .. }));
    }
}
