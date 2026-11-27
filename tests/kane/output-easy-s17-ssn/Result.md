---
test: ../easy-s17-ssn_test.md
status: passed
started: 2026-11-26T22:40:50.151Z
duration_s: 45
session_id: be241b54-0d20-40e0-a102-7436f0051931
---

# easy-s17: ssn is removed, medical content is kept — Result

## Submit the record ✓ passed (23.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone ✓ passed (17.5s)
md5: a36253645aaf577437ac998084fcfb98
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text '999-35-7133'.
