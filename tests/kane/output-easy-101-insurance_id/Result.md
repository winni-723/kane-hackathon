---
test: ../easy-101-insurance_id_test.md
status: passed
started: 2026-11-26T22:32:46.739Z
duration_s: 56
session_id: b2aef316-8a26-428c-81f2-7b01fe8d26bc
---

# easy-101: insurance id is removed, medical content is kept — Result

## Submit the record ✓ passed (26.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted insurance id is gone ✓ passed (26.2s)
md5: 716dcb3564d9a867d2452a8d8947f7aa
Verify that the Result region contains the text 'Burning epigastric pain after meals'.
Verify that the Result region contains the text 'Gastroesophageal reflux disease'.
Verify that the Result region contains the text 'Omeprazole 20 mg daily before breakfast for 8 weeks'.
Verify that the Result region contains the text 'elevate head of bed'.
Verify that the Result region does not contain the text 'QRT-88120457'.
