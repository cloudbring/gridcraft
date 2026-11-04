# Speedup re-measure: conditional formats (artcraft-algorithms#192, claim C09 from #114)

Claim: "gridcraft conditional formats 2.0x" (50k-cell viewport x 200 rules, 115 -> 58 ms), branch `algo/114-cf-rule-cache`.
(The TEXTSPLIT claim C06 is on its own branch, `algo/192-remeasure`, with its own `docs/verification/speedups.md`; the two source branches differ, so they cannot share one NEW build.)

- OLD: merge-base with origin/main `85e51e009c24fad47575f04d3c08f12342615d82`, renamed git dependencies on `https://github.com/storytold/gridcraft` (`gridcraft-engine`, `-xlsx`, `-model`, `-core`).
- NEW: `48d4b6ac45e89b47db66c0ad69a52c69db2ea32a`, path dependencies.
- Harness: `tools/perf-remeasure/` (standalone workspace), craft-bench from craft-libs `94c6d76e5bc2c936d3db5ce5e19d70213e8390c0`.
  Run: `cargo run --release -p perf-remeasure -- /Users/emm/ArtCraft/corpus docs/verification/data 40`.
- Workload = one paint: a fresh `gridcraft_engine::cf::CfCache` per painted sheet (NEW's cache is documented per paint, and the renderer paints one sheet at a time) and `CfCache::look` for every cell of the viewport.
  Real workbooks: every sheet that has conditional formats, viewport = its used range, capped at 50,000 cells per sheet. Each side parses the file with its own `read_xlsx`; parsing is not timed.
- Machine: Apple M6, 12 cores, 32 GiB, macOS Darwin 27.0.0, rustc 1.99.0, release. Run 2026-11-04 6:42 PM EST. Load average 2.3-2.5 on 12 cores (threshold 6.0); every report `publishable: true`, `contended: false`.
- Method: 40 runs per side per input, ABBA/BAAB alternation, 2 warm-ups, 4,000 bootstrap resamples, wall and process CPU time. craft-bench reports new/old; below, speedup = old/new and the CI is inverted. Below 1.0x is a slowdown.
- Equivalence: before any timing, the exact `Debug` form of every cell's `Option<CfLook>` was required equal between OLD and NEW on every input (assert; inputs that differ are listed in `data/cf-*-unequal.md` and excluded, the file is empty: none). Passed on 88 of 88 inputs. craft-bench `.equal()` re-checks a digest.

## Real workbooks: 78 qualify

Scanned `/Users/emm/ArtCraft/corpus/{office-poi,office-libreoffice,office-openxml-sdk}`: 1,022 .xlsx/.xlsm files, 0 read errors on either side; **78 contain conditional formats** (same 78 as a raw `<conditionalFormatting` search of the sheet XML, so the reader loses none).

| corpus | files | with conditional formats | rules in them |
|---|---|---|---|
| office-poi | 348 | 20 | 131 |
| office-libreoffice | 565 | 41 | 206 |
| office-openxml-sdk | 109 | 17 | 324 |

## Results (speedup = old/new wall, geometric mean over the class's inputs, [95% CI])

| input class | inputs | speedup [95% CI] | CPU-time speedup [95% CI] | faster / slower / inconclusive | publishable |
|---|---|---|---|---|---|
| real: office-poi | 20 | 0.67x [0.66, 0.70] | 0.69x [0.66, 0.71] | 8 / 11 / 1 | yes |
| real: office-libreoffice | 41 | 0.62x [0.61, 0.64] | 0.69x [0.58, 0.71] | 9 / 30 / 2 | yes |
| real: office-openxml-sdk | 17 | 0.32x [0.32, 0.33] | 0.32x [0.32, 0.34] | 0 / 17 / 0 | yes |
| real, all 78 pooled (geomean of per-file ratios, no pooled CI) | 78 | 0.55x | | 17 / 58 / 3 | |
| synthetic, 10 shapes | 10 | 1.55x [1.55, 1.55] | 1.55x [1.55, 1.55] | 10 / 0 / 0 | yes |

Real workbooks are small: the median painted sheet takes 16.7 us on OLD (max 12.3 ms), so these ratios are dominated by fixed per-paint cost, in absolute terms microseconds.
The workbooks with the most work do gain: `StructuredRefs-lots-with-lookups.xlsx` 12.3 -> 4.3 ms (2.87x), `tdf171828_fail_to_import_file.xlsx` 11.4 -> 3.9 ms (2.93x),
`ConditionalFormattingSamples.xlsx` 1.27 -> 0.98 ms (1.30x). The worst: `condformat_databar.xlsx` 9.6 -> 79.7 us (0.12x), `FilterByCF.xlsx` 0.47 -> 3.96 ms (0.12x), `RelationalPivotB1.xlsx` 1.7 -> 9.3 ms (0.19x).
Per-file rows with CIs: `data/cf-real-*.md`.

Synthetic (rows x 10 columns, each rule covers a horizontal band, formulas `=A1>0` or a mix of CellIs / Expression / ContainsText / Duplicate; cells numeric with some text):

| shape | OLD | NEW | speedup [95% CI] |
|---|---|---|---|
| 1k cells x 10 expression rules | 852 us | 570 us | 1.49x [1.48, 1.50] |
| 10k cells x 10 | 8.78 ms | 5.70 ms | 1.54x [1.53, 1.55] |
| 10k cells x 200 | 9.42 ms | 5.80 ms | 1.62x [1.61, 1.63] |
| **50k cells x 200 (the claim's shape)** | 48.4 ms | 29.0 ms | **1.67x [1.65, 1.68]** |
| 100k cells x 10 | 88.5 ms | 57.3 ms | 1.55x [1.53, 1.55] |
| 100k cells x 200 | 97.3 ms | 58.1 ms | 1.67x [1.67, 1.68] |
| mixed rule types, 1k x 10 / 10k x 10 / 10k x 200 / 100k x 200 | | | 1.46x / 1.48x / 1.52x / 1.52x (CIs within +-0.02) |

## Side finding: OLD shares per-rule statistics across sheets

If one `CfCache` serves several sheets of a workbook, OLD keys its rule statistics by rule index only, so sheet 2's rule 0 reuses sheet 1's min/max/percentiles; NEW keys by (sheet, rule).
Three corpus workbooks then render differently (`data/cf-*-shared-cache-diffs.md`): `colorscale.xlsx` (7 of 78 cells), `condformat_theme_color.xlsx` (5 of 11), `ConditionalFormattingSamples.xlsx` (207 of 3,688).
In the timed runs each sheet gets its own cache (as the renderer does), where outputs are identical, so this does not affect the ratios. It is read from the differing colour-scale RGB values and the key change in the diff; no red-then-green test was written here (not in scope), so it is L1 at most.

## Verdict

- 2.0x: not reproduced. On its own shape (50k cells x 200 rules) the figure is 1.67x [1.65, 1.68]; synthetic shapes span 1.46-1.67x. The claimed 2.0x is outside every CI.
- Real workbooks: not a speedup in general. 58 of 78 are slower (pooled 0.55x), 17 faster, 3 inconclusive. The gain appears on sheets with many cells and formula rules (up to 2.9x); small sheets pay a fixed per-paint cost for building the range index.
  Hypothesis, not measured here: the per-cell lookup walks a 14-level column tree with a hash lookup per level, which costs more than a linear scan when there are only a few rules.
- Correct status page wording: "conditional-format paint ~1.5-1.7x on large synthetic viewports with many formula rules; ~0.3-0.7x (slower, microseconds) on real corpus workbooks; faster only on the few with thousands of cells".
- Not measured: a cache reused across frames (OLD's persistent stats cache vs NEW's per-paint one), the egui paint itself, workbooks beyond the 78.

Raw JSON: `data/cf-real-*.json`, `data/cf-synthetic.json`; per-input tables: `data/cf-*.md`; census: `data/cf-census.md`.
