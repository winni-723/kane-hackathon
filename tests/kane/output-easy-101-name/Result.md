---
test: ../easy-101-name_test.md
status: passed
started: 2026-11-26T22:33:15.805Z
duration_s: 59
session_id: 24d0df2d-a953-43dc-b565-285088f6a9bd
---

# easy-101: name is removed, medical content is kept — Result

## Submit the record ✓ passed (27.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (27.9s)
md5: 51726c33ba427e5e3e939092c7cb5bcc
Verify that the Result region contains the text 'Burning epigastric pain after meals'.
Verify that the Result region contains the text 'Gastroesophageal reflux disease'.
Verify that the Result region contains the text 'Omeprazole 20 mg daily before breakfast for 8 weeks'.
Verify that the Result region contains the text 'elevate head of bed'.
Verify that the Result region does not contain the text 'Brennan'.
Verify that the Result region does not contain the text 'Oliver'.
