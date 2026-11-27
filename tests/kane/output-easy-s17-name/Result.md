---
test: ../easy-s17-name_test.md
status: passed
started: 2026-11-26T22:40:03.718Z
duration_s: 49
session_id: a0b55bad-f936-4017-adb0-ff779842d720
---

# easy-s17: name is removed, medical content is kept — Result

## Submit the record ✓ passed (20.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (24.9s)
md5: 536179e440934ad4d6c6f48cee426149
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text 'Evia'.
Verify that the Result region does not contain the text 'Ernser'.
