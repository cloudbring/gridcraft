#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

#[path = "../src/layers.rs"]
mod layers;

static NEXT: AtomicUsize = AtomicUsize::new(0);

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

struct Tree(PathBuf);

impl Tree {
    fn new() -> Self {
        let tree = Self(std::env::temp_dir().join(format!("gridcraft-xtask-golden-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed))));
        tree.write("Cargo.toml", include_bytes!("golden/version.input.toml"));
        tree.write("member/Cargo.toml", b"[package]\nname = \"golden\"\nversion.workspace = true\nedition.workspace = true\n");
        tree.write("member/src/lib.rs", b"#[test]\nfn sample() {}\n// #[test] ignored\n");
        tree
    }

    fn write(&self, name: &str, bytes: &[u8]) {
        let path = self.0.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, bytes).unwrap();
    }

    fn shared(&self, task: &str, args: &[&str], expected: &[u8]) {
        let output = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "fixture_driver", "--nocapture"])
            .env("GRIDCRAFT_GOLDEN_ROOT", &self.0)
            .env("GRIDCRAFT_GOLDEN_TASK", task)
            .env("GRIDCRAFT_GOLDEN_ARGS", serde_json::to_string(args).unwrap())
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
        let stdout = String::from_utf8(output.stdout).unwrap();
        let actual = stdout.split_once("<golden-output>\n").unwrap().1.split_once("</golden-output>\n").unwrap().0;
        assert_eq!(actual.as_bytes(), expected);
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn fixture_driver() {
    let Some(directory) = std::env::var_os("GRIDCRAFT_GOLDEN_ROOT") else { return };
    let task = std::env::var("GRIDCRAFT_GOLDEN_TASK").unwrap();
    let args: Vec<String> = serde_json::from_str(&std::env::var("GRIDCRAFT_GOLDEN_ARGS").unwrap()).unwrap();
    let mut configs = craft_xtask::Configs::for_root(directory);
    configs.stats.follow_symlinks = true;
    configs.stats.max_entries = usize::MAX;
    configs.stats.fail_on_budget = false;
    println!("<golden-output>");
    match task.as_str() {
        "version" => craft_xtask::version::run(&configs.version, &args).unwrap(),
        "stats" => craft_xtask::stats::run(&configs.stats, &args).unwrap(),
        _ => panic!("unexpected fixture task: {task}"),
    }
    println!("</golden-output>");
}

fn assert_output(output: Output, expected: &[u8]) {
    assert!(output.status.success(), "{output:?}");
    assert_eq!(output.stdout, expected);
}

fn binary(args: &[&str], directory: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_xtask")).args(args).current_dir(directory).output().unwrap()
}

#[test]
fn version_stdout_and_edits_match_legacy() {
    let tree = Tree::new();
    tree.shared("version", &[], include_bytes!("golden/version.stdout"));
    tree.shared("version", &["set", "1.2.3-rc.1"], include_bytes!("golden/version-set.stdout"));
    assert_eq!(std::fs::read(tree.0.join("Cargo.toml")).unwrap(), include_bytes!("golden/version.updated.toml"));
    assert_eq!(std::fs::read(tree.0.join("Cargo.lock")).unwrap(), include_bytes!("golden/version.updated.lock"));
    let crlf = include_str!("golden/version.input.toml").replace('\n', "\r\n");
    assert_eq!(craft_xtask::version::replace(&crlf, "1.2.3-rc.1").unwrap(), include_str!("golden/version.updated.toml").replace('\n', "\r\n"));
    assert_output(binary(&["version"], &root()), include_bytes!("golden/version.stdout"));
}

#[test]
fn stats_stdout_and_exclusions_match_legacy() {
    let tree = Tree::new();
    for directory in ["target", "fuzz", ".hidden"] {
        tree.write(&format!("member/{directory}/ignored.rs"), b"#[test]\nfn ignored() {}\n");
    }
    tree.shared("stats", &[], include_bytes!("golden/stats.stdout"));
}

#[cfg(unix)]
#[test]
fn stats_follows_symlinks_without_recounting_directories() {
    let tree = Tree::new();
    std::os::unix::fs::symlink(tree.0.join("member/src"), tree.0.join("member/alias")).unwrap();
    std::os::unix::fs::symlink(tree.0.join("member"), tree.0.join("member/src/cycle")).unwrap();
    tree.shared("stats", &[], include_bytes!("golden/stats.stdout"));
}

#[test]
fn current_tree_stats_match_except_changed_xtask_and_total() {
    let output = binary(&["stats"], &root());
    assert!(output.status.success(), "{output:?}");
    let unchanged =
        |text: &str| text.lines().filter(|line| !line.starts_with("xtask ") && !line.starts_with("TOTAL ")).collect::<Vec<_>>().join("\n");
    assert_eq!(unchanged(&String::from_utf8(output.stdout).unwrap()), unchanged(include_str!("golden/stats-repo.stdout")));
}

#[test]
fn ico_stdout_and_bytes_match_legacy() {
    let tree = Tree::new();
    for size in [16_u32, 32, 48, 256] {
        let mut png = b"\x89PNG\r\n\x1a\n\0\0\0\x0dIHDR".to_vec();
        png.extend_from_slice(&size.to_be_bytes());
        png.extend_from_slice(&size.to_be_bytes());
        png.extend_from_slice(&[8, 6, 0, 0, 0, 1, 2, 3, 4]);
        tree.write(&format!("{size}.png"), &png);
    }
    assert_output(binary(&["ico", "out.ico", "16.png", "32.png", "48.png", "256.png"], &tree.0), include_bytes!("golden/ico.stdout"));
    let actual: String = std::fs::read(tree.0.join("out.ico")).unwrap().iter().map(|byte| format!("{byte:02x}")).collect();
    assert_eq!(actual, include_str!("golden/ico.hex").trim_end());
}

#[test]
fn current_tree_layers_match_legacy() {
    assert_output(binary(&["layers"], &root()), include_bytes!("golden/layers.stdout"));
}

#[test]
fn help_keeps_the_local_descriptions_and_shared_syntax() {
    let output = binary(&["--help"], &root());
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("parity [--check] [--out PATH]"));
    assert!(text.contains("keytips [--check]"));
    assert!(text.contains(craft_xtask::version::cli_help()));
    assert!(text.contains(craft_xtask::stats::cli_help()));
    assert!(!text.contains("ico/icns"));
}

#[test]
fn frozen_graph_matches_legacy() {
    let metadata = serde_json::from_str(include_str!("golden/metadata.json")).unwrap();
    let graph = layers::from_metadata(&metadata).unwrap();
    let (text, violations) = layers::report(&graph).unwrap();
    assert_eq!(violations, 0);
    assert_eq!(text, include_str!("golden/layers.stdout"));
}

#[test]
fn layer_policy_rejects_forbidden_edges_with_legacy_diagnostics() {
    for (source, target, workspace, kind, expected) in [
        (
            "gridcraft-formula",
            "gridcraft-engine",
            true,
            layers::DepKind::Normal,
            "gridcraft-formula (L1) -> gridcraft-engine (L5) [Normal]: may only depend on strictly lower layers",
        ),
        (
            "gridcraft-xlsx",
            "gridcraft-calc",
            true,
            layers::DepKind::Dev,
            "gridcraft-xlsx (L2) -> gridcraft-calc (L2) [Dev]: may only depend on strictly lower layers",
        ),
        (
            "gridcraft-core",
            "wgpu-types",
            false,
            layers::DepKind::Build,
            "gridcraft-core (L0) depends on UI crate `wgpu-types`; UI toolkits are only allowed in L6+",
        ),
        (
            "gridcraft-xlsx",
            "gridcraft-numfmt",
            true,
            layers::DepKind::Normal,
            "gridcraft-xlsx: format crates may only depend on core/formula/model, not gridcraft-numfmt",
        ),
        (
            "gridcraft-core",
            "xtask",
            true,
            layers::DepKind::Normal,
            "gridcraft-core (L0) -> xtask (L7) [Normal]: may only depend on strictly lower layers",
        ),
    ] {
        let graph = [layers::Crate { name: source.into(), deps: vec![layers::Dep { name: target.into(), kind, workspace }] }];
        let (text, violations) = layers::report(&graph).unwrap();
        assert!(violations > 0 && text.contains(expected), "{text}");
    }
}

#[test]
fn layer_policy_preserves_exemptions_and_allowed_edges() {
    for (source, target, kind) in [
        ("gridcraft-functions", "gridcraft-formula", layers::DepKind::Normal),
        ("gridcraft-xlsx", "gridcraft-math", layers::DepKind::Normal),
        ("gridcraft-xlsx", "gridcraft-numfmt", layers::DepKind::Dev),
        ("gridcraft", "xtask", layers::DepKind::Normal),
        ("gridcraft-core", "unregistered", layers::DepKind::Normal),
        ("gridcraft-core", "gridcraft-core", layers::DepKind::Dev),
    ] {
        let graph = [layers::Crate { name: source.into(), deps: vec![layers::Dep { name: target.into(), kind, workspace: true }] }];
        assert_eq!(layers::report(&graph).unwrap().1, 0, "{source} -> {target}");
    }
    let graph = [layers::Crate { name: "gridcraft-unknown".into(), deps: vec![] }];
    let (text, violations) = layers::report(&graph).unwrap();
    assert_eq!(violations, 1);
    assert!(text.contains("register it in xtask/src/layers.rs TABLE"));
}

#[cfg(feature = "engine")]
#[test]
fn parity_write_check_and_document_match_legacy() {
    let tree = Tree::new();
    assert_output(binary(&["parity", "--out", "parity.md"], &tree.0), include_bytes!("golden/parity-write.stdout"));
    assert_eq!(std::fs::read(tree.0.join("parity.md")).unwrap(), include_bytes!("golden/parity.md"));
    assert_output(binary(&["parity", "--check", "--out", "parity.md"], &tree.0), include_bytes!("golden/parity-check.stdout"));
    tree.write("parity.md", b"stale\n");
    let output = binary(&["parity", "--check", "--out", "parity.md"], &tree.0);
    assert!(!output.status.success());
    assert_eq!(std::fs::read(tree.0.join("parity.md")).unwrap(), b"stale\n");
}

#[cfg(not(feature = "engine"))]
#[test]
fn parity_without_engine_keeps_the_local_diagnostic() {
    let output = binary(&["parity"], &root());
    assert!(!output.status.success());
    assert_eq!(output.stderr, b"error: xtask was built without the `engine` feature; run `cargo xtask parity` with default features\n");
}
