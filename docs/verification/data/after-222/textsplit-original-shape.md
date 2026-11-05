### original-shape  (publishable: true, contended: false, load Some(5.6)->Some(5.6), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| 1MiB abcdefgh x 1 delims | 2.360 ms | 1.608 ms | 1.47x [1.45, 1.50] | 1.47x [1.45, 1.50] | faster |
| 1MiB abcdefgh x 3 delims | 4.508 ms | 1.594 ms | 2.83x [2.74, 3.20] | 2.82x [2.74, 3.20] | faster |
| 1MiB abcdefgh x 4 delims | 5.710 ms | 1.212 ms | 4.71x [4.67, 5.34] | 4.71x [4.67, 5.34] | faster |
| 1MiB abcdefgh x 8 delims | 10.198 ms | 1.212 ms | 8.41x [8.17, 8.93] | 8.36x [8.13, 8.92] | faster |
| 1MiB abcdefgh x 16 delims | 18.202 ms | 1.230 ms | 14.80x [14.64, 15.19] | 14.80x [14.65, 15.15] | faster |
| **geomean** | | | **4.76x [4.72, 4.96]** | 4.75x [4.71, 4.96] | |
