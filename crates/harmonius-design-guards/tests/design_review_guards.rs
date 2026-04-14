//! Machine checks for cross-cutting design-review resolutions (see
//! `docs/design/design-review.md`).

use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn read_repo_file(rel: &str) -> String {
    let path = repo_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!("failed to read {}: {err}", path.display());
    })
}

#[test]
fn network_transport_bans_tokio_in_game_runtime_doc() {
    let body = read_repo_file("docs/design/networking/network-transport.md");
    assert!(
        !body.contains("tokio::"),
        "P0-7: game transport doc must not reference tokio:: (sync request/handle pattern)"
    );
}

#[test]
fn p0_design_review_artifacts_exist_on_disk() {
    let paths = [
        "docs/design/core-runtime/graph-runtime.md",
        "docs/design/core-runtime/primitives.md",
        "docs/design/core-runtime/ids.md",
        "docs/design/core-runtime/hot-reload-protocol.md",
        "docs/design/data-systems/composition.md",
    ];
    for rel in paths {
        let path = repo_root().join(rel);
        assert!(
            path.is_file(),
            "expected design artifact {}",
            path.display()
        );
    }
}

#[test]
fn hot_reload_protocol_traces_logic_graph_feature() {
    let body = read_repo_file("docs/design/core-runtime/hot-reload-protocol.md");
    assert!(
        body.contains("F-13.4.3"),
        "hot-reload-protocol must reference F-13.4.3 (logic graph hot reload)"
    );
}

#[test]
fn gameplay_scripting_features_avoid_coroutine_engine_terminology() {
    let body = read_repo_file("docs/features/game-framework/scripting.md");
    assert!(
        !body.to_lowercase().contains("coroutine"),
        "F-13.4.3 + design-review P1-20: avoid coroutine wording; use suspend-state / graph execution"
    );
}

#[test]
fn logic_graph_hot_reload_feature_avoids_coroutine_engine_terminology() {
    let body = read_repo_file("docs/features/content-pipeline/hot-reload.md");
    assert!(
        !body.to_lowercase().contains("coroutine"),
        "F-12.4.4 + design-review: logic graph hot reload copy must not say coroutine"
    );
}
