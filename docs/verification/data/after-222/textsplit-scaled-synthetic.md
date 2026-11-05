### scaled-synthetic  (publishable: true, contended: false, load Some(5.6)->Some(5.31), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| 1000 cells x 1 col-delims (4889 B) | 83.188 us | 72.251 us | 1.15x [1.13, 1.17] | 1.15x [1.12, 1.17] | faster |
| 1000 cells x 3 col-delims (4889 B) | 99.374 us | 72.605 us | 1.37x [1.33, 1.39] | 1.36x [1.33, 1.39] | faster |
| 1000 cells x 8 col-delims (4889 B) | 142.813 us | 72.499 us | 1.97x [1.88, 2.01] | 1.97x [1.87, 2.02] | faster |
| 1000 cells x 8 col-delims, single row, no row-delim (4889 B) | 101.833 us | 68.833 us | 1.48x [1.39, 1.52] | 1.48x [1.38, 1.51] | faster |
| 10000 cells x 1 col-delims (58889 B) | 867.084 us | 692.917 us | 1.25x [1.24, 1.27] | 1.25x [1.24, 1.27] | faster |
| 10000 cells x 3 col-delims (58889 B) | 1.065 ms | 691.375 us | 1.54x [1.50, 1.55] | 1.54x [1.49, 1.55] | faster |
| 10000 cells x 8 col-delims (58889 B) | 1.503 ms | 695.958 us | 2.16x [2.10, 2.26] | 2.14x [2.10, 2.26] | faster |
| 10000 cells x 8 col-delims, single row, no row-delim (58889 B) | 1.142 ms | 567.292 us | 2.01x [1.94, 2.03] | 2.01x [1.92, 2.03] | faster |
| 100000 cells x 1 col-delims (688889 B) | 9.125 ms | 7.148 ms | 1.28x [1.27, 1.29] | 1.28x [1.26, 1.29] | faster |
| 100000 cells x 3 col-delims (688889 B) | 11.080 ms | 7.190 ms | 1.54x [1.52, 1.56] | 1.54x [1.52, 1.55] | faster |
| 100000 cells x 8 col-delims (688889 B) | 16.826 ms | 7.177 ms | 2.34x [2.29, 2.39] | 2.35x [2.29, 2.39] | faster |
| 100000 cells x 8 col-delims, single row, no row-delim (688889 B) | 12.152 ms | 5.714 ms | 2.13x [2.10, 2.18] | 2.12x [2.09, 2.19] | faster |
| **geomean** | | | **1.64x [1.62, 1.65]** | 1.64x [1.62, 1.65] | |
