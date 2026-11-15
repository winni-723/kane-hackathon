---
test: ../easy-s17-name_test.md
status: passed
started: 2026-11-15T17:44:23.475Z
duration_s: 54
session_id: 851e9539-7c5a-44e1-ba43-2a0932a74779
---

# easy-s17: name is removed, medical content is kept — Result

## Submit the record ✓ passed (27.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (21.9s)
md5: 536179e440934ad4d6c6f48cee426149
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text 'Evia'.
Verify that the Result region does not contain the text 'Ernser'.
