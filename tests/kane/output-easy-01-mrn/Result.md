---
test: ../easy-01-mrn_test.md
status: passed
started: 2026-11-26T22:17:36.129Z
duration_s: 89
session_id: 167ac8db-d53e-474f-a65a-5b7bfb89c6c0
---

# easy-01: mrn is removed, medical content is kept — Result

## Submit the record ✓ passed (26.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted mrn is gone ✓ passed (28.8s)
md5: 8385a11e11a78791e2e31bd8cb74308b
Verify that the Result region contains the text 'Shortness of breath on exertion'.
Verify that the Result region contains the text 'Congestive heart failure'.
Verify that the Result region contains the text 'Furosemide 40 mg daily'.
Verify that the Result region contains the text 'Lisinopril 10 mg daily'.
Verify that the Result region contains the text 'echocardiogram in 4 weeks'.
Verify that the Result region does not contain the text '00482917'.
