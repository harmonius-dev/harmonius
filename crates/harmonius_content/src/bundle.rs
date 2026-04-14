//! LZ4-compressed multi-asset bundle with selective decode stats.

/// Counts decompression work (test observable).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BundleDecodeStats {
    /// How many LZ4 chunks were decompressed.
    pub chunks_decompressed: u32,
}

/// Build-only writer.
#[derive(Debug, Default)]
pub struct BundleWriter {
    entries: Vec<Vec<u8>>,
}

impl BundleWriter {
    /// Empty bundle.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append entry (will be LZ4 compressed in final blob).
    pub fn push_entry(&mut self, data: Vec<u8>) {
        self.entries.push(data);
    }

    /// Serialize: u32 count, then per entry u32 len + lz4 payload.
    pub fn finish(self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for e in self.entries {
            let compressed = lz4_flex::block::compress_prepend_size(&e);
            out.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
            out.extend_from_slice(&compressed);
        }
        out
    }
}

/// Read bundle and extract one entry without touching others' payloads.
pub struct BundleReader<'a> {
    data: &'a [u8],
    offsets: Vec<(usize, usize)>,
}

impl<'a> BundleReader<'a> {
    /// Parse bundle bytes.
    pub fn new(data: &'a [u8]) -> Result<Self, &'static str> {
        if data.len() < 4 {
            return Err("short");
        }
        let n = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
        let mut i = 4usize;
        let mut offsets = Vec::with_capacity(n);
        for _ in 0..n {
            if i + 4 > data.len() {
                return Err("bad len");
            }
            let len = u32::from_le_bytes(data[i..i + 4].try_into().unwrap()) as usize;
            i += 4;
            if i + len > data.len() {
                return Err("bad payload");
            }
            offsets.push((i, len));
            i += len;
        }
        Ok(Self { data, offsets })
    }

    /// Decompress only entry `index`; updates `stats`.
    pub fn extract(
        &self,
        index: usize,
        stats: &mut BundleDecodeStats,
    ) -> Result<Vec<u8>, &'static str> {
        let (off, len) = *self.offsets.get(index).ok_or("oob")?;
        let slice = self.data.get(off..off + len).ok_or("slice")?;
        stats.chunks_decompressed += 1;
        lz4_flex::block::decompress_size_prepended(slice).map_err(|_| "lz4")
    }
}
