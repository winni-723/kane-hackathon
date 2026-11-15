---
test: ../easy-101-phone_test.md
status: passed
started: 2026-11-15T18:04:12.716Z
duration_s: 85
session_id: b9687ef6-67bf-4e74-a8ba-01c25431fb2e
---

# easy-101: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (34s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (38.9s)
md5: 7927f25862dc9dd679082a83c977335c
Verify that the Result region contains the text 'Burning epigastric pain after meals'.
Verify that the Result region contains the text 'Gastroesophageal reflux disease'.
Verify that the Result region contains the text 'Omeprazole 20 mg daily before breakfast for 8 weeks'.
Verify that the Result region contains the text 'elevate head of bed'.
Verify that the Result region does not contain the text '503-555-0166'.
Verify that the Result region does not contain the text '503-555-0119'.
