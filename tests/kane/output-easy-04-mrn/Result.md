---
test: ../easy-04-mrn_test.md
status: passed
started: 2026-11-14T03:45:12.429Z
duration_s: 85
session_id: d2c4c039-9a9b-4987-ade5-a6a89c73e393
---

# easy-04: mrn is removed, medical content is kept — Result

## Submit the record ✓ passed (35.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted mrn is gone ✓ passed (43.5s)
md5: d7c6f03a7a6b476dd1e8848970d8cec5
Verify that the Result region contains the text 'Resting tremor of the right hand'.
Verify that the Result region contains the text "Parkinson's disease".
Verify that the Result region contains the text 'Hoehn and Yahr stage 2'.
Verify that the Result region contains the text 'Carbidopa-Levodopa 25-100 mg three times daily'.
Verify that the Result region contains the text 'fall precautions'.
Verify that the Result region does not contain the text '77120345'.
