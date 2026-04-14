use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crc32fast::Hasher;

use crate::cache_error::CacheError;
use crate::device::DeviceFingerprint;
use crate::pso_key::{read_key_wire, write_key_wire, PsoKey, PSO_KEY_WIRE_LEN};

const INDEX_MAGIC: &[u8; 8] = b"HMNSPIDX";
const INDEX_VERSION: u32 = 3;
const RECORD_LEN: usize = PSO_KEY_WIRE_LEN + 8 + 8 + 8 + 8;

/// Metadata for a blob stored in `blobs.bin`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiskRecord {
    /// Byte offset into `blobs.bin` for the framed record.
    pub offset: u64,
    /// Total framed size on disk (length + crc + payload).
    pub size: u64,
    /// CRC32 over the raw payload bytes.
    pub checksum: u64,
    /// Last recorded use tick.
    pub last_used_tick: u64,
}

#[derive(Clone, Debug)]
struct InternalRecord {
    key: PsoKey,
    offset: u64,
    size: u64,
    checksum: u64,
    last_used_tick: u64,
}

/// On-disk PSO blob index rooted at a device fingerprint directory.
#[derive(Debug)]
pub struct DiskIndex {
    device_dir: PathBuf,
    records: Vec<InternalRecord>,
    blobs_len: u64,
}

impl DiskIndex {
    /// Active device cache directory (`$CACHE/v#/fingerprint`).
    #[must_use]
    pub fn device_dir(&self) -> &Path {
        &self.device_dir
    }

    /// Returns the versioned cache directory for `root`.
    #[must_use]
    pub fn versioned_root(root: &Path, format_version: u32) -> PathBuf {
        root.join(format!("v{format_version}"))
    }

    /// Returns the fingerprint-specific subdirectory.
    #[must_use]
    pub fn fingerprint_dir(root: &Path, format_version: u32, fp: &DeviceFingerprint) -> PathBuf {
        Self::versioned_root(root, format_version).join(fp.directory_name())
    }

    /// Opens or creates the on-disk index under `root`.
    pub fn open(
        root: &Path,
        format_version: u32,
        fp: &DeviceFingerprint,
    ) -> Result<Self, CacheError> {
        let device_dir = Self::fingerprint_dir(root, format_version, fp);
        fs::create_dir_all(&device_dir)?;
        let tmp_dir = device_dir.join(format!("tmp-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir_all(&tmp_dir)?;

        let index_path = device_dir.join("index.bin");
        let blobs_path = device_dir.join("blobs.bin");

        if !blobs_path.exists() {
            File::create(&blobs_path)?;
        }

        let mut records = Vec::new();
        let blobs_len = fs::metadata(&blobs_path).map(|m| m.len()).unwrap_or(0);

        if index_path.exists() {
            let mut file = File::open(&index_path)?;
            let mut header = [0u8; 20];
            let read = file.read(&mut header)?;
            if read != header.len() {
                drop(file);
                Self::reset_corrupted_dir(&device_dir)?;
                return Self::open(root, format_version, fp);
            }
            if &header[0..8] != INDEX_MAGIC {
                Self::reset_corrupted_dir(&device_dir)?;
                return Self::open(root, format_version, fp);
            }
            let version = read_u32_le(&header, 8)?;
            if version != INDEX_VERSION {
                return Err(CacheError::VersionMismatch);
            }
            let count = read_u32_le(&header, 12)? as usize;
            let expected_crc = read_u32_le(&header, 16)?;

            let mut body = vec![0u8; count.saturating_mul(RECORD_LEN)];
            file.read_exact(&mut body)?;
            let actual_crc = crc32_slice(&body);
            if actual_crc != expected_crc {
                Self::reset_corrupted_dir(&device_dir)?;
                return Self::open(root, format_version, fp);
            }

            let mut cursor = 0usize;
            for _ in 0..count {
                let end = cursor + RECORD_LEN;
                let slice = body
                    .get(cursor..end)
                    .ok_or(CacheError::Corrupted("index record"))?;
                let mut key_buf = [0u8; PSO_KEY_WIRE_LEN];
                key_buf.copy_from_slice(&slice[0..PSO_KEY_WIRE_LEN]);
                let key = read_key_wire(&key_buf);
                let offset = u64::from_le_bytes(
                    slice[PSO_KEY_WIRE_LEN..PSO_KEY_WIRE_LEN + 8]
                        .try_into()
                        .map_err(|_| CacheError::Corrupted("offset"))?,
                );
                let size = u64::from_le_bytes(
                    slice[PSO_KEY_WIRE_LEN + 8..PSO_KEY_WIRE_LEN + 16]
                        .try_into()
                        .map_err(|_| CacheError::Corrupted("size"))?,
                );
                let checksum = u64::from_le_bytes(
                    slice[PSO_KEY_WIRE_LEN + 16..PSO_KEY_WIRE_LEN + 24]
                        .try_into()
                        .map_err(|_| CacheError::Corrupted("checksum"))?,
                );
                let last_used_tick = u64::from_le_bytes(
                    slice[PSO_KEY_WIRE_LEN + 24..RECORD_LEN]
                        .try_into()
                        .map_err(|_| CacheError::Corrupted("tick"))?,
                );
                records.push(InternalRecord {
                    key,
                    offset,
                    size,
                    checksum,
                    last_used_tick,
                });
                cursor = end;
            }
        }

        records.sort_by(|a, b| a.key.cmp(&b.key));

        Ok(Self {
            device_dir,
            records,
            blobs_len,
        })
    }

    fn reset_corrupted_dir(device_dir: &Path) -> Result<(), CacheError> {
        if device_dir.exists() {
            fs::remove_dir_all(device_dir)?;
        }
        fs::create_dir_all(device_dir)?;
        File::create(device_dir.join("blobs.bin"))?;
        Ok(())
    }

    /// Reads a payload for `key` when checksums match.
    pub fn read(&self, key: &PsoKey) -> Result<Vec<u8>, CacheError> {
        let idx = self
            .records
            .binary_search_by_key(key, |rec| rec.key)
            .map_err(|_| CacheError::Missing)?;
        let rec = &self.records[idx];
        let payload = Self::read_framed_payload(&self.device_dir, rec.offset)?;
        let actual = u64::from(crc32_slice(&payload));
        if actual != rec.checksum {
            return Err(CacheError::Corrupted("blob checksum"));
        }
        Ok(payload)
    }

    /// Appends a blob for `key` and refreshes the index atomically.
    pub fn write(
        &mut self,
        key: &PsoKey,
        blob: &[u8],
        last_used_tick: u64,
    ) -> Result<(), CacheError> {
        let checksum = u64::from(crc32_slice(blob));
        let blobs_path = self.device_dir.join("blobs.bin");
        let mut file = OpenOptions::new().append(true).open(&blobs_path)?;
        let offset = file.metadata()?.len();
        let len_u32 =
            u32::try_from(blob.len()).map_err(|_| CacheError::Corrupted("blob too large"))?;
        let crc = crc32_slice(blob);
        let framed_size = 4 + 4 + blob.len();
        file.write_all(&len_u32.to_le_bytes())?;
        file.write_all(&crc.to_le_bytes())?;
        file.write_all(blob)?;
        file.sync_all()?;

        match self.records.binary_search_by_key(key, |rec| rec.key) {
            Ok(idx) => {
                self.records[idx].offset = offset;
                self.records[idx].size = framed_size as u64;
                self.records[idx].checksum = checksum;
                self.records[idx].last_used_tick = last_used_tick;
            }
            Err(idx) => {
                self.records.insert(
                    idx,
                    InternalRecord {
                        key: *key,
                        offset,
                        size: framed_size as u64,
                        checksum,
                        last_used_tick,
                    },
                );
            }
        }

        self.blobs_len = fs::metadata(&blobs_path)?.len();
        self.flush_index()?;
        Ok(())
    }

    /// Removes entries older than `cutoff_tick`, returning the number removed.
    pub fn purge_stale(&mut self, cutoff_tick: u64) -> u32 {
        let before = self.records.len();
        self.records.retain(|rec| rec.last_used_tick >= cutoff_tick);
        (before - self.records.len()) as u32
    }

    /// Compacts `blobs.bin` so its byte length equals the framed sum of remaining entries.
    pub fn compact_blob_file(&mut self) -> Result<(), CacheError> {
        if self.records.is_empty() {
            let path = self.device_dir.join("blobs.bin");
            fs::write(&path, [])?;
            self.blobs_len = 0;
            self.flush_index()?;
            return Ok(());
        }

        let mut sorted = self.records.clone();
        sorted.sort_by_key(|rec| rec.offset);

        let mut new_blob = Vec::new();
        let mut mapping: Vec<(PsoKey, DiskRecord)> = Vec::new();

        for rec in sorted {
            let payload = Self::read_framed_payload(&self.device_dir, rec.offset)?;
            let offset = new_blob.len() as u64;
            let len_u32 =
                u32::try_from(payload.len()).map_err(|_| CacheError::Corrupted("blob"))?;
            let crc = crc32_slice(&payload);
            new_blob.extend_from_slice(&len_u32.to_le_bytes());
            new_blob.extend_from_slice(&crc.to_le_bytes());
            new_blob.extend_from_slice(&payload);
            let framed = (4 + 4 + payload.len()) as u64;
            mapping.push((
                rec.key,
                DiskRecord {
                    offset,
                    size: framed,
                    checksum: rec.checksum,
                    last_used_tick: rec.last_used_tick,
                },
            ));
        }

        let path = self.device_dir.join("blobs.bin");
        let tmp = self
            .device_dir
            .join(format!("tmp-{}", std::process::id()))
            .join("blobs.next");
        if let Some(parent) = tmp.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&tmp, &new_blob)?;
        fs::rename(&tmp, &path)?;
        self.blobs_len = new_blob.len() as u64;

        self.records.clear();
        for (key, disk) in mapping {
            self.records.push(InternalRecord {
                key,
                offset: disk.offset,
                size: disk.size,
                checksum: disk.checksum,
                last_used_tick: disk.last_used_tick,
            });
        }
        self.records.sort_by(|a, b| a.key.cmp(&b.key));
        self.flush_index()?;
        Ok(())
    }

    /// Total on-disk bytes for framed blobs.
    #[must_use]
    pub fn total_blob_bytes(&self) -> u64 {
        self.blobs_len
    }

    fn live_blob_bytes(&self) -> u64 {
        self.records.iter().map(|rec| rec.size).sum()
    }

    /// Removes a single key from the index (used when a blob is corrupt).
    pub fn remove_key(&mut self, key: &PsoKey) -> Result<(), CacheError> {
        if let Ok(idx) = self.records.binary_search_by_key(key, |rec| rec.key) {
            self.records.remove(idx);
            self.flush_index()?;
        }
        Ok(())
    }

    /// Deletes scratch directories created for atomic writes.
    pub fn cleanup_tmp(&self) -> Result<(), CacheError> {
        for entry in fs::read_dir(&self.device_dir)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            if name.starts_with("tmp-") {
                let _ = fs::remove_dir_all(entry.path());
            }
        }
        Ok(())
    }

    /// Drops least-recently-used records until framed blob storage is within `cap_bytes`.
    pub fn gc_disk_lru(&mut self, cap_bytes: u64) -> Result<u32, CacheError> {
        let mut removed = 0u32;
        while self.live_blob_bytes() > cap_bytes && !self.records.is_empty() {
            let Some((pos, _)) = self
                .records
                .iter()
                .enumerate()
                .min_by_key(|(_, rec)| rec.last_used_tick)
            else {
                break;
            };
            self.records.remove(pos);
            removed = removed.saturating_add(1);
        }
        self.compact_blob_file()?;
        Ok(removed)
    }

    fn read_framed_payload(device_dir: &Path, offset: u64) -> Result<Vec<u8>, CacheError> {
        let mut file = File::open(device_dir.join("blobs.bin"))?;
        file.seek(SeekFrom::Start(offset))?;
        let mut len_buf = [0u8; 4];
        file.read_exact(&mut len_buf)?;
        let len = u32::from_le_bytes(len_buf) as usize;
        let mut crc_buf = [0u8; 4];
        file.read_exact(&mut crc_buf)?;
        let expected = u32::from_le_bytes(crc_buf);
        let mut payload = vec![0u8; len];
        file.read_exact(&mut payload)?;
        let actual = crc32_slice(&payload);
        if actual != expected {
            return Err(CacheError::Corrupted("blob checksum"));
        }
        Ok(payload)
    }

    fn flush_index(&self) -> Result<(), CacheError> {
        let mut body = Vec::new();
        for rec in &self.records {
            let mut key_buf = [0u8; PSO_KEY_WIRE_LEN];
            write_key_wire(&rec.key, &mut key_buf);
            body.extend_from_slice(&key_buf);
            body.extend_from_slice(&rec.offset.to_le_bytes());
            body.extend_from_slice(&rec.size.to_le_bytes());
            body.extend_from_slice(&rec.checksum.to_le_bytes());
            body.extend_from_slice(&rec.last_used_tick.to_le_bytes());
        }
        let crc = crc32_slice(&body);
        let mut header = [0u8; 20];
        header[0..8].copy_from_slice(INDEX_MAGIC);
        header[8..12].copy_from_slice(&INDEX_VERSION.to_le_bytes());
        header[12..16].copy_from_slice(&(self.records.len() as u32).to_le_bytes());
        header[16..20].copy_from_slice(&crc.to_le_bytes());

        let tmp_root = self.device_dir.join(format!("tmp-{}", std::process::id()));
        fs::create_dir_all(&tmp_root)?;
        let tmp_path = tmp_root.join("index.next");
        let mut file = File::create(&tmp_path)?;
        file.write_all(&header)?;
        file.write_all(&body)?;
        file.sync_all()?;
        fs::rename(&tmp_path, self.device_dir.join("index.bin"))?;
        let _ = fs::remove_dir_all(&tmp_root);
        Ok(())
    }
}

fn crc32_slice(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

fn read_u32_le(buf: &[u8], offset: usize) -> Result<u32, CacheError> {
    let slice = buf
        .get(offset..offset + 4)
        .ok_or(CacheError::Corrupted("short buffer"))?;
    let arr: [u8; 4] = slice
        .try_into()
        .map_err(|_| CacheError::Corrupted("short buffer"))?;
    Ok(u32::from_le_bytes(arr))
}
