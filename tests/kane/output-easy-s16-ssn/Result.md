---
test: ../easy-s16-ssn_test.md
status: passed
started: 2026-11-15T17:43:24.350Z
duration_s: 51
session_id: fcfe69e5-0604-4a7d-9a1f-badcdc892155
---

# easy-s16: ssn is removed, medical content is kept — Result

## Submit the record ✓ passed (23.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone ✓ passed (22.5s)
md5: e6b45ce9844cd032d5c8688e5da1443a
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text '999-66-6592'.
