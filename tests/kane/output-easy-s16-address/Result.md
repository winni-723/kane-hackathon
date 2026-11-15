---
test: ../easy-s16-address_test.md
status: passed
started: 2026-11-15T18:10:27.441Z
duration_s: 73
session_id: 2fdd0543-38bc-456a-80f1-078c09d9a767
---

# easy-s16: address is removed, medical content is kept — Result

## Submit the record ✓ passed (36s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (31.8s)
md5: 56045eae9f84595af41516f24b8c4169
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text '491 Ondricka Neck'.
Verify that the Result region does not contain the text 'Haverhill'.
Verify that the Result region does not contain the text '01832'.
