---
test: ../easy-s16-name_test.md
status: passed
started: 2026-11-15T17:43:04.051Z
duration_s: 55
session_id: 191056f6-0526-4707-903b-0a9a044848df
---

# easy-s16: name is removed, medical content is kept — Result

## Submit the record ✓ passed (26.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (23.4s)
md5: ae46d83b0aad987b8082c3fc5b227ed8
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text 'Elsa'.
Verify that the Result region does not contain the text 'Solís'.
