### real-office-libreoffice  (publishable: true, contended: false, load Some(4.24)->Some(4.24), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| office-libreoffice/sc/qa/uitest/data/autofilter/tdf142579.xlsx | 2.500 us | 2.542 us | 0.98x [0.79, 1.14] | 1.00x [1.00, 1.00] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/uitest/data/autofilter/tdf142580.xlsx | 1.584 us | 1.562 us | 1.01x [0.83, 1.26] | 1.00x [1.00, 1.00] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/uitest/data/tdf118206.xlsx | 48.541 us | 20.062 us | 2.42x [2.33, 2.49] | 2.45x [2.29, 2.50] | faster |
| office-libreoffice/sc/qa/unit/data/functions/dynamic_array/xlsx/DynamicArrayFixture.xlsx | 71.542 us | 32.729 us | 2.19x [1.96, 2.42] | 2.17x [1.94, 2.39] | faster |
| office-libreoffice/sc/qa/unit/data/xlsm/tdf111974.xlsm | 971.917 us | 386.625 us | 2.51x [2.44, 2.55] | 2.52x [2.45, 2.54] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/colorscale.xlsx | 15.292 us | 13.688 us | 1.12x [1.07, 1.21] | 1.14x [1.03, 1.23] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/colorscale_num_with_ref.xlsx | 12.646 us | 9.709 us | 1.30x [1.24, 1.37] | 1.30x [1.20, 1.30] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/cond_format_formula_listener.xlsx | 2.396 us | 1.833 us | 1.31x [1.00, 1.78] | 1.20x [1.00, 1.50] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/cond_format_theme_color2.xlsx | 3.250 us | 3.125 us | 1.04x [0.96, 1.15] | 1.17x [0.75, 1.33] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/cond_format_theme_color3.xlsx | 5.854 us | 5.979 us | 0.98x [0.87, 1.08] | 1.00x [0.86, 1.17] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/cond_parent.xlsx | 417 ns | 417 ns | 1.00x [0.78, 1.11] | 1.00x [1.00, 1000.00] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/condformat_databar.xlsx | 9.750 us | 9.396 us | 1.04x [1.02, 1.06] | 1.00x [1.00, 1.11] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/condformat_theme_color.xlsx | 3.688 us | 3.979 us | 0.93x [0.84, 1.06] | 1.00x [0.80, 1.25] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/conditional_fmt_checkpriority.xlsx | 333 ns | 375 ns | 0.89x [0.61, 1.17] | 0.50x [0.00, 1.00] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/conditional_fmt_origin.xlsx | 1.270 us | 1.292 us | 0.98x [0.85, 1.16] | 0.50x [0.50, 1.00] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/databar.xlsx | 10.125 us | 9.229 us | 1.10x [0.92, 1.32] | 1.22x [0.91, 1.33] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/formats.xlsx | 22.480 us | 8.625 us | 2.61x [2.13, 3.06] | 2.56x [2.09, 2.94] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/invalid_ext_data_validation.xlsx | 438.209 us | 483.334 us | 0.91x [0.89, 0.92] | 0.91x [0.89, 0.92] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/new_cond_format_test.xlsx | 66.521 us | 60.916 us | 1.09x [1.05, 1.14] | 1.10x [1.05, 1.15] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/new_cond_format_test_export.xlsx | 65.812 us | 60.291 us | 1.09x [1.03, 1.13] | 1.08x [1.03, 1.14] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf105272.xlsx | 13.792 us | 5.938 us | 2.32x [1.98, 2.76] | 2.15x [2.00, 2.62] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf113271.xlsx | 397.417 us | 203.417 us | 1.95x [1.89, 1.99] | 1.95x [1.89, 2.00] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf113621.xlsx | 4.729 us | 2.751 us | 1.72x [1.49, 1.90] | 1.67x [1.43, 1.67] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf130959.xlsx | 374.729 us | 158.855 us | 2.36x [2.30, 2.39] | 2.36x [2.29, 2.39] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf134392.xlsx | 23.145 us | 8.604 us | 2.69x [2.53, 2.90] | 2.67x [2.44, 2.88] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf134769.xlsx | 33.708 us | 16.167 us | 2.08x [1.95, 2.25] | 2.12x [1.92, 2.25] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf138601.xlsx | 6.188 us | 4.313 us | 1.43x [1.23, 1.66] | 1.40x [1.20, 1.75] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf148820.xlsx | 13.646 us | 5.687 us | 2.40x [2.06, 2.70] | 2.33x [2.00, 2.50] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf152581_bordercolorNotExportedToXLSX.xlsx | 2.646 us | 1.604 us | 1.65x [1.36, 1.87] | 1.50x [1.50, 1.50] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf153767.xlsx | 172.353 us | 75.020 us | 2.30x [2.26, 2.37] | 2.30x [2.25, 2.38] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf155321.xlsx | 1.105 us | 1.208 us | 0.91x [0.81, 1.10] | 0.50x [0.50, 1.50] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf156028.xlsx | 500 ns | 542 ns | 0.92x [0.65, 1.12] | 1.00x [0.50, 1.00] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf162948.xlsx | 2.250 us | 2.375 us | 0.95x [0.77, 1.09] | 0.83x [0.67, 1.20] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf166413.xlsx | 1.395 us | 1.625 us | 0.86x [0.76, 1.03] | 1.00x [0.80, 1.00] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf167019.xlsx | 68.417 us | 25.146 us | 2.72x [2.56, 2.82] | 2.76x [2.56, 2.80] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf169379.xlsx | 19.813 us | 20.958 us | 0.95x [0.88, 1.06] | 0.93x [0.89, 1.05] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf170298.xlsx | 174.188 us | 69.062 us | 2.52x [2.46, 2.59] | 2.53x [2.47, 2.58] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf171828_fail_to_import_file.xlsx | 11.696 ms | 3.274 ms | 3.57x [3.51, 3.61] | 3.56x [3.51, 3.60] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf41722.xlsx | 770 ns | 792 ns | 0.97x [0.82, 1.35] | 1.00x [1.00, 1.50] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/uicalc/data/tdf118189.xlsx | 49.334 us | 20.500 us | 2.41x [2.26, 2.50] | 2.36x [2.23, 2.52] | faster |
| office-libreoffice/sc/qa/unit/uicalc/data/tdf158802.xlsx | 5.343 ms | 3.434 ms | 1.56x [1.53, 1.58] | 1.56x [1.52, 1.58] | faster |
| **geomean** | | | **1.46x [1.43, 1.49]** | 1.40x [1.17, 1.48] | |
