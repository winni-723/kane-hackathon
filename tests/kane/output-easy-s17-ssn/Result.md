---
test: ../easy-s17-ssn_test.md
status: passed
started: 2026-11-15T17:44:42.978Z
duration_s: 57
session_id: db329c0c-1589-49b9-a139-e144b144dea1
---

# easy-s17: ssn is removed, medical content is kept — Result

## Submit the record ✓ passed (29.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone ✓ passed (22s)
md5: a36253645aaf577437ac998084fcfb98
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text '999-35-7133'.
