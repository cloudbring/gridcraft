### real-text  (publishable: true, contended: false, load Some(4.28)->Some(3.65), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| real text 1000 B, 1 delim, rows by newline | 21.209 us | 29.958 us | 0.71x [0.68, 0.78] | 0.72x [0.68, 0.78] | slower |
| real text 1000 B, 3 delims, rows by newline | 24.916 us | 32.084 us | 0.78x [0.73, 0.80] | 0.76x [0.72, 0.81] | slower |
| real text 1000 B, 8 delims, rows by newline | 35.083 us | 566.833 us | 0.06x [0.06, 0.06] | 0.06x [0.06, 0.07] | slower |
| real text 1000 B, 8 delims, single row, no row-delim | 18.417 us | 20.980 us | 0.88x [0.83, 0.94] | 0.86x [0.80, 0.95] | slower |
| real text 10000 B, 1 delim, rows by newline | 259.979 us | 309.875 us | 0.84x [0.81, 0.86] | 0.84x [0.81, 0.86] | slower |
| real text 10000 B, 3 delims, rows by newline | 293.812 us | 332.209 us | 0.88x [0.86, 0.91] | 0.89x [0.87, 0.91] | slower |
| real text 10000 B, 8 delims, rows by newline | 413.709 us | 4.185 ms | 0.10x [0.10, 0.10] | 0.10x [0.10, 0.10] | slower |
| real text 10000 B, 8 delims, single row, no row-delim | 199.292 us | 109.270 us | 1.82x [1.71, 1.90] | 1.82x [1.71, 1.91] | faster |
| real text 100000 B, 1 delim, rows by newline | 2.734 ms | 3.523 ms | 0.78x [0.77, 0.78] | 0.77x [0.77, 0.78] | slower |
| real text 100000 B, 3 delims, rows by newline | 3.142 ms | 3.826 ms | 0.82x [0.81, 0.83] | 0.82x [0.81, 0.83] | slower |
| real text 100000 B, 8 delims, rows by newline | 4.622 ms | 56.725 ms | 0.08x [0.08, 0.08] | 0.08x [0.08, 0.08] | slower |
| real text 100000 B, 8 delims, single row, no row-delim | 1.764 ms | 666.937 us | 2.65x [2.61, 2.70] | 2.65x [2.61, 2.69] | faster |
| real text 1000000 B, 1 delim, rows by newline | 13.640 ms | 20.806 ms | 0.66x [0.65, 0.67] | 0.66x [0.65, 0.67] | slower |
| real text 1000000 B, 3 delims, rows by newline | 17.616 ms | 23.574 ms | 0.75x [0.74, 0.76] | 0.75x [0.74, 0.75] | slower |
| real text 1000000 B, 8 delims, rows by newline | 30.160 ms | 505.531 ms | 0.06x [0.06, 0.06] | 0.06x [0.06, 0.06] | slower |
| real text 1000000 B, 8 delims, single row, no row-delim | 17.820 ms | 7.571 ms | 2.35x [2.32, 2.41] | 2.35x [2.31, 2.41] | faster |
| **geomean** | | | **0.53x [0.52, 0.53]** | 0.53x [0.52, 0.53] | |
