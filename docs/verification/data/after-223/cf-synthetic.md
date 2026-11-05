### synthetic  (publishable: true, contended: false, load Some(4.94)->Some(4.55), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| synthetic 1k cells x 10 expression rules | 875.979 us | 524.458 us | 1.67x [1.62, 1.72] | 1.67x [1.61, 1.72] | faster |
| synthetic 10k cells x 10 expression rules | 9.024 ms | 5.138 ms | 1.76x [1.72, 1.79] | 1.76x [1.73, 1.79] | faster |
| synthetic 10k cells x 200 expression rules | 9.629 ms | 5.534 ms | 1.74x [1.72, 1.77] | 1.74x [1.72, 1.76] | faster |
| synthetic 50k cells x 200 expression rules | 49.939 ms | 27.500 ms | 1.82x [1.80, 1.82] | 1.81x [1.81, 1.82] | faster |
| synthetic 100k cells x 10 expression rules | 91.668 ms | 51.765 ms | 1.77x [1.76, 1.78] | 1.77x [1.76, 1.78] | faster |
| synthetic 100k cells x 200 expression rules | 100.642 ms | 55.236 ms | 1.82x [1.81, 1.84] | 1.82x [1.81, 1.84] | faster |
| synthetic 1k cells x 10 mixed rules | 709.145 us | 418.500 us | 1.69x [1.63, 1.77] | 1.69x [1.63, 1.78] | faster |
| synthetic 10k cells x 10 mixed rules | 7.152 ms | 4.091 ms | 1.75x [1.72, 1.78] | 1.75x [1.72, 1.78] | faster |
| synthetic 10k cells x 200 mixed rules | 7.764 ms | 4.709 ms | 1.65x [1.63, 1.66] | 1.65x [1.63, 1.66] | faster |
| synthetic 100k cells x 200 mixed rules | 77.402 ms | 45.419 ms | 1.70x [1.70, 1.71] | 1.70x [1.70, 1.71] | faster |
| **geomean** | | | **1.74x [1.73, 1.75]** | 1.74x [1.73, 1.75] | |
