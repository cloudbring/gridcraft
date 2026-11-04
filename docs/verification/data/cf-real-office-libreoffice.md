### real-office-libreoffice  (publishable: true, contended: false, load Some(2.4)->Some(2.4), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| office-libreoffice/sc/qa/uitest/data/autofilter/tdf142579.xlsx | 2.396 us | 3.208 us | 0.75x [0.68, 0.88] | 0.86x [0.75, 1.00] | slower |
| office-libreoffice/sc/qa/uitest/data/autofilter/tdf142580.xlsx | 1.542 us | 2.396 us | 0.64x [0.57, 0.78] | 0.67x [0.67, 1.00] | slower |
| office-libreoffice/sc/qa/uitest/data/tdf118206.xlsx | 47.479 us | 27.416 us | 1.73x [1.69, 1.77] | 1.78x [1.68, 1.81] | faster |
| office-libreoffice/sc/qa/unit/data/functions/dynamic_array/xlsx/DynamicArrayFixture.xlsx | 64.625 us | 417.000 us | 0.15x [0.15, 0.18] | 0.16x [0.15, 0.18] | slower |
| office-libreoffice/sc/qa/unit/data/xlsm/tdf111974.xlsm | 939.417 us | 4.343 ms | 0.22x [0.21, 0.22] | 0.22x [0.21, 0.22] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/colorscale.xlsx | 15.626 us | 22.250 us | 0.70x [0.66, 0.77] | 0.70x [0.65, 0.77] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/colorscale_num_with_ref.xlsx | 12.166 us | 15.917 us | 0.76x [0.74, 0.83] | 0.81x [0.75, 0.87] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/cond_format_formula_listener.xlsx | 2.479 us | 3.208 us | 0.77x [0.57, 1.14] | 0.75x [0.75, 1.33] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/cond_format_theme_color2.xlsx | 3.146 us | 8.166 us | 0.39x [0.36, 0.43] | 0.44x [0.33, 0.50] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/cond_format_theme_color3.xlsx | 5.729 us | 10.604 us | 0.54x [0.51, 0.63] | 0.55x [0.55, 0.64] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/cond_parent.xlsx | 396 ns | 1.958 us | 0.20x [0.17, 0.27] | 0.50x [0.00, 0.50] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/condformat_databar.xlsx | 9.625 us | 79.708 us | 0.12x [0.12, 0.12] | 0.12x [0.12, 0.12] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/condformat_theme_color.xlsx | 3.771 us | 4.917 us | 0.77x [0.68, 0.86] | 0.80x [0.67, 1.00] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/conditional_fmt_checkpriority.xlsx | 334 ns | 854 ns | 0.39x [0.30, 0.56] | 1.00x [1.00, 1.00] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/conditional_fmt_origin.xlsx | 1.208 us | 5.104 us | 0.24x [0.22, 0.30] | 0.40x [0.20, 0.40] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/databar.xlsx | 10.063 us | 19.521 us | 0.52x [0.49, 0.59] | 0.50x [0.49, 0.62] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/formats.xlsx | 20.979 us | 11.625 us | 1.80x [1.74, 1.91] | 1.75x [1.62, 2.00] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/invalid_ext_data_validation.xlsx | 425.666 us | 1.773 ms | 0.24x [0.24, 0.24] | 0.24x [0.24, 0.24] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/new_cond_format_test.xlsx | 66.834 us | 92.250 us | 0.72x [0.71, 0.77] | 0.72x [0.71, 0.77] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/new_cond_format_test_export.xlsx | 66.812 us | 90.645 us | 0.74x [0.71, 0.77] | 0.74x [0.71, 0.77] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf105272.xlsx | 13.459 us | 31.667 us | 0.43x [0.40, 0.51] | 0.44x [0.39, 0.50] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf113271.xlsx | 390.438 us | 475.479 us | 0.82x [0.81, 0.83] | 0.82x [0.81, 0.83] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf113621.xlsx | 4.541 us | 3.458 us | 1.31x [1.16, 1.49] | 1.25x [1.25, 1.67] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf130959.xlsx | 373.146 us | 1.662 ms | 0.22x [0.22, 0.23] | 0.22x [0.22, 0.23] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf134392.xlsx | 22.688 us | 87.395 us | 0.26x [0.25, 0.28] | 0.26x [0.25, 0.28] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf134769.xlsx | 33.625 us | 49.312 us | 0.68x [0.65, 0.71] | 0.67x [0.64, 0.72] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf138601.xlsx | 6.021 us | 13.292 us | 0.45x [0.43, 0.52] | 0.43x [0.43, 0.54] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf148820.xlsx | 13.271 us | 35.896 us | 0.37x [0.35, 0.42] | 0.39x [0.35, 0.42] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf152581_bordercolorNotExportedToXLSX.xlsx | 2.562 us | 2.729 us | 0.94x [0.83, 1.11] | 1.00x [1.00, 1.00] | inconclusive (CI includes 1.0) |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf153767.xlsx | 169.979 us | 85.625 us | 1.99x [1.94, 2.05] | 1.98x [1.93, 2.06] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf155321.xlsx | 1.125 us | 1.917 us | 0.59x [0.52, 0.72] | 0.50x [0.50, 1.00] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf156028.xlsx | 500 ns | 896 ns | 0.56x [0.41, 0.86] | 1.00x [1.00, 1.00] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf162948.xlsx | 2.313 us | 3.125 us | 0.74x [0.65, 0.85] | 1.00x [0.57, 1.00] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf166413.xlsx | 1.375 us | 3.042 us | 0.45x [0.38, 0.61] | 0.67x [0.50, 0.67] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf167019.xlsx | 67.959 us | 48.772 us | 1.39x [1.31, 1.46] | 1.40x [1.29, 1.47] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf169379.xlsx | 19.791 us | 24.812 us | 0.80x [0.76, 0.88] | 0.80x [0.75, 0.88] | slower |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf170298.xlsx | 171.542 us | 94.355 us | 1.82x [1.76, 1.86] | 1.81x [1.76, 1.86] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf171828_fail_to_import_file.xlsx | 11.445 ms | 3.913 ms | 2.93x [2.92, 2.93] | 2.93x [2.92, 2.93] | faster |
| office-libreoffice/sc/qa/unit/data/xlsx/tdf41722.xlsx | 750 ns | 1.125 us | 0.67x [0.51, 0.89] | 1.00x [0.50, 1.00] | slower |
| office-libreoffice/sc/qa/unit/uicalc/data/tdf118189.xlsx | 47.770 us | 26.771 us | 1.78x [1.68, 1.81] | 1.75x [1.68, 1.81] | faster |
| office-libreoffice/sc/qa/unit/uicalc/data/tdf158802.xlsx | 5.252 ms | 3.660 ms | 1.43x [1.43, 1.44] | 1.43x [1.43, 1.44] | faster |
| **geomean** | | | **0.62x [0.61, 0.64]** | 0.69x [0.58, 0.71] | |
