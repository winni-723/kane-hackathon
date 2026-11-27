---
test: ../easy-101-phone_test.md
status: passed
started: 2026-11-26T22:33:23.680Z
duration_s: 56
session_id: 15c63bf0-ca18-486b-831a-5ac48ffcb8ab
---

# easy-101: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (24.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (27.5s)
md5: 7927f25862dc9dd679082a83c977335c
Verify that the Result region contains the text 'Burning epigastric pain after meals'.
Verify that the Result region contains the text 'Gastroesophageal reflux disease'.
Verify that the Result region contains the text 'Omeprazole 20 mg daily before breakfast for 8 weeks'.
Verify that the Result region contains the text 'elevate head of bed'.
Verify that the Result region does not contain the text '503-555-0166'.
Verify that the Result region does not contain the text '503-555-0119'.
