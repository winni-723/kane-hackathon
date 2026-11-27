---
test: ../easy-101-dob_test.md
status: passed
started: 2026-11-26T22:32:16.588Z
duration_s: 51
session_id: 15050d3b-0fc2-411c-b1a5-4c9165ec1d68
---

# easy-101: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (21s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (26.3s)
md5: f6153e19b5241ebed85b58eab3cee421
Verify that the Result region contains the text 'Burning epigastric pain after meals'.
Verify that the Result region contains the text 'Gastroesophageal reflux disease'.
Verify that the Result region contains the text 'Omeprazole 20 mg daily before breakfast for 8 weeks'.
Verify that the Result region contains the text 'elevate head of bed'.
Verify that the Result region does not contain the text '6/2/1975'.
