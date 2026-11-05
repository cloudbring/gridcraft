### real-text  (publishable: true, contended: false, load Some(5.31)->Some(5.21), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| real text 1000 B, 1 delim, rows by newline | 31.937 us | 24.521 us | 1.30x [1.17, 1.38] | 1.28x [1.18, 1.40] | faster |
| real text 1000 B, 3 delims, rows by newline | 38.416 us | 25.146 us | 1.53x [1.40, 1.57] | 1.50x [1.39, 1.58] | faster |
| real text 1000 B, 8 delims, rows by newline | 55.584 us | 25.209 us | 2.20x [2.15, 2.28] | 2.20x [2.12, 2.28] | faster |
| real text 1000 B, 8 delims, single row, no row-delim | 22.750 us | 10.125 us | 2.25x [1.96, 2.35] | 2.19x [1.84, 2.30] | faster |
| real text 10000 B, 1 delim, rows by newline | 351.854 us | 297.584 us | 1.18x [1.16, 1.21] | 1.18x [1.16, 1.21] | faster |
| real text 10000 B, 3 delims, rows by newline | 397.291 us | 294.833 us | 1.35x [1.32, 1.38] | 1.35x [1.32, 1.38] | faster |
| real text 10000 B, 8 delims, rows by newline | 557.083 us | 300.666 us | 1.85x [1.82, 1.90] | 1.85x [1.82, 1.89] | faster |
| real text 10000 B, 8 delims, single row, no row-delim | 195.980 us | 96.875 us | 2.02x [1.90, 2.12] | 2.03x [1.91, 2.13] | faster |
| real text 100000 B, 1 delim, rows by newline | 3.070 ms | 2.770 ms | 1.11x [1.09, 1.12] | 1.11x [1.09, 1.12] | faster |
| real text 100000 B, 3 delims, rows by newline | 3.464 ms | 2.792 ms | 1.24x [1.22, 1.25] | 1.24x [1.22, 1.25] | faster |
| real text 100000 B, 8 delims, rows by newline | 4.584 ms | 2.858 ms | 1.60x [1.59, 1.62] | 1.60x [1.58, 1.62] | faster |
| real text 100000 B, 8 delims, single row, no row-delim | 1.980 ms | 993.000 us | 1.99x [1.96, 2.04] | 1.99x [1.96, 2.04] | faster |
| real text 1000000 B, 1 delim, rows by newline | 14.618 ms | 10.785 ms | 1.36x [1.34, 1.37] | 1.35x [1.34, 1.37] | faster |
| real text 1000000 B, 3 delims, rows by newline | 18.726 ms | 10.848 ms | 1.73x [1.71, 1.75] | 1.72x [1.71, 1.75] | faster |
| real text 1000000 B, 8 delims, rows by newline | 32.102 ms | 11.533 ms | 2.78x [2.74, 2.81] | 2.78x [2.74, 2.80] | faster |
| real text 1000000 B, 8 delims, single row, no row-delim | 18.545 ms | 7.953 ms | 2.33x [2.31, 2.37] | 2.33x [2.31, 2.37] | faster |
| **geomean** | | | **1.68x [1.66, 1.69]** | 1.67x [1.65, 1.69] | |
