---
test: ../easy-s16-address_test.md
status: passed
started: 2026-11-26T22:38:06.319Z
duration_s: 52
session_id: bb257f9c-3c6b-4ee9-a027-112ee676966b
---

# easy-s16: address is removed, medical content is kept — Result

## Submit the record ✓ passed (23.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (24.4s)
md5: 56045eae9f84595af41516f24b8c4169
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text '491 Ondricka Neck'.
Verify that the Result region does not contain the text 'Haverhill'.
Verify that the Result region does not contain the text '01832'.
