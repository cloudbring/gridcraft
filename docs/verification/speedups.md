# Speedup re-measure: TEXTSPLIT (artcraft-algorithms#192, claim C06 from #57)

Claim: "gridcraft TEXTSPLIT 7.8x" (1 MiB string, 8 delimiters, 22.4 -> 2.9 ms), branch `algo/57-textsplit-matcher`.
(The conditional-formats claim C09 is measured on its own branch, `algo/192-remeasure-cf`, which has its own `docs/verification/speedups.md`;
the two claims live on different source branches that cannot share one NEW build.)

- OLD: merge-base with origin/main `85e51e009c24fad47575f04d3c08f12342615d82`, renamed git dependencies on `https://github.com/storytold/gridcraft` (`gridcraft-functions`, `gridcraft-core`).
- NEW: `024430150526eacc1a983f3cabbb7713ebc30749`, path dependencies.
- Harness: `tools/perf-remeasure/` (standalone workspace), craft-bench from craft-libs `94c6d76e5bc2c936d3db5ce5e19d70213e8390c0`.
  Run: `cargo run --release -p perf-remeasure -- real-strings.txt docs/verification/data 40`.
- Both sides are driven through the public `lookup("TEXTSPLIT")` + `call`. What is timed is the whole function (UTF-8 to `Vec<char>`, matching, split, array build),
  not the private `find_matches` the claim's `#[ignore]` bench timed (it is `pub(crate)` on NEW). That dilutes the matcher gain, so a figure here is lower than a matcher-only figure would be.
- Machine: Apple M6, 12 cores, 32 GiB, macos Darwin 27.0.0, rustc 1.99.0 (b940084d7 2026-09-28) (Homebrew), release. Run 2026-11-04 6:35 PM EST.
  Load average 2.7-4.3 on 12 cores (threshold 6.0); every report `publishable: true`, `contended: false`.
- Method: 40 runs per side per input, ABBA/BAAB alternation, 2 warm-ups, 4,000 bootstrap resamples, wall and process CPU time.
  craft-bench reports new/old; below, speedup = old/new and the CI is inverted. Below 1.0x is a slowdown.
- Equivalence: before any timing, the exact `Debug` form of the returned `Value` (all cells) was required equal between OLD and NEW on all 33 inputs (assert); craft-bench `.equal()` then re-checks a digest. All passed.

## Real workbooks: none qualify

Scanned `/Users/emm/ArtCraft/corpus/{office-poi,office-libreoffice,office-openxml-sdk}` (1,022 .xlsx/.xlsm; 4 unreadable as zip) by searching every `xl/worksheets/*.xml` for `TEXTSPLIT` (case-insensitive, covers `_xlfn.TEXTSPLIT`):
**0 of 1,022 workbooks contain TEXTSPLIT.** The "real workbook" requirement cannot be met for this claim. As a substitute input, the shared strings of 212 of those workbooks
(3.2 MB of real cell text, incl. Korean) are used as the haystack (`real-text`); the formula shape (delimiters, one large string) is still synthetic.
Input realism is therefore L2 for this claim; the paired method is L3.

## Results

Speedup = old/new (wall) [95% CI].

### Claim shape: 1 MiB "abcdefgh" x 131,072, non-matching delimiters, one row

| delimiters | OLD | NEW | speedup [95% CI] |
|---|---|---|---|
| 1 | 2.68 ms | 1.64 ms | 1.63x [1.52, 1.64] |
| 3 | 5.20 ms | 1.64 ms | 3.17x [2.70, 3.28] |
| 4 | 6.59 ms | 1.29 ms | 5.10x [5.05, 5.15] |
| **8 (the claim)** | 11.05 ms | 1.28 ms | **8.61x [7.93, 8.89]** |
| 16 | 19.50 ms | 1.30 ms | 14.98x [14.47, 15.90] |

The claim (7.8x) is inside the CI for the claim shape, measured on the whole function. First run on the same inputs, same method, gave 9.38x [9.24, 9.61]
(a run-to-run spread larger than either CI; see `data/` for the second run only).

### Real text, many rows (rows split by newline, then columns by the delimiters): regression

| real text, delimiters | 1 kB | 10 kB | 100 kB | 1 MB |
|---|---|---|---|---|
| 1 | 0.71x [0.68, 0.78] | 0.84x [0.81, 0.86] | 0.78x [0.77, 0.78] | 0.66x [0.65, 0.67] |
| 3 | 0.78x [0.73, 0.80] | 0.88x [0.86, 0.91] | 0.82x [0.81, 0.83] | 0.75x [0.74, 0.76] |
| 8 | 0.06x [0.06, 0.06] | 0.10x [0.10, 0.10] | 0.08x [0.08, 0.08] | 0.06x [0.06, 0.06] |

### Real text, one row (no row delimiter), 8 delimiters

1 kB 0.88x [0.83, 0.94]; 10 kB 1.82x [1.71, 1.90]; 100 kB 2.65x [2.61, 2.70]; 1 MB 2.35x [2.32, 2.41].

### Scaled synthetic (N output cells; fields `f<i>`, `,` between columns, `;` between rows of 10)

| cells | 1 delim | 3 delims | 8 delims | 8 delims, one row |
|---|---|---|---|---|
| 1,000 | 0.94x [0.92, 0.96] | 1.07x [1.05, 1.09] | 0.12x [0.12, 0.12] | 1.45x [1.40, 1.51] |
| 10,000 | 0.99x [0.97, 1.00] (inconclusive) | 1.13x [1.11, 1.14] | 0.13x [0.13, 0.14] | 2.01x [1.96, 2.04] |
| 100,000 | 1.03x [1.02, 1.04] | 1.18x [1.17, 1.20] | 0.15x [0.14, 0.15] | 2.15x [2.11, 2.25] |

CPU-time speedups equal the wall figures to within the CIs on every input (process CPU time, single thread). Per-input tables: `data/textsplit-*.md`; raw runs: `data/textsplit-*.json`.

## Cause of the regression (read from the code, confirmed by the single-row rows above)

`textsplit` calls `split_by` once for the row delimiters and then once per row for the column delimiters; each call runs `find_matches`. NEW's `find_matches` builds, on every call,
a 256-entry first-byte dispatch table (<=3 delimiters) or an `AhoCorasick` automaton (>3 delimiters). With many short rows that setup is paid per row: 8 delimiters over a 1 MB, ~25k-row text costs 505 ms against 30 ms.
With a single row the setup is paid once and the gain appears (2.0-2.65x). A fix (build the matcher once per `textsplit` call and reuse it for every row) is outside this ticket and is not applied here.

## Verdict

- 7.8x: reproduced for its own shape (single 1 MiB row, 8 non-matching delimiters): 8.61x [7.93, 8.89] on the whole function. It is the best case.
- General claim: not supported. On real text split into rows, NEW is 1.1-17x slower for every delimiter count (1.5x slower at 1 delimiter, up to 17x at 8). Gain exists only for a single large row, 2-2.7x with 8 real delimiters.
- Status page wording should say: "TEXTSPLIT 8x on one 1 MiB string with 8 delimiters; slower on multi-row input (0.06-0.9x) pending a fix that builds the matcher once per call".
- Not measured: matcher-only time, case-insensitive mode, reverse search, Excel-spill through the calc engine (no workbook contains TEXTSPLIT).
