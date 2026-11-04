### real-office-openxml-sdk  (publishable: true, contended: false, load Some(2.4)->Some(2.29), 40 runs/side)

| input | old median | new median | wall speedup [95% CI] | cpu speedup [95% CI] | verdict |
|---|---|---|---|---|---|
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/Robustness/OFCAT/00bd6e64-896c-4ad0-980c-e7abdd4b0a52.xlsx | 45.374 us | 58.938 us | 0.77x [0.72, 0.84] | 0.76x [0.73, 0.84] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/asSources/spreadsheet/Styles.xlsx | 17.105 us | 58.583 us | 0.29x [0.27, 0.30] | 0.29x [0.27, 0.31] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/CF.xlsx | 4.688 us | 6.146 us | 0.76x [0.66, 0.92] | 0.71x [0.71, 0.86] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/Connection.xlsx | 153.125 us | 283.458 us | 0.54x [0.53, 0.56] | 0.54x [0.53, 0.56] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/FilterByCF.xlsx | 471.604 us | 3.957 ms | 0.12x [0.12, 0.12] | 0.12x [0.12, 0.12] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/FilterByColor.xlsx | 4.875 us | 17.604 us | 0.28x [0.26, 0.34] | 0.31x [0.26, 0.34] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/NoExtDataB1.xlsx | 1.690 ms | 5.646 ms | 0.30x [0.29, 0.30] | 0.30x [0.30, 0.30] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/NoExtDataF1.xlsx | 4.205 ms | 15.330 ms | 0.27x [0.27, 0.28] | 0.27x [0.27, 0.28] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/OlapPivotA3.xlsx | 2.043 ms | 3.408 ms | 0.60x [0.60, 0.60] | 0.60x [0.60, 0.60] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/Pivot2.xlsx | 1.917 us | 7.396 us | 0.26x [0.22, 0.39] | 0.25x [0.24, 0.43] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/Pivot3.xlsx | 78.459 us | 206.500 us | 0.38x [0.36, 0.40] | 0.38x [0.36, 0.41] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/RelationalPivotA1.xlsx | 306.271 us | 1.253 ms | 0.24x [0.24, 0.25] | 0.24x [0.24, 0.25] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/RelationalPivotB1.xlsx | 1.726 ms | 9.308 ms | 0.19x [0.18, 0.19] | 0.19x [0.18, 0.19] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/Styles.xlsx | 16.938 us | 58.062 us | 0.29x [0.26, 0.30] | 0.29x [0.27, 0.31] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/noextdatab4.xlsx | 856.604 us | 2.950 ms | 0.29x [0.29, 0.29] | 0.29x [0.29, 0.29] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestDataStorage/v2FxTestFiles/spreadsheet/smallset/Styles.xlsx | 17.291 us | 58.521 us | 0.30x [0.28, 0.33] | 0.31x [0.29, 0.33] | slower |
| office-openxml-sdk/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestFiles/basicspreadsheet.xlsx | 16.541 us | 58.958 us | 0.28x [0.27, 0.29] | 0.29x [0.27, 0.29] | slower |
| **geomean** | | | **0.32x [0.32, 0.33]** | 0.32x [0.32, 0.34] | |
