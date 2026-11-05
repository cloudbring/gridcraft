//! `cargo xtask check-corpus [--update-baseline]`: run `gridcraft-cli check` (recalculation against
//! the cached values, package round trip) over every `.xlsx`/`.xlsm` of the verified Office corpora
//! and compare with `baselines/xlsx-corpus.json`, the way pdfcraft's `xtask check` compares with
//! `baselines/pdfjs.json`.
//!
//! The corpora are read-only inputs from craft-corpus: `office-poi`, `office-libreoffice` and
//! `office-openxml-sdk` under `$GRIDCRAFT_OFFICE_CORPUS` (default `~/ArtCraft/corpus`). Each has a
//! `.craft-corpus.json` manifest whose `path` and `sha256` key the baseline. When the corpus is
//! absent the command skips (exit 0) unless `GRIDCRAFT_REQUIRE_OFFICE_CORPUS` is set, which turns that
//! into a failure.
//!
//! A baseline record per file: `ok`, or the number of mismatches by function, the package/round-trip
//! categories that differ, or an open error / crash. A run fails on any crash, on a file or function or
//! category that is worse than its baseline record, and on a non-ok file the baseline doesn't know
//! (new path, or the same path with another sha256). Cached values from producers other than Excel are
//! a weaker oracle, so every record carries the producer class (`excel` or `other`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

const CORPORA: [&str; 3] = ["office-poi", "office-libreoffice", "office-openxml-sdk"];
const BASELINE: &str = "baselines/xlsx-corpus.json";
const DEFAULT_ROOT: &str = "/Users/emm/ArtCraft/corpus";
const SCHEMA: &str = "gridcraft-xlsx-corpus/1";

/// Excel-produced: `Application` is "Microsoft Excel", "Microsoft Macintosh Excel", "Microsoft Excel Online".
pub fn is_excel(producer: Option<&str>) -> bool {
    producer.is_some_and(|p| p.starts_with("Microsoft") && p.contains("Excel"))
}

/// The baseline record of one `check --json` file entry.
pub fn record(file: &Value, sha256: &str) -> Value {
    let producer = file["producer"].as_str();
    let class = if is_excel(producer) { "excel" } else { "other" };
    let base = |status: &str| json!({"sha256": sha256, "producer": producer, "class": class, "status": status});
    if file["opened"] != json!(true) {
        let error = file["error"].as_str().unwrap_or("");
        let mut r = base(if error.contains("internal error while checking") { "crash" } else { "open-error" });
        r["error"] = json!(error.chars().take(400).collect::<String>());
        return r;
    }
    let calc = &file["calc"];
    let mut mismatches = BTreeMap::new();
    for f in calc["functions"].as_array().into_iter().flatten() {
        let n = f["mismatched"].as_u64().unwrap_or(0);
        if n > 0 {
            mismatches.insert(f["name"].as_str().unwrap_or("?").to_string(), n);
        }
    }
    let mut package = BTreeMap::new();
    if let Some(e) = file["roundTrip"]["error"].as_str() {
        package.insert(format!("round-trip failed: {}", e.chars().take(80).collect::<String>()), 1);
    }
    for (name, c) in file["roundTrip"]["categories"].as_object().into_iter().flatten() {
        let n = c["diffs"].as_u64().unwrap_or(0);
        if n > 0 {
            package.insert(name.clone(), n);
        }
    }
    let ok = calc["mismatched"].as_u64() == Some(0) && package.is_empty();
    let mut r = base(if ok { "ok" } else { "differs" });
    r["formulaCells"] = calc["formulaCells"].clone();
    r["checked"] = calc["checked"].clone();
    r["matched"] = calc["matched"].clone();
    if !mismatches.is_empty() {
        r["mismatches"] = json!(mismatches);
    }
    if !package.is_empty() {
        r["package"] = json!(package);
    }
    r
}

fn counts(v: &Value, key: &str) -> BTreeMap<String, u64> {
    v[key].as_object().into_iter().flatten().map(|(k, n)| (k.clone(), n.as_u64().unwrap_or(0))).collect()
}

/// Why `now` is worse than the baseline record `then` (empty: not worse). `then` of `None` is a file the
/// baseline doesn't know, or knows under another sha256.
pub fn regressions(now: &Value, then: Option<&Value>) -> Vec<String> {
    let status = now["status"].as_str().unwrap_or("?");
    if status == "crash" {
        return vec!["crash".into()];
    }
    let Some(then) = then.filter(|t| t["sha256"] == now["sha256"]) else {
        return if status == "ok" { vec![] } else { vec![format!("{status} and not in the baseline")] };
    };
    let mut out = Vec::new();
    match (then["status"].as_str().unwrap_or("?"), status) {
        ("ok", "ok") => {}
        (_, "open-error") if then["status"] != json!("open-error") => out.push("no longer opens".into()),
        (_, "open-error") | (_, "ok") => {}
        _ => {
            for key in ["mismatches", "package"] {
                let before = counts(then, key);
                for (name, n) in counts(now, key) {
                    let was = before.get(&name).copied().unwrap_or(0);
                    if n > was {
                        out.push(format!("{key} {name}: {was} -> {n}"));
                    }
                }
            }
        }
    }
    out
}

fn manifest(dir: &Path) -> Result<BTreeMap<String, String>, String> {
    let text = std::fs::read_to_string(dir.join(".craft-corpus.json")).map_err(|e| format!("{}: no manifest: {e}", dir.display()))?;
    let m: Value = serde_json::from_str(&text).map_err(|e| format!("{}: bad manifest: {e}", dir.display()))?;
    Ok(m["files"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|f| Some((f["path"].as_str()?.to_string(), f["sha256"].as_str()?.to_string())))
        .collect())
}

fn gridcraft_cli() -> PathBuf {
    crate::root().join("target/release/gridcraft-cli")
}

pub fn run(root: &Path, args: &[&str]) -> Result<(), String> {
    let update = args.contains(&"--update-baseline");
    let corpus_root = PathBuf::from(std::env::var_os("GRIDCRAFT_OFFICE_CORPUS").unwrap_or_else(|| DEFAULT_ROOT.into()));
    let required = std::env::var_os("GRIDCRAFT_REQUIRE_OFFICE_CORPUS").is_some_and(|v| !v.is_empty() && v != "0");
    let missing: Vec<&str> = CORPORA.iter().copied().filter(|c| !corpus_root.join(c).join(".craft-corpus.json").is_file()).collect();
    if !missing.is_empty() {
        let msg = format!("corpus {missing:?} not found under {} (set GRIDCRAFT_OFFICE_CORPUS)", corpus_root.display());
        if required {
            return Err(format!("{msg}; GRIDCRAFT_REQUIRE_OFFICE_CORPUS is set"));
        }
        println!("check-corpus: SKIPPED, {msg}");
        return Ok(());
    }
    let mut build = crate::cargo();
    build.args(["build", "--release", "-p", "gridcraft-cli"]);
    crate::run(build, "cargo build --release -p gridcraft-cli")?;
    // The cargo target dir may be redirected (CARGO_TARGET_DIR); ask for the real one.
    let cli = std::env::var_os("CARGO_TARGET_DIR").map_or_else(gridcraft_cli, |d| PathBuf::from(d).join("release/gridcraft-cli"));

    let mut current: BTreeMap<String, Value> = BTreeMap::new();
    for name in CORPORA {
        let dir = corpus_root.join(name);
        let shas = manifest(&dir)?;
        eprintln!("$ gridcraft-cli check {} --json --max-report 0", dir.display());
        let out = Command::new(&cli)
            .arg("check")
            .arg(&dir)
            .args(["--json", "--max-report", "0"])
            .stderr(Stdio::inherit())
            .output()
            .map_err(|e| format!("failed to spawn {}: {e}", cli.display()))?;
        // Exit 1 just means some file differs; no JSON means the process itself died.
        let report: Value =
            serde_json::from_slice(&out.stdout).map_err(|e| format!("{name}: gridcraft-cli check died ({}) without a report: {e}", out.status))?;
        for f in report["files"].as_array().into_iter().flatten() {
            let path = f["path"].as_str().unwrap_or("");
            let Some(rel) = Path::new(path).strip_prefix(&dir).ok().map(|p| p.to_string_lossy().replace('\\', "/")) else {
                return Err(format!("{name}: {path} is outside the corpus"));
            };
            let sha = shas.get(&rel).cloned().unwrap_or_else(|| "not-in-manifest".into());
            let mut r = record(f, &sha);
            // Keep the baseline machine-independent: no corpus location in error messages.
            if let Some(e) = r["error"].as_str() {
                r["error"] = json!(e.replace(&*corpus_root.to_string_lossy(), "$GRIDCRAFT_OFFICE_CORPUS"));
            }
            current.insert(format!("{name}/{rel}"), r);
        }
    }

    let path = root.join(BASELINE);
    if update {
        let doc = json!({"schema": SCHEMA, "files": current});
        std::fs::create_dir_all(path.parent().unwrap_or(root)).map_err(|e| e.to_string())?;
        std::fs::write(&path, serde_json::to_string_pretty(&doc).map_err(|e| e.to_string())? + "\n").map_err(|e| e.to_string())?;
        println!("baseline updated: {} files", current.len());
    }
    let baseline: Value = std::fs::read_to_string(&path).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null);
    let known = &baseline["files"];

    let (mut ok, mut crashes, mut failed) = (0usize, Vec::new(), Vec::new());
    let mut fixed = Vec::new();
    for (key, now) in &current {
        let then = known.get(key);
        if now["status"] == json!("ok") {
            ok += 1;
        }
        let why = regressions(now, then);
        if now["status"] == json!("crash") {
            crashes.push(key.clone());
        } else if !why.is_empty() {
            failed.push(format!("{key}: {}", why.join("; ")));
        }
        if let Some(t) = then
            && t["status"] != json!("ok")
            && now["status"] == json!("ok")
        {
            fixed.push(key.clone());
        }
    }
    let gone = known.as_object().map_or(0, |m| m.keys().filter(|k| !current.contains_key(*k)).count());
    let by_class = |class: &str| current.values().filter(|r| r["class"] == json!(class)).count();
    println!(
        "\n{} files ({} Excel-produced, {} other): {} ok, {} regressions, {} newly fixed, {} crashes, {} in the baseline but not on disk",
        current.len(),
        by_class("excel"),
        by_class("other"),
        ok,
        failed.len(),
        fixed.len(),
        crashes.len(),
        gone
    );
    if !fixed.is_empty() {
        println!("newly fixed (run with --update-baseline): {fixed:?}");
    }
    if !crashes.is_empty() {
        return Err(format!("crashes: {crashes:?}"));
    }
    if baseline.is_null() {
        return Err(format!("no baseline at {BASELINE}; run with --update-baseline"));
    }
    if !failed.is_empty() {
        return Err(format!("regressions vs {BASELINE}:\n  {}", failed.join("\n  ")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(mismatched: &[(&str, u64)], package: &[(&str, u64)]) -> Value {
        let fns: Vec<Value> = mismatched.iter().map(|(n, m)| json!({"name": n, "mismatched": m})).collect();
        let cats: serde_json::Map<String, Value> = package.iter().map(|(n, d)| (n.to_string(), json!({"diffs": d}))).collect();
        json!({"producer": "Microsoft Excel", "opened": true, "calc": {"mismatched": mismatched.iter().map(|m| m.1).sum::<u64>(),
            "functions": fns, "formulaCells": 3, "checked": 3, "matched": 3}, "roundTrip": {"categories": cats}})
    }

    #[test]
    fn producer_classes() {
        assert!(is_excel(Some("Microsoft Excel")));
        assert!(is_excel(Some("Microsoft Macintosh Excel")));
        assert!(!is_excel(Some("LibreOffice/7.3")));
        assert!(!is_excel(Some("Apache POI")));
        assert!(!is_excel(None));
    }

    #[test]
    fn records_and_regressions() {
        let ok = record(&file(&[], &[]), "a");
        assert_eq!(ok["status"], "ok");
        let bad = record(&file(&[("SUM", 2)], &[("values", 1)]), "a");
        assert_eq!(bad["mismatches"]["SUM"], 2);
        assert_eq!(bad["package"]["values"], 1);
        // Same or better than the baseline: fine.
        assert!(regressions(&bad, Some(&bad)).is_empty());
        assert!(regressions(&ok, Some(&bad)).is_empty());
        // More mismatches, a new function, a new category.
        let worse = record(&file(&[("SUM", 3), ("AVERAGE", 1)], &[("values", 1), ("names", 1)]), "a");
        assert_eq!(regressions(&worse, Some(&bad)).len(), 3);
        assert!(!regressions(&bad, Some(&ok)).is_empty());
        // A file the baseline doesn't know, or knows with another sha256.
        assert!(!regressions(&bad, None).is_empty());
        assert!(!regressions(&bad, Some(&record(&file(&[("SUM", 9)], &[]), "b"))).is_empty());
        assert!(regressions(&ok, None).is_empty());
    }

    #[test]
    fn crashes_and_open_errors() {
        let crash = record(&json!({"opened": false, "error": "internal error while checking: boom"}), "a");
        assert_eq!(crash["status"], "crash");
        assert_eq!(regressions(&crash, Some(&crash)), vec!["crash".to_string()]);
        let err = record(&json!({"opened": false, "error": "password-protected"}), "a");
        assert_eq!(err["status"], "open-error");
        assert!(regressions(&err, Some(&err)).is_empty());
        let ok = record(&file(&[], &[]), "a");
        assert!(!regressions(&err, Some(&ok)).is_empty());
    }
}
