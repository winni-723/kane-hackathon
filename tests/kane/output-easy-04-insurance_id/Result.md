---
test: ../easy-04-insurance_id_test.md
status: passed
started: 2026-11-26T22:23:45.112Z
duration_s: 53
session_id: 2c278e6b-901b-4b86-be00-2e633d0af4bf
---

# easy-04: insurance id is removed, medical content is kept — Result

## Submit the record ✓ passed (21s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted insurance id is gone ✓ passed (27.8s)
md5: 65d4020f0d829daeee81eff4c603419f
Verify that the Result region contains the text 'Resting tremor of the right hand'.
Verify that the Result region contains the text "Parkinson's disease".
Verify that the Result region contains the text 'Hoehn and Yahr stage 2'.
Verify that the Result region contains the text 'Carbidopa-Levodopa 25-100 mg three times daily'.
Verify that the Result region contains the text 'fall precautions'.
Verify that the Result region does not contain the text 'HX-4490217'.
