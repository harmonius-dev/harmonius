//! Deterministic stack fingerprinting (`R-14.4.3`).

use blake3::Hasher;

/// Opaque fingerprint for clustering.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StackFingerprint(pub [u8; 32]);

/// Normalizes a single symbolicated frame name per design rules.
#[must_use]
pub fn normalize_frame_name(frame: &str) -> String {
    let mut s = strip_line_numbers(frame);
    s = strip_template_args(&s);
    s = strip_param_lists(&s);
    s
}

/// Drops frames that belong to the Harmonius crash stub namespace.
#[must_use]
pub fn drop_handler_frames(frames: &[String]) -> Vec<String> {
    frames
        .iter()
        .filter(|f| !f.contains("harmonius::crash::"))
        .cloned()
        .collect()
}

/// Computes a deterministic fingerprint for `frames` (already normalized or not).
#[must_use]
pub fn fingerprint_frames(frames: &[String]) -> StackFingerprint {
    let mut hasher = Hasher::new();
    for frame in frames {
        hasher.update(frame.as_bytes());
        hasher.update(b"\n");
    }
    StackFingerprint(*hasher.finalize().as_bytes())
}

fn strip_line_numbers(frame: &str) -> String {
    // Replace `:digits:` with `:` (symbol server style line number stripping).
    let mut out = String::with_capacity(frame.len());
    let bytes = frame.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b':' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b':' {
                out.push(':');
                i = j + 1;
                continue;
            }
        }
        out.push(char::from(bytes[i]));
        i += 1;
    }
    out
}

fn strip_template_args(frame: &str) -> String {
    // Very small deterministic normalizer: strip `<...>` segments recursively from innermost.
    let mut s = frame.to_owned();
    while let Some(open) = s.rfind('<') {
        if let Some(close) = s[open..].find('>') {
            let close = open + close;
            s.replace_range(open..=close, "");
        } else {
            break;
        }
    }
    s
}

fn strip_param_lists(frame: &str) -> String {
    // Remove `( ... )` parameter lists at the end of each path segment.
    let mut out = String::with_capacity(frame.len());
    let mut depth = 0i32;
    for ch in frame.chars() {
        match ch {
            '(' => depth += 1,
            ')' => depth = (depth - 1).max(0),
            _ if depth == 0 => out.push(ch),
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_normalizes_line_numbers() {
        let a = vec![
            normalize_frame_name("foo::bar:12:baz"),
            normalize_frame_name("main:99:start"),
        ];
        let b = vec![
            normalize_frame_name("foo::bar:999:baz"),
            normalize_frame_name("main:1:start"),
        ];
        assert_eq!(a, b);
        assert_eq!(fingerprint_frames(&a), fingerprint_frames(&b));
    }

    #[test]
    fn test_fingerprint_strips_param_lists() {
        let a = vec![normalize_frame_name("foo(i32)")];
        let b = vec![normalize_frame_name("foo(u32)")];
        assert_eq!(fingerprint_frames(&a), fingerprint_frames(&b));
    }

    #[test]
    fn test_fingerprint_strips_template_args() {
        let a = vec![normalize_frame_name("Vec<i32>::push")];
        let b = vec![normalize_frame_name("Vec<u8>::push")];
        assert_eq!(fingerprint_frames(&a), fingerprint_frames(&b));
    }

    #[test]
    fn test_fingerprint_drops_handler_frames() {
        let dirty = vec![
            "harmonius::crash::stub::on_signal".to_owned(),
            "game::update".to_owned(),
        ];
        let clean = drop_handler_frames(&dirty);
        assert_eq!(clean, vec!["game::update".to_owned()]);
        let fp_with_stub = fingerprint_frames(
            &dirty
                .iter()
                .map(|s| normalize_frame_name(s))
                .collect::<Vec<_>>(),
        );
        let fp_without_stub = fingerprint_frames(
            &clean
                .iter()
                .map(|s| normalize_frame_name(s))
                .collect::<Vec<_>>(),
        );
        assert_ne!(fp_with_stub, fp_without_stub);
    }
}
