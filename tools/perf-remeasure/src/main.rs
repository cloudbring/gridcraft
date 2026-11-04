//! Re-measure of gridcraft TEXTSPLIT, old matcher vs leftmost-longest multi-pattern matcher (#57, #192).
//!
//! OLD: `gridcraft-functions` at merge-base 85e51e009c24fad47575f04d3c08f12342615d82 (renamed git dep).
//! NEW: `gridcraft-functions` at 02443015... (branch algo/57-textsplit-matcher), path dep.
//! Both are driven through the public `lookup("TEXTSPLIT")` + `call`, so what is timed is the whole
//! function (char conversion, matching, split, array build), not the private `find_matches` the
//! claim's bench timed; that function is `pub(crate)` on the NEW side and cannot be called here.
//!
//! Usage: perf-remeasure TEXT_FILE OUT_DIR [reps]
use std::hint::black_box;

use craft_bench::report::{Ratio, Report, Revs};
use craft_bench::{Config, Paired};

macro_rules! side {
    ($m:ident, $f:ident, $c:ident) => {
        mod $m {
            use $c::{Array, Value};
            use $f::{Arg, Ctx};
            struct C;
            impl Ctx for C {
                fn date_system(&self) -> $c::DateSystem {
                    $c::DateSystem::default()
                }
                fn now_serial(&self) -> f64 {
                    0.0
                }
                fn random(&mut self) -> f64 {
                    0.0
                }
            }
            pub struct Args(Vec<Arg>);
            fn list(d: &[String]) -> Value {
                Value::Array(std::sync::Arc::new(Array::row(d.iter().map(|s| Value::from(s.as_str())).collect())))
            }
            pub fn args(text: &str, col: &[String], row: &[String]) -> Args {
                let mut a = vec![Arg::val(text), Arg::reference(list(col))];
                if !row.is_empty() {
                    a.push(Arg::reference(list(row)));
                }
                Args(a)
            }
            fn call(a: &Args) -> Value {
                let spec = $f::lookup("TEXTSPLIT").expect("TEXTSPLIT registered");
                $f::call(spec, &a.0, &mut C)
            }
            /// Cheap digest used inside the timed region: shape and total text length.
            pub fn digest(a: &Args) -> (usize, usize, usize) {
                match call(a) {
                    Value::Array(arr) => (arr.rows, arr.cols, arr.data.iter().map(|v| if let Value::Text(t) = v { t.len() } else { 1 }).sum()),
                    Value::Text(t) => (1, 1, t.len()),
                    _ => (0, 0, 0),
                }
            }
            /// Exact canonical form, used for the pre-timing equivalence check.
            pub fn canon(a: &Args) -> String {
                format!("{:?}", call(a))
            }
        }
    };
}
side!(old, gridcraft_functions_old, gridcraft_core_old);
side!(new, gridcraft_functions, gridcraft_core);

struct Input {
    old: old::Args,
    new: new::Args,
}

fn input(text: &str, col: &[String], row: &[String]) -> Input {
    Input { old: old::args(text, col, row), new: new::args(text, col, row) }
}

fn strs(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

/// `n` non-matching two-character delimiters (like the claim's bench), `hit` of which are real separators.
fn delims(n: usize, hits: &[&str]) -> Vec<String> {
    let filler = ["zz", "yx", "wv", "ut", "sr", "qp", "on", "ml", "kj", "ih", "gf", "ed", "cb", "xw", "vu", "ts"];
    let mut d = strs(hits);
    let mut i = 0;
    while d.len() < n {
        d.push(filler[i % filler.len()].to_string());
        i += 1;
    }
    d.truncate(n.max(hits.len()));
    d
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let real = std::fs::read_to_string(&a[1]).expect("real text");
    let out_dir = a[2].as_str();
    let reps: usize = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(40);
    let revs = Revs { old: Some("85e51e009c24fad47575f04d3c08f12342615d82".into()), new: Some("024430150526eacc1a983f3cabbb7713ebc30749".into()) };
    std::fs::create_dir_all(out_dir).unwrap();

    let mut classes: Vec<(&str, Vec<(String, Input)>)> = Vec::new();

    // 1. The claim's own shape: 1 MiB "abcdefgh" repeated, 8 non-matching delimiters (also 1, 3, 16 for scaling).
    let mib = "abcdefgh".repeat(131_072);
    classes.push((
        "original-shape",
        [1usize, 3, 4, 8, 16].iter().map(|&n| (format!("1MiB abcdefgh x {n} delims"), input(&mib, &delims(n, &[]), &[]))).collect(),
    ));

    // 2. Scaled synthetic: N output cells = 1k / 10k / 100k; fields "f<i>" separated by ',' with ';' rows of 10.
    let mut scaled = Vec::new();
    for cells in [1_000usize, 10_000, 100_000] {
        let text: String = (0..cells / 10).map(|r| (0..10).map(|c| format!("f{}", r * 10 + c)).collect::<Vec<_>>().join(",")).collect::<Vec<_>>().join(";");
        for n in [1usize, 3, 8] {
            let col = delims(n, &[","]);
            scaled.push((format!("{cells} cells x {n} col-delims ({} B)", text.len()), input(&text, &col, &strs(&[";"]))));
        }
        // One row only (no row delimiter): one matcher call over the whole text.
        let flat = text.replace(';', ",");
        scaled.push((format!("{cells} cells x 8 col-delims, single row, no row-delim ({} B)", flat.len()), input(&flat, &delims(8, &[","]), &[])));
    }
    classes.push(("scaled-synthetic", scaled));

    // 3. Real text: shared strings of 212 corpus workbooks (office-poi / office-libreoffice / office-openxml-sdk).
    let mut realin = Vec::new();
    for bytes in [1_000usize, 10_000, 100_000, 1_000_000] {
        let mut end = bytes.min(real.len());
        while !real.is_char_boundary(end) {
            end -= 1;
        }
        let text = &real[..end];
        for (name, col) in [
            ("1 delim", strs(&[" "])),
            ("3 delims", strs(&[", ", ";", " "])),
            ("8 delims", strs(&[", ", ";", "\t", " - ", "/", "|", ":", " "])),
        ] {
            realin.push((format!("real text {bytes} B, {name}, rows by newline"), input(text, &col, &strs(&["\n"]))));
        }
        let col8 = strs(&[", ", ";", "\t", " - ", "/", "|", ":", " "]);
        realin.push((format!("real text {bytes} B, 8 delims, single row, no row-delim"), input(text, &col8, &[])));
    }
    classes.push(("real-text", realin));

    for (class, inputs) in classes {
        for (n, i) in &inputs {
            assert_eq!(old::canon(&i.old), new::canon(&i.new), "output differs on {class}: {n}");
        }
        eprintln!("{class}: {} inputs, outputs identical", inputs.len());
        let cfg = Config { repetitions: reps, resamples: 4000, revs: revs.clone(), ..Config::default() };
        let report = Paired::new(format!("gridcraft TEXTSPLIT, {class}"))
            .old(|i: &Input| black_box(old::digest(&i.old)))
            .new(|i: &Input| black_box(new::digest(&i.new)))
            .inputs(inputs)
            .equal()
            .run(&cfg)
            .expect("bench");
        std::fs::write(format!("{out_dir}/textsplit-{class}.json"), report.to_json()).unwrap();
        std::fs::write(format!("{out_dir}/textsplit-{class}.md"), summary(class, &report)).unwrap();
        eprintln!("{class}: publishable={} contended={}", report.publishable, report.contended);
    }
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
