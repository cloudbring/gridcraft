### real-office-poi  (publishable: true, contended: false, load Some(4.24)->Some(4.24), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| office-poi/spreadsheet/46535.xlsx | 164.438 us | 47.395 us | 3.47x [3.23, 3.54] | 3.44x [3.19, 3.53] | faster |
| office-poi/spreadsheet/48539.xlsx | 27.916 us | 9.874 us | 2.83x [2.72, 3.01] | 2.80x [2.55, 2.90] | faster |
| office-poi/spreadsheet/55406_Conditional_formatting_sample.xlsx | 2.772 us | 1.917 us | 1.45x [1.07, 1.78] | 1.50x [1.00, 1.75] | faster |
| office-poi/spreadsheet/61060-conditional-number-formatting.xlsx | 9.354 us | 3.438 us | 2.72x [2.50, 2.85] | 2.38x [2.25, 2.50] | faster |
| office-poi/spreadsheet/61495-test.xlsm | 7.875 us | 3.917 us | 2.01x [1.64, 2.23] | 2.00x [1.60, 2.25] | faster |
| office-poi/spreadsheet/61652.xlsx | 7.896 us | 5.479 us | 1.44x [1.27, 1.66] | 1.33x [1.33, 1.64] | faster |
| office-poi/spreadsheet/ConditionalFormattingSamples.xlsx | 1.248 ms | 570.604 us | 2.19x [2.16, 2.24] | 2.19x [2.15, 2.24] | faster |
| office-poi/spreadsheet/FillWithoutColor.xlsx | 72.041 us | 30.959 us | 2.33x [2.23, 2.48] | 2.32x [2.20, 2.46] | faster |
| office-poi/spreadsheet/LIBRE_OFFICE-128382-0.xlsx | 5.083 us | 3.500 us | 1.45x [1.35, 1.61] | 1.25x [1.25, 1.67] | faster |
| office-poi/spreadsheet/NewStyleConditionalFormattings.xlsx | 153.083 us | 109.229 us | 1.40x [1.34, 1.46] | 1.40x [1.34, 1.45] | faster |
| office-poi/spreadsheet/StructuredRefs-lots-with-lookups.xlsx | 12.250 ms | 3.660 ms | 3.35x [3.33, 3.38] | 3.35x [3.32, 3.38] | faster |
| office-poi/spreadsheet/Themes2.xlsx | 8.396 us | 4.333 us | 1.94x [1.73, 2.06] | 1.80x [1.60, 2.25] | faster |
| office-poi/spreadsheet/WithConditionalFormatting.xlsx | 16.125 us | 7.417 us | 2.17x [1.85, 2.33] | 2.12x [1.78, 2.27] | faster |
| office-poi/spreadsheet/conditional_formatting_cell_is.xlsx | 2.208 us | 2.521 us | 0.88x [0.78, 1.05] | 1.00x [0.67, 1.00] | inconclusive (CI includes 1.0) |
| office-poi/spreadsheet/conditional_formatting_multiple_ranges.xlsx | 13.312 us | 8.125 us | 1.64x [1.53, 1.73] | 1.75x [1.44, 1.75] | faster |
| office-poi/spreadsheet/conditional_formatting_with_formula_on_second_sheet.xlsx | 4.562 us | 3.979 us | 1.15x [1.00, 1.31] | 1.25x [1.00, 1.38] | inconclusive (CI includes 1.0) |
| office-poi/spreadsheet/dataValidationTableRange.xlsx | 375.791 us | 143.270 us | 2.62x [2.57, 2.67] | 2.60x [2.56, 2.66] | faster |
| office-poi/spreadsheet/simple-monthly-budget.xlsx | 6.583 us | 4.375 us | 1.50x [1.14, 1.91] | 1.40x [1.20, 1.78] | faster |
| office-poi/spreadsheet/test_conditional_formatting.xlsx | 2.021 us | 2.042 us | 0.99x [0.87, 1.13] | 1.00x [0.67, 1.25] | inconclusive (CI includes 1.0) |
| office-poi/spreadsheet/xssf-enum.xltx.xlsx | 43.167 us | 17.584 us | 2.45x [2.26, 2.61] | 2.42x [2.21, 2.50] | faster |
| **geomean** | | | **1.86x [1.81, 1.91]** | 1.84x [1.76, 1.88] | |
