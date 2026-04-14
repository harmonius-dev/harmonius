//! Schema revision negotiation (optional fields).

/// Monotonic schema revision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaRev(pub u32);

/// Server payload at rev N+1 with optional extension; client stays at N.
#[derive(Clone, Debug, PartialEq)]
pub struct ServerPayload {
    /// Base field always present at rev N.
    pub base: i32,
    /// Added at N+1.
    pub field_x: Option<i32>,
}

/// Decode on client at negotiated `client_rev`.
pub fn decode_with_optional_field(bytes: &[u8], client_rev: SchemaRev) -> (i32, Option<i32>) {
    if bytes.len() < 4 {
        return (0, None);
    }
    let base = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    if client_rev.0 >= 1 && bytes.len() >= 8 {
        let x = i32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        (base, Some(x))
    } else {
        (base, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.2.2.1 — older client ignores unknown optional tail.
    #[test]
    fn test_schema_version_negotiation() {
        let mut wire = Vec::new();
        wire.extend_from_slice(&42i32.to_le_bytes());
        wire.extend_from_slice(&7i32.to_le_bytes());
        let client = SchemaRev(0);
        let (base, opt) = decode_with_optional_field(&wire, client);
        assert_eq!(base, 42);
        assert_eq!(opt, None);
        let server = SchemaRev(1);
        let (base2, opt2) = decode_with_optional_field(&wire, server);
        assert_eq!(base2, 42);
        assert_eq!(opt2, Some(7));
    }
}
