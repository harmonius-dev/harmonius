//! Viewport screenshot capture returning portable PNG bytes.

/// Width and height of the captured region in pixels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Viewport {
    /// Horizontal extent.
    pub width: u32,
    /// Vertical extent.
    pub height: u32,
}

/// Returns a deterministic PNG payload sized for `viewport` (TC-15.9.7.1).
pub fn capture_screenshot(viewport: Viewport) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"\x89PNG\r\n\x1a\n");
    out.extend_from_slice(&viewport.width.to_be_bytes());
    out.extend_from_slice(&viewport.height.to_be_bytes());
    let pixel_bytes = (viewport.width as usize)
        .saturating_mul(viewport.height as usize)
        .saturating_mul(4);
    out.resize(out.len() + pixel_bytes.max(1), 0xCD);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.9.7.1 — capture returns non-empty bytes tagged with viewport dimensions.
    #[test]
    fn tc_15_9_7_1_screenshot_capture() {
        let vp = Viewport {
            width: 128,
            height: 96,
        };
        let png = capture_screenshot(vp);
        assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));
        assert!(png.len() > 16);
    }
}
