### real-office-openxml-sdk  (publishable: true, contended: false, load Some(4.24)->Some(4.24), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/Robustness/OFCAT/00bd6e64-896c-4ad0-980c-e7abdd4b0a52.xlsx | 45.125 us | 34.105 us | 1.32x [1.19, 1.47] | 1.34x [1.21, 1.44] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/asSources/spreadsheet/Styles.xlsx | 16.375 us | 10.646 us | 1.54x [1.46, 1.62] | 1.55x [1.42, 1.55] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/CF.xlsx | 4.833 us | 5.209 us | 0.93x [0.82, 1.09] | 0.91x [0.83, 1.00] | inconclusive (CI includes 1.0) |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/Connection.xlsx | 158.916 us | 132.250 us | 1.20x [1.17, 1.25] | 1.20x [1.16, 1.25] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/FilterByCF.xlsx | 477.812 us | 453.750 us | 1.05x [1.04, 1.06] | 1.05x [1.04, 1.06] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/FilterByColor.xlsx | 5.042 us | 4.541 us | 1.11x [0.97, 1.34] | 1.10x [1.00, 1.20] | inconclusive (CI includes 1.0) |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/NoExtDataB1.xlsx | 1.748 ms | 1.286 ms | 1.36x [1.35, 1.37] | 1.36x [1.35, 1.38] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/NoExtDataF1.xlsx | 4.288 ms | 3.294 ms | 1.30x [1.28, 1.32] | 1.30x [1.28, 1.32] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/OlapPivotA3.xlsx | 2.099 ms | 600.145 us | 3.50x [3.41, 3.58] | 3.50x [3.40, 3.56] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/Pivot2.xlsx | 1.958 us | 1.583 us | 1.24x [0.82, 1.84] | 1.00x [0.80, 1.50] | inconclusive (CI includes 1.0) |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/Pivot3.xlsx | 78.750 us | 53.021 us | 1.49x [1.38, 1.61] | 1.49x [1.39, 1.60] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/RelationalPivotA1.xlsx | 320.166 us | 213.770 us | 1.50x [1.44, 1.53] | 1.49x [1.44, 1.52] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/RelationalPivotB1.xlsx | 1.820 ms | 918.979 us | 1.98x [1.94, 2.02] | 1.98x [1.95, 2.02] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/Styles.xlsx | 16.500 us | 10.562 us | 1.56x [1.43, 1.63] | 1.55x [1.45, 1.55] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/noextdatab4.xlsx | 861.000 us | 670.188 us | 1.28x [1.28, 1.30] | 1.29x [1.28, 1.30] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/smallset/Styles.xlsx | 17.354 us | 11.001 us | 1.58x [1.41, 1.78] | 1.59x [1.40, 1.77] | faster |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestFiles/basicspreadsheet.xlsx | 17.042 us | 10.729 us | 1.59x [1.47, 1.65] | 1.55x [1.42, 1.64] | faster |
| **geomean** | | | **1.43x [1.39, 1.48]** | 1.41x [1.38, 1.46] | |
