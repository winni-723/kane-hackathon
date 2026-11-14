---
test: ../easy-06-mrn_test.md
status: passed
started: 2026-11-14T03:46:46.595Z
duration_s: 79
session_id: 998d1c35-457e-49ab-995d-d2a5fec06d2d
---

# easy-06: mrn is removed, medical content is kept — Result

## Submit the record ✓ passed (33s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted mrn is gone ✓ passed (39.4s)
md5: f443c760ac730a4a1a443e0ff979ddb9
Verify that the Result region contains the text 'Major depressive disorder'.
Verify that the Result region contains the text 'Sertraline 100 mg daily'.
Verify that the Result region contains the text 'PHQ-9 score 14'.
Verify that the Result region contains the text 'improved from 19'.
Verify that the Result region contains the text 'cognitive behavioral therapy weekly'.
Verify that the Result region does not contain the text '31058862'.
