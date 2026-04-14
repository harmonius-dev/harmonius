//! Harmonius plugin marketplace protocol core: catalog queries, CAS-backed installs,
//! semver resolution, trust classification, and update discovery.
//!
//! Transport (HTTP/3) and editor UI are out of crate scope; this library holds deterministic
//! data-plane logic and filesystem layout helpers.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod catalog;
mod install;
mod manifest;
mod resolve;
mod trust;
mod types;

pub use catalog::{CatalogQuery, Listing, Page, paginate_catalog};
pub use install::{
    BlobSource, CodegenPort, InstallError, InstallPipeline, InstallReport, UpdateCandidate,
    blake3_hex, blob_path, bulk_update_targets, check_updates, installed_current_path,
    manifest_path,
};
pub use manifest::{
    BlobRef, CodegenEntry, FileEntry, FileRole, ManifestDependency, PlatformSet, PluginManifest,
    file_role_counts, manifest_from_bytes, manifest_rkyv_roundtrip_bitidentical,
    manifest_signing_digest, manifest_to_bytes,
};
pub use resolve::{ManifestSource, ResolveError, ResolvedSet, Resolver};
pub use trust::{Signature, TrustError, TrustLevel, TrustStore, sign_manifest};
pub use types::{Blake3Hash, DependencyReq, InstalledPlugin, PluginId, PublisherId};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::install::{BlobSource, CodegenPort, InstallPipeline};
    use crate::resolve::{ManifestSource, ResolvedSet, Resolver};
    use crate::trust::{TrustLevel, TrustStore, sign_manifest};
    use ed25519_dalek::SigningKey;
    use rand::RngCore;
    use semver::Version;
    use std::collections::HashMap;
    use std::fs;
    use std::sync::atomic::{AtomicU32, Ordering};
    use tempfile::tempdir;

    #[test]
    fn test_catalog_query_roundtrip() {
        let q = CatalogQuery {
            query: "foo".into(),
            limit: 10,
            cursor: Some("o:20".into()),
        };
        let json = serde_json::to_string(&q).expect("serialize");
        let back: CatalogQuery = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(q, back);
    }

    #[test]
    fn test_catalog_pagination_cursor() {
        let listings: Vec<Listing> = (0..25)
            .map(|i| Listing {
                id: format!("p-{i:02}"),
                version: "1.0.0".into(),
                summary: format!("s{i}"),
            })
            .collect();
        let p1 = paginate_catalog(
            &listings,
            &CatalogQuery {
                query: String::new(),
                limit: 10,
                cursor: None,
            },
        );
        assert_eq!(p1.items.len(), 10);
        let c1 = p1.next_cursor.expect("next");
        let p2 = paginate_catalog(
            &listings,
            &CatalogQuery {
                query: String::new(),
                limit: 10,
                cursor: Some(c1.clone()),
            },
        );
        assert_eq!(p2.items.len(), 10);
        let ids1: Vec<_> = p1.items.iter().map(|l| l.id.as_str()).collect();
        let ids2: Vec<_> = p2.items.iter().map(|l| l.id.as_str()).collect();
        assert!(ids1.iter().all(|i| !ids2.contains(i)));
    }

    struct CountingFetcher<'a> {
        inner: HashMap<[u8; 32], Vec<u8>>,
        fetches: &'a AtomicU32,
    }

    impl BlobSource for CountingFetcher<'_> {
        fn fetch_blob(&mut self, hash: Blake3Hash) -> Result<Vec<u8>, InstallError> {
            self.fetches.fetch_add(1, Ordering::SeqCst);
            self.inner
                .get(&hash.0)
                .cloned()
                .ok_or_else(|| InstallError::CodegenFailed("missing fixture blob".into()))
        }
    }

    struct OkCodegen;

    impl CodegenPort for OkCodegen {
        fn rebuild(&mut self) -> Result<(), InstallError> {
            Ok(())
        }
    }

    struct FailCodegen;

    impl CodegenPort for FailCodegen {
        fn rebuild(&mut self) -> Result<(), InstallError> {
            Err(InstallError::CodegenFailed("injected".into()))
        }
    }

    fn sample_manifest_with_blob(blob: &[u8]) -> PluginManifest {
        let h = *blake3::hash(blob).as_bytes();
        PluginManifest {
            id: "demo".into(),
            version: "1.0.0".into(),
            author: "a".into(),
            display_name: "Demo".into(),
            description: "d".into(),
            entry_point: CodegenEntry::MiddlemanModule { module: "m".into() },
            dependencies: vec![],
            files: vec![FileEntry {
                path: "a.txt".into(),
                blob: BlobRef {
                    hash: h,
                    size: blob.len() as u64,
                },
                role: FileRole::Asset,
            }],
            engine_range: ">=1.0.0".into(),
            platforms: PlatformSet(0),
        }
    }

    #[test]
    fn test_install_fetches_missing_blobs() {
        let dir = tempdir().expect("tempdir");
        let data = b"hello-cas";
        let hash = Blake3Hash(*blake3::hash(data).as_bytes());
        let mut map = HashMap::new();
        map.insert(hash.0, data.to_vec());
        let fetches = AtomicU32::new(0);
        let fetcher = CountingFetcher {
            inner: map,
            fetches: &fetches,
        };
        let m = sample_manifest_with_blob(data);
        let set = ResolvedSet {
            plugins: vec![("demo".into(), Version::new(1, 0, 0))],
            order: vec!["demo".into()],
        };
        let mut pipe = InstallPipeline::new(dir.path().to_path_buf(), fetcher, OkCodegen);
        pipe.install(&set, &[("demo".into(), Version::new(1, 0, 0), m.clone())])
            .expect("install");
        assert!(fetches.load(Ordering::SeqCst) >= 1);
        let bp = blob_path(dir.path(), &hash);
        assert!(bp.exists());
    }

    #[test]
    fn test_install_skips_cached_blobs() {
        let dir = tempdir().expect("tempdir");
        let data = b"cached";
        let hash = Blake3Hash(*blake3::hash(data).as_bytes());
        let blob_dest = blob_path(dir.path(), &hash);
        fs::create_dir_all(blob_dest.parent().expect("parent")).expect("mkdir");
        fs::write(&blob_dest, data).expect("write blob");
        let mut map = HashMap::new();
        map.insert(hash.0, data.to_vec());
        let fetches = AtomicU32::new(0);
        let fetcher = CountingFetcher {
            inner: map,
            fetches: &fetches,
        };
        let m = sample_manifest_with_blob(data);
        let set = ResolvedSet {
            plugins: vec![("demo".into(), Version::new(1, 0, 0))],
            order: vec!["demo".into()],
        };
        let mut pipe = InstallPipeline::new(dir.path().to_path_buf(), fetcher, OkCodegen);
        pipe.install(&set, &[("demo".into(), Version::new(1, 0, 0), m)])
            .expect("install");
        assert_eq!(fetches.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_install_atomic_on_failure() {
        let dir = tempdir().expect("tempdir");
        let data = b"rollback";
        let hash = Blake3Hash(*blake3::hash(data).as_bytes());
        let mut map = HashMap::new();
        map.insert(hash.0, data.to_vec());
        let fetches = AtomicU32::new(0);
        let fetcher = CountingFetcher {
            inner: map,
            fetches: &fetches,
        };
        let m = sample_manifest_with_blob(data);
        let set = ResolvedSet {
            plugins: vec![("demo".into(), Version::new(1, 0, 0))],
            order: vec!["demo".into()],
        };
        let mut pipe = InstallPipeline::new(dir.path().to_path_buf(), fetcher, FailCodegen);
        assert!(
            pipe.install(&set, &[("demo".into(), Version::new(1, 0, 0), m)],)
                .is_err()
        );
        let cur = installed_current_path(dir.path(), "demo");
        assert!(!cur.exists());
    }

    #[test]
    fn test_uninstall_removes_symlink() {
        let dir = tempdir().expect("tempdir");
        let data = b"x";
        let mut map = HashMap::new();
        map.insert(*blake3::hash(data).as_bytes(), data.to_vec());
        let fetches = AtomicU32::new(0);
        let fetcher = CountingFetcher {
            inner: map,
            fetches: &fetches,
        };
        let m = sample_manifest_with_blob(data);
        let set = ResolvedSet {
            plugins: vec![("demo".into(), Version::new(1, 0, 0))],
            order: vec!["demo".into()],
        };
        let mut pipe = InstallPipeline::new(dir.path().to_path_buf(), fetcher, OkCodegen);
        pipe.install(&set, &[("demo".into(), Version::new(1, 0, 0), m)])
            .expect("install");
        let cur = installed_current_path(dir.path(), "demo");
        assert!(cur.exists());
        pipe.uninstall("demo").expect("uninstall");
        assert!(!cur.exists());
    }

    #[test]
    fn test_manifest_rkyv_roundtrip() {
        let m = sample_manifest_with_blob(b"payload");
        assert!(manifest_rkyv_roundtrip_bitidentical(&m));
    }

    #[test]
    fn test_blob_integrity_mismatch_rejected() {
        let dir = tempdir().expect("tempdir");
        let data = b"good";
        let wrong = b"bad";
        let hash = Blake3Hash(*blake3::hash(data).as_bytes());
        let mut map = HashMap::new();
        map.insert(hash.0, wrong.to_vec());
        let fetches = AtomicU32::new(0);
        let fetcher = CountingFetcher {
            inner: map,
            fetches: &fetches,
        };
        let m = sample_manifest_with_blob(data);
        let set = ResolvedSet {
            plugins: vec![("demo".into(), Version::new(1, 0, 0))],
            order: vec!["demo".into()],
        };
        let mut pipe = InstallPipeline::new(dir.path().to_path_buf(), fetcher, OkCodegen);
        let err = pipe
            .install(&set, &[("demo".into(), Version::new(1, 0, 0), m)])
            .expect_err("integrity");
        assert!(matches!(err, InstallError::IntegrityFailure));
    }

    #[test]
    fn test_file_role_counts_by_kind() {
        let m = PluginManifest {
            id: "x".into(),
            version: "1.0.0".into(),
            author: "a".into(),
            display_name: "x".into(),
            description: "d".into(),
            entry_point: CodegenEntry::MiddlemanModule { module: "m".into() },
            dependencies: vec![],
            files: vec![
                FileEntry {
                    path: "c1.rs".into(),
                    blob: BlobRef {
                        hash: [1u8; 32],
                        size: 1,
                    },
                    role: FileRole::CodegenSource,
                },
                FileEntry {
                    path: "c2.rs".into(),
                    blob: BlobRef {
                        hash: [2u8; 32],
                        size: 1,
                    },
                    role: FileRole::CodegenSource,
                },
                FileEntry {
                    path: "a1".into(),
                    blob: BlobRef {
                        hash: [3u8; 32],
                        size: 1,
                    },
                    role: FileRole::Asset,
                },
                FileEntry {
                    path: "a2".into(),
                    blob: BlobRef {
                        hash: [4u8; 32],
                        size: 1,
                    },
                    role: FileRole::Asset,
                },
                FileEntry {
                    path: "a3".into(),
                    blob: BlobRef {
                        hash: [5u8; 32],
                        size: 1,
                    },
                    role: FileRole::Asset,
                },
                FileEntry {
                    path: "L".into(),
                    blob: BlobRef {
                        hash: [6u8; 32],
                        size: 1,
                    },
                    role: FileRole::License,
                },
            ],
            engine_range: ">=1".into(),
            platforms: PlatformSet(0),
        };
        let c = file_role_counts(&m);
        assert_eq!(c[0], 2);
        assert_eq!(c[1], 3);
        assert_eq!(c[2], 1);
        assert_eq!(c[3], 0);
        assert_eq!(c[4], 0);
    }

    #[test]
    fn test_signature_valid_accepted() {
        let bytes = b"manifest-bytes";
        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);
        let key = SigningKey::from_bytes(&seed);
        let sig = sign_manifest(bytes, &key);
        let mut store = TrustStore::new();
        store.register_publisher("pub".into(), TrustLevel::Verified);
        let level = store
            .verify_with_author(bytes, &"pub".into(), &sig)
            .expect("verify");
        assert_eq!(level, TrustLevel::Verified);
    }

    #[test]
    fn test_signature_invalid_rejected() {
        let bytes = b"manifest-bytes";
        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);
        let key = SigningKey::from_bytes(&seed);
        let mut sig = sign_manifest(bytes, &key);
        sig.sig[0] ^= 0xff;
        let store = TrustStore::new();
        assert!(store.verify(bytes, &sig).is_err());
    }

    #[test]
    fn test_unknown_publisher_unsigned_level() {
        let bytes = b"m";
        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);
        let key = SigningKey::from_bytes(&seed);
        let sig = sign_manifest(bytes, &key);
        let store = TrustStore::new();
        let level = store.verify(bytes, &sig).expect("ok");
        assert_eq!(level, TrustLevel::Unsigned);
    }

    #[test]
    fn test_official_key_trust_level() {
        let bytes = b"m";
        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);
        let key = SigningKey::from_bytes(&seed);
        let sig = sign_manifest(bytes, &key);
        let mut store = TrustStore::new();
        store.add_official_key(sig.publisher_key);
        let level = store.verify(bytes, &sig).expect("ok");
        assert_eq!(level, TrustLevel::Official);
    }

    struct FakeCatalog {
        engine: Version,
        versions: HashMap<String, Vec<Version>>,
        manifests: HashMap<(String, Version), (semver::VersionReq, Vec<DependencyReq>)>,
    }

    impl ManifestSource for FakeCatalog {
        fn current_engine(&self) -> Version {
            self.engine.clone()
        }

        fn list_versions(&self, id: &PluginId) -> Vec<Version> {
            self.versions.get(id).cloned().unwrap_or_default()
        }

        fn manifest_constraints(
            &self,
            id: &PluginId,
            version: &Version,
        ) -> Option<(semver::VersionReq, Vec<DependencyReq>)> {
            self.manifests.get(&(id.clone(), version.clone())).cloned()
        }
    }

    fn vr(s: &str) -> semver::VersionReq {
        semver::VersionReq::parse(s).expect("parse range")
    }

    #[test]
    fn test_resolver_picks_highest_compatible() {
        let mut c = FakeCatalog {
            engine: Version::new(1, 0, 0),
            versions: HashMap::new(),
            manifests: HashMap::new(),
        };
        c.versions.insert(
            "foo".into(),
            vec![
                Version::new(2, 0, 0),
                Version::new(1, 1, 0),
                Version::new(1, 0, 0),
            ],
        );
        c.manifests.insert(
            ("foo".into(), Version::new(2, 0, 0)),
            (vr(">=1.0.0"), vec![]),
        );
        c.manifests.insert(
            ("foo".into(), Version::new(1, 1, 0)),
            (vr(">=1.0.0"), vec![]),
        );
        c.manifests.insert(
            ("foo".into(), Version::new(1, 0, 0)),
            (vr(">=1.0.0"), vec![]),
        );
        let got = Resolver::resolve(
            &[DependencyReq {
                id: "foo".into(),
                range: vr("^1.0"),
            }],
            &[],
            &c,
        )
        .expect("resolve");
        assert_eq!(
            got.plugins.iter().find(|(i, _)| i == "foo").map(|(_, v)| v),
            Some(&Version::new(1, 1, 0))
        );
    }

    #[test]
    fn test_resolver_backtracks_on_conflict() {
        let mut c = FakeCatalog {
            engine: Version::new(1, 0, 0),
            versions: HashMap::new(),
            manifests: HashMap::new(),
        };
        c.versions.insert(
            "root".into(),
            vec![Version::new(2, 0, 0), Version::new(1, 0, 0)],
        );
        c.versions.insert("x".into(), vec![Version::new(1, 5, 0)]);
        c.manifests.insert(
            ("root".into(), Version::new(2, 0, 0)),
            (
                vr(">=1.0.0"),
                vec![DependencyReq {
                    id: "x".into(),
                    range: vr("^2.0"),
                }],
            ),
        );
        c.manifests.insert(
            ("root".into(), Version::new(1, 0, 0)),
            (
                vr(">=1.0.0"),
                vec![DependencyReq {
                    id: "x".into(),
                    range: vr("^1.0"),
                }],
            ),
        );
        c.manifests
            .insert(("x".into(), Version::new(1, 5, 0)), (vr(">=1.0.0"), vec![]));
        let got = Resolver::resolve(
            &[DependencyReq {
                id: "root".into(),
                range: vr("^1.0"),
            }],
            &[],
            &c,
        )
        .expect("resolve");
        assert_eq!(
            got.plugins
                .iter()
                .find(|(i, _)| i == "root")
                .map(|(_, v)| v),
            Some(&Version::new(1, 0, 0))
        );
    }

    #[test]
    fn test_resolver_detects_cycle() {
        let mut c = FakeCatalog {
            engine: Version::new(1, 0, 0),
            versions: HashMap::new(),
            manifests: HashMap::new(),
        };
        c.versions.insert("a".into(), vec![Version::new(1, 0, 0)]);
        c.versions.insert("b".into(), vec![Version::new(1, 0, 0)]);
        c.manifests.insert(
            ("a".into(), Version::new(1, 0, 0)),
            (
                vr(">=1.0.0"),
                vec![DependencyReq {
                    id: "b".into(),
                    range: vr("^1.0"),
                }],
            ),
        );
        c.manifests.insert(
            ("b".into(), Version::new(1, 0, 0)),
            (
                vr(">=1.0.0"),
                vec![DependencyReq {
                    id: "a".into(),
                    range: vr("^1.0"),
                }],
            ),
        );
        let err = Resolver::resolve(
            &[DependencyReq {
                id: "a".into(),
                range: vr("^1.0"),
            }],
            &[],
            &c,
        )
        .expect_err("cycle");
        assert!(matches!(err, ResolveError::Cycle));
    }

    #[test]
    fn test_resolver_topological_order() {
        let mut c = FakeCatalog {
            engine: Version::new(1, 0, 0),
            versions: HashMap::new(),
            manifests: HashMap::new(),
        };
        for id in ["a", "b"] {
            c.versions.insert(id.into(), vec![Version::new(1, 0, 0)]);
            c.manifests
                .insert((id.into(), Version::new(1, 0, 0)), (vr(">=1.0.0"), vec![]));
        }
        c.manifests.insert(
            ("a".into(), Version::new(1, 0, 0)),
            (
                vr(">=1.0.0"),
                vec![DependencyReq {
                    id: "b".into(),
                    range: vr("^1.0"),
                }],
            ),
        );
        let got = Resolver::resolve(
            &[DependencyReq {
                id: "a".into(),
                range: vr("^1.0"),
            }],
            &[],
            &c,
        )
        .expect("resolve");
        let pos_b = got.order.iter().position(|x| x == "b").expect("b");
        let pos_a = got.order.iter().position(|x| x == "a").expect("a");
        assert!(pos_b < pos_a);
    }

    #[test]
    fn test_resolver_engine_range_filter() {
        let mut c = FakeCatalog {
            engine: Version::new(1, 1, 0),
            versions: HashMap::new(),
            manifests: HashMap::new(),
        };
        c.versions.insert("p".into(), vec![Version::new(1, 0, 0)]);
        c.manifests
            .insert(("p".into(), Version::new(1, 0, 0)), (vr("^1.2.0"), vec![]));
        let err = Resolver::resolve(
            &[DependencyReq {
                id: "p".into(),
                range: vr("^1.0"),
            }],
            &[],
            &c,
        )
        .expect_err("engine");
        assert!(matches!(err, ResolveError::EngineRangeUnmet { .. }));
    }

    #[test]
    fn test_resolver_deterministic_with_equal_set() {
        let mut c = FakeCatalog {
            engine: Version::new(1, 0, 0),
            versions: HashMap::new(),
            manifests: HashMap::new(),
        };
        c.versions.insert("z".into(), vec![Version::new(1, 0, 0)]);
        c.manifests
            .insert(("z".into(), Version::new(1, 0, 0)), (vr(">=1.0.0"), vec![]));
        let wanted = [DependencyReq {
            id: "z".into(),
            range: vr("^1.0"),
        }];
        let a = Resolver::resolve(&wanted, &[], &c).expect("a");
        let b = Resolver::resolve(&wanted, &[], &c).expect("b");
        assert_eq!(a.plugins, b.plugins);
        assert_eq!(a.order, b.order);
    }

    #[test]
    fn test_update_check_reports_higher_version() {
        let installed = vec![("a".into(), Version::new(1, 0, 0), false)];
        let latest = |id: &str| {
            if id == "a" {
                Some(Version::new(1, 1, 0))
            } else {
                None
            }
        };
        let u = check_updates(&installed, &latest);
        assert_eq!(u.len(), 1);
        assert_eq!(u[0].available, Version::new(1, 1, 0));
    }

    #[test]
    fn test_update_check_honors_pinning() {
        let installed = vec![("a".into(), Version::new(1, 0, 0), true)];
        let latest = |_id: &str| Some(Version::new(2, 0, 0));
        let u = check_updates(&installed, &latest);
        assert_eq!(u.len(), 1);
        assert!(u[0].pinned);
        let targets = bulk_update_targets(&u);
        assert!(targets.is_empty());
    }

    #[test]
    fn test_bulk_update_resolves_all_non_pinned() {
        let cands = vec![
            UpdateCandidate {
                plugin: "b".into(),
                installed: Version::new(1, 0, 0),
                available: Version::new(1, 1, 0),
                changelog: None,
                pinned: false,
            },
            UpdateCandidate {
                plugin: "a".into(),
                installed: Version::new(1, 0, 0),
                available: Version::new(1, 2, 0),
                changelog: None,
                pinned: false,
            },
        ];
        let t = bulk_update_targets(&cands);
        assert_eq!(t, vec!["a".to_string(), "b".to_string()]);
    }
}
