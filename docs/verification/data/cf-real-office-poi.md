### real-office-poi  (publishable: true, contended: false, load Some(2.4)->Some(2.4), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| office-poi/spreadsheet/46535.xlsx | 166.292 us | 612.583 us | 0.27x [0.26, 0.28] | 0.27x [0.26, 0.28] | slower |
| office-poi/spreadsheet/48539.xlsx | 29.084 us | 52.083 us | 0.56x [0.53, 0.59] | 0.56x [0.53, 0.60] | slower |
| office-poi/spreadsheet/55406_Conditional_formatting_sample.xlsx | 2.771 us | 8.791 us | 0.32x [0.27, 0.43] | 0.33x [0.30, 0.44] | slower |
| office-poi/spreadsheet/61060-conditional-number-formatting.xlsx | 9.438 us | 4.271 us | 2.21x [2.01, 2.33] | 2.00x [1.80, 2.50] | faster |
| office-poi/spreadsheet/61495-test.xlsm | 7.646 us | 4.938 us | 1.55x [1.21, 1.93] | 1.64x [1.23, 2.00] | faster |
| office-poi/spreadsheet/61652.xlsx | 7.958 us | 28.270 us | 0.28x [0.27, 0.34] | 0.28x [0.28, 0.33] | slower |
| office-poi/spreadsheet/ConditionalFormattingSamples.xlsx | 1.274 ms | 981.500 us | 1.30x [1.28, 1.32] | 1.30x [1.28, 1.33] | faster |
| office-poi/spreadsheet/FillWithoutColor.xlsx | 74.999 us | 130.729 us | 0.57x [0.56, 0.60] | 0.58x [0.56, 0.59] | slower |
| office-poi/spreadsheet/LIBRE_OFFICE-128382-0.xlsx | 5.542 us | 34.084 us | 0.16x [0.15, 0.18] | 0.18x [0.14, 0.18] | slower |
| office-poi/spreadsheet/NewStyleConditionalFormattings.xlsx | 153.292 us | 143.854 us | 1.07x [1.03, 1.09] | 1.07x [1.03, 1.09] | faster |
| office-poi/spreadsheet/StructuredRefs-lots-with-lookups.xlsx | 12.340 ms | 4.293 ms | 2.87x [2.81, 2.92] | 2.87x [2.82, 2.92] | faster |
| office-poi/spreadsheet/Themes2.xlsx | 8.854 us | 13.771 us | 0.64x [0.61, 0.68] | 0.64x [0.62, 0.71] | slower |
| office-poi/spreadsheet/WithConditionalFormatting.xlsx | 16.520 us | 10.146 us | 1.63x [1.44, 1.79] | 1.70x [1.48, 1.80] | faster |
| office-poi/spreadsheet/conditional_formatting_cell_is.xlsx | 2.292 us | 5.583 us | 0.41x [0.38, 0.55] | 0.50x [0.33, 0.50] | slower |
| office-poi/spreadsheet/conditional_formatting_multiple_ranges.xlsx | 13.000 us | 14.021 us | 0.93x [0.88, 1.01] | 0.96x [0.87, 1.00] | inconclusive (CI includes 1.0) |
| office-poi/spreadsheet/conditional_formatting_with_formula_on_second_sheet.xlsx | 4.896 us | 15.729 us | 0.31x [0.29, 0.37] | 0.31x [0.29, 0.38] | slower |
| office-poi/spreadsheet/dataValidationTableRange.xlsx | 373.646 us | 278.812 us | 1.34x [1.31, 1.35] | 1.34x [1.32, 1.35] | faster |
| office-poi/spreadsheet/simple-monthly-budget.xlsx | 6.833 us | 26.292 us | 0.26x [0.21, 0.35] | 0.26x [0.22, 0.35] | slower |
| office-poi/spreadsheet/test_conditional_formatting.xlsx | 2.083 us | 6.021 us | 0.35x [0.32, 0.42] | 0.33x [0.29, 0.50] | slower |
| office-poi/spreadsheet/xssf-enum.xltx.xlsx | 42.688 us | 28.542 us | 1.50x [1.42, 1.55] | 1.48x [1.40, 1.53] | faster |
| **geomean** | | | **0.67x [0.66, 0.70]** | 0.69x [0.66, 0.71] | |
