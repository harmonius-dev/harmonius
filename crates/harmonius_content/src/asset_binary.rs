//! Universal binary asset format (`HAST` sections + TOC).

use std::collections::HashMap;
use std::io::Write as _;

/// Magic bytes for Harmonius binary assets.
pub const ASSET_MAGIC: [u8; 4] = *b"HAST";
/// Supported container format version.
pub const FORMAT_VERSION: u32 = 1;

const HEADER_LEN: usize = 68;

/// On-disk header (fixed layout for tests).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetHeader {
    /// Magic.
    pub magic: [u8; 4],
    /// Format version.
    pub format_version: u32,
    /// User type id.
    pub asset_type_id: u64,
    /// BLAKE3 of body (everything after this header).
    pub content_hash: [u8; 32],
    /// Byte offset of TOC from start of file.
    pub toc_offset: u64,
    /// Number of TOC entries.
    pub toc_count: u32,
    /// Total file size.
    pub total_size: u64,
}

#[derive(Clone, Debug)]
struct WriterSection {
    name: String,
    data: Vec<u8>,
}

/// Incremental asset writer.
#[derive(Debug)]
pub struct AssetWriter {
    asset_type_id: u64,
    sections: Vec<WriterSection>,
}

/// Parse / section lookup errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssetError {
    /// Bad magic or layout.
    InvalidFormat(&'static str),
    /// Unknown section name.
    UnknownSection(String),
}

impl AssetWriter {
    /// New writer for `asset_type_id`.
    pub fn new(asset_type_id: u64) -> Self {
        Self {
            asset_type_id,
            sections: Vec::new(),
        }
    }

    /// Append a named section (uncompressed payload).
    pub fn add_section(&mut self, name: &str, data: Vec<u8>) -> &mut Self {
        self.sections.push(WriterSection {
            name: name.into(),
            data,
        });
        self
    }

    /// Serialize to bytes.
    pub fn build(self) -> Result<Vec<u8>, AssetError> {
        let mut sections_blob = Vec::new();
        let mut toc_entries: Vec<(String, u64, u32)> = Vec::new();
        for s in &self.sections {
            let off = (HEADER_LEN + sections_blob.len()) as u64;
            let len = s.data.len() as u32;
            toc_entries.push((s.name.clone(), off, len));
            sections_blob.extend_from_slice(&s.data);
        }
        let toc_offset = (HEADER_LEN + sections_blob.len()) as u64;
        let mut toc_bytes = Vec::new();
        for (name, off, len) in &toc_entries {
            toc_bytes.extend_from_slice(&(name.len() as u32).to_le_bytes());
            toc_bytes.extend_from_slice(name.as_bytes());
            toc_bytes.extend_from_slice(&off.to_le_bytes());
            toc_bytes.extend_from_slice(&len.to_le_bytes());
        }
        let body: Vec<u8> = sections_blob
            .into_iter()
            .chain(toc_bytes.iter().copied())
            .collect();
        let content_hash = *blake3::hash(&body).as_bytes();
        let total_size = (HEADER_LEN + body.len()) as u64;
        let toc_count = toc_entries.len() as u32;
        let mut out = Vec::with_capacity(HEADER_LEN + body.len());
        out.write_all(&ASSET_MAGIC).unwrap();
        out.write_all(&FORMAT_VERSION.to_le_bytes()).unwrap();
        out.write_all(&self.asset_type_id.to_le_bytes()).unwrap();
        out.write_all(&content_hash).unwrap();
        out.write_all(&toc_offset.to_le_bytes()).unwrap();
        out.write_all(&toc_count.to_le_bytes()).unwrap();
        out.write_all(&total_size.to_le_bytes()).unwrap();
        out.write_all(&body).unwrap();
        Ok(out)
    }
}

/// Zero-copy reader over borrowed bytes.
pub struct AssetReader<'a> {
    data: &'a [u8],
    /// Parsed header fields.
    pub header: AssetHeader,
    toc: HashMap<String, (u64, u32)>,
}

impl<'a> AssetReader<'a> {
    /// Parse header and TOC index.
    pub fn from_bytes(data: &'a [u8]) -> Result<Self, AssetError> {
        if data.len() < HEADER_LEN {
            return Err(AssetError::InvalidFormat("truncated"));
        }
        let magic: [u8; 4] = data[0..4].try_into().unwrap();
        if magic != ASSET_MAGIC {
            return Err(AssetError::InvalidFormat("magic"));
        }
        let format_version = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let asset_type_id = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let mut content_hash = [0u8; 32];
        content_hash.copy_from_slice(&data[16..48]);
        let toc_offset = u64::from_le_bytes(data[48..56].try_into().unwrap());
        let toc_count = u32::from_le_bytes(data[56..60].try_into().unwrap());
        let total_size = u64::from_le_bytes(data[60..68].try_into().unwrap());
        let header = AssetHeader {
            magic,
            format_version,
            asset_type_id,
            content_hash,
            toc_offset,
            toc_count,
            total_size,
        };
        let toc_slice = data
            .get(toc_offset as usize..)
            .ok_or(AssetError::InvalidFormat("toc"))?;
        let mut toc = HashMap::new();
        let mut p = 0usize;
        for _ in 0..toc_count {
            if p + 4 > toc_slice.len() {
                return Err(AssetError::InvalidFormat("toc entry"));
            }
            let n = u32::from_le_bytes(toc_slice[p..p + 4].try_into().unwrap()) as usize;
            p += 4;
            if p + n + 12 > toc_slice.len() {
                return Err(AssetError::InvalidFormat("toc name"));
            }
            let name = std::str::from_utf8(&toc_slice[p..p + n])
                .map_err(|_| AssetError::InvalidFormat("utf8"))?;
            p += n;
            let off = u64::from_le_bytes(toc_slice[p..p + 8].try_into().unwrap());
            p += 8;
            let len = u32::from_le_bytes(toc_slice[p..p + 4].try_into().unwrap());
            p += 4;
            toc.insert(name.to_string(), (off, len));
        }
        Ok(Self { data, header, toc })
    }

    /// O(1) TOC lookup then slice.
    pub fn section(&self, name: &str) -> Result<&'a [u8], AssetError> {
        let (off, len) = self
            .toc
            .get(name)
            .ok_or_else(|| AssetError::UnknownSection(name.into()))?;
        let start = *off as usize;
        let end = start + *len as usize;
        self.data
            .get(start..end)
            .ok_or(AssetError::InvalidFormat("section bounds"))
    }

    /// Recompute BLAKE3 over body and compare header hash.
    pub fn verify_integrity(&self) -> Result<(), AssetError> {
        let body = self
            .data
            .get(HEADER_LEN..)
            .ok_or(AssetError::InvalidFormat("no body"))?;
        let actual = blake3::hash(body);
        if actual.as_bytes() != &self.header.content_hash {
            return Err(AssetError::InvalidFormat("hash"));
        }
        Ok(())
    }
}
