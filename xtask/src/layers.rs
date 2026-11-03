//! `cargo xtask layers`: dependency layering rules (AGENTS.md "Layering", plan/architecture.md).
//!
//! The rule engine works on a small, metadata-independent model so it can be
//! unit-tested; `from_metadata` builds that model from `cargo metadata`.

use craft_xtask::layers::{Config, ExternalRule, WorkspaceRule};
pub use craft_xtask::layers::{Crate, Dep, DepKind};
use serde_json::Value;
use std::fmt::Write;

/// Where a workspace crate sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// Regular layered crate.
    Layer(u8),
    /// Binaries and build tooling: exempt from the rules.
    Exempt,
}

/// The layering table (AGENTS.md "Layering", plan/architecture.md). Names are package names
/// without the `gridcraft-` prefix.
pub const TABLE: &[(&str, Class)] = &[
    ("core", Class::Layer(0)),
    ("numfmt", Class::Layer(0)),
    ("pdf", Class::Layer(0)),
    ("formula", Class::Layer(1)),
    ("functions", Class::Layer(1)),
    // Equation layout (LaTeX to glyphs, rules and paths) and OMML, on top of RaTeX.
    ("math", Class::Layer(1)),
    ("model", Class::Layer(2)),
    ("calc", Class::Layer(2)),
    ("xlsx", Class::Layer(2)),
    ("chart", Class::Layer(3)),
    ("engine", Class::Layer(5)),
    ("ui-egui", Class::Layer(6)),
    ("mcp", Class::Layer(6)),
    // L7 apps (exempt) and tooling (unchecked)
    ("gridcraft", Class::Layer(7)),
    ("cli", Class::Layer(7)),
    ("web", Class::Layer(7)),
    ("xtask", Class::Exempt),
];

/// Explicit orderings *within* a layer (earlier may be used by later). Each chain is separate:
/// `calc` and `xlsx` both build on `model` but not on each other.
pub const INTRA_LAYER_ORDER: &[&[&str]] = &[&["core", "numfmt"], &["formula", "functions"], &["model", "calc"], &["model", "xlsx"]];

/// Format crates may only depend on these workspace crates (on top of the layer rule), so a
/// file format never pulls in calculation or engine code. `math` converts equations to and from
/// OMML.
pub const FORMAT_ALLOWED: &[(&str, &[&str])] = &[("xlsx", &["core", "formula", "math", "model"])];

/// External crates that constitute a UI toolkit / windowing dependency.
/// Entries ending in `*` are prefixes.
pub const UI_CRATES: &[&str] = &["egui*", "eframe", "winit", "rfd", "wgpu*"];

/// First layer allowed to use UI crates.
pub const UI_MIN_LAYER: u8 = 6;

pub fn short_name(pkg: &str) -> &str {
    pkg.strip_prefix("gridcraft-").unwrap_or(pkg)
}

pub fn classify(pkg: &str) -> Option<Class> {
    let s = short_name(pkg);
    TABLE.iter().find(|(n, _)| *n == s).map(|(_, c)| *c)
}

/// Build the model from `cargo metadata --format-version 1 --no-deps`.
pub fn from_metadata(meta: &Value) -> Result<Vec<Crate>, String> {
    let pkgs = meta["packages"].as_array().ok_or("metadata: no packages array")?;
    let members: Vec<&str> = pkgs.iter().filter_map(|p| p["name"].as_str()).collect();
    let mut out = Vec::new();
    for p in pkgs {
        let name = p["name"].as_str().ok_or("package without name")?.to_owned();
        let mut deps = Vec::new();
        for d in p["dependencies"].as_array().into_iter().flatten() {
            let dname = d["name"].as_str().unwrap_or_default().to_owned();
            let kind = match d["kind"].as_str() {
                Some("dev") => DepKind::Dev,
                Some("build") => DepKind::Build,
                _ => DepKind::Normal,
            };
            let workspace = members.contains(&dname.as_str()) || d["path"].is_string();
            deps.push(Dep { name: dname, kind, workspace });
        }
        out.push(Crate { name, deps });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

pub fn describe(class: Option<Class>) -> String {
    match class {
        Some(Class::Layer(7)) => "L7 app".into(),
        Some(Class::Layer(l)) => format!("L{l}"),
        Some(Class::Exempt) => "unchecked".into(),
        None => "UNREGISTERED".into(),
    }
}

pub fn config() -> Config {
    let full = |name: &str| if matches!(name, "gridcraft" | "xtask") { name.to_owned() } else { format!("gridcraft-{name}") };
    let mut config = Config { root: crate::root(), ..Config::default() };
    for layer in 0..=7 {
        config
            .layers
            .push((format!("L{layer}"), TABLE.iter().filter(|(_, class)| *class == Class::Layer(layer)).map(|(name, _)| full(name)).collect()));
    }
    config.exempt = TABLE.iter().filter(|(_, class)| matches!(class, Class::Exempt | Class::Layer(7))).map(|(name, _)| full(name)).collect();
    for chain in INTRA_LAYER_ORDER {
        for (index, from) in chain.iter().enumerate() {
            for to in &chain[..index] {
                config.allowed_edges.push((full(from), full(to)));
            }
        }
    }
    config.external_rules.push(ExternalRule {
        dependencies: UI_CRATES.iter().map(|name| (*name).into()).collect(),
        allowed_from: TABLE
            .iter()
            .filter(|(_, class)| matches!(class, Class::Layer(layer) if *layer >= UI_MIN_LAYER))
            .map(|(name, _)| full(name))
            .collect(),
        allowed_dev_from: Vec::new(),
    });
    config.workspace_rules = FORMAT_ALLOWED
        .iter()
        .map(|(name, allowed)| WorkspaceRule { from: full(name), allowed: allowed.iter().map(|name| full(name)).collect(), allow_dev: true })
        .collect();
    config
}

fn diagnostic(message: String) -> Option<String> {
    if let Some(name) = message.strip_suffix(": unregistered workspace crate") {
        return Some(format!("{name}: unknown workspace crate; register it in xtask/src/layers.rs TABLE (see plan/architecture.md §3)"));
    }
    let Some((source, rest)) = message.split_once(" -> ") else { return Some(message) };
    let Some((target, rule)) = rest.split_once(": ") else { return Some(message) };
    let (target, kind) = target.split_once(" [").map_or((target, ""), |(target, kind)| (target, kind.trim_end_matches(']')));
    let level = |name| match classify(name) {
        Some(Class::Layer(layer)) => layer,
        _ => 7,
    };
    Some(match rule {
        "forbidden external dependency" => {
            format!("{source} (L{}) depends on UI crate `{target}`; UI toolkits are only allowed in L6+", level(source))
        }
        "workspace dependency allowlist" => format!("{source}: format crates may only depend on core/formula/model, not {target}"),
        "disallowed layer edge" => {
            classify(target)?;
            format!("{source} (L{}) -> {target} (L{}) [{kind}]: may only depend on strictly lower layers", level(source), level(target))
        }
        _ => message,
    })
}

pub fn report(crates: &[Crate]) -> Result<(String, usize), String> {
    let mut violations: Vec<_> = craft_xtask::layers::check(&config(), crates)?.into_iter().filter_map(diagnostic).collect();
    violations.sort();
    violations.dedup();
    let mut output = String::from("Dependency layering (AGENTS.md \"Layering\")\n\n");
    writeln!(output, "{:<28} {:<10} workspace deps", "crate", "layer").map_err(|error| error.to_string())?;
    for krate in crates {
        let dependencies: Vec<_> = krate
            .deps
            .iter()
            .filter(|dep| dep.workspace)
            .map(|dep| {
                let kind = match dep.kind {
                    DepKind::Normal => "",
                    DepKind::Dev => " (dev)",
                    DepKind::Build => " (build)",
                };
                format!("{}{kind}", short_name(&dep.name))
            })
            .collect();
        writeln!(output, "{:<28} {:<10} {}", krate.name, describe(classify(&krate.name)), dependencies.join(", "))
            .map_err(|error| error.to_string())?;
    }
    output.push('\n');
    if violations.is_empty() {
        writeln!(output, "OK: {} crates, no layering violations.", crates.len()).map_err(|error| error.to_string())?;
    } else {
        writeln!(output, "{} violation(s):", violations.len()).map_err(|error| error.to_string())?;
        for violation in &violations {
            writeln!(output, "  - {violation}").map_err(|error| error.to_string())?;
        }
    }
    Ok((output, violations.len()))
}
