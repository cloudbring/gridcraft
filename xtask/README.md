# GridCraft xtask

`cargo xtask --help` retains the local task descriptions and appends the shared
syntax for compatible commands. `craft-xtask` is an xtask-only dependency from
`cloudbring/craft-libs`, branch `algo/integration`, locked to `89133cd`.
Repoint the dependency to `storytold/craft-libs` after its upstream merge.

## Shared implementations

- **version:** `version::run` with default policies and this workspace root:
  byte-preserving manifest edits, permissive prereleases, staging under `target`,
  offline lockfile refresh with online fallback, no rollback or changelog edits.
- **stats:** `stats::run`, preserving ignored extra arguments, `--exact`, directory
  symlinks, exclusions (`target`, `fuzz`, dot directories), the 10,000-directory
  limit, and silent truncation. Disable the additional entry budget with
  `usize::MAX`. Shared traversal canonicalizes visited directories.
- **ico:** `ico::run` with default config: positional paths relative to the caller,
  ordered PNGs, arbitrary square dimensions from 1 through 256. No ICNS task added.
- **layers:** `layers.rs` contains only GridCraft's policy, metadata adaptation,
  and presentation over `craft_xtask::layers::check`. Preserve numeric L0–L7
  positions, intra-layer chains, exempt apps/tooling, UI rules including dev/build
  dependencies, XLSX's non-dev allowlist (including math), and legacy diagnostics.
  Keep the local metadata interpretation: path dependencies count as workspace
  dependencies, unlike shared `from_metadata`. WASM selection uses the same table.

## Intentionally local

Shared `layers::run` prints only `layers: ok`, losing the dependency table; the
local report adapter preserves its complete text instead. There is no need to
wait for the separately planned structured Report API.

**Parity remains unchanged.** GridCraft counts engine catalog command IDs,
deduplicates IDs in catalog order, and supports `--out` plus non-writing checks.
Shared parity instead rewrites weighted checklist markers; its command hook
prints the entire generated document and cannot reproduce GridCraft's summary
line or argument semantics. The shared survey identifies this inventory as
app-owned. Retain the engine feature and its feature-disabled diagnostic.

Assets, i18n_report, keytips, CI, workspace tests/doctests and WASM orchestration
also stay local. No application code, assets or CI workflows change.

## Compatibility boundary

Successful reports and file bytes have legacy golden coverage. Stats naturally
reports different xtask/TOTAL counts after this refactor. Shared version adds
`tag` and does not echo lockfile subprocess commands to stderr. Shared I/O,
invalid-argument and malformed-icon diagnostics may differ; shared ICO also
creates missing output parents. Canonical directory deduplication and deterministic
walk ordering can change partial counts on pathological/budget-exhausted trees.

## Verification

For ticket #70, set
`CARGO_TARGET_DIR=/Users/emm/ArtCraft/worktrees/gridcraft-70/target-70` on every
Cargo invocation, and remove that directory after verification:

```sh
cargo build -p xtask
cargo test -p xtask
cargo clippy -p xtask --all-targets -- -D warnings
cargo fmt --check -p xtask
cargo xtask layers
```

Only xtask is selected for compilation/testing; its default engine feature still
compiles engine dependencies. Do not run workspace CI or `stats --exact` here.
Golden tests never build another package's tests. See `tests/golden/README.md`.
