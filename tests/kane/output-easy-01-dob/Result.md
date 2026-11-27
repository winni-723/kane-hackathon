---
test: ../easy-01-dob_test.md
status: passed
started: 2026-11-26T23:35:52.165Z
duration_s: 57
session_id: 7909abbe-2345-4e03-9d43-ab126e13e129
---

# easy-01: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (21s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (28.9s)
md5: c88a5d9cea7eeaad1c1b67498313fa97
Verify that the Result region contains the text 'Shortness of breath on exertion'.
Verify that the Result region contains the text 'Congestive heart failure'.
Verify that the Result region contains the text 'Furosemide 40 mg daily'.
Verify that the Result region contains the text 'Lisinopril 10 mg daily'.
Verify that the Result region contains the text 'echocardiogram in 4 weeks'.
Verify that the Result region does not contain the text '03/14/1958'.
