### scaled-synthetic  (publishable: true, contended: false, load Some(2.71)->Some(4.28), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| 1000 cells x 1 col-delims (4889 B) | 83.729 us | 88.771 us | 0.94x [0.92, 0.96] | 0.94x [0.92, 0.96] | slower |
| 1000 cells x 3 col-delims (4889 B) | 100.354 us | 93.791 us | 1.07x [1.05, 1.09] | 1.06x [1.05, 1.10] | faster |
| 1000 cells x 8 col-delims (4889 B) | 143.458 us | 1.161 ms | 0.12x [0.12, 0.12] | 0.12x [0.12, 0.12] | slower |
| 1000 cells x 8 col-delims, single row, no row-delim (4889 B) | 102.625 us | 70.646 us | 1.45x [1.40, 1.51] | 1.45x [1.39, 1.51] | faster |
| 10000 cells x 1 col-delims (58889 B) | 875.855 us | 886.208 us | 0.99x [0.97, 1.00] | 0.99x [0.97, 1.00] | inconclusive (CI includes 1.0) |
| 10000 cells x 3 col-delims (58889 B) | 1.049 ms | 928.480 us | 1.13x [1.11, 1.14] | 1.12x [1.11, 1.15] | faster |
| 10000 cells x 8 col-delims (58889 B) | 1.514 ms | 11.543 ms | 0.13x [0.13, 0.14] | 0.13x [0.13, 0.14] | slower |
| 10000 cells x 8 col-delims, single row, no row-delim (58889 B) | 1.145 ms | 569.833 us | 2.01x [1.96, 2.04] | 2.01x [1.94, 2.04] | faster |
| 100000 cells x 1 col-delims (688889 B) | 9.318 ms | 9.059 ms | 1.03x [1.02, 1.04] | 1.03x [1.02, 1.04] | faster |
| 100000 cells x 3 col-delims (688889 B) | 11.356 ms | 9.609 ms | 1.18x [1.17, 1.20] | 1.18x [1.17, 1.20] | faster |
| 100000 cells x 8 col-delims (688889 B) | 17.103 ms | 115.871 ms | 0.15x [0.14, 0.15] | 0.15x [0.14, 0.15] | slower |
| 100000 cells x 8 col-delims, single row, no row-delim (688889 B) | 12.425 ms | 5.773 ms | 2.15x [2.11, 2.25] | 2.14x [2.11, 2.25] | faster |
| **geomean** | | | **0.72x [0.72, 0.73]** | 0.72x [0.72, 0.73] | |
