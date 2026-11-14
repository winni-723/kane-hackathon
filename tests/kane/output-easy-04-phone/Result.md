---
test: ../easy-04-phone_test.md
status: passed
started: 2026-11-14T03:26:53.624Z
duration_s: 81
session_id: 5233e2e3-d027-47a6-bff5-ae05a3ab9da0
---

# easy-04: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (34.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (37.7s)
md5: e5ffef8e5d79457f4606a5d65f39a980
Verify that the Result region contains the text 'Resting tremor of the right hand'.
Verify that the Result region contains the text "Parkinson's disease".
Verify that the Result region contains the text 'Hoehn and Yahr stage 2'.
Verify that the Result region contains the text 'Carbidopa-Levodopa 25-100 mg three times daily'.
Verify that the Result region contains the text 'fall precautions'.
Verify that the Result region does not contain the text '206.555.0108'.
