---
test: ../easy-01-address_test.md
status: passed
started: 2026-11-14T04:04:20.530Z
duration_s: 93
session_id: dcc93bbb-31c1-48ea-892b-4dea57c7103c
---

# easy-01: address is removed, medical content is kept — Result

## Submit the record ✓ passed (38.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (46s)
md5: a2b2a18531eb117098ce4e6d32956963
Verify that the Result region contains the text 'Shortness of breath on exertion'.
Verify that the Result region contains the text 'Congestive heart failure'.
Verify that the Result region contains the text 'Furosemide 40 mg daily'.
Verify that the Result region contains the text 'Lisinopril 10 mg daily'.
Verify that the Result region contains the text 'echocardiogram in 4 weeks'.
Verify that the Result region does not contain the text '48 Willow Bend Road'.
Verify that the Result region does not contain the text 'Westmarch'.
Verify that the Result region does not contain the text '02199'.
