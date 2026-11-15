---
test: ../easy-101-insurance_id_test.md
status: passed
started: 2026-11-15T18:04:12.340Z
duration_s: 86
session_id: 2168e34d-19cf-48ff-9627-fabdb62184ec
---

# easy-101: insurance id is removed, medical content is kept — Result

## Submit the record ✓ passed (40s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted insurance id is gone ✓ passed (33.8s)
md5: 716dcb3564d9a867d2452a8d8947f7aa
Verify that the Result region contains the text 'Burning epigastric pain after meals'.
Verify that the Result region contains the text 'Gastroesophageal reflux disease'.
Verify that the Result region contains the text 'Omeprazole 20 mg daily before breakfast for 8 weeks'.
Verify that the Result region contains the text 'elevate head of bed'.
Verify that the Result region does not contain the text 'QRT-88120457'.
