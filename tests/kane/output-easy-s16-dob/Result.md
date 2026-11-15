---
test: ../easy-s16-dob_test.md
status: passed
started: 2026-11-15T17:42:39.378Z
duration_s: 52
session_id: 7e5894c0-d413-41d6-90c4-418e7f20f3b4
---

# easy-s16: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (27s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (20.3s)
md5: d2c40719c801f76a14c1bdbaafd74573
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text '2025-01-06'.
