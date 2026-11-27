---
test: ../easy-s16-dob_test.md
status: passed
started: 2026-11-26T22:38:58.031Z
duration_s: 46
session_id: 9dbb4c21-d66e-45a7-959c-0a22cebb397c
---

# easy-s16: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (24.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (17.1s)
md5: d2c40719c801f76a14c1bdbaafd74573
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text '2025-01-06'.
