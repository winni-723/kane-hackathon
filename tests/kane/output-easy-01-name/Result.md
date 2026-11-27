---
test: ../easy-01-name_test.md
status: passed
started: 2026-11-26T23:35:52.251Z
duration_s: 67
session_id: e7244a0e-e04f-402a-a941-cd0c304dc529
---

# easy-01: name is removed, medical content is kept — Result

## Submit the record ✓ passed (30.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (28.4s)
md5: f8fee4e591671481a137b498b8195152
Verify that the Result region contains the text 'Shortness of breath on exertion'.
Verify that the Result region contains the text 'Congestive heart failure'.
Verify that the Result region contains the text 'Furosemide 40 mg daily'.
Verify that the Result region contains the text 'Lisinopril 10 mg daily'.
Verify that the Result region contains the text 'echocardiogram in 4 weeks'.
Verify that the Result region does not contain the text 'Eleanor Carter'.
