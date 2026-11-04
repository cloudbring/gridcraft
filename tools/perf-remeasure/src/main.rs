//! Re-measure of gridcraft conditional-format lookups, old per-cell rescans vs per-paint range index
//! and cached compiled formulas (#114, #192).
//!
//! OLD: `gridcraft-engine` (+ xlsx, model, core) at merge-base 85e51e009c24fad47575f04d3c08f12342615d82,
//! renamed git dependencies. NEW: branch algo/114-cf-rule-cache, 48d4b6ac..., path dependencies.
//! Workload = one paint: a fresh `CfCache` (the NEW cache is documented per paint) and `look` for every cell of
//! the viewport, which is what the renderer does per frame.
//!
//! Usage: perf-remeasure CORPUS_DIR OUT_DIR [reps]
use std::hint::black_box;
use std::path::{Path, PathBuf};

use craft_bench::report::{Ratio, Report, Revs};
use craft_bench::{Config, Paired};

macro_rules! side {
    ($m:ident, $eng:ident, $xlsx:ident, $model:ident, $core:ident) => {
        mod $m {
            use $core::{CellRef, RangeRef, Value};
            use $eng::cf::CfCache;
            use $model::{CfOperator, CfRule, CondFormat, Style, Workbook};

            pub struct Wb {
                pub wb: Workbook,
                /// (sheet index, rows, cols) viewports to paint: every sheet that has conditional formats.
                pub views: Vec<(usize, u32, u32)>,
            }

            /// Cap on painted cells per sheet, a very large window.
            const CAP: u64 = 50_000;

            fn views_of(wb: &Workbook) -> Vec<(usize, u32, u32)> {
                let mut v = Vec::new();
                for i in 0..wb.sheets.len() {
                    let Some(sh) = wb.sheet(i) else { continue };
                    if sh.cond_formats.is_empty() {
                        continue;
                    }
                    let (mut rows, mut cols) = sh.used_range().map_or((50, 20), |r| (r.end.row + 1, r.end.col + 1));
                    cols = cols.clamp(1, 200);
                    rows = rows.clamp(1, (CAP / u64::from(cols)).max(1) as u32);
                    v.push((i, rows, cols));
                }
                v
            }

            /// The workbook, when it reads and has at least one sheet with conditional formats; the rule count too.
            pub fn read(bytes: &[u8]) -> Option<(Wb, usize)> {
                let (wb, _) = $xlsx::read_xlsx(bytes).ok()?;
                let views = views_of(&wb);
                if views.is_empty() {
                    return None;
                }
                let n = views.iter().map(|(i, ..)| wb.sheet(*i).map_or(0, |s| s.cond_formats.len())).sum();
                Some((Wb { wb, views }, n))
            }

            /// Synthetic sheet: `rows` x 10 numbers (some text), `rules` rules each covering a horizontal band.
            pub fn synth(rows: u32, rules: u32, kind: &str) -> Wb {
                let mut wb = Workbook::new();
                {
                    let sh = wb.sheet_mut(0).expect("sheet");
                    for r in 0..rows {
                        for c in 0..10 {
                            let v = if (r + c) % 7 == 0 { Value::from(format!("item {}", r % 13).as_str()) } else { Value::from(f64::from((r * 3 + c) % 17)) };
                            sh.set_value(CellRef::new(r, c), v);
                        }
                    }
                    let band = (rows / rules).max(1);
                    for i in 0..rules {
                        let start = i * band;
                        let end = if i + 1 == rules { rows - 1 } else { start + band - 1 };
                        let range = RangeRef::new(CellRef::new(start, 0), CellRef::new(end, 9));
                        let style = Box::new(Style::default());
                        let rule = match kind {
                            "expression" => CfRule::Expression { formula: "=A1>0".to_owned(), style },
                            _ => match i % 4 {
                                0 => CfRule::CellIs { op: CfOperator::Greater, a: "8".to_owned(), b: None, style },
                                1 => CfRule::Expression { formula: "=$A1>5".to_owned(), style },
                                2 => CfRule::ContainsText { text: "item 3".to_owned(), style },
                                _ => CfRule::Duplicate { style },
                            },
                        };
                        sh.cond_formats.push(CondFormat { ranges: vec![range], rule, priority: i, stop_if_true: false });
                    }
                }
                Wb { wb, views: vec![(0, rows, 10)] }
            }

            /// Timed: one paint. A cheap digest of the looks.
            pub fn paint(w: &Wb) -> (usize, usize) {
                let (mut some, mut styled) = (0usize, 0usize);
                for &(s, rows, cols) in &w.views {
                    let mut cache = CfCache::new(); // one cache per painted sheet, as the renderer paints one sheet at a time
                    for r in 0..rows {
                        for c in 0..cols {
                            if let Some(l) = cache.look(&w.wb, s, CellRef::new(r, c)) {
                                some += 1;
                                styled += usize::from(l.style.is_some()) + usize::from(l.scale_fill.is_some()) + usize::from(l.bar.is_some()) + usize::from(l.icon.is_some());
                            }
                        }
                    }
                }
                (some, styled)
            }

            /// Exact, one `CfCache` shared by all sheets of the workbook (not how the renderer paints).
            pub fn canon_shared(w: &Wb) -> String {
                let mut cache = CfCache::new();
                let mut out = String::new();
                for &(s, rows, cols) in &w.views {
                    for r in 0..rows {
                        for c in 0..cols {
                            out.push_str(&format!("{s}:{r}:{c}={:?}\n", cache.look(&w.wb, s, CellRef::new(r, c))));
                        }
                    }
                }
                out
            }

            /// Exact: the Debug form of every cell's look.
            pub fn canon(w: &Wb) -> String {
                let mut out = String::new();
                for &(s, rows, cols) in &w.views {
                    let mut cache = CfCache::new();
                    for r in 0..rows {
                        for c in 0..cols {
                            out.push_str(&format!("{s}:{r}:{c}={:?}\n", cache.look(&w.wb, s, CellRef::new(r, c))));
                        }
                    }
                }
                out
            }
        }
    };
}
side!(old, gridcraft_engine_old, gridcraft_xlsx_old, gridcraft_model_old, gridcraft_core_old);
side!(new, gridcraft_engine, gridcraft_xlsx, gridcraft_model, gridcraft_core);

struct Input {
    old: old::Wb,
    new: new::Wb,
}

fn xlsx_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("xlsx") || x.eq_ignore_ascii_case("xlsm")) {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let corpus = PathBuf::from(&a[1]);
    let out_dir = a[2].as_str();
    let reps: usize = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(40);
    let revs = Revs { old: Some("85e51e009c24fad47575f04d3c08f12342615d82".into()), new: Some("48d4b6ac45e89b47db66c0ad69a52c69db2ea32a".into()) };
    std::fs::create_dir_all(out_dir).unwrap();
    let mut classes: Vec<(String, Vec<(String, Input)>)> = Vec::new();
    let mut census = String::from("| corpus | .xlsx/.xlsm files | read by OLD and NEW | with conditional formats (qualify) | rules in qualifying files | read errors (either side) |\n|---|---|---|---|---|---|\n");

    for corp in ["office-poi", "office-libreoffice", "office-openxml-sdk"] {
        let files = xlsx_files(&corpus.join(corp));
        let (mut inputs, mut unreadable, mut rules, mut mismatched_qualify) = (Vec::new(), 0usize, 0usize, 0usize);
        for f in &files {
            let Ok(bytes) = std::fs::read(f) else { unreadable += 1; continue };
            let (o, n) = (old::read(&bytes), new::read(&bytes));
            match (o, n) {
                (Some((o, no)), Some((n, nn))) => {
                    assert_eq!(no, nn, "rule count differs in {}", f.display());
                    rules += nn;
                    let name = f.strip_prefix(&corpus).unwrap_or(f).display().to_string();
                    inputs.push((name, Input { old: o, new: n }));
                }
                (None, None) => {}
                _ => mismatched_qualify += 1,
            }
        }
        census.push_str(&format!("| {corp} | {} | - | {} | {rules} | {unreadable} (one-sided qualify: {mismatched_qualify}) |\n", files.len(), inputs.len()));
        classes.push((format!("real-{corp}"), inputs));
    }

    // Synthetic, row counts x 10 columns = 1k / 10k / 50k (the claim: 5000 rows, 200 rules) / 100k cells.
    let mut synth = Vec::new();
    for (rows, rules, kind) in [
        (100u32, 10u32, "expression"),
        (1000, 10, "expression"),
        (1000, 200, "expression"),
        (5000, 200, "expression"), // the claim's shape: 50k cells x 200 rules
        (10_000, 10, "expression"),
        (10_000, 200, "expression"),
        (100, 10, "mixed"),
        (1000, 10, "mixed"),
        (1000, 200, "mixed"),
        (10_000, 200, "mixed"),
    ] {
        let name = format!("synthetic {}k cells x {rules} {kind} rules", rows * 10 / 1000.max(1));
        let name = if rows == 100 { format!("synthetic 1k cells x {rules} {kind} rules") } else { name };
        synth.push((name, Input { old: old::synth(rows, rules, kind), new: new::synth(rows, rules, kind) }));
    }
    classes.push(("synthetic".into(), synth));

    for (class, inputs) in classes {
        if inputs.is_empty() {
            continue;
        }
        // Equivalence gate: inputs whose outputs differ are listed (first differing cell, truncated) and
        // excluded from timing; a faster wrong answer is not a speedup.
        let total = inputs.len();
        let mut shared = String::new();
        for (n, i) in &inputs {
            let (o, w) = (old::canon_shared(&i.old), new::canon_shared(&i.new));
            if o != w {
                let ndiff = o.lines().zip(w.lines()).filter(|(x, y)| x != y).count();
                let first = o.lines().zip(w.lines()).find(|(x, y)| x != y).map(|(x, y)| format!("OLD {} || NEW {}", &x[..x.len().min(130)], &y[..y.len().min(130)])).unwrap_or_default();
                shared.push_str(&format!("- {n}: {ndiff} of {} cells differ; first: {first}\n", o.lines().count()));
            }
        }
        std::fs::write(format!("{out_dir}/cf-{class}-shared-cache-diffs.md"), &shared).unwrap();
        let mut unequal = String::new();
        let mut kept = Vec::new();
        for (n, i) in inputs {
            let (o, w) = (old::canon(&i.old), new::canon(&i.new));
            if o == w {
                kept.push((n, i));
            } else {
                let ndiff = o.lines().zip(w.lines()).filter(|(x, y)| x != y).count();
                let first = o.lines().zip(w.lines()).find(|(x, y)| x != y).map(|(x, y)| format!("OLD {} || NEW {}", &x[..x.len().min(160)], &y[..y.len().min(160)])).unwrap_or_default();
                unequal.push_str(&format!("- {n}: {ndiff} of {} cells differ; first: {first}\n", o.lines().count()));
            }
        }
        let inputs = kept;
        std::fs::write(format!("{out_dir}/cf-{class}-unequal.md"), &unequal).unwrap();
        eprintln!("{class}: {}/{total} inputs identical, {} differ\n{unequal}", inputs.len(), total - inputs.len());
        if inputs.is_empty() {
            continue;
        }
        let cfg = Config { repetitions: reps, resamples: 4000, revs: revs.clone(), ..Config::default() };
        let report = Paired::new(format!("gridcraft conditional formats, {class}"))
            .old(|i: &Input| black_box(old::paint(&i.old)))
            .new(|i: &Input| black_box(new::paint(&i.new)))
            .inputs(inputs)
            .equal()
            .run(&cfg)
            .expect("bench");
        std::fs::write(format!("{out_dir}/cf-{class}.json"), report.to_json()).unwrap();
        std::fs::write(format!("{out_dir}/cf-{class}.md"), summary(&class, &report)).unwrap();
        eprintln!("{class}: publishable={} contended={}", report.publishable, report.contended);
    }
    std::fs::write(format!("{out_dir}/cf-census.md"), census).unwrap();
}

fn inv(x: &Ratio) -> String {
    format!("{:.2}x [{:.2}, {:.2}]", 1.0 / x.point, 1.0 / x.ci_hi, 1.0 / x.ci_lo)
}

/// speedup = old/new = 1/ratio; the CI inverts too.
fn summary(class: &str, r: &Report) -> String {
    let mut s = format!(
        "### {class}  (publishable: {}, contended: {}, load {:?}->{:?}, {} runs/side)\n\n| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |\n|---|---|---|---|---|---|\n",
        r.publishable, r.contended, r.guard.load_before, r.guard.load_after, r.config.runs_per_side
    );
    for i in &r.inputs {
        s.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            i.name,
            craft_bench::report::fmt_ns(i.wall.old.median_ns),
            craft_bench::report::fmt_ns(i.wall.new.median_ns),
            inv(&i.wall.ratio),
            i.cpu.as_ref().map(|c| inv(&c.ratio)).unwrap_or_default(),
            i.wall.ratio.verdict().label()
        ));
    }
    s.push_str(&format!("| **geomean** | | | **{}** | {} | |\n", inv(&r.aggregate.wall), r.aggregate.cpu.as_ref().map(inv).unwrap_or_default()));
    s
}
