---
test: ../easy-01-phone_test.md
status: passed
started: 2026-11-14T02:31:10.076Z
duration_s: 85
session_id: 6d68ccc9-e9f2-4aec-a1a2-5a08c95d0074
---

# easy-01: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (39.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (41s)
md5: 136589a8ffcd88b19100f2de21e57d6e
Verify that the Result region contains the text 'Shortness of breath on exertion'.
Verify that the Result region contains the text 'Congestive heart failure'.
Verify that the Result region contains the text 'Furosemide 40 mg daily'.
Verify that the Result region contains the text 'Lisinopril 10 mg daily'.
Verify that the Result region contains the text 'echocardiogram in 4 weeks'.
Verify that the Result region does not contain the text '(617) 555-0142'.
