# XLSX corpus check (`cargo xtask check-corpus`)

`gridcraft-cli check` ([cli.md](../cli.md#check)) recalculates every formula of a workbook and compares the
result with the value its producer cached, then writes the workbook back and validates the package round
trip. `cargo xtask check-corpus` runs it over every `.xlsx`/`.xlsm` of the three verified Office corpora and
compares the outcome with the committed [`baselines/xlsx-corpus.json`](../../baselines/xlsx-corpus.json),
the way pdfcraft's `cargo xtask check` compares with `baselines/pdfjs.json`.

```sh
cargo xtask check-corpus                    # fails on a crash, or a file/function/category worse than the baseline
cargo xtask check-corpus --update-baseline  # rewrite the baseline (review the diff: every change is a claim)
```

| Environment | Meaning |
|---|---|
| `GRIDCRAFT_OFFICE_CORPUS` | Directory holding `office-poi`, `office-libreoffice`, `office-openxml-sdk` (default `/Users/emm/ArtCraft/corpus`). Read-only. |
| `GRIDCRAFT_REQUIRE_OFFICE_CORPUS` | Set (not `0`): a missing corpus fails the command. Unset: it prints `SKIPPED` and exits 0. |

Baseline records are keyed by `<corpus>/<manifest path>` and carry the manifest `sha256` (a file with another
sha256 has no baseline and must be `ok`), the producer (`docProps/app.xml` `Application`) and its class
(`excel`: "Microsoft Excel", "Microsoft Macintosh Excel", "Microsoft Excel Online"; `other`: LibreOffice, POI,
absent), and a status: `ok`, `differs` (`mismatches` by function, `package` issues by category), `open-error` or
`crash` (a panic caught by `check`). A run fails when a file crashes, no longer opens, has more mismatches for a
function or more differences in a category than recorded, or is not `ok` and not in the baseline. Counts going
down are reported as "newly fixed"; run with `--update-baseline` to lock them in.

CI: [`.github/workflows/xlsx-corpus.yml`](../../.github/workflows/xlsx-corpus.yml) (path-filtered to the
calc/format/xlsx crates, the CLI, xtask and the baseline). GitHub runs workflows from the default branch, so it
cannot run on the fork until it reaches main, and the corpus fetch step waits for craft-corpus
(artcraft-algorithms#180).

## Result (2026-11-04, GridCraft 0.16.0 plus the fixes below)

Cached values from a producer that is not Excel are a weaker oracle (LibreOffice and POI write their own
results, and POI test files often hold deliberately wrong or locale-specific ones), so the two are separate.

| | Excel-produced | Other producers |
|---|---:|---:|
| Files | 868 | 154 |
| Opened | 868 | 150 (2 password-protected, 2 not valid packages) |
| Formula cells | 68,329 | 36,823 |
| Recomputed and compared | 45,780 | 26,316 |
| Agree with the cached value | 42,246 (**92.3 %**) | 24,728 (94.0 %) |
| Mismatched | 3,534 in 32 files | 1,588 in 12 files |
| Volatile (skipped) / no oracle | 22,548 / 1 | 10,487 / 20 |
| Files with no mismatch and a clean round trip | 454 | 71 |
| Crashes / panics | **0** | **0** |

Excluding the CUBE* functions (an external data connection, below) the Excel-produced agreement is 97.9 %
(42,246 of 43,158). Corpus split: office-poi 348 files, office-libreoffice 565, office-openxml-sdk 109 (1,022 in
all; `SheetData.xlsx` and `Connection.xlsx` alone hold 60,000 formulas, most of them volatile).

## Top functions by mismatch count (Excel-produced files)

Counts are cells whose formula calls the function; *primary* means no precedent of the cell mismatches too.

| Function | Checked | Mismatched | Primary | Examples (file: cell, cached, GridCraft) |
|---|---:|---:|---:|---|
| CUBEVALUE | 2,278 | 1,927 | 0 | `Connection.xlsx`, `SheetData.xlsx` (openxml-sdk): downstream of CUBEMEMBER, cached numbers, GridCraft `#N/A` |
| CUBEMEMBER | 1,010 | 660 | 660 | `Connection.xlsx`: `Sheet1!D1` "Sales Amount" vs `#N/A`; `Sheet1!A2` "Accessories" vs `#N/A`; `A3` "Bike Racks" vs `#N/A` |
| (operators, no function) | 21,903 | 365 | 208 | `0-www-crossref-org...suffix-generator.xlsm`: `Data_sheet!C12` `=A:A&"/"&B:B`, "/" vs `#SPILL!`; `C13`, `C14` the same |
| FORMULATEXT | 225 | 103 | 55 | `DynamicArrayFixture.xlsx`: `'CSE Array Spill Cases'!D9` "=F5:F8 * 2" vs "=F5:F8*2"; `D13` "=F5:F8 + G5:G8" vs "=F5:F8+G5:G8" |
| ROUND | 1,048 | 57 | 5 | `FormulaEvalTestData_Copy.xlsx`: `EverythingTests!N1180` `=ROUND(M9:O9,K10)` 345 vs `#SPILL!`; `forcepoint97.xlsx`: `Sheet1!D14` `=ROUND(B14,12)=ROUND(C14,12)` TRUE vs FALSE |
| IF | 8,397 | 49 | 42 | `61495-test.xlsm`: `TEST!B1` "D 0.067" vs "D 67.10"; `empty_ext_defined_name.xlsx`: `'2er-Schichtplanung'!A8` 0 vs `#REF!` |
| FORECAST.ETS.CONFINT | 44 | 44 | 0 | `tdf131380.xlsx` (downstream of FORECAST.ETS) |
| TEXT | 1,044 | 36 | 36 | `GeneralFormatTests.xlsx`: `Tests!A27` "0.000000001" vs "1E-09"; `A46` "-1.234567891" vs "-1.23456789"; `A56` "-1.12345E+11" vs "-1.1235E+11" |
| CUBEMEMBERPROPERTY | 770 | 35 | 35 | `Pivot3.xlsx`: `Sheet1!B3` "Accessories" vs `#N/A` |
| VLOOKUP | 2,217 | 26 | 26 | `empty_ext_defined_name.xlsx`: `'2er-Schichtplanung'!A8..C8` 0 vs `#REF!` |
| ROW, SIGN, TANH, ... (array argument to a scalar function) | | 24, 23, 6 | | `FormulaEvalTestData_Copy.xlsx`: `EverythingTests!F1192` `=ROW(2:3)` 2 vs `#SPILL!`; `M1248` `=SIGN(J8:M8)` 1 vs `#SPILL!` |

By signature (`cached → GridCraft`): `number → #N/A` 1,938 and `text → #N/A` 695 (CUBE*), then 377 cells in
`... → #SPILL!` (the implicit-intersection cluster).

Other producers: 1,536 of the 1,588 mismatches are in `(no function)` cells, mostly circular-reference and
spill-vs-intersection test documents from LibreOffice and POI (`bug67784.xlsx` `=A9*A9` cached `#VALUE!`, GridCraft
`#CIRC!`; `named-ranges-global.xlsx` `=Global4` 1 vs `#SPILL!`). Add-in names (`COM.SUN.STAR.SHEET.ADDIN...`)
and LibreOffice-only functions (`TEST_FUNCTION`) are `#NAME?` in GridCraft.

## Package fidelity (round trip), files with at least one difference

| Category | Excel-produced | Other | Typical difference |
|---|---:|---:|---|
| package | 393 | 72 | a relationship kind of the source is not rewritten: `core-properties` (the most frequent, in about 250 files), `chartStyle`/`chartColorStyle` (about 100 each), `thumbnail`, `drawing`, `vbaProject`, `extendedProperties`, `pivotCacheDefinition`/`pivotTable`/`pivotCacheRecords`, `image`, `vmlDrawing` |
| tables | 4 | 0 | a table header cell holding a single space becomes `Column1` (`auto_marker_excel10.xlsx`, `tdf90839-1.xlsx`) |
| values | 5 | 0 | the same header cell: `" " -> "Column1"` |
| charts | 2 | 0 | a combo chart is written as a plain column chart (`tdf100084.xlsx`, `tdf111173.xlsx`) |
| condFormats | 2 | 0 | rule font `charset` dropped (`formats.xlsx`, `tdf167019.xlsx`) |
| pivots | 1 | 0 | `tdf122471.xlsx`: 1 pivot becomes 0 |
| sheets, formulas, numFmts, merges, names, validations, comments, hyperlinks | 0 | 0 | |

No file failed to write or read back. The totals are the baseline's `package` counts; `check` lists at most
`--max-report` examples per category, so the "typical difference" frequencies come from those examples.

## Root causes of the top mismatches

1. **CUBEVALUE / CUBEMEMBER / CUBEMEMBERPROPERTY (2,622 cells, 5.7 % of all checked): cached value from an
   external data connection. Not a GridCraft calculation bug.** The cached values ("Sales Amount", "Bike Racks")
   come from a live Analysis Services connection (`xlextdat9 Adventure Works DW`) that the test file only names;
   GridCraft has no OLAP client and returns `#N/A`; the cached values cannot be reproduced offline. CUBEVALUE is
   downstream of CUBEMEMBER (0 primary). Follow-up for the product (not this ticket): a recalculation on open
   replaces the file's cached values with `#N/A`; keeping the cached value of a CUBE* cell until the connection is
   refreshed would not lose data. Not changed here.
2. **`(no function)` and the `#SPILL!` cluster (about 380 cells with the array-argument cases of ROUND, ROW,
   SIGN, ... ): GridCraft is wrong.** A formula stored without cell metadata `cm` (and not `t="array"`) is a
   legacy formula, which Excel evaluates with implicit intersection: `=A:A&"/"&B:B` in `C12` takes `A12` and `B12`
   (cached "/"), `=ROUND(M9:O9,K10)` in `N1180` takes `N9`. GridCraft evaluates every formula as a dynamic array
   and spills (`#SPILL!` when the neighbours are occupied). `Formula.dynamic` records the `cm` mark but the
   evaluator doesn't use its absence, because a formula entered in GridCraft also has `dynamic == false` and must
   keep array behaviour. The fix needs an "imported as legacy" state on the formula, intersection of range
   operands in scalar positions inside the evaluator, and the writer keeping the legacy mark (it already writes
   `_xlfn.SINGLE` for an explicit `@`). Not a small change; recorded as a finding below. The same cause gives
   the three `FORMULATEXT` cells that show `@` (`=@F4#`).
3. **FORMULATEXT (103 cells, 55 primary): GridCraft is wrong, in display only.** 52 of the 55 primary
   mismatches differ from the cached text only by whitespace (`=F5:F8 * 2` vs `=F5:F8*2`): `Formula.text` is the
   canonical printed form, so the spaces the user typed are gone after import. The other 3 are the `@` cases of
   cause 2. Keeping the source text next to the parsed expression is a model change; recorded as a finding.

Also looked at (fixed, below): `TEXT` with elapsed formats. Left alone: `TEXT(x,"00.00")` in `61495-test.xlsm`
("D 0.067": the file was saved under a locale where `.` is the thousands separator, so the cached text is not
reproducible with GridCraft's en-US `TEXT`), and the 12 `TEXT(x,"General")` cells of `GeneralFormatTests.xlsx`.

### Relation to #26 (compensated summation, decimal formatting)

No SUM/AVERAGE mismatch in the corpus traces to summation order: the 22 `SUM` mismatches in Excel-produced files are one LibreOffice
test document (`forum-mso-en4-134670.xlsx`, a FREQUENCY/MATCH array case). So the
corpus gives #26 no evidence for changing `array.rs`/`math.rs` summation. Decimal formatting does show up:
`GeneralFormatTests.xlsx` was saved by Excel for Mac and caches `TEXT(x,"General")` results that differ from
GridCraft's 11-character General fit: Excel shows `0.000000001` where GridCraft shows `1E-09`, and keeps 11
digits for a negative number (`-1.234567891`, `-10000000000`) where GridCraft reserves a character for the sign
(`-1.23456789`, `-1E+10`) and rounds `-1.12345E+11` to `-1.1235E+11`. These are 12 cells of Excel's own output
for `numfmt`'s General rule (cell display and `TEXT` may differ); they are the oracle #26 asks for before
changing that kernel, and are not changed here.

## Fixes made (red test, then green)

Three fixes in `crates/numfmt`, from `ElapsedFormatTests.xlsx` (office-poi, saved by Excel for Mac): its 26 mismatches
are now 0.

- `m` directly after seconds is minutes: `s:m" @ hour "[hh]` for 3.14159 gives `53:23 @ hour 75`, not `53:1`
  (the month). Test `minutes_after_seconds_and_plain_tokens_beside_elapsed`.
- A plain `h`/`m`/`s` token in a format that also has an elapsed `[h]`/`[m]`/`[s]` of the same unit shows the
  elapsed count: `"It was "[h]" [yes, "h"] hours"` gives `It was 75 [yes, 75] hours`. Same test.
- `TEXT` of a negative number through a format with an elapsed part keeps the minus sign
  (`TEXT(-1.5,"[h]:mm")` is `-36:00`) instead of `#VALUE!`; a cell still shows `########`, and a format
  without an elapsed part is still `#VALUE!`. Test `text_of_negative_with_elapsed_part`.

## Findings (not fixed here)

- Legacy formulas must evaluate with implicit intersection (cause 2): about 380 Excel-cached cells.
- FORMULATEXT must return the formula as typed (cause 3): 52 cells.
- `core-properties` (about 250 files), chart style/colour parts, thumbnails, `vbaProject` and pivot parts are
  dropped by the XLSX writer's package rewrite.
- A table header cell holding only a space is renamed `Column1`; combo charts are written as plain column charts.
- CUBE* cells lose their cached value on recalculation (cause 1).
- Four corpus files do not open: `58616.xlsx` and `protected_passtika.xlsx` (password-protected, expected),
  `deep-data.xlsx` and `forcepoint107.xlsx` (not valid packages).
- No crash or panic anywhere in the 1,022 files.
