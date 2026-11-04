### original-shape  (publishable: true, contended: false, load Some(2.71)->Some(2.71), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| 1MiB abcdefgh x 1 delims | 2.678 ms | 1.644 ms | 1.63x [1.52, 1.64] | 1.63x [1.52, 1.64] | faster |
| 1MiB abcdefgh x 3 delims | 5.199 ms | 1.640 ms | 3.17x [2.70, 3.28] | 3.17x [2.69, 3.28] | faster |
| 1MiB abcdefgh x 4 delims | 6.594 ms | 1.292 ms | 5.10x [5.05, 5.15] | 5.11x [5.05, 5.16] | faster |
| 1MiB abcdefgh x 8 delims | 11.051 ms | 1.283 ms | 8.61x [7.93, 8.89] | 8.61x [7.91, 8.85] | faster |
| 1MiB abcdefgh x 16 delims | 19.500 ms | 1.302 ms | 14.98x [14.47, 15.90] | 14.98x [14.45, 15.88] | faster |
| **geomean** | | | **5.09x [4.87, 5.15]** | 5.09x [4.86, 5.14] | |
