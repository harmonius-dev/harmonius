//! HBIN binary companion format (RF-7).

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// Magic bytes for companion files.
pub const COMPANION_MAGIC: [u8; 4] = *b"HBIN";

/// Current on-disk companion format version.
pub const COMPANION_VERSION: u32 = 1;

/// Compression selector for companion blobs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Compression {
    /// Store bytes verbatim.
    None,
    /// LZ4-compressed payload.
    Lz4,
}

impl Compression {
    fn tag(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Lz4 => 1,
        }
    }

    fn from_tag(tag: u8) -> Result<Self, std::io::Error> {
        match tag {
            0 => Ok(Self::None),
            1 => Ok(Self::Lz4),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("unknown compression {tag}"),
            )),
        }
    }
}

/// Fixed header at the start of every companion file.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CompanionHeader {
    /// Magic `HBIN`.
    pub magic: [u8; 4],
    /// Format version.
    pub version: u32,
    /// Byte offset of the table of contents from file start.
    pub toc_offset: u64,
    /// Number of [`TocEntry`] records.
    pub toc_count: u32,
    /// Reserved.
    pub _reserved: [u8; 12],
}

impl CompanionHeader {
    /// Wire size.
    pub const SIZE: usize = 32;

    fn new(toc_offset: u64, toc_count: u32) -> Self {
        Self {
            magic: COMPANION_MAGIC,
            version: COMPANION_VERSION,
            toc_offset,
            toc_count,
            _reserved: [0; 12],
        }
    }

    fn write_to<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        let mut b = [0u8; Self::SIZE];
        b[0..4].copy_from_slice(&self.magic);
        b[4..8].copy_from_slice(&self.version.to_le_bytes());
        b[8..16].copy_from_slice(&self.toc_offset.to_le_bytes());
        b[16..20].copy_from_slice(&self.toc_count.to_le_bytes());
        b[20..32].copy_from_slice(&self._reserved);
        w.write_all(&b)
    }

    fn read_from<R: Read>(r: &mut R) -> std::io::Result<Self> {
        let mut b = [0u8; Self::SIZE];
        r.read_exact(&mut b)?;
        Ok(Self {
            magic: b[0..4].try_into().unwrap(),
            version: u32::from_le_bytes(b[4..8].try_into().unwrap()),
            toc_offset: u64::from_le_bytes(b[8..16].try_into().unwrap()),
            toc_count: u32::from_le_bytes(b[16..20].try_into().unwrap()),
            _reserved: b[20..32].try_into().unwrap(),
        })
    }
}

/// One companion blob entry.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct TocEntry {
    /// BLAKE3 hash of the blob name.
    pub name_hash: [u8; 32],
    /// Byte offset from file start.
    pub offset: u64,
    /// Uncompressed length.
    pub uncompressed_len: u64,
    /// Stored length (compressed or not).
    pub compressed_len: u64,
    /// See [`Compression::tag`].
    pub compression: u8,
    /// BLAKE3 of uncompressed bytes.
    pub content_hash: [u8; 32],
    /// Padding.
    pub _pad: [u8; 7],
}

impl TocEntry {
    const RECORD: usize = 32 + 8 + 8 + 8 + 1 + 32 + 7;

    fn write_to<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        let mut buf = [0u8; Self::RECORD];
        let mut o = 0;
        buf[o..o + 32].copy_from_slice(&self.name_hash);
        o += 32;
        buf[o..o + 8].copy_from_slice(&self.offset.to_le_bytes());
        o += 8;
        buf[o..o + 8].copy_from_slice(&self.uncompressed_len.to_le_bytes());
        o += 8;
        buf[o..o + 8].copy_from_slice(&self.compressed_len.to_le_bytes());
        o += 8;
        buf[o] = self.compression;
        o += 1;
        buf[o..o + 32].copy_from_slice(&self.content_hash);
        o += 32;
        buf[o..o + 7].copy_from_slice(&self._pad);
        w.write_all(&buf)
    }

    fn read_from<R: Read>(r: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; Self::RECORD];
        r.read_exact(&mut buf)?;
        let mut o = 0;
        let name_hash: [u8; 32] = buf[o..o + 32].try_into().unwrap();
        o += 32;
        let offset = u64::from_le_bytes(buf[o..o + 8].try_into().unwrap());
        o += 8;
        let uncompressed_len = u64::from_le_bytes(buf[o..o + 8].try_into().unwrap());
        o += 8;
        let compressed_len = u64::from_le_bytes(buf[o..o + 8].try_into().unwrap());
        o += 8;
        let compression = buf[o];
        o += 1;
        let content_hash: [u8; 32] = buf[o..o + 32].try_into().unwrap();
        o += 32;
        let _pad: [u8; 7] = buf[o..o + 7].try_into().unwrap();
        Ok(Self {
            name_hash,
            offset,
            uncompressed_len,
            compressed_len,
            compression,
            content_hash,
            _pad,
        })
    }
}

fn name_key(name: &str) -> [u8; 32] {
    *blake3::hash(name.as_bytes()).as_bytes()
}

fn content_key(bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(bytes).as_bytes()
}

#[derive(Clone, Debug)]
struct StoredBlob {
    name: String,
    uncompressed: Vec<u8>,
    compression: Compression,
    stored: Vec<u8>,
    content_hash: [u8; 32],
    name_hash: [u8; 32],
}

impl StoredBlob {
    fn new(name: &str, data: &[u8], compression: Compression) -> Self {
        let uncompressed = data.to_vec();
        let content_hash = content_key(&uncompressed);
        let name_hash = name_key(name);
        let stored = match compression {
            Compression::None => uncompressed.clone(),
            Compression::Lz4 => lz4_flex::block::compress_prepend_size(&uncompressed),
        };
        Self {
            name: name.to_string(),
            uncompressed,
            compression,
            stored,
            content_hash,
            name_hash,
        }
    }

    fn decompress(&self) -> std::io::Result<Vec<u8>> {
        match self.compression {
            Compression::None => Ok(self.uncompressed.clone()),
            Compression::Lz4 => lz4_flex::block::decompress_size_prepended(&self.stored)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }
}

/// Append-friendly binary companion container.
#[derive(Debug)]
pub struct BinaryCompanion {
    path: PathBuf,
    blobs: Vec<StoredBlob>,
}

impl BinaryCompanion {
    /// Opens an existing companion or creates a new empty file.
    pub fn open(path: PathBuf) -> std::io::Result<Self> {
        if path.exists() {
            let mut f = File::open(&path)?;
            let hdr = CompanionHeader::read_from(&mut f)?;
            if hdr.magic != COMPANION_MAGIC {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "bad companion magic",
                ));
            }
            f.seek(SeekFrom::Start(hdr.toc_offset))?;
            let mut blobs = Vec::new();
            for _ in 0..hdr.toc_count {
                let e = TocEntry::read_from(&mut f)?;
                let mut nlen_bytes = [0u8; 2];
                f.read_exact(&mut nlen_bytes)?;
                let nlen = u16::from_le_bytes(nlen_bytes) as usize;
                let mut name_buf = vec![0u8; nlen];
                f.read_exact(&mut name_buf)?;
                let name = String::from_utf8(name_buf)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                f.seek(SeekFrom::Start(e.offset))?;
                let mut stored = vec![0u8; e.compressed_len as usize];
                f.read_exact(&mut stored)?;
                let compression = Compression::from_tag(e.compression)?;
                let mut s = StoredBlob {
                    name,
                    uncompressed: Vec::new(),
                    compression,
                    stored,
                    content_hash: e.content_hash,
                    name_hash: e.name_hash,
                };
                s.uncompressed = s.decompress()?;
                blobs.push(s);
            }
            Ok(Self { path, blobs })
        } else {
            Ok(Self {
                path,
                blobs: Vec::new(),
            })
        }
    }

    /// Writes `data` under `name`, skipping duplicate storage when the content hash matches.
    pub fn write_blob(
        &mut self,
        name: &str,
        data: &[u8],
        compression: Compression,
        _align: usize,
    ) -> std::io::Result<()> {
        let h = content_key(data);
        if self.blobs.iter().any(|b| b.content_hash == h) {
            return Ok(());
        }
        self.blobs.push(StoredBlob::new(name, data, compression));
        self.flush()
    }

    /// Appends a blob after existing content (same dedup rules as [`Self::write_blob`]).
    pub fn append_blob(
        &mut self,
        name: &str,
        data: &[u8],
        compression: Compression,
        align: usize,
    ) -> std::io::Result<()> {
        self.write_blob(name, data, compression, align)
    }

    /// Reads a blob by logical name.
    pub fn read_blob(&self, name: &str) -> std::io::Result<Vec<u8>> {
        let b =
            self.blobs.iter().find(|b| b.name == name).ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "blob not found")
            })?;
        b.decompress()
    }

    /// Lists blob names in insertion order.
    pub fn list_blobs(&self) -> Vec<String> {
        self.blobs.iter().map(|b| b.name.clone()).collect()
    }

    /// Verifies every stored blob against its content hash.
    pub fn verify(&self) -> std::io::Result<()> {
        for b in &self.blobs {
            if content_key(&b.uncompressed) != b.content_hash {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "content hash mismatch",
                ));
            }
        }
        Ok(())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut body: Vec<u8> = Vec::new();
        let mut entries = Vec::new();
        let mut cursor = CompanionHeader::SIZE as u64;
        for b in &self.blobs {
            let offset = cursor;
            let uncompressed_len = b.uncompressed.len() as u64;
            let compressed_len = b.stored.len() as u64;
            body.extend_from_slice(&b.stored);
            cursor += compressed_len;
            entries.push((
                TocEntry {
                    name_hash: b.name_hash,
                    offset,
                    uncompressed_len,
                    compressed_len,
                    compression: b.compression.tag(),
                    content_hash: b.content_hash,
                    _pad: [0; 7],
                },
                b.name.clone(),
            ));
        }
        let toc_offset = cursor;
        let mut tail: Vec<u8> = Vec::new();
        for (e, name) in &entries {
            e.write_to(&mut tail)?;
            let nb = name.as_bytes();
            let nlen: u16 = nb.len().try_into().map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "name too long")
            })?;
            tail.extend_from_slice(&nlen.to_le_bytes());
            tail.extend_from_slice(nb);
        }
        let hdr = CompanionHeader::new(toc_offset, entries.len() as u32);
        let mut out = File::create(&self.path)?;
        hdr.write_to(&mut out)?;
        out.write_all(&body)?;
        out.write_all(&tail)?;
        out.sync_all()
    }
}

/// Writes `text` to `final_path` via `temp_path`, then atomically renames.
pub fn atomic_write_file(temp_path: &Path, final_path: &Path, text: &[u8]) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(temp_path)?;
    f.write_all(text)?;
    f.sync_all()?;
    drop(f);
    std::fs::rename(temp_path, final_path)
}
