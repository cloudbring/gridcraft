# craft-ooxml reader swap: old-vs-new differential on real files

Ticket: cloudbring/artcraft-algorithms#184 (gridcraft part). Evidence standard:
research/VERIFICATION.md section 1.1 (refactor or shared-crate swap).

- OLD: `gridcraft-xlsx` at `014f030a15d586ddc0ff5d775fa51991cabb8907` (merge-base, before #122).
- NEW: `gridcraft-xlsx` at `fb4a0354` (the LazyPackage adapter), with craft-ooxml re-pinned from `fbceacb`
  to craft-libs `7d46713` and the tolerance switches on (`LazyLimits::lenient()`). First run: `fbceacb`.
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

| Run | Identical | Expected | Both fail | Regression / only-new-fails |
|---|---:|---:|---:|---:|
| Before: craft-ooxml `fbceacb`, as swapped | 1004 | 0 | 4 | 2 / 3 |
| After the case-variant adapter fix | 1005 | 4 registered | 4 | 0 |
| After: craft-ooxml `7d46713`, tolerance on | **1009** | **0** | **4** | **0** |

Final, per corpus: office-poi 344 identical + 3 both-fail (347); office-openxml-sdk 101 identical;
office-libreoffice 564 identical + 1 both-fail (565). `expected.toml` is empty. No panics or
timeouts. The 4 both-fail files are rejected by both sides.

Not covered: password-encrypted and `.xls`/`.xlsb` inputs, which the engine routes before
`read_xlsx` (three `Could not find EOCD` both-fail files are encrypted containers).

## Differences found, and how each ended

### Case-variant duplicate entries (1 file): fixed upstream

`duplicate-filename-case-insensitive.xlsx` holds `sheet1.xml` and `Sheet1.xml`; the target is
`sheet1.xml`. craft-ooxml's sorted index let `Sheet1.xml` win, so new read `v2` where old read
`v1`. I first fixed it in the adapter (`Package::stored`); craft-libs `7d46713` now prefers the
exact-case entry in `LazyPackage::find`, and I confirmed the adapter branch is redundant (the
test and the corpus run pass without it) and removed it. The regression test
`package::tests::a_case_variant_entry_does_not_shadow_the_exact_one` stays.

### Backslash entry names (3 files): fixed upstream

`49609.xlsx`, `tdf131575.xlsx`, `tdf76115.xlsx` name entries `xl\workbook.xml`,
`[content_types].xml`. Old normalised stored names and opened them; the swap failed with "no
workbook part". `7d46713` adds `LazyLimits::normalize_backslash_names` (and a case-insensitive
`[Content_Types].xml` lookup); the adapter now uses `LazyLimits::lenient()`. All three open and
are byte-identical to old, so the ticket's "backslash names rejected" change is reversed for
the lazy reader.

### DOCTYPE in `[Content_Types].xml` (1 file): fixed upstream

`54764-2.xlsx` (entity expansion) lost its `fileVersion` element on save because the DTD made the
content types read as empty. `7d46713`'s `lenient_content_types` skips the DOCTYPE without
expanding entities; the file is now identical to old.

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

L3 for the reader swap on open and open-then-save over 1013 real workbooks: 1009 identical to the
byte, 4 rejected by both sides, 0 unexplained and nothing registered. The "known changes" the
corpus does not exercise (above) stay at the level shown there.
