//! Socket vocabulary for modular assembly.

/// Socket identifier for authored prefab rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketKind {
    /// Wide door opening.
    DoorWide,
    /// Small window opening.
    WindowSmall,
}

/// Assembly failures for modular graphs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketError {
    /// Mismatched socket categories.
    SocketTypeMismatch,
}

/// Validates a socket pairing.
pub fn validate_socket_connection(a: SocketKind, b: SocketKind) -> Result<(), SocketError> {
    if a == SocketKind::DoorWide && b == SocketKind::WindowSmall {
        return Err(SocketError::SocketTypeMismatch);
    }
    if a == SocketKind::WindowSmall && b == SocketKind::DoorWide {
        return Err(SocketError::SocketTypeMismatch);
    }
    Ok(())
}

/// Resolves world translation for piece `b` when snapped to `a`'s outgoing socket offset.
pub fn resolve_socket_transform(offset_a: glam::Vec3) -> glam::Vec3 {
    offset_a
}
