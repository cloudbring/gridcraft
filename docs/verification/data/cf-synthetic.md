### synthetic  (publishable: true, contended: false, load Some(2.29)->Some(2.45), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| synthetic 1k cells x 10 expression rules | 851.646 us | 569.896 us | 1.49x [1.48, 1.50] | 1.49x [1.48, 1.50] | faster |
| synthetic 10k cells x 10 expression rules | 8.779 ms | 5.704 ms | 1.54x [1.53, 1.55] | 1.54x [1.53, 1.55] | faster |
| synthetic 10k cells x 200 expression rules | 9.417 ms | 5.804 ms | 1.62x [1.61, 1.63] | 1.62x [1.61, 1.63] | faster |
| synthetic 50k cells x 200 expression rules | 48.423 ms | 29.033 ms | 1.67x [1.65, 1.68] | 1.67x [1.65, 1.67] | faster |
| synthetic 100k cells x 10 expression rules | 88.541 ms | 57.302 ms | 1.55x [1.53, 1.55] | 1.54x [1.54, 1.55] | faster |
| synthetic 100k cells x 200 expression rules | 97.344 ms | 58.140 ms | 1.67x [1.67, 1.68] | 1.67x [1.67, 1.68] | faster |
| synthetic 1k cells x 10 mixed rules | 702.708 us | 482.103 us | 1.46x [1.45, 1.47] | 1.46x [1.45, 1.47] | faster |
| synthetic 10k cells x 10 mixed rules | 7.007 ms | 4.719 ms | 1.48x [1.47, 1.49] | 1.48x [1.47, 1.49] | faster |
| synthetic 10k cells x 200 mixed rules | 7.641 ms | 5.032 ms | 1.52x [1.50, 1.52] | 1.52x [1.50, 1.52] | faster |
| synthetic 100k cells x 200 mixed rules | 75.233 ms | 49.391 ms | 1.52x [1.52, 1.53] | 1.52x [1.52, 1.53] | faster |
| **geomean** | | | **1.55x [1.55, 1.55]** | 1.55x [1.55, 1.55] | |
