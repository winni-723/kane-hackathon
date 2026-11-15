---
test: ../easy-s17-dob_test.md
status: passed
started: 2026-11-15T18:10:42.930Z
duration_s: 69
session_id: 1ac0b66e-79ed-43de-be67-9f029fbf2e24
---

# easy-s17: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (32.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (32.3s)
md5: 254fb0f449e22aa4fd48e0f75905c176
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text '2024-03-17'.
