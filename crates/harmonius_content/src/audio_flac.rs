//! FLAC metadata block scan for Vorbis comment loop tags.

/// Parse `LOOPSTART` / `LOOPEND` from embedded Vorbis comment block (metadata type 4).
pub fn parse_flac_loop_comments(bytes: &[u8]) -> Result<(Option<u32>, Option<u32>), &'static str> {
    if bytes.len() < 4 || &bytes[0..4] != b"fLaC" {
        return Err("not flac");
    }
    let mut i = 4usize;
    let mut loop_start = None;
    let mut loop_end = None;
    while i + 4 <= bytes.len() {
        let hdr = bytes[i];
        let last = (hdr & 0x80) != 0;
        let block_type = hdr & 0x7F;
        let len = u24(&bytes[i + 1..i + 4]);
        i += 4;
        let end = i + len as usize;
        if end > bytes.len() {
            return Err("truncated meta");
        }
        let payload = &bytes[i..end];
        if block_type == 4 {
            // Vorbis comment: vendor len + string, then repeated user comments
            if payload.len() < 4 {
                break;
            }
            let vn = u32::from_le_bytes(payload[0..4].try_into().unwrap()) as usize;
            let mut p = 4 + vn;
            while p + 4 <= payload.len() {
                let sl = u32::from_le_bytes(payload[p..p + 4].try_into().unwrap()) as usize;
                p += 4;
                if p + sl > payload.len() {
                    break;
                }
                let s = std::str::from_utf8(&payload[p..p + sl]).map_err(|_| "utf8")?;
                p += sl;
                if let Some(v) = s.strip_prefix("LOOPSTART=") {
                    loop_start = v.parse().ok();
                } else if let Some(v) = s.strip_prefix("LOOPEND=") {
                    loop_end = v.parse().ok();
                }
            }
        }
        i = end;
        if last {
            break;
        }
    }
    Ok((loop_start, loop_end))
}

fn u24(b: &[u8]) -> u32 {
    u32::from(b[0]) | (u32::from(b[1]) << 8) | (u32::from(b[2]) << 16)
}
