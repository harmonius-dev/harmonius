//! Packaging, signing, certification, and delta patching (R-15.14.*).

/// Target platform for asset cooking.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CookPlatform {
    /// Desktop or mobile Windows.
    Windows,
    /// Apple macOS.
    MacOs,
    /// Linux desktop.
    Linux,
}

/// Dispatch result for asset cooking (TC-15.14.1.1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CookDispatch {
    /// DirectX family path.
    D3d,
    /// Metal family path.
    Metal,
    /// Vulkan family path.
    Vulkan,
}

/// Selects cook path for a platform.
#[must_use]
pub(crate) fn cook_dispatch(platform: CookPlatform) -> CookDispatch {
    match platform {
        CookPlatform::Windows => CookDispatch::D3d,
        CookPlatform::MacOs => CookDispatch::Metal,
        CookPlatform::Linux => CookDispatch::Vulkan,
    }
}

/// BLAKE3 digest over bundle manifest bytes (TC-15.14.1.2).
#[must_use]
pub(crate) fn bundle_manifest_hash(manifest: &[u8]) -> [u8; 32] {
    *blake3::hash(manifest).as_bytes()
}

/// Certification rule outcome (TC-15.14.3.1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CertVerdict {
    /// Rule satisfied.
    Pass,
    /// Rule violated.
    Fail,
}

/// Evaluates a trivial compliance rule: executable must be signed flag true.
#[must_use]
pub(crate) fn cert_check_signed(is_signed: bool) -> CertVerdict {
    if is_signed {
        CertVerdict::Pass
    } else {
        CertVerdict::Fail
    }
}

/// Code signing backend dispatch (TC-15.14.4.1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SigningTool {
    /// Apple `codesign`.
    Codesign,
    /// Windows `signtool`.
    Signtool,
    /// Linux `osslsigncode` or similar.
    Osslsigncode,
}

/// Picks signing tool for a cook platform.
#[must_use]
pub(crate) fn signing_dispatch(platform: CookPlatform) -> SigningTool {
    match platform {
        CookPlatform::MacOs => SigningTool::Codesign,
        CookPlatform::Windows => SigningTool::Signtool,
        CookPlatform::Linux => SigningTool::Osslsigncode,
    }
}

/// Installer packaging format (TC-15.14.5.1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InstallerFormat {
    /// macOS disk image.
    Dmg,
    /// Windows installer.
    Msi,
    /// Linux AppImage.
    AppImage,
}

/// Selects installer format for a platform.
#[must_use]
pub(crate) fn installer_format(platform: CookPlatform) -> InstallerFormat {
    match platform {
        CookPlatform::MacOs => InstallerFormat::Dmg,
        CookPlatform::Windows => InstallerFormat::Msi,
        CookPlatform::Linux => InstallerFormat::AppImage,
    }
}

/// DLC entitlement gate (TC-15.14.6.1).
#[must_use]
pub(crate) fn dlc_allowed(owns_base: bool, owns_dlc: bool) -> bool {
    owns_base && owns_dlc
}

/// Store identifiers for validation (TC-15.14.8.1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StoreKind {
    /// Valve Steam.
    Steam,
    /// Apple App Store.
    AppStore,
    /// Microsoft Store.
    MicrosoftStore,
}

/// Validates store configuration string token.
#[must_use]
pub(crate) fn validate_store_config(token: &str) -> Option<StoreKind> {
    match token {
        "steam" => Some(StoreKind::Steam),
        "appstore" => Some(StoreKind::AppStore),
        "msstore" => Some(StoreKind::MicrosoftStore),
        _ => None,
    }
}

/// Content-defined chunk boundaries (indices are end-exclusive) (TC-15.14.7.1).
#[must_use]
pub(crate) fn cdc_chunk_end_indices(data: &[u8], min: usize, max: usize) -> Vec<usize> {
    let mut ends = Vec::new();
    if data.is_empty() {
        return ends;
    }
    let mut start = 0usize;
    let mut h: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut i = 0usize;
    while i < data.len() {
        h = h.rotate_left(5).wrapping_add(u64::from(data[i]));
        let len = i + 1 - start;
        let cut = len >= min && (h & 0x3F == 0 || len >= max);
        if cut || i + 1 == data.len() {
            ends.push(i + 1);
            start = i + 1;
            h = 0x9E37_79B9_7F4A_7C15;
        }
        i += 1;
    }
    ends
}

/// Patch bundle: copy ranges from old plus literal tail (TC-15.14.7.2).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PatchBundle {
    /// `(offset,len)` slices copied from the old artifact.
    pub old_slices: Vec<(usize, usize)>,
    /// Literal suffix appended after copies.
    pub tail: Vec<u8>,
}

/// Builds a patch when `new` extends `old` with a literal suffix (TC-15.14.7.2).
#[must_use]
pub(crate) fn build_patch(old: &[u8], new: &[u8], _chunk: usize) -> PatchBundle {
    if new.starts_with(old) {
        return PatchBundle {
            old_slices: vec![(0, old.len())],
            tail: new.get(old.len()..).unwrap_or_default().to_vec(),
        };
    }
    PatchBundle {
        old_slices: Vec::new(),
        tail: new.to_vec(),
    }
}

/// Applies a patch by reading old bytes then appending tail (TC-15.14.7.3).
#[must_use]
pub(crate) fn apply_patch(old: &[u8], patch: &PatchBundle) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    for (off, len) in &patch.old_slices {
        let end = off.checked_add(*len)?;
        let slice = old.get(*off..end)?;
        out.extend_from_slice(slice);
    }
    out.extend_from_slice(&patch.tail);
    Some(out)
}

/// Verifies patched output matches expected BLAKE3 (TC-15.14.7.3).
#[must_use]
pub(crate) fn verify_patched(old: &[u8], patch: &PatchBundle, expected_new: &[u8]) -> bool {
    let Some(got) = apply_patch(old, patch) else {
        return false;
    };
    got == expected_new
}

/// Lists chunk hashes that differ between local and remote (TC-15.14.2.1).
#[must_use]
pub(crate) fn incremental_needed_hashes(local: &[u8], remote: &[u8], chunk: usize) -> Vec<[u8; 32]> {
    let mut need = Vec::new();
    let mut i = 0usize;
    while i < remote.len() {
        let e = (i + chunk).min(remote.len());
        let rh = *blake3::hash(&remote[i..e]).as_bytes();
        let lh = if i < local.len() {
            let le = (i + chunk).min(local.len());
            *blake3::hash(&local[i..le]).as_bytes()
        } else {
            [0u8; 32]
        };
        if lh != rh {
            need.push(rh);
        }
        i = e;
    }
    need
}

/// Host/target pair for build matrix dispatch (TC-15.14.9.1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct HostTarget {
    /// Host cook platform.
    pub host: CookPlatform,
    /// Target cook platform.
    pub target: CookPlatform,
}

/// Returns true when cross-compilation requires remote farm (TC-15.14.9.1).
#[must_use]
pub(crate) fn host_target_requires_farm(ht: HostTarget) -> bool {
    ht.host as u8 != ht.target as u8
}

#[cfg(test)]
mod tests {
    use super::{
        CookPlatform, HostTarget, PatchBundle, apply_patch, build_patch, bundle_manifest_hash,
        cdc_chunk_end_indices, cert_check_signed, cook_dispatch, dlc_allowed,
        host_target_requires_farm, incremental_needed_hashes, installer_format, signing_dispatch,
        validate_store_config, verify_patched, CertVerdict, CookDispatch, InstallerFormat,
        SigningTool, StoreKind,
    };
    use blake3::Hasher;

    /// TC-15.14.1.1 — Asset cooker platform dispatch (R-15.14.1).
    #[test]
    fn tc_15_14_1_1_cook_dispatch() {
        assert_eq!(cook_dispatch(CookPlatform::Windows), CookDispatch::D3d);
    }

    /// TC-15.14.1.2 — Bundle builder BLAKE3 hash (R-15.14.1).
    #[test]
    fn tc_15_14_1_2_bundle_hash() {
        let m = b"bundle-manifest-v0";
        let h = bundle_manifest_hash(m);
        let mut hasher = Hasher::new();
        hasher.update(m);
        assert_eq!(h, *hasher.finalize().as_bytes());
    }

    /// TC-15.14.3.1 — Cert rule pass/fail (R-15.14.3).
    #[test]
    fn tc_15_14_3_1_cert_rule() {
        assert_eq!(cert_check_signed(true), CertVerdict::Pass);
        assert_eq!(cert_check_signed(false), CertVerdict::Fail);
    }

    /// TC-15.14.4.1 — Code signing dispatch (R-15.14.4).
    #[test]
    fn tc_15_14_4_1_signing_dispatch() {
        assert_eq!(signing_dispatch(CookPlatform::MacOs), SigningTool::Codesign);
    }

    /// TC-15.14.5.1 — Installer format selection (R-15.14.5).
    #[test]
    fn tc_15_14_5_1_installer_format() {
        assert_eq!(
            installer_format(CookPlatform::Linux),
            InstallerFormat::AppImage
        );
    }

    /// TC-15.14.6.1 — DLC entitlement gating (R-15.14.6).
    #[test]
    fn tc_15_14_6_1_dlc_gate() {
        assert!(!dlc_allowed(true, false));
        assert!(dlc_allowed(true, true));
    }

    /// TC-15.14.7.1 — CDC chunking determinism (R-15.14.7).
    #[test]
    fn tc_15_14_7_1_cdc_deterministic() {
        let data: Vec<u8> = (0u8..=200).collect();
        let a = cdc_chunk_end_indices(&data, 16, 128);
        let b = cdc_chunk_end_indices(&data, 16, 128);
        assert_eq!(a, b);
        assert_eq!(*a.last().unwrap(), data.len());
    }

    /// TC-15.14.7.2 — Delta patch generation (R-15.14.7).
    #[test]
    fn tc_15_14_7_2_delta_generation() {
        let old = b"hello-";
        let new = b"hello-world";
        let p = build_patch(old, new, 5);
        assert_eq!(p.old_slices, vec![(0, old.len())]);
        assert_eq!(p.tail, b"world");
    }

    /// TC-15.14.7.3 — Patch apply + verify (R-15.14.7).
    #[test]
    fn tc_15_14_7_3_patch_apply_verify() {
        let old = b"aaaa";
        let new = b"aaaabbbb";
        let patch = PatchBundle {
            old_slices: vec![(0, 4)],
            tail: b"bbbb".to_vec(),
        };
        assert!(verify_patched(old, &patch, new));
        assert_eq!(apply_patch(old, &patch).as_deref(), Some(new.as_slice()));
    }

    /// TC-15.14.8.1 — Store config validation (R-15.14.8).
    #[test]
    fn tc_15_14_8_1_store_validate() {
        assert_eq!(validate_store_config("steam"), Some(StoreKind::Steam));
        assert_eq!(validate_store_config("bad"), None);
    }

    /// TC-15.14.2.1 — Deploy-to-device incremental transfer (R-15.14.2).
    #[test]
    fn tc_15_14_2_1_incremental_hashes() {
        let local = b"abcabc";
        let remote = b"abcabd";
        let need = incremental_needed_hashes(local, remote, 3);
        assert_eq!(need.len(), 1);
    }

    /// TC-15.14.9.1 — Host/target build matrix dispatch (R-15.14.9).
    #[test]
    fn tc_15_14_9_1_host_target_matrix() {
        let ht = HostTarget {
            host: CookPlatform::MacOs,
            target: CookPlatform::Windows,
        };
        assert!(host_target_requires_farm(ht));
    }
}
