//! On-disk shader variant bundle (header, sorted index, bytecode blob, footer CRC).

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crc32fast::Hasher as Crc32;
use memmap2::{Mmap, MmapOptions};

use crate::permutation::{LodLevel, PermutationKey, RenderPath, ShaderFeatures, ShadingModel};

/// Magic bytes for Harmonius shader pak v1 (`HMNS_SH` + NUL).
pub const BUNDLE_MAGIC: &[u8; 8] = b"HMNS_SH\0";

/// Supported bundle format version (increment on breaking layout changes).
pub const FORMAT_VERSION_V1: u32 = 1;

/// Bytecode API target for a [`VariantRecord`].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShaderApi {
    /// DirectX 12 / DXIL.
    D3D12 = 0,
    /// Vulkan SPIR-V.
    Vulkan = 1,
    /// Apple Metal IR.
    Metal = 2,
}

impl ShaderApi {
    fn from_u8(v: u8) -> Result<Self, VariantError> {
        match v {
            0 => Ok(Self::D3D12),
            1 => Ok(Self::Vulkan),
            2 => Ok(Self::Metal),
            _ => Err(VariantError::UnknownShaderApi(v)),
        }
    }
}

/// One compiled variant entry inside a bundle index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VariantRecord {
    /// Byte offset into the blob section (from start of file).
    pub bytecode_offset: u64,
    /// Byte length of bytecode.
    pub bytecode_size: u64,
    /// Build timestamp (opaque u64).
    pub compiled_at: u64,
    /// Target graphics API.
    pub api: ShaderApi,
}

/// Errors loading, compiling, or interpreting shader variant data.
#[derive(Debug)]
pub enum VariantError {
    /// Logical failure in resolver or compiler glue.
    Engine(&'static str),
    /// Underlying I/O failure.
    Io(std::io::Error),
    /// File magic or declared version does not match expectations.
    FormatMismatch {
        /// Context label.
        what: &'static str,
    },
    /// Index or blob layout is inconsistent with the file size.
    BadLayout(&'static str),
    /// Bytecode slice would read past the mapped file.
    OutOfBounds,
    /// Unknown API tag in an on-disk record.
    UnknownShaderApi(u8),
}

impl From<std::io::Error> for VariantError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

/// Memory-mapped variant bundle with sorted index for `O(log n)` lookup.
pub struct VariantBundle {
    mmap: Mmap,
    entries: Vec<(PermutationKey, VariantRecord)>,
}

impl core::fmt::Debug for VariantBundle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VariantBundle")
            .field("variant_count", &self.entries.len())
            .finish_non_exhaustive()
    }
}

impl VariantBundle {
    /// Opens and maps an existing bundle file (read-only).
    pub fn open_mmap(path: &Path) -> Result<Self, VariantError> {
        let file = File::open(path)?;
        // SAFETY: We map an existing file read-only; length matches the OS-reported file size.
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        Self::parse_mmap(mmap)
    }

    fn parse_mmap(mmap: Mmap) -> Result<Self, VariantError> {
        let len = mmap.len();
        if len < 24 + 12 {
            return Err(VariantError::BadLayout("truncated"));
        }
        let payload_end = len - 12;
        let file_len = read_u64_le(&mmap[payload_end..payload_end + 8]);
        if file_len as usize != len {
            return Err(VariantError::FormatMismatch { what: "file_len" });
        }
        let stored_crc = read_u32_le(&mmap[payload_end + 8..payload_end + 12]);
        let mut crc = Crc32::new();
        crc.update(&mmap[0..payload_end]);
        if crc.finalize() != stored_crc {
            return Err(VariantError::FormatMismatch { what: "crc" });
        }
        if &mmap[0..8] != BUNDLE_MAGIC.as_slice() {
            return Err(VariantError::FormatMismatch { what: "magic" });
        }
        let ver = read_u32_le(&mmap[8..12]);
        if ver != FORMAT_VERSION_V1 {
            return Err(VariantError::FormatMismatch { what: "version" });
        }
        let n = read_u32_le(&mmap[12..16]) as usize;
        const HDR: usize = 24;
        const ROW: usize = 32;
        let index_end = HDR
            .checked_add(n.checked_mul(ROW).ok_or(VariantError::BadLayout("index"))?)
            .ok_or(VariantError::BadLayout("index end"))?;
        if index_end > payload_end {
            return Err(VariantError::BadLayout("index past payload"));
        }
        let mut entries = Vec::with_capacity(n);
        let mut off = HDR;
        for _ in 0..n {
            let key = read_key(&mmap[off..off + 7])?;
            let bytecode_offset = read_u64_le(&mmap[off + 7..off + 15]);
            let bytecode_size = read_u64_le(&mmap[off + 15..off + 23]);
            let compiled_at = read_u64_le(&mmap[off + 23..off + 31]);
            let api = ShaderApi::from_u8(mmap[off + 31])?;
            off += ROW;
            entries.push((
                key,
                VariantRecord {
                    bytecode_offset,
                    bytecode_size,
                    compiled_at,
                    api,
                },
            ));
        }
        for (k, r) in &entries {
            let start = r.bytecode_offset as usize;
            let end = start
                .checked_add(r.bytecode_size as usize)
                .ok_or(VariantError::OutOfBounds)?;
            if start < index_end || end > payload_end {
                return Err(VariantError::OutOfBounds);
            }
            let _ = mmap.get(end - 1).ok_or(VariantError::OutOfBounds)?;
            let _ = k;
        }
        Ok(Self { mmap, entries })
    }

    /// Binary search for `key`.
    #[must_use]
    pub fn get_record(&self, key: &PermutationKey) -> Option<&VariantRecord> {
        self.entries
            .binary_search_by_key(key, |(k, _)| *k)
            .ok()
            .map(|i| &self.entries[i].1)
    }

    /// Bytecode view for a record (bounds-checked against the mmap).
    #[must_use]
    pub fn slice(&self, record: &VariantRecord) -> &[u8] {
        let start = record.bytecode_offset as usize;
        let end = start.saturating_add(record.bytecode_size as usize);
        &self.mmap[start..end]
    }

    /// Number of indexed variants.
    #[must_use]
    pub fn variant_count(&self) -> usize {
        self.entries.len()
    }
}

/// Writes a bundle file (sorted index, contiguous blobs, CRC footer).
pub struct VariantBundleWriter {
    api: ShaderApi,
    items: Vec<(PermutationKey, Vec<u8>, u64)>,
}

impl VariantBundleWriter {
    /// Starts a writer for the given API target.
    #[must_use]
    pub fn new(api: ShaderApi) -> Self {
        Self {
            api,
            items: Vec::new(),
        }
    }

    /// Adds a variant (sorted by key at write time).
    pub fn push_variant(&mut self, key: PermutationKey, bytecode: Vec<u8>, compiled_at: u64) {
        self.items.push((key, bytecode, compiled_at));
    }

    /// Serializes to `path` (overwrites).
    pub fn write_to_path(&self, path: &Path) -> Result<(), VariantError> {
        let mut items = self.items.clone();
        items.sort_by(|a, b| a.0.cmp(&b.0));
        let n = items.len() as u32;
        const HDR: usize = 24;
        const ROW: usize = 32;
        let index_bytes = (n as usize)
            .checked_mul(ROW)
            .ok_or(VariantError::BadLayout("index"))?;
        let index_end = HDR
            .checked_add(index_bytes)
            .ok_or(VariantError::BadLayout("index end"))?;
        let mut out = Vec::new();
        out.extend_from_slice(BUNDLE_MAGIC.as_slice());
        out.extend_from_slice(&FORMAT_VERSION_V1.to_le_bytes());
        out.extend_from_slice(&n.to_le_bytes());
        out.extend_from_slice(&0u64.to_le_bytes());
        debug_assert_eq!(out.len(), HDR);
        out.resize(index_end, 0);
        let mut blob_off = index_end as u64;
        for (i, (key, code, ts)) in items.iter().enumerate() {
            let row = HDR + i * ROW;
            write_key_into(&mut out[row..row + 7], *key);
            out[row + 7..row + 15].copy_from_slice(&blob_off.to_le_bytes());
            out[row + 15..row + 23].copy_from_slice(&(code.len() as u64).to_le_bytes());
            out[row + 23..row + 31].copy_from_slice(&ts.to_le_bytes());
            out[row + 31] = self.api as u8;
            blob_off = blob_off
                .checked_add(code.len() as u64)
                .ok_or(VariantError::BadLayout("blob"))?;
        }
        for (_, code, _) in items {
            out.extend_from_slice(&code);
        }
        let payload_len = out.len();
        let mut crc = Crc32::new();
        crc.update(&out);
        let checksum = crc.finalize();
        let total_len = (payload_len + 12) as u64;
        out.extend_from_slice(&total_len.to_le_bytes());
        out.extend_from_slice(&checksum.to_le_bytes());
        let mut f = File::create(path)?;
        f.write_all(&out)?;
        Ok(())
    }
}

fn write_key_into(dst: &mut [u8], key: PermutationKey) {
    dst[0] = key.shading_model as u8;
    dst[1..5].copy_from_slice(&key.features.bits.to_le_bytes());
    dst[5] = key.render_path as u8;
    dst[6] = key.lod as u8;
}

fn read_key(buf: &[u8]) -> Result<PermutationKey, VariantError> {
    if buf.len() < 7 {
        return Err(VariantError::BadLayout("key"));
    }
    let sm =
        ShadingModel::try_from(buf[0]).map_err(|_| VariantError::BadLayout("shading_model"))?;
    let bits = read_u32_le(&buf[1..5]);
    let rp = RenderPath::try_from(buf[5]).map_err(|_| VariantError::BadLayout("render_path"))?;
    let lod = LodLevel::try_from(buf[6]).map_err(|_| VariantError::BadLayout("lod"))?;
    Ok(PermutationKey {
        shading_model: sm,
        features: ShaderFeatures { bits },
        render_path: rp,
        lod,
    })
}

impl TryFrom<u8> for ShadingModel {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (value as usize) < Self::VARIANTS.len() {
            Ok(Self::VARIANTS[value as usize])
        } else {
            Err(())
        }
    }
}

impl TryFrom<u8> for RenderPath {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (value as usize) < Self::VARIANTS.len() {
            Ok(Self::VARIANTS[value as usize])
        } else {
            Err(())
        }
    }
}

impl TryFrom<u8> for LodLevel {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (value as usize) < Self::VARIANTS.len() {
            Ok(Self::VARIANTS[value as usize])
        } else {
            Err(())
        }
    }
}

fn read_u32_le(b: &[u8]) -> u32 {
    u32::from_le_bytes(b.try_into().unwrap())
}

fn read_u64_le(b: &[u8]) -> u64 {
    u64::from_le_bytes(b.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permutation::{LodLevel, RenderPath, ShaderFeatures, ShadingModel};
    use tempfile::tempdir;

    fn sample_key(i: u32) -> PermutationKey {
        PermutationKey {
            shading_model: ShadingModel::Lit,
            features: ShaderFeatures { bits: i },
            render_path: RenderPath::Forward,
            lod: LodLevel::High,
        }
    }

    #[test]
    fn test_bundle_roundtrip_mmap() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("s.pak");
        let mut w = VariantBundleWriter::new(ShaderApi::D3D12);
        for i in 0..5u32 {
            w.push_variant(sample_key(i), vec![i as u8, 2, 3], i as u64);
        }
        w.write_to_path(&p).unwrap();
        let b = VariantBundle::open_mmap(&p).unwrap();
        for i in 0..5u32 {
            let rec = b.get_record(&sample_key(i)).unwrap();
            assert_eq!(b.slice(rec), &[i as u8, 2, 3]);
        }
    }

    #[test]
    fn test_bundle_format_version_check() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("bad.pak");
        let mut payload = Vec::new();
        payload.extend_from_slice(BUNDLE_MAGIC);
        payload.extend_from_slice(&999u32.to_le_bytes());
        payload.extend_from_slice(&0u32.to_le_bytes());
        payload.extend_from_slice(&0u64.to_le_bytes());
        let payload_len = payload.len();
        let mut c = Crc32::new();
        c.update(&payload);
        let checksum = c.finalize();
        let total = (payload_len + 12) as u64;
        payload.extend_from_slice(&total.to_le_bytes());
        payload.extend_from_slice(&checksum.to_le_bytes());
        std::fs::write(&p, payload).unwrap();
        let err = VariantBundle::open_mmap(&p).unwrap_err();
        match err {
            VariantError::FormatMismatch { what: "version" } => {}
            e => panic!("unexpected {e:?}"),
        }
    }
}
