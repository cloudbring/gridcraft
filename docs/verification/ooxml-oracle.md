# craft-ooxml reader swap: old-vs-new differential on real files

Ticket: cloudbring/artcraft-algorithms#184 (gridcraft part). Evidence standard:
research/VERIFICATION.md section 1.1 (refactor or shared-crate swap).

- OLD: `gridcraft-xlsx` at `014f030a15d586ddc0ff5d775fa51991cabb8907` (merge-base, before #122).
- NEW: `gridcraft-xlsx` at `fb4a0354` (the LazyPackage adapter) plus the fix below.
- Harness: `tools/ooxml-oracle` (craft-oracle `Differential` + craft-compare, craft-libs rev `313fbe2`),
  expected differences in `tools/ooxml-oracle/expected.toml`.

## Run it

```text
GRIDCRAFT_OFFICE_CORPUS=/path/to/corpus \
  cargo test --release --manifest-path tools/ooxml-oracle/Cargo.toml -- --ignored --nocapture
```

The corpus root holds `office-poi`, `office-openxml-sdk` and `office-libreoffice`. Output goes to
`GRIDCRAFT_ORACLE_OUT` (default `target/ooxml-oracle`): `report.json`, `report.md`, `details.txt`
(every difference per non-identical file) and `failures/` (inputs of failing cases). Without the
env var the test is ignored; a second, always-run test covers the synthetic known changes. The
corpus runs in about 6 s.

The tool is its own cargo workspace on purpose. The old crate is a git dependency also named
`gridcraft-xlsx`; as a dev-dependency of the main workspace it makes `-p gridcraft-xlsx` ambiguous
for every cargo command (checked: `cargo clippy -p gridcraft-xlsx` fails), and `cargo xtask wasm`
passes `-p` names. Its `Cargo.lock` starts from the main one, so both revisions build against the
same dependency versions.

## What is compared

Each side opens the bytes with `read_xlsx` and then saves with `write_xlsx`. Per file:

1. Open result (ok or error; craft-oracle classifies both-fail / only-old / only-new).
2. Import warnings, in order.
3. Semantic dump: every field of the workbook and of every sheet as serde JSON, one section per
   field (sheets, cells with values and formulas, styles and number formats, merges, defined names,
   tables, conditional formats, validations, charts, comments, hyperlinks, pivots, protection,
   print settings, ...), plus the sheet `file_id` and the `preserved` content serde skips. This is a
   superset of the 13 categories `gridcraft-cli check` compares.
4. Saved package: part list, part order, canonical XML per part (craft-compare), then bytes.

Identical means all four agree, including saved bytes. A file with a difference in any of them is
a Regression unless an `expected.toml` entry covers it.

## Result

984 `.xlsx` + 29 `.xlsm` = 1013 workbooks (no `.ods` in the corpus).

| Corpus | Files | Identical | Expected difference | Both fail | Regression |
|---|---:|---:|---:|---:|---:|
| office-poi | 347 | 342 | 2 | 3 | 0 |
| office-openxml-sdk | 101 | 101 | 0 | 0 | 0 |
| office-libreoffice | 565 | 562 | 2 | 1 | 0 |
| **Total** | **1013** | **1005** | **4** | **4** | **0** |

No panics, timeouts, only-old-fails or only-new-fails remain after the expected entries. Before
the fix and the registrations the same run gave 1004 identical, 2 regressions, 3 only-new-fails,
4 both-fail.

Not covered: password-encrypted and `.xls`/`.xlsb` inputs, which the engine routes before
`read_xlsx` (three `Could not find EOCD` both-fail files are encrypted containers).

## Differences

### Fixed: case-variant duplicate entries (1 file, was a Regression)

`office-poi/spreadsheet/duplicate-filename-case-insensitive.xlsx` holds `xl/worksheets/sheet1.xml`
and `xl/worksheets/Sheet1.xml`. The relationship targets `worksheets/sheet1.xml`. Old read the
exactly spelled entry (`v1`); new read `v2`. Root cause: `LazyPackage` keeps a sorted map and
`find` is a case-insensitive scan, so the spelling that sorts first (`Sheet1.xml`) shadows the one
requested. Fix in `Package::stored` (`crates/xlsx/src/package.rs`): an exact-spelled entry wins,
then the case-insensitive lookup. Regression test:
`package::tests::a_case_variant_entry_does_not_shadow_the_exact_one`. After the fix the file is
byte-identical on both sides.

### Registered: backslash entry names (3 files, only-new-fails)

`office-poi/spreadsheet/49609.xlsx`, `office-libreoffice/sc/qa/unit/data/xlsx/tdf131575.xlsx`,
`.../tdf76115.xlsx` name their zip entries `xl\workbook.xml`, `[content_types].xml`, ... Old
normalised stored names (`\` to `/`, lower case) and opened them. NEW fails with "no workbook
part": `LazyPackage` indexes the raw name, so no lookup finds it (the adapter's
`part.replace('\\', "/")` is applied to the request, not to the stored names; craft-ooxml's
`checked_part` treats a backslash as an unsafe name). The ticket lists this change as intended.
Finding: it costs 3 of 1013 real files (0.3%), all written by real producers (the LibreOffice two
are bug-report files for exactly this). Recommendation: reverse in craft-ooxml with a lenient name
mapping in `LazyPackage::open_with` (index `\` as `/` and let `rels_of` and the content-types read
use it); the adapter alone cannot, because `rels_of` and the content-types read are inside
craft-ooxml. Left registered, not fixed, because it needs a craft-libs change.

### Registered: DOCTYPE in `[Content_Types].xml` (1 file, semantic section differs)

`office-poi/spreadsheet/54764-2.xlsx` (POI bug 54764, an entity-expansion file) declares entities
in `[Content_Types].xml`. craft-ooxml's XML reader refuses any DTD, so the content types read as
empty and the passthrough of unmodelled parts is switched off (`KeptReader::new`). The model and
every modelled section are identical; the difference is `workbook.preserved` (old kept the
`fileVersion` element, new has none), so the saved `xl/workbook.xml` loses
`<fileVersion appName="xl" .../>`. Not on the ticket's list. Accepted: DTD refusal is a hardening
decision and the cost is limited to attributes Excel regenerates. The adapter drops the package
warning (`LazyPackage::warnings` is not surfaced), so the user is not told.

### Both-fail files with different error text (4 files)

Both sides reject these files; only the text differs. Three are encrypted containers
(`58616.xlsx`, `deep-data.xlsx`, `protected_passtika.xlsx`): old `invalid Zip archive: Could not
find EOCD`, new `ZIP error: invalid Zip archive: ...` (the craft-ooxml error prefix).
`forcepoint107.xlsx` (zip bomb): old `package expands to more than 1 GB`, new `package bytes`
(`OpcError::Limit` text wrapped in `IoError::TooLarge`). Same error class either way. The second
message is less informative for a user; not registered because craft-oracle does not model
both-fail text.

### Known changes the corpus does not exercise

The corpus has no workbook with a duplicate relationship id (0 of 1013), no internal relationship
target that is percent-encoded (0; 33 files have `%` only in external URLs), and no part declaring
more than the part limit (`oversized_part` agrees old/new on all 1013).

- Duplicate relationship ids: new skips repeats. Checked on a minimal package
  (`synthetic_known_changes`): the first definition wins on both sides, so the model is the same.
- Percent-encoded internal targets: new decodes them. Checked on a minimal package: a sheet with
  target `worksheets/sheet%201.xml` and entry `sheet 1.xml` is empty in old (the raw `%20` finds
  no part) and has its cell in new. This is a fix, not a loss.
- `oversized_part` order, and the damaged-optional-part budget charge (1025 to 1024): unit-test
  level only (`package::tests`, 329 xlsx lib tests pass). No corpus file exercises either, so
  these stay at L1 for this ticket.

## Evidence level

L3 for the reader swap on open and open-then-save over 1013 real workbooks: 1005 identical, 4
registered (1 bounded fidelity loss, 3 open failures to reverse upstream), 0 unexplained. The
backslash finding means the claim "behaviour unchanged" does not hold until it is reversed in
craft-ooxml or the loss is accepted.
